<!-- pushizo:header -->
<p align="center">
  <a href="https://pushizo.com"><img src="https://raw.githubusercontent.com/pushizo/.github/main/profile/assets/mark.svg" alt="Pushizo" width="56" height="56"></a>
</p>
<p align="center">
  <a href="https://docs.pushizo.com/docs/sdk/rust"><img src="https://img.shields.io/badge/docs-docs.pushizo.com-5b4bdb" alt="Docs"></a>
  <a href="https://docs.pushizo.com/docs/api"><img src="https://img.shields.io/badge/API-reference-b44fc4" alt="API reference"></a>
  <a href="https://github.com/pushizo/pushizo-rust/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-16130f" alt="License: MIT"></a>
</p>
<p align="center"><a href="#english">English</a> · <a href="#türkçe">Türkçe</a></p>

## English

**Pushizo Rust SDK** — Server-side SDK for Rust 1.71+ (blocking, ureq). Models and services are generated from the Pushizo OpenAPI schema and cover the same endpoints as the other server SDKs; authentication, retries, idempotency, cursor pagination, multipart uploads, webhook signature verification and identity verification helpers are hand-written.

```bash
cargo add pushizo
```

The detailed guide below is in Turkish; code samples are language-neutral. Full documentation:
[docs.pushizo.com/docs/sdk/rust](https://docs.pushizo.com/docs/sdk/rust) · API reference: [docs.pushizo.com/docs/api](https://docs.pushizo.com/docs/api).

## Türkçe
<!-- /pushizo:header -->

# Pushizo Rust SDK

Pushizo public API'si için Rust sunucu SDK'sı (Rust 1.71+, senkron; HTTP için `ureq`).
Modeller (`pushizo::models`) ve kaynak servisleri (`pushizo::resources`) [Pushizo OpenAPI şemasından](https://github.com/pushizo/pushizo-openapi)
üretilir ve diğer sunucu SDK'larıyla (Node, Python, Go, Java, .NET, Ruby, PHP) aynı uçları kapsar; kimlik doğrulama,
yeniden deneme, idempotency, sayfalama, multipart ve webhook imza doğrulaması el yazımıdır.

## Kurulum

```bash
cargo add pushizo
```

## Kullanım

```rust
use std::collections::HashMap;

use pushizo::models::{CreateNotificationDto, CreateNotificationDtoTarget, UpsertUserDto};
use pushizo::resources::SegmentsListParams;
use pushizo::{Client, ErrorKind, RequestOptions};

fn main() -> pushizo::Result<()> {
    let client = Client::builder(std::env::var("PUSHIZO_API_KEY").unwrap_or_default())
        .app_id("app_…")
        // .base_url("http://localhost:4000")
        .build()?;

    // Bildirim gönder: Idempotency-Key verilmezse otomatik UUID (yeniden denemelerde aynı anahtar)
    let mut target = CreateNotificationDtoTarget::default();
    target.include_aliases = Some(HashMap::from([("external_id".into(), vec!["user-42".into()])]));
    let body = CreateNotificationDto {
        target,
        contents: Some(HashMap::from([
            ("en".into(), "Your order shipped".into()),
            ("tr".into(), "Siparişiniz kargoya verildi".into()),
        ])),
        thread_id: Some("orders".into()),
        ..Default::default()
    };
    let res = client
        .notifications()
        .with_options(RequestOptions::new().idempotency_key("order-1001"))
        .create(&body)?;
    println!("{} {}", res.id, res.status);

    // Kullanıcı oluştur/güncelle
    client.users().upsert(&UpsertUserDto {
        external_id: Some("user-42".into()),
        tags: Some(HashMap::from([("plan".into(), "pro".into())])),
        ..Default::default()
    })?;

    // Cursor sayfalama: tüm sayfalar tek yineleyici
    let params = SegmentsListParams { limit: Some(100), ..Default::default() };
    for segment in client.segments().list_all(Some(&params)) {
        println!("{}", segment?.name);
    }

    // Hata kategorileri
    match client.segments().get("seg_yok") {
        Err(e) if e.kind() == ErrorKind::NotFound => println!("yok: {}", e.request_id.unwrap_or_default()),
        other => println!("{other:?}"),
    }
    Ok(())
}
```

Çalıştırılabilir örnek: `examples/send_notification.rs`
(`PUSHIZO_API_KEY=hs_live_… PUSHIZO_APP_ID=app_… cargo run --example send_notification -- user-42`).

## İstemci ve çağrı ayarları

| Ayar | Açıklama |
|------|----------|
| `ClientBuilder::app_id` | Varsayılan app; `{appId}` yolları, `app_id` sorgusu ve gövdedeki boş `app_id` bununla dolar |
| `ClientBuilder::base_url` | API adresi (varsayılan `https://api.pushizo.com`) |
| `ClientBuilder::timeout` | Deneme başına zaman aşımı (varsayılan 30 sn) |
| `ClientBuilder::max_retries` | En fazla yeniden deneme (varsayılan 3) |
| `ClientBuilder::agent` | Özel `ureq::Agent` (proxy, TLS) |
| `RequestOptions::app_id` | Tek servis çağrısı için farklı app |
| `RequestOptions::idempotency_key` | `Idempotency-Key` başlığı (aynı anahtarla tekrar aynı yanıtı döner) |
| `RequestOptions::header`, `timeout`, `max_retries` | Çağrı başına ek başlık ve sınırlar |

Ayarlar servis üzerinde verilir: `client.segments().with_options(RequestOptions::new().app_id("app_2")).list(None)`.

**Yeniden deneme:** 429 her metotta; 5xx ve ağ hataları GET/PUT/DELETE'te, POST/PATCH'te yalnızca
`Idempotency-Key` varsa veya istek sunucuya hiç ulaşmadıysa yeniden denenir. `Retry-After` başlığına uyulur.

**Hatalar:** `pushizo::Error` (`status`, `code`, `message`, `request_id`, `details`, `retry_after`) ve `kind()`
(`Network`, `InvalidRequest`, `Authentication`, `PermissionDenied`, `NotFound`, `Conflict`, `RateLimited`,
`Server`, `Decode`, `Config`). API anahtarı hata metnine ve `Debug` çıktısına yazılmaz.

**Düşük seviyeli istek:** SDK'da metodu olmayan uçlar veya CSV gibi JSON dışı yanıtlar için
`client.request(...)` ve `client.request_raw(...)`.

## Webhook imza doğrulaması

```rust
use pushizo::{construct_webhook_events, WebhookVerifyOptions, WEBHOOK_SIGNATURE_HEADER};

fn handle(raw_body: &[u8], signature: Option<&str>, secret: &str) -> Result<(), pushizo::WebhookSignatureError> {
    // raw_body: HAM istek gövdesi; signature: `Pushizo-Signature` başlığı (WEBHOOK_SIGNATURE_HEADER)
    let events = construct_webhook_events(raw_body, signature, &[secret], &WebhookVerifyOptions::default())?;
    for event in events {
        println!("{} {}", event.event_type, event.id);
    }
    Ok(())
}
```

Rotasyon sırasında eski ve yeni secret birlikte verilebilir. Zaman damgası toleransı varsayılan 5 dakikadır.

## Kimlik doğrulama (identity verification)

```rust
// HMAC yöntemi: istemci SDK'sına identity_hash verilir; sdk_secret istemciye gönderilmez.
let hash = pushizo::identity_hash(&std::env::var("PUSHIZO_SDK_SECRET").unwrap_or_default(), "user-42");

// JWT yöntemi (ES256): uygulamanızın sunucusu belirteci üretir, istemci SDK'sı login(externalId, jwt) ile kullanır.
let token = pushizo::identity_token(&pushizo::IdentityTokenOptions {
    private_key_pem: std::fs::read_to_string("identity-key.pem").unwrap_or_default(),
    kid: "main".into(),
    app_id: "app_…".into(),
    external_id: "user-42".into(),
    ..Default::default()
});
```

## Geliştirme

```bash
cargo test
cargo package --allow-dirty --no-verify
```

Üretilmiş dosyalar (`src/generated/`) ana depoda `npm run sdk:generate` ile yenilenir; elle düzenlenmez.

<!-- pushizo:footer -->
---

## Support · Destek

- **Guide · Rehber:** [docs.pushizo.com/docs/sdk/rust](https://docs.pushizo.com/docs/sdk/rust)
- **API reference & OpenAPI · API referansı ve OpenAPI:** [docs.pushizo.com/docs/api](https://docs.pushizo.com/docs/api) ·
  [pushizo-openapi](https://github.com/pushizo/pushizo-openapi)
- **Bugs & ideas · Hata ve öneri:** [Issues](https://github.com/pushizo/pushizo-rust/issues)
- **Security · Güvenlik:** please do not open public issues; email [security@pushizo.com](mailto:security@pushizo.com) ·
  herkese açık issue açmayın, bu adrese yazın.
- **Help · Destek:** [support@pushizo.com](mailto:support@pushizo.com)

This repository is mirrored automatically from the Pushizo main repository on every release. Please open an issue
instead of a pull request; fixes land in the main repository and ship with the next release.
Bu depo her sürümde Pushizo ana deposundan otomatik yansıtılır. Pull request yerine issue açın; düzeltmeler ana depoda
yapılır ve bir sonraki sürümle buraya gelir.

Released under the MIT License · MIT lisanslıdır — [LICENSE](https://github.com/pushizo/pushizo-rust/blob/main/LICENSE).
<!-- /pushizo:footer -->
