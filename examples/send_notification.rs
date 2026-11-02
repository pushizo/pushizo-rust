//! Bildirim gönderme ve segmentleri listeleme örneği.
//!
//! ```bash
//! PUSHIZO_API_KEY=hs_live_… PUSHIZO_APP_ID=app_… cargo run --example send_notification -- user-42
//! ```

use std::collections::HashMap;

use pushizo::models::{CreateNotificationDto, CreateNotificationDtoTarget};
use pushizo::{Client, ErrorKind, RequestOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("PUSHIZO_API_KEY").map_err(|_| "PUSHIZO_API_KEY is required")?;
    let app_id = std::env::var("PUSHIZO_APP_ID").map_err(|_| "PUSHIZO_APP_ID is required")?;
    let external_id = std::env::args().nth(1).unwrap_or_else(|| "user-42".to_string());

    let mut builder = Client::builder(api_key).app_id(app_id);
    if let Ok(url) = std::env::var("PUSHIZO_API_URL") {
        builder = builder.base_url(url);
    }
    let client = builder.build()?;

    let mut target = CreateNotificationDtoTarget::default();
    target.include_aliases = Some(HashMap::from([("external_id".to_string(), vec![external_id.clone()])]));
    let body = CreateNotificationDto {
        target,
        headings: Some(HashMap::from([
            ("en".to_string(), "Your order shipped".to_string()),
            ("tr".to_string(), "Siparişiniz kargoya verildi".to_string()),
        ])),
        contents: Some(HashMap::from([
            ("en".to_string(), "Track it in the app.".to_string()),
            ("tr".to_string(), "Uygulamadan takip edin.".to_string()),
        ])),
        ..Default::default()
    };

    // Aynı Idempotency-Key ile tekrar gönderim yeni bildirim üretmez.
    match client
        .notifications()
        .with_options(RequestOptions::new().idempotency_key(format!("example-{external_id}")))
        .create(&body)
    {
        Ok(result) => println!("notification {} ({}), ~{} recipients", result.id, result.status, result.recipients_estimate),
        Err(err) if err.kind() == ErrorKind::RateLimited => eprintln!("rate limited, retry after {:?}", err.retry_after),
        Err(err) => return Err(err.into()),
    }

    for segment in client.segments().list_all(None).take(20) {
        let segment = segment?;
        println!("segment {} {}", segment.id, segment.name);
    }
    Ok(())
}
