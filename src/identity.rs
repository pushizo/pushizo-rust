//! Kimlik doğrulama yardımcıları: `identity_hash` (HMAC) ve ES256 SDK JWT'si (ADR-0023 §9).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use hmac::{Hmac, Mac};
use p256::ecdsa::signature::Signer;
use p256::ecdsa::{Signature, SigningKey};
use p256::pkcs8::DecodePrivateKey;
use sha2::Sha256;

use crate::error::{Error, Result};

/// SDK JWT'sinin en uzun ömrü.
pub const IDENTITY_TOKEN_MAX_LIFETIME: Duration = Duration::from_secs(24 * 60 * 60);

/// SDK identity verification için `identity_hash = HMAC-SHA256(sdk_secret, external_id)` (hex) üretir.
/// Sunucu tarafında hesaplanıp istemci SDK'sına iletilir; `sdk_secret` istemciye gönderilmez.
pub fn identity_hash(sdk_secret: &str, external_id: &str) -> String {
    let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(sdk_secret.as_bytes())
        .expect("HMAC her uzunlukta anahtarı kabul eder");
    mac.update(external_id.as_bytes());
    mac.finalize().into_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

/// `identity_token` parametreleri.
#[derive(Clone, Default)]
pub struct IdentityTokenOptions {
    /// ES256 (P-256) özel anahtarı, PKCS#8 (`BEGIN PRIVATE KEY`) veya SEC1 (`BEGIN EC PRIVATE KEY`) PEM.
    pub private_key_pem: String,
    /// Panelde/API'de kayıtlı genel anahtarın kimliği.
    pub kid: String,
    pub app_id: String,
    pub external_id: String,
    /// Belirteç ömrü (varsayılan 1 saat, en çok 24 saat).
    pub ttl: Option<Duration>,
    /// Testler için sabit zaman.
    pub now: Option<SystemTime>,
}

impl std::fmt::Debug for IdentityTokenOptions {
    // Özel anahtar log'lara yazılmaz.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdentityTokenOptions")
            .field("kid", &self.kid)
            .field("app_id", &self.app_id)
            .field("external_id", &self.external_id)
            .field("ttl", &self.ttl)
            .finish_non_exhaustive()
    }
}

/// İstemci SDK'sına verilecek ES256 JWT'yi üretir: `sub = external_id`, `aud = app_id`. Uygulamanız belirteci kendi
/// sunucusundan alır ve SDK'nın `login(externalId, jwt)` çağrısına verir.
pub fn identity_token(options: &IdentityTokenOptions) -> Result<String> {
    let ttl = options.ttl.unwrap_or(Duration::from_secs(3600));
    if ttl < Duration::from_secs(1) || ttl > IDENTITY_TOKEN_MAX_LIFETIME {
        return Err(Error::config("pushizo: TTL 1 sn … 24 saat aralığında olmalı"));
    }
    let key = parse_signing_key(&options.private_key_pem)?;
    let now = options
        .now
        .unwrap_or_else(SystemTime::now)
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::config("pushizo: geçersiz zaman"))?
        .as_secs();
    let header = serde_json::json!({ "alg": "ES256", "typ": "JWT", "kid": options.kid });
    let payload = serde_json::json!({
        "sub": options.external_id,
        "aud": options.app_id,
        "iat": now,
        "exp": now + ttl.as_secs(),
    });
    let signing_input = format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(header.to_string()),
        URL_SAFE_NO_PAD.encode(payload.to_string())
    );
    // JWS ES256 imzası DER değil r||s (her biri 32 bayt).
    let signature: Signature = key.sign(signing_input.as_bytes());
    Ok(format!("{}.{}", signing_input, URL_SAFE_NO_PAD.encode(signature.to_bytes())))
}

fn parse_signing_key(pem: &str) -> Result<SigningKey> {
    let invalid = || Error::config("pushizo: ES256 için P-256 EC özel anahtarı (PEM) gerekli");
    let pem = pem.trim();
    if pem.contains("BEGIN EC PRIVATE KEY") {
        let secret = p256::SecretKey::from_sec1_pem(pem).map_err(|_| invalid())?;
        return Ok(SigningKey::from(&secret));
    }
    let secret = p256::SecretKey::from_pkcs8_pem(pem).map_err(|_| invalid())?;
    Ok(SigningKey::from(&secret))
}
