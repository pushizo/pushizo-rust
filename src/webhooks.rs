//! Giden webhook imza doğrulaması (`Pushizo-Signature: t=<unix>,v1=<hex>`).

use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// İmza başlığı: `t=<unix>,v1=<hex>`.
pub const WEBHOOK_SIGNATURE_HEADER: &str = "Pushizo-Signature";
/// Olay kimliği başlığı; tekrar gönderimleri ayıklamak için kullanın.
pub const WEBHOOK_EVENT_ID_HEADER: &str = "Pushizo-Event-Id";
/// Varsayılan zaman damgası toleransı; daha eski `t` tekrar (replay) sayılır.
pub const WEBHOOK_DEFAULT_TOLERANCE: Duration = Duration::from_secs(300);

/// Doğrulama hata nedenleri (tüm SDK'larda aynı).
pub const WEBHOOK_REASON_MISSING_HEADER: &str = "missing_header";
pub const WEBHOOK_REASON_MALFORMED_HEADER: &str = "malformed_header";
pub const WEBHOOK_REASON_TIMESTAMP_OUT_OF_TOLERANCE: &str = "timestamp_out_of_tolerance";
pub const WEBHOOK_REASON_SIGNATURE_MISMATCH: &str = "signature_mismatch";
pub const WEBHOOK_REASON_MALFORMED_BODY: &str = "malformed_body";

/// Doğrulama ayarları; `None` değerler varsayılanı kullanır.
#[derive(Debug, Clone, Copy, Default)]
pub struct WebhookVerifyOptions {
    /// Zaman damgası toleransı (varsayılan 5 dk).
    pub tolerance: Option<Duration>,
    /// Test için şimdiki zaman.
    pub now: Option<SystemTime>,
}

/// Doğrulama sonucu. `ok` ise `timestamp`, değilse `reason` doludur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookVerifyResult {
    pub ok: bool,
    pub timestamp: Option<i64>,
    pub reason: Option<&'static str>,
}

impl WebhookVerifyResult {
    fn fail(reason: &'static str) -> Self {
        Self { ok: false, timestamp: None, reason: Some(reason) }
    }
}

/// `construct_webhook_events`'in geçersiz imzada veya bozuk gövdede döndürdüğü hata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookSignatureError {
    /// `WEBHOOK_REASON_*` değerlerinden biri.
    pub reason: &'static str,
}

impl fmt::Display for WebhookSignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pushizo: webhook imzası geçersiz: {}", self.reason)
    }
}

impl std::error::Error for WebhookSignatureError {}

/// Webhook olayı (tekil gövde). Batch gövdesi `{"events": [...]}` biçimindedir.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WebhookEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub created_at: String,
    pub app_id: String,
    pub data: serde_json::Value,
}

/// `Pushizo-Signature` başlığını doğrular: `v1 = HMAC-SHA256(secret, t + "." + ham gövde)`.
///
/// `payload` HAM istek gövdesidir (JSON'u ayrıştırıp yeniden serileştirmeyin). Rotasyon sırasında eski ve yeni
/// secret birlikte verilebilir; herhangi biri eşleşirse imza geçerlidir. Karşılaştırma sabit zamanlıdır.
pub fn verify_webhook_signature<S: AsRef<str>>(
    payload: &[u8],
    header: Option<&str>,
    secrets: &[S],
    options: &WebhookVerifyOptions,
) -> WebhookVerifyResult {
    let header = match header {
        Some(h) if !h.is_empty() => h,
        _ => return WebhookVerifyResult::fail(WEBHOOK_REASON_MISSING_HEADER),
    };
    let mut timestamp: Option<i64> = None;
    let mut signatures: Vec<Vec<u8>> = Vec::new();
    for part in header.split(',') {
        let Some(idx) = part.find('=') else { continue };
        if idx == 0 {
            continue;
        }
        let key = part[..idx].trim();
        let value = part[idx + 1..].trim();
        match key {
            "t" => {
                if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) {
                    if let Ok(ts) = value.parse::<i64>() {
                        timestamp = Some(ts);
                    }
                }
            }
            "v1" => {
                if let Some(sig) = decode_lower_hex64(value) {
                    signatures.push(sig);
                }
            }
            _ => {}
        }
    }
    let Some(timestamp) = timestamp else {
        return WebhookVerifyResult::fail(WEBHOOK_REASON_MALFORMED_HEADER);
    };
    if signatures.is_empty() {
        return WebhookVerifyResult::fail(WEBHOOK_REASON_MALFORMED_HEADER);
    }
    let tolerance = options.tolerance.filter(|d| !d.is_zero()).unwrap_or(WEBHOOK_DEFAULT_TOLERANCE);
    let now = options.now.unwrap_or_else(SystemTime::now);
    let now_secs = match now.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(e) => -(e.duration().as_secs() as i64),
    };
    if (now_secs - timestamp).unsigned_abs() > tolerance.as_secs() {
        return WebhookVerifyResult::fail(WEBHOOK_REASON_TIMESTAMP_OUT_OF_TOLERANCE);
    }
    let prefix = format!("{timestamp}.");
    for secret in secrets {
        let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_ref().as_bytes()) else { continue };
        mac.update(prefix.as_bytes());
        mac.update(payload);
        for sig in &signatures {
            if mac.clone().verify_slice(sig).is_ok() {
                return WebhookVerifyResult { ok: true, timestamp: Some(timestamp), reason: None };
            }
        }
    }
    WebhookVerifyResult::fail(WEBHOOK_REASON_SIGNATURE_MISMATCH)
}

/// İmzayı doğrular ve gövdeyi olay listesine çevirir (tekil veya batch).
pub fn construct_webhook_events<S: AsRef<str>>(
    payload: &[u8],
    header: Option<&str>,
    secrets: &[S],
    options: &WebhookVerifyOptions,
) -> Result<Vec<WebhookEvent>, WebhookSignatureError> {
    let result = verify_webhook_signature(payload, header, secrets, options);
    if !result.ok {
        return Err(WebhookSignatureError { reason: result.reason.unwrap_or(WEBHOOK_REASON_SIGNATURE_MISMATCH) });
    }
    let malformed = WebhookSignatureError { reason: WEBHOOK_REASON_MALFORMED_BODY };
    let value: serde_json::Value = serde_json::from_slice(payload).map_err(|_| malformed.clone())?;
    if !value.is_object() {
        return Err(malformed);
    }
    if let Some(events) = value.get("events") {
        if let Ok(list) = serde_json::from_value::<Vec<WebhookEvent>>(events.clone()) {
            return Ok(list);
        }
    }
    serde_json::from_value::<WebhookEvent>(value).map(|e| vec![e]).map_err(|_| malformed)
}

fn decode_lower_hex64(value: &str) -> Option<Vec<u8>> {
    if value.len() != 64 || !value.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return None;
    }
    (0..32).map(|i| u8::from_str_radix(&value[i * 2..i * 2 + 2], 16).ok()).collect()
}
