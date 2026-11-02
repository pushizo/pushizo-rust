//! Webhook imza doğrulaması ve kimlik yardımcıları.

use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use pushizo::{
    construct_webhook_events, identity_hash, verify_webhook_signature, WebhookVerifyOptions,
    WEBHOOK_REASON_MISSING_HEADER, WEBHOOK_REASON_SIGNATURE_MISMATCH,
};

const PAYLOAD: &str = r#"{"id":"whe_1","type":"user.created","created_at":"2026-10-07T00:00:00Z","app_id":"app_1","data":{"user":{"id":"usr_1"}}}"#;
const HEADER: &str = "t=1800000000,v1=6c7cf62b0759ebf076d4bc08b79a1529f7e668469ddba6136bb7ac0a70364eda";

fn at(secs: u64) -> WebhookVerifyOptions {
    WebhookVerifyOptions { tolerance: None, now: Some(UNIX_EPOCH + Duration::from_secs(secs)) }
}

#[test]
fn gecerli_imza_ve_olay() {
    let res = verify_webhook_signature(PAYLOAD.as_bytes(), Some(HEADER), &["whsec_test_secret"], &at(1_800_000_000));
    assert!(res.ok, "{:?}", res.reason);
    assert_eq!(res.timestamp, Some(1_800_000_000));
    let events = construct_webhook_events(PAYLOAD.as_bytes(), Some(HEADER), &["whsec_test_secret"], &at(1_800_000_000)).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_type, "user.created");
    assert_eq!(events[0].data["user"]["id"], "usr_1");
}

#[test]
fn hatali_secret_ve_eksik_baslik() {
    let res = verify_webhook_signature(PAYLOAD.as_bytes(), Some(HEADER), &["baska"], &at(1_800_000_000));
    assert_eq!(res.reason, Some(WEBHOOK_REASON_SIGNATURE_MISMATCH));
    let res = verify_webhook_signature::<&str>(PAYLOAD.as_bytes(), None, &[], &WebhookVerifyOptions::default());
    assert_eq!(res.reason, Some(WEBHOOK_REASON_MISSING_HEADER));
    let err = construct_webhook_events(PAYLOAD.as_bytes(), Some(HEADER), &["baska"], &at(1_800_000_000)).unwrap_err();
    assert_eq!(err.reason, WEBHOOK_REASON_SIGNATURE_MISMATCH);
}

#[test]
fn identity_hash_node_ile_ayni() {
    // Node: createHmac('sha256', 'secret').update('user-1').digest('hex')
    assert_eq!(
        identity_hash("secret", "user-1"),
        "1d16fd7e96e8a9681f283b8a251822ffdc71e2a5bda0c5267a044874fa21b82b"
    );
}

/// Ortak vektörler: ana depoda `sdk/testdata/webhook-signature-vectors.json` (Node, PHP, Python, Go, Java, .NET,
/// Ruby ile aynı). Açık depo yansıtmasında dosya yoktur; o zaman yukarıdaki testler yeterlidir.
#[test]
fn ortak_vektorler() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../testdata/webhook-signature-vectors.json");
    let Ok(raw) = std::fs::read_to_string(&file) else {
        eprintln!("ortak vektör dosyası yok ({}); atlandı", file.display());
        return;
    };
    let doc: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let cases = doc["cases"].as_array().unwrap();
    assert!(!cases.is_empty());
    for c in cases {
        let name = c["name"].as_str().unwrap();
        let payload = c["payload"].as_str().unwrap().as_bytes();
        let header = c["header"].as_str();
        let secrets: Vec<&str> = c["secrets"].as_array().unwrap().iter().map(|s| s.as_str().unwrap()).collect();
        let options = WebhookVerifyOptions {
            tolerance: Some(Duration::from_secs(c["toleranceSeconds"].as_u64().unwrap())),
            now: Some(SystemTime::UNIX_EPOCH + Duration::from_secs(c["now"].as_u64().unwrap())),
        };
        let expected_ok = c["expected"]["ok"].as_bool().unwrap();
        let res = verify_webhook_signature(payload, header, &secrets, &options);
        assert_eq!(res.ok, expected_ok, "{name}: {:?}", res.reason);
        if expected_ok {
            assert_eq!(res.timestamp, c["expected"]["timestamp"].as_i64(), "{name}");
            let events = construct_webhook_events(payload, header, &secrets, &options)
                .unwrap_or_else(|e| panic!("{name}: {e}"));
            if let Some(n) = c["events"].as_u64() {
                assert_eq!(events.len() as u64, n, "{name}");
            }
        } else {
            let reason = c["expected"]["reason"].as_str().unwrap();
            assert_eq!(res.reason, Some(reason), "{name}");
            let err = construct_webhook_events(payload, header, &secrets, &options).unwrap_err();
            assert_eq!(err.reason, reason, "{name}");
        }
    }
}
