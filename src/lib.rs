//! Pushizo public API'si için Rust sunucu SDK'sı.
//!
//! ```no_run
//! use pushizo::models::{CreateNotificationDto, CreateNotificationDtoTarget};
//! use pushizo::{Client, RequestOptions};
//! use std::collections::HashMap;
//!
//! # fn main() -> pushizo::Result<()> {
//! let client = Client::builder(std::env::var("PUSHIZO_API_KEY").unwrap_or_default())
//!     .app_id("app_…")
//!     .build()?;
//!
//! let mut target = CreateNotificationDtoTarget::default();
//! target.include_aliases = Some(HashMap::from([(
//!     "external_id".to_string(),
//!     vec!["user-42".to_string()],
//! )]));
//! let body = CreateNotificationDto {
//!     target,
//!     contents: Some(HashMap::from([("en".to_string(), "Your order shipped".to_string())])),
//!     ..Default::default()
//! };
//! let result = client
//!     .notifications()
//!     .with_options(RequestOptions::new().idempotency_key("order-1001"))
//!     .create(&body)?;
//! println!("{}", result.id);
//! # Ok(())
//! # }
//! ```
//!
//! Modeller (`models`) ve kaynak servisleri (`resources`) `api/openapi.yaml`'dan üretilir; HTTP katmanı, yeniden
//! deneme, idempotency, sayfalama, multipart, webhook imza doğrulaması ve kimlik doğrulama yardımcıları el yazımıdır.
//! API anahtarı hata metinlerine ve `Debug` çıktısına yazılmaz.

#![forbid(unsafe_code)]

mod client;
mod de;
mod error;
mod identity;
mod multipart;
mod pagination;
mod webhooks;

#[path = "generated/models.rs"]
pub mod models;
#[path = "generated/resources.rs"]
pub mod resources;

pub use client::{Client, ClientBuilder, RequestOptions, DEFAULT_BASE_URL, VERSION};
pub use error::{codes, Error, ErrorKind, Result};
pub use identity::{
    identity_hash, identity_token, IdentityTokenOptions, IDENTITY_TOKEN_MAX_LIFETIME,
};
pub use multipart::FileUpload;
pub use pagination::Paginator;
pub use resources::API_VERSION;
pub use webhooks::{
    construct_webhook_events, verify_webhook_signature, WebhookEvent, WebhookSignatureError,
    WebhookVerifyOptions, WebhookVerifyResult, WEBHOOK_DEFAULT_TOLERANCE,
    WEBHOOK_EVENT_ID_HEADER, WEBHOOK_REASON_MALFORMED_BODY, WEBHOOK_REASON_MALFORMED_HEADER,
    WEBHOOK_REASON_MISSING_HEADER, WEBHOOK_REASON_SIGNATURE_MISMATCH,
    WEBHOOK_REASON_TIMESTAMP_OUT_OF_TOLERANCE, WEBHOOK_SIGNATURE_HEADER,
};
