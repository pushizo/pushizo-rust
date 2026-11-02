//! İstemci davranışı: kimlik doğrulama, app kapsamı, idempotency, yeniden deneme, hata eşleme ve sayfalama.
//! Yerel bir TCP sunucusu sırayla hazır yanıtlar döner ve gelen istekleri kaydeder.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::thread::JoinHandle;
use std::time::Duration;

use pushizo::models::{CreateNotificationDto, CreateNotificationDtoTarget};
use pushizo::resources::SegmentsListParams;
use pushizo::{Client, ErrorKind, RequestOptions};

#[derive(Debug)]
struct Recorded {
    method: String,
    target: String,
    headers: HashMap<String, String>,
    body: String,
}

struct Reply {
    status: u16,
    headers: Vec<(&'static str, String)>,
    body: String,
}

fn reply(status: u16, body: &str) -> Reply {
    Reply { status, headers: Vec::new(), body: body.to_string() }
}

/// Her bağlantıya sıradaki yanıtı verir; tüm yanıtlar bitince kayıtları döndürür.
fn serve(replies: Vec<Reply>) -> (String, JoinHandle<Vec<Recorded>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let handle = std::thread::spawn(move || {
        let mut recorded = Vec::new();
        for r in replies {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let mut parts = line.split_whitespace();
            let method = parts.next().unwrap_or_default().to_string();
            let target = parts.next().unwrap_or_default().to_string();
            let mut headers = HashMap::new();
            loop {
                let mut h = String::new();
                reader.read_line(&mut h).unwrap();
                let h = h.trim_end();
                if h.is_empty() {
                    break;
                }
                if let Some((k, v)) = h.split_once(':') {
                    headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
                }
            }
            let len: usize = headers.get("content-length").and_then(|v| v.parse().ok()).unwrap_or(0);
            let mut body = vec![0u8; len];
            reader.read_exact(&mut body).unwrap();
            recorded.push(Recorded { method, target, headers, body: String::from_utf8_lossy(&body).into_owned() });
            let mut stream = reader.into_inner();
            let mut head = format!(
                "HTTP/1.1 {} Status\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
                r.status,
                r.body.len()
            );
            for (k, v) in &r.headers {
                head.push_str(&format!("{k}: {v}\r\n"));
            }
            head.push_str("\r\n");
            stream.write_all(head.as_bytes()).unwrap();
            stream.write_all(r.body.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        recorded
    });
    (base, handle)
}

fn client(base: &str) -> Client {
    Client::builder("hs_test_key")
        .base_url(base)
        .app_id("app_1")
        .backoff(vec![Duration::from_millis(1)])
        .build()
        .unwrap()
}

#[test]
fn bildirim_olusturma_app_ve_idempotency() {
    let (base, server) = serve(vec![reply(202, r#"{"id":"msg_1","status":"queued","recipients_estimate":3,"external_id_missing":[]}"#)]);
    let mut target = CreateNotificationDtoTarget::default();
    target.include_aliases = Some(HashMap::from([("external_id".to_string(), vec!["u1".to_string()])]));
    let body = CreateNotificationDto {
        target,
        contents: Some(HashMap::from([("en".to_string(), "Hi".to_string())])),
        ..Default::default()
    };
    let res = client(&base).notifications().create(&body).unwrap();
    assert_eq!(res.id, "msg_1");
    assert_eq!(res.recipients_estimate, 3);

    let rec = server.join().unwrap();
    assert_eq!(rec[0].method, "POST");
    assert_eq!(rec[0].target, "/v1/notifications");
    assert_eq!(rec[0].headers["authorization"], "Bearer hs_test_key");
    assert!(rec[0].headers["user-agent"].starts_with("pushizo-rust/"));
    assert_eq!(rec[0].headers["idempotency-key"].len(), 36, "otomatik UUID");
    let sent: serde_json::Value = serde_json::from_str(&rec[0].body).unwrap();
    assert_eq!(sent["app_id"], "app_1");
    assert_eq!(sent["target"]["include_aliases"]["external_id"][0], "u1");
    assert!(sent.get("headings").is_none(), "boş isteğe bağlı alan gönderilmez");
}

#[test]
fn govdedeki_idempotency_key_ve_app_onceligi() {
    let (base, server) = serve(vec![reply(202, r#"{"id":"msg_2","status":"queued"}"#)]);
    let body = CreateNotificationDto {
        app_id: Some("app_9".into()),
        idempotency_key: Some("order-7".into()),
        ..Default::default()
    };
    client(&base).notifications().create(&body).unwrap();
    let rec = server.join().unwrap();
    assert_eq!(rec[0].headers["idempotency-key"], "order-7");
    let sent: serde_json::Value = serde_json::from_str(&rec[0].body).unwrap();
    assert_eq!(sent["app_id"], "app_9");
}

#[test]
fn get_yeniden_denenir_ve_sorgu_kodlanir() {
    let mut busy = reply(503, r#"{"error":{"code":"HS_SERVICE_UNAVAILABLE","message":"busy"}}"#);
    busy.headers.push(("Retry-After", "0".into()));
    let (base, server) = serve(vec![
        busy,
        reply(200, r#"{"data":[{"id":"seg_1","name":"VIP"}],"next_cursor":null,"has_more":false}"#),
    ]);
    let params = SegmentsListParams { limit: Some(10), ..Default::default() };
    let page = client(&base)
        .segments()
        .with_options(RequestOptions::new().app_id("app 2"))
        .list(Some(&params))
        .unwrap();
    assert_eq!(page.data.len(), 1);
    let rec = server.join().unwrap();
    assert_eq!(rec.len(), 2);
    assert_eq!(rec[1].target, "/v1/apps/app%202/segments?limit=10");
}

#[test]
fn hata_zarfi_eslenir_ve_404_yeniden_denenmez() {
    let mut not_found = reply(404, r#"{"error":{"code":"HS_NOT_FOUND","message":"yok","request_id":"req_1"}}"#);
    not_found.headers.push(("X-Request-Id", "req_header".into()));
    let (base, server) = serve(vec![not_found]);
    let err = client(&base).segments().get("seg_x").unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NotFound);
    assert_eq!(err.code, "HS_NOT_FOUND");
    assert_eq!(err.request_id.as_deref(), Some("req_1"));
    assert!(!err.to_string().contains("hs_test_key"), "anahtar hata metnine yazılmaz");
    assert_eq!(server.join().unwrap().len(), 1);
}

#[test]
fn sayfalama_tum_sayfalari_dolasir() {
    let (base, server) = serve(vec![
        reply(200, r#"{"data":[{"id":"seg_1"},{"id":"seg_2"}],"next_cursor":"c2","has_more":true}"#),
        reply(200, r#"{"data":[{"id":"seg_3"}],"next_cursor":null,"has_more":false}"#),
    ]);
    let c = client(&base);
    let ids: Vec<String> = c.segments().list_all(None).map(|s| s.unwrap().id).collect();
    assert_eq!(ids, vec!["seg_1", "seg_2", "seg_3"]);
    let rec = server.join().unwrap();
    assert_eq!(rec[1].target, "/v1/apps/app_1/segments?cursor=c2");
}

#[test]
fn app_kimligi_yoksa_istek_gonderilmez() {
    let c = Client::builder("hs_test_key").base_url("http://127.0.0.1:9").build().unwrap();
    let err = c.segments().list(None).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Config);
    assert!(Client::new("").is_err());
    assert!(!format!("{c:?}").contains("hs_test_key"));
}
