//! API hata zarfından (`{"error": {"code", "message", "request_id", "details"}}`) türetilen hata tipi.

use std::fmt;
use std::time::Duration;

/// SDK çağrılarının sonuç tipi.
pub type Result<T> = std::result::Result<T, Error>;

/// API hata zarfındaki `error.code` değerleri (HS_ önekli katalog) ve istemci tarafı kodlar.
pub mod codes {
    pub const VALIDATION_ERROR: &str = "HS_VALIDATION_ERROR";
    pub const BAD_REQUEST: &str = "HS_BAD_REQUEST";
    pub const UNAUTHENTICATED: &str = "HS_UNAUTHENTICATED";
    pub const FORBIDDEN: &str = "HS_FORBIDDEN";
    pub const TENANT_MISMATCH: &str = "HS_TENANT_MISMATCH";
    pub const NOT_FOUND: &str = "HS_NOT_FOUND";
    pub const CONFLICT: &str = "HS_CONFLICT";
    pub const IDEMPOTENCY_CONFLICT: &str = "HS_IDEMPOTENCY_CONFLICT";
    pub const IDEMPOTENCY_IN_PROGRESS: &str = "HS_IDEMPOTENCY_IN_PROGRESS";
    pub const NO_RECIPIENTS: &str = "HS_NO_RECIPIENTS";
    pub const CHANNEL_NOT_CONFIGURED: &str = "HS_CHANNEL_NOT_CONFIGURED";
    pub const CONTENT_INVALID: &str = "HS_CONTENT_INVALID";
    pub const SUBSCRIPTION_INVALID: &str = "HS_SUBSCRIPTION_INVALID";
    pub const UNPROCESSABLE: &str = "HS_UNPROCESSABLE";
    pub const RATE_LIMITED: &str = "HS_RATE_LIMITED";
    pub const QUOTA_EXCEEDED: &str = "HS_QUOTA_EXCEEDED";
    pub const INTERNAL: &str = "HS_INTERNAL";
    pub const NOT_IMPLEMENTED: &str = "HS_NOT_IMPLEMENTED";
    pub const PROVIDER_UNAVAILABLE: &str = "HS_PROVIDER_UNAVAILABLE";
    pub const SERVICE_UNAVAILABLE: &str = "HS_SERVICE_UNAVAILABLE";
    /// İstemci tarafı: bağlantı kurulamadı veya zaman aşımı (`status` 0).
    pub const NETWORK: &str = "NETWORK";
    /// İstemci tarafı: yanıt JSON'u ayrıştırılamadı.
    pub const DECODE: &str = "DECODE";
    /// İstemci tarafı: eksik ayar veya geçersiz argüman (ör. app kimliği verilmedi).
    pub const CONFIG: &str = "CONFIG";
}

/// HTTP durum sınıflarına karşılık gelen hata kategorisi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Ağ hatası: bağlantı kurulamadı, zaman aşımı.
    Network,
    /// Geçersiz istek (400/422).
    InvalidRequest,
    /// Kimlik doğrulama (401).
    Authentication,
    /// Yetki yok (403).
    PermissionDenied,
    /// Bulunamadı (404).
    NotFound,
    /// Çakışma (409).
    Conflict,
    /// Hız sınırı (429).
    RateLimited,
    /// Sunucu hatası (5xx).
    Server,
    /// Yanıt ayrıştırılamadı.
    Decode,
    /// İstemci ayarı veya argüman hatası; istek gönderilmedi.
    Config,
    /// Diğer HTTP durumları.
    Other,
}

/// Pushizo API hatası. Ağ ve istemci hatalarında `status` 0'dır.
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    /// HTTP durum kodu (istemci tarafı hatalarda 0).
    pub status: u16,
    /// Hata kodu (`codes` modülündeki değerler; tanımsızsa `HTTP_<durum>`).
    pub code: String,
    /// Okunur hata mesajı.
    pub message: String,
    /// Destek taleplerinde kullanılacak istek kimliği.
    pub request_id: Option<String>,
    /// Ayrıntılar (ör. doğrulama hataları).
    pub details: Option<serde_json::Value>,
    /// 429/503 yanıtındaki `Retry-After` başlığı.
    pub retry_after: Option<Duration>,
}

impl Error {
    pub(crate) fn client(code: &str, message: impl Into<String>) -> Self {
        Self {
            status: 0,
            code: code.to_string(),
            message: message.into(),
            request_id: None,
            details: None,
            retry_after: None,
        }
    }

    pub(crate) fn config(message: impl Into<String>) -> Self {
        Self::client(codes::CONFIG, message)
    }

    pub(crate) fn network(message: impl Into<String>) -> Self {
        Self::client(codes::NETWORK, message)
    }

    /// Hata yanıtından (`{"error": {...}}`) hata üretir; zarf yoksa HTTP durumundan türetilir.
    pub(crate) fn from_response(
        status: u16,
        body: &[u8],
        request_id: Option<String>,
        retry_after: Option<Duration>,
    ) -> Self {
        let envelope: serde_json::Value = serde_json::from_slice(body).unwrap_or_default();
        let err = envelope.get("error");
        let text = |key: &str| {
            err.and_then(|e| e.get(key))
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        };
        let details = err
            .and_then(|e| e.get("details"))
            .filter(|v| !v.is_null())
            .cloned();
        Self {
            status,
            code: text("code").unwrap_or_else(|| format!("HTTP_{status}")),
            message: text("message").unwrap_or_else(|| format!("HTTP {status}")),
            request_id: text("request_id").or(request_id),
            details,
            retry_after,
        }
    }

    /// Hata kategorisi.
    pub fn kind(&self) -> ErrorKind {
        match self.status {
            0 => match self.code.as_str() {
                codes::DECODE => ErrorKind::Decode,
                codes::CONFIG => ErrorKind::Config,
                _ => ErrorKind::Network,
            },
            400 | 422 => ErrorKind::InvalidRequest,
            401 => ErrorKind::Authentication,
            403 => ErrorKind::PermissionDenied,
            404 => ErrorKind::NotFound,
            409 => ErrorKind::Conflict,
            429 => ErrorKind::RateLimited,
            500..=599 => ErrorKind::Server,
            _ => ErrorKind::Other,
        }
    }

    /// 429, 5xx veya ağ hatası: yeniden denenebilir.
    pub fn is_retryable(&self) -> bool {
        self.kind() == ErrorKind::Network || self.status == 429 || self.status >= 500
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pushizo: [{} {}] {}", self.status, self.code, self.message)?;
        if let Some(id) = &self.request_id {
            write!(f, " (request_id={id})")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}
