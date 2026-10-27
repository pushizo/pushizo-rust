//! Pushizo public API istemcisi. Anahtar hata metnine yazılmaz.

pub struct Client {
    api_url: String,
    api_key: String,
    app_id: String,
}

impl Client {
    pub fn new(api_url: impl Into<String>, api_key: impl Into<String>, app_id: impl Into<String>) -> Self {
        Self { api_url: api_url.into().trim_end_matches('/').to_string(), api_key: api_key.into(), app_id: app_id.into() }
    }

    pub fn send(&self, title: &str, body: &str) -> Result<u16, String> {
        let payload = format!(
            r#"{{"app_id":"{}","target":{{"included_segments":["Subscribed Users"]}},"headings":{{"en":"{}"}},"contents":{{"en":"{}"}}}}"#,
            escape(&self.app_id),
            escape(title),
            escape(body)
        );
        let response = ureq::post(&format!("{}/v1/notifications", self.api_url))
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_string(&payload)
            .map_err(|err| err.to_string())?;
        Ok(response.status())
    }
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
