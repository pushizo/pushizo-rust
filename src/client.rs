//! HTTP istemcisi: kimlik doğrulama, app kapsamı, yeniden deneme ve idempotency.

use std::fmt;
use std::io::Read;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::error::{codes, Error, Result};
use crate::multipart::{random_hex, Multipart};

/// SDK sürümü (`User-Agent` başlığında gönderilir).
pub const VERSION: &str = "0.1.0";

/// Varsayılan API adresi; yerel geliştirmede `ClientBuilder::base_url("http://localhost:4000")`.
pub const DEFAULT_BASE_URL: &str = "https://api.pushizo.com";

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_MAX_RETRIES: u32 = 3;

/// Pushizo API istemcisi. Klonlaması ucuzdur (`Arc`) ve iş parçacıkları arasında paylaşılabilir.
///
/// Kaynak servisleri `client.notifications()`, `client.users()`, `client.segments()` … ile alınır.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    api_key: String,
    base_url: String,
    app_id: Option<String>,
    agent: ureq::Agent,
    timeout: Duration,
    max_retries: u32,
    user_agent: String,
    auto_idempotency_key: bool,
    backoff: Vec<Duration>,
}

impl fmt::Debug for Client {
    // API anahtarı log'lara sızmasın diye yazılmaz.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.inner.base_url)
            .field("app_id", &self.inner.app_id)
            .finish_non_exhaustive()
    }
}

/// `Client` yapılandırması.
pub struct ClientBuilder {
    api_key: String,
    base_url: String,
    app_id: Option<String>,
    agent: Option<ureq::Agent>,
    timeout: Duration,
    max_retries: u32,
    user_agent: String,
    auto_idempotency_key: bool,
    backoff: Vec<Duration>,
}

impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientBuilder")
            .field("base_url", &self.base_url)
            .field("app_id", &self.app_id)
            .finish_non_exhaustive()
    }
}

impl ClientBuilder {
    /// Çağrılarda `RequestOptions::app_id` verilmezse kullanılacak varsayılan app kimliği.
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// API adresini değiştirir.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_string();
        self
    }

    /// Özel `ureq::Agent` (proxy, TLS ayarları) kullanır.
    pub fn agent(mut self, agent: ureq::Agent) -> Self {
        self.agent = Some(agent);
        self
    }

    /// Deneme başına istek zaman aşımı (varsayılan 30 sn).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// En fazla yeniden deneme sayısı (varsayılan 3). 429 her metotta; 5xx ve ağ hataları GET/HEAD/OPTIONS/PUT/
    /// DELETE'te, POST/PATCH'te yalnızca `Idempotency-Key` varsa veya istek sunucuya hiç gönderilmediyse (DNS
    /// hatası, bağlantı reddi) yeniden denenir.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Yeniden denemeler arasındaki bekleme süreleri (varsayılan 300 ms, 1 sn, 3 sn; son değer tekrarlanır).
    /// Sunucu `Retry-After` gönderirse o süre beklenir.
    pub fn backoff(mut self, delays: Vec<Duration>) -> Self {
        if !delays.is_empty() {
            self.backoff = delays;
        }
        self
    }

    /// `User-Agent` başlığını değiştirir.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// `Idempotency-Key` destekleyen çağrılarda anahtar verilmezse UUID üretilmesini açar/kapatır (varsayılan açık).
    pub fn auto_idempotency_key(mut self, enabled: bool) -> Self {
        self.auto_idempotency_key = enabled;
        self
    }

    /// İstemciyi oluşturur; API anahtarı boşsa hata döner.
    pub fn build(self) -> Result<Client> {
        if self.api_key.trim().is_empty() {
            return Err(Error::config("pushizo: api key gerekli"));
        }
        Ok(Client {
            inner: Arc::new(Inner {
                api_key: self.api_key,
                base_url: self.base_url,
                app_id: self.app_id.filter(|s| !s.is_empty()),
                agent: self.agent.unwrap_or_else(|| ureq::AgentBuilder::new().build()),
                timeout: self.timeout,
                max_retries: self.max_retries,
                user_agent: self.user_agent,
                auto_idempotency_key: self.auto_idempotency_key,
                backoff: self.backoff,
            }),
        })
    }
}

/// Tek bir servis çağrısının ayarları: `client.segments().with_options(RequestOptions::new().app_id("app_2"))`.
#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    app_id: Option<String>,
    idempotency_key: Option<String>,
    headers: Vec<(String, String)>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl RequestOptions {
    /// Boş ayar (istemcinin varsayılanları kullanılır).
    pub fn new() -> Self {
        Self::default()
    }

    /// Bu çağrı için app kimliği (istemcinin varsayılan app'i yerine).
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// `Idempotency-Key` başlığı (aynı anahtarla 24 saat içindeki tekrar aynı yanıtı döner).
    pub fn idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    /// Ek HTTP başlığı.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Deneme başına zaman aşımı.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// En fazla yeniden deneme sayısı.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }
}

/// Üretilmiş servislerin `Client::execute`'a verdiği istek tanımı.
pub(crate) struct Operation {
    method: &'static str,
    path: &'static str,
    path_params: Vec<(&'static str, String)>,
    query: Vec<(String, String)>,
    pub(crate) query_app_id: bool,
    pub(crate) body_app_id: bool,
    pub(crate) idempotent: bool,
    pub(crate) multipart: Option<Multipart>,
    body: Option<Value>,
}

impl Operation {
    pub(crate) fn new(method: &'static str, path: &'static str) -> Self {
        Self {
            method,
            path,
            path_params: Vec::new(),
            query: Vec::new(),
            query_app_id: false,
            body_app_id: false,
            idempotent: false,
            multipart: None,
            body: None,
        }
    }

    pub(crate) fn path_param(&mut self, name: &'static str, value: &str) {
        self.path_params.push((name, value.to_string()));
    }

    pub(crate) fn query<V: fmt::Display + ?Sized>(&mut self, name: &str, value: &V) {
        self.query.push((name.to_string(), value.to_string()));
    }

    pub(crate) fn json<B: Serialize + ?Sized>(&mut self, body: &B) -> Result<()> {
        let value = serde_json::to_value(body)
            .map_err(|e| Error::config(format!("pushizo: gövde serileştirilemedi: {e}")))?;
        self.body = Some(value);
        Ok(())
    }
}

impl Client {
    /// API anahtarıyla (`hs_live_…`) varsayılan ayarlarda istemci oluşturur.
    pub fn new(api_key: impl Into<String>) -> Result<Client> {
        Client::builder(api_key).build()
    }

    /// Ayarlanabilir istemci kurucusu.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            app_id: None,
            agent: None,
            timeout: DEFAULT_TIMEOUT,
            max_retries: DEFAULT_MAX_RETRIES,
            user_agent: format!("pushizo-rust/{VERSION}"),
            auto_idempotency_key: true,
            backoff: vec![
                Duration::from_millis(300),
                Duration::from_secs(1),
                Duration::from_secs(3),
            ],
        }
    }

    /// API adresi.
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// Varsayılan app kimliği.
    pub fn app_id(&self) -> Option<&str> {
        self.inner.app_id.as_deref()
    }

    /// SDK'da metodu olmayan uçlar için düşük seviyeli JSON isteği. `path` `/v1/…` biçimindedir; `{appId}` varsa
    /// app kimliğiyle doldurulur. Boş yanıt `Value::Null` döner.
    pub fn request(
        &self,
        method: &str,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&Value>,
        opts: &RequestOptions,
    ) -> Result<Value> {
        let data = self.request_raw(method, path, query, body, opts)?;
        decode(&data)
    }

    /// JSON dışı yanıtlar (ör. analitik `format=csv`) için ham gövde döndüren düşük seviyeli istek.
    pub fn request_raw(
        &self,
        method: &str,
        path: &str,
        query: &[(&str, &str)],
        body: Option<&Value>,
        opts: &RequestOptions,
    ) -> Result<Vec<u8>> {
        let mut op = Operation::new("GET", "");
        let method = method.to_ascii_uppercase();
        let path = path.to_string();
        for (k, v) in query {
            op.query(k, v);
        }
        op.body = body.cloned();
        self.send(&method, &path, op, opts)
    }

    pub(crate) fn execute<T: DeserializeOwned>(&self, op: Operation, opts: &RequestOptions) -> Result<T> {
        let data = self.send(op.method, op.path, op, opts)?;
        decode(&data)
    }

    pub(crate) fn execute_empty(&self, op: Operation, opts: &RequestOptions) -> Result<()> {
        self.send(op.method, op.path, op, opts).map(|_| ())
    }

    fn send(&self, method: &str, path: &str, op: Operation, opts: &RequestOptions) -> Result<Vec<u8>> {
        let inner = &self.inner;
        let app_id = opts
            .app_id
            .as_deref()
            .filter(|s| !s.is_empty())
            .or(inner.app_id.as_deref());
        let resolve_app = || {
            app_id.map(str::to_string).ok_or_else(|| {
                Error::config("pushizo: app kimliği verilmedi (ClientBuilder::app_id veya RequestOptions::app_id)")
            })
        };

        let mut path = path.to_string();
        if path.contains("{appId}") {
            path = path.replace("{appId}", &encode_component(&resolve_app()?));
        }
        for (name, value) in &op.path_params {
            if value.is_empty() {
                return Err(Error::config(format!("pushizo: \"{name}\" yol parametresi boş olamaz")));
            }
            path = path.replace(&format!("{{{name}}}"), &encode_component(value));
        }
        let mut query = op.query;
        if op.query_app_id {
            query.retain(|(k, _)| k != "app_id");
            query.push(("app_id".to_string(), resolve_app()?));
        }
        let mut url = format!("{}{}", inner.base_url, path);
        if !query.is_empty() {
            let encoded: Vec<String> = query
                .iter()
                .map(|(k, v)| format!("{}={}", encode_component(k), encode_component(v)))
                .collect();
            url.push('?');
            url.push_str(&encoded.join("&"));
        }

        let mut body_key: Option<String> = None;
        let mut payload: Option<Vec<u8>> = None;
        let mut content_type: Option<String> = None;
        if let Some(mp) = &op.multipart {
            let (data, ct) = mp.encode();
            payload = Some(data);
            content_type = Some(ct);
        } else if let Some(mut body) = op.body {
            if op.body_app_id {
                if let Some(obj) = body.as_object_mut() {
                    let missing = match obj.get("app_id") {
                        None | Some(Value::Null) => true,
                        Some(Value::String(s)) => s.is_empty(),
                        Some(_) => false,
                    };
                    if missing {
                        obj.insert("app_id".to_string(), Value::String(resolve_app()?));
                    }
                }
            }
            if op.idempotent {
                if let Some(Value::String(k)) = body.get("idempotency_key") {
                    if !k.is_empty() {
                        body_key = Some(k.clone());
                    }
                }
            }
            let data = serde_json::to_vec(&body)
                .map_err(|e| Error::config(format!("pushizo: gövde serileştirilemedi: {e}")))?;
            payload = Some(data);
            content_type = Some("application/json".to_string());
        }

        let key = opts
            .idempotency_key
            .clone()
            .filter(|k| !k.is_empty())
            .or(body_key)
            .or_else(|| (op.idempotent && inner.auto_idempotency_key).then(new_uuid));
        let has_key = key.is_some()
            || opts
                .headers
                .iter()
                .any(|(k, _)| k.eq_ignore_ascii_case("idempotency-key"));
        let max_retries = opts.max_retries.unwrap_or(inner.max_retries);
        let timeout = opts.timeout.unwrap_or(inner.timeout);
        let authorization = format!("Bearer {}", inner.api_key);

        let mut attempt: u32 = 0;
        loop {
            let mut req = inner
                .agent
                .request(method, &url)
                .timeout(timeout)
                .set("Authorization", &authorization)
                .set("Accept", "application/json")
                .set("User-Agent", &inner.user_agent);
            if let Some(ct) = &content_type {
                req = req.set("Content-Type", ct);
            }
            if let Some(k) = &key {
                req = req.set("Idempotency-Key", k);
            }
            for (name, value) in &opts.headers {
                req = req.set(name, value);
            }
            let result = match &payload {
                Some(data) => req.send_bytes(data),
                None => req.call(),
            };
            let (err, not_sent) = match result {
                Ok(resp) => {
                    let status = resp.status();
                    let request_id = resp.header("X-Request-Id").map(str::to_string);
                    let retry_after = resp.header("Retry-After").and_then(parse_retry_after);
                    match read_body(resp) {
                        Ok(data) if (200..300).contains(&status) => return Ok(data),
                        Ok(data) => (Error::from_response(status, &data, request_id, retry_after), false),
                        Err(e) => (Error::network(format!("pushizo: yanıt okunamadı: {e}")), false),
                    }
                }
                Err(ureq::Error::Status(status, resp)) => {
                    let request_id = resp.header("X-Request-Id").map(str::to_string);
                    let retry_after = resp.header("Retry-After").and_then(parse_retry_after);
                    let data = read_body(resp).unwrap_or_default();
                    (Error::from_response(status, &data, request_id, retry_after), false)
                }
                Err(ureq::Error::Transport(t)) => {
                    let not_sent = matches!(
                        t.kind(),
                        ureq::ErrorKind::Dns | ureq::ErrorKind::ConnectionFailed | ureq::ErrorKind::ProxyConnect
                    );
                    (Error::network(format!("pushizo: {t}")), not_sent)
                }
            };
            if attempt >= max_retries || !should_retry(method, &err, has_key, not_sent) {
                return Err(err);
            }
            let index = (attempt as usize).min(inner.backoff.len().saturating_sub(1));
            let delay = err
                .retry_after
                .unwrap_or_else(|| inner.backoff.get(index).copied().unwrap_or_default());
            std::thread::sleep(delay);
            attempt += 1;
        }
    }
}

/// Yeniden deneme politikası: 429 her metotta (istek işlenmeden reddedilmiştir); 5xx ve ağ hataları idempotent
/// metotlarda, POST/PATCH'te yalnızca Idempotency-Key varsa ya da istek sunucuya hiç gönderilmediyse. Aksi hâlde
/// sunucu isteği işlemiş olabilir ve tekrar çift kayıt üretebilir.
fn should_retry(method: &str, err: &Error, has_key: bool, not_sent: bool) -> bool {
    if err.status == 429 {
        return true;
    }
    if !err.is_retryable() || err.code == codes::CONFIG || err.code == codes::DECODE {
        return false;
    }
    match method {
        "GET" | "HEAD" | "OPTIONS" | "PUT" | "DELETE" => true,
        _ => has_key || not_sent,
    }
}

fn read_body(resp: ureq::Response) -> std::io::Result<Vec<u8>> {
    let mut data = Vec::new();
    resp.into_reader().read_to_end(&mut data)?;
    Ok(data)
}

fn decode<T: DeserializeOwned>(data: &[u8]) -> Result<T> {
    let text = std::str::from_utf8(data).unwrap_or("").trim();
    if text.is_empty() {
        // Gövdesiz yanıt: `null` kabul eden tipler (Option, Value) veya varsayılan nesne.
        return serde_json::from_str("null")
            .or_else(|_| serde_json::from_str("{}"))
            .map_err(|e| Error::client(codes::DECODE, format!("pushizo: boş yanıt ayrıştırılamadı: {e}")));
    }
    serde_json::from_slice(data)
        .map_err(|e| Error::client(codes::DECODE, format!("pushizo: yanıt ayrıştırılamadı: {e}")))
}

/// Saniye veya HTTP tarihi biçimindeki `Retry-After` başlığını süreye çevirir.
fn parse_retry_after(value: &str) -> Option<Duration> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if let Ok(secs) = value.parse::<f64>() {
        return (secs > 0.0 && secs.is_finite()).then(|| Duration::from_secs_f64(secs));
    }
    let at = httpdate::parse_http_date(value).ok()?;
    at.duration_since(SystemTime::now()).ok().filter(|d| !d.is_zero())
}

/// RFC 3986 "unreserved" dışındaki baytları yüzde kodlar (yol parçası ve sorgu değerleri için).
fn encode_component(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for b in value.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Rastgele (v4) UUID.
fn new_uuid() -> String {
    let hex = random_hex(16);
    let mut bytes: Vec<u8> = (0..16)
        .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap_or(0))
        .collect();
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let h: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!("{}-{}-{}-{}-{}", &h[0..8], &h[8..12], &h[12..16], &h[16..20], &h[20..32])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_v4_bicimi() {
        let id = new_uuid();
        assert_eq!(id.len(), 36);
        assert_eq!(&id[14..15], "4");
        assert!(matches!(&id[19..20], "8" | "9" | "a" | "b"));
    }

    #[test]
    fn yuzde_kodlama() {
        assert_eq!(encode_component("a b/ç"), "a%20b%2F%C3%A7");
        assert_eq!(encode_component("A-z_0.9~"), "A-z_0.9~");
    }

    #[test]
    fn retry_after_saniye() {
        assert_eq!(parse_retry_after("2"), Some(Duration::from_secs(2)));
        assert_eq!(parse_retry_after("0"), None);
        assert_eq!(parse_retry_after("x"), None);
    }

    #[test]
    fn yeniden_deneme_politikasi() {
        let server = Error::from_response(503, b"", None, None);
        assert!(should_retry("GET", &server, false, false));
        assert!(!should_retry("POST", &server, false, false));
        assert!(should_retry("POST", &server, true, false));
        let limited = Error::from_response(429, b"", None, None);
        assert!(should_retry("POST", &limited, false, false));
        let bad = Error::from_response(400, b"", None, None);
        assert!(!should_retry("GET", &bad, true, false));
        let network = Error::network("x");
        assert!(should_retry("POST", &network, false, true));
        assert!(!should_retry("POST", &network, false, false));
    }
}
