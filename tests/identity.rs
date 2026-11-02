//! ES256 SDK JWT'si: imza ve talepler.

use std::time::{Duration, UNIX_EPOCH};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use p256::ecdsa::signature::Verifier;
use p256::ecdsa::{Signature, SigningKey, VerifyingKey};
use p256::pkcs8::{EncodePrivateKey, LineEnding};
use pushizo::{identity_token, ErrorKind, IdentityTokenOptions};

/// Test anahtarı çalışma anında sabit skalerden üretilir (depoda PEM özel anahtarı tutulmaz).
fn test_key() -> (String, VerifyingKey) {
    let secret = p256::SecretKey::from_bytes(&p256::FieldBytes::clone_from_slice(&[7u8; 32])).unwrap();
    let pem = secret.to_pkcs8_pem(LineEnding::LF).unwrap();
    let verifying = VerifyingKey::from(&SigningKey::from(&secret));
    (pem.as_str().to_owned(), verifying)
}

#[test]
fn imza_ve_talepler() {
    let (pem, verifying) = test_key();
    let token = identity_token(&IdentityTokenOptions {
        private_key_pem: pem,
        kid: "main".into(),
        app_id: "app_1".into(),
        external_id: "kullanici-1".into(),
        ttl: Some(Duration::from_secs(600)),
        now: Some(UNIX_EPOCH + Duration::from_secs(1_800_000_000)),
    })
    .unwrap();
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3);
    let header: serde_json::Value = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(parts[0]).unwrap()).unwrap();
    assert_eq!(header["alg"], "ES256");
    assert_eq!(header["kid"], "main");
    let payload: serde_json::Value = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(parts[1]).unwrap()).unwrap();
    assert_eq!(payload["sub"], "kullanici-1");
    assert_eq!(payload["aud"], "app_1");
    assert_eq!(payload["iat"], 1_800_000_000u64);
    assert_eq!(payload["exp"], 1_800_000_600u64);
    let signature = Signature::from_slice(&URL_SAFE_NO_PAD.decode(parts[2]).unwrap()).unwrap();
    let signing_input = format!("{}.{}", parts[0], parts[1]);
    verifying.verify(signing_input.as_bytes(), &signature).unwrap();
}

#[test]
fn gecersiz_ttl_ve_anahtar() {
    let (pem, _) = test_key();
    let err = identity_token(&IdentityTokenOptions {
        private_key_pem: pem,
        ttl: Some(Duration::from_secs(25 * 3600)),
        ..Default::default()
    })
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Config);
    let err = identity_token(&IdentityTokenOptions { private_key_pem: "bozuk".into(), ..Default::default() }).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Config);
}
