// Bu dosya api/openapi.yaml'dan sdk/scripts/generate-server-sdks.mjs ile üretilmiştir; elle düzenlemeyin.

//! API modelleri (istek gövdeleri ve yanıtlar).

#![allow(clippy::all, missing_docs)]

/// `AdAccount` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAccount {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu. Değerler: `"meta"`, `"google"`, `"tiktok"`, `"linkedin"`.
    #[serde(rename = "platform", deserialize_with = "crate::de::null_default")]
    pub platform: String,
    /// Zorunlu.
    #[serde(rename = "external_id", deserialize_with = "crate::de::null_default")]
    pub external_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Zorunlu.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// Zorunlu. Değerler: `"active"`, `"error"`, `"revoked"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "scopes", deserialize_with = "crate::de::null_default")]
    pub scopes: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "login_customer_id")]
    pub login_customer_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "token_expires_at")]
    pub token_expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_refreshed_at")]
    pub last_refreshed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_error_at")]
    pub last_error_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "terms_accepted_at")]
    pub terms_accepted_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "terms_version")]
    pub terms_version: Option<String>,
    /// Zorunlu.
    #[serde(rename = "audiences", deserialize_with = "crate::de::null_default")]
    pub audiences: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `AdAccountList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAccountList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AdAccount>,
}

/// `AdAttributionReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAttributionReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu. Değerler: `"last_click"`, `"first_click"`, `"linear"`, `"time_decay"`.
    #[serde(rename = "model", deserialize_with = "crate::de::null_default")]
    pub model: String,
    /// Zorunlu.
    #[serde(rename = "window_days", deserialize_with = "crate::de::null_default")]
    pub window_days: i64,
    /// Zorunlu.
    #[serde(rename = "half_life_days", deserialize_with = "crate::de::null_default")]
    pub half_life_days: i64,
    /// Zorunlu.
    #[serde(rename = "currency", deserialize_with = "crate::de::null_default")]
    pub currency: String,
    /// Zorunlu.
    #[serde(rename = "group_by", deserialize_with = "crate::de::null_default")]
    pub group_by: String,
    /// Zorunlu.
    #[serde(rename = "conversion_events", deserialize_with = "crate::de::null_default")]
    pub conversion_events: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<AdAttributionReportRowsItem>,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: AdAttributionReportTotals,
    /// Zorunlu.
    #[serde(rename = "truncated", deserialize_with = "crate::de::null_default")]
    pub truncated: bool,
    /// Zorunlu.
    #[serde(rename = "fx_missing", deserialize_with = "crate::de::null_default")]
    pub fx_missing: Vec<String>,
}

/// `AdAttributionReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAttributionReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu. Değerler: `"meta"`, `"google"`, `"tiktok"`, `"linkedin"`, `"microsoft"`, `"other_paid"`, `"utm"`, `"push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"direct"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
    /// Zorunlu.
    #[serde(rename = "paid", deserialize_with = "crate::de::null_default")]
    pub paid: bool,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: f64,
    /// Zorunlu.
    #[serde(rename = "revenue", deserialize_with = "crate::de::null_default")]
    pub revenue: f64,
    /// Zorunlu.
    #[serde(rename = "new_customers", deserialize_with = "crate::de::null_default")]
    pub new_customers: f64,
    /// Zorunlu.
    #[serde(rename = "spend")]
    pub spend: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "roas")]
    pub roas: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "cac")]
    pub cac: Option<f64>,
}

/// `AdAttributionReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAttributionReportTotals {
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: f64,
    /// Zorunlu.
    #[serde(rename = "revenue", deserialize_with = "crate::de::null_default")]
    pub revenue: f64,
    /// Zorunlu.
    #[serde(rename = "spend", deserialize_with = "crate::de::null_default")]
    pub spend: f64,
    /// Zorunlu.
    #[serde(rename = "new_customers", deserialize_with = "crate::de::null_default")]
    pub new_customers: f64,
    /// Zorunlu.
    #[serde(rename = "roas")]
    pub roas: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "platform_roas")]
    pub platform_roas: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "cac")]
    pub cac: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "paid_cac")]
    pub paid_cac: Option<f64>,
}

/// `AdAudience` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAudience {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "ad_account_id", deserialize_with = "crate::de::null_default")]
    pub ad_account_id: String,
    /// Zorunlu. Değerler: `"meta"`, `"google"`, `"tiktok"`, `"linkedin"`.
    #[serde(rename = "platform", deserialize_with = "crate::de::null_default")]
    pub platform: String,
    /// Zorunlu. Değerler: `"segment"`, `"journey"`, `"lookalike"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu.
    #[serde(rename = "segment_id")]
    pub segment_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "segment_name")]
    pub segment_name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu. Değerler: `"pending"`, `"active"`, `"paused"`, `"error"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu. Değerler: `"email"`, `"phone"`, `"maid"`.
    #[serde(rename = "identifiers", deserialize_with = "crate::de::null_default")]
    pub identifiers: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "sync_interval_minutes", deserialize_with = "crate::de::null_default")]
    pub sync_interval_minutes: i64,
    /// Zorunlu.
    #[serde(rename = "next_sync_at")]
    pub next_sync_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_synced_at")]
    pub last_synced_at: Option<String>,
    /// Zorunlu. Değerler: `"succeeded"`, `"partial"`, `"failed"`.
    #[serde(rename = "last_sync_status")]
    pub last_sync_status: Option<String>,
    /// Zorunlu.
    #[serde(rename = "full_reload_requested", deserialize_with = "crate::de::null_default")]
    pub full_reload_requested: bool,
    /// Zorunlu.
    #[serde(rename = "member_count", deserialize_with = "crate::de::null_default")]
    pub member_count: i64,
    /// Zorunlu.
    #[serde(rename = "estimated_size")]
    pub estimated_size: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "match_rate")]
    pub match_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "size_updated_at")]
    pub size_updated_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "lookalike_source_id")]
    pub lookalike_source_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "lookalike_spec")]
    pub lookalike_spec: Option<AdAudienceLookalikeSpec>,
    /// Zorunlu.
    #[serde(rename = "consecutive_failures", deserialize_with = "crate::de::null_default")]
    pub consecutive_failures: i64,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_error_at")]
    pub last_error_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `AdAudienceList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAudienceList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AdAudience>,
}

/// `AdAudienceLookalikeSpec` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAudienceLookalikeSpec {
    /// Zorunlu.
    #[serde(rename = "country", deserialize_with = "crate::de::null_default")]
    pub country: String,
    /// Zorunlu.
    #[serde(rename = "ratio", deserialize_with = "crate::de::null_default")]
    pub ratio: f64,
}

/// `AdAudienceSyncList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAudienceSyncList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AdAudienceSyncListDataItem>,
}

/// `AdAudienceSyncListDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAudienceSyncListDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "audience_id", deserialize_with = "crate::de::null_default")]
    pub audience_id: String,
    /// Zorunlu. Değerler: `"schedule"`, `"manual"`, `"journey"`.
    #[serde(rename = "trigger", deserialize_with = "crate::de::null_default")]
    pub trigger: String,
    /// Zorunlu. Değerler: `"incremental"`, `"full"`.
    #[serde(rename = "mode", deserialize_with = "crate::de::null_default")]
    pub mode: String,
    /// Zorunlu. Değerler: `"running"`, `"succeeded"`, `"partial"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "added", deserialize_with = "crate::de::null_default")]
    pub added: i64,
    /// Zorunlu.
    #[serde(rename = "removed", deserialize_with = "crate::de::null_default")]
    pub removed: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "skipped_no_consent", deserialize_with = "crate::de::null_default")]
    pub skipped_no_consent: i64,
    /// Zorunlu.
    #[serde(rename = "skipped_no_identifier", deserialize_with = "crate::de::null_default")]
    pub skipped_no_identifier: i64,
    /// Zorunlu.
    #[serde(rename = "batches", deserialize_with = "crate::de::null_default")]
    pub batches: i64,
    /// Zorunlu.
    #[serde(rename = "retries", deserialize_with = "crate::de::null_default")]
    pub retries: i64,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "started_at", deserialize_with = "crate::de::null_default")]
    pub started_at: String,
    /// Zorunlu.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "duration_ms")]
    pub duration_ms: Option<i64>,
}

/// `AdAudienceSyncQueued` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdAudienceSyncQueued {
    /// Zorunlu.
    #[serde(rename = "queued", deserialize_with = "crate::de::null_default")]
    pub queued: bool,
    /// Zorunlu.
    #[serde(rename = "next_sync_at", deserialize_with = "crate::de::null_default")]
    pub next_sync_at: String,
    /// Zorunlu. Değerler: `"incremental"`, `"full"`.
    #[serde(rename = "mode", deserialize_with = "crate::de::null_default")]
    pub mode: String,
}

/// `AdBudgetAlertList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdBudgetAlertList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AdBudgetAlertListDataItem>,
}

/// `AdBudgetAlertListDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdBudgetAlertListDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "rule_id", deserialize_with = "crate::de::null_default")]
    pub rule_id: String,
    /// Zorunlu.
    #[serde(rename = "rule_name", deserialize_with = "crate::de::null_default")]
    pub rule_name: String,
    /// Zorunlu. Değerler: `"daily_spend"`, `"roas_drop"`, `"budget_depletion"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu.
    #[serde(rename = "day", deserialize_with = "crate::de::null_default")]
    pub day: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: f64,
    /// Zorunlu.
    #[serde(rename = "threshold", deserialize_with = "crate::de::null_default")]
    pub threshold: f64,
    /// Zorunlu.
    #[serde(rename = "details", deserialize_with = "crate::de::null_default")]
    pub details: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "emailed", deserialize_with = "crate::de::null_default")]
    pub emailed: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `AdBudgetAlertRule` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdBudgetAlertRule {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"daily_spend"`, `"roas_drop"`, `"budget_depletion"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu.
    #[serde(rename = "ad_account_id")]
    pub ad_account_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "campaign_external_id")]
    pub campaign_external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "threshold", deserialize_with = "crate::de::null_default")]
    pub threshold: f64,
    /// Zorunlu.
    #[serde(rename = "window_days", deserialize_with = "crate::de::null_default")]
    pub window_days: i64,
    /// Zorunlu.
    #[serde(rename = "recipients", deserialize_with = "crate::de::null_default")]
    pub recipients: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "last_value")]
    pub last_value: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "last_evaluated_at")]
    pub last_evaluated_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_triggered_on")]
    pub last_triggered_on: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `AdBudgetAlertRuleList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdBudgetAlertRuleList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AdBudgetAlertRule>,
}

/// `AdCapiConfig` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdCapiConfig {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu. Değerler: `"meta"`, `"google"`, `"tiktok"`.
    #[serde(rename = "platform", deserialize_with = "crate::de::null_default")]
    pub platform: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "ad_account_id")]
    pub ad_account_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: AdCapiConfigSettings,
    /// Zorunlu.
    #[serde(rename = "event_mappings", deserialize_with = "crate::de::null_default")]
    pub event_mappings: Vec<AdCapiConfigEventMappingsItem>,
    /// Zorunlu.
    #[serde(rename = "test_event_code")]
    pub test_event_code: Option<String>,
    /// Zorunlu.
    #[serde(rename = "test_mode", deserialize_with = "crate::de::null_default")]
    pub test_mode: bool,
    /// Zorunlu. Değerler: `"website"`, `"app"`.
    #[serde(rename = "action_source", deserialize_with = "crate::de::null_default")]
    pub action_source: String,
    /// Zorunlu.
    #[serde(rename = "default_currency", deserialize_with = "crate::de::null_default")]
    pub default_currency: String,
    /// Zorunlu.
    #[serde(rename = "has_access_token", deserialize_with = "crate::de::null_default")]
    pub has_access_token: bool,
    /// Zorunlu.
    #[serde(rename = "last_sent_at")]
    pub last_sent_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_error_at")]
    pub last_error_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: AdCapiConfigStats,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `AdCapiConfigEventMappingsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdCapiConfigEventMappingsItem {
    /// Zorunlu.
    #[serde(rename = "event_name", deserialize_with = "crate::de::null_default")]
    pub event_name: String,
    /// Zorunlu.
    #[serde(rename = "platform_event", deserialize_with = "crate::de::null_default")]
    pub platform_event: String,
    /// Zorunlu.
    #[serde(rename = "value_property")]
    pub value_property: Option<String>,
    /// Zorunlu.
    #[serde(rename = "currency_property")]
    pub currency_property: Option<String>,
}

/// `AdCapiConfigList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdCapiConfigList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AdCapiConfig>,
}

/// `AdCapiConfigSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdCapiConfigSettings {
    /// Zorunlu.
    #[serde(rename = "pixel_id")]
    pub pixel_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "customer_id")]
    pub customer_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "login_customer_id")]
    pub login_customer_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "pixel_code")]
    pub pixel_code: Option<String>,
}

/// `AdCapiConfigStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdCapiConfigStats {
    /// Zorunlu.
    #[serde(rename = "pending", deserialize_with = "crate::de::null_default")]
    pub pending: i64,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "skipped", deserialize_with = "crate::de::null_default")]
    pub skipped: i64,
}

/// `AdCapiDelivery` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdCapiDelivery {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "config_id", deserialize_with = "crate::de::null_default")]
    pub config_id: String,
    /// Zorunlu. Değerler: `"meta"`, `"google"`, `"tiktok"`.
    #[serde(rename = "platform", deserialize_with = "crate::de::null_default")]
    pub platform: String,
    /// Zorunlu.
    #[serde(rename = "event_id", deserialize_with = "crate::de::null_default")]
    pub event_id: String,
    /// Zorunlu.
    #[serde(rename = "event_name", deserialize_with = "crate::de::null_default")]
    pub event_name: String,
    /// Zorunlu.
    #[serde(rename = "platform_event", deserialize_with = "crate::de::null_default")]
    pub platform_event: String,
    /// Zorunlu.
    #[serde(rename = "dedup_id", deserialize_with = "crate::de::null_default")]
    pub dedup_id: String,
    /// Zorunlu.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Zorunlu. Değerler: `"pending"`, `"sent"`, `"failed"`, `"skipped"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "skip_reason")]
    pub skip_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "attempts", deserialize_with = "crate::de::null_default")]
    pub attempts: i64,
    /// Zorunlu.
    #[serde(rename = "next_attempt_at")]
    pub next_attempt_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "test", deserialize_with = "crate::de::null_default")]
    pub test: bool,
    /// Zorunlu.
    #[serde(rename = "event_occurred_at", deserialize_with = "crate::de::null_default")]
    pub event_occurred_at: String,
    /// Zorunlu.
    #[serde(rename = "sent_at")]
    pub sent_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `AdCapiDeliveryList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdCapiDeliveryList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AdCapiDelivery>,
}

/// `AddAliasDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AddAliasDto {
    /// Zorunlu.
    #[serde(rename = "identity", deserialize_with = "crate::de::null_default")]
    pub identity: std::collections::HashMap<String, String>,
}

/// `AddSubscriptionDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AddSubscriptionDto {
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "token", deserialize_with = "crate::de::null_default")]
    pub token: String,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<AddSubscriptionDtoDevice>,
    /// Değerler: `"subscribed"`, `"unsubscribed"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "opt_in", skip_serializing_if = "Option::is_none")]
    pub opt_in: Option<AddSubscriptionDtoOptIn>,
}

/// `AddSubscriptionDtoDevice` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AddSubscriptionDtoDevice {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "app_version", skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "sdk_version", skip_serializing_if = "Option::is_none")]
    pub sdk_version: Option<String>,
    #[serde(rename = "sdk_platform", skip_serializing_if = "Option::is_none")]
    pub sdk_platform: Option<String>,
    #[serde(rename = "carrier", skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "push_capable", skip_serializing_if = "Option::is_none")]
    pub push_capable: Option<bool>,
    /// Değerler: `"sandbox"`, `"production"`.
    #[serde(rename = "push_environment", skip_serializing_if = "Option::is_none")]
    pub push_environment: Option<String>,
    /// Değerler: `"phone"`, `"watch"`.
    #[serde(rename = "form_factor", skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
    /// Değerler: `"alert"`, `"voip"`, `"app_clip"`.
    #[serde(rename = "push_kind", skip_serializing_if = "Option::is_none")]
    pub push_kind: Option<String>,
}

/// `AddSubscriptionDtoOptIn` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AddSubscriptionDtoOptIn {
    /// Zorunlu.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    #[serde(rename = "at", skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
}

/// `AdEntity` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdEntity {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "ad_account_id", deserialize_with = "crate::de::null_default")]
    pub ad_account_id: String,
    /// Zorunlu. Değerler: `"meta"`, `"google"`, `"tiktok"`.
    #[serde(rename = "platform", deserialize_with = "crate::de::null_default")]
    pub platform: String,
    /// Zorunlu. Değerler: `"campaign"`, `"ad_set"`, `"ad"`.
    #[serde(rename = "level", deserialize_with = "crate::de::null_default")]
    pub level: String,
    /// Zorunlu.
    #[serde(rename = "external_id", deserialize_with = "crate::de::null_default")]
    pub external_id: String,
    /// Zorunlu.
    #[serde(rename = "campaign_external_id", deserialize_with = "crate::de::null_default")]
    pub campaign_external_id: String,
    /// Zorunlu.
    #[serde(rename = "ad_set_external_id")]
    pub ad_set_external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"active"`, `"paused"`, `"pending"`, `"rejected"`, `"archived"`, `"deleted"`, `"unknown"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "platform_status")]
    pub platform_status: Option<String>,
    /// Zorunlu.
    #[serde(rename = "objective")]
    pub objective: Option<String>,
    /// Zorunlu.
    #[serde(rename = "optimization_goal")]
    pub optimization_goal: Option<String>,
    /// Zorunlu.
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Zorunlu.
    #[serde(rename = "daily_budget")]
    pub daily_budget: Option<String>,
    /// Zorunlu.
    #[serde(rename = "lifetime_budget")]
    pub lifetime_budget: Option<String>,
    /// Zorunlu.
    #[serde(rename = "start_at")]
    pub start_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "audience_external_ids", deserialize_with = "crate::de::null_default")]
    pub audience_external_ids: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "creative")]
    pub creative: Option<AdEntityCreative>,
    /// Zorunlu.
    #[serde(rename = "budget_editable", deserialize_with = "crate::de::null_default")]
    pub budget_editable: bool,
    /// Zorunlu.
    #[serde(rename = "created_by_pushizo", deserialize_with = "crate::de::null_default")]
    pub created_by_pushizo: bool,
    /// Zorunlu.
    #[serde(rename = "last_seen_at")]
    pub last_seen_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "platform_url", deserialize_with = "crate::de::null_default")]
    pub platform_url: String,
    /// Zorunlu.
    #[serde(rename = "metrics")]
    pub metrics: Option<AdEntityMetrics>,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `AdEntityCreative` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdEntityCreative {
    /// Zorunlu.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Zorunlu.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "link_url")]
    pub link_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "call_to_action")]
    pub call_to_action: Option<String>,
}

/// `AdEntityList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdEntityList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AdEntity>,
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "accounts", deserialize_with = "crate::de::null_default")]
    pub accounts: Vec<AdEntityListAccountsItem>,
}

/// `AdEntityListAccountsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdEntityListAccountsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "campaigns_synced_at")]
    pub campaigns_synced_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "insights_synced_at")]
    pub insights_synced_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "campaigns_sync_error")]
    pub campaigns_sync_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "insights_sync_error")]
    pub insights_sync_error: Option<String>,
}

/// `AdEntityMetrics` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdEntityMetrics {
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: f64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: f64,
    /// Zorunlu.
    #[serde(rename = "spend", deserialize_with = "crate::de::null_default")]
    pub spend: f64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: f64,
    /// Zorunlu.
    #[serde(rename = "conversion_value", deserialize_with = "crate::de::null_default")]
    pub conversion_value: f64,
    /// Zorunlu.
    #[serde(rename = "ctr")]
    pub ctr: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "cpc")]
    pub cpc: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "roas")]
    pub roas: Option<f64>,
}

/// `AdLtvReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdLtvReport {
    /// Zorunlu.
    #[serde(rename = "currency", deserialize_with = "crate::de::null_default")]
    pub currency: String,
    /// Zorunlu.
    #[serde(rename = "months", deserialize_with = "crate::de::null_default")]
    pub months: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_events", deserialize_with = "crate::de::null_default")]
    pub conversion_events: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "cohorts", deserialize_with = "crate::de::null_default")]
    pub cohorts: Vec<AdLtvReportCohortsItem>,
    /// Zorunlu.
    #[serde(rename = "average", deserialize_with = "crate::de::null_default")]
    pub average: Vec<f64>,
    /// Zorunlu.
    #[serde(rename = "fx_missing", deserialize_with = "crate::de::null_default")]
    pub fx_missing: Vec<String>,
}

/// `AdLtvReportCohortsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdLtvReportCohortsItem {
    /// Zorunlu.
    #[serde(rename = "cohort", deserialize_with = "crate::de::null_default")]
    pub cohort: String,
    /// Zorunlu.
    #[serde(rename = "size", deserialize_with = "crate::de::null_default")]
    pub size: i64,
    /// Zorunlu.
    #[serde(rename = "revenue", deserialize_with = "crate::de::null_default")]
    pub revenue: Vec<f64>,
    /// Zorunlu.
    #[serde(rename = "ltv", deserialize_with = "crate::de::null_default")]
    pub ltv: Vec<f64>,
}

/// `AdOk` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdOk {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
}

/// `AdRefreshQueued` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdRefreshQueued {
    /// Zorunlu.
    #[serde(rename = "queued", deserialize_with = "crate::de::null_default")]
    pub queued: bool,
    /// Zorunlu.
    #[serde(rename = "next_sync_at", deserialize_with = "crate::de::null_default")]
    pub next_sync_at: String,
}

/// `AdSpendReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdSpendReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "currency", deserialize_with = "crate::de::null_default")]
    pub currency: String,
    /// Zorunlu.
    #[serde(rename = "group_by", deserialize_with = "crate::de::null_default")]
    pub group_by: String,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<AdSpendReportRowsItem>,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: AdSpendReportTotals,
    /// Zorunlu.
    #[serde(rename = "fx_missing", deserialize_with = "crate::de::null_default")]
    pub fx_missing: Vec<String>,
    /// Zorunlu. Değerler: `"postgres"`, `"clickhouse"`.
    #[serde(rename = "store", deserialize_with = "crate::de::null_default")]
    pub store: String,
}

/// `AdSpendReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdSpendReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
    /// Zorunlu.
    #[serde(rename = "platform")]
    pub platform: Option<String>,
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: f64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: f64,
    /// Zorunlu.
    #[serde(rename = "spend", deserialize_with = "crate::de::null_default")]
    pub spend: f64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: f64,
    /// Zorunlu.
    #[serde(rename = "conversion_value", deserialize_with = "crate::de::null_default")]
    pub conversion_value: f64,
    /// Zorunlu.
    #[serde(rename = "ctr")]
    pub ctr: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "cpc")]
    pub cpc: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "cpa")]
    pub cpa: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "roas")]
    pub roas: Option<f64>,
}

/// `AdSpendReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdSpendReportTotals {
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: f64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: f64,
    /// Zorunlu.
    #[serde(rename = "spend", deserialize_with = "crate::de::null_default")]
    pub spend: f64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: f64,
    /// Zorunlu.
    #[serde(rename = "conversion_value", deserialize_with = "crate::de::null_default")]
    pub conversion_value: f64,
    /// Zorunlu.
    #[serde(rename = "ctr")]
    pub ctr: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "cpc")]
    pub cpc: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "cpa")]
    pub cpa: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "roas")]
    pub roas: Option<f64>,
}

/// `ApiEventsDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ApiEventsDto {
    /// Zorunlu.
    #[serde(rename = "events", deserialize_with = "crate::de::null_default")]
    pub events: Vec<ApiEventsDtoEventsItem>,
}

/// `ApiEventsDtoEventsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ApiEventsDtoEventsItem {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "occurred_at", skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<String>,
    #[serde(rename = "event_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "subscription_id", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

/// `ApiKey` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ApiKey {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "org_id", deserialize_with = "crate::de::null_default")]
    pub org_id: String,
    /// Zorunlu.
    #[serde(rename = "app_id")]
    pub app_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "prefix", deserialize_with = "crate::de::null_default")]
    pub prefix: String,
    /// Zorunlu.
    #[serde(rename = "scopes", deserialize_with = "crate::de::null_default")]
    pub scopes: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "last_used_at")]
    pub last_used_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "revoked_at")]
    pub revoked_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "ip_allowlist", deserialize_with = "crate::de::null_default")]
    pub ip_allowlist: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "rotated_at")]
    pub rotated_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `ApiKeyList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ApiKeyList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<ApiKey>,
}

/// `App` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct App {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "org_id", deserialize_with = "crate::de::null_default")]
    pub org_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu.
    #[serde(rename = "platform_flags", deserialize_with = "crate::de::null_default")]
    pub platform_flags: AppPlatformFlags,
    /// Zorunlu.
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "default_language", deserialize_with = "crate::de::null_default")]
    pub default_language: String,
    /// Zorunlu.
    #[serde(rename = "timezone", deserialize_with = "crate::de::null_default")]
    pub timezone: String,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: AppSettings,
    /// Zorunlu. Değerler: `"hmac"`, `"jwt"`, `"both"`.
    #[serde(rename = "identity_verification_mode", deserialize_with = "crate::de::null_default")]
    pub identity_verification_mode: String,
    /// Zorunlu. Değerler: `"active"`, `"suspended"`, `"deleted"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `AppFlag` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppFlag {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu. Değerler: `"boolean"`, `"string"`, `"number"`, `"json"`.
    #[serde(rename = "value_type", deserialize_with = "crate::de::null_default")]
    pub value_type: String,
    /// Zorunlu.
    #[serde(rename = "on_value")]
    pub on_value: Option<serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "off_value")]
    pub off_value: Option<serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "rollout_percent", deserialize_with = "crate::de::null_default")]
    pub rollout_percent: i64,
    /// Zorunlu.
    #[serde(rename = "segment_id")]
    pub segment_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "rollout_schedule", deserialize_with = "crate::de::null_default")]
    pub rollout_schedule: Vec<AppFlagRolloutScheduleItem>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `AppFlagEvaluation` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppFlagEvaluation {
    /// Zorunlu.
    #[serde(rename = "flags", deserialize_with = "crate::de::null_default")]
    pub flags: std::collections::HashMap<String, AppFlagEvaluationFlagsValue>,
}

/// `AppFlagEvaluationFlagsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppFlagEvaluationFlagsValue {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
    /// Zorunlu. Değerler: `"on"`, `"disabled"`, `"segment"`, `"rollout"`.
    #[serde(rename = "reason", deserialize_with = "crate::de::null_default")]
    pub reason: String,
}

/// `AppFlagList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppFlagList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AppFlag>,
}

/// `AppFlagRolloutScheduleItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppFlagRolloutScheduleItem {
    /// Zorunlu.
    #[serde(rename = "at", deserialize_with = "crate::de::null_default")]
    pub at: String,
    /// Zorunlu.
    #[serde(rename = "percent", deserialize_with = "crate::de::null_default")]
    pub percent: i64,
}

/// `AppList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<App>,
}

/// `AppPlatformFlags` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppPlatformFlags {
    #[serde(rename = "ios", skip_serializing_if = "Option::is_none")]
    pub ios: Option<bool>,
    #[serde(rename = "android", skip_serializing_if = "Option::is_none")]
    pub android: Option<bool>,
    #[serde(rename = "web", skip_serializing_if = "Option::is_none")]
    pub web: Option<bool>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
}

/// `AppSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettings {
    #[serde(rename = "quietHours", skip_serializing_if = "Option::is_none")]
    pub quiet_hours: Option<AppSettingsQuietHours>,
    #[serde(rename = "frequencyCap", skip_serializing_if = "Option::is_none")]
    pub frequency_cap: Option<AppSettingsFrequencyCap>,
    #[serde(rename = "frequencyCaps", skip_serializing_if = "Option::is_none")]
    pub frequency_caps: Option<Vec<AppSettingsFrequencyCapsItem>>,
    #[serde(rename = "inApp", skip_serializing_if = "Option::is_none")]
    pub in_app: Option<AppSettingsInApp>,
    #[serde(rename = "androidChannels", skip_serializing_if = "Option::is_none")]
    pub android_channels: Option<Vec<AppSettingsAndroidChannelsItem>>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<AppSettingsSms>,
    #[serde(rename = "preferenceCenter", skip_serializing_if = "Option::is_none")]
    pub preference_center: Option<AppSettingsPreferenceCenter>,
    #[serde(rename = "brandKit", skip_serializing_if = "Option::is_none")]
    pub brand_kit: Option<AppSettingsBrandKit>,
    #[serde(rename = "defaultTtlSeconds", skip_serializing_if = "Option::is_none")]
    pub default_ttl_seconds: Option<i64>,
    #[serde(rename = "identityVerificationRequired", skip_serializing_if = "Option::is_none")]
    pub identity_verification_required: Option<bool>,
    /// Değerler: `"off"`, `"full"`, `"masked"`.
    #[serde(rename = "ipCapture", skip_serializing_if = "Option::is_none")]
    pub ip_capture: Option<String>,
    #[serde(rename = "ipRetentionDays", skip_serializing_if = "Option::is_none")]
    pub ip_retention_days: Option<i64>,
    #[serde(rename = "adaptiveFrequency", skip_serializing_if = "Option::is_none")]
    pub adaptive_frequency: Option<AppSettingsAdaptiveFrequency>,
    #[serde(rename = "allowedOrigins", skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(rename = "campaignApprovalRequired", skip_serializing_if = "Option::is_none")]
    pub campaign_approval_required: Option<bool>,
    #[serde(rename = "webPush", skip_serializing_if = "Option::is_none")]
    pub web_push: Option<AppSettingsWebPush>,
    #[serde(rename = "onboarding", skip_serializing_if = "Option::is_none")]
    pub onboarding: Option<AppSettingsOnboarding>,
    #[serde(rename = "sdkSetup", skip_serializing_if = "Option::is_none")]
    pub sdk_setup: Option<AppSettingsSdkSetup>,
}

/// `AppSettingsAdaptiveFrequency` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsAdaptiveFrequency {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "min", deserialize_with = "crate::de::null_default")]
    pub min: i64,
    /// Zorunlu.
    #[serde(rename = "max", deserialize_with = "crate::de::null_default")]
    pub max: i64,
}

/// `AppSettingsAndroidChannelsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsAndroidChannelsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Zorunlu. Değerler: `"low"`, `"default"`, `"high"`, `"urgent"`.
    #[serde(rename = "importance", deserialize_with = "crate::de::null_default")]
    pub importance: String,
    #[serde(rename = "sound", skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "vibration", skip_serializing_if = "Option::is_none")]
    pub vibration: Option<bool>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// `AppSettingsBrandKit` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsBrandKit {
    #[serde(rename = "website_url", skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "primary_color", skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[serde(rename = "accent_color", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    #[serde(rename = "font", skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "tone", skip_serializing_if = "Option::is_none")]
    pub tone: Option<String>,
}

/// `AppSettingsFrequencyCap` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsFrequencyCap {
    /// Zorunlu.
    #[serde(rename = "max", deserialize_with = "crate::de::null_default")]
    pub max: i64,
    /// Zorunlu.
    #[serde(rename = "windowHours", deserialize_with = "crate::de::null_default")]
    pub window_hours: i64,
}

/// `AppSettingsFrequencyCapsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsFrequencyCapsItem {
    /// Zorunlu.
    #[serde(rename = "max", deserialize_with = "crate::de::null_default")]
    pub max: i64,
    /// Zorunlu.
    #[serde(rename = "window", deserialize_with = "crate::de::null_default")]
    pub window: AppSettingsFrequencyCapsItemWindow,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
}

/// `AppSettingsFrequencyCapsItemWindow` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsFrequencyCapsItemWindow {
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: i64,
    /// Zorunlu. Değerler: `"minutes"`, `"hours"`, `"days"`, `"weeks"`.
    #[serde(rename = "unit", deserialize_with = "crate::de::null_default")]
    pub unit: String,
}

/// `AppSettingsInApp` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsInApp {
    #[serde(rename = "minIntervalSeconds", skip_serializing_if = "Option::is_none")]
    pub min_interval_seconds: Option<i64>,
}

/// `AppSettingsOnboarding` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsOnboarding {
    /// Zorunlu.
    #[serde(rename = "step", deserialize_with = "crate::de::null_default")]
    pub step: i64,
    #[serde(rename = "simulation", skip_serializing_if = "Option::is_none")]
    pub simulation: Option<bool>,
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// Değerler: `"ios"`, `"android"`, `"huawei"`, `"web"`, `"amazon"`, `"windows"`, `"email"`, `"sms"`.
    #[serde(rename = "platforms", skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
    /// Değerler: `"native"`, `"flutter"`, `"react-native"`, `"expo"`, `"kmp"`, `"unity"`, `"capacitor"`, `"cordova"`, `"maui"`.
    #[serde(rename = "framework", skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// Değerler: `"code"`, `"wordpress"`.
    #[serde(rename = "web_integration", skip_serializing_if = "Option::is_none")]
    pub web_integration: Option<String>,
}

/// `AppSettingsPreferenceCenter` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsPreferenceCenter {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "brandName", skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(rename = "logoUrl", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "accentColor", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    #[serde(rename = "customUnsubscribeUrl", skip_serializing_if = "Option::is_none")]
    pub custom_unsubscribe_url: Option<String>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<std::collections::HashMap<String, AppSettingsPreferenceCenterTextsValue>>,
}

/// `AppSettingsPreferenceCenterTextsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsPreferenceCenterTextsValue {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "unsubscribeAll", skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<String>,
    #[serde(rename = "saved", skip_serializing_if = "Option::is_none")]
    pub saved: Option<String>,
}

/// `AppSettingsQuietHours` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsQuietHours {
    /// Zorunlu.
    #[serde(rename = "start", deserialize_with = "crate::de::null_default")]
    pub start: String,
    /// Zorunlu.
    #[serde(rename = "end", deserialize_with = "crate::de::null_default")]
    pub end: String,
    /// Zorunlu. Değerler: `"user"`, `"app"`.
    #[serde(rename = "timezoneMode", deserialize_with = "crate::de::null_default")]
    pub timezone_mode: String,
}

/// `AppSettingsSdkSetup` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsSdkSetup {
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "platform")]
    pub platform: Option<String>,
    /// Zorunlu.
    #[serde(rename = "step", deserialize_with = "crate::de::null_default")]
    pub step: String,
    /// Zorunlu.
    #[serde(rename = "furthest", deserialize_with = "crate::de::null_default")]
    pub furthest: String,
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(rename = "detectedSubscriptionId", skip_serializing_if = "Option::is_none")]
    pub detected_subscription_id: Option<String>,
    #[serde(rename = "testMessageId", skip_serializing_if = "Option::is_none")]
    pub test_message_id: Option<String>,
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

/// `AppSettingsSms` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsSms {
    #[serde(rename = "helpReply", skip_serializing_if = "Option::is_none")]
    pub help_reply: Option<String>,
    #[serde(rename = "stopReply", skip_serializing_if = "Option::is_none")]
    pub stop_reply: Option<String>,
    #[serde(rename = "autoReply", skip_serializing_if = "Option::is_none")]
    pub auto_reply: Option<bool>,
}

/// `AppSettingsWebPush` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPush {
    #[serde(rename = "serviceWorkerPath", skip_serializing_if = "Option::is_none")]
    pub service_worker_path: Option<String>,
    #[serde(rename = "serviceWorkerScope", skip_serializing_if = "Option::is_none")]
    pub service_worker_scope: Option<String>,
    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<AppSettingsWebPushPrompt>,
    #[serde(rename = "welcomeNotification", skip_serializing_if = "Option::is_none")]
    pub welcome_notification: Option<AppSettingsWebPushWelcomeNotification>,
    #[serde(rename = "clickBehavior", skip_serializing_if = "Option::is_none")]
    pub click_behavior: Option<AppSettingsWebPushClickBehavior>,
    #[serde(rename = "customLink", skip_serializing_if = "Option::is_none")]
    pub custom_link: Option<AppSettingsWebPushCustomLink>,
    #[serde(rename = "promptBackoffDays", skip_serializing_if = "Option::is_none")]
    pub prompt_backoff_days: Option<Vec<i64>>,
}

/// `AppSettingsWebPushClickBehavior` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushClickBehavior {
    /// Zorunlu. Değerler: `"exact"`, `"origin"`.
    #[serde(rename = "match", deserialize_with = "crate::de::null_default")]
    pub r#match: String,
    /// Zorunlu. Değerler: `"navigate"`, `"focus"`.
    #[serde(rename = "action", deserialize_with = "crate::de::null_default")]
    pub action: String,
}

/// `AppSettingsWebPushCustomLink` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushCustomLink {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu. Değerler: `"button"`, `"link"`.
    #[serde(rename = "style", deserialize_with = "crate::de::null_default")]
    pub style: String,
    /// Zorunlu. Değerler: `"small"`, `"medium"`, `"large"`.
    #[serde(rename = "size", deserialize_with = "crate::de::null_default")]
    pub size: String,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<AppSettingsWebPushCustomLinkColor>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<std::collections::HashMap<String, AppSettingsWebPushCustomLinkTextsValue>>,
    #[serde(rename = "unsubscribeEnabled", skip_serializing_if = "Option::is_none")]
    pub unsubscribe_enabled: Option<bool>,
}

/// `AppSettingsWebPushCustomLinkColor` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushCustomLinkColor {
    #[serde(rename = "button", skip_serializing_if = "Option::is_none")]
    pub button: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// `AppSettingsWebPushCustomLinkTextsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushCustomLinkTextsValue {
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<String>,
    #[serde(rename = "unsubscribe", skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<String>,
    #[serde(rename = "explanation", skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

/// `AppSettingsWebPushPrompt` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushPrompt {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu. Değerler: `"slidedown"`, `"native"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Değerler: `"push"`, `"email"`, `"sms"`, `"email_sms"`.
    #[serde(rename = "collect", skip_serializing_if = "Option::is_none")]
    pub collect: Option<String>,
    #[serde(rename = "delaySeconds", skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<i64>,
    #[serde(rename = "pageViews", skip_serializing_if = "Option::is_none")]
    pub page_views: Option<i64>,
    #[serde(rename = "scrollPercent", skip_serializing_if = "Option::is_none")]
    pub scroll_percent: Option<i64>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<std::collections::HashMap<String, AppSettingsWebPushPromptTextsValue>>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<AppSettingsWebPushPromptCategoriesItem>>,
    #[serde(rename = "accentColor", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
}

/// `AppSettingsWebPushPromptCategoriesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushPromptCategoriesItem {
    /// Zorunlu.
    #[serde(rename = "tag", deserialize_with = "crate::de::null_default")]
    pub tag: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: std::collections::HashMap<String, String>,
}

/// `AppSettingsWebPushPromptTextsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushPromptTextsValue {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "accept", skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(rename = "cancel", skip_serializing_if = "Option::is_none")]
    pub cancel: Option<String>,
}

/// `AppSettingsWebPushWelcomeNotification` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushWelcomeNotification {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<std::collections::HashMap<String, AppSettingsWebPushWelcomeNotificationTextsValue>>,
}

/// `AppSettingsWebPushWelcomeNotificationTextsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSettingsWebPushWelcomeNotificationTextsValue {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `AuditEntry` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuditEntry {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "org_id", deserialize_with = "crate::de::null_default")]
    pub org_id: String,
    /// Zorunlu.
    #[serde(rename = "app_id")]
    pub app_id: Option<String>,
    /// Zorunlu. Değerler: `"account"`, `"api_key"`, `"system"`.
    #[serde(rename = "actor_type", deserialize_with = "crate::de::null_default")]
    pub actor_type: String,
    /// Zorunlu.
    #[serde(rename = "actor_id")]
    pub actor_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "action", deserialize_with = "crate::de::null_default")]
    pub action: String,
    /// Zorunlu.
    #[serde(rename = "target_type")]
    pub target_type: Option<String>,
    /// Zorunlu.
    #[serde(rename = "target_id")]
    pub target_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "metadata", deserialize_with = "crate::de::null_default")]
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "ip")]
    pub ip: Option<String>,
    /// Zorunlu.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `AuditPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuditPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<AuditEntry>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `Campaign` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Campaign {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "labels", deserialize_with = "crate::de::null_default")]
    pub labels: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Zorunlu.
    #[serde(rename = "goals", deserialize_with = "crate::de::null_default")]
    pub goals: CampaignGoals,
    /// Zorunlu.
    #[serde(rename = "ab_test", deserialize_with = "crate::de::null_default")]
    pub ab_test: CampaignAbTest,
    /// Zorunlu.
    #[serde(rename = "variants", deserialize_with = "crate::de::null_default")]
    pub variants: Vec<CampaignVariantsItem>,
    /// Zorunlu. Değerler: `"one_time"`, `"scheduled"`, `"recurring"`, `"automated"`, `"transactional"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu. Değerler: `"draft"`, `"scheduled"`, `"running"`, `"paused"`, `"completed"`, `"cancelled"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "audience", deserialize_with = "crate::de::null_default")]
    pub audience: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "content", deserialize_with = "crate::de::null_default")]
    pub content: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "template_id")]
    pub template_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "template_version")]
    pub template_version: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "delivery", deserialize_with = "crate::de::null_default")]
    pub delivery: CampaignDelivery,
    /// Zorunlu.
    #[serde(rename = "recurrence")]
    pub recurrence: Option<CampaignRecurrence>,
    /// Zorunlu.
    #[serde(rename = "approval", deserialize_with = "crate::de::null_default")]
    pub approval: CampaignApproval,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: CampaignStats2,
    /// Zorunlu.
    #[serde(rename = "send_at")]
    pub send_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "next_run_at")]
    pub next_run_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_run_at")]
    pub last_run_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "run_count", deserialize_with = "crate::de::null_default")]
    pub run_count: i64,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "started_at")]
    pub started_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu.
    #[serde(rename = "estimated_recipients")]
    pub estimated_recipients: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "preflight")]
    pub preflight: Option<CampaignPreflight>,
}

/// `CampaignAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignAbTest {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "test_percentage", skip_serializing_if = "Option::is_none")]
    pub test_percentage: Option<i64>,
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<CampaignAbTestWinner>,
    /// Değerler: `"testing"`, `"winner_sent"`.
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(rename = "winner_variant_id", skip_serializing_if = "Option::is_none")]
    pub winner_variant_id: Option<String>,
    #[serde(rename = "decided_at", skip_serializing_if = "Option::is_none")]
    pub decided_at: Option<String>,
    #[serde(rename = "decided_by", skip_serializing_if = "Option::is_none")]
    pub decided_by: Option<String>,
    #[serde(rename = "significance", skip_serializing_if = "Option::is_none")]
    pub significance: Option<CampaignAbTestSignificance>,
}

/// `CampaignAbTestSignificance` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignAbTestSignificance {
    /// Zorunlu.
    #[serde(rename = "p_value", deserialize_with = "crate::de::null_default")]
    pub p_value: f64,
    /// Zorunlu.
    #[serde(rename = "significant", deserialize_with = "crate::de::null_default")]
    pub significant: bool,
    /// Zorunlu.
    #[serde(rename = "metric", deserialize_with = "crate::de::null_default")]
    pub metric: String,
}

/// `CampaignAbTestWinner` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignAbTestWinner {
    /// Zorunlu. Değerler: `"manual"`, `"auto"`.
    #[serde(rename = "mode", deserialize_with = "crate::de::null_default")]
    pub mode: String,
    /// Zorunlu. Değerler: `"open_rate"`, `"click_rate"`, `"conversion_rate"`.
    #[serde(rename = "metric", deserialize_with = "crate::de::null_default")]
    pub metric: String,
    #[serde(rename = "evaluate_after_minutes", skip_serializing_if = "Option::is_none")]
    pub evaluate_after_minutes: Option<i64>,
}

/// `CampaignApproval` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignApproval {
    /// Zorunlu.
    #[serde(rename = "required", deserialize_with = "crate::de::null_default")]
    pub required: bool,
    /// Değerler: `"send"`, `"schedule"`.
    #[serde(rename = "pending_action", skip_serializing_if = "Option::is_none")]
    pub pending_action: Option<String>,
    #[serde(rename = "requested_by", skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    #[serde(rename = "requested_at", skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<String>,
    #[serde(rename = "approved_by", skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
    #[serde(rename = "approved_at", skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
    #[serde(rename = "rejected_by", skip_serializing_if = "Option::is_none")]
    pub rejected_by: Option<String>,
    #[serde(rename = "rejected_at", skip_serializing_if = "Option::is_none")]
    pub rejected_at: Option<String>,
}

/// `CampaignDelivery` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignDelivery {
    /// Değerler: `"immediate"`, `"per_user_timezone"`, `"optimized"`.
    #[serde(rename = "timezone_mode", skip_serializing_if = "Option::is_none")]
    pub timezone_mode: Option<String>,
    #[serde(rename = "delivery_time_of_day", skip_serializing_if = "Option::is_none")]
    pub delivery_time_of_day: Option<String>,
    #[serde(rename = "throttle_per_minute", skip_serializing_if = "Option::is_none")]
    pub throttle_per_minute: Option<i64>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// Değerler: `5`, `10`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "collapse_id", skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    #[serde(rename = "frequency_cap_exempt", skip_serializing_if = "Option::is_none")]
    pub frequency_cap_exempt: Option<bool>,
    /// Değerler: `"low"`, `"normal"`, `"high"`.
    #[serde(rename = "frequency_priority", skip_serializing_if = "Option::is_none")]
    pub frequency_priority: Option<String>,
    #[serde(rename = "global_control_exempt", skip_serializing_if = "Option::is_none")]
    pub global_control_exempt: Option<bool>,
    #[serde(rename = "quiet_hours_exempt", skip_serializing_if = "Option::is_none")]
    pub quiet_hours_exempt: Option<bool>,
    #[serde(rename = "one_per_user", skip_serializing_if = "Option::is_none")]
    pub one_per_user: Option<bool>,
}

/// `CampaignEmailRisk` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignEmailRisk {
    /// Zorunlu.
    #[serde(rename = "applies", deserialize_with = "crate::de::null_default")]
    pub applies: bool,
    /// Zorunlu.
    #[serde(rename = "recipients", deserialize_with = "crate::de::null_default")]
    pub recipients: i64,
    /// Zorunlu.
    #[serde(rename = "flagged_invalid", deserialize_with = "crate::de::null_default")]
    pub flagged_invalid: i64,
    /// Zorunlu.
    #[serde(rename = "sendable", deserialize_with = "crate::de::null_default")]
    pub sendable: i64,
    /// Zorunlu.
    #[serde(rename = "valid", deserialize_with = "crate::de::null_default")]
    pub valid: i64,
    /// Zorunlu.
    #[serde(rename = "unknown", deserialize_with = "crate::de::null_default")]
    pub unknown: i64,
    /// Zorunlu.
    #[serde(rename = "unchecked", deserialize_with = "crate::de::null_default")]
    pub unchecked: i64,
    /// Zorunlu.
    #[serde(rename = "estimated_invalid_ratio", deserialize_with = "crate::de::null_default")]
    pub estimated_invalid_ratio: f64,
    /// Zorunlu.
    #[serde(rename = "max_invalid_ratio", deserialize_with = "crate::de::null_default")]
    pub max_invalid_ratio: f64,
    /// Zorunlu.
    #[serde(rename = "blocked", deserialize_with = "crate::de::null_default")]
    pub blocked: bool,
    /// Zorunlu.
    #[serde(rename = "bounce_rate_7d")]
    pub bounce_rate_7d: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "bounce_rate_30d")]
    pub bounce_rate_30d: Option<f64>,
}

/// `CampaignGoals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignGoals {
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(rename = "secondary", skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<String>>,
}

/// `CampaignPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Campaign>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `CampaignPreflight` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignPreflight {
    /// Zorunlu.
    #[serde(rename = "checked_at", deserialize_with = "crate::de::null_default")]
    pub checked_at: String,
    /// Zorunlu.
    #[serde(rename = "summary", deserialize_with = "crate::de::null_default")]
    pub summary: CampaignPreflightSummary,
    /// Zorunlu.
    #[serde(rename = "codes", deserialize_with = "crate::de::null_default")]
    pub codes: Vec<String>,
    #[serde(rename = "acknowledged_by", skip_serializing_if = "Option::is_none")]
    pub acknowledged_by: Option<String>,
    #[serde(rename = "acknowledged_at", skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<String>,
}

/// `CampaignPreflightReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignPreflightReport {
    /// Zorunlu.
    #[serde(rename = "campaign_id", deserialize_with = "crate::de::null_default")]
    pub campaign_id: String,
    /// Zorunlu.
    #[serde(rename = "checked_at", deserialize_with = "crate::de::null_default")]
    pub checked_at: String,
    /// Zorunlu.
    #[serde(rename = "links_checked", deserialize_with = "crate::de::null_default")]
    pub links_checked: bool,
    /// Zorunlu.
    #[serde(rename = "summary", deserialize_with = "crate::de::null_default")]
    pub summary: CampaignPreflightReportSummary,
    /// Zorunlu.
    #[serde(rename = "findings", deserialize_with = "crate::de::null_default")]
    pub findings: Vec<CampaignPreflightReportFindingsItem>,
}

/// `CampaignPreflightReportFindingsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignPreflightReportFindingsItem {
    /// Zorunlu. Değerler: `"brand_name_casing"`, `"off_brand_color"`, `"off_brand_font"`, `"banned_phrase"`, `"phishing_term"`, `"link_invalid"`, `"link_insecure_http"`, `"link_unsafe_host"`, `"link_shortener"`, `"link_missing_utm"`, `"link_broken"`, `"link_unreachable"`, `"liquid_syntax_error"`, `"liquid_missing_default"`, `"liquid_unknown_variable"`, `"liquid_undefined_property"`, `"commercial_no_category"`, `"iys_sms_filter_informational"`, `"sms_missing_opt_out"`, `"email_missing_postal_address"`, `"send_time_outside_hours"`.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    /// Zorunlu. Değerler: `"brand"`, `"content"`, `"links"`, `"personalization"`, `"compliance"`.
    #[serde(rename = "category", deserialize_with = "crate::de::null_default")]
    pub category: String,
    /// Zorunlu. Değerler: `"critical"`, `"warning"`, `"info"`.
    #[serde(rename = "severity", deserialize_with = "crate::de::null_default")]
    pub severity: String,
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// `CampaignPreflightReportSummary` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignPreflightReportSummary {
    /// Zorunlu.
    #[serde(rename = "critical", deserialize_with = "crate::de::null_default")]
    pub critical: i64,
    /// Zorunlu.
    #[serde(rename = "warning", deserialize_with = "crate::de::null_default")]
    pub warning: i64,
    /// Zorunlu.
    #[serde(rename = "info", deserialize_with = "crate::de::null_default")]
    pub info: i64,
    /// Zorunlu.
    #[serde(rename = "blocked", deserialize_with = "crate::de::null_default")]
    pub blocked: bool,
}

/// `CampaignPreflightSummary` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignPreflightSummary {
    /// Zorunlu.
    #[serde(rename = "critical", deserialize_with = "crate::de::null_default")]
    pub critical: i64,
    /// Zorunlu.
    #[serde(rename = "warning", deserialize_with = "crate::de::null_default")]
    pub warning: i64,
    /// Zorunlu.
    #[serde(rename = "info", deserialize_with = "crate::de::null_default")]
    pub info: i64,
    /// Zorunlu.
    #[serde(rename = "blocked", deserialize_with = "crate::de::null_default")]
    pub blocked: bool,
}

/// `CampaignRecurrence` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignRecurrence {
    /// Zorunlu.
    #[serde(rename = "cron", deserialize_with = "crate::de::null_default")]
    pub cron: String,
    #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// `CampaignsCreateBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBody {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"one_time"`, `"scheduled"`, `"recurring"`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<CampaignsCreateBodyAudience>,
    /// messages.content ile aynı şema (headings, contents, url, buttons…)
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Tercih kategorisi anahtarı; kapalı kullanıcılar bastırılır
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Dönüşüm hedefleri (ADR-0080): birincil A/B kazananını belirler; olaylar dönüşüm olayı olarak tanımlı olmalı.
    #[serde(rename = "goals", skip_serializing_if = "Option::is_none")]
    pub goals: Option<CampaignsCreateBodyGoals>,
    #[serde(rename = "ab_test", skip_serializing_if = "Option::is_none")]
    pub ab_test: Option<CampaignsCreateBodyAbTest>,
    /// A/B varyantları (ağırlıklar toplamı 100; en fazla bir is_control)
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<CampaignsCreateBodyVariantsItem>>,
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "template_version", skip_serializing_if = "Option::is_none")]
    pub template_version: Option<i64>,
    #[serde(rename = "delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CampaignsCreateBodyDelivery>,
    #[serde(rename = "recurrence", skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<CampaignsCreateBodyRecurrence>,
    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
}

/// `CampaignsCreateBodyAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBodyAbTest {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "test_percentage", skip_serializing_if = "Option::is_none")]
    pub test_percentage: Option<i64>,
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<CampaignsCreateBodyAbTestWinner>,
}

/// `CampaignsCreateBodyAbTestWinner` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBodyAbTestWinner {
    /// Değerler: `"manual"`, `"auto"`.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Değerler: `"open_rate"`, `"click_rate"`, `"conversion_rate"`.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "evaluate_after_minutes", skip_serializing_if = "Option::is_none")]
    pub evaluate_after_minutes: Option<i64>,
}

/// `CampaignsCreateBodyAudience` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBodyAudience {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<String>>,
    #[serde(rename = "exclude_segments", skip_serializing_if = "Option::is_none")]
    pub exclude_segments: Option<Vec<String>>,
    /// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<CampaignsCreateBodyAudienceFilters>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "test_devices_only", skip_serializing_if = "Option::is_none")]
    pub test_devices_only: Option<bool>,
}

/// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBodyAudienceFilters {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// `CampaignsCreateBodyDelivery` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBodyDelivery {
    /// Değerler: `"immediate"`, `"per_user_timezone"`, `"optimized"`.
    #[serde(rename = "timezone_mode", skip_serializing_if = "Option::is_none")]
    pub timezone_mode: Option<String>,
    #[serde(rename = "delivery_time_of_day", skip_serializing_if = "Option::is_none")]
    pub delivery_time_of_day: Option<String>,
    #[serde(rename = "throttle_per_minute", skip_serializing_if = "Option::is_none")]
    pub throttle_per_minute: Option<i64>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// Değerler: `5`, `10`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "collapse_id", skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    #[serde(rename = "frequency_cap_exempt", skip_serializing_if = "Option::is_none")]
    pub frequency_cap_exempt: Option<bool>,
    /// Değerler: `"low"`, `"normal"`, `"high"`.
    #[serde(rename = "frequency_priority", skip_serializing_if = "Option::is_none")]
    pub frequency_priority: Option<String>,
    #[serde(rename = "global_control_exempt", skip_serializing_if = "Option::is_none")]
    pub global_control_exempt: Option<bool>,
    #[serde(rename = "quiet_hours_exempt", skip_serializing_if = "Option::is_none")]
    pub quiet_hours_exempt: Option<bool>,
    #[serde(rename = "one_per_user", skip_serializing_if = "Option::is_none")]
    pub one_per_user: Option<bool>,
}

/// Dönüşüm hedefleri (ADR-0080): birincil A/B kazananını belirler; olaylar dönüşüm olayı olarak tanımlı olmalı.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBodyGoals {
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(rename = "secondary", skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<String>>,
}

/// `CampaignsCreateBodyRecurrence` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBodyRecurrence {
    #[serde(rename = "cron", skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// `CampaignsCreateBodyVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsCreateBodyVariantsItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "is_control", skip_serializing_if = "Option::is_none")]
    pub is_control: Option<bool>,
}

/// `CampaignStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignStats {
    /// Zorunlu.
    #[serde(rename = "campaign", deserialize_with = "crate::de::null_default")]
    pub campaign: Campaign,
    /// Zorunlu.
    #[serde(rename = "rates", deserialize_with = "crate::de::null_default")]
    pub rates: CampaignStatsRates,
    /// Zorunlu.
    #[serde(rename = "goals", deserialize_with = "crate::de::null_default")]
    pub goals: Vec<CampaignStatsGoalsItem>,
    /// Zorunlu.
    #[serde(rename = "cards")]
    pub cards: Option<CampaignStatsCards>,
    /// Zorunlu.
    #[serde(rename = "channels", deserialize_with = "crate::de::null_default")]
    pub channels: Vec<CampaignStatsChannelsItem>,
    /// Zorunlu.
    #[serde(rename = "hourly", deserialize_with = "crate::de::null_default")]
    pub hourly: Vec<CampaignStatsHourlyItem>,
    /// Zorunlu.
    #[serde(rename = "errors", deserialize_with = "crate::de::null_default")]
    pub errors: Vec<CampaignStatsErrorsItem>,
    /// Zorunlu.
    #[serde(rename = "messages", deserialize_with = "crate::de::null_default")]
    pub messages: Vec<Message>,
}

/// `CampaignStats2` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignStats2 {
    #[serde(rename = "runs", skip_serializing_if = "Option::is_none")]
    pub runs: Option<i64>,
    #[serde(rename = "targeted", skip_serializing_if = "Option::is_none")]
    pub targeted: Option<i64>,
    #[serde(rename = "suppressed", skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<i64>,
    #[serde(rename = "sent", skip_serializing_if = "Option::is_none")]
    pub sent: Option<i64>,
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(rename = "opened", skip_serializing_if = "Option::is_none")]
    pub opened: Option<i64>,
    #[serde(rename = "clicked", skip_serializing_if = "Option::is_none")]
    pub clicked: Option<i64>,
    #[serde(rename = "converted", skip_serializing_if = "Option::is_none")]
    pub converted: Option<i64>,
    #[serde(rename = "bounced", skip_serializing_if = "Option::is_none")]
    pub bounced: Option<i64>,
    #[serde(rename = "complained", skip_serializing_if = "Option::is_none")]
    pub complained: Option<i64>,
    #[serde(rename = "unsubscribed", skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
    #[serde(rename = "sms_segments", skip_serializing_if = "Option::is_none")]
    pub sms_segments: Option<i64>,
}

/// `CampaignStatsCards` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignStatsCards {
    /// Zorunlu.
    #[serde(rename = "created", deserialize_with = "crate::de::null_default")]
    pub created: i64,
    /// Zorunlu.
    #[serde(rename = "viewed", deserialize_with = "crate::de::null_default")]
    pub viewed: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "dismissed", deserialize_with = "crate::de::null_default")]
    pub dismissed: i64,
    /// Zorunlu.
    #[serde(rename = "view_rate")]
    pub view_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "click_rate")]
    pub click_rate: Option<f64>,
}

/// `CampaignStatsChannelsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignStatsChannelsItem {
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "targeted", deserialize_with = "crate::de::null_default")]
    pub targeted: i64,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
}

/// `CampaignStatsErrorsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignStatsErrorsItem {
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
}

/// `CampaignStatsGoalsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignStatsGoalsItem {
    /// Zorunlu.
    #[serde(rename = "event", deserialize_with = "crate::de::null_default")]
    pub event: String,
    /// Zorunlu.
    #[serde(rename = "primary", deserialize_with = "crate::de::null_default")]
    pub primary: bool,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate")]
    pub conversion_rate: Option<f64>,
}

/// `CampaignStatsHourlyItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignStatsHourlyItem {
    /// Zorunlu.
    #[serde(rename = "hour", deserialize_with = "crate::de::null_default")]
    pub hour: String,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
}

/// `CampaignStatsRates` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignStatsRates {
    /// Zorunlu.
    #[serde(rename = "delivery_rate")]
    pub delivery_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "open_rate")]
    pub open_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "click_rate")]
    pub click_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "failure_rate")]
    pub failure_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "conversion_rate")]
    pub conversion_rate: Option<f64>,
}

/// `CampaignsUpdateBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBody {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"one_time"`, `"scheduled"`, `"recurring"`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<CampaignsUpdateBodyAudience>,
    /// messages.content ile aynı şema (headings, contents, url, buttons…)
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Tercih kategorisi anahtarı; kapalı kullanıcılar bastırılır
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Dönüşüm hedefleri (ADR-0080): birincil A/B kazananını belirler; olaylar dönüşüm olayı olarak tanımlı olmalı.
    #[serde(rename = "goals", skip_serializing_if = "Option::is_none")]
    pub goals: Option<CampaignsUpdateBodyGoals>,
    #[serde(rename = "ab_test", skip_serializing_if = "Option::is_none")]
    pub ab_test: Option<CampaignsUpdateBodyAbTest>,
    /// A/B varyantları (ağırlıklar toplamı 100; en fazla bir is_control)
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<CampaignsUpdateBodyVariantsItem>>,
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "template_version", skip_serializing_if = "Option::is_none")]
    pub template_version: Option<i64>,
    #[serde(rename = "delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CampaignsUpdateBodyDelivery>,
    #[serde(rename = "recurrence", skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<CampaignsUpdateBodyRecurrence>,
    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
}

/// `CampaignsUpdateBodyAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBodyAbTest {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "test_percentage", skip_serializing_if = "Option::is_none")]
    pub test_percentage: Option<i64>,
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<CampaignsUpdateBodyAbTestWinner>,
}

/// `CampaignsUpdateBodyAbTestWinner` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBodyAbTestWinner {
    /// Değerler: `"manual"`, `"auto"`.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Değerler: `"open_rate"`, `"click_rate"`, `"conversion_rate"`.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "evaluate_after_minutes", skip_serializing_if = "Option::is_none")]
    pub evaluate_after_minutes: Option<i64>,
}

/// `CampaignsUpdateBodyAudience` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBodyAudience {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<String>>,
    #[serde(rename = "exclude_segments", skip_serializing_if = "Option::is_none")]
    pub exclude_segments: Option<Vec<String>>,
    /// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<CampaignsUpdateBodyAudienceFilters>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "test_devices_only", skip_serializing_if = "Option::is_none")]
    pub test_devices_only: Option<bool>,
}

/// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBodyAudienceFilters {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// `CampaignsUpdateBodyDelivery` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBodyDelivery {
    /// Değerler: `"immediate"`, `"per_user_timezone"`, `"optimized"`.
    #[serde(rename = "timezone_mode", skip_serializing_if = "Option::is_none")]
    pub timezone_mode: Option<String>,
    #[serde(rename = "delivery_time_of_day", skip_serializing_if = "Option::is_none")]
    pub delivery_time_of_day: Option<String>,
    #[serde(rename = "throttle_per_minute", skip_serializing_if = "Option::is_none")]
    pub throttle_per_minute: Option<i64>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// Değerler: `5`, `10`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "collapse_id", skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    #[serde(rename = "frequency_cap_exempt", skip_serializing_if = "Option::is_none")]
    pub frequency_cap_exempt: Option<bool>,
    /// Değerler: `"low"`, `"normal"`, `"high"`.
    #[serde(rename = "frequency_priority", skip_serializing_if = "Option::is_none")]
    pub frequency_priority: Option<String>,
    #[serde(rename = "global_control_exempt", skip_serializing_if = "Option::is_none")]
    pub global_control_exempt: Option<bool>,
    #[serde(rename = "quiet_hours_exempt", skip_serializing_if = "Option::is_none")]
    pub quiet_hours_exempt: Option<bool>,
    #[serde(rename = "one_per_user", skip_serializing_if = "Option::is_none")]
    pub one_per_user: Option<bool>,
}

/// Dönüşüm hedefleri (ADR-0080): birincil A/B kazananını belirler; olaylar dönüşüm olayı olarak tanımlı olmalı.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBodyGoals {
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(rename = "secondary", skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<String>>,
}

/// `CampaignsUpdateBodyRecurrence` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBodyRecurrence {
    #[serde(rename = "cron", skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// `CampaignsUpdateBodyVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignsUpdateBodyVariantsItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "is_control", skip_serializing_if = "Option::is_none")]
    pub is_control: Option<bool>,
}

/// `CampaignVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "content")]
    pub content: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "is_control", deserialize_with = "crate::de::null_default")]
    pub is_control: bool,
    /// Zorunlu.
    #[serde(rename = "position", deserialize_with = "crate::de::null_default")]
    pub position: i64,
}

/// `CampaignVariantStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignVariantStats {
    /// Zorunlu.
    #[serde(rename = "ab_test", deserialize_with = "crate::de::null_default")]
    pub ab_test: CampaignVariantStatsAbTest,
    /// Zorunlu.
    #[serde(rename = "variants", deserialize_with = "crate::de::null_default")]
    pub variants: Vec<CampaignVariantStatsVariantsItem>,
    /// Zorunlu.
    #[serde(rename = "leader")]
    pub leader: Option<String>,
    /// Zorunlu.
    #[serde(rename = "significance", deserialize_with = "crate::de::null_default")]
    pub significance: CampaignVariantStatsSignificance,
}

/// `CampaignVariantStatsAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignVariantStatsAbTest {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "test_percentage", skip_serializing_if = "Option::is_none")]
    pub test_percentage: Option<i64>,
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<CampaignVariantStatsAbTestWinner>,
    /// Değerler: `"testing"`, `"winner_sent"`.
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(rename = "winner_variant_id", skip_serializing_if = "Option::is_none")]
    pub winner_variant_id: Option<String>,
    #[serde(rename = "decided_at", skip_serializing_if = "Option::is_none")]
    pub decided_at: Option<String>,
    #[serde(rename = "decided_by", skip_serializing_if = "Option::is_none")]
    pub decided_by: Option<String>,
    #[serde(rename = "significance", skip_serializing_if = "Option::is_none")]
    pub significance: Option<CampaignVariantStatsAbTestSignificance>,
}

/// `CampaignVariantStatsAbTestSignificance` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignVariantStatsAbTestSignificance {
    /// Zorunlu.
    #[serde(rename = "p_value", deserialize_with = "crate::de::null_default")]
    pub p_value: f64,
    /// Zorunlu.
    #[serde(rename = "significant", deserialize_with = "crate::de::null_default")]
    pub significant: bool,
    /// Zorunlu.
    #[serde(rename = "metric", deserialize_with = "crate::de::null_default")]
    pub metric: String,
}

/// `CampaignVariantStatsAbTestWinner` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignVariantStatsAbTestWinner {
    /// Zorunlu. Değerler: `"manual"`, `"auto"`.
    #[serde(rename = "mode", deserialize_with = "crate::de::null_default")]
    pub mode: String,
    /// Zorunlu. Değerler: `"open_rate"`, `"click_rate"`, `"conversion_rate"`.
    #[serde(rename = "metric", deserialize_with = "crate::de::null_default")]
    pub metric: String,
    #[serde(rename = "evaluate_after_minutes", skip_serializing_if = "Option::is_none")]
    pub evaluate_after_minutes: Option<i64>,
}

/// `CampaignVariantStatsSignificance` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignVariantStatsSignificance {
    /// Zorunlu.
    #[serde(rename = "p_value", deserialize_with = "crate::de::null_default")]
    pub p_value: f64,
    /// Zorunlu.
    #[serde(rename = "significant", deserialize_with = "crate::de::null_default")]
    pub significant: bool,
    /// Zorunlu.
    #[serde(rename = "metric", deserialize_with = "crate::de::null_default")]
    pub metric: String,
}

/// `CampaignVariantStatsVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignVariantStatsVariantsItem {
    /// Zorunlu.
    #[serde(rename = "variant_id", deserialize_with = "crate::de::null_default")]
    pub variant_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "is_control", deserialize_with = "crate::de::null_default")]
    pub is_control: bool,
    /// Zorunlu.
    #[serde(rename = "targeted", deserialize_with = "crate::de::null_default")]
    pub targeted: i64,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "delivered", deserialize_with = "crate::de::null_default")]
    pub delivered: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "holdout", deserialize_with = "crate::de::null_default")]
    pub holdout: i64,
    /// Zorunlu.
    #[serde(rename = "open_rate", deserialize_with = "crate::de::null_default")]
    pub open_rate: f64,
    /// Zorunlu.
    #[serde(rename = "click_rate", deserialize_with = "crate::de::null_default")]
    pub click_rate: f64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
    /// Zorunlu.
    #[serde(rename = "goals", deserialize_with = "crate::de::null_default")]
    pub goals: Vec<CampaignVariantStatsVariantsItemGoalsItem>,
}

/// `CampaignVariantStatsVariantsItemGoalsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CampaignVariantStatsVariantsItemGoalsItem {
    /// Zorunlu.
    #[serde(rename = "event", deserialize_with = "crate::de::null_default")]
    pub event: String,
    /// Zorunlu.
    #[serde(rename = "primary", deserialize_with = "crate::de::null_default")]
    pub primary: bool,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
}

/// `CannedInputDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CannedInputDto {
    /// Zorunlu.
    #[serde(rename = "shortcut", deserialize_with = "crate::de::null_default")]
    pub shortcut: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    /// Zorunlu.
    #[serde(rename = "body", deserialize_with = "crate::de::null_default")]
    pub body: String,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// `CannedResponse` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CannedResponse {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "shortcut", deserialize_with = "crate::de::null_default")]
    pub shortcut: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    /// Zorunlu.
    #[serde(rename = "body", deserialize_with = "crate::de::null_default")]
    pub body: String,
    /// Zorunlu.
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `CannedResponseList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CannedResponseList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<CannedResponse>,
}

/// `CannedUpdateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CannedUpdateDto {
    #[serde(rename = "shortcut", skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// `Catalog` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Catalog {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "currency", deserialize_with = "crate::de::null_default")]
    pub currency: String,
    /// Zorunlu.
    #[serde(rename = "item_count", deserialize_with = "crate::de::null_default")]
    pub item_count: i64,
    /// Zorunlu.
    #[serde(rename = "liquid", deserialize_with = "crate::de::null_default")]
    pub liquid: String,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `CatalogItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CatalogItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "price")]
    pub price: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "sale_price")]
    pub sale_price: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "currency", deserialize_with = "crate::de::null_default")]
    pub currency: String,
    /// Zorunlu.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Zorunlu.
    #[serde(rename = "brand")]
    pub brand: Option<String>,
    /// Zorunlu.
    #[serde(rename = "in_stock", deserialize_with = "crate::de::null_default")]
    pub in_stock: bool,
    /// Zorunlu.
    #[serde(rename = "attributes", deserialize_with = "crate::de::null_default")]
    pub attributes: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `CatalogItemsPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CatalogItemsPage {
    /// Zorunlu.
    #[serde(rename = "total", deserialize_with = "crate::de::null_default")]
    pub total: i64,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<CatalogItem>,
}

/// `CatalogList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CatalogList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Catalog>,
}

/// `CatalogUpsertResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CatalogUpsertResult {
    /// Zorunlu.
    #[serde(rename = "catalog", deserialize_with = "crate::de::null_default")]
    pub catalog: Catalog,
    /// Zorunlu.
    #[serde(rename = "upserted", deserialize_with = "crate::de::null_default")]
    pub upserted: i64,
}

/// `ChannelConfig` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelConfig {
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu. Değerler: `"apns"`, `"fcm"`, `"hms"`, `"adm"`, `"wns"`, `"webpush"`, `"smtp"`, `"ses"`, `"resend"`, `"twilio"`, `"netgsm"`, `"meta_whatsapp"`, `"google_rbm"`, `"telegram_bot"`, `"viber_bot"`, `"mock"`.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Zorunlu. Değerler: `"unconfigured"`, `"active"`, `"invalid"`, `"suspended"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "config", deserialize_with = "crate::de::null_default")]
    pub config: ChannelConfigConfig,
    /// Zorunlu.
    #[serde(rename = "has_secret", deserialize_with = "crate::de::null_default")]
    pub has_secret: bool,
    /// Zorunlu.
    #[serde(rename = "has_hook", deserialize_with = "crate::de::null_default")]
    pub has_hook: bool,
    /// Zorunlu.
    #[serde(rename = "last_validated_at")]
    pub last_validated_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "suspended_reason")]
    pub suspended_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "dns_check")]
    pub dns_check: Option<ChannelConfigDnsCheck>,
}

/// `ChannelConfigConfig` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelConfigConfig {
    /// Değerler: `"token"`, `"certificate"`.
    #[serde(rename = "auth_type", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// Değerler: `"sandbox"`, `"production"`, `"universal"`.
    #[serde(rename = "certificate_environment", skip_serializing_if = "Option::is_none")]
    pub certificate_environment: Option<String>,
    #[serde(rename = "certificate_subject", skip_serializing_if = "Option::is_none")]
    pub certificate_subject: Option<String>,
    #[serde(rename = "certificate_expires_at", skip_serializing_if = "Option::is_none")]
    pub certificate_expires_at: Option<String>,
    #[serde(rename = "key_id", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "bundle_id", skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Değerler: `"sandbox"`, `"production"`.
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "client_email", skip_serializing_if = "Option::is_none")]
    pub client_email: Option<String>,
    #[serde(rename = "hms_app_id", skip_serializing_if = "Option::is_none")]
    pub hms_app_id: Option<String>,
    #[serde(rename = "high_priority", skip_serializing_if = "Option::is_none")]
    pub high_priority: Option<bool>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "vapid_public_key", skip_serializing_if = "Option::is_none")]
    pub vapid_public_key: Option<String>,
    #[serde(rename = "site_url", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(rename = "from_address", skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(rename = "from_name", skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    #[serde(rename = "reply_to", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(rename = "dkim_selector", skip_serializing_if = "Option::is_none")]
    pub dkim_selector: Option<String>,
    #[serde(rename = "postal_address", skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<String>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "access_key_id", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "configuration_set", skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "from_number", skip_serializing_if = "Option::is_none")]
    pub from_number: Option<String>,
    #[serde(rename = "messaging_service_sid", skip_serializing_if = "Option::is_none")]
    pub messaging_service_sid: Option<String>,
    #[serde(rename = "verify_service_sid", skip_serializing_if = "Option::is_none")]
    pub verify_service_sid: Option<String>,
    #[serde(rename = "usercode", skip_serializing_if = "Option::is_none")]
    pub usercode: Option<String>,
    #[serde(rename = "msgheader", skip_serializing_if = "Option::is_none")]
    pub msgheader: Option<String>,
    /// Değerler: `"0"`, `"11"`, `"12"`.
    #[serde(rename = "iys_filter", skip_serializing_if = "Option::is_none")]
    pub iys_filter: Option<String>,
    #[serde(rename = "waba_id", skip_serializing_if = "Option::is_none")]
    pub waba_id: Option<String>,
    #[serde(rename = "phone_number_id", skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    #[serde(rename = "graph_api_version", skip_serializing_if = "Option::is_none")]
    pub graph_api_version: Option<String>,
    #[serde(rename = "display_phone_number", skip_serializing_if = "Option::is_none")]
    pub display_phone_number: Option<String>,
    #[serde(rename = "verified_name", skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
    #[serde(rename = "rbm_agent_id", skip_serializing_if = "Option::is_none")]
    pub rbm_agent_id: Option<String>,
    #[serde(rename = "content_sid", skip_serializing_if = "Option::is_none")]
    pub content_sid: Option<String>,
    #[serde(rename = "sms_fallback", skip_serializing_if = "Option::is_none")]
    pub sms_fallback: Option<bool>,
    #[serde(rename = "telegram_bot_username", skip_serializing_if = "Option::is_none")]
    pub telegram_bot_username: Option<String>,
    #[serde(rename = "viber_uri", skip_serializing_if = "Option::is_none")]
    pub viber_uri: Option<String>,
    #[serde(rename = "viber_sender_name", skip_serializing_if = "Option::is_none")]
    pub viber_sender_name: Option<String>,
    #[serde(rename = "viber_sender_avatar", skip_serializing_if = "Option::is_none")]
    pub viber_sender_avatar: Option<String>,
    #[serde(rename = "webhook_registered_at", skip_serializing_if = "Option::is_none")]
    pub webhook_registered_at: Option<String>,
}

/// `ChannelConfigDnsCheck` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelConfigDnsCheck {
    /// Zorunlu.
    #[serde(rename = "domain", deserialize_with = "crate::de::null_default")]
    pub domain: String,
    /// Zorunlu.
    #[serde(rename = "checked_at", deserialize_with = "crate::de::null_default")]
    pub checked_at: String,
    /// Zorunlu.
    #[serde(rename = "spf", deserialize_with = "crate::de::null_default")]
    pub spf: ChannelConfigDnsCheckSpf,
    /// Zorunlu.
    #[serde(rename = "dkim", deserialize_with = "crate::de::null_default")]
    pub dkim: ChannelConfigDnsCheckDkim,
    /// Zorunlu.
    #[serde(rename = "dmarc", deserialize_with = "crate::de::null_default")]
    pub dmarc: ChannelConfigDnsCheckDmarc,
    /// Zorunlu.
    #[serde(rename = "issues", deserialize_with = "crate::de::null_default")]
    pub issues: Vec<String>,
}

/// `ChannelConfigDnsCheckDkim` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelConfigDnsCheckDkim {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
    /// Zorunlu.
    #[serde(rename = "selector", deserialize_with = "crate::de::null_default")]
    pub selector: String,
    /// Zorunlu.
    #[serde(rename = "record")]
    pub record: Option<String>,
    /// Zorunlu.
    #[serde(rename = "expected", deserialize_with = "crate::de::null_default")]
    pub expected: String,
}

/// `ChannelConfigDnsCheckDmarc` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelConfigDnsCheckDmarc {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
    /// Zorunlu.
    #[serde(rename = "record")]
    pub record: Option<String>,
    /// Zorunlu.
    #[serde(rename = "expected", deserialize_with = "crate::de::null_default")]
    pub expected: String,
}

/// `ChannelConfigDnsCheckSpf` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelConfigDnsCheckSpf {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
    /// Zorunlu.
    #[serde(rename = "record")]
    pub record: Option<String>,
    /// Zorunlu.
    #[serde(rename = "expected", deserialize_with = "crate::de::null_default")]
    pub expected: String,
}

/// `ChannelList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<ChannelConfig>,
}

/// `ChannelScoreConfig` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelScoreConfig {
    /// Zorunlu.
    #[serde(rename = "configured", deserialize_with = "crate::de::null_default")]
    pub configured: bool,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "window_days", deserialize_with = "crate::de::null_default")]
    pub window_days: i64,
    /// Zorunlu.
    #[serde(rename = "min_sends", deserialize_with = "crate::de::null_default")]
    pub min_sends: i64,
    /// Zorunlu.
    #[serde(rename = "count_email_opens", deserialize_with = "crate::de::null_default")]
    pub count_email_opens: bool,
    /// Zorunlu. Değerler: `"idle"`, `"queued"`, `"running"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_computed_at")]
    pub last_computed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "scored_users", deserialize_with = "crate::de::null_default")]
    pub scored_users: i64,
}

/// `ChannelScoreOverview` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelScoreOverview {
    /// Zorunlu.
    #[serde(rename = "total", deserialize_with = "crate::de::null_default")]
    pub total: i64,
    /// Zorunlu.
    #[serde(rename = "low_population", deserialize_with = "crate::de::null_default")]
    pub low_population: bool,
    /// Zorunlu.
    #[serde(rename = "last_computed_at")]
    pub last_computed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "channels", deserialize_with = "crate::de::null_default")]
    pub channels: Vec<ChannelScoreOverviewChannelsItem>,
}

/// `ChannelScoreOverviewChannelsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelScoreOverviewChannelsItem {
    /// Zorunlu. Değerler: `"push"`, `"email"`, `"whatsapp"`, `"rcs"`, `"sms"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "best_users", deserialize_with = "crate::de::null_default")]
    pub best_users: i64,
    /// Zorunlu.
    #[serde(rename = "share", deserialize_with = "crate::de::null_default")]
    pub share: f64,
    /// Zorunlu.
    #[serde(rename = "sends", deserialize_with = "crate::de::null_default")]
    pub sends: i64,
    /// Zorunlu.
    #[serde(rename = "engaged", deserialize_with = "crate::de::null_default")]
    pub engaged: i64,
    /// Zorunlu.
    #[serde(rename = "engagement_rate")]
    pub engagement_rate: Option<f64>,
}

/// `ChannelScoreUser` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelScoreUser {
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu. Değerler: `"push"`, `"email"`, `"whatsapp"`, `"rcs"`, `"sms"`.
    #[serde(rename = "best_channel")]
    pub best_channel: Option<String>,
    /// Zorunlu.
    #[serde(rename = "scores", deserialize_with = "crate::de::null_default")]
    pub scores: Vec<ChannelScoreUserScoresItem>,
}

/// `ChannelScoreUserScoresItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelScoreUserScoresItem {
    /// Zorunlu. Değerler: `"push"`, `"email"`, `"whatsapp"`, `"rcs"`, `"sms"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "sends", deserialize_with = "crate::de::null_default")]
    pub sends: i64,
    /// Zorunlu.
    #[serde(rename = "engaged", deserialize_with = "crate::de::null_default")]
    pub engaged: i64,
    /// Zorunlu.
    #[serde(rename = "score", deserialize_with = "crate::de::null_default")]
    pub score: f64,
}

/// `ChannelScoreUsersPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelScoreUsersPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<ChannelScoreUser>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
}

/// `ChannelValidation` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChannelValidation {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
    /// Zorunlu. Değerler: `"auth"`, `"config"`, `"transient"`.
    #[serde(rename = "kind")]
    pub kind: Option<String>,
    /// Zorunlu.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Zorunlu.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Zorunlu.
    #[serde(rename = "details")]
    pub details: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: ChannelConfig,
}

/// `CheckVerifyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CheckVerifyDto {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
}

/// `ContentTable` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ContentTable {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "key_column", deserialize_with = "crate::de::null_default")]
    pub key_column: String,
    /// Zorunlu.
    #[serde(rename = "columns", deserialize_with = "crate::de::null_default")]
    pub columns: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "row_count", deserialize_with = "crate::de::null_default")]
    pub row_count: i64,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "source_file_name")]
    pub source_file_name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "liquid", deserialize_with = "crate::de::null_default")]
    pub liquid: String,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `ContentTableList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ContentTableList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<ContentTable>,
}

/// `ContentTableRows` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ContentTableRows {
    /// Zorunlu.
    #[serde(rename = "columns", deserialize_with = "crate::de::null_default")]
    pub columns: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "key_column", deserialize_with = "crate::de::null_default")]
    pub key_column: String,
    /// Zorunlu.
    #[serde(rename = "total", deserialize_with = "crate::de::null_default")]
    pub total: i64,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<std::collections::HashMap<String, String>>,
}

/// `Conversation` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Conversation {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`, `"email"`, `"chat"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu. Değerler: `"open"`, `"pending"`, `"resolved"`, `"closed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "subscription_id")]
    pub subscription_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "subscription_status")]
    pub subscription_status: Option<String>,
    /// Zorunlu.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "contact", deserialize_with = "crate::de::null_default")]
    pub contact: String,
    /// Zorunlu.
    #[serde(rename = "opted_in", deserialize_with = "crate::de::null_default")]
    pub opted_in: bool,
    /// Zorunlu.
    #[serde(rename = "last_message_at", deserialize_with = "crate::de::null_default")]
    pub last_message_at: String,
    /// Zorunlu.
    #[serde(rename = "last_inbound_at")]
    pub last_inbound_at: Option<String>,
    /// Zorunlu. Değerler: `"inbound"`, `"outbound"`.
    #[serde(rename = "last_direction")]
    pub last_direction: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_message_preview")]
    pub last_message_preview: Option<String>,
    /// Zorunlu.
    #[serde(rename = "unread_count", deserialize_with = "crate::de::null_default")]
    pub unread_count: i64,
    /// Zorunlu.
    #[serde(rename = "window")]
    pub window: Option<ConversationWindow>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "assignee_id")]
    pub assignee_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "assigned_at")]
    pub assigned_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Zorunlu.
    #[serde(rename = "bot_active", deserialize_with = "crate::de::null_default")]
    pub bot_active: bool,
    /// Zorunlu.
    #[serde(rename = "last_seq", deserialize_with = "crate::de::null_default")]
    pub last_seq: i64,
    /// Zorunlu.
    #[serde(rename = "visitor_last_read_seq", deserialize_with = "crate::de::null_default")]
    pub visitor_last_read_seq: i64,
    /// Zorunlu.
    #[serde(rename = "visitor_last_delivered_seq", deserialize_with = "crate::de::null_default")]
    pub visitor_last_delivered_seq: i64,
    /// Zorunlu.
    #[serde(rename = "agent_last_read_seq", deserialize_with = "crate::de::null_default")]
    pub agent_last_read_seq: i64,
    /// Zorunlu.
    #[serde(rename = "waiting_since")]
    pub waiting_since: Option<String>,
    /// Zorunlu.
    #[serde(rename = "first_response_at")]
    pub first_response_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "sla_due_at")]
    pub sla_due_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "resolved_at")]
    pub resolved_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "visitor")]
    pub visitor: Option<ConversationVisitor>,
}

/// `ConversationActivity` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationActivity {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<ConversationActivityDataItem>,
}

/// `ConversationActivityDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationActivityDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "actor_type", deserialize_with = "crate::de::null_default")]
    pub actor_type: String,
    /// Zorunlu.
    #[serde(rename = "actor_id")]
    pub actor_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `ConversationAssignDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationAssignDto {
    #[serde(rename = "assignee_id", skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

/// `ConversationMessagePage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationMessagePage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<ConversationMessagePageDataItem>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `ConversationMessagePageDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationMessagePageDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"inbound"`, `"outbound"`.
    #[serde(rename = "direction", deserialize_with = "crate::de::null_default")]
    pub direction: String,
    /// Zorunlu.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Zorunlu.
    #[serde(rename = "content", deserialize_with = "crate::de::null_default")]
    pub content: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "delivery_id")]
    pub delivery_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "sent_by")]
    pub sent_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "error_code")]
    pub error_code: Option<String>,
    /// Zorunlu.
    #[serde(rename = "read_at")]
    pub read_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "seq", deserialize_with = "crate::de::null_default")]
    pub seq: i64,
    /// Zorunlu.
    #[serde(rename = "sender_type", deserialize_with = "crate::de::null_default")]
    pub sender_type: String,
    /// Zorunlu.
    #[serde(rename = "client_message_id")]
    pub client_message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "attachments", deserialize_with = "crate::de::null_default")]
    pub attachments: Vec<ConversationMessagePageDataItemAttachmentsItem>,
    /// Zorunlu.
    #[serde(rename = "quick_replies")]
    pub quick_replies: Option<Vec<ConversationMessagePageDataItemQuickRepliesItem>>,
}

/// `ConversationMessagePageDataItemAttachmentsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationMessagePageDataItemAttachmentsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "size", deserialize_with = "crate::de::null_default")]
    pub size: i64,
}

/// `ConversationMessagePageDataItemQuickRepliesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationMessagePageDataItemQuickRepliesItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
}

/// `ConversationPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Conversation>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `ConversationReplyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDto {
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<ConversationReplyDtoTemplate>,
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<ConversationReplyDtoMedia>,
    #[serde(rename = "interactive", skip_serializing_if = "Option::is_none")]
    pub interactive: Option<ConversationReplyDtoInteractive>,
    #[serde(rename = "client_message_id", skip_serializing_if = "Option::is_none")]
    pub client_message_id: Option<String>,
    /// Değerler: `"manual"`, `"canned"`, `"ai_suggestion"`.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// `ConversationReplyDtoInteractive` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDtoInteractive {
    /// Zorunlu. Değerler: `"button"`, `"list"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<ConversationReplyDtoInteractiveButtonsItem>>,
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<ConversationReplyDtoInteractiveList>,
}

/// `ConversationReplyDtoInteractiveButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDtoInteractiveButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
}

/// `ConversationReplyDtoInteractiveList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDtoInteractiveList {
    /// Zorunlu.
    #[serde(rename = "button", deserialize_with = "crate::de::null_default")]
    pub button: String,
    /// Zorunlu.
    #[serde(rename = "sections", deserialize_with = "crate::de::null_default")]
    pub sections: Vec<ConversationReplyDtoInteractiveListSectionsItem>,
}

/// `ConversationReplyDtoInteractiveListSectionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDtoInteractiveListSectionsItem {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<ConversationReplyDtoInteractiveListSectionsItemRowsItem>,
}

/// `ConversationReplyDtoInteractiveListSectionsItemRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDtoInteractiveListSectionsItemRowsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `ConversationReplyDtoMedia` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDtoMedia {
    /// Zorunlu. Değerler: `"image"`, `"video"`, `"document"`, `"audio"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// `ConversationReplyDtoTemplate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDtoTemplate {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<serde_json::Value>,
    #[serde(rename = "body_params", skip_serializing_if = "Option::is_none")]
    pub body_params: Option<Vec<String>>,
    #[serde(rename = "button_params", skip_serializing_if = "Option::is_none")]
    pub button_params: Option<Vec<ConversationReplyDtoTemplateButtonParamsItem>>,
}

/// `ConversationReplyDtoTemplateButtonParamsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyDtoTemplateButtonParamsItem {
    /// Zorunlu.
    #[serde(rename = "index", deserialize_with = "crate::de::null_default")]
    pub index: i64,
    /// Zorunlu. Değerler: `"url"`, `"quick_reply"`, `"copy_code"`.
    #[serde(rename = "sub_type", deserialize_with = "crate::de::null_default")]
    pub sub_type: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `ConversationReplyResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyResult {
    /// Zorunlu.
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "conversation_message", deserialize_with = "crate::de::null_default")]
    pub conversation_message: ConversationReplyResultConversationMessage,
}

/// `ConversationReplyResultConversationMessage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyResultConversationMessage {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"inbound"`, `"outbound"`.
    #[serde(rename = "direction", deserialize_with = "crate::de::null_default")]
    pub direction: String,
    /// Zorunlu.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Zorunlu.
    #[serde(rename = "content", deserialize_with = "crate::de::null_default")]
    pub content: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "delivery_id")]
    pub delivery_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "sent_by")]
    pub sent_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "error_code")]
    pub error_code: Option<String>,
    /// Zorunlu.
    #[serde(rename = "read_at")]
    pub read_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "seq", deserialize_with = "crate::de::null_default")]
    pub seq: i64,
    /// Zorunlu.
    #[serde(rename = "sender_type", deserialize_with = "crate::de::null_default")]
    pub sender_type: String,
    /// Zorunlu.
    #[serde(rename = "client_message_id")]
    pub client_message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "attachments", deserialize_with = "crate::de::null_default")]
    pub attachments: Vec<ConversationReplyResultConversationMessageAttachmentsItem>,
    /// Zorunlu.
    #[serde(rename = "quick_replies")]
    pub quick_replies: Option<Vec<ConversationReplyResultConversationMessageQuickRepliesItem>>,
}

/// `ConversationReplyResultConversationMessageAttachmentsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyResultConversationMessageAttachmentsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "size", deserialize_with = "crate::de::null_default")]
    pub size: i64,
}

/// `ConversationReplyResultConversationMessageQuickRepliesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationReplyResultConversationMessageQuickRepliesItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
}

/// `ConversationTagsDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationTagsDto {
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

/// `ConversationTagsResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationTagsResult {
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "tags", deserialize_with = "crate::de::null_default")]
    pub tags: std::collections::HashMap<String, serde_json::Value>,
}

/// `ConversationUpdateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationUpdateDto {
    /// Zorunlu. Değerler: `"open"`, `"pending"`, `"resolved"`, `"closed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
}

/// `ConversationVisitor` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationVisitor {
    /// Zorunlu.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "email")]
    pub email: Option<String>,
}

/// `ConversationWindow` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversationWindow {
    /// Zorunlu.
    #[serde(rename = "open", deserialize_with = "crate::de::null_default")]
    pub open: bool,
    /// Zorunlu.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
}

/// `ConversionsReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversionsReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: ConversionsReportTotals,
    /// Zorunlu.
    #[serde(rename = "previous")]
    pub previous: Option<ConversionsReportPrevious>,
    /// Zorunlu.
    #[serde(rename = "by_campaign", deserialize_with = "crate::de::null_default")]
    pub by_campaign: Vec<ConversionsReportByCampaignItem>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<ConversionsReportRowsItem>,
}

/// `ConversionsReportByCampaignItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversionsReportByCampaignItem {
    /// Zorunlu.
    #[serde(rename = "campaign_id")]
    pub campaign_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "direct", deserialize_with = "crate::de::null_default")]
    pub direct: i64,
    /// Zorunlu.
    #[serde(rename = "influenced", deserialize_with = "crate::de::null_default")]
    pub influenced: i64,
    /// Zorunlu.
    #[serde(rename = "unattributed", deserialize_with = "crate::de::null_default")]
    pub unattributed: i64,
    /// Zorunlu.
    #[serde(rename = "revenue", deserialize_with = "crate::de::null_default")]
    pub revenue: f64,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate")]
    pub conversion_rate: Option<f64>,
}

/// `ConversionsReportPrevious` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversionsReportPrevious {
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "direct", deserialize_with = "crate::de::null_default")]
    pub direct: i64,
    /// Zorunlu.
    #[serde(rename = "influenced", deserialize_with = "crate::de::null_default")]
    pub influenced: i64,
    /// Zorunlu.
    #[serde(rename = "unattributed", deserialize_with = "crate::de::null_default")]
    pub unattributed: i64,
    /// Zorunlu.
    #[serde(rename = "revenue", deserialize_with = "crate::de::null_default")]
    pub revenue: f64,
}

/// `ConversionsReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversionsReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "revenue", deserialize_with = "crate::de::null_default")]
    pub revenue: f64,
}

/// `ConversionsReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConversionsReportTotals {
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "direct", deserialize_with = "crate::de::null_default")]
    pub direct: i64,
    /// Zorunlu.
    #[serde(rename = "influenced", deserialize_with = "crate::de::null_default")]
    pub influenced: i64,
    /// Zorunlu.
    #[serde(rename = "unattributed", deserialize_with = "crate::de::null_default")]
    pub unattributed: i64,
    /// Zorunlu.
    #[serde(rename = "revenue", deserialize_with = "crate::de::null_default")]
    pub revenue: f64,
}

/// `CopyTemplateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CopyTemplateDto {
    /// Zorunlu.
    #[serde(rename = "target_app_id", deserialize_with = "crate::de::null_default")]
    pub target_app_id: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `CreateAdAudienceDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAdAudienceDto {
    /// Zorunlu.
    #[serde(rename = "ad_account_id", deserialize_with = "crate::de::null_default")]
    pub ad_account_id: String,
    /// Zorunlu. Değerler: `"segment"`, `"journey"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    #[serde(rename = "segment_id", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Değerler: `"email"`, `"phone"`, `"maid"`.
    #[serde(rename = "identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<String>>,
    #[serde(rename = "sync_interval_minutes", skip_serializing_if = "Option::is_none")]
    pub sync_interval_minutes: Option<i64>,
}

/// `CreateAppApiKeyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAppApiKeyDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"admin"`, `"notifications:write"`, `"users:read"`, `"users:write"`, `"events:write"`, `"segments:read"`, `"analytics:read"`, `"mcp:write"`.
    #[serde(rename = "scopes", deserialize_with = "crate::de::null_default")]
    pub scopes: Vec<String>,
    /// Değerler: `"live"`, `"test"`.
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "ip_allowlist", skip_serializing_if = "Option::is_none")]
    pub ip_allowlist: Option<Vec<String>>,
}

/// `CreateAppDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAppDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "platform_flags", skip_serializing_if = "Option::is_none")]
    pub platform_flags: Option<CreateAppDtoPlatformFlags>,
    #[serde(rename = "default_language", skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// `CreateAppDtoPlatformFlags` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAppDtoPlatformFlags {
    #[serde(rename = "ios", skip_serializing_if = "Option::is_none")]
    pub ios: Option<bool>,
    #[serde(rename = "android", skip_serializing_if = "Option::is_none")]
    pub android: Option<bool>,
    #[serde(rename = "web", skip_serializing_if = "Option::is_none")]
    pub web: Option<bool>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
}

/// `CreateAppFlagDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAppFlagDto {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Değerler: `"boolean"`, `"string"`, `"number"`, `"json"`.
    #[serde(rename = "value_type", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(rename = "on_value", skip_serializing_if = "Option::is_none")]
    pub on_value: Option<serde_json::Value>,
    #[serde(rename = "off_value", skip_serializing_if = "Option::is_none")]
    pub off_value: Option<serde_json::Value>,
    #[serde(rename = "rollout_percent", skip_serializing_if = "Option::is_none")]
    pub rollout_percent: Option<i64>,
    #[serde(rename = "segment_id", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "rollout_schedule", skip_serializing_if = "Option::is_none")]
    pub rollout_schedule: Option<Vec<CreateAppFlagDtoRolloutScheduleItem>>,
}

/// `CreateAppFlagDtoRolloutScheduleItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateAppFlagDtoRolloutScheduleItem {
    /// Zorunlu.
    #[serde(rename = "at", deserialize_with = "crate::de::null_default")]
    pub at: String,
    /// Zorunlu.
    #[serde(rename = "percent", deserialize_with = "crate::de::null_default")]
    pub percent: i64,
}

/// `CreateBudgetAlertRuleDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateBudgetAlertRuleDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"daily_spend"`, `"roas_drop"`, `"budget_depletion"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    #[serde(rename = "ad_account_id", skip_serializing_if = "Option::is_none")]
    pub ad_account_id: Option<String>,
    #[serde(rename = "campaign_external_id", skip_serializing_if = "Option::is_none")]
    pub campaign_external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "threshold", deserialize_with = "crate::de::null_default")]
    pub threshold: f64,
    #[serde(rename = "window_days", skip_serializing_if = "Option::is_none")]
    pub window_days: Option<i64>,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// `CreateCatalogDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateCatalogDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// `CreateCategoryDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateCategoryDto {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: std::collections::HashMap<String, String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<std::collections::HashMap<String, String>>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "default_opted_in", skip_serializing_if = "Option::is_none")]
    pub default_opted_in: Option<bool>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

/// `CreatedApiKey` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreatedApiKey {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "org_id", deserialize_with = "crate::de::null_default")]
    pub org_id: String,
    /// Zorunlu.
    #[serde(rename = "app_id")]
    pub app_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "prefix", deserialize_with = "crate::de::null_default")]
    pub prefix: String,
    /// Zorunlu.
    #[serde(rename = "scopes", deserialize_with = "crate::de::null_default")]
    pub scopes: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "last_used_at")]
    pub last_used_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "revoked_at")]
    pub revoked_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "ip_allowlist", deserialize_with = "crate::de::null_default")]
    pub ip_allowlist: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "rotated_at")]
    pub rotated_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
}

/// `CreateDataFeedDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateDataFeedDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "url_template", deserialize_with = "crate::de::null_default")]
    pub url_template: String,
    /// Değerler: `"GET"`, `"POST"`.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "body_template", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "secret_headers", skip_serializing_if = "Option::is_none")]
    pub secret_headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "cache_ttl_seconds", skip_serializing_if = "Option::is_none")]
    pub cache_ttl_seconds: Option<i64>,
    #[serde(rename = "timeout_ms", skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// `CreateDecisionAgentDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateDecisionAgentDto {
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "arms", deserialize_with = "crate::de::null_default")]
    pub arms: Vec<CreateDecisionAgentDtoArmsItem>,
    /// Zorunlu.
    #[serde(rename = "reward_events", deserialize_with = "crate::de::null_default")]
    pub reward_events: Vec<String>,
    #[serde(rename = "reward_window_hours", skip_serializing_if = "Option::is_none")]
    pub reward_window_hours: Option<i64>,
    #[serde(rename = "holdout_percent", skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i64>,
    #[serde(rename = "context_prediction", skip_serializing_if = "Option::is_none")]
    pub context_prediction: Option<String>,
    #[serde(rename = "use_best_channel", skip_serializing_if = "Option::is_none")]
    pub use_best_channel: Option<bool>,
}

/// `CreateDecisionAgentDtoArmsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateDecisionAgentDtoArmsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
}

/// `CreateExportDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateExportDto {
    /// Zorunlu. Değerler: `"users"`, `"subscriptions"`, `"events"`, `"deliveries"`, `"message_events"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<CreateExportDtoFilters>,
}

/// `CreateExportDtoFilters` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateExportDtoFilters {
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "campaign_id", skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "segment_id", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
}

/// `CreateFormDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFormDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "fields", deserialize_with = "crate::de::null_default")]
    pub fields: Vec<CreateFormDtoFieldsItem>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateFormDtoSettings>,
    /// Değerler: `"active"`, `"archived"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// `CreateFormDtoFieldsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFormDtoFieldsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"email"`, `"phone"`, `"first_name"`, `"last_name"`, `"full_name"`, `"text"`, `"textarea"`, `"select"`, `"radio"`, `"checkbox"`, `"consent"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: std::collections::HashMap<String, String>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CreateFormDtoFieldsItemOptionsItem>>,
    #[serde(rename = "maps_to", skip_serializing_if = "Option::is_none")]
    pub maps_to: Option<String>,
    #[serde(rename = "max_length", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "policy_url", skip_serializing_if = "Option::is_none")]
    pub policy_url: Option<String>,
}

/// `CreateFormDtoFieldsItemOptionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFormDtoFieldsItemOptionsItem {
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: std::collections::HashMap<String, String>,
}

/// `CreateFormDtoSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFormDtoSettings {
    #[serde(rename = "submit_label", skip_serializing_if = "Option::is_none")]
    pub submit_label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "success_message", skip_serializing_if = "Option::is_none")]
    pub success_message: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<CreateFormDtoSettingsSubscribe>,
    #[serde(rename = "double_opt_in", skip_serializing_if = "Option::is_none")]
    pub double_opt_in: Option<CreateFormDtoSettingsDoubleOptIn>,
    #[serde(rename = "event_name", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "captcha", skip_serializing_if = "Option::is_none")]
    pub captcha: Option<bool>,
    #[serde(rename = "default_language", skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
}

/// `CreateFormDtoSettingsDoubleOptIn` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFormDtoSettingsDoubleOptIn {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<std::collections::HashMap<String, String>>,
}

/// `CreateFormDtoSettingsSubscribe` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateFormDtoSettingsSubscribe {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
}

/// `CreateGeofenceDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateGeofenceDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "lat", deserialize_with = "crate::de::null_default")]
    pub lat: f64,
    /// Zorunlu.
    #[serde(rename = "lng", deserialize_with = "crate::de::null_default")]
    pub lng: f64,
    /// Zorunlu.
    #[serde(rename = "radius_m", deserialize_with = "crate::de::null_default")]
    pub radius_m: i64,
    #[serde(rename = "cooldown_minutes", skip_serializing_if = "Option::is_none")]
    pub cooldown_minutes: Option<i64>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// `CreateIdentityKeyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateIdentityKeyDto {
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "public_jwk", skip_serializing_if = "Option::is_none")]
    pub public_jwk: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "public_key_pem", skip_serializing_if = "Option::is_none")]
    pub public_key_pem: Option<String>,
}

/// `CreateInAppDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateInAppDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "triggers", deserialize_with = "crate::de::null_default")]
    pub triggers: Vec<serde_json::Value>,
    #[serde(rename = "trigger_groups", skip_serializing_if = "Option::is_none")]
    pub trigger_groups: Option<Vec<Vec<serde_json::Value>>>,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<CreateInAppDtoVariantsItem>>,
    #[serde(rename = "ab_test", skip_serializing_if = "Option::is_none")]
    pub ab_test: Option<CreateInAppDtoAbTest>,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: CreateInAppDtoDisplay,
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<CreateInAppDtoAudience>,
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<CreateInAppDtoFrequency>,
    #[serde(rename = "journey_only", skip_serializing_if = "Option::is_none")]
    pub journey_only: Option<bool>,
    #[serde(rename = "placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
}

/// `CreateInAppDtoAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateInAppDtoAbTest {
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<CreateInAppDtoAbTestWinner>,
}

/// `CreateInAppDtoAbTestWinner` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateInAppDtoAbTestWinner {
    /// Zorunlu. Değerler: `"manual"`, `"auto"`.
    #[serde(rename = "mode", deserialize_with = "crate::de::null_default")]
    pub mode: String,
    #[serde(rename = "min_impressions", skip_serializing_if = "Option::is_none")]
    pub min_impressions: Option<i64>,
    #[serde(rename = "confidence", skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// `CreateInAppDtoAudience` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateInAppDtoAudience {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    #[serde(rename = "global_control_exempt", skip_serializing_if = "Option::is_none")]
    pub global_control_exempt: Option<bool>,
}

/// `CreateInAppDtoDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateInAppDtoDisplay {
    /// Zorunlu. Değerler: `"modal"`, `"banner_top"`, `"banner_bottom"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    /// Değerler: `"blocks"`, `"html"`.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "html_height", skip_serializing_if = "Option::is_none")]
    pub html_height: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
}

/// `CreateInAppDtoFrequency` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateInAppDtoFrequency {
    #[serde(rename = "max_impressions", skip_serializing_if = "Option::is_none")]
    pub max_impressions: Option<i64>,
    #[serde(rename = "min_interval_seconds", skip_serializing_if = "Option::is_none")]
    pub min_interval_seconds: Option<i64>,
    #[serde(rename = "stop_after_interaction", skip_serializing_if = "Option::is_none")]
    pub stop_after_interaction: Option<bool>,
}

/// `CreateInAppDtoVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateInAppDtoVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: CreateInAppDtoVariantsItemDisplay,
}

/// `CreateInAppDtoVariantsItemDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateInAppDtoVariantsItemDisplay {
    /// Zorunlu. Değerler: `"modal"`, `"banner_top"`, `"banner_bottom"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    /// Değerler: `"blocks"`, `"html"`.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "html_height", skip_serializing_if = "Option::is_none")]
    pub html_height: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
}

/// `CreateKeywordDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateKeywordDto {
    /// Zorunlu.
    #[serde(rename = "keyword", deserialize_with = "crate::de::null_default")]
    pub keyword: String,
    /// Değerler: `"org"`, `"app"`.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Değerler: `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    /// Zorunlu.
    #[serde(rename = "actions", deserialize_with = "crate::de::null_default")]
    pub actions: CreateKeywordDtoActions,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `CreateKeywordDtoActions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateKeywordDtoActions {
    #[serde(rename = "add_tags", skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "remove_tags", skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<CreateKeywordDtoActionsEvent>,
    #[serde(rename = "reply", skip_serializing_if = "Option::is_none")]
    pub reply: Option<CreateKeywordDtoActionsReply>,
    #[serde(rename = "opt_in", skip_serializing_if = "Option::is_none")]
    pub opt_in: Option<bool>,
}

/// `CreateKeywordDtoActionsEvent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateKeywordDtoActionsEvent {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

/// `CreateKeywordDtoActionsReply` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateKeywordDtoActionsReply {
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
}

/// `CreateLiveActivityTypeDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLiveActivityTypeDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "content_state_schema", deserialize_with = "crate::de::null_default")]
    pub content_state_schema: CreateLiveActivityTypeDtoContentStateSchema,
    #[serde(rename = "attributes_schema", skip_serializing_if = "Option::is_none")]
    pub attributes_schema: Option<CreateLiveActivityTypeDtoAttributesSchema>,
}

/// `CreateLiveActivityTypeDtoAttributesSchema` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLiveActivityTypeDtoAttributesSchema {
    /// Zorunlu. Değerler: `"object"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, CreateLiveActivityTypeDtoAttributesSchemaPropertiesValue>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// `CreateLiveActivityTypeDtoAttributesSchemaPropertiesValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLiveActivityTypeDtoAttributesSchemaPropertiesValue {
    /// Zorunlu. Değerler: `"string"`, `"number"`, `"integer"`, `"boolean"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `CreateLiveActivityTypeDtoContentStateSchema` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLiveActivityTypeDtoContentStateSchema {
    /// Zorunlu. Değerler: `"object"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, CreateLiveActivityTypeDtoContentStateSchemaPropertiesValue>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// `CreateLiveActivityTypeDtoContentStateSchemaPropertiesValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLiveActivityTypeDtoContentStateSchemaPropertiesValue {
    /// Zorunlu. Değerler: `"string"`, `"number"`, `"integer"`, `"boolean"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `CreateLookalikeDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateLookalikeDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "country", deserialize_with = "crate::de::null_default")]
    pub country: String,
    /// Zorunlu.
    #[serde(rename = "ratio", deserialize_with = "crate::de::null_default")]
    pub ratio: f64,
}

/// `CreateMessengerLinkDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateMessengerLinkDto {
    /// Zorunlu. Değerler: `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "ttl_seconds", skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<i64>,
}

/// `CreateNotificationDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDto {
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "idempotency_key", skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// Zorunlu.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: CreateNotificationDtoTarget,
    #[serde(rename = "headings", skip_serializing_if = "Option::is_none")]
    pub headings: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "app_url", skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<CreateNotificationDtoButtonsItem>>,
    #[serde(rename = "big_picture", skip_serializing_if = "Option::is_none")]
    pub big_picture: Option<String>,
    #[serde(rename = "ios_attachments", skip_serializing_if = "Option::is_none")]
    pub ios_attachments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ios_sound", skip_serializing_if = "Option::is_none")]
    pub ios_sound: Option<String>,
    /// Değerler: `"None"`, `"SetTo"`, `"Increase"`.
    #[serde(rename = "ios_badge_type", skip_serializing_if = "Option::is_none")]
    pub ios_badge_type: Option<String>,
    #[serde(rename = "ios_badge_count", skip_serializing_if = "Option::is_none")]
    pub ios_badge_count: Option<i64>,
    /// Değerler: `"passive"`, `"active"`, `"time-sensitive"`, `"critical"`.
    #[serde(rename = "ios_interruption_level", skip_serializing_if = "Option::is_none")]
    pub ios_interruption_level: Option<String>,
    #[serde(rename = "ios_category", skip_serializing_if = "Option::is_none")]
    pub ios_category: Option<String>,
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(rename = "ios_relevance_score", skip_serializing_if = "Option::is_none")]
    pub ios_relevance_score: Option<f64>,
    #[serde(rename = "target_content_identifier", skip_serializing_if = "Option::is_none")]
    pub target_content_identifier: Option<String>,
    #[serde(rename = "android_channel_id", skip_serializing_if = "Option::is_none")]
    pub android_channel_id: Option<String>,
    #[serde(rename = "android_group", skip_serializing_if = "Option::is_none")]
    pub android_group: Option<String>,
    #[serde(rename = "android_sound", skip_serializing_if = "Option::is_none")]
    pub android_sound: Option<String>,
    #[serde(rename = "small_icon", skip_serializing_if = "Option::is_none")]
    pub small_icon: Option<String>,
    #[serde(rename = "large_icon", skip_serializing_if = "Option::is_none")]
    pub large_icon: Option<String>,
    #[serde(rename = "android_accent_color", skip_serializing_if = "Option::is_none")]
    pub android_accent_color: Option<String>,
    /// Değerler: `1`, `0`, `-1`.
    #[serde(rename = "android_visibility", skip_serializing_if = "Option::is_none")]
    pub android_visibility: Option<i64>,
    #[serde(rename = "ios_carousel", skip_serializing_if = "Option::is_none")]
    pub ios_carousel: Option<Vec<CreateNotificationDtoIosCarouselItem>>,
    #[serde(rename = "android_live_update", skip_serializing_if = "Option::is_none")]
    pub android_live_update: Option<CreateNotificationDtoAndroidLiveUpdate>,
    #[serde(rename = "voip", skip_serializing_if = "Option::is_none")]
    pub voip: Option<bool>,
    #[serde(rename = "app_clip", skip_serializing_if = "Option::is_none")]
    pub app_clip: Option<bool>,
    #[serde(rename = "chrome_web_icon", skip_serializing_if = "Option::is_none")]
    pub chrome_web_icon: Option<String>,
    #[serde(rename = "chrome_web_image", skip_serializing_if = "Option::is_none")]
    pub chrome_web_image: Option<String>,
    #[serde(rename = "chrome_web_badge", skip_serializing_if = "Option::is_none")]
    pub chrome_web_badge: Option<String>,
    #[serde(rename = "web_push_topic", skip_serializing_if = "Option::is_none")]
    pub web_push_topic: Option<String>,
    #[serde(rename = "web_buttons", skip_serializing_if = "Option::is_none")]
    pub web_buttons: Option<Vec<CreateNotificationDtoWebButtonsItem>>,
    #[serde(rename = "require_interaction", skip_serializing_if = "Option::is_none")]
    pub require_interaction: Option<bool>,
    #[serde(rename = "content_available", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<bool>,
    #[serde(rename = "mutable_content", skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<bool>,
    #[serde(rename = "collapse_id", skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    /// Değerler: `5`, `10`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(rename = "send_after", skip_serializing_if = "Option::is_none")]
    pub send_after: Option<String>,
    /// Değerler: `"timezone"`, `"optimized"`.
    #[serde(rename = "delayed_option", skip_serializing_if = "Option::is_none")]
    pub delayed_option: Option<String>,
    #[serde(rename = "delivery_time_of_day", skip_serializing_if = "Option::is_none")]
    pub delivery_time_of_day: Option<String>,
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "custom_data", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "email_subject", skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[serde(rename = "email_body", skip_serializing_if = "Option::is_none")]
    pub email_body: Option<String>,
    #[serde(rename = "email_mjml", skip_serializing_if = "Option::is_none")]
    pub email_mjml: Option<String>,
    #[serde(rename = "email_preheader", skip_serializing_if = "Option::is_none")]
    pub email_preheader: Option<String>,
    #[serde(rename = "email_from_name", skip_serializing_if = "Option::is_none")]
    pub email_from_name: Option<String>,
    #[serde(rename = "email_from_address", skip_serializing_if = "Option::is_none")]
    pub email_from_address: Option<String>,
    #[serde(rename = "email_reply_to_address", skip_serializing_if = "Option::is_none")]
    pub email_reply_to_address: Option<String>,
    #[serde(rename = "disable_email_click_tracking", skip_serializing_if = "Option::is_none")]
    pub disable_email_click_tracking: Option<bool>,
    #[serde(rename = "sms_from", skip_serializing_if = "Option::is_none")]
    pub sms_from: Option<String>,
    #[serde(rename = "sms_media_urls", skip_serializing_if = "Option::is_none")]
    pub sms_media_urls: Option<Vec<String>>,
    #[serde(rename = "disable_sms_link_shortening", skip_serializing_if = "Option::is_none")]
    pub disable_sms_link_shortening: Option<bool>,
    #[serde(rename = "whatsapp_template", skip_serializing_if = "Option::is_none")]
    pub whatsapp_template: Option<CreateNotificationDtoWhatsappTemplate>,
    #[serde(rename = "whatsapp_media", skip_serializing_if = "Option::is_none")]
    pub whatsapp_media: Option<CreateNotificationDtoWhatsappMedia>,
    #[serde(rename = "whatsapp_interactive", skip_serializing_if = "Option::is_none")]
    pub whatsapp_interactive: Option<CreateNotificationDtoWhatsappInteractive>,
    #[serde(rename = "rcs_card", skip_serializing_if = "Option::is_none")]
    pub rcs_card: Option<CreateNotificationDtoRcsCard>,
    #[serde(rename = "rcs_suggestions", skip_serializing_if = "Option::is_none")]
    pub rcs_suggestions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "rcs_content_sid", skip_serializing_if = "Option::is_none")]
    pub rcs_content_sid: Option<String>,
    #[serde(rename = "rcs_content_variables", skip_serializing_if = "Option::is_none")]
    pub rcs_content_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rcs_fallback_text", skip_serializing_if = "Option::is_none")]
    pub rcs_fallback_text: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "messenger_image_url", skip_serializing_if = "Option::is_none")]
    pub messenger_image_url: Option<String>,
    #[serde(rename = "messenger_buttons", skip_serializing_if = "Option::is_none")]
    pub messenger_buttons: Option<Vec<serde_json::Value>>,
    #[serde(rename = "add_to_inbox", skip_serializing_if = "Option::is_none")]
    pub add_to_inbox: Option<bool>,
    #[serde(rename = "inbox_ttl", skip_serializing_if = "Option::is_none")]
    pub inbox_ttl: Option<i64>,
    #[serde(rename = "content_card", skip_serializing_if = "Option::is_none")]
    pub content_card: Option<CreateNotificationDtoContentCard>,
    #[serde(rename = "frequency_cap_exempt", skip_serializing_if = "Option::is_none")]
    pub frequency_cap_exempt: Option<bool>,
    #[serde(rename = "quiet_hours_exempt", skip_serializing_if = "Option::is_none")]
    pub quiet_hours_exempt: Option<bool>,
    #[serde(rename = "one_per_user", skip_serializing_if = "Option::is_none")]
    pub one_per_user: Option<bool>,
    #[serde(rename = "throttle_rate_per_minute", skip_serializing_if = "Option::is_none")]
    pub throttle_rate_per_minute: Option<i64>,
    #[serde(rename = "include_unsubscribed", skip_serializing_if = "Option::is_none")]
    pub include_unsubscribed: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Değerler: `"sms"`, `"email"`.
    #[serde(rename = "fallback_channel", skip_serializing_if = "Option::is_none")]
    pub fallback_channel: Option<String>,
    /// Değerler: `"phone"`, `"watch"`.
    #[serde(rename = "form_factor", skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
}

/// `CreateNotificationDtoAndroidLiveUpdate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoAndroidLiveUpdate {
    /// Zorunlu.
    #[serde(rename = "progress", deserialize_with = "crate::de::null_default")]
    pub progress: i64,
    #[serde(rename = "ongoing", skip_serializing_if = "Option::is_none")]
    pub ongoing: Option<bool>,
}

/// `CreateNotificationDtoButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `CreateNotificationDtoContentCard` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoContentCard {
    /// Zorunlu. Değerler: `"classic"`, `"captioned_image"`, `"image_only"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "pinned", skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "button_text", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<std::collections::HashMap<String, String>>,
}

/// `CreateNotificationDtoIosCarouselItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoIosCarouselItem {
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `CreateNotificationDtoRcsCard` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoRcsCard {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "media_url", skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// Değerler: `"SHORT"`, `"MEDIUM"`, `"TALL"`.
    #[serde(rename = "media_height", skip_serializing_if = "Option::is_none")]
    pub media_height: Option<String>,
    /// Değerler: `"VERTICAL"`, `"HORIZONTAL"`.
    #[serde(rename = "orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
}

/// `CreateNotificationDtoTarget` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoTarget {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(rename = "include_subscription_ids", skip_serializing_if = "Option::is_none")]
    pub include_subscription_ids: Option<Vec<String>>,
    #[serde(rename = "include_external_ids", skip_serializing_if = "Option::is_none")]
    pub include_external_ids: Option<Vec<String>>,
    #[serde(rename = "include_aliases", skip_serializing_if = "Option::is_none")]
    pub include_aliases: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<serde_json::Value>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "test_devices_only", skip_serializing_if = "Option::is_none")]
    pub test_devices_only: Option<bool>,
    #[serde(rename = "email_to", skip_serializing_if = "Option::is_none")]
    pub email_to: Option<Vec<String>>,
    #[serde(rename = "include_phone_numbers", skip_serializing_if = "Option::is_none")]
    pub include_phone_numbers: Option<Vec<String>>,
}

/// `CreateNotificationDtoWebButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWebButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `CreateNotificationDtoWhatsappInteractive` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWhatsappInteractive {
    /// Zorunlu. Değerler: `"button"`, `"list"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<CreateNotificationDtoWhatsappInteractiveButtonsItem>>,
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<CreateNotificationDtoWhatsappInteractiveList>,
}

/// `CreateNotificationDtoWhatsappInteractiveButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWhatsappInteractiveButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
}

/// `CreateNotificationDtoWhatsappInteractiveList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWhatsappInteractiveList {
    /// Zorunlu.
    #[serde(rename = "button", deserialize_with = "crate::de::null_default")]
    pub button: String,
    /// Zorunlu.
    #[serde(rename = "sections", deserialize_with = "crate::de::null_default")]
    pub sections: Vec<CreateNotificationDtoWhatsappInteractiveListSectionsItem>,
}

/// `CreateNotificationDtoWhatsappInteractiveListSectionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWhatsappInteractiveListSectionsItem {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<CreateNotificationDtoWhatsappInteractiveListSectionsItemRowsItem>,
}

/// `CreateNotificationDtoWhatsappInteractiveListSectionsItemRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWhatsappInteractiveListSectionsItemRowsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `CreateNotificationDtoWhatsappMedia` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWhatsappMedia {
    /// Zorunlu. Değerler: `"image"`, `"video"`, `"document"`, `"audio"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// `CreateNotificationDtoWhatsappTemplate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWhatsappTemplate {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<serde_json::Value>,
    #[serde(rename = "body_params", skip_serializing_if = "Option::is_none")]
    pub body_params: Option<Vec<String>>,
    #[serde(rename = "button_params", skip_serializing_if = "Option::is_none")]
    pub button_params: Option<Vec<CreateNotificationDtoWhatsappTemplateButtonParamsItem>>,
}

/// `CreateNotificationDtoWhatsappTemplateButtonParamsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationDtoWhatsappTemplateButtonParamsItem {
    /// Zorunlu.
    #[serde(rename = "index", deserialize_with = "crate::de::null_default")]
    pub index: i64,
    /// Zorunlu. Değerler: `"url"`, `"quick_reply"`, `"copy_code"`.
    #[serde(rename = "sub_type", deserialize_with = "crate::de::null_default")]
    pub sub_type: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `CreateNotificationResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateNotificationResult {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"queued"`, `"processing"`, `"completed"`, `"cancelled"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "recipients_estimate", deserialize_with = "crate::de::null_default")]
    pub recipients_estimate: i64,
    /// Zorunlu.
    #[serde(rename = "external_id_missing", deserialize_with = "crate::de::null_default")]
    pub external_id_missing: Vec<String>,
}

/// `CreatePredictionDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreatePredictionDto {
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"churn"`, `"event"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    #[serde(rename = "event_names", skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
    #[serde(rename = "horizon_days", skip_serializing_if = "Option::is_none")]
    pub horizon_days: Option<i64>,
}

/// `CreatePromoCodeListDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreatePromoCodeListDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// `CreateSavedReportDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSavedReportDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "definition", deserialize_with = "crate::de::null_default")]
    pub definition: CreateSavedReportDtoDefinition,
}

/// `CreateSavedReportDtoDefinition` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSavedReportDtoDefinition {
    /// Zorunlu. Değerler: `"deliveries"`, `"events"`, `"conversions"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu. Değerler: `"sent"`, `"delivered"`, `"failed"`, `"opened"`, `"clicked"`, `"delivery_rate"`, `"open_rate"`, `"click_rate"`, `"failure_rate"`, `"count"`, `"unique_users"`, `"value_sum"`, `"value_avg"`, `"conversions"`, `"revenue"`.
    #[serde(rename = "metrics", deserialize_with = "crate::de::null_default")]
    pub metrics: Vec<String>,
    /// Değerler: `"day"`, `"week"`, `"month"`, `"channel"`, `"provider"`, `"campaign"`, `"status"`, `"name"`, `"source"`, `"event_name"`, `"kind"`.
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<CreateSavedReportDtoDefinitionSort>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// `CreateSavedReportDtoDefinitionSort` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSavedReportDtoDefinitionSort {
    /// Zorunlu.
    #[serde(rename = "by", deserialize_with = "crate::de::null_default")]
    pub by: String,
    /// Değerler: `"asc"`, `"desc"`.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// `CreateSuppressionDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSuppressionDto {
    /// Zorunlu. Değerler: `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
    /// Değerler: `"manual"`, `"bounce"`, `"complaint"`, `"unsubscribe_link"`, `"stop"`.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// `CreateSurveyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSurveyDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "questions", deserialize_with = "crate::de::null_default")]
    pub questions: Vec<CreateSurveyDtoQuestionsItem>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateSurveyDtoSettings>,
}

/// `CreateSurveyDtoQuestionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSurveyDtoQuestionsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"nps"`, `"csat"`, `"single_choice"`, `"multiple_choice"`, `"text"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: std::collections::HashMap<String, String>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CreateSurveyDtoQuestionsItemOptionsItem>>,
    #[serde(rename = "low_label", skip_serializing_if = "Option::is_none")]
    pub low_label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "high_label", skip_serializing_if = "Option::is_none")]
    pub high_label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<std::collections::HashMap<String, String>>,
}

/// `CreateSurveyDtoQuestionsItemOptionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSurveyDtoQuestionsItemOptionsItem {
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: std::collections::HashMap<String, String>,
}

/// `CreateSurveyDtoSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateSurveyDtoSettings {
    #[serde(rename = "thank_you", skip_serializing_if = "Option::is_none")]
    pub thank_you: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "submit_label", skip_serializing_if = "Option::is_none")]
    pub submit_label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "write_properties", skip_serializing_if = "Option::is_none")]
    pub write_properties: Option<bool>,
    #[serde(rename = "allow_multiple", skip_serializing_if = "Option::is_none")]
    pub allow_multiple: Option<bool>,
    #[serde(rename = "default_language", skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
}

/// `CreateTemplateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Zorunlu.
    #[serde(rename = "content", deserialize_with = "crate::de::null_default")]
    pub content: CreateTemplateDtoContent,
}

/// `CreateTemplateDtoContent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContent {
    #[serde(rename = "headings", skip_serializing_if = "Option::is_none")]
    pub headings: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "app_url", skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<CreateTemplateDtoContentButtonsItem>>,
    #[serde(rename = "big_picture", skip_serializing_if = "Option::is_none")]
    pub big_picture: Option<String>,
    #[serde(rename = "ios_attachments", skip_serializing_if = "Option::is_none")]
    pub ios_attachments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ios_sound", skip_serializing_if = "Option::is_none")]
    pub ios_sound: Option<String>,
    /// Değerler: `"None"`, `"SetTo"`, `"Increase"`.
    #[serde(rename = "ios_badge_type", skip_serializing_if = "Option::is_none")]
    pub ios_badge_type: Option<String>,
    #[serde(rename = "ios_badge_count", skip_serializing_if = "Option::is_none")]
    pub ios_badge_count: Option<i64>,
    /// Değerler: `"passive"`, `"active"`, `"time-sensitive"`, `"critical"`.
    #[serde(rename = "ios_interruption_level", skip_serializing_if = "Option::is_none")]
    pub ios_interruption_level: Option<String>,
    #[serde(rename = "ios_category", skip_serializing_if = "Option::is_none")]
    pub ios_category: Option<String>,
    #[serde(rename = "android_channel_id", skip_serializing_if = "Option::is_none")]
    pub android_channel_id: Option<String>,
    #[serde(rename = "android_group", skip_serializing_if = "Option::is_none")]
    pub android_group: Option<String>,
    #[serde(rename = "android_sound", skip_serializing_if = "Option::is_none")]
    pub android_sound: Option<String>,
    #[serde(rename = "chrome_web_icon", skip_serializing_if = "Option::is_none")]
    pub chrome_web_icon: Option<String>,
    #[serde(rename = "chrome_web_image", skip_serializing_if = "Option::is_none")]
    pub chrome_web_image: Option<String>,
    #[serde(rename = "chrome_web_badge", skip_serializing_if = "Option::is_none")]
    pub chrome_web_badge: Option<String>,
    #[serde(rename = "web_push_topic", skip_serializing_if = "Option::is_none")]
    pub web_push_topic: Option<String>,
    #[serde(rename = "content_available", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<bool>,
    #[serde(rename = "mutable_content", skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<bool>,
    #[serde(rename = "custom_data", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "email_subject", skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[serde(rename = "email_body", skip_serializing_if = "Option::is_none")]
    pub email_body: Option<String>,
    #[serde(rename = "email_mjml", skip_serializing_if = "Option::is_none")]
    pub email_mjml: Option<String>,
    #[serde(rename = "email_preheader", skip_serializing_if = "Option::is_none")]
    pub email_preheader: Option<String>,
    #[serde(rename = "email_from_name", skip_serializing_if = "Option::is_none")]
    pub email_from_name: Option<String>,
    #[serde(rename = "email_from_address", skip_serializing_if = "Option::is_none")]
    pub email_from_address: Option<String>,
    #[serde(rename = "email_reply_to_address", skip_serializing_if = "Option::is_none")]
    pub email_reply_to_address: Option<String>,
    #[serde(rename = "disable_email_click_tracking", skip_serializing_if = "Option::is_none")]
    pub disable_email_click_tracking: Option<bool>,
    #[serde(rename = "sms_from", skip_serializing_if = "Option::is_none")]
    pub sms_from: Option<String>,
    #[serde(rename = "disable_sms_link_shortening", skip_serializing_if = "Option::is_none")]
    pub disable_sms_link_shortening: Option<bool>,
    #[serde(rename = "add_to_inbox", skip_serializing_if = "Option::is_none")]
    pub add_to_inbox: Option<bool>,
    #[serde(rename = "inbox_ttl", skip_serializing_if = "Option::is_none")]
    pub inbox_ttl: Option<i64>,
    #[serde(rename = "content_card", skip_serializing_if = "Option::is_none")]
    pub content_card: Option<CreateTemplateDtoContentContentCard>,
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(rename = "ios_relevance_score", skip_serializing_if = "Option::is_none")]
    pub ios_relevance_score: Option<f64>,
    #[serde(rename = "target_content_identifier", skip_serializing_if = "Option::is_none")]
    pub target_content_identifier: Option<String>,
    #[serde(rename = "small_icon", skip_serializing_if = "Option::is_none")]
    pub small_icon: Option<String>,
    #[serde(rename = "large_icon", skip_serializing_if = "Option::is_none")]
    pub large_icon: Option<String>,
    #[serde(rename = "android_accent_color", skip_serializing_if = "Option::is_none")]
    pub android_accent_color: Option<String>,
    /// Değerler: `1`, `0`, `-1`.
    #[serde(rename = "android_visibility", skip_serializing_if = "Option::is_none")]
    pub android_visibility: Option<i64>,
    #[serde(rename = "web_buttons", skip_serializing_if = "Option::is_none")]
    pub web_buttons: Option<Vec<CreateTemplateDtoContentWebButtonsItem>>,
    #[serde(rename = "require_interaction", skip_serializing_if = "Option::is_none")]
    pub require_interaction: Option<bool>,
    #[serde(rename = "sms_media_urls", skip_serializing_if = "Option::is_none")]
    pub sms_media_urls: Option<Vec<String>>,
    #[serde(rename = "whatsapp_template", skip_serializing_if = "Option::is_none")]
    pub whatsapp_template: Option<CreateTemplateDtoContentWhatsappTemplate>,
    #[serde(rename = "whatsapp_media", skip_serializing_if = "Option::is_none")]
    pub whatsapp_media: Option<CreateTemplateDtoContentWhatsappMedia>,
    #[serde(rename = "whatsapp_interactive", skip_serializing_if = "Option::is_none")]
    pub whatsapp_interactive: Option<CreateTemplateDtoContentWhatsappInteractive>,
    #[serde(rename = "rcs_card", skip_serializing_if = "Option::is_none")]
    pub rcs_card: Option<CreateTemplateDtoContentRcsCard>,
    #[serde(rename = "rcs_suggestions", skip_serializing_if = "Option::is_none")]
    pub rcs_suggestions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "rcs_content_sid", skip_serializing_if = "Option::is_none")]
    pub rcs_content_sid: Option<String>,
    #[serde(rename = "rcs_content_variables", skip_serializing_if = "Option::is_none")]
    pub rcs_content_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rcs_fallback_text", skip_serializing_if = "Option::is_none")]
    pub rcs_fallback_text: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "messenger_image_url", skip_serializing_if = "Option::is_none")]
    pub messenger_image_url: Option<String>,
    #[serde(rename = "messenger_buttons", skip_serializing_if = "Option::is_none")]
    pub messenger_buttons: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ios_carousel", skip_serializing_if = "Option::is_none")]
    pub ios_carousel: Option<Vec<CreateTemplateDtoContentIosCarouselItem>>,
    #[serde(rename = "android_live_update", skip_serializing_if = "Option::is_none")]
    pub android_live_update: Option<CreateTemplateDtoContentAndroidLiveUpdate>,
    #[serde(rename = "voip", skip_serializing_if = "Option::is_none")]
    pub voip: Option<bool>,
    #[serde(rename = "app_clip", skip_serializing_if = "Option::is_none")]
    pub app_clip: Option<bool>,
}

/// `CreateTemplateDtoContentAndroidLiveUpdate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentAndroidLiveUpdate {
    /// Zorunlu.
    #[serde(rename = "progress", deserialize_with = "crate::de::null_default")]
    pub progress: i64,
    #[serde(rename = "ongoing", skip_serializing_if = "Option::is_none")]
    pub ongoing: Option<bool>,
}

/// `CreateTemplateDtoContentButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `CreateTemplateDtoContentContentCard` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentContentCard {
    /// Zorunlu. Değerler: `"classic"`, `"captioned_image"`, `"image_only"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "pinned", skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "button_text", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<std::collections::HashMap<String, String>>,
}

/// `CreateTemplateDtoContentIosCarouselItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentIosCarouselItem {
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `CreateTemplateDtoContentRcsCard` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentRcsCard {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "media_url", skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// Değerler: `"SHORT"`, `"MEDIUM"`, `"TALL"`.
    #[serde(rename = "media_height", skip_serializing_if = "Option::is_none")]
    pub media_height: Option<String>,
    /// Değerler: `"VERTICAL"`, `"HORIZONTAL"`.
    #[serde(rename = "orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
}

/// `CreateTemplateDtoContentWebButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWebButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `CreateTemplateDtoContentWhatsappInteractive` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWhatsappInteractive {
    /// Zorunlu. Değerler: `"button"`, `"list"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<CreateTemplateDtoContentWhatsappInteractiveButtonsItem>>,
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<CreateTemplateDtoContentWhatsappInteractiveList>,
}

/// `CreateTemplateDtoContentWhatsappInteractiveButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWhatsappInteractiveButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
}

/// `CreateTemplateDtoContentWhatsappInteractiveList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWhatsappInteractiveList {
    /// Zorunlu.
    #[serde(rename = "button", deserialize_with = "crate::de::null_default")]
    pub button: String,
    /// Zorunlu.
    #[serde(rename = "sections", deserialize_with = "crate::de::null_default")]
    pub sections: Vec<CreateTemplateDtoContentWhatsappInteractiveListSectionsItem>,
}

/// `CreateTemplateDtoContentWhatsappInteractiveListSectionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWhatsappInteractiveListSectionsItem {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<CreateTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem>,
}

/// `CreateTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `CreateTemplateDtoContentWhatsappMedia` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWhatsappMedia {
    /// Zorunlu. Değerler: `"image"`, `"video"`, `"document"`, `"audio"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// `CreateTemplateDtoContentWhatsappTemplate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWhatsappTemplate {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<serde_json::Value>,
    #[serde(rename = "body_params", skip_serializing_if = "Option::is_none")]
    pub body_params: Option<Vec<String>>,
    #[serde(rename = "button_params", skip_serializing_if = "Option::is_none")]
    pub button_params: Option<Vec<CreateTemplateDtoContentWhatsappTemplateButtonParamsItem>>,
}

/// `CreateTemplateDtoContentWhatsappTemplateButtonParamsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateTemplateDtoContentWhatsappTemplateButtonParamsItem {
    /// Zorunlu.
    #[serde(rename = "index", deserialize_with = "crate::de::null_default")]
    pub index: i64,
    /// Zorunlu. Değerler: `"url"`, `"quick_reply"`, `"copy_code"`.
    #[serde(rename = "sub_type", deserialize_with = "crate::de::null_default")]
    pub sub_type: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `CreateWalletTemplateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWalletTemplateDto {
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"coupon"`, `"loyalty"`, `"event"`, `"generic"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu.
    #[serde(rename = "design", deserialize_with = "crate::de::null_default")]
    pub design: CreateWalletTemplateDtoDesign,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<std::collections::HashMap<String, String>>,
}

/// `CreateWalletTemplateDtoDesign` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWalletTemplateDtoDesign {
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    /// Zorunlu.
    #[serde(rename = "description", deserialize_with = "crate::de::null_default")]
    pub description: String,
    /// Zorunlu.
    #[serde(rename = "organization", deserialize_with = "crate::de::null_default")]
    pub organization: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "foreground_color", skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,
    #[serde(rename = "label_color", skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    /// Değerler: `"qr"`, `"pdf417"`, `"aztec"`, `"code128"`.
    #[serde(rename = "barcode", skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<CreateWalletTemplateDtoDesignFieldsItem>>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "relevant_at", skip_serializing_if = "Option::is_none")]
    pub relevant_at: Option<String>,
}

/// `CreateWalletTemplateDtoDesignFieldsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWalletTemplateDtoDesignFieldsItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Değerler: `"header"`, `"primary"`, `"secondary"`, `"auxiliary"`, `"back"`.
    #[serde(rename = "section", skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}

/// `CreateWarmupPlanDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWarmupPlanDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"domain"`, `"ip"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: String,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "daily_limits", skip_serializing_if = "Option::is_none")]
    pub daily_limits: Option<Vec<i64>>,
    #[serde(rename = "curve", skip_serializing_if = "Option::is_none")]
    pub curve: Option<CreateWarmupPlanDtoCurve>,
    #[serde(rename = "provider_shares", skip_serializing_if = "Option::is_none")]
    pub provider_shares: Option<CreateWarmupPlanDtoProviderShares>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// `CreateWarmupPlanDtoCurve` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWarmupPlanDtoCurve {
    /// Zorunlu.
    #[serde(rename = "start", deserialize_with = "crate::de::null_default")]
    pub start: i64,
    /// Zorunlu.
    #[serde(rename = "growth", deserialize_with = "crate::de::null_default")]
    pub growth: f64,
    /// Zorunlu.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: i64,
}

/// `CreateWarmupPlanDtoProviderShares` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWarmupPlanDtoProviderShares {
    /// Zorunlu.
    #[serde(rename = "gmail", deserialize_with = "crate::de::null_default")]
    pub gmail: f64,
    /// Zorunlu.
    #[serde(rename = "outlook", deserialize_with = "crate::de::null_default")]
    pub outlook: f64,
    /// Zorunlu.
    #[serde(rename = "yahoo", deserialize_with = "crate::de::null_default")]
    pub yahoo: f64,
    /// Zorunlu.
    #[serde(rename = "other", deserialize_with = "crate::de::null_default")]
    pub other: f64,
}

/// `CreateWebCampaignDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: CreateWebCampaignDtoDisplay,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<CreateWebCampaignDtoVariantsItem>>,
    /// Zorunlu.
    #[serde(rename = "trigger")]
    pub trigger: Option<serde_json::Value>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<CreateWebCampaignDtoConditions>,
    #[serde(rename = "targeting", skip_serializing_if = "Option::is_none")]
    pub targeting: Option<CreateWebCampaignDtoTargeting>,
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<CreateWebCampaignDtoFrequency>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
}

/// `CreateWebCampaignDtoConditions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoConditions {
    #[serde(rename = "url_rules", skip_serializing_if = "Option::is_none")]
    pub url_rules: Option<Vec<CreateWebCampaignDtoConditionsUrlRulesItem>>,
    #[serde(rename = "utm_rules", skip_serializing_if = "Option::is_none")]
    pub utm_rules: Option<Vec<CreateWebCampaignDtoConditionsUtmRulesItem>>,
    #[serde(rename = "visits", skip_serializing_if = "Option::is_none")]
    pub visits: Option<CreateWebCampaignDtoConditionsVisits>,
}

/// `CreateWebCampaignDtoConditionsUrlRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoConditionsUrlRulesItem {
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"starts_with"`, `"not_contains"`, `"matches"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `CreateWebCampaignDtoConditionsUtmRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoConditionsUtmRulesItem {
    /// Zorunlu. Değerler: `"utm_source"`, `"utm_medium"`, `"utm_campaign"`, `"utm_term"`, `"utm_content"`.
    #[serde(rename = "param", deserialize_with = "crate::de::null_default")]
    pub param: String,
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// `CreateWebCampaignDtoConditionsVisits` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoConditionsVisits {
    /// Zorunlu. Değerler: `"gte"`, `"lte"`, `"eq"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: i64,
}

/// `CreateWebCampaignDtoDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoDisplay {
    /// Zorunlu. Değerler: `"popup"`, `"banner_top"`, `"banner_bottom"`, `"slide_in"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    /// Değerler: `"left"`, `"right"`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    #[serde(rename = "max_width", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "survey_id", skip_serializing_if = "Option::is_none")]
    pub survey_id: Option<String>,
}

/// `CreateWebCampaignDtoFrequency` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoFrequency {
    #[serde(rename = "max_per_session", skip_serializing_if = "Option::is_none")]
    pub max_per_session: Option<i64>,
    #[serde(rename = "max_per_day", skip_serializing_if = "Option::is_none")]
    pub max_per_day: Option<i64>,
    #[serde(rename = "max_total", skip_serializing_if = "Option::is_none")]
    pub max_total: Option<i64>,
    #[serde(rename = "hide_after_close_days", skip_serializing_if = "Option::is_none")]
    pub hide_after_close_days: Option<i64>,
    #[serde(rename = "stop_after_conversion", skip_serializing_if = "Option::is_none")]
    pub stop_after_conversion: Option<bool>,
}

/// `CreateWebCampaignDtoTargeting` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoTargeting {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    /// Değerler: `"all"`, `"new"`, `"returning"`.
    #[serde(rename = "visitor", skip_serializing_if = "Option::is_none")]
    pub visitor: Option<String>,
    /// Değerler: `"desktop"`, `"mobile"`, `"tablet"`.
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
}

/// `CreateWebCampaignDtoVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: CreateWebCampaignDtoVariantsItemDisplay,
}

/// `CreateWebCampaignDtoVariantsItemDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebCampaignDtoVariantsItemDisplay {
    /// Zorunlu. Değerler: `"popup"`, `"banner_top"`, `"banner_bottom"`, `"slide_in"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    /// Değerler: `"left"`, `"right"`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    #[serde(rename = "max_width", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "survey_id", skip_serializing_if = "Option::is_none")]
    pub survey_id: Option<String>,
}

/// `CreateWebhookDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebhookDto {
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Zorunlu. Değerler: `"message.queued"`, `"message.completed"`, `"message.failed"`, `"delivery.sent"`, `"delivery.delivered"`, `"delivery.failed"`, `"delivery.opened"`, `"delivery.clicked"`, `"delivery.bounced"`, `"delivery.complained"`, `"delivery.unsubscribed"`, `"in_app.impression"`, `"in_app.clicked"`, `"in_app.dismissed"`, `"subscription.created"`, `"subscription.unsubscribed"`, `"subscription.invalidated"`, `"user.created"`, `"user.updated"`, `"user.merged"`, `"user.deleted"`, `"event.received"`, `"conversion.created"`, `"campaign.scheduled"`, `"campaign.started"`, `"campaign.completed"`, `"journey.entered"`, `"journey.step_completed"`, `"journey.completed"`, `"journey.exited"`, `"conversation.created"`, `"conversation.message"`, `"conversation.assigned"`, `"conversation.resolved"`, `"audit.created"`, `"form.submitted"`, `"survey.responded"`, `"ad_audience.synced"`, `"ad_account.revoked"`, `"ad_budget.alert"`, `"ad_insights.imported"`.
    #[serde(rename = "events", deserialize_with = "crate::de::null_default")]
    pub events: Vec<String>,
    #[serde(rename = "event_filters", skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<CreateWebhookDtoEventFilters>,
    #[serde(rename = "batch", skip_serializing_if = "Option::is_none")]
    pub batch: Option<bool>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "body_template", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// Şablon çıktısının içerik türü (varsayılan application/json). Değerler: `"application/json"`, `"text/plain"`.
    #[serde(rename = "body_content_type", skip_serializing_if = "Option::is_none")]
    pub body_content_type: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<CreateWebhookDtoFilters>,
}

/// `CreateWebhookDtoEventFilters` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebhookDtoEventFilters {
    #[serde(rename = "event_names", skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
}

/// Olay filtresi: { match: all|any, conditions: [{ field, op, value? }] }.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebhookDtoFilters {
    /// Zorunlu. Değerler: `"all"`, `"any"`.
    #[serde(rename = "match", deserialize_with = "crate::de::null_default")]
    pub r#match: String,
    /// Zorunlu.
    #[serde(rename = "conditions", deserialize_with = "crate::de::null_default")]
    pub conditions: Vec<CreateWebhookDtoFiltersConditionsItem>,
}

/// `CreateWebhookDtoFiltersConditionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebhookDtoFiltersConditionsItem {
    /// Zorunlu.
    #[serde(rename = "field", deserialize_with = "crate::de::null_default")]
    pub field: String,
    /// Zorunlu. Değerler: `"eq"`, `"neq"`, `"in"`, `"not_in"`, `"starts_with"`, `"contains"`, `"exists"`, `"not_exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// `CreateWebPersonalizationDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebPersonalizationDto {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<serde_json::Value>>,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<CreateWebPersonalizationDtoVariantsItem>>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<CreateWebPersonalizationDtoConditions>,
    #[serde(rename = "targeting", skip_serializing_if = "Option::is_none")]
    pub targeting: Option<CreateWebPersonalizationDtoTargeting>,
    #[serde(rename = "goal", skip_serializing_if = "Option::is_none")]
    pub goal: Option<serde_json::Value>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
}

/// `CreateWebPersonalizationDtoConditions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebPersonalizationDtoConditions {
    #[serde(rename = "url_rules", skip_serializing_if = "Option::is_none")]
    pub url_rules: Option<Vec<CreateWebPersonalizationDtoConditionsUrlRulesItem>>,
    #[serde(rename = "utm_rules", skip_serializing_if = "Option::is_none")]
    pub utm_rules: Option<Vec<CreateWebPersonalizationDtoConditionsUtmRulesItem>>,
    #[serde(rename = "visits", skip_serializing_if = "Option::is_none")]
    pub visits: Option<CreateWebPersonalizationDtoConditionsVisits>,
}

/// `CreateWebPersonalizationDtoConditionsUrlRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebPersonalizationDtoConditionsUrlRulesItem {
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"starts_with"`, `"not_contains"`, `"matches"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `CreateWebPersonalizationDtoConditionsUtmRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebPersonalizationDtoConditionsUtmRulesItem {
    /// Zorunlu. Değerler: `"utm_source"`, `"utm_medium"`, `"utm_campaign"`, `"utm_term"`, `"utm_content"`.
    #[serde(rename = "param", deserialize_with = "crate::de::null_default")]
    pub param: String,
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// `CreateWebPersonalizationDtoConditionsVisits` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebPersonalizationDtoConditionsVisits {
    /// Zorunlu. Değerler: `"gte"`, `"lte"`, `"eq"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: i64,
}

/// `CreateWebPersonalizationDtoTargeting` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebPersonalizationDtoTargeting {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    /// Değerler: `"all"`, `"new"`, `"returning"`.
    #[serde(rename = "visitor", skip_serializing_if = "Option::is_none")]
    pub visitor: Option<String>,
    /// Değerler: `"desktop"`, `"mobile"`, `"tablet"`.
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
}

/// `CreateWebPersonalizationDtoVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreateWebPersonalizationDtoVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "changes", deserialize_with = "crate::de::null_default")]
    pub changes: Vec<serde_json::Value>,
}

/// `DataFeed` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataFeed {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "url_template", deserialize_with = "crate::de::null_default")]
    pub url_template: String,
    /// Zorunlu. Değerler: `"GET"`, `"POST"`.
    #[serde(rename = "method", deserialize_with = "crate::de::null_default")]
    pub method: String,
    /// Zorunlu.
    #[serde(rename = "body_template")]
    pub body_template: Option<String>,
    /// Zorunlu.
    #[serde(rename = "headers", deserialize_with = "crate::de::null_default")]
    pub headers: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "secret_header_names", deserialize_with = "crate::de::null_default")]
    pub secret_header_names: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "cache_ttl_seconds", deserialize_with = "crate::de::null_default")]
    pub cache_ttl_seconds: i64,
    /// Zorunlu.
    #[serde(rename = "timeout_ms", deserialize_with = "crate::de::null_default")]
    pub timeout_ms: i64,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "liquid", deserialize_with = "crate::de::null_default")]
    pub liquid: String,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `DataFeedList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataFeedList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<DataFeed>,
}

/// `DataFeedTestResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataFeedTestResult {
    /// Zorunlu. Değerler: `"ok"`, `"cached"`, `"timeout"`, `"error"`, `"circuit_open"`, `"blocked"`, `"invalid_json"`, `"too_large"`, `"http_error"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "http_status")]
    pub http_status: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "duration_ms", deserialize_with = "crate::de::null_default")]
    pub duration_ms: i64,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "preview")]
    pub preview: Option<serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "truncated", deserialize_with = "crate::de::null_default")]
    pub truncated: bool,
    /// Zorunlu.
    #[serde(rename = "user")]
    pub user: Option<DataFeedTestResultUser>,
}

/// `DataFeedTestResultUser` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataFeedTestResultUser {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
}

/// `DecideDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecideDto {
    /// Zorunlu.
    #[serde(rename = "external_id", deserialize_with = "crate::de::null_default")]
    pub external_id: String,
}

/// `DecisionAgent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionAgent {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "arms", deserialize_with = "crate::de::null_default")]
    pub arms: Vec<DecisionAgentArmsItem>,
    /// Zorunlu.
    #[serde(rename = "reward_events", deserialize_with = "crate::de::null_default")]
    pub reward_events: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "reward_window_hours", deserialize_with = "crate::de::null_default")]
    pub reward_window_hours: i64,
    /// Zorunlu.
    #[serde(rename = "holdout_percent", deserialize_with = "crate::de::null_default")]
    pub holdout_percent: i64,
    /// Zorunlu.
    #[serde(rename = "context_prediction")]
    pub context_prediction: Option<String>,
    /// Zorunlu.
    #[serde(rename = "use_best_channel", deserialize_with = "crate::de::null_default")]
    pub use_best_channel: bool,
    /// Zorunlu. Değerler: `"active"`, `"paused"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `DecisionAgentArmsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionAgentArmsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
}

/// `DecisionAgentList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionAgentList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<DecisionAgent>,
}

/// `DecisionReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionReport {
    /// Zorunlu.
    #[serde(rename = "agent_id", deserialize_with = "crate::de::null_default")]
    pub agent_id: String,
    /// Zorunlu.
    #[serde(rename = "days", deserialize_with = "crate::de::null_default")]
    pub days: i64,
    /// Zorunlu.
    #[serde(rename = "learned", deserialize_with = "crate::de::null_default")]
    pub learned: DecisionReportLearned,
    /// Zorunlu.
    #[serde(rename = "holdout", deserialize_with = "crate::de::null_default")]
    pub holdout: DecisionReportHoldout,
    /// Zorunlu.
    #[serde(rename = "lift")]
    pub lift: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "p_value")]
    pub p_value: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "significant", deserialize_with = "crate::de::null_default")]
    pub significant: bool,
    /// Zorunlu.
    #[serde(rename = "arms", deserialize_with = "crate::de::null_default")]
    pub arms: Vec<DecisionReportArmsItem>,
    /// Zorunlu.
    #[serde(rename = "contexts", deserialize_with = "crate::de::null_default")]
    pub contexts: Vec<DecisionReportContextsItem>,
}

/// `DecisionReportArmsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionReportArmsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
    /// Zorunlu.
    #[serde(rename = "share", deserialize_with = "crate::de::null_default")]
    pub share: f64,
    /// Zorunlu.
    #[serde(rename = "assignments", deserialize_with = "crate::de::null_default")]
    pub assignments: i64,
    /// Zorunlu.
    #[serde(rename = "settled", deserialize_with = "crate::de::null_default")]
    pub settled: i64,
    /// Zorunlu.
    #[serde(rename = "rewards", deserialize_with = "crate::de::null_default")]
    pub rewards: i64,
    /// Zorunlu.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
}

/// `DecisionReportContextsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionReportContextsItem {
    /// Zorunlu.
    #[serde(rename = "context", deserialize_with = "crate::de::null_default")]
    pub context: String,
    /// Zorunlu.
    #[serde(rename = "arm_id", deserialize_with = "crate::de::null_default")]
    pub arm_id: String,
    /// Zorunlu.
    #[serde(rename = "settled", deserialize_with = "crate::de::null_default")]
    pub settled: i64,
    /// Zorunlu.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
}

/// `DecisionReportHoldout` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionReportHoldout {
    /// Zorunlu.
    #[serde(rename = "assignments", deserialize_with = "crate::de::null_default")]
    pub assignments: i64,
    /// Zorunlu.
    #[serde(rename = "settled", deserialize_with = "crate::de::null_default")]
    pub settled: i64,
    /// Zorunlu.
    #[serde(rename = "rewards", deserialize_with = "crate::de::null_default")]
    pub rewards: i64,
    /// Zorunlu.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
}

/// `DecisionReportLearned` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionReportLearned {
    /// Zorunlu.
    #[serde(rename = "assignments", deserialize_with = "crate::de::null_default")]
    pub assignments: i64,
    /// Zorunlu.
    #[serde(rename = "settled", deserialize_with = "crate::de::null_default")]
    pub settled: i64,
    /// Zorunlu.
    #[serde(rename = "rewards", deserialize_with = "crate::de::null_default")]
    pub rewards: i64,
    /// Zorunlu.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
}

/// `DecisionResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DecisionResult {
    /// Zorunlu.
    #[serde(rename = "arm_id", deserialize_with = "crate::de::null_default")]
    pub arm_id: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
    /// Zorunlu. Değerler: `"thompson"`, `"holdout"`.
    #[serde(rename = "policy", deserialize_with = "crate::de::null_default")]
    pub policy: String,
    /// Zorunlu.
    #[serde(rename = "context", deserialize_with = "crate::de::null_default")]
    pub context: String,
}

/// `DeliverabilitySettingsDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeliverabilitySettingsDto {
    #[serde(rename = "bcc_address", skip_serializing_if = "Option::is_none")]
    pub bcc_address: Option<String>,
    #[serde(rename = "validation", skip_serializing_if = "Option::is_none")]
    pub validation: Option<DeliverabilitySettingsDtoValidation>,
    #[serde(rename = "verifier", skip_serializing_if = "Option::is_none")]
    pub verifier: Option<DeliverabilitySettingsDtoVerifier>,
    #[serde(rename = "volume_spike", skip_serializing_if = "Option::is_none")]
    pub volume_spike: Option<DeliverabilitySettingsDtoVolumeSpike>,
    #[serde(rename = "max_invalid_ratio", skip_serializing_if = "Option::is_none")]
    pub max_invalid_ratio: Option<f64>,
    #[serde(rename = "postmaster", skip_serializing_if = "Option::is_none")]
    pub postmaster: Option<DeliverabilitySettingsDtoPostmaster>,
}

/// `DeliverabilitySettingsDtoPostmaster` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeliverabilitySettingsDtoPostmaster {
    /// Zorunlu.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

/// `DeliverabilitySettingsDtoValidation` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeliverabilitySettingsDtoValidation {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "check_mx", skip_serializing_if = "Option::is_none")]
    pub check_mx: Option<bool>,
    #[serde(rename = "block_disposable", skip_serializing_if = "Option::is_none")]
    pub block_disposable: Option<bool>,
    #[serde(rename = "block_role", skip_serializing_if = "Option::is_none")]
    pub block_role: Option<bool>,
}

/// `DeliverabilitySettingsDtoVerifier` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeliverabilitySettingsDtoVerifier {
    /// Zorunlu. Değerler: `"zerobounce"`, `"neverbounce"`, `"mock"`.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    #[serde(rename = "api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

/// `DeliverabilitySettingsDtoVolumeSpike` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeliverabilitySettingsDtoVolumeSpike {
    /// Zorunlu.
    #[serde(rename = "multiplier")]
    pub multiplier: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "floor")]
    pub floor: Option<i64>,
}

/// `Delivery` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Delivery {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "message_id", deserialize_with = "crate::de::null_default")]
    pub message_id: String,
    /// Zorunlu.
    #[serde(rename = "subscription_id", deserialize_with = "crate::de::null_default")]
    pub subscription_id: String,
    /// Zorunlu.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Zorunlu. Değerler: `"pending"`, `"sent"`, `"delivered"`, `"failed"`, `"bounced"`, `"suppressed"`, `"expired"`, `"cancelled"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "attempt_count", deserialize_with = "crate::de::null_default")]
    pub attempt_count: i64,
    /// Zorunlu.
    #[serde(rename = "provider_message_id")]
    pub provider_message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "error_code")]
    pub error_code: Option<String>,
    /// Zorunlu.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// Zorunlu.
    #[serde(rename = "suppression_reason")]
    pub suppression_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "send_after")]
    pub send_after: Option<String>,
    /// Zorunlu.
    #[serde(rename = "sent_at")]
    pub sent_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "delivered_at")]
    pub delivered_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "opened_at")]
    pub opened_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "clicked_at")]
    pub clicked_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "failed_at")]
    pub failed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `DeliveryList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeliveryList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Delivery>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `EmailAddressCheck` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailAddressCheck {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
    /// Zorunlu. Değerler: `"syntax"`, `"no_mx"`, `"disposable"`, `"role"`, `"verifier_invalid"`.
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// Zorunlu.
    #[serde(rename = "checks", deserialize_with = "crate::de::null_default")]
    pub checks: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "unknown", deserialize_with = "crate::de::null_default")]
    pub unknown: Vec<String>,
}

/// `EmailDeliverabilitySettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailDeliverabilitySettings {
    /// Zorunlu.
    #[serde(rename = "bcc_address")]
    pub bcc_address: Option<String>,
    /// Zorunlu.
    #[serde(rename = "validation", deserialize_with = "crate::de::null_default")]
    pub validation: EmailDeliverabilitySettingsValidation,
    /// Zorunlu.
    #[serde(rename = "verifier", deserialize_with = "crate::de::null_default")]
    pub verifier: EmailDeliverabilitySettingsVerifier,
    /// Zorunlu.
    #[serde(rename = "volume_spike", deserialize_with = "crate::de::null_default")]
    pub volume_spike: EmailDeliverabilitySettingsVolumeSpike,
    /// Zorunlu.
    #[serde(rename = "postmaster", deserialize_with = "crate::de::null_default")]
    pub postmaster: EmailDeliverabilitySettingsPostmaster,
    /// Zorunlu.
    #[serde(rename = "max_invalid_ratio", deserialize_with = "crate::de::null_default")]
    pub max_invalid_ratio: f64,
}

/// `EmailDeliverabilitySettingsPostmaster` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailDeliverabilitySettingsPostmaster {
    /// Zorunlu.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_token", deserialize_with = "crate::de::null_default")]
    pub has_token: bool,
}

/// `EmailDeliverabilitySettingsValidation` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailDeliverabilitySettingsValidation {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "check_mx", deserialize_with = "crate::de::null_default")]
    pub check_mx: bool,
    /// Zorunlu.
    #[serde(rename = "block_disposable", deserialize_with = "crate::de::null_default")]
    pub block_disposable: bool,
    /// Zorunlu.
    #[serde(rename = "block_role", deserialize_with = "crate::de::null_default")]
    pub block_role: bool,
}

/// `EmailDeliverabilitySettingsVerifier` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailDeliverabilitySettingsVerifier {
    /// Zorunlu. Değerler: `"zerobounce"`, `"neverbounce"`, `"mock"`.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_key", deserialize_with = "crate::de::null_default")]
    pub has_key: bool,
}

/// `EmailDeliverabilitySettingsVolumeSpike` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailDeliverabilitySettingsVolumeSpike {
    /// Zorunlu.
    #[serde(rename = "multiplier")]
    pub multiplier: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "floor")]
    pub floor: Option<i64>,
}

/// `EmailWarmupPlan` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailWarmupPlan {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"domain"`, `"ip"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: String,
    /// Zorunlu.
    #[serde(rename = "start_date", deserialize_with = "crate::de::null_default")]
    pub start_date: String,
    /// Zorunlu.
    #[serde(rename = "daily_limits", deserialize_with = "crate::de::null_default")]
    pub daily_limits: Vec<i64>,
    /// Zorunlu.
    #[serde(rename = "provider_shares", deserialize_with = "crate::de::null_default")]
    pub provider_shares: std::collections::HashMap<String, f64>,
    /// Zorunlu.
    #[serde(rename = "timezone", deserialize_with = "crate::de::null_default")]
    pub timezone: String,
    /// Zorunlu. Değerler: `"active"`, `"paused"`, `"completed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "today", deserialize_with = "crate::de::null_default")]
    pub today: EmailWarmupPlanToday,
    /// Zorunlu.
    #[serde(rename = "scheduled_ahead", deserialize_with = "crate::de::null_default")]
    pub scheduled_ahead: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `EmailWarmupPlanList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailWarmupPlanList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<EmailWarmupPlan>,
}

/// `EmailWarmupPlanToday` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailWarmupPlanToday {
    /// Zorunlu.
    #[serde(rename = "day", deserialize_with = "crate::de::null_default")]
    pub day: String,
    /// Zorunlu.
    #[serde(rename = "index", deserialize_with = "crate::de::null_default")]
    pub index: i64,
    /// Zorunlu.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "used", deserialize_with = "crate::de::null_default")]
    pub used: i64,
    /// Zorunlu.
    #[serde(rename = "buckets", deserialize_with = "crate::de::null_default")]
    pub buckets: std::collections::HashMap<String, EmailWarmupPlanTodayBucketsValue>,
}

/// `EmailWarmupPlanTodayBucketsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailWarmupPlanTodayBucketsValue {
    /// Zorunlu.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "used", deserialize_with = "crate::de::null_default")]
    pub used: i64,
}

/// `EmailWarmupUsage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailWarmupUsage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<EmailWarmupUsageDataItem>,
}

/// `EmailWarmupUsageDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EmailWarmupUsageDataItem {
    /// Zorunlu.
    #[serde(rename = "day", deserialize_with = "crate::de::null_default")]
    pub day: String,
    /// Zorunlu.
    #[serde(rename = "bucket", deserialize_with = "crate::de::null_default")]
    pub bucket: String,
    /// Zorunlu.
    #[serde(rename = "used", deserialize_with = "crate::de::null_default")]
    pub used: i64,
    /// Zorunlu.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
}

/// `EndLiveActivityDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EndLiveActivityDto {
    /// Zorunlu.
    #[serde(rename = "activity_id", deserialize_with = "crate::de::null_default")]
    pub activity_id: String,
    #[serde(rename = "content_state", skip_serializing_if = "Option::is_none")]
    pub content_state: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<EndLiveActivityDtoAlert>,
    #[serde(rename = "stale_date", skip_serializing_if = "Option::is_none")]
    pub stale_date: Option<i64>,
    #[serde(rename = "dismissal_date", skip_serializing_if = "Option::is_none")]
    pub dismissal_date: Option<i64>,
    #[serde(rename = "relevance_score", skip_serializing_if = "Option::is_none")]
    pub relevance_score: Option<f64>,
    /// Değerler: `5`, `10`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `EndLiveActivityDtoAlert` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EndLiveActivityDtoAlert {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "sound", skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
}

/// `EngagementReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EngagementReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "timezone", deserialize_with = "crate::de::null_default")]
    pub timezone: String,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: EngagementReportTotals,
    /// Zorunlu.
    #[serde(rename = "previous")]
    pub previous: Option<EngagementReportPrevious>,
    /// Zorunlu.
    #[serde(rename = "cells", deserialize_with = "crate::de::null_default")]
    pub cells: Vec<EngagementReportCellsItem>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<EngagementReportRowsItem>,
}

/// `EngagementReportCellsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EngagementReportCellsItem {
    /// Zorunlu.
    #[serde(rename = "weekday", deserialize_with = "crate::de::null_default")]
    pub weekday: i64,
    /// Zorunlu.
    #[serde(rename = "hour", deserialize_with = "crate::de::null_default")]
    pub hour: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
}

/// `EngagementReportPrevious` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EngagementReportPrevious {
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
}

/// `EngagementReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EngagementReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
}

/// `EngagementReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EngagementReportTotals {
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
}

/// `EstimateAudienceDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EstimateAudienceDto {
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: EstimateAudienceDtoTarget,
    #[serde(rename = "include_unsubscribed", skip_serializing_if = "Option::is_none")]
    pub include_unsubscribed: Option<bool>,
}

/// `EstimateAudienceDtoTarget` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EstimateAudienceDtoTarget {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(rename = "include_subscription_ids", skip_serializing_if = "Option::is_none")]
    pub include_subscription_ids: Option<Vec<String>>,
    #[serde(rename = "include_external_ids", skip_serializing_if = "Option::is_none")]
    pub include_external_ids: Option<Vec<String>>,
    #[serde(rename = "include_aliases", skip_serializing_if = "Option::is_none")]
    pub include_aliases: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<serde_json::Value>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "test_devices_only", skip_serializing_if = "Option::is_none")]
    pub test_devices_only: Option<bool>,
    #[serde(rename = "email_to", skip_serializing_if = "Option::is_none")]
    pub email_to: Option<Vec<String>>,
    #[serde(rename = "include_phone_numbers", skip_serializing_if = "Option::is_none")]
    pub include_phone_numbers: Option<Vec<String>>,
}

/// `EstimateAudienceResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EstimateAudienceResult {
    /// Zorunlu.
    #[serde(rename = "recipients", deserialize_with = "crate::de::null_default")]
    pub recipients: i64,
    /// Zorunlu.
    #[serde(rename = "by_channel", deserialize_with = "crate::de::null_default")]
    pub by_channel: std::collections::HashMap<String, i64>,
    /// Zorunlu.
    #[serde(rename = "new_contacts", deserialize_with = "crate::de::null_default")]
    pub new_contacts: i64,
    /// Zorunlu.
    #[serde(rename = "external_id_missing", deserialize_with = "crate::de::null_default")]
    pub external_id_missing: Vec<String>,
}

/// `EvaluateAppFlagsDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EvaluateAppFlagsDto {
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// `EventDefinition` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventDefinition {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "properties_schema", deserialize_with = "crate::de::null_default")]
    pub properties_schema: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "is_conversion", deserialize_with = "crate::de::null_default")]
    pub is_conversion: bool,
    /// Zorunlu.
    #[serde(rename = "outcome_unique", deserialize_with = "crate::de::null_default")]
    pub outcome_unique: bool,
    /// Zorunlu.
    #[serde(rename = "outcome_unattributed", deserialize_with = "crate::de::null_default")]
    pub outcome_unattributed: bool,
    /// Zorunlu.
    #[serde(rename = "retention_days")]
    pub retention_days: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "first_seen_at", deserialize_with = "crate::de::null_default")]
    pub first_seen_at: String,
    /// Zorunlu.
    #[serde(rename = "last_seen_at", deserialize_with = "crate::de::null_default")]
    pub last_seen_at: String,
    /// Zorunlu.
    #[serde(rename = "count_30d", deserialize_with = "crate::de::null_default")]
    pub count_30d: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `EventDefinitionDetail` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventDefinitionDetail {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "properties_schema", deserialize_with = "crate::de::null_default")]
    pub properties_schema: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "is_conversion", deserialize_with = "crate::de::null_default")]
    pub is_conversion: bool,
    /// Zorunlu.
    #[serde(rename = "outcome_unique", deserialize_with = "crate::de::null_default")]
    pub outcome_unique: bool,
    /// Zorunlu.
    #[serde(rename = "outcome_unattributed", deserialize_with = "crate::de::null_default")]
    pub outcome_unattributed: bool,
    /// Zorunlu.
    #[serde(rename = "retention_days")]
    pub retention_days: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "first_seen_at", deserialize_with = "crate::de::null_default")]
    pub first_seen_at: String,
    /// Zorunlu.
    #[serde(rename = "last_seen_at", deserialize_with = "crate::de::null_default")]
    pub last_seen_at: String,
    /// Zorunlu.
    #[serde(rename = "count_30d", deserialize_with = "crate::de::null_default")]
    pub count_30d: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu.
    #[serde(rename = "daily", deserialize_with = "crate::de::null_default")]
    pub daily: Vec<EventDefinitionDetailDailyItem>,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: Vec<EventDefinitionDetailPropertiesItem>,
}

/// `EventDefinitionDetailDailyItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventDefinitionDetailDailyItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
}

/// `EventDefinitionDetailPropertiesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventDefinitionDetailPropertiesItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "values", deserialize_with = "crate::de::null_default")]
    pub values: Vec<EventDefinitionDetailPropertiesItemValuesItem>,
}

/// `EventDefinitionDetailPropertiesItemValuesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventDefinitionDetailPropertiesItemValuesItem {
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
}

/// `EventDefinitionList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventDefinitionList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<EventDefinition>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `EventsAccepted` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventsAccepted {
    /// Zorunlu.
    #[serde(rename = "accepted", deserialize_with = "crate::de::null_default")]
    pub accepted: i64,
}

/// `EventsReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventsReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu. Değerler: `"day"`, `"name"`.
    #[serde(rename = "group_by", deserialize_with = "crate::de::null_default")]
    pub group_by: String,
    /// Zorunlu.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: EventsReportTotals,
    /// Zorunlu.
    #[serde(rename = "previous")]
    pub previous: Option<EventsReportPrevious>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<EventsReportRowsItem>,
}

/// `EventsReportPrevious` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventsReportPrevious {
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
}

/// `EventsReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventsReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
}

/// `EventsReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventsReportTotals {
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
}

/// `Export` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Export {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu. Değerler: `"users"`, `"subscriptions"`, `"events"`, `"deliveries"`, `"message_events"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "filters", deserialize_with = "crate::de::null_default")]
    pub filters: ExportFilters,
    /// Zorunlu. Değerler: `"pending"`, `"running"`, `"completed"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "row_count")]
    pub row_count: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "download_url")]
    pub download_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "requested_by")]
    pub requested_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<String>,
}

/// `ExportFilters` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ExportFilters {
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "campaign_id", skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "segment_id", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
}

/// `ExportList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ExportList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Export>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `Form` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Form {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"active"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "fields", deserialize_with = "crate::de::null_default")]
    pub fields: Vec<FormFieldsItem>,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: FormSettings,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: FormStats,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `FormFieldsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormFieldsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"email"`, `"phone"`, `"first_name"`, `"last_name"`, `"full_name"`, `"text"`, `"textarea"`, `"select"`, `"radio"`, `"checkbox"`, `"consent"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: std::collections::HashMap<String, String>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<FormFieldsItemOptionsItem>>,
    #[serde(rename = "maps_to", skip_serializing_if = "Option::is_none")]
    pub maps_to: Option<String>,
    #[serde(rename = "max_length", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "policy_url", skip_serializing_if = "Option::is_none")]
    pub policy_url: Option<String>,
}

/// `FormFieldsItemOptionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormFieldsItemOptionsItem {
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: std::collections::HashMap<String, String>,
}

/// `FormPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Form>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `FormSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormSettings {
    #[serde(rename = "submit_label", skip_serializing_if = "Option::is_none")]
    pub submit_label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "success_message", skip_serializing_if = "Option::is_none")]
    pub success_message: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<FormSettingsSubscribe>,
    #[serde(rename = "double_opt_in", skip_serializing_if = "Option::is_none")]
    pub double_opt_in: Option<FormSettingsDoubleOptIn>,
    #[serde(rename = "event_name", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "captcha", skip_serializing_if = "Option::is_none")]
    pub captcha: Option<bool>,
    #[serde(rename = "default_language", skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
}

/// `FormSettingsDoubleOptIn` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormSettingsDoubleOptIn {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<std::collections::HashMap<String, String>>,
}

/// `FormSettingsSubscribe` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormSettingsSubscribe {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
}

/// `FormStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormStats {
    /// Zorunlu.
    #[serde(rename = "submissions", deserialize_with = "crate::de::null_default")]
    pub submissions: i64,
    /// Zorunlu.
    #[serde(rename = "last_submission_at")]
    pub last_submission_at: Option<String>,
}

/// `FormSubmission` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormSubmission {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "form_id", deserialize_with = "crate::de::null_default")]
    pub form_id: String,
    /// Zorunlu.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "subscription_ids", deserialize_with = "crate::de::null_default")]
    pub subscription_ids: Vec<String>,
    /// Zorunlu. Değerler: `"web_campaign"`, `"landing_page"`, `"sdk"`, `"api"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu.
    #[serde(rename = "source_id")]
    pub source_id: Option<String>,
    /// Zorunlu. Değerler: `"accepted"`, `"pending_confirmation"`, `"confirmed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "consent", deserialize_with = "crate::de::null_default")]
    pub consent: Vec<FormSubmissionConsentItem>,
    /// Zorunlu.
    #[serde(rename = "page_url")]
    pub page_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Zorunlu.
    #[serde(rename = "confirmed_at")]
    pub confirmed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `FormSubmissionConsentItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormSubmissionConsentItem {
    /// Zorunlu.
    #[serde(rename = "field_id", deserialize_with = "crate::de::null_default")]
    pub field_id: String,
    /// Zorunlu.
    #[serde(rename = "given", deserialize_with = "crate::de::null_default")]
    pub given: bool,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    /// Zorunlu.
    #[serde(rename = "text_hash", deserialize_with = "crate::de::null_default")]
    pub text_hash: String,
    /// Zorunlu.
    #[serde(rename = "policy_url")]
    pub policy_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    /// Zorunlu.
    #[serde(rename = "at", deserialize_with = "crate::de::null_default")]
    pub at: String,
}

/// `FormSubmissionPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormSubmissionPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<FormSubmission>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `FormSubmitResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FormSubmitResult {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
    /// Zorunlu. Değerler: `"accepted"`, `"pending_confirmation"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "submission_id")]
    pub submission_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "success_message", deserialize_with = "crate::de::null_default")]
    pub success_message: String,
    /// Zorunlu.
    #[serde(rename = "redirect_url")]
    pub redirect_url: Option<String>,
}

/// `FunnelBodyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunnelBodyDto {
    /// Zorunlu.
    #[serde(rename = "steps", deserialize_with = "crate::de::null_default")]
    pub steps: Vec<FunnelBodyDtoStepsItem>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "window_hours", skip_serializing_if = "Option::is_none")]
    pub window_hours: Option<i64>,
    #[serde(rename = "compare", skip_serializing_if = "Option::is_none")]
    pub compare: Option<bool>,
    /// Değerler: `"json"`, `"csv"`.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// `FunnelBodyDtoStepsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunnelBodyDtoStepsItem {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub r#where: Option<FunnelBodyDtoStepsItemWhere>,
}

/// `FunnelBodyDtoStepsItemWhere` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunnelBodyDtoStepsItemWhere {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `FunnelReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunnelReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "window_hours", deserialize_with = "crate::de::null_default")]
    pub window_hours: i64,
    /// Zorunlu.
    #[serde(rename = "steps", deserialize_with = "crate::de::null_default")]
    pub steps: Vec<FunnelReportStepsItem>,
    /// Zorunlu.
    #[serde(rename = "overall_rate")]
    pub overall_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "previous")]
    pub previous: Option<FunnelReportPrevious>,
}

/// `FunnelReportPrevious` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunnelReportPrevious {
    /// Zorunlu.
    #[serde(rename = "steps", deserialize_with = "crate::de::null_default")]
    pub steps: Vec<i64>,
    /// Zorunlu.
    #[serde(rename = "overall_rate")]
    pub overall_rate: Option<f64>,
}

/// `FunnelReportStepsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FunnelReportStepsItem {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate")]
    pub conversion_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "drop_off", deserialize_with = "crate::de::null_default")]
    pub drop_off: i64,
}

/// `Geofence` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Geofence {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "lat", deserialize_with = "crate::de::null_default")]
    pub lat: f64,
    /// Zorunlu.
    #[serde(rename = "lng", deserialize_with = "crate::de::null_default")]
    pub lng: f64,
    /// Zorunlu.
    #[serde(rename = "radius_m", deserialize_with = "crate::de::null_default")]
    pub radius_m: i64,
    /// Zorunlu.
    #[serde(rename = "cooldown_minutes", deserialize_with = "crate::de::null_default")]
    pub cooldown_minutes: i64,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "enter_count", deserialize_with = "crate::de::null_default")]
    pub enter_count: i64,
    /// Zorunlu.
    #[serde(rename = "exit_count", deserialize_with = "crate::de::null_default")]
    pub exit_count: i64,
    /// Zorunlu.
    #[serde(rename = "inside_count", deserialize_with = "crate::de::null_default")]
    pub inside_count: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `GeofenceList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GeofenceList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Geofence>,
}

/// `GlobalControl` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GlobalControl {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "percent", deserialize_with = "crate::de::null_default")]
    pub percent: i64,
    /// Zorunlu.
    #[serde(rename = "started_at")]
    pub started_at: Option<String>,
}

/// `GlobalControlReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GlobalControlReport {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "percent", deserialize_with = "crate::de::null_default")]
    pub percent: i64,
    /// Zorunlu.
    #[serde(rename = "started_at")]
    pub started_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "events", deserialize_with = "crate::de::null_default")]
    pub events: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "control", deserialize_with = "crate::de::null_default")]
    pub control: GlobalControlReportControl,
    /// Zorunlu.
    #[serde(rename = "treatment", deserialize_with = "crate::de::null_default")]
    pub treatment: GlobalControlReportTreatment,
    /// Zorunlu.
    #[serde(rename = "lift_percent")]
    pub lift_percent: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "incremental_conversions", deserialize_with = "crate::de::null_default")]
    pub incremental_conversions: i64,
    /// Zorunlu.
    #[serde(rename = "p_value")]
    pub p_value: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "significant", deserialize_with = "crate::de::null_default")]
    pub significant: bool,
    /// Zorunlu.
    #[serde(rename = "withheld_messages", deserialize_with = "crate::de::null_default")]
    pub withheld_messages: i64,
}

/// `GlobalControlReportControl` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GlobalControlReportControl {
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
}

/// `GlobalControlReportTreatment` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GlobalControlReportTreatment {
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
}

/// `IdentityKey` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IdentityKey {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "kid", deserialize_with = "crate::de::null_default")]
    pub kid: String,
    /// Zorunlu. Değerler: `"ES256"`.
    #[serde(rename = "alg", deserialize_with = "crate::de::null_default")]
    pub alg: String,
    /// Zorunlu.
    #[serde(rename = "public_jwk", deserialize_with = "crate::de::null_default")]
    pub public_jwk: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "revoked_at")]
    pub revoked_at: Option<String>,
}

/// `IdentityKeyList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IdentityKeyList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<IdentityKey>,
}

/// `Import` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Import {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu. Değerler: `"legacy_players"`, `"csv_users"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    /// Zorunlu. Değerler: `"pending"`, `"running"`, `"completed"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: ImportStats,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "error_report_url")]
    pub error_report_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "requested_by")]
    pub requested_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<String>,
}

/// `ImportList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ImportList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Import>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `ImportStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ImportStats {
    #[serde(rename = "processed", skip_serializing_if = "Option::is_none")]
    pub processed: Option<i64>,
    #[serde(rename = "created_users", skip_serializing_if = "Option::is_none")]
    pub created_users: Option<i64>,
    #[serde(rename = "updated_users", skip_serializing_if = "Option::is_none")]
    pub updated_users: Option<i64>,
    #[serde(rename = "created_subscriptions", skip_serializing_if = "Option::is_none")]
    pub created_subscriptions: Option<i64>,
    #[serde(rename = "skipped", skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<i64>,
    #[serde(rename = "rejected_emails", skip_serializing_if = "Option::is_none")]
    pub rejected_emails: Option<i64>,
    #[serde(rename = "rejected_email_reasons", skip_serializing_if = "Option::is_none")]
    pub rejected_email_reasons: Option<std::collections::HashMap<String, i64>>,
}

/// `InAppMessage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessage {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "priority", deserialize_with = "crate::de::null_default")]
    pub priority: i64,
    /// Zorunlu.
    #[serde(rename = "triggers", deserialize_with = "crate::de::null_default")]
    pub triggers: Vec<serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "trigger_groups")]
    pub trigger_groups: Option<Vec<Vec<serde_json::Value>>>,
    /// Zorunlu.
    #[serde(rename = "variants", deserialize_with = "crate::de::null_default")]
    pub variants: Vec<InAppMessageVariantsItem>,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: InAppMessageDisplay,
    /// Zorunlu.
    #[serde(rename = "audience", deserialize_with = "crate::de::null_default")]
    pub audience: InAppMessageAudience,
    /// Zorunlu.
    #[serde(rename = "frequency", deserialize_with = "crate::de::null_default")]
    pub frequency: InAppMessageFrequency,
    /// Zorunlu.
    #[serde(rename = "journey_only", deserialize_with = "crate::de::null_default")]
    pub journey_only: bool,
    /// Zorunlu.
    #[serde(rename = "placement")]
    pub placement: Option<String>,
    /// Zorunlu.
    #[serde(rename = "start_at")]
    pub start_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: InAppMessageStats,
    /// Zorunlu.
    #[serde(rename = "variant_stats", deserialize_with = "crate::de::null_default")]
    pub variant_stats: Vec<InAppMessageVariantStatsItem>,
    /// Zorunlu.
    #[serde(rename = "ab_test", deserialize_with = "crate::de::null_default")]
    pub ab_test: InAppMessageAbTest,
    /// Zorunlu.
    #[serde(rename = "variant_significance")]
    pub variant_significance: Option<InAppMessageVariantSignificance>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `InAppMessageAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageAbTest {
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<InAppMessageAbTestWinner>,
    /// Zorunlu.
    #[serde(rename = "winner_variant_id")]
    pub winner_variant_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "decided_at")]
    pub decided_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "decided_by")]
    pub decided_by: Option<String>,
}

/// `InAppMessageAbTestWinner` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageAbTestWinner {
    /// Zorunlu. Değerler: `"manual"`, `"auto"`.
    #[serde(rename = "mode", deserialize_with = "crate::de::null_default")]
    pub mode: String,
    #[serde(rename = "min_impressions", skip_serializing_if = "Option::is_none")]
    pub min_impressions: Option<i64>,
    #[serde(rename = "confidence", skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// `InAppMessageAudience` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageAudience {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    #[serde(rename = "global_control_exempt", skip_serializing_if = "Option::is_none")]
    pub global_control_exempt: Option<bool>,
}

/// `InAppMessageDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageDisplay {
    /// Zorunlu. Değerler: `"modal"`, `"banner_top"`, `"banner_bottom"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    /// Değerler: `"blocks"`, `"html"`.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "html_height", skip_serializing_if = "Option::is_none")]
    pub html_height: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
}

/// `InAppMessageFrequency` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageFrequency {
    #[serde(rename = "max_impressions", skip_serializing_if = "Option::is_none")]
    pub max_impressions: Option<i64>,
    #[serde(rename = "min_interval_seconds", skip_serializing_if = "Option::is_none")]
    pub min_interval_seconds: Option<i64>,
    #[serde(rename = "stop_after_interaction", skip_serializing_if = "Option::is_none")]
    pub stop_after_interaction: Option<bool>,
}

/// `InAppMessagePage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessagePage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<InAppMessage>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `InAppMessageStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageStats {
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: i64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: i64,
    /// Zorunlu.
    #[serde(rename = "dismissals", deserialize_with = "crate::de::null_default")]
    pub dismissals: i64,
    /// Zorunlu.
    #[serde(rename = "unique_users", deserialize_with = "crate::de::null_default")]
    pub unique_users: i64,
    /// Zorunlu.
    #[serde(rename = "ctr", deserialize_with = "crate::de::null_default")]
    pub ctr: f64,
}

/// `InAppMessageVariantSignificance` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageVariantSignificance {
    /// Zorunlu.
    #[serde(rename = "leader")]
    pub leader: Option<String>,
    /// Zorunlu.
    #[serde(rename = "p_value", deserialize_with = "crate::de::null_default")]
    pub p_value: f64,
    /// Zorunlu.
    #[serde(rename = "significant", deserialize_with = "crate::de::null_default")]
    pub significant: bool,
}

/// `InAppMessageVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: InAppMessageVariantsItemDisplay,
}

/// `InAppMessageVariantsItemDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageVariantsItemDisplay {
    /// Zorunlu. Değerler: `"modal"`, `"banner_top"`, `"banner_bottom"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    /// Değerler: `"blocks"`, `"html"`.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "html_height", skip_serializing_if = "Option::is_none")]
    pub html_height: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
}

/// `InAppMessageVariantStatsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppMessageVariantStatsItem {
    /// Zorunlu.
    #[serde(rename = "variant_id", deserialize_with = "crate::de::null_default")]
    pub variant_id: String,
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: i64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: i64,
    /// Zorunlu.
    #[serde(rename = "ctr", deserialize_with = "crate::de::null_default")]
    pub ctr: f64,
}

/// `InAppStatusDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppStatusDto {
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
}

/// `InAppWinnerDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InAppWinnerDto {
    /// Zorunlu.
    #[serde(rename = "variant_id", deserialize_with = "crate::de::null_default")]
    pub variant_id: String,
}

/// `InboxAgent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxAgent {
    /// Zorunlu.
    #[serde(rename = "account_id", deserialize_with = "crate::de::null_default")]
    pub account_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "email", deserialize_with = "crate::de::null_default")]
    pub email: String,
    /// Zorunlu.
    #[serde(rename = "role", deserialize_with = "crate::de::null_default")]
    pub role: String,
    /// Zorunlu. Değerler: `"online"`, `"busy"`, `"offline"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "last_heartbeat_at")]
    pub last_heartbeat_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "max_concurrent")]
    pub max_concurrent: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "languages", deserialize_with = "crate::de::null_default")]
    pub languages: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "open_conversations", deserialize_with = "crate::de::null_default")]
    pub open_conversations: i64,
    /// Zorunlu.
    #[serde(rename = "team_ids", deserialize_with = "crate::de::null_default")]
    pub team_ids: Vec<String>,
}

/// `InboxAgentList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxAgentList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<InboxAgent>,
}

/// `InboxItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Zorunlu.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Zorunlu.
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "read", deserialize_with = "crate::de::null_default")]
    pub read: bool,
    /// Zorunlu.
    #[serde(rename = "read_at")]
    pub read_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "archived_at")]
    pub archived_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `InboxMetrics` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetrics {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "first_response", deserialize_with = "crate::de::null_default")]
    pub first_response: InboxMetricsFirstResponse,
    /// Zorunlu.
    #[serde(rename = "resolution", deserialize_with = "crate::de::null_default")]
    pub resolution: InboxMetricsResolution,
    /// Zorunlu.
    #[serde(rename = "sla", deserialize_with = "crate::de::null_default")]
    pub sla: InboxMetricsSla,
    /// Zorunlu.
    #[serde(rename = "volume", deserialize_with = "crate::de::null_default")]
    pub volume: InboxMetricsVolume,
    /// Zorunlu.
    #[serde(rename = "agents", deserialize_with = "crate::de::null_default")]
    pub agents: Vec<InboxMetricsAgentsItem>,
    /// Zorunlu.
    #[serde(rename = "daily", deserialize_with = "crate::de::null_default")]
    pub daily: Vec<InboxMetricsDailyItem>,
}

/// `InboxMetricsAgentsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetricsAgentsItem {
    /// Zorunlu.
    #[serde(rename = "account_id", deserialize_with = "crate::de::null_default")]
    pub account_id: String,
    /// Zorunlu.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "first_response", deserialize_with = "crate::de::null_default")]
    pub first_response: InboxMetricsAgentsItemFirstResponse,
    /// Zorunlu.
    #[serde(rename = "resolution", deserialize_with = "crate::de::null_default")]
    pub resolution: InboxMetricsAgentsItemResolution,
    /// Zorunlu.
    #[serde(rename = "resolved", deserialize_with = "crate::de::null_default")]
    pub resolved: i64,
    /// Zorunlu.
    #[serde(rename = "open_now", deserialize_with = "crate::de::null_default")]
    pub open_now: i64,
}

/// `InboxMetricsAgentsItemFirstResponse` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetricsAgentsItemFirstResponse {
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
    /// Zorunlu.
    #[serde(rename = "avg_seconds")]
    pub avg_seconds: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "p90_seconds")]
    pub p90_seconds: Option<f64>,
}

/// `InboxMetricsAgentsItemResolution` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetricsAgentsItemResolution {
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
    /// Zorunlu.
    #[serde(rename = "avg_seconds")]
    pub avg_seconds: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "p90_seconds")]
    pub p90_seconds: Option<f64>,
}

/// `InboxMetricsDailyItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetricsDailyItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "created", deserialize_with = "crate::de::null_default")]
    pub created: i64,
    /// Zorunlu.
    #[serde(rename = "resolved", deserialize_with = "crate::de::null_default")]
    pub resolved: i64,
    /// Zorunlu.
    #[serde(rename = "first_response_avg_seconds")]
    pub first_response_avg_seconds: Option<f64>,
}

/// `InboxMetricsFirstResponse` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetricsFirstResponse {
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
    /// Zorunlu.
    #[serde(rename = "avg_seconds")]
    pub avg_seconds: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "p90_seconds")]
    pub p90_seconds: Option<f64>,
}

/// `InboxMetricsResolution` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetricsResolution {
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
    /// Zorunlu.
    #[serde(rename = "avg_seconds")]
    pub avg_seconds: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "p90_seconds")]
    pub p90_seconds: Option<f64>,
}

/// `InboxMetricsSla` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetricsSla {
    /// Zorunlu.
    #[serde(rename = "target_seconds", deserialize_with = "crate::de::null_default")]
    pub target_seconds: i64,
    /// Zorunlu.
    #[serde(rename = "met", deserialize_with = "crate::de::null_default")]
    pub met: i64,
    /// Zorunlu.
    #[serde(rename = "breached", deserialize_with = "crate::de::null_default")]
    pub breached: i64,
    /// Zorunlu.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
}

/// `InboxMetricsVolume` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxMetricsVolume {
    /// Zorunlu.
    #[serde(rename = "created", deserialize_with = "crate::de::null_default")]
    pub created: i64,
    /// Zorunlu.
    #[serde(rename = "resolved", deserialize_with = "crate::de::null_default")]
    pub resolved: i64,
    /// Zorunlu.
    #[serde(rename = "open_now", deserialize_with = "crate::de::null_default")]
    pub open_now: i64,
    /// Zorunlu.
    #[serde(rename = "unassigned_now", deserialize_with = "crate::de::null_default")]
    pub unassigned_now: i64,
    /// Zorunlu.
    #[serde(rename = "by_channel", deserialize_with = "crate::de::null_default")]
    pub by_channel: std::collections::HashMap<String, i64>,
}

/// `InboxPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<InboxItem>,
    /// Zorunlu.
    #[serde(rename = "unread_count", deserialize_with = "crate::de::null_default")]
    pub unread_count: i64,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `InboxReadAllResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxReadAllResult {
    /// Zorunlu.
    #[serde(rename = "updated", deserialize_with = "crate::de::null_default")]
    pub updated: i64,
    /// Zorunlu.
    #[serde(rename = "unread_count", deserialize_with = "crate::de::null_default")]
    pub unread_count: i64,
}

/// `InboxSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettings {
    /// Zorunlu.
    #[serde(rename = "widget", deserialize_with = "crate::de::null_default")]
    pub widget: InboxSettingsWidget,
    /// Zorunlu.
    #[serde(rename = "business_hours", deserialize_with = "crate::de::null_default")]
    pub business_hours: InboxSettingsBusinessHours,
    /// Zorunlu.
    #[serde(rename = "routing", deserialize_with = "crate::de::null_default")]
    pub routing: InboxSettingsRouting,
    /// Zorunlu.
    #[serde(rename = "email", deserialize_with = "crate::de::null_default")]
    pub email: InboxSettingsEmail,
    /// Zorunlu.
    #[serde(rename = "timezone", deserialize_with = "crate::de::null_default")]
    pub timezone: String,
    /// Zorunlu.
    #[serde(rename = "within_business_hours", deserialize_with = "crate::de::null_default")]
    pub within_business_hours: bool,
    /// Zorunlu.
    #[serde(rename = "install", deserialize_with = "crate::de::null_default")]
    pub install: InboxSettingsInstall,
}

/// `InboxSettingsBusinessHours` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsBusinessHours {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<InboxSettingsBusinessHoursDaysItem>>,
    #[serde(rename = "holidays", skip_serializing_if = "Option::is_none")]
    pub holidays: Option<Vec<String>>,
}

/// `InboxSettingsBusinessHoursDaysItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsBusinessHoursDaysItem {
    /// Zorunlu.
    #[serde(rename = "day", deserialize_with = "crate::de::null_default")]
    pub day: i64,
    /// Zorunlu.
    #[serde(rename = "start", deserialize_with = "crate::de::null_default")]
    pub start: String,
    /// Zorunlu.
    #[serde(rename = "end", deserialize_with = "crate::de::null_default")]
    pub end: String,
}

/// `InboxSettingsEmail` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsEmail {
    /// Zorunlu.
    #[serde(rename = "capture_replies", deserialize_with = "crate::de::null_default")]
    pub capture_replies: bool,
    /// Zorunlu.
    #[serde(rename = "reply_domain")]
    pub reply_domain: Option<String>,
}

/// `InboxSettingsInstall` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsInstall {
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "sdk_url", deserialize_with = "crate::de::null_default")]
    pub sdk_url: String,
    /// Zorunlu.
    #[serde(rename = "api_url", deserialize_with = "crate::de::null_default")]
    pub api_url: String,
}

/// `InboxSettingsRouting` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsRouting {
    /// Zorunlu.
    #[serde(rename = "auto_assign", deserialize_with = "crate::de::null_default")]
    pub auto_assign: bool,
    /// Zorunlu. Değerler: `"all"`, `"team"`, `"assigned"`.
    #[serde(rename = "visibility", deserialize_with = "crate::de::null_default")]
    pub visibility: String,
    /// Zorunlu.
    #[serde(rename = "show_unassigned", deserialize_with = "crate::de::null_default")]
    pub show_unassigned: bool,
    /// Zorunlu.
    #[serde(rename = "first_response_sla_seconds", deserialize_with = "crate::de::null_default")]
    pub first_response_sla_seconds: i64,
    /// Zorunlu.
    #[serde(rename = "max_concurrent_default")]
    pub max_concurrent_default: Option<i64>,
}

/// `InboxSettingsUpdateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsUpdateDto {
    #[serde(rename = "widget", skip_serializing_if = "Option::is_none")]
    pub widget: Option<InboxSettingsUpdateDtoWidget>,
    #[serde(rename = "business_hours", skip_serializing_if = "Option::is_none")]
    pub business_hours: Option<InboxSettingsUpdateDtoBusinessHours>,
    #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
    pub routing: Option<InboxSettingsUpdateDtoRouting>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<InboxSettingsUpdateDtoEmail>,
}

/// `InboxSettingsUpdateDtoBusinessHours` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsUpdateDtoBusinessHours {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<InboxSettingsUpdateDtoBusinessHoursDaysItem>>,
    #[serde(rename = "holidays", skip_serializing_if = "Option::is_none")]
    pub holidays: Option<Vec<String>>,
}

/// `InboxSettingsUpdateDtoBusinessHoursDaysItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsUpdateDtoBusinessHoursDaysItem {
    /// Zorunlu.
    #[serde(rename = "day", deserialize_with = "crate::de::null_default")]
    pub day: i64,
    /// Zorunlu.
    #[serde(rename = "start", deserialize_with = "crate::de::null_default")]
    pub start: String,
    /// Zorunlu.
    #[serde(rename = "end", deserialize_with = "crate::de::null_default")]
    pub end: String,
}

/// `InboxSettingsUpdateDtoEmail` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsUpdateDtoEmail {
    #[serde(rename = "capture_replies", skip_serializing_if = "Option::is_none")]
    pub capture_replies: Option<bool>,
}

/// `InboxSettingsUpdateDtoRouting` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsUpdateDtoRouting {
    #[serde(rename = "auto_assign", skip_serializing_if = "Option::is_none")]
    pub auto_assign: Option<bool>,
    /// Değerler: `"all"`, `"team"`, `"assigned"`.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "show_unassigned", skip_serializing_if = "Option::is_none")]
    pub show_unassigned: Option<bool>,
    #[serde(rename = "first_response_sla_seconds", skip_serializing_if = "Option::is_none")]
    pub first_response_sla_seconds: Option<i64>,
    #[serde(rename = "max_concurrent_default", skip_serializing_if = "Option::is_none")]
    pub max_concurrent_default: Option<i64>,
}

/// `InboxSettingsUpdateDtoWidget` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsUpdateDtoWidget {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// Değerler: `"right"`, `"left"`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// Değerler: `"light"`, `"dark"`, `"auto"`.
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "greeting", skip_serializing_if = "Option::is_none")]
    pub greeting: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "offline_message", skip_serializing_if = "Option::is_none")]
    pub offline_message: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "collect_email_offline", skip_serializing_if = "Option::is_none")]
    pub collect_email_offline: Option<bool>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<bool>,
    #[serde(rename = "require_email", skip_serializing_if = "Option::is_none")]
    pub require_email: Option<bool>,
}

/// `InboxSettingsWidget` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxSettingsWidget {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// Değerler: `"right"`, `"left"`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// Değerler: `"light"`, `"dark"`, `"auto"`.
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "greeting", skip_serializing_if = "Option::is_none")]
    pub greeting: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "offline_message", skip_serializing_if = "Option::is_none")]
    pub offline_message: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "collect_email_offline", skip_serializing_if = "Option::is_none")]
    pub collect_email_offline: Option<bool>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<bool>,
    #[serde(rename = "require_email", skip_serializing_if = "Option::is_none")]
    pub require_email: Option<bool>,
}

/// `InboxTeam` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxTeam {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "member_ids", deserialize_with = "crate::de::null_default")]
    pub member_ids: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `InboxTeamList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InboxTeamList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<InboxTeam>,
}

/// `IntegrationOk` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IntegrationOk {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
}

/// `IssueWalletPassDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IssueWalletPassDto {
    /// Zorunlu.
    #[serde(rename = "template_id", deserialize_with = "crate::de::null_default")]
    pub template_id: String,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "barcode_value", skip_serializing_if = "Option::is_none")]
    pub barcode_value: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<std::collections::HashMap<String, String>>,
}

/// `Journey` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Journey {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "current_version")]
    pub current_version: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "draft_version")]
    pub draft_version: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "trigger_kind")]
    pub trigger_kind: Option<String>,
    /// Zorunlu.
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu.
    #[serde(rename = "summary", deserialize_with = "crate::de::null_default")]
    pub summary: JourneySummary,
}

/// `JourneyDetail` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyDetail {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "current_version")]
    pub current_version: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "draft_version")]
    pub draft_version: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "trigger_kind")]
    pub trigger_kind: Option<String>,
    /// Zorunlu.
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu.
    #[serde(rename = "summary", deserialize_with = "crate::de::null_default")]
    pub summary: JourneyDetailSummary,
    /// Zorunlu.
    #[serde(rename = "editing", deserialize_with = "crate::de::null_default")]
    pub editing: JourneyDetailEditing,
    /// Zorunlu.
    #[serde(rename = "issues", deserialize_with = "crate::de::null_default")]
    pub issues: Vec<JourneyDetailIssuesItem>,
}

/// `JourneyDetailEditing` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyDetailEditing {
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "graph", deserialize_with = "crate::de::null_default")]
    pub graph: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "published_by")]
    pub published_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `JourneyDetailIssuesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyDetailIssuesItem {
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "edge_id", skip_serializing_if = "Option::is_none")]
    pub edge_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "message", deserialize_with = "crate::de::null_default")]
    pub message: String,
}

/// `JourneyDetailSummary` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyDetailSummary {
    /// Zorunlu.
    #[serde(rename = "runs_total", deserialize_with = "crate::de::null_default")]
    pub runs_total: i64,
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
    /// Zorunlu.
    #[serde(rename = "completed", deserialize_with = "crate::de::null_default")]
    pub completed: i64,
    /// Zorunlu.
    #[serde(rename = "exited", deserialize_with = "crate::de::null_default")]
    pub exited: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "goal_reached", deserialize_with = "crate::de::null_default")]
    pub goal_reached: i64,
}

/// `JourneyPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Journey>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `JourneyPublishResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyPublishResult {
    /// Zorunlu.
    #[serde(rename = "journey", deserialize_with = "crate::de::null_default")]
    pub journey: Journey,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "issues", deserialize_with = "crate::de::null_default")]
    pub issues: Vec<JourneyPublishResultIssuesItem>,
}

/// `JourneyPublishResultIssuesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyPublishResultIssuesItem {
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "edge_id", skip_serializing_if = "Option::is_none")]
    pub edge_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "message", deserialize_with = "crate::de::null_default")]
    pub message: String,
}

/// `JourneyRun` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyRun {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "journey_id", deserialize_with = "crate::de::null_default")]
    pub journey_id: String,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu. Değerler: `"active"`, `"waiting"`, `"completed"`, `"exited"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "current_node_id")]
    pub current_node_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "next_action_at")]
    pub next_action_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "entered_at", deserialize_with = "crate::de::null_default")]
    pub entered_at: String,
    /// Zorunlu.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "exit_reason")]
    pub exit_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "goal_id")]
    pub goal_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "goal_reached_at")]
    pub goal_reached_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "context", deserialize_with = "crate::de::null_default")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
}

/// `JourneyRunDetail` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyRunDetail {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "journey_id", deserialize_with = "crate::de::null_default")]
    pub journey_id: String,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu. Değerler: `"active"`, `"waiting"`, `"completed"`, `"exited"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "current_node_id")]
    pub current_node_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "next_action_at")]
    pub next_action_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "entered_at", deserialize_with = "crate::de::null_default")]
    pub entered_at: String,
    /// Zorunlu.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "exit_reason")]
    pub exit_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "goal_id")]
    pub goal_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "goal_reached_at")]
    pub goal_reached_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "context", deserialize_with = "crate::de::null_default")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "journey_name", deserialize_with = "crate::de::null_default")]
    pub journey_name: String,
    /// Zorunlu.
    #[serde(rename = "steps", deserialize_with = "crate::de::null_default")]
    pub steps: Vec<JourneyRunDetailStepsItem>,
}

/// `JourneyRunDetailStepsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyRunDetailStepsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "node_id", deserialize_with = "crate::de::null_default")]
    pub node_id: String,
    /// Zorunlu.
    #[serde(rename = "node_type", deserialize_with = "crate::de::null_default")]
    pub node_type: String,
    /// Zorunlu. Değerler: `"completed"`, `"waiting"`, `"failed"`, `"skipped"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "branch")]
    pub branch: Option<String>,
    /// Zorunlu.
    #[serde(rename = "result", deserialize_with = "crate::de::null_default")]
    pub result: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "started_at", deserialize_with = "crate::de::null_default")]
    pub started_at: String,
    /// Zorunlu.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<String>,
}

/// `JourneyRunPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyRunPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<JourneyRun>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `JourneysCreateBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneysCreateBody {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// React Flow uyumlu journey grafı: { nodes: [{ id, type, position, data }], edges: [{ id, source, target, sourceHandle }] } (docs/journeys.md)
    #[serde(rename = "graph", skip_serializing_if = "Option::is_none")]
    pub graph: Option<JourneysCreateBodyGraph>,
    /// Yeniden giriş, yalnızca yeni girenler, çıkış kuralları, çıkış tag'leri, hedefler, takvim (docs/journeys.md)
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// React Flow uyumlu journey grafı: { nodes: [{ id, type, position, data }], edges: [{ id, source, target, sourceHandle }] } (docs/journeys.md)
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneysCreateBodyGraph {
    /// Zorunlu.
    #[serde(rename = "nodes", deserialize_with = "crate::de::null_default")]
    pub nodes: Vec<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "edges", deserialize_with = "crate::de::null_default")]
    pub edges: Vec<std::collections::HashMap<String, serde_json::Value>>,
}

/// `JourneysSaveDraftBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneysSaveDraftBody {
    /// Zorunlu. React Flow uyumlu journey grafı: { nodes: [{ id, type, position, data }], edges: [{ id, source, target, sourceHandle }] } (docs/journeys.md)
    #[serde(rename = "graph", deserialize_with = "crate::de::null_default")]
    pub graph: JourneysSaveDraftBodyGraph,
    /// Yeniden giriş, yalnızca yeni girenler, çıkış kuralları, çıkış tag'leri, hedefler, takvim (docs/journeys.md)
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// React Flow uyumlu journey grafı: { nodes: [{ id, type, position, data }], edges: [{ id, source, target, sourceHandle }] } (docs/journeys.md)
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneysSaveDraftBodyGraph {
    /// Zorunlu.
    #[serde(rename = "nodes", deserialize_with = "crate::de::null_default")]
    pub nodes: Vec<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "edges", deserialize_with = "crate::de::null_default")]
    pub edges: Vec<std::collections::HashMap<String, serde_json::Value>>,
}

/// `JourneyStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyStats {
    /// Zorunlu.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "summary", deserialize_with = "crate::de::null_default")]
    pub summary: JourneyStatsSummary,
    /// Zorunlu.
    #[serde(rename = "nodes", deserialize_with = "crate::de::null_default")]
    pub nodes: Vec<JourneyStatsNodesItem>,
    /// Zorunlu.
    #[serde(rename = "goals", deserialize_with = "crate::de::null_default")]
    pub goals: Vec<JourneyStatsGoalsItem>,
    /// Zorunlu.
    #[serde(rename = "exit_reasons", deserialize_with = "crate::de::null_default")]
    pub exit_reasons: std::collections::HashMap<String, i64>,
}

/// `JourneyStatsGoalsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyStatsGoalsItem {
    /// Zorunlu.
    #[serde(rename = "goal_id", deserialize_with = "crate::de::null_default")]
    pub goal_id: String,
    /// Zorunlu.
    #[serde(rename = "reached", deserialize_with = "crate::de::null_default")]
    pub reached: i64,
}

/// `JourneyStatsNodesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyStatsNodesItem {
    /// Zorunlu.
    #[serde(rename = "node_id", deserialize_with = "crate::de::null_default")]
    pub node_id: String,
    /// Zorunlu.
    #[serde(rename = "node_type", deserialize_with = "crate::de::null_default")]
    pub node_type: String,
    /// Zorunlu.
    #[serde(rename = "entered", deserialize_with = "crate::de::null_default")]
    pub entered: i64,
    /// Zorunlu.
    #[serde(rename = "completed", deserialize_with = "crate::de::null_default")]
    pub completed: i64,
    /// Zorunlu.
    #[serde(rename = "waiting_now", deserialize_with = "crate::de::null_default")]
    pub waiting_now: i64,
    /// Zorunlu.
    #[serde(rename = "skipped", deserialize_with = "crate::de::null_default")]
    pub skipped: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "branches", deserialize_with = "crate::de::null_default")]
    pub branches: std::collections::HashMap<String, i64>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<std::collections::HashMap<String, i64>>,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<JourneyStatsNodesItemVariantsItem>>,
}

/// `JourneyStatsNodesItemVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyStatsNodesItemVariantsItem {
    /// Zorunlu.
    #[serde(rename = "variant_id", deserialize_with = "crate::de::null_default")]
    pub variant_id: String,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "winner", deserialize_with = "crate::de::null_default")]
    pub winner: bool,
}

/// `JourneyStatsSummary` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyStatsSummary {
    /// Zorunlu.
    #[serde(rename = "runs_total", deserialize_with = "crate::de::null_default")]
    pub runs_total: i64,
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
    /// Zorunlu.
    #[serde(rename = "completed", deserialize_with = "crate::de::null_default")]
    pub completed: i64,
    /// Zorunlu.
    #[serde(rename = "exited", deserialize_with = "crate::de::null_default")]
    pub exited: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "goal_reached", deserialize_with = "crate::de::null_default")]
    pub goal_reached: i64,
    /// Zorunlu.
    #[serde(rename = "waiting", deserialize_with = "crate::de::null_default")]
    pub waiting: i64,
    /// Zorunlu.
    #[serde(rename = "goal_conversion_rate", deserialize_with = "crate::de::null_default")]
    pub goal_conversion_rate: f64,
}

/// `JourneySummary` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneySummary {
    /// Zorunlu.
    #[serde(rename = "runs_total", deserialize_with = "crate::de::null_default")]
    pub runs_total: i64,
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
    /// Zorunlu.
    #[serde(rename = "completed", deserialize_with = "crate::de::null_default")]
    pub completed: i64,
    /// Zorunlu.
    #[serde(rename = "exited", deserialize_with = "crate::de::null_default")]
    pub exited: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "goal_reached", deserialize_with = "crate::de::null_default")]
    pub goal_reached: i64,
}

/// `JourneyTriggerResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyTriggerResult {
    /// Zorunlu.
    #[serde(rename = "entered", deserialize_with = "crate::de::null_default")]
    pub entered: bool,
    /// Zorunlu.
    #[serde(rename = "run")]
    pub run: Option<JourneyRun>,
    /// Zorunlu. Değerler: `"inactive"`, `"outside_schedule"`, `"re_entry"`, `"already_active"`, `"unknown_user"`.
    #[serde(rename = "reason")]
    pub reason: Option<String>,
}

/// `JourneyVersionList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyVersionList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<JourneyVersionListDataItem>,
}

/// `JourneyVersionListDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct JourneyVersionListDataItem {
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "graph", deserialize_with = "crate::de::null_default")]
    pub graph: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "published_by")]
    pub published_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `LandingPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPage {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu. Değerler: `"draft"`, `"published"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "seo", deserialize_with = "crate::de::null_default")]
    pub seo: LandingPageSeo,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: LandingPageSettings,
    /// Zorunlu.
    #[serde(rename = "custom_domain_id")]
    pub custom_domain_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "published_version")]
    pub published_version: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_unpublished_changes", deserialize_with = "crate::de::null_default")]
    pub has_unpublished_changes: bool,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    /// Zorunlu.
    #[serde(rename = "domain_url")]
    pub domain_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: LandingPageStats,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu.
    #[serde(rename = "document", deserialize_with = "crate::de::null_default")]
    pub document: LandingPageDocument,
}

/// `LandingPageDocument` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageDocument {
    /// Zorunlu. Değerler: `1`.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: f64,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: LandingPageDocumentSettings,
    /// Zorunlu.
    #[serde(rename = "sections", deserialize_with = "crate::de::null_default")]
    pub sections: Vec<LandingPageDocumentSectionsItem>,
}

/// `LandingPageDocumentSectionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageDocumentSectionsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"section"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "background_image", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    #[serde(rename = "padding", skip_serializing_if = "Option::is_none")]
    pub padding: Option<LandingPageDocumentSectionsItemPadding>,
    #[serde(rename = "stack_on_mobile", skip_serializing_if = "Option::is_none")]
    pub stack_on_mobile: Option<bool>,
    /// Zorunlu.
    #[serde(rename = "columns", deserialize_with = "crate::de::null_default")]
    pub columns: Vec<LandingPageDocumentSectionsItemColumnsItem>,
}

/// `LandingPageDocumentSectionsItemColumnsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageDocumentSectionsItemColumnsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    #[serde(rename = "width_percent", skip_serializing_if = "Option::is_none")]
    pub width_percent: Option<i64>,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// Değerler: `"top"`, `"middle"`, `"bottom"`.
    #[serde(rename = "vertical_align", skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<String>,
    #[serde(rename = "padding", skip_serializing_if = "Option::is_none")]
    pub padding: Option<LandingPageDocumentSectionsItemColumnsItemPadding>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
}

/// `LandingPageDocumentSectionsItemColumnsItemPadding` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageDocumentSectionsItemColumnsItemPadding {
    /// Zorunlu.
    #[serde(rename = "top", deserialize_with = "crate::de::null_default")]
    pub top: i64,
    /// Zorunlu.
    #[serde(rename = "right", deserialize_with = "crate::de::null_default")]
    pub right: i64,
    /// Zorunlu.
    #[serde(rename = "bottom", deserialize_with = "crate::de::null_default")]
    pub bottom: i64,
    /// Zorunlu.
    #[serde(rename = "left", deserialize_with = "crate::de::null_default")]
    pub left: i64,
}

/// `LandingPageDocumentSectionsItemPadding` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageDocumentSectionsItemPadding {
    /// Zorunlu.
    #[serde(rename = "top", deserialize_with = "crate::de::null_default")]
    pub top: i64,
    /// Zorunlu.
    #[serde(rename = "right", deserialize_with = "crate::de::null_default")]
    pub right: i64,
    /// Zorunlu.
    #[serde(rename = "bottom", deserialize_with = "crate::de::null_default")]
    pub bottom: i64,
    /// Zorunlu.
    #[serde(rename = "left", deserialize_with = "crate::de::null_default")]
    pub left: i64,
}

/// `LandingPageDocumentSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageDocumentSettings {
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "content_background_color", skip_serializing_if = "Option::is_none")]
    pub content_background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "link_color", skip_serializing_if = "Option::is_none")]
    pub link_color: Option<String>,
    /// Değerler: `"system"`, `"arial"`, `"georgia"`, `"verdana"`, `"trebuchet"`, `"courier"`.
    #[serde(rename = "font_family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(rename = "dark_mode", skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<LandingPageDocumentSettingsDarkMode>,
}

/// `LandingPageDocumentSettingsDarkMode` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageDocumentSettingsDarkMode {
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "content_background_color", skip_serializing_if = "Option::is_none")]
    pub content_background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "link_color", skip_serializing_if = "Option::is_none")]
    pub link_color: Option<String>,
}

/// `LandingPageList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<LandingPageListDataItem>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `LandingPageListDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageListDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu. Değerler: `"draft"`, `"published"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "seo", deserialize_with = "crate::de::null_default")]
    pub seo: LandingPageListDataItemSeo,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: LandingPageListDataItemSettings,
    /// Zorunlu.
    #[serde(rename = "custom_domain_id")]
    pub custom_domain_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "published_version")]
    pub published_version: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_unpublished_changes", deserialize_with = "crate::de::null_default")]
    pub has_unpublished_changes: bool,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    /// Zorunlu.
    #[serde(rename = "domain_url")]
    pub domain_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: LandingPageListDataItemStats,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `LandingPageListDataItemSeo` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageListDataItemSeo {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "og_image", skip_serializing_if = "Option::is_none")]
    pub og_image: Option<String>,
    #[serde(rename = "canonical_url", skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
    #[serde(rename = "noindex", skip_serializing_if = "Option::is_none")]
    pub noindex: Option<bool>,
}

/// `LandingPageListDataItemSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageListDataItemSettings {
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "form_section_id", skip_serializing_if = "Option::is_none")]
    pub form_section_id: Option<String>,
    #[serde(rename = "cookie_consent", skip_serializing_if = "Option::is_none")]
    pub cookie_consent: Option<LandingPageListDataItemSettingsCookieConsent>,
    #[serde(rename = "web_sdk", skip_serializing_if = "Option::is_none")]
    pub web_sdk: Option<bool>,
    #[serde(rename = "favicon_url", skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
}

/// `LandingPageListDataItemSettingsCookieConsent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageListDataItemSettingsCookieConsent {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "accept_label", skip_serializing_if = "Option::is_none")]
    pub accept_label: Option<String>,
    #[serde(rename = "reject_label", skip_serializing_if = "Option::is_none")]
    pub reject_label: Option<String>,
    #[serde(rename = "policy_url", skip_serializing_if = "Option::is_none")]
    pub policy_url: Option<String>,
}

/// `LandingPageListDataItemStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageListDataItemStats {
    /// Zorunlu.
    #[serde(rename = "submissions", deserialize_with = "crate::de::null_default")]
    pub submissions: i64,
}

/// `LandingPageSeo` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageSeo {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "og_image", skip_serializing_if = "Option::is_none")]
    pub og_image: Option<String>,
    #[serde(rename = "canonical_url", skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
    #[serde(rename = "noindex", skip_serializing_if = "Option::is_none")]
    pub noindex: Option<bool>,
}

/// `LandingPageSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageSettings {
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "form_section_id", skip_serializing_if = "Option::is_none")]
    pub form_section_id: Option<String>,
    #[serde(rename = "cookie_consent", skip_serializing_if = "Option::is_none")]
    pub cookie_consent: Option<LandingPageSettingsCookieConsent>,
    #[serde(rename = "web_sdk", skip_serializing_if = "Option::is_none")]
    pub web_sdk: Option<bool>,
    #[serde(rename = "favicon_url", skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
}

/// `LandingPageSettingsCookieConsent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageSettingsCookieConsent {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "accept_label", skip_serializing_if = "Option::is_none")]
    pub accept_label: Option<String>,
    #[serde(rename = "reject_label", skip_serializing_if = "Option::is_none")]
    pub reject_label: Option<String>,
    #[serde(rename = "policy_url", skip_serializing_if = "Option::is_none")]
    pub policy_url: Option<String>,
}

/// `LandingPageStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LandingPageStats {
    /// Zorunlu.
    #[serde(rename = "submissions", deserialize_with = "crate::de::null_default")]
    pub submissions: i64,
}

/// `LiveActivitySendResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveActivitySendResult {
    /// Zorunlu.
    #[serde(rename = "message_id", deserialize_with = "crate::de::null_default")]
    pub message_id: String,
    /// Zorunlu.
    #[serde(rename = "activity_id", deserialize_with = "crate::de::null_default")]
    pub activity_id: String,
    /// Zorunlu. Değerler: `"start"`, `"update"`, `"end"`.
    #[serde(rename = "event", deserialize_with = "crate::de::null_default")]
    pub event: String,
    /// Zorunlu.
    #[serde(rename = "recipients_estimate", deserialize_with = "crate::de::null_default")]
    pub recipients_estimate: i64,
}

/// `LiveActivityTokenResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveActivityTokenResult {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
}

/// `LiveActivityType` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveActivityType {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "content_state_schema", deserialize_with = "crate::de::null_default")]
    pub content_state_schema: LiveActivityTypeContentStateSchema,
    /// Zorunlu.
    #[serde(rename = "attributes_schema", deserialize_with = "crate::de::null_default")]
    pub attributes_schema: LiveActivityTypeAttributesSchema,
    /// Zorunlu.
    #[serde(rename = "push_to_start_tokens", deserialize_with = "crate::de::null_default")]
    pub push_to_start_tokens: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `LiveActivityTypeAttributesSchema` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveActivityTypeAttributesSchema {
    /// Zorunlu. Değerler: `"object"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, LiveActivityTypeAttributesSchemaPropertiesValue>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// `LiveActivityTypeAttributesSchemaPropertiesValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveActivityTypeAttributesSchemaPropertiesValue {
    /// Zorunlu. Değerler: `"string"`, `"number"`, `"integer"`, `"boolean"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `LiveActivityTypeContentStateSchema` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveActivityTypeContentStateSchema {
    /// Zorunlu. Değerler: `"object"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, LiveActivityTypeContentStateSchemaPropertiesValue>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// `LiveActivityTypeContentStateSchemaPropertiesValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveActivityTypeContentStateSchemaPropertiesValue {
    /// Zorunlu. Değerler: `"string"`, `"number"`, `"integer"`, `"boolean"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `LiveActivityTypeList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LiveActivityTypeList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<LiveActivityType>,
}

/// `McpCreateCampaignDraftBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBody {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"one_time"`, `"scheduled"`, `"recurring"`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<McpCreateCampaignDraftBodyAudience>,
    /// messages.content ile aynı şema (headings, contents, url, buttons…)
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Tercih kategorisi anahtarı; kapalı kullanıcılar bastırılır
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Dönüşüm hedefleri (ADR-0080): birincil A/B kazananını belirler; olaylar dönüşüm olayı olarak tanımlı olmalı.
    #[serde(rename = "goals", skip_serializing_if = "Option::is_none")]
    pub goals: Option<McpCreateCampaignDraftBodyGoals>,
    #[serde(rename = "ab_test", skip_serializing_if = "Option::is_none")]
    pub ab_test: Option<McpCreateCampaignDraftBodyAbTest>,
    /// A/B varyantları (ağırlıklar toplamı 100; en fazla bir is_control)
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<McpCreateCampaignDraftBodyVariantsItem>>,
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "template_version", skip_serializing_if = "Option::is_none")]
    pub template_version: Option<i64>,
    #[serde(rename = "delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<McpCreateCampaignDraftBodyDelivery>,
    #[serde(rename = "recurrence", skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<McpCreateCampaignDraftBodyRecurrence>,
    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
}

/// `McpCreateCampaignDraftBodyAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBodyAbTest {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "test_percentage", skip_serializing_if = "Option::is_none")]
    pub test_percentage: Option<i64>,
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<McpCreateCampaignDraftBodyAbTestWinner>,
}

/// `McpCreateCampaignDraftBodyAbTestWinner` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBodyAbTestWinner {
    /// Değerler: `"manual"`, `"auto"`.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Değerler: `"open_rate"`, `"click_rate"`, `"conversion_rate"`.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "evaluate_after_minutes", skip_serializing_if = "Option::is_none")]
    pub evaluate_after_minutes: Option<i64>,
}

/// `McpCreateCampaignDraftBodyAudience` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBodyAudience {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<String>>,
    #[serde(rename = "exclude_segments", skip_serializing_if = "Option::is_none")]
    pub exclude_segments: Option<Vec<String>>,
    /// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<McpCreateCampaignDraftBodyAudienceFilters>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "test_devices_only", skip_serializing_if = "Option::is_none")]
    pub test_devices_only: Option<bool>,
}

/// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBodyAudienceFilters {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// `McpCreateCampaignDraftBodyDelivery` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBodyDelivery {
    /// Değerler: `"immediate"`, `"per_user_timezone"`, `"optimized"`.
    #[serde(rename = "timezone_mode", skip_serializing_if = "Option::is_none")]
    pub timezone_mode: Option<String>,
    #[serde(rename = "delivery_time_of_day", skip_serializing_if = "Option::is_none")]
    pub delivery_time_of_day: Option<String>,
    #[serde(rename = "throttle_per_minute", skip_serializing_if = "Option::is_none")]
    pub throttle_per_minute: Option<i64>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// Değerler: `5`, `10`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "collapse_id", skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    #[serde(rename = "frequency_cap_exempt", skip_serializing_if = "Option::is_none")]
    pub frequency_cap_exempt: Option<bool>,
    /// Değerler: `"low"`, `"normal"`, `"high"`.
    #[serde(rename = "frequency_priority", skip_serializing_if = "Option::is_none")]
    pub frequency_priority: Option<String>,
    #[serde(rename = "global_control_exempt", skip_serializing_if = "Option::is_none")]
    pub global_control_exempt: Option<bool>,
    #[serde(rename = "quiet_hours_exempt", skip_serializing_if = "Option::is_none")]
    pub quiet_hours_exempt: Option<bool>,
    #[serde(rename = "one_per_user", skip_serializing_if = "Option::is_none")]
    pub one_per_user: Option<bool>,
}

/// Dönüşüm hedefleri (ADR-0080): birincil A/B kazananını belirler; olaylar dönüşüm olayı olarak tanımlı olmalı.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBodyGoals {
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(rename = "secondary", skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<String>>,
}

/// `McpCreateCampaignDraftBodyRecurrence` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBodyRecurrence {
    #[serde(rename = "cron", skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// `McpCreateCampaignDraftBodyVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateCampaignDraftBodyVariantsItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "is_control", skip_serializing_if = "Option::is_none")]
    pub is_control: Option<bool>,
}

/// `McpCreateJourneyDraftBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateJourneyDraftBody {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// React Flow uyumlu journey grafı: { nodes: [{ id, type, position, data }], edges: [{ id, source, target, sourceHandle }] } (docs/journeys.md)
    #[serde(rename = "graph", skip_serializing_if = "Option::is_none")]
    pub graph: Option<McpCreateJourneyDraftBodyGraph>,
    /// Yeniden giriş, yalnızca yeni girenler, çıkış kuralları, çıkış tag'leri, hedefler, takvim (docs/journeys.md)
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// React Flow uyumlu journey grafı: { nodes: [{ id, type, position, data }], edges: [{ id, source, target, sourceHandle }] } (docs/journeys.md)
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateJourneyDraftBodyGraph {
    /// Zorunlu.
    #[serde(rename = "nodes", deserialize_with = "crate::de::null_default")]
    pub nodes: Vec<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "edges", deserialize_with = "crate::de::null_default")]
    pub edges: Vec<std::collections::HashMap<String, serde_json::Value>>,
}

/// `McpCreateSegmentDraftBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateSegmentDraftBody {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Zorunlu. Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
    #[serde(rename = "definition", deserialize_with = "crate::de::null_default")]
    pub definition: McpCreateSegmentDraftBodyDefinition,
    #[serde(rename = "is_dynamic", skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
}

/// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct McpCreateSegmentDraftBodyDefinition {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// `Member` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Member {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "account_id", deserialize_with = "crate::de::null_default")]
    pub account_id: String,
    /// Zorunlu.
    #[serde(rename = "email", deserialize_with = "crate::de::null_default")]
    pub email: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"owner"`, `"admin"`, `"developer"`, `"marketer"`, `"analyst"`, `"billing"`, `"viewer"`, `"composer"`, `"agent"`.
    #[serde(rename = "role", deserialize_with = "crate::de::null_default")]
    pub role: String,
    /// Zorunlu.
    #[serde(rename = "app_scopes", deserialize_with = "crate::de::null_default")]
    pub app_scopes: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "app_roles", deserialize_with = "crate::de::null_default")]
    pub app_roles: Vec<MemberAppRolesItem>,
    /// Zorunlu.
    #[serde(rename = "accepted_at")]
    pub accepted_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "last_login_at")]
    pub last_login_at: Option<String>,
}

/// `MemberAppRolesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MemberAppRolesItem {
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu. Değerler: `"owner"`, `"admin"`, `"developer"`, `"marketer"`, `"analyst"`, `"billing"`, `"viewer"`, `"composer"`, `"agent"`.
    #[serde(rename = "role", deserialize_with = "crate::de::null_default")]
    pub role: String,
}

/// `MemberList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MemberList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Member>,
}

/// `Message` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Message {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu. Değerler: `"campaign"`, `"api"`, `"journey"`, `"test"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu. Değerler: `"queued"`, `"processing"`, `"completed"`, `"cancelled"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "idempotency_key")]
    pub idempotency_key: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "content", deserialize_with = "crate::de::null_default")]
    pub content: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "options", deserialize_with = "crate::de::null_default")]
    pub options: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "labels", deserialize_with = "crate::de::null_default")]
    pub labels: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Zorunlu.
    #[serde(rename = "template_id")]
    pub template_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "campaign_id")]
    pub campaign_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "variants")]
    pub variants: Option<Vec<MessageVariantsItem>>,
    /// Zorunlu.
    #[serde(rename = "counts", deserialize_with = "crate::de::null_default")]
    pub counts: MessageCounts,
    /// Zorunlu.
    #[serde(rename = "requested_by", deserialize_with = "crate::de::null_default")]
    pub requested_by: MessageRequestedBy,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "scheduled_for")]
    pub scheduled_for: Option<String>,
    /// Zorunlu.
    #[serde(rename = "started_at")]
    pub started_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `MessageCategory` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessageCategory {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "description", deserialize_with = "crate::de::null_default")]
    pub description: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "channels")]
    pub channels: Option<Vec<String>>,
    /// Zorunlu.
    #[serde(rename = "default_opted_in", deserialize_with = "crate::de::null_default")]
    pub default_opted_in: bool,
    /// Zorunlu.
    #[serde(rename = "position", deserialize_with = "crate::de::null_default")]
    pub position: i64,
    /// Zorunlu.
    #[serde(rename = "opted_out_users", deserialize_with = "crate::de::null_default")]
    pub opted_out_users: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `MessageCategoryList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessageCategoryList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<MessageCategory>,
}

/// `MessageCounts` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessageCounts {
    #[serde(rename = "targeted", skip_serializing_if = "Option::is_none")]
    pub targeted: Option<i64>,
    #[serde(rename = "suppressed", skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<i64>,
    #[serde(rename = "sent", skip_serializing_if = "Option::is_none")]
    pub sent: Option<i64>,
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(rename = "errored", skip_serializing_if = "Option::is_none")]
    pub errored: Option<i64>,
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<i64>,
    #[serde(rename = "opened", skip_serializing_if = "Option::is_none")]
    pub opened: Option<i64>,
    #[serde(rename = "clicked", skip_serializing_if = "Option::is_none")]
    pub clicked: Option<i64>,
    #[serde(rename = "converted", skip_serializing_if = "Option::is_none")]
    pub converted: Option<i64>,
    #[serde(rename = "bounced", skip_serializing_if = "Option::is_none")]
    pub bounced: Option<i64>,
    #[serde(rename = "complained", skip_serializing_if = "Option::is_none")]
    pub complained: Option<i64>,
    #[serde(rename = "unsubscribed", skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
    #[serde(rename = "sms_segments", skip_serializing_if = "Option::is_none")]
    pub sms_segments: Option<i64>,
}

/// `MessageList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessageList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Message>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `MessageRequestedBy` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessageRequestedBy {
    /// Zorunlu. Değerler: `"api_key"`, `"account"`, `"system"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// `MessagesReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessagesReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu. Değerler: `"day"`, `"channel"`, `"campaign"`.
    #[serde(rename = "group_by", deserialize_with = "crate::de::null_default")]
    pub group_by: String,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: MessagesReportTotals,
    /// Zorunlu.
    #[serde(rename = "previous")]
    pub previous: Option<MessagesReportPrevious>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<MessagesReportRowsItem>,
}

/// `MessagesReportPrevious` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessagesReportPrevious {
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "delivered", deserialize_with = "crate::de::null_default")]
    pub delivered: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "delivery_rate")]
    pub delivery_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "open_rate")]
    pub open_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "click_rate")]
    pub click_rate: Option<f64>,
}

/// `MessagesReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessagesReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "delivered", deserialize_with = "crate::de::null_default")]
    pub delivered: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
}

/// `MessagesReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessagesReportTotals {
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "delivered", deserialize_with = "crate::de::null_default")]
    pub delivered: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "delivery_rate")]
    pub delivery_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "open_rate")]
    pub open_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "click_rate")]
    pub click_rate: Option<f64>,
}

/// `MessageVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessageVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "is_control", deserialize_with = "crate::de::null_default")]
    pub is_control: bool,
}

/// `MessengerLink` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MessengerLink {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    /// Zorunlu.
    #[serde(rename = "token", deserialize_with = "crate::de::null_default")]
    pub token: String,
    /// Zorunlu.
    #[serde(rename = "expires_at", deserialize_with = "crate::de::null_default")]
    pub expires_at: String,
}

/// `Ok` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OkModel {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
}

/// `Org` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Org {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu.
    #[serde(rename = "plan_code", deserialize_with = "crate::de::null_default")]
    pub plan_code: String,
    /// Zorunlu. Değerler: `"active"`, `"suspended"`, `"deleted"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: OrgSettings,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// Zorunlu.
    #[serde(rename = "managed_by_reseller", deserialize_with = "crate::de::null_default")]
    pub managed_by_reseller: bool,
}

/// `OrgSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrgSettings {
    #[serde(rename = "mfaRequired", skip_serializing_if = "Option::is_none")]
    pub mfa_required: Option<bool>,
    #[serde(rename = "ipAllowlist", skip_serializing_if = "Option::is_none")]
    pub ip_allowlist: Option<Vec<String>>,
    #[serde(rename = "retentionDays", skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,
    #[serde(rename = "sessionMaxHours", skip_serializing_if = "Option::is_none")]
    pub session_max_hours: Option<i64>,
    #[serde(rename = "auditStream", skip_serializing_if = "Option::is_none")]
    pub audit_stream: Option<OrgSettingsAuditStream>,
}

/// `OrgSettingsAuditStream` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrgSettingsAuditStream {
    #[serde(rename = "s3", skip_serializing_if = "Option::is_none")]
    pub s3: Option<bool>,
    #[serde(rename = "webhookAppId", skip_serializing_if = "Option::is_none")]
    pub webhook_app_id: Option<String>,
}

/// `OverviewStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewStats {
    /// Zorunlu. Değerler: `"7d"`, `"30d"`, `"custom"`.
    #[serde(rename = "range", deserialize_with = "crate::de::null_default")]
    pub range: String,
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "kpis", deserialize_with = "crate::de::null_default")]
    pub kpis: OverviewStatsKpis,
    /// Zorunlu.
    #[serde(rename = "series", deserialize_with = "crate::de::null_default")]
    pub series: Vec<OverviewStatsSeriesItem>,
    /// Zorunlu.
    #[serde(rename = "recent_messages", deserialize_with = "crate::de::null_default")]
    pub recent_messages: Vec<Message>,
    /// Zorunlu.
    #[serde(rename = "recent_campaigns", deserialize_with = "crate::de::null_default")]
    pub recent_campaigns: Vec<OverviewStatsRecentCampaignsItem>,
    /// Zorunlu.
    #[serde(rename = "top_errors", deserialize_with = "crate::de::null_default")]
    pub top_errors: Vec<OverviewStatsTopErrorsItem>,
    /// Zorunlu.
    #[serde(rename = "channels", deserialize_with = "crate::de::null_default")]
    pub channels: Vec<OverviewStatsChannelsItem>,
}

/// `OverviewStatsChannelsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewStatsChannelsItem {
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu. Değerler: `"apns"`, `"fcm"`, `"hms"`, `"adm"`, `"wns"`, `"webpush"`, `"smtp"`, `"ses"`, `"resend"`, `"twilio"`, `"netgsm"`, `"meta_whatsapp"`, `"google_rbm"`, `"telegram_bot"`, `"viber_bot"`, `"mock"`.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Zorunlu. Değerler: `"unconfigured"`, `"active"`, `"invalid"`, `"suspended"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "active_subscriptions", deserialize_with = "crate::de::null_default")]
    pub active_subscriptions: i64,
}

/// `OverviewStatsKpis` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewStatsKpis {
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "delivered", deserialize_with = "crate::de::null_default")]
    pub delivered: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "delivery_rate")]
    pub delivery_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "open_rate")]
    pub open_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "click_rate")]
    pub click_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "active_subscriptions", deserialize_with = "crate::de::null_default")]
    pub active_subscriptions: i64,
    /// Zorunlu.
    #[serde(rename = "new_subscriptions", deserialize_with = "crate::de::null_default")]
    pub new_subscriptions: i64,
    /// Zorunlu.
    #[serde(rename = "opt_outs", deserialize_with = "crate::de::null_default")]
    pub opt_outs: i64,
}

/// `OverviewStatsRecentCampaignsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewStatsRecentCampaignsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `OverviewStatsSeriesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewStatsSeriesItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "delivered", deserialize_with = "crate::de::null_default")]
    pub delivered: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
}

/// `OverviewStatsTopErrorsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewStatsTopErrorsItem {
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
}

/// `PatchEventDefinitionDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PatchEventDefinitionDto {
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "is_conversion", skip_serializing_if = "Option::is_none")]
    pub is_conversion: Option<bool>,
    #[serde(rename = "outcome_unique", skip_serializing_if = "Option::is_none")]
    pub outcome_unique: Option<bool>,
    #[serde(rename = "outcome_unattributed", skip_serializing_if = "Option::is_none")]
    pub outcome_unattributed: Option<bool>,
    #[serde(rename = "retention_days", skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,
}

/// `PatchSubscriptionDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PatchSubscriptionDto {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<PatchSubscriptionDtoDevice>,
    /// Değerler: `"subscribed"`, `"unsubscribed"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "test_device", skip_serializing_if = "Option::is_none")]
    pub test_device: Option<bool>,
    #[serde(rename = "test_device_name", skip_serializing_if = "Option::is_none")]
    pub test_device_name: Option<String>,
}

/// `PatchSubscriptionDtoDevice` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PatchSubscriptionDtoDevice {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "app_version", skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "sdk_version", skip_serializing_if = "Option::is_none")]
    pub sdk_version: Option<String>,
    #[serde(rename = "sdk_platform", skip_serializing_if = "Option::is_none")]
    pub sdk_platform: Option<String>,
    #[serde(rename = "carrier", skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "push_capable", skip_serializing_if = "Option::is_none")]
    pub push_capable: Option<bool>,
    /// Değerler: `"sandbox"`, `"production"`.
    #[serde(rename = "push_environment", skip_serializing_if = "Option::is_none")]
    pub push_environment: Option<String>,
    /// Değerler: `"phone"`, `"watch"`.
    #[serde(rename = "form_factor", skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
    /// Değerler: `"alert"`, `"voip"`, `"app_clip"`.
    #[serde(rename = "push_kind", skip_serializing_if = "Option::is_none")]
    pub push_kind: Option<String>,
}

/// `PatchUserDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PatchUserDto {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "deltas", skip_serializing_if = "Option::is_none")]
    pub deltas: Option<PatchUserDtoDeltas>,
}

/// `PatchUserDtoDeltas` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PatchUserDtoDeltas {
    #[serde(rename = "session_count", skip_serializing_if = "Option::is_none")]
    pub session_count: Option<i64>,
    #[serde(rename = "session_time", skip_serializing_if = "Option::is_none")]
    pub session_time: Option<i64>,
    #[serde(rename = "purchases", skip_serializing_if = "Option::is_none")]
    pub purchases: Option<Vec<PatchUserDtoDeltasPurchasesItem>>,
}

/// `PatchUserDtoDeltasPurchasesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PatchUserDtoDeltasPurchasesItem {
    /// Zorunlu.
    #[serde(rename = "sku", deserialize_with = "crate::de::null_default")]
    pub sku: String,
    /// Zorunlu.
    #[serde(rename = "amount", deserialize_with = "crate::de::null_default")]
    pub amount: f64,
    #[serde(rename = "iso", skip_serializing_if = "Option::is_none")]
    pub iso: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// `PathBodyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathBodyDto {
    /// Zorunlu.
    #[serde(rename = "event", deserialize_with = "crate::de::null_default")]
    pub event: String,
    /// Değerler: `"after"`, `"before"`.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    #[serde(rename = "top", skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    #[serde(rename = "window_hours", skip_serializing_if = "Option::is_none")]
    pub window_hours: Option<i64>,
    #[serde(rename = "exclude", skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// `PathReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "event", deserialize_with = "crate::de::null_default")]
    pub event: String,
    /// Zorunlu. Değerler: `"after"`, `"before"`.
    #[serde(rename = "direction", deserialize_with = "crate::de::null_default")]
    pub direction: String,
    /// Zorunlu.
    #[serde(rename = "window_hours", deserialize_with = "crate::de::null_default")]
    pub window_hours: i64,
    /// Zorunlu.
    #[serde(rename = "total_users", deserialize_with = "crate::de::null_default")]
    pub total_users: i64,
    /// Zorunlu.
    #[serde(rename = "truncated", deserialize_with = "crate::de::null_default")]
    pub truncated: bool,
    /// Zorunlu.
    #[serde(rename = "levels", deserialize_with = "crate::de::null_default")]
    pub levels: Vec<PathReportLevelsItem>,
    /// Zorunlu.
    #[serde(rename = "links", deserialize_with = "crate::de::null_default")]
    pub links: Vec<PathReportLinksItem>,
    /// Zorunlu.
    #[serde(rename = "top_paths", deserialize_with = "crate::de::null_default")]
    pub top_paths: Vec<PathReportTopPathsItem>,
}

/// `PathReportLevelsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathReportLevelsItem {
    /// Zorunlu.
    #[serde(rename = "step", deserialize_with = "crate::de::null_default")]
    pub step: i64,
    /// Zorunlu.
    #[serde(rename = "nodes", deserialize_with = "crate::de::null_default")]
    pub nodes: Vec<PathReportLevelsItemNodesItem>,
}

/// `PathReportLevelsItemNodesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathReportLevelsItemNodesItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Zorunlu. Değerler: `"event"`, `"other"`, `"end"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "share", deserialize_with = "crate::de::null_default")]
    pub share: f64,
}

/// `PathReportLinksItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathReportLinksItem {
    /// Zorunlu.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: String,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
}

/// `PathReportTopPathsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathReportTopPathsItem {
    /// Zorunlu.
    #[serde(rename = "path", deserialize_with = "crate::de::null_default")]
    pub path: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "share", deserialize_with = "crate::de::null_default")]
    pub share: f64,
}

/// `PostmasterStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PostmasterStats {
    /// Zorunlu.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<PostmasterStatsDataItem>,
}

/// `PostmasterStatsDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PostmasterStatsDataItem {
    /// Zorunlu.
    #[serde(rename = "day", deserialize_with = "crate::de::null_default")]
    pub day: String,
    /// Zorunlu.
    #[serde(rename = "domain_reputation")]
    pub domain_reputation: Option<String>,
    /// Zorunlu.
    #[serde(rename = "spam_rate")]
    pub spam_rate: Option<String>,
}

/// `PredictionModel` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PredictionModel {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"churn"`, `"event"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu.
    #[serde(rename = "event_names", deserialize_with = "crate::de::null_default")]
    pub event_names: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "horizon_days", deserialize_with = "crate::de::null_default")]
    pub horizon_days: i64,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu. Değerler: `"idle"`, `"queued"`, `"running"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu. Değerler: `"ready"`, `"insufficient_data"`, `"low_quality"`.
    #[serde(rename = "outcome")]
    pub outcome: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_computed_at")]
    pub last_computed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "scored_users", deserialize_with = "crate::de::null_default")]
    pub scored_users: i64,
    /// Zorunlu.
    #[serde(rename = "metrics")]
    pub metrics: Option<PredictionModelMetrics>,
    /// Zorunlu.
    #[serde(rename = "distribution", deserialize_with = "crate::de::null_default")]
    pub distribution: Vec<PredictionModelDistributionItem>,
}

/// `PredictionModelDistributionItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PredictionModelDistributionItem {
    /// Zorunlu. Değerler: `"low"`, `"medium"`, `"high"`.
    #[serde(rename = "bucket", deserialize_with = "crate::de::null_default")]
    pub bucket: String,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
}

/// `PredictionModelList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PredictionModelList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<PredictionModel>,
}

/// `PredictionModelMetrics` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PredictionModelMetrics {
    /// Zorunlu.
    #[serde(rename = "auc")]
    pub auc: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "base_rate")]
    pub base_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "train_users", deserialize_with = "crate::de::null_default")]
    pub train_users: i64,
    /// Zorunlu.
    #[serde(rename = "holdout_users", deserialize_with = "crate::de::null_default")]
    pub holdout_users: i64,
    /// Zorunlu.
    #[serde(rename = "positives", deserialize_with = "crate::de::null_default")]
    pub positives: i64,
    /// Zorunlu.
    #[serde(rename = "negatives", deserialize_with = "crate::de::null_default")]
    pub negatives: i64,
    /// Zorunlu.
    #[serde(rename = "calibration", deserialize_with = "crate::de::null_default")]
    pub calibration: Vec<PredictionModelMetricsCalibrationItem>,
    /// Zorunlu.
    #[serde(rename = "importance", deserialize_with = "crate::de::null_default")]
    pub importance: Vec<PredictionModelMetricsImportanceItem>,
}

/// `PredictionModelMetricsCalibrationItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PredictionModelMetricsCalibrationItem {
    /// Zorunlu. Değerler: `"low"`, `"medium"`, `"high"`.
    #[serde(rename = "bucket", deserialize_with = "crate::de::null_default")]
    pub bucket: String,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "predicted")]
    pub predicted: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "actual")]
    pub actual: Option<f64>,
}

/// `PredictionModelMetricsImportanceItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PredictionModelMetricsImportanceItem {
    /// Zorunlu. Değerler: `"recency_days"`, `"events_7"`, `"events_30"`, `"active_days_30"`, `"target_30"`, `"days_since_target"`, `"tenure_days"`, `"sent_30"`, `"engaged_30"`.
    #[serde(rename = "feature", deserialize_with = "crate::de::null_default")]
    pub feature: String,
    /// Zorunlu.
    #[serde(rename = "weight", deserialize_with = "crate::de::null_default")]
    pub weight: f64,
}

/// `PredictionUsersPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PredictionUsersPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<PredictionUsersPageDataItem>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
}

/// `PredictionUsersPageDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PredictionUsersPageDataItem {
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "score", deserialize_with = "crate::de::null_default")]
    pub score: i64,
    /// Zorunlu. Değerler: `"low"`, `"medium"`, `"high"`.
    #[serde(rename = "bucket", deserialize_with = "crate::de::null_default")]
    pub bucket: String,
}

/// `PromoCodeList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PromoCodeList {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "total_count", deserialize_with = "crate::de::null_default")]
    pub total_count: i64,
    /// Zorunlu.
    #[serde(rename = "assigned_count", deserialize_with = "crate::de::null_default")]
    pub assigned_count: i64,
    /// Zorunlu.
    #[serde(rename = "available_count", deserialize_with = "crate::de::null_default")]
    pub available_count: i64,
    /// Zorunlu.
    #[serde(rename = "liquid", deserialize_with = "crate::de::null_default")]
    pub liquid: String,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `PromoCodeListList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PromoCodeListList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<PromoCodeList>,
}

/// `PromoCodesPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PromoCodesPage {
    /// Zorunlu.
    #[serde(rename = "total", deserialize_with = "crate::de::null_default")]
    pub total: i64,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<PromoCodesPageDataItem>,
}

/// `PromoCodesPageDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PromoCodesPageDataItem {
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    /// Zorunlu.
    #[serde(rename = "delivery_id")]
    pub delivery_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "assigned_at")]
    pub assigned_at: Option<String>,
}

/// `PromoCodeUploadResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PromoCodeUploadResult {
    /// Zorunlu.
    #[serde(rename = "list", deserialize_with = "crate::de::null_default")]
    pub list: PromoCodeList,
    /// Zorunlu.
    #[serde(rename = "added", deserialize_with = "crate::de::null_default")]
    pub added: i64,
    /// Zorunlu.
    #[serde(rename = "skipped", deserialize_with = "crate::de::null_default")]
    pub skipped: i64,
}

/// `ProvidersReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvidersReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: ProvidersReportTotals,
    /// Zorunlu.
    #[serde(rename = "previous")]
    pub previous: Option<ProvidersReportPrevious>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<ProvidersReportRowsItem>,
}

/// `ProvidersReportPrevious` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvidersReportPrevious {
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "failure_rate")]
    pub failure_rate: Option<f64>,
}

/// `ProvidersReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvidersReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "failure_rate")]
    pub failure_rate: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "latency_p50_ms")]
    pub latency_p50_ms: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "latency_p95_ms")]
    pub latency_p95_ms: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "errors", deserialize_with = "crate::de::null_default")]
    pub errors: Vec<ProvidersReportRowsItemErrorsItem>,
}

/// `ProvidersReportRowsItemErrorsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvidersReportRowsItemErrorsItem {
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    /// Zorunlu.
    #[serde(rename = "count", deserialize_with = "crate::de::null_default")]
    pub count: i64,
}

/// `ProvidersReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProvidersReportTotals {
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "failed", deserialize_with = "crate::de::null_default")]
    pub failed: i64,
    /// Zorunlu.
    #[serde(rename = "failure_rate")]
    pub failure_rate: Option<f64>,
}

/// `PublicFormSubmitDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PublicFormSubmitDto {
    /// Zorunlu.
    #[serde(rename = "values", deserialize_with = "crate::de::null_default")]
    pub values: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "page_url", skip_serializing_if = "Option::is_none")]
    pub page_url: Option<String>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// `PublicSurveyResponseDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PublicSurveyResponseDto {
    /// Zorunlu.
    #[serde(rename = "answers", deserialize_with = "crate::de::null_default")]
    pub answers: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

/// `RecommendationConfig` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RecommendationConfig {
    /// Zorunlu.
    #[serde(rename = "configured", deserialize_with = "crate::de::null_default")]
    pub configured: bool,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "view_events", deserialize_with = "crate::de::null_default")]
    pub view_events: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "purchase_events", deserialize_with = "crate::de::null_default")]
    pub purchase_events: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "item_property", deserialize_with = "crate::de::null_default")]
    pub item_property: String,
    /// Zorunlu.
    #[serde(rename = "window_days", deserialize_with = "crate::de::null_default")]
    pub window_days: i64,
    /// Zorunlu. Değerler: `"idle"`, `"queued"`, `"running"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_computed_at")]
    pub last_computed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "scored_items", deserialize_with = "crate::de::null_default")]
    pub scored_items: i64,
    /// Zorunlu.
    #[serde(rename = "scored_users", deserialize_with = "crate::de::null_default")]
    pub scored_users: i64,
}

/// `Recommendations` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Recommendations {
    /// Zorunlu. Değerler: `"personalized"`, `"popular"`, `"trending"`, `"similar"`, `"category"`, `"recently_viewed"`, `"new_arrivals"`.
    #[serde(rename = "strategy", deserialize_with = "crate::de::null_default")]
    pub strategy: String,
    /// Zorunlu. Değerler: `"personalized"`, `"popular"`, `"trending"`, `"similar"`, `"category"`, `"recently_viewed"`, `"new_arrivals"`.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<CatalogItem>,
}

/// `RenderPreview` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderPreview {
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    /// Zorunlu.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Zorunlu.
    #[serde(rename = "subtitle")]
    pub subtitle: Option<String>,
    /// Zorunlu.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Zorunlu.
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    /// Zorunlu.
    #[serde(rename = "buttons", deserialize_with = "crate::de::null_default")]
    pub buttons: Vec<RenderPreviewButtonsItem>,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "warnings", deserialize_with = "crate::de::null_default")]
    pub warnings: Vec<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<RenderPreviewEmail>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<RenderPreviewSms>,
}

/// `RenderPreviewButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderPreviewButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    /// Zorunlu.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

/// `RenderPreviewEmail` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderPreviewEmail {
    /// Zorunlu.
    #[serde(rename = "subject", deserialize_with = "crate::de::null_default")]
    pub subject: String,
    /// Zorunlu.
    #[serde(rename = "preheader")]
    pub preheader: Option<String>,
    /// Zorunlu.
    #[serde(rename = "html", deserialize_with = "crate::de::null_default")]
    pub html: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
}

/// `RenderPreviewSms` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderPreviewSms {
    /// Zorunlu.
    #[serde(rename = "body", deserialize_with = "crate::de::null_default")]
    pub body: String,
    /// Zorunlu. Değerler: `"gsm7"`, `"gsm7_tr"`, `"ucs2"`.
    #[serde(rename = "encoding", deserialize_with = "crate::de::null_default")]
    pub encoding: String,
    /// Zorunlu.
    #[serde(rename = "characters", deserialize_with = "crate::de::null_default")]
    pub characters: i64,
    /// Zorunlu.
    #[serde(rename = "segments", deserialize_with = "crate::de::null_default")]
    pub segments: i64,
    /// Zorunlu.
    #[serde(rename = "per_segment", deserialize_with = "crate::de::null_default")]
    pub per_segment: i64,
    /// Zorunlu.
    #[serde(rename = "non_gsm_characters", deserialize_with = "crate::de::null_default")]
    pub non_gsm_characters: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "turkish_characters", deserialize_with = "crate::de::null_default")]
    pub turkish_characters: Vec<String>,
}

/// `RenderTemplateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDto {
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<RenderTemplateDtoUser>,
    #[serde(rename = "custom_data", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<RenderTemplateDtoContent>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// `RenderTemplateDtoContent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContent {
    #[serde(rename = "headings", skip_serializing_if = "Option::is_none")]
    pub headings: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "app_url", skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<RenderTemplateDtoContentButtonsItem>>,
    #[serde(rename = "big_picture", skip_serializing_if = "Option::is_none")]
    pub big_picture: Option<String>,
    #[serde(rename = "ios_attachments", skip_serializing_if = "Option::is_none")]
    pub ios_attachments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ios_sound", skip_serializing_if = "Option::is_none")]
    pub ios_sound: Option<String>,
    /// Değerler: `"None"`, `"SetTo"`, `"Increase"`.
    #[serde(rename = "ios_badge_type", skip_serializing_if = "Option::is_none")]
    pub ios_badge_type: Option<String>,
    #[serde(rename = "ios_badge_count", skip_serializing_if = "Option::is_none")]
    pub ios_badge_count: Option<i64>,
    /// Değerler: `"passive"`, `"active"`, `"time-sensitive"`, `"critical"`.
    #[serde(rename = "ios_interruption_level", skip_serializing_if = "Option::is_none")]
    pub ios_interruption_level: Option<String>,
    #[serde(rename = "ios_category", skip_serializing_if = "Option::is_none")]
    pub ios_category: Option<String>,
    #[serde(rename = "android_channel_id", skip_serializing_if = "Option::is_none")]
    pub android_channel_id: Option<String>,
    #[serde(rename = "android_group", skip_serializing_if = "Option::is_none")]
    pub android_group: Option<String>,
    #[serde(rename = "android_sound", skip_serializing_if = "Option::is_none")]
    pub android_sound: Option<String>,
    #[serde(rename = "chrome_web_icon", skip_serializing_if = "Option::is_none")]
    pub chrome_web_icon: Option<String>,
    #[serde(rename = "chrome_web_image", skip_serializing_if = "Option::is_none")]
    pub chrome_web_image: Option<String>,
    #[serde(rename = "chrome_web_badge", skip_serializing_if = "Option::is_none")]
    pub chrome_web_badge: Option<String>,
    #[serde(rename = "web_push_topic", skip_serializing_if = "Option::is_none")]
    pub web_push_topic: Option<String>,
    #[serde(rename = "content_available", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<bool>,
    #[serde(rename = "mutable_content", skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<bool>,
    #[serde(rename = "custom_data", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "email_subject", skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[serde(rename = "email_body", skip_serializing_if = "Option::is_none")]
    pub email_body: Option<String>,
    #[serde(rename = "email_mjml", skip_serializing_if = "Option::is_none")]
    pub email_mjml: Option<String>,
    #[serde(rename = "email_preheader", skip_serializing_if = "Option::is_none")]
    pub email_preheader: Option<String>,
    #[serde(rename = "email_from_name", skip_serializing_if = "Option::is_none")]
    pub email_from_name: Option<String>,
    #[serde(rename = "email_from_address", skip_serializing_if = "Option::is_none")]
    pub email_from_address: Option<String>,
    #[serde(rename = "email_reply_to_address", skip_serializing_if = "Option::is_none")]
    pub email_reply_to_address: Option<String>,
    #[serde(rename = "disable_email_click_tracking", skip_serializing_if = "Option::is_none")]
    pub disable_email_click_tracking: Option<bool>,
    #[serde(rename = "sms_from", skip_serializing_if = "Option::is_none")]
    pub sms_from: Option<String>,
    #[serde(rename = "disable_sms_link_shortening", skip_serializing_if = "Option::is_none")]
    pub disable_sms_link_shortening: Option<bool>,
    #[serde(rename = "add_to_inbox", skip_serializing_if = "Option::is_none")]
    pub add_to_inbox: Option<bool>,
    #[serde(rename = "inbox_ttl", skip_serializing_if = "Option::is_none")]
    pub inbox_ttl: Option<i64>,
    #[serde(rename = "content_card", skip_serializing_if = "Option::is_none")]
    pub content_card: Option<RenderTemplateDtoContentContentCard>,
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(rename = "ios_relevance_score", skip_serializing_if = "Option::is_none")]
    pub ios_relevance_score: Option<f64>,
    #[serde(rename = "target_content_identifier", skip_serializing_if = "Option::is_none")]
    pub target_content_identifier: Option<String>,
    #[serde(rename = "small_icon", skip_serializing_if = "Option::is_none")]
    pub small_icon: Option<String>,
    #[serde(rename = "large_icon", skip_serializing_if = "Option::is_none")]
    pub large_icon: Option<String>,
    #[serde(rename = "android_accent_color", skip_serializing_if = "Option::is_none")]
    pub android_accent_color: Option<String>,
    /// Değerler: `1`, `0`, `-1`.
    #[serde(rename = "android_visibility", skip_serializing_if = "Option::is_none")]
    pub android_visibility: Option<i64>,
    #[serde(rename = "web_buttons", skip_serializing_if = "Option::is_none")]
    pub web_buttons: Option<Vec<RenderTemplateDtoContentWebButtonsItem>>,
    #[serde(rename = "require_interaction", skip_serializing_if = "Option::is_none")]
    pub require_interaction: Option<bool>,
    #[serde(rename = "sms_media_urls", skip_serializing_if = "Option::is_none")]
    pub sms_media_urls: Option<Vec<String>>,
    #[serde(rename = "whatsapp_template", skip_serializing_if = "Option::is_none")]
    pub whatsapp_template: Option<RenderTemplateDtoContentWhatsappTemplate>,
    #[serde(rename = "whatsapp_media", skip_serializing_if = "Option::is_none")]
    pub whatsapp_media: Option<RenderTemplateDtoContentWhatsappMedia>,
    #[serde(rename = "whatsapp_interactive", skip_serializing_if = "Option::is_none")]
    pub whatsapp_interactive: Option<RenderTemplateDtoContentWhatsappInteractive>,
    #[serde(rename = "rcs_card", skip_serializing_if = "Option::is_none")]
    pub rcs_card: Option<RenderTemplateDtoContentRcsCard>,
    #[serde(rename = "rcs_suggestions", skip_serializing_if = "Option::is_none")]
    pub rcs_suggestions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "rcs_content_sid", skip_serializing_if = "Option::is_none")]
    pub rcs_content_sid: Option<String>,
    #[serde(rename = "rcs_content_variables", skip_serializing_if = "Option::is_none")]
    pub rcs_content_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rcs_fallback_text", skip_serializing_if = "Option::is_none")]
    pub rcs_fallback_text: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "messenger_image_url", skip_serializing_if = "Option::is_none")]
    pub messenger_image_url: Option<String>,
    #[serde(rename = "messenger_buttons", skip_serializing_if = "Option::is_none")]
    pub messenger_buttons: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ios_carousel", skip_serializing_if = "Option::is_none")]
    pub ios_carousel: Option<Vec<RenderTemplateDtoContentIosCarouselItem>>,
    #[serde(rename = "android_live_update", skip_serializing_if = "Option::is_none")]
    pub android_live_update: Option<RenderTemplateDtoContentAndroidLiveUpdate>,
    #[serde(rename = "voip", skip_serializing_if = "Option::is_none")]
    pub voip: Option<bool>,
    #[serde(rename = "app_clip", skip_serializing_if = "Option::is_none")]
    pub app_clip: Option<bool>,
}

/// `RenderTemplateDtoContentAndroidLiveUpdate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentAndroidLiveUpdate {
    /// Zorunlu.
    #[serde(rename = "progress", deserialize_with = "crate::de::null_default")]
    pub progress: i64,
    #[serde(rename = "ongoing", skip_serializing_if = "Option::is_none")]
    pub ongoing: Option<bool>,
}

/// `RenderTemplateDtoContentButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `RenderTemplateDtoContentContentCard` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentContentCard {
    /// Zorunlu. Değerler: `"classic"`, `"captioned_image"`, `"image_only"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "pinned", skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "button_text", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<std::collections::HashMap<String, String>>,
}

/// `RenderTemplateDtoContentIosCarouselItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentIosCarouselItem {
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `RenderTemplateDtoContentRcsCard` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentRcsCard {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "media_url", skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// Değerler: `"SHORT"`, `"MEDIUM"`, `"TALL"`.
    #[serde(rename = "media_height", skip_serializing_if = "Option::is_none")]
    pub media_height: Option<String>,
    /// Değerler: `"VERTICAL"`, `"HORIZONTAL"`.
    #[serde(rename = "orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
}

/// `RenderTemplateDtoContentWebButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWebButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `RenderTemplateDtoContentWhatsappInteractive` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWhatsappInteractive {
    /// Zorunlu. Değerler: `"button"`, `"list"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<RenderTemplateDtoContentWhatsappInteractiveButtonsItem>>,
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<RenderTemplateDtoContentWhatsappInteractiveList>,
}

/// `RenderTemplateDtoContentWhatsappInteractiveButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWhatsappInteractiveButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
}

/// `RenderTemplateDtoContentWhatsappInteractiveList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWhatsappInteractiveList {
    /// Zorunlu.
    #[serde(rename = "button", deserialize_with = "crate::de::null_default")]
    pub button: String,
    /// Zorunlu.
    #[serde(rename = "sections", deserialize_with = "crate::de::null_default")]
    pub sections: Vec<RenderTemplateDtoContentWhatsappInteractiveListSectionsItem>,
}

/// `RenderTemplateDtoContentWhatsappInteractiveListSectionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWhatsappInteractiveListSectionsItem {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<RenderTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem>,
}

/// `RenderTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `RenderTemplateDtoContentWhatsappMedia` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWhatsappMedia {
    /// Zorunlu. Değerler: `"image"`, `"video"`, `"document"`, `"audio"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// `RenderTemplateDtoContentWhatsappTemplate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWhatsappTemplate {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<serde_json::Value>,
    #[serde(rename = "body_params", skip_serializing_if = "Option::is_none")]
    pub body_params: Option<Vec<String>>,
    #[serde(rename = "button_params", skip_serializing_if = "Option::is_none")]
    pub button_params: Option<Vec<RenderTemplateDtoContentWhatsappTemplateButtonParamsItem>>,
}

/// `RenderTemplateDtoContentWhatsappTemplateButtonParamsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoContentWhatsappTemplateButtonParamsItem {
    /// Zorunlu.
    #[serde(rename = "index", deserialize_with = "crate::de::null_default")]
    pub index: i64,
    /// Zorunlu. Değerler: `"url"`, `"quick_reply"`, `"copy_code"`.
    #[serde(rename = "sub_type", deserialize_with = "crate::de::null_default")]
    pub sub_type: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `RenderTemplateDtoUser` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderTemplateDtoUser {
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// `ReportResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReportResult {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "columns", deserialize_with = "crate::de::null_default")]
    pub columns: Vec<ReportResultColumnsItem>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "totals")]
    pub totals: Option<std::collections::HashMap<String, f64>>,
    /// Zorunlu.
    #[serde(rename = "truncated", deserialize_with = "crate::de::null_default")]
    pub truncated: bool,
}

/// `ReportResultColumnsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReportResultColumnsItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu. Değerler: `"dimension"`, `"metric"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu. Değerler: `"text"`, `"date"`, `"number"`, `"percent"`.
    #[serde(rename = "format", deserialize_with = "crate::de::null_default")]
    pub format: String,
}

/// `ReportRunBodyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReportRunBodyDto {
    /// Zorunlu. Değerler: `"deliveries"`, `"events"`, `"conversions"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu. Değerler: `"sent"`, `"delivered"`, `"failed"`, `"opened"`, `"clicked"`, `"delivery_rate"`, `"open_rate"`, `"click_rate"`, `"failure_rate"`, `"count"`, `"unique_users"`, `"value_sum"`, `"value_avg"`, `"conversions"`, `"revenue"`.
    #[serde(rename = "metrics", deserialize_with = "crate::de::null_default")]
    pub metrics: Vec<String>,
    /// Değerler: `"day"`, `"week"`, `"month"`, `"channel"`, `"provider"`, `"campaign"`, `"status"`, `"name"`, `"source"`, `"event_name"`, `"kind"`.
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<ReportRunBodyDtoSort>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// `ReportRunBodyDtoSort` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReportRunBodyDtoSort {
    /// Zorunlu.
    #[serde(rename = "by", deserialize_with = "crate::de::null_default")]
    pub by: String,
    /// Değerler: `"asc"`, `"desc"`.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// `RetentionReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RetentionReport {
    /// Zorunlu.
    #[serde(rename = "weeks", deserialize_with = "crate::de::null_default")]
    pub weeks: i64,
    /// Zorunlu.
    #[serde(rename = "cohorts", deserialize_with = "crate::de::null_default")]
    pub cohorts: Vec<RetentionReportCohortsItem>,
    /// Zorunlu.
    #[serde(rename = "average", deserialize_with = "crate::de::null_default")]
    pub average: Vec<f64>,
}

/// `RetentionReportCohortsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RetentionReportCohortsItem {
    /// Zorunlu.
    #[serde(rename = "week", deserialize_with = "crate::de::null_default")]
    pub week: String,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "retention", deserialize_with = "crate::de::null_default")]
    pub retention: Vec<f64>,
}

/// `ReverseEtlRun` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReverseEtlRun {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "sync_id", deserialize_with = "crate::de::null_default")]
    pub sync_id: String,
    /// Zorunlu. Değerler: `"schedule"`, `"manual"`.
    #[serde(rename = "trigger", deserialize_with = "crate::de::null_default")]
    pub trigger: String,
    /// Zorunlu. Değerler: `"queued"`, `"running"`, `"succeeded"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "row_count", deserialize_with = "crate::de::null_default")]
    pub row_count: i64,
    /// Zorunlu.
    #[serde(rename = "attempt", deserialize_with = "crate::de::null_default")]
    pub attempt: i64,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "started_at")]
    pub started_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "duration_ms")]
    pub duration_ms: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `ReverseEtlRunPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReverseEtlRunPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<ReverseEtlRun>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `ReverseEtlSync` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReverseEtlSync {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"snowflake"`, `"bigquery"`, `"postgres"`, `"mysql"`, `"s3"`, `"google_sheets"`, `"clickhouse"`, `"sqlserver"`, `"trino"`, `"athena"`, `"elasticsearch"`, `"kafka"`, `"pubsub"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu. Değerler: `"event"`, `"user"`.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: String,
    /// Zorunlu.
    #[serde(rename = "config", deserialize_with = "crate::de::null_default")]
    pub config: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "credentials_hint", deserialize_with = "crate::de::null_default")]
    pub credentials_hint: String,
    /// Zorunlu. Değerler: `"every_5_minutes"`, `"hourly"`, `"daily"`.
    #[serde(rename = "schedule", deserialize_with = "crate::de::null_default")]
    pub schedule: String,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu. Değerler: `"active"`, `"paused"`, `"failing"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "consecutive_failures", deserialize_with = "crate::de::null_default")]
    pub consecutive_failures: i64,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_run_at")]
    pub last_run_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_success_at")]
    pub last_success_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "next_run_at")]
    pub next_run_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `ReverseEtlSyncList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ReverseEtlSyncList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<ReverseEtlSync>,
}

/// `RfmConfig` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RfmConfig {
    /// Zorunlu.
    #[serde(rename = "configured", deserialize_with = "crate::de::null_default")]
    pub configured: bool,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "event_names", deserialize_with = "crate::de::null_default")]
    pub event_names: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "value_property")]
    pub value_property: Option<String>,
    /// Zorunlu.
    #[serde(rename = "use_monetary", deserialize_with = "crate::de::null_default")]
    pub use_monetary: bool,
    /// Zorunlu.
    #[serde(rename = "window_days", deserialize_with = "crate::de::null_default")]
    pub window_days: i64,
    /// Zorunlu. Değerler: `"idle"`, `"queued"`, `"running"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "first_computed_at")]
    pub first_computed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_computed_at")]
    pub last_computed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "scored_users", deserialize_with = "crate::de::null_default")]
    pub scored_users: i64,
}

/// `RfmOverview` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RfmOverview {
    /// Zorunlu.
    #[serde(rename = "total", deserialize_with = "crate::de::null_default")]
    pub total: i64,
    /// Zorunlu.
    #[serde(rename = "low_population", deserialize_with = "crate::de::null_default")]
    pub low_population: bool,
    /// Zorunlu.
    #[serde(rename = "last_computed_at")]
    pub last_computed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "segments", deserialize_with = "crate::de::null_default")]
    pub segments: Vec<RfmOverviewSegmentsItem>,
}

/// `RfmOverviewSegmentsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RfmOverviewSegmentsItem {
    /// Zorunlu. Değerler: `"champions"`, `"loyal"`, `"potential"`, `"new"`, `"promising"`, `"need_attention"`, `"about_to_sleep"`, `"at_risk"`, `"cant_lose"`, `"hibernating"`, `"lost"`.
    #[serde(rename = "segment", deserialize_with = "crate::de::null_default")]
    pub segment: String,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "share", deserialize_with = "crate::de::null_default")]
    pub share: f64,
    /// Zorunlu.
    #[serde(rename = "avg_recency_days")]
    pub avg_recency_days: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "avg_frequency")]
    pub avg_frequency: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "avg_monetary")]
    pub avg_monetary: Option<f64>,
}

/// `RfmTransitions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RfmTransitions {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "first_computed_at")]
    pub first_computed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "transitions", deserialize_with = "crate::de::null_default")]
    pub transitions: Vec<RfmTransitionsTransitionsItem>,
}

/// `RfmTransitionsTransitionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RfmTransitionsTransitionsItem {
    /// Zorunlu. Değerler: `"champions"`, `"loyal"`, `"potential"`, `"new"`, `"promising"`, `"need_attention"`, `"about_to_sleep"`, `"at_risk"`, `"cant_lose"`, `"hibernating"`, `"lost"`.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// Zorunlu. Değerler: `"champions"`, `"loyal"`, `"potential"`, `"new"`, `"promising"`, `"need_attention"`, `"about_to_sleep"`, `"at_risk"`, `"cant_lose"`, `"hibernating"`, `"lost"`.
    #[serde(rename = "to")]
    pub to: Option<String>,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
}

/// `RfmUsersPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RfmUsersPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<RfmUsersPageDataItem>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
}

/// `RfmUsersPageDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RfmUsersPageDataItem {
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu. Değerler: `"champions"`, `"loyal"`, `"potential"`, `"new"`, `"promising"`, `"need_attention"`, `"about_to_sleep"`, `"at_risk"`, `"cant_lose"`, `"hibernating"`, `"lost"`.
    #[serde(rename = "segment", deserialize_with = "crate::de::null_default")]
    pub segment: String,
    /// Zorunlu.
    #[serde(rename = "r", deserialize_with = "crate::de::null_default")]
    pub r: i64,
    /// Zorunlu.
    #[serde(rename = "f", deserialize_with = "crate::de::null_default")]
    pub f: i64,
    /// Zorunlu.
    #[serde(rename = "m")]
    pub m: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "recency_days", deserialize_with = "crate::de::null_default")]
    pub recency_days: i64,
    /// Zorunlu.
    #[serde(rename = "frequency", deserialize_with = "crate::de::null_default")]
    pub frequency: i64,
    /// Zorunlu.
    #[serde(rename = "monetary", deserialize_with = "crate::de::null_default")]
    pub monetary: f64,
    /// Zorunlu.
    #[serde(rename = "last_event_at", deserialize_with = "crate::de::null_default")]
    pub last_event_at: String,
    /// Zorunlu.
    #[serde(rename = "segment_since", deserialize_with = "crate::de::null_default")]
    pub segment_since: String,
}

/// `SavedReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SavedReport {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "definition", deserialize_with = "crate::de::null_default")]
    pub definition: SavedReportDefinition,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `SavedReportDefinition` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SavedReportDefinition {
    /// Zorunlu. Değerler: `"deliveries"`, `"events"`, `"conversions"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu. Değerler: `"sent"`, `"delivered"`, `"failed"`, `"opened"`, `"clicked"`, `"delivery_rate"`, `"open_rate"`, `"click_rate"`, `"failure_rate"`, `"count"`, `"unique_users"`, `"value_sum"`, `"value_avg"`, `"conversions"`, `"revenue"`.
    #[serde(rename = "metrics", deserialize_with = "crate::de::null_default")]
    pub metrics: Vec<String>,
    /// Zorunlu. Değerler: `"day"`, `"week"`, `"month"`, `"channel"`, `"provider"`, `"campaign"`, `"status"`, `"name"`, `"source"`, `"event_name"`, `"kind"`.
    #[serde(rename = "dimensions", deserialize_with = "crate::de::null_default")]
    pub dimensions: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "filters", deserialize_with = "crate::de::null_default")]
    pub filters: std::collections::HashMap<String, Vec<String>>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<SavedReportDefinitionSort>,
    /// Zorunlu.
    #[serde(rename = "limit", deserialize_with = "crate::de::null_default")]
    pub limit: i64,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// `SavedReportDefinitionSort` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SavedReportDefinitionSort {
    /// Zorunlu.
    #[serde(rename = "by", deserialize_with = "crate::de::null_default")]
    pub by: String,
    /// Zorunlu. Değerler: `"asc"`, `"desc"`.
    #[serde(rename = "direction", deserialize_with = "crate::de::null_default")]
    pub direction: String,
}

/// `SavedReportList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SavedReportList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<SavedReport>,
}

/// `ScheduleCampaignDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScheduleCampaignDto {
    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
    #[serde(rename = "recurrence", skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<ScheduleCampaignDtoRecurrence>,
}

/// `ScheduleCampaignDtoRecurrence` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ScheduleCampaignDtoRecurrence {
    /// Zorunlu.
    #[serde(rename = "cron", deserialize_with = "crate::de::null_default")]
    pub cron: String,
    #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// `Segment` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Segment {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "definition", deserialize_with = "crate::de::null_default")]
    pub definition: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "is_dynamic", deserialize_with = "crate::de::null_default")]
    pub is_dynamic: bool,
    /// Zorunlu.
    #[serde(rename = "estimated_count")]
    pub estimated_count: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "estimated_at")]
    pub estimated_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu.
    #[serde(rename = "sql")]
    pub sql: Option<SegmentSql>,
}

/// `SegmentCampaigns` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentCampaigns {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<SegmentCampaignsDataItem>,
}

/// `SegmentCampaignsDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentCampaignsDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu. Değerler: `"include"`, `"exclude"`.
    #[serde(rename = "usage", deserialize_with = "crate::de::null_default")]
    pub usage: String,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `SegmentEstimate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentEstimate {
    /// Zorunlu.
    #[serde(rename = "segment_id")]
    pub segment_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "subscriptions", deserialize_with = "crate::de::null_default")]
    pub subscriptions: i64,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "estimated_at", deserialize_with = "crate::de::null_default")]
    pub estimated_at: String,
    /// Zorunlu.
    #[serde(rename = "cached", deserialize_with = "crate::de::null_default")]
    pub cached: bool,
    /// Zorunlu.
    #[serde(rename = "sampled", deserialize_with = "crate::de::null_default")]
    pub sampled: bool,
}

/// `SegmentPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Segment>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `SegmentPreview` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentPreview {
    /// Zorunlu.
    #[serde(rename = "subscriptions", deserialize_with = "crate::de::null_default")]
    pub subscriptions: i64,
    /// Zorunlu.
    #[serde(rename = "users", deserialize_with = "crate::de::null_default")]
    pub users: i64,
    /// Zorunlu.
    #[serde(rename = "sampled", deserialize_with = "crate::de::null_default")]
    pub sampled: bool,
    /// Zorunlu.
    #[serde(rename = "sample", deserialize_with = "crate::de::null_default")]
    pub sample: Vec<SegmentPreviewSampleItem>,
}

/// `SegmentPreviewSampleItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentPreviewSampleItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "tags", deserialize_with = "crate::de::null_default")]
    pub tags: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "last_active_at", deserialize_with = "crate::de::null_default")]
    pub last_active_at: String,
}

/// `SegmentsCreateBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentsCreateBody {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Zorunlu. Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
    #[serde(rename = "definition", deserialize_with = "crate::de::null_default")]
    pub definition: SegmentsCreateBodyDefinition,
    #[serde(rename = "is_dynamic", skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
}

/// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentsCreateBodyDefinition {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// `SegmentsPreviewBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentsPreviewBody {
    /// Zorunlu. Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
    #[serde(rename = "definition", deserialize_with = "crate::de::null_default")]
    pub definition: SegmentsPreviewBodyDefinition,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentsPreviewBodyDefinition {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// `SegmentSql` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentSql {
    /// Zorunlu.
    #[serde(rename = "query", deserialize_with = "crate::de::null_default")]
    pub query: String,
    /// Zorunlu.
    #[serde(rename = "refresh_interval_minutes")]
    pub refresh_interval_minutes: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "refreshed_at")]
    pub refreshed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "member_count")]
    pub member_count: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
}

/// `SegmentsUpdateBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentsUpdateBody {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
    #[serde(rename = "definition", skip_serializing_if = "Option::is_none")]
    pub definition: Option<SegmentsUpdateBodyDefinition>,
    #[serde(rename = "is_dynamic", skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
}

/// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SegmentsUpdateBodyDefinition {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// `SelectWinnerDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SelectWinnerDto {
    /// Zorunlu.
    #[serde(rename = "variant_id", deserialize_with = "crate::de::null_default")]
    pub variant_id: String,
}

/// `SetOwnerDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SetOwnerDto {
    /// Zorunlu.
    #[serde(rename = "external_id", deserialize_with = "crate::de::null_default")]
    pub external_id: String,
}

/// `SetPreferencesDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SetPreferencesDto {
    /// Zorunlu.
    #[serde(rename = "preferences", deserialize_with = "crate::de::null_default")]
    pub preferences: Vec<SetPreferencesDtoPreferencesItem>,
}

/// `SetPreferencesDtoPreferencesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SetPreferencesDtoPreferencesItem {
    /// Zorunlu.
    #[serde(rename = "category", deserialize_with = "crate::de::null_default")]
    pub category: String,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Zorunlu.
    #[serde(rename = "opted_in", deserialize_with = "crate::de::null_default")]
    pub opted_in: bool,
}

/// `SmsKeyword` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SmsKeyword {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "keyword", deserialize_with = "crate::de::null_default")]
    pub keyword: String,
    /// Zorunlu. Değerler: `"org"`, `"app"`.
    #[serde(rename = "scope", deserialize_with = "crate::de::null_default")]
    pub scope: String,
    /// Zorunlu.
    #[serde(rename = "app_id")]
    pub app_id: Option<String>,
    /// Zorunlu. Değerler: `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", deserialize_with = "crate::de::null_default")]
    pub channels: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "actions", deserialize_with = "crate::de::null_default")]
    pub actions: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `SmsKeywordList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SmsKeywordList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<SmsKeyword>,
    /// Zorunlu.
    #[serde(rename = "reserved", deserialize_with = "crate::de::null_default")]
    pub reserved: Vec<String>,
}

/// `SmsVerification` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SmsVerification {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "expires_at", deserialize_with = "crate::de::null_default")]
    pub expires_at: String,
}

/// `SmsVerificationCheck` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SmsVerificationCheck {
    /// Zorunlu.
    #[serde(rename = "verified", deserialize_with = "crate::de::null_default")]
    pub verified: bool,
}

/// `StartLiveActivityDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StartLiveActivityDto {
    #[serde(rename = "activity_id", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "target", deserialize_with = "crate::de::null_default")]
    pub target: StartLiveActivityDtoTarget,
    /// Zorunlu.
    #[serde(rename = "content_state", deserialize_with = "crate::de::null_default")]
    pub content_state: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<StartLiveActivityDtoAlert>,
    #[serde(rename = "stale_date", skip_serializing_if = "Option::is_none")]
    pub stale_date: Option<i64>,
    #[serde(rename = "dismissal_date", skip_serializing_if = "Option::is_none")]
    pub dismissal_date: Option<i64>,
    #[serde(rename = "relevance_score", skip_serializing_if = "Option::is_none")]
    pub relevance_score: Option<f64>,
    /// Değerler: `5`, `10`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `StartLiveActivityDtoAlert` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StartLiveActivityDtoAlert {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "sound", skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
}

/// `StartLiveActivityDtoTarget` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StartLiveActivityDtoTarget {
    #[serde(rename = "include_external_ids", skip_serializing_if = "Option::is_none")]
    pub include_external_ids: Option<Vec<String>>,
    #[serde(rename = "include_subscription_ids", skip_serializing_if = "Option::is_none")]
    pub include_subscription_ids: Option<Vec<String>>,
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

/// `StartVerifyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StartVerifyDto {
    /// Zorunlu.
    #[serde(rename = "phone", deserialize_with = "crate::de::null_default")]
    pub phone: String,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// `Subscription` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Subscription {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "token", deserialize_with = "crate::de::null_default")]
    pub token: String,
    /// Zorunlu. Değerler: `"subscribed"`, `"unsubscribed"`, `"invalid"`, `"pending"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu. Değerler: `"provider_rejected"`, `"user_opt_out"`, `"api"`, `"uninstall"`.
    #[serde(rename = "unsubscribe_reason")]
    pub unsubscribe_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "device", deserialize_with = "crate::de::null_default")]
    pub device: SubscriptionDevice,
    /// Zorunlu.
    #[serde(rename = "test_device", deserialize_with = "crate::de::null_default")]
    pub test_device: bool,
    /// Zorunlu.
    #[serde(rename = "test_device_name")]
    pub test_device_name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_active_at", deserialize_with = "crate::de::null_default")]
    pub last_active_at: String,
    /// Zorunlu.
    #[serde(rename = "last_sent_at")]
    pub last_sent_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_delivered_at")]
    pub last_delivered_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "opted_in_at")]
    pub opted_in_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "opt_in_source")]
    pub opt_in_source: Option<String>,
    /// Zorunlu.
    #[serde(rename = "unsubscribed_at")]
    pub unsubscribed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "location")]
    pub location: Option<SubscriptionLocation>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `SubscriptionDevice` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubscriptionDevice {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "app_version", skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "sdk_version", skip_serializing_if = "Option::is_none")]
    pub sdk_version: Option<String>,
    #[serde(rename = "sdk_platform", skip_serializing_if = "Option::is_none")]
    pub sdk_platform: Option<String>,
    #[serde(rename = "carrier", skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "push_capable", skip_serializing_if = "Option::is_none")]
    pub push_capable: Option<bool>,
    /// Değerler: `"sandbox"`, `"production"`.
    #[serde(rename = "push_environment", skip_serializing_if = "Option::is_none")]
    pub push_environment: Option<String>,
    /// Değerler: `"phone"`, `"watch"`.
    #[serde(rename = "form_factor", skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
    /// Değerler: `"alert"`, `"voip"`, `"app_clip"`.
    #[serde(rename = "push_kind", skip_serializing_if = "Option::is_none")]
    pub push_kind: Option<String>,
}

/// `SubscriptionLocation` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubscriptionLocation {
    /// Zorunlu.
    #[serde(rename = "lat", deserialize_with = "crate::de::null_default")]
    pub lat: f64,
    /// Zorunlu.
    #[serde(rename = "lng", deserialize_with = "crate::de::null_default")]
    pub lng: f64,
    /// Zorunlu.
    #[serde(rename = "accuracy_m")]
    pub accuracy_m: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "geohash")]
    pub geohash: Option<String>,
    /// Zorunlu.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "consent_at")]
    pub consent_at: Option<String>,
}

/// `SubscriptionsReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubscriptionsReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: SubscriptionsReportTotals,
    /// Zorunlu.
    #[serde(rename = "previous")]
    pub previous: Option<SubscriptionsReportPrevious>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<SubscriptionsReportRowsItem>,
    /// Zorunlu.
    #[serde(rename = "by_channel", deserialize_with = "crate::de::null_default")]
    pub by_channel: Vec<SubscriptionsReportByChannelItem>,
}

/// `SubscriptionsReportByChannelItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubscriptionsReportByChannelItem {
    /// Zorunlu.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
    /// Zorunlu.
    #[serde(rename = "new", deserialize_with = "crate::de::null_default")]
    pub new: i64,
    /// Zorunlu.
    #[serde(rename = "unsubscribed", deserialize_with = "crate::de::null_default")]
    pub unsubscribed: i64,
}

/// `SubscriptionsReportPrevious` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubscriptionsReportPrevious {
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
    /// Zorunlu.
    #[serde(rename = "new", deserialize_with = "crate::de::null_default")]
    pub new: i64,
    /// Zorunlu.
    #[serde(rename = "unsubscribed", deserialize_with = "crate::de::null_default")]
    pub unsubscribed: i64,
    /// Zorunlu.
    #[serde(rename = "opt_outs", deserialize_with = "crate::de::null_default")]
    pub opt_outs: i64,
    /// Zorunlu.
    #[serde(rename = "net", deserialize_with = "crate::de::null_default")]
    pub net: i64,
}

/// `SubscriptionsReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubscriptionsReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "new", deserialize_with = "crate::de::null_default")]
    pub new: i64,
    /// Zorunlu.
    #[serde(rename = "unsubscribed", deserialize_with = "crate::de::null_default")]
    pub unsubscribed: i64,
    /// Zorunlu.
    #[serde(rename = "opt_outs", deserialize_with = "crate::de::null_default")]
    pub opt_outs: i64,
    /// Zorunlu.
    #[serde(rename = "net", deserialize_with = "crate::de::null_default")]
    pub net: i64,
}

/// `SubscriptionsReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubscriptionsReportTotals {
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
    /// Zorunlu.
    #[serde(rename = "new", deserialize_with = "crate::de::null_default")]
    pub new: i64,
    /// Zorunlu.
    #[serde(rename = "unsubscribed", deserialize_with = "crate::de::null_default")]
    pub unsubscribed: i64,
    /// Zorunlu.
    #[serde(rename = "opt_outs", deserialize_with = "crate::de::null_default")]
    pub opt_outs: i64,
    /// Zorunlu.
    #[serde(rename = "net", deserialize_with = "crate::de::null_default")]
    pub net: i64,
}

/// `Suppression` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Suppression {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu. Değerler: `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "value_masked", deserialize_with = "crate::de::null_default")]
    pub value_masked: String,
    /// Zorunlu. Değerler: `"bounce"`, `"complaint"`, `"manual"`, `"unsubscribe_link"`, `"stop"`.
    #[serde(rename = "reason", deserialize_with = "crate::de::null_default")]
    pub reason: String,
    /// Zorunlu.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu.
    #[serde(rename = "detail", deserialize_with = "crate::de::null_default")]
    pub detail: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `SuppressionPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SuppressionPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Suppression>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `Survey` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Survey {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"closed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "questions", deserialize_with = "crate::de::null_default")]
    pub questions: Vec<SurveyQuestionsItem>,
    /// Zorunlu.
    #[serde(rename = "settings", deserialize_with = "crate::de::null_default")]
    pub settings: SurveySettings,
    /// Zorunlu.
    #[serde(rename = "in_app_message_id")]
    pub in_app_message_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: SurveyStats,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `SurveyPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Survey>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `SurveyQuestionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyQuestionsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"nps"`, `"csat"`, `"single_choice"`, `"multiple_choice"`, `"text"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: std::collections::HashMap<String, String>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SurveyQuestionsItemOptionsItem>>,
    #[serde(rename = "low_label", skip_serializing_if = "Option::is_none")]
    pub low_label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "high_label", skip_serializing_if = "Option::is_none")]
    pub high_label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<std::collections::HashMap<String, String>>,
}

/// `SurveyQuestionsItemOptionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyQuestionsItemOptionsItem {
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: std::collections::HashMap<String, String>,
}

/// `SurveyReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyReport {
    /// Zorunlu.
    #[serde(rename = "survey_id", deserialize_with = "crate::de::null_default")]
    pub survey_id: String,
    /// Zorunlu.
    #[serde(rename = "days", deserialize_with = "crate::de::null_default")]
    pub days: i64,
    /// Zorunlu.
    #[serde(rename = "responses", deserialize_with = "crate::de::null_default")]
    pub responses: i64,
    /// Zorunlu.
    #[serde(rename = "nps", deserialize_with = "crate::de::null_default")]
    pub nps: SurveyReportNps,
    /// Zorunlu.
    #[serde(rename = "csat", deserialize_with = "crate::de::null_default")]
    pub csat: SurveyReportCsat,
    /// Zorunlu.
    #[serde(rename = "trend", deserialize_with = "crate::de::null_default")]
    pub trend: Vec<SurveyReportTrendItem>,
    /// Zorunlu.
    #[serde(rename = "channels", deserialize_with = "crate::de::null_default")]
    pub channels: std::collections::HashMap<String, i64>,
    /// Zorunlu.
    #[serde(rename = "questions", deserialize_with = "crate::de::null_default")]
    pub questions: Vec<SurveyReportQuestionsItem>,
}

/// `SurveyReportCsat` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyReportCsat {
    /// Zorunlu.
    #[serde(rename = "responses", deserialize_with = "crate::de::null_default")]
    pub responses: i64,
    /// Zorunlu.
    #[serde(rename = "average")]
    pub average: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "distribution", deserialize_with = "crate::de::null_default")]
    pub distribution: std::collections::HashMap<String, i64>,
}

/// `SurveyReportNps` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyReportNps {
    /// Zorunlu.
    #[serde(rename = "responses", deserialize_with = "crate::de::null_default")]
    pub responses: i64,
    /// Zorunlu.
    #[serde(rename = "promoters", deserialize_with = "crate::de::null_default")]
    pub promoters: i64,
    /// Zorunlu.
    #[serde(rename = "passives", deserialize_with = "crate::de::null_default")]
    pub passives: i64,
    /// Zorunlu.
    #[serde(rename = "detractors", deserialize_with = "crate::de::null_default")]
    pub detractors: i64,
    /// Zorunlu.
    #[serde(rename = "nps")]
    pub nps: Option<f64>,
}

/// `SurveyReportQuestionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyReportQuestionsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu. Değerler: `"nps"`, `"csat"`, `"single_choice"`, `"multiple_choice"`, `"text"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "counts", deserialize_with = "crate::de::null_default")]
    pub counts: std::collections::HashMap<String, i64>,
}

/// `SurveyReportTrendItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyReportTrendItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "responses", deserialize_with = "crate::de::null_default")]
    pub responses: i64,
    /// Zorunlu.
    #[serde(rename = "promoters", deserialize_with = "crate::de::null_default")]
    pub promoters: i64,
    /// Zorunlu.
    #[serde(rename = "passives", deserialize_with = "crate::de::null_default")]
    pub passives: i64,
    /// Zorunlu.
    #[serde(rename = "detractors", deserialize_with = "crate::de::null_default")]
    pub detractors: i64,
    /// Zorunlu.
    #[serde(rename = "nps")]
    pub nps: Option<f64>,
}

/// `SurveyResponse` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyResponse {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "survey_id", deserialize_with = "crate::de::null_default")]
    pub survey_id: String,
    /// Zorunlu.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "visitor_id")]
    pub visitor_id: Option<String>,
    /// Zorunlu. Değerler: `"in_app"`, `"web"`, `"email"`, `"api"`, `"sdk"`.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "answers", deserialize_with = "crate::de::null_default")]
    pub answers: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "nps_score")]
    pub nps_score: Option<i64>,
    /// Zorunlu. Değerler: `"promoter"`, `"passive"`, `"detractor"`.
    #[serde(rename = "nps_category")]
    pub nps_category: Option<String>,
    /// Zorunlu.
    #[serde(rename = "csat_score")]
    pub csat_score: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `SurveyResponsePage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyResponsePage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<SurveyResponse>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `SurveyResponseResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyResponseResult {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
    /// Zorunlu.
    #[serde(rename = "response_id")]
    pub response_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "duplicate", deserialize_with = "crate::de::null_default")]
    pub duplicate: bool,
    /// Zorunlu.
    #[serde(rename = "thank_you", deserialize_with = "crate::de::null_default")]
    pub thank_you: String,
}

/// `SurveySettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveySettings {
    #[serde(rename = "thank_you", skip_serializing_if = "Option::is_none")]
    pub thank_you: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "submit_label", skip_serializing_if = "Option::is_none")]
    pub submit_label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "write_properties", skip_serializing_if = "Option::is_none")]
    pub write_properties: Option<bool>,
    #[serde(rename = "allow_multiple", skip_serializing_if = "Option::is_none")]
    pub allow_multiple: Option<bool>,
    #[serde(rename = "default_language", skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
}

/// `SurveyStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyStats {
    /// Zorunlu.
    #[serde(rename = "responses", deserialize_with = "crate::de::null_default")]
    pub responses: i64,
    /// Zorunlu.
    #[serde(rename = "nps")]
    pub nps: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "last_response_at")]
    pub last_response_at: Option<String>,
}

/// `SurveyStatusDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurveyStatusDto {
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"closed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
}

/// `SyncAdAudienceDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SyncAdAudienceDto {
    #[serde(rename = "full", skip_serializing_if = "Option::is_none")]
    pub full: Option<bool>,
}

/// `Template` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Template {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel")]
    pub channel: Option<String>,
    /// Zorunlu.
    #[serde(rename = "current_version", deserialize_with = "crate::de::null_default")]
    pub current_version: i64,
    /// Zorunlu.
    #[serde(rename = "tags", deserialize_with = "crate::de::null_default")]
    pub tags: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "content", deserialize_with = "crate::de::null_default")]
    pub content: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `TemplatePage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TemplatePage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Template>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `TemplateStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TemplateStats {
    /// Zorunlu.
    #[serde(rename = "template_id", deserialize_with = "crate::de::null_default")]
    pub template_id: String,
    /// Zorunlu.
    #[serde(rename = "messages", deserialize_with = "crate::de::null_default")]
    pub messages: i64,
    /// Zorunlu.
    #[serde(rename = "campaigns", deserialize_with = "crate::de::null_default")]
    pub campaigns: i64,
    /// Zorunlu.
    #[serde(rename = "targeted", deserialize_with = "crate::de::null_default")]
    pub targeted: i64,
    /// Zorunlu.
    #[serde(rename = "sent", deserialize_with = "crate::de::null_default")]
    pub sent: i64,
    /// Zorunlu.
    #[serde(rename = "delivered", deserialize_with = "crate::de::null_default")]
    pub delivered: i64,
    /// Zorunlu.
    #[serde(rename = "opened", deserialize_with = "crate::de::null_default")]
    pub opened: i64,
    /// Zorunlu.
    #[serde(rename = "clicked", deserialize_with = "crate::de::null_default")]
    pub clicked: i64,
    /// Zorunlu.
    #[serde(rename = "converted", deserialize_with = "crate::de::null_default")]
    pub converted: i64,
    /// Zorunlu.
    #[serde(rename = "open_rate", deserialize_with = "crate::de::null_default")]
    pub open_rate: f64,
    /// Zorunlu.
    #[serde(rename = "click_rate", deserialize_with = "crate::de::null_default")]
    pub click_rate: f64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
    /// Zorunlu.
    #[serde(rename = "last_used_at")]
    pub last_used_at: Option<String>,
}

/// `TemplateVersionList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TemplateVersionList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<TemplateVersionListDataItem>,
}

/// `TemplateVersionListDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TemplateVersionListDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "content", deserialize_with = "crate::de::null_default")]
    pub content: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `TestDataFeedDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TestDataFeedDto {
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// `TestPushDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TestPushDto {
    #[serde(rename = "subscription_ids", skip_serializing_if = "Option::is_none")]
    pub subscription_ids: Option<Vec<String>>,
    #[serde(rename = "headings", skip_serializing_if = "Option::is_none")]
    pub headings: Option<std::collections::HashMap<String, String>>,
    /// Zorunlu.
    #[serde(rename = "contents", deserialize_with = "crate::de::null_default")]
    pub contents: std::collections::HashMap<String, String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
}

/// `TriggerRunDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TriggerRunDto {
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<TriggerRunDtoAlias>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// `TriggerRunDtoAlias` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TriggerRunDtoAlias {
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `UninstallReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UninstallReport {
    /// Zorunlu.
    #[serde(rename = "from", deserialize_with = "crate::de::null_default")]
    pub from: String,
    /// Zorunlu.
    #[serde(rename = "to", deserialize_with = "crate::de::null_default")]
    pub to: String,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: UninstallReportTotals,
    /// Zorunlu.
    #[serde(rename = "previous")]
    pub previous: Option<UninstallReportPrevious>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<UninstallReportRowsItem>,
    /// Zorunlu.
    #[serde(rename = "by_channel", deserialize_with = "crate::de::null_default")]
    pub by_channel: Vec<UninstallReportByChannelItem>,
    /// Zorunlu.
    #[serde(rename = "last_messages", deserialize_with = "crate::de::null_default")]
    pub last_messages: Vec<UninstallReportLastMessagesItem>,
}

/// `UninstallReportByChannelItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UninstallReportByChannelItem {
    /// Zorunlu.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "uninstalls", deserialize_with = "crate::de::null_default")]
    pub uninstalls: i64,
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
}

/// `UninstallReportLastMessagesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UninstallReportLastMessagesItem {
    /// Zorunlu.
    #[serde(rename = "campaign_id")]
    pub campaign_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu.
    #[serde(rename = "uninstalls", deserialize_with = "crate::de::null_default")]
    pub uninstalls: i64,
}

/// `UninstallReportPrevious` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UninstallReportPrevious {
    /// Zorunlu.
    #[serde(rename = "uninstalls", deserialize_with = "crate::de::null_default")]
    pub uninstalls: i64,
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
    /// Zorunlu.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
}

/// `UninstallReportRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UninstallReportRowsItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "uninstalls", deserialize_with = "crate::de::null_default")]
    pub uninstalls: i64,
}

/// `UninstallReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UninstallReportTotals {
    /// Zorunlu.
    #[serde(rename = "uninstalls", deserialize_with = "crate::de::null_default")]
    pub uninstalls: i64,
    /// Zorunlu.
    #[serde(rename = "active", deserialize_with = "crate::de::null_default")]
    pub active: i64,
    /// Zorunlu.
    #[serde(rename = "rate")]
    pub rate: Option<f64>,
}

/// `UpdateAdAudienceDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAdAudienceDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "segment_id", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// Değerler: `"email"`, `"phone"`, `"maid"`.
    #[serde(rename = "identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<String>>,
    #[serde(rename = "sync_interval_minutes", skip_serializing_if = "Option::is_none")]
    pub sync_interval_minutes: Option<i64>,
    /// Değerler: `"active"`, `"paused"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// `UpdateAdCampaignDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAdCampaignDto {
    /// Değerler: `"active"`, `"paused"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "daily_budget", skip_serializing_if = "Option::is_none")]
    pub daily_budget: Option<serde_json::Value>,
}

/// `UpdateApiKeyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateApiKeyDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"admin"`, `"notifications:write"`, `"users:read"`, `"users:write"`, `"events:write"`, `"segments:read"`, `"analytics:read"`, `"mcp:write"`.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "ip_allowlist", skip_serializing_if = "Option::is_none")]
    pub ip_allowlist: Option<Vec<String>>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// `UpdateAppDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "platform_flags", skip_serializing_if = "Option::is_none")]
    pub platform_flags: Option<UpdateAppDtoPlatformFlags>,
    #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "default_language", skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateAppDtoSettings>,
    /// Değerler: `"hmac"`, `"jwt"`, `"both"`.
    #[serde(rename = "identity_verification_mode", skip_serializing_if = "Option::is_none")]
    pub identity_verification_mode: Option<String>,
}

/// `UpdateAppDtoPlatformFlags` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoPlatformFlags {
    #[serde(rename = "ios", skip_serializing_if = "Option::is_none")]
    pub ios: Option<bool>,
    #[serde(rename = "android", skip_serializing_if = "Option::is_none")]
    pub android: Option<bool>,
    #[serde(rename = "web", skip_serializing_if = "Option::is_none")]
    pub web: Option<bool>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
}

/// `UpdateAppDtoSettings` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettings {
    #[serde(rename = "quietHours", skip_serializing_if = "Option::is_none")]
    pub quiet_hours: Option<UpdateAppDtoSettingsQuietHours>,
    #[serde(rename = "frequencyCap", skip_serializing_if = "Option::is_none")]
    pub frequency_cap: Option<UpdateAppDtoSettingsFrequencyCap>,
    #[serde(rename = "frequencyCaps", skip_serializing_if = "Option::is_none")]
    pub frequency_caps: Option<Vec<UpdateAppDtoSettingsFrequencyCapsItem>>,
    #[serde(rename = "inApp", skip_serializing_if = "Option::is_none")]
    pub in_app: Option<UpdateAppDtoSettingsInApp>,
    #[serde(rename = "androidChannels", skip_serializing_if = "Option::is_none")]
    pub android_channels: Option<Vec<UpdateAppDtoSettingsAndroidChannelsItem>>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<UpdateAppDtoSettingsSms>,
    #[serde(rename = "preferenceCenter", skip_serializing_if = "Option::is_none")]
    pub preference_center: Option<UpdateAppDtoSettingsPreferenceCenter>,
    #[serde(rename = "brandKit", skip_serializing_if = "Option::is_none")]
    pub brand_kit: Option<UpdateAppDtoSettingsBrandKit>,
    #[serde(rename = "defaultTtlSeconds", skip_serializing_if = "Option::is_none")]
    pub default_ttl_seconds: Option<i64>,
    #[serde(rename = "identityVerificationRequired", skip_serializing_if = "Option::is_none")]
    pub identity_verification_required: Option<bool>,
    /// Değerler: `"off"`, `"full"`, `"masked"`.
    #[serde(rename = "ipCapture", skip_serializing_if = "Option::is_none")]
    pub ip_capture: Option<String>,
    #[serde(rename = "ipRetentionDays", skip_serializing_if = "Option::is_none")]
    pub ip_retention_days: Option<i64>,
    #[serde(rename = "adaptiveFrequency", skip_serializing_if = "Option::is_none")]
    pub adaptive_frequency: Option<UpdateAppDtoSettingsAdaptiveFrequency>,
    #[serde(rename = "allowedOrigins", skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(rename = "campaignApprovalRequired", skip_serializing_if = "Option::is_none")]
    pub campaign_approval_required: Option<bool>,
    #[serde(rename = "webPush", skip_serializing_if = "Option::is_none")]
    pub web_push: Option<UpdateAppDtoSettingsWebPush>,
    #[serde(rename = "onboarding", skip_serializing_if = "Option::is_none")]
    pub onboarding: Option<UpdateAppDtoSettingsOnboarding>,
    #[serde(rename = "sdkSetup", skip_serializing_if = "Option::is_none")]
    pub sdk_setup: Option<UpdateAppDtoSettingsSdkSetup>,
}

/// `UpdateAppDtoSettingsAdaptiveFrequency` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsAdaptiveFrequency {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "min", deserialize_with = "crate::de::null_default")]
    pub min: i64,
    /// Zorunlu.
    #[serde(rename = "max", deserialize_with = "crate::de::null_default")]
    pub max: i64,
}

/// `UpdateAppDtoSettingsAndroidChannelsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsAndroidChannelsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Zorunlu. Değerler: `"low"`, `"default"`, `"high"`, `"urgent"`.
    #[serde(rename = "importance", deserialize_with = "crate::de::null_default")]
    pub importance: String,
    #[serde(rename = "sound", skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "vibration", skip_serializing_if = "Option::is_none")]
    pub vibration: Option<bool>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// `UpdateAppDtoSettingsBrandKit` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsBrandKit {
    #[serde(rename = "website_url", skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "primary_color", skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[serde(rename = "accent_color", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    #[serde(rename = "font", skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "tone", skip_serializing_if = "Option::is_none")]
    pub tone: Option<String>,
}

/// `UpdateAppDtoSettingsFrequencyCap` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsFrequencyCap {
    /// Zorunlu.
    #[serde(rename = "max", deserialize_with = "crate::de::null_default")]
    pub max: i64,
    /// Zorunlu.
    #[serde(rename = "windowHours", deserialize_with = "crate::de::null_default")]
    pub window_hours: i64,
}

/// `UpdateAppDtoSettingsFrequencyCapsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsFrequencyCapsItem {
    /// Zorunlu.
    #[serde(rename = "max", deserialize_with = "crate::de::null_default")]
    pub max: i64,
    /// Zorunlu.
    #[serde(rename = "window", deserialize_with = "crate::de::null_default")]
    pub window: UpdateAppDtoSettingsFrequencyCapsItemWindow,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
}

/// `UpdateAppDtoSettingsFrequencyCapsItemWindow` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsFrequencyCapsItemWindow {
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: i64,
    /// Zorunlu. Değerler: `"minutes"`, `"hours"`, `"days"`, `"weeks"`.
    #[serde(rename = "unit", deserialize_with = "crate::de::null_default")]
    pub unit: String,
}

/// `UpdateAppDtoSettingsInApp` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsInApp {
    #[serde(rename = "minIntervalSeconds", skip_serializing_if = "Option::is_none")]
    pub min_interval_seconds: Option<i64>,
}

/// `UpdateAppDtoSettingsOnboarding` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsOnboarding {
    /// Zorunlu.
    #[serde(rename = "step", deserialize_with = "crate::de::null_default")]
    pub step: i64,
    #[serde(rename = "simulation", skip_serializing_if = "Option::is_none")]
    pub simulation: Option<bool>,
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// Değerler: `"ios"`, `"android"`, `"huawei"`, `"web"`, `"amazon"`, `"windows"`, `"email"`, `"sms"`.
    #[serde(rename = "platforms", skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
    /// Değerler: `"native"`, `"flutter"`, `"react-native"`, `"expo"`, `"kmp"`, `"unity"`, `"capacitor"`, `"cordova"`, `"maui"`.
    #[serde(rename = "framework", skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// Değerler: `"code"`, `"wordpress"`.
    #[serde(rename = "web_integration", skip_serializing_if = "Option::is_none")]
    pub web_integration: Option<String>,
}

/// `UpdateAppDtoSettingsPreferenceCenter` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsPreferenceCenter {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "brandName", skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(rename = "logoUrl", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "accentColor", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    #[serde(rename = "customUnsubscribeUrl", skip_serializing_if = "Option::is_none")]
    pub custom_unsubscribe_url: Option<String>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<std::collections::HashMap<String, UpdateAppDtoSettingsPreferenceCenterTextsValue>>,
}

/// `UpdateAppDtoSettingsPreferenceCenterTextsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsPreferenceCenterTextsValue {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "unsubscribeAll", skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<String>,
    #[serde(rename = "saved", skip_serializing_if = "Option::is_none")]
    pub saved: Option<String>,
}

/// `UpdateAppDtoSettingsQuietHours` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsQuietHours {
    /// Zorunlu.
    #[serde(rename = "start", deserialize_with = "crate::de::null_default")]
    pub start: String,
    /// Zorunlu.
    #[serde(rename = "end", deserialize_with = "crate::de::null_default")]
    pub end: String,
    /// Zorunlu. Değerler: `"user"`, `"app"`.
    #[serde(rename = "timezoneMode", deserialize_with = "crate::de::null_default")]
    pub timezone_mode: String,
}

/// `UpdateAppDtoSettingsSdkSetup` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsSdkSetup {
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "platform")]
    pub platform: Option<String>,
    /// Zorunlu.
    #[serde(rename = "step", deserialize_with = "crate::de::null_default")]
    pub step: String,
    /// Zorunlu.
    #[serde(rename = "furthest", deserialize_with = "crate::de::null_default")]
    pub furthest: String,
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(rename = "detectedSubscriptionId", skip_serializing_if = "Option::is_none")]
    pub detected_subscription_id: Option<String>,
    #[serde(rename = "testMessageId", skip_serializing_if = "Option::is_none")]
    pub test_message_id: Option<String>,
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

/// `UpdateAppDtoSettingsSms` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsSms {
    #[serde(rename = "helpReply", skip_serializing_if = "Option::is_none")]
    pub help_reply: Option<String>,
    #[serde(rename = "stopReply", skip_serializing_if = "Option::is_none")]
    pub stop_reply: Option<String>,
    #[serde(rename = "autoReply", skip_serializing_if = "Option::is_none")]
    pub auto_reply: Option<bool>,
}

/// `UpdateAppDtoSettingsWebPush` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPush {
    #[serde(rename = "serviceWorkerPath", skip_serializing_if = "Option::is_none")]
    pub service_worker_path: Option<String>,
    #[serde(rename = "serviceWorkerScope", skip_serializing_if = "Option::is_none")]
    pub service_worker_scope: Option<String>,
    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<UpdateAppDtoSettingsWebPushPrompt>,
    #[serde(rename = "welcomeNotification", skip_serializing_if = "Option::is_none")]
    pub welcome_notification: Option<UpdateAppDtoSettingsWebPushWelcomeNotification>,
    #[serde(rename = "clickBehavior", skip_serializing_if = "Option::is_none")]
    pub click_behavior: Option<UpdateAppDtoSettingsWebPushClickBehavior>,
    #[serde(rename = "customLink", skip_serializing_if = "Option::is_none")]
    pub custom_link: Option<UpdateAppDtoSettingsWebPushCustomLink>,
    #[serde(rename = "promptBackoffDays", skip_serializing_if = "Option::is_none")]
    pub prompt_backoff_days: Option<Vec<i64>>,
}

/// `UpdateAppDtoSettingsWebPushClickBehavior` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushClickBehavior {
    /// Zorunlu. Değerler: `"exact"`, `"origin"`.
    #[serde(rename = "match", deserialize_with = "crate::de::null_default")]
    pub r#match: String,
    /// Zorunlu. Değerler: `"navigate"`, `"focus"`.
    #[serde(rename = "action", deserialize_with = "crate::de::null_default")]
    pub action: String,
}

/// `UpdateAppDtoSettingsWebPushCustomLink` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushCustomLink {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu. Değerler: `"button"`, `"link"`.
    #[serde(rename = "style", deserialize_with = "crate::de::null_default")]
    pub style: String,
    /// Zorunlu. Değerler: `"small"`, `"medium"`, `"large"`.
    #[serde(rename = "size", deserialize_with = "crate::de::null_default")]
    pub size: String,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<UpdateAppDtoSettingsWebPushCustomLinkColor>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<std::collections::HashMap<String, UpdateAppDtoSettingsWebPushCustomLinkTextsValue>>,
    #[serde(rename = "unsubscribeEnabled", skip_serializing_if = "Option::is_none")]
    pub unsubscribe_enabled: Option<bool>,
}

/// `UpdateAppDtoSettingsWebPushCustomLinkColor` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushCustomLinkColor {
    #[serde(rename = "button", skip_serializing_if = "Option::is_none")]
    pub button: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// `UpdateAppDtoSettingsWebPushCustomLinkTextsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushCustomLinkTextsValue {
    #[serde(rename = "subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<String>,
    #[serde(rename = "unsubscribe", skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<String>,
    #[serde(rename = "explanation", skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

/// `UpdateAppDtoSettingsWebPushPrompt` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushPrompt {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu. Değerler: `"slidedown"`, `"native"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Değerler: `"push"`, `"email"`, `"sms"`, `"email_sms"`.
    #[serde(rename = "collect", skip_serializing_if = "Option::is_none")]
    pub collect: Option<String>,
    #[serde(rename = "delaySeconds", skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<i64>,
    #[serde(rename = "pageViews", skip_serializing_if = "Option::is_none")]
    pub page_views: Option<i64>,
    #[serde(rename = "scrollPercent", skip_serializing_if = "Option::is_none")]
    pub scroll_percent: Option<i64>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<std::collections::HashMap<String, UpdateAppDtoSettingsWebPushPromptTextsValue>>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<UpdateAppDtoSettingsWebPushPromptCategoriesItem>>,
    #[serde(rename = "accentColor", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
}

/// `UpdateAppDtoSettingsWebPushPromptCategoriesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushPromptCategoriesItem {
    /// Zorunlu.
    #[serde(rename = "tag", deserialize_with = "crate::de::null_default")]
    pub tag: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: std::collections::HashMap<String, String>,
}

/// `UpdateAppDtoSettingsWebPushPromptTextsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushPromptTextsValue {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "accept", skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(rename = "cancel", skip_serializing_if = "Option::is_none")]
    pub cancel: Option<String>,
}

/// `UpdateAppDtoSettingsWebPushWelcomeNotification` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushWelcomeNotification {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<std::collections::HashMap<String, UpdateAppDtoSettingsWebPushWelcomeNotificationTextsValue>>,
}

/// `UpdateAppDtoSettingsWebPushWelcomeNotificationTextsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppDtoSettingsWebPushWelcomeNotificationTextsValue {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `UpdateAppFlagDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppFlagDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Değerler: `"boolean"`, `"string"`, `"number"`, `"json"`.
    #[serde(rename = "value_type", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(rename = "on_value", skip_serializing_if = "Option::is_none")]
    pub on_value: Option<serde_json::Value>,
    #[serde(rename = "off_value", skip_serializing_if = "Option::is_none")]
    pub off_value: Option<serde_json::Value>,
    #[serde(rename = "rollout_percent", skip_serializing_if = "Option::is_none")]
    pub rollout_percent: Option<i64>,
    #[serde(rename = "segment_id", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "rollout_schedule", skip_serializing_if = "Option::is_none")]
    pub rollout_schedule: Option<Vec<UpdateAppFlagDtoRolloutScheduleItem>>,
}

/// `UpdateAppFlagDtoRolloutScheduleItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateAppFlagDtoRolloutScheduleItem {
    /// Zorunlu.
    #[serde(rename = "at", deserialize_with = "crate::de::null_default")]
    pub at: String,
    /// Zorunlu.
    #[serde(rename = "percent", deserialize_with = "crate::de::null_default")]
    pub percent: i64,
}

/// `UpdateBudgetAlertRuleDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateBudgetAlertRuleDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ad_account_id", skip_serializing_if = "Option::is_none")]
    pub ad_account_id: Option<String>,
    #[serde(rename = "campaign_external_id", skip_serializing_if = "Option::is_none")]
    pub campaign_external_id: Option<String>,
    #[serde(rename = "threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "window_days", skip_serializing_if = "Option::is_none")]
    pub window_days: Option<i64>,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// `UpdateCatalogDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateCatalogDto {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// `UpdateCategoryDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateCategoryDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<std::collections::HashMap<String, String>>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "default_opted_in", skip_serializing_if = "Option::is_none")]
    pub default_opted_in: Option<bool>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

/// `UpdateChannelScoreConfigDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateChannelScoreConfigDto {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "window_days", skip_serializing_if = "Option::is_none")]
    pub window_days: Option<i64>,
    #[serde(rename = "min_sends", skip_serializing_if = "Option::is_none")]
    pub min_sends: Option<i64>,
    #[serde(rename = "count_email_opens", skip_serializing_if = "Option::is_none")]
    pub count_email_opens: Option<bool>,
}

/// `UpdateContentTableDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateContentTableDto {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `UpdateDataFeedDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateDataFeedDto {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "url_template", skip_serializing_if = "Option::is_none")]
    pub url_template: Option<String>,
    /// Değerler: `"GET"`, `"POST"`.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "body_template", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "secret_headers", skip_serializing_if = "Option::is_none")]
    pub secret_headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "cache_ttl_seconds", skip_serializing_if = "Option::is_none")]
    pub cache_ttl_seconds: Option<i64>,
    #[serde(rename = "timeout_ms", skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// `UpdateDecisionAgentDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateDecisionAgentDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"active"`, `"paused"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "reward_events", skip_serializing_if = "Option::is_none")]
    pub reward_events: Option<Vec<String>>,
    #[serde(rename = "reward_window_hours", skip_serializing_if = "Option::is_none")]
    pub reward_window_hours: Option<i64>,
    #[serde(rename = "holdout_percent", skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i64>,
}

/// `UpdateGeofenceDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateGeofenceDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(rename = "lng", skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    #[serde(rename = "radius_m", skip_serializing_if = "Option::is_none")]
    pub radius_m: Option<i64>,
    #[serde(rename = "cooldown_minutes", skip_serializing_if = "Option::is_none")]
    pub cooldown_minutes: Option<i64>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// `UpdateGlobalControlDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateGlobalControlDto {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "percent", deserialize_with = "crate::de::null_default")]
    pub percent: i64,
}

/// `UpdateInAppDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateInAppDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "triggers", skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<serde_json::Value>>,
    #[serde(rename = "trigger_groups", skip_serializing_if = "Option::is_none")]
    pub trigger_groups: Option<Vec<Vec<serde_json::Value>>>,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<UpdateInAppDtoVariantsItem>>,
    #[serde(rename = "ab_test", skip_serializing_if = "Option::is_none")]
    pub ab_test: Option<UpdateInAppDtoAbTest>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<UpdateInAppDtoDisplay>,
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<UpdateInAppDtoAudience>,
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<UpdateInAppDtoFrequency>,
    #[serde(rename = "journey_only", skip_serializing_if = "Option::is_none")]
    pub journey_only: Option<bool>,
    #[serde(rename = "placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
}

/// `UpdateInAppDtoAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateInAppDtoAbTest {
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<UpdateInAppDtoAbTestWinner>,
}

/// `UpdateInAppDtoAbTestWinner` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateInAppDtoAbTestWinner {
    /// Zorunlu. Değerler: `"manual"`, `"auto"`.
    #[serde(rename = "mode", deserialize_with = "crate::de::null_default")]
    pub mode: String,
    #[serde(rename = "min_impressions", skip_serializing_if = "Option::is_none")]
    pub min_impressions: Option<i64>,
    #[serde(rename = "confidence", skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// `UpdateInAppDtoAudience` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateInAppDtoAudience {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    #[serde(rename = "global_control_exempt", skip_serializing_if = "Option::is_none")]
    pub global_control_exempt: Option<bool>,
}

/// `UpdateInAppDtoDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateInAppDtoDisplay {
    /// Zorunlu. Değerler: `"modal"`, `"banner_top"`, `"banner_bottom"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    /// Değerler: `"blocks"`, `"html"`.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "html_height", skip_serializing_if = "Option::is_none")]
    pub html_height: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
}

/// `UpdateInAppDtoFrequency` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateInAppDtoFrequency {
    #[serde(rename = "max_impressions", skip_serializing_if = "Option::is_none")]
    pub max_impressions: Option<i64>,
    #[serde(rename = "min_interval_seconds", skip_serializing_if = "Option::is_none")]
    pub min_interval_seconds: Option<i64>,
    #[serde(rename = "stop_after_interaction", skip_serializing_if = "Option::is_none")]
    pub stop_after_interaction: Option<bool>,
}

/// `UpdateInAppDtoVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateInAppDtoVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: UpdateInAppDtoVariantsItemDisplay,
}

/// `UpdateInAppDtoVariantsItemDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateInAppDtoVariantsItemDisplay {
    /// Zorunlu. Değerler: `"modal"`, `"banner_top"`, `"banner_bottom"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    /// Değerler: `"blocks"`, `"html"`.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "html_height", skip_serializing_if = "Option::is_none")]
    pub html_height: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
}

/// `UpdateJourneyDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateJourneyDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `UpdateKeywordDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateKeywordDto {
    #[serde(rename = "keyword", skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Değerler: `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<UpdateKeywordDtoActions>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `UpdateKeywordDtoActions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateKeywordDtoActions {
    #[serde(rename = "add_tags", skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "remove_tags", skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<UpdateKeywordDtoActionsEvent>,
    #[serde(rename = "reply", skip_serializing_if = "Option::is_none")]
    pub reply: Option<UpdateKeywordDtoActionsReply>,
    #[serde(rename = "opt_in", skip_serializing_if = "Option::is_none")]
    pub opt_in: Option<bool>,
}

/// `UpdateKeywordDtoActionsEvent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateKeywordDtoActionsEvent {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

/// `UpdateKeywordDtoActionsReply` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateKeywordDtoActionsReply {
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
}

/// `UpdateLiveActivityDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateLiveActivityDto {
    /// Zorunlu.
    #[serde(rename = "activity_id", deserialize_with = "crate::de::null_default")]
    pub activity_id: String,
    /// Zorunlu.
    #[serde(rename = "content_state", deserialize_with = "crate::de::null_default")]
    pub content_state: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<UpdateLiveActivityDtoAlert>,
    #[serde(rename = "stale_date", skip_serializing_if = "Option::is_none")]
    pub stale_date: Option<i64>,
    #[serde(rename = "dismissal_date", skip_serializing_if = "Option::is_none")]
    pub dismissal_date: Option<i64>,
    #[serde(rename = "relevance_score", skip_serializing_if = "Option::is_none")]
    pub relevance_score: Option<f64>,
    /// Değerler: `5`, `10`.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `UpdateLiveActivityDtoAlert` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateLiveActivityDtoAlert {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "sound", skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
}

/// `UpdateLiveActivityTypeDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateLiveActivityTypeDto {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "content_state_schema", skip_serializing_if = "Option::is_none")]
    pub content_state_schema: Option<UpdateLiveActivityTypeDtoContentStateSchema>,
    #[serde(rename = "attributes_schema", skip_serializing_if = "Option::is_none")]
    pub attributes_schema: Option<UpdateLiveActivityTypeDtoAttributesSchema>,
}

/// `UpdateLiveActivityTypeDtoAttributesSchema` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateLiveActivityTypeDtoAttributesSchema {
    /// Zorunlu. Değerler: `"object"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, UpdateLiveActivityTypeDtoAttributesSchemaPropertiesValue>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// `UpdateLiveActivityTypeDtoAttributesSchemaPropertiesValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateLiveActivityTypeDtoAttributesSchemaPropertiesValue {
    /// Zorunlu. Değerler: `"string"`, `"number"`, `"integer"`, `"boolean"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `UpdateLiveActivityTypeDtoContentStateSchema` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateLiveActivityTypeDtoContentStateSchema {
    /// Zorunlu. Değerler: `"object"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, UpdateLiveActivityTypeDtoContentStateSchemaPropertiesValue>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// `UpdateLiveActivityTypeDtoContentStateSchemaPropertiesValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateLiveActivityTypeDtoContentStateSchemaPropertiesValue {
    /// Zorunlu. Değerler: `"string"`, `"number"`, `"integer"`, `"boolean"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `UpdateNodeDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateNodeDto {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<UpdateNodeDtoPosition>,
}

/// `UpdateNodeDtoPosition` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateNodeDtoPosition {
    /// Zorunlu.
    #[serde(rename = "x", deserialize_with = "crate::de::null_default")]
    pub x: f64,
    /// Zorunlu.
    #[serde(rename = "y", deserialize_with = "crate::de::null_default")]
    pub y: f64,
}

/// `UpdatePredictionDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdatePredictionDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "event_names", skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
    #[serde(rename = "horizon_days", skip_serializing_if = "Option::is_none")]
    pub horizon_days: Option<i64>,
}

/// `UpdatePromoCodeListDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdatePromoCodeListDto {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// `UpdateRecommendationConfigDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateRecommendationConfigDto {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    #[serde(rename = "view_events", skip_serializing_if = "Option::is_none")]
    pub view_events: Option<Vec<String>>,
    #[serde(rename = "purchase_events", skip_serializing_if = "Option::is_none")]
    pub purchase_events: Option<Vec<String>>,
    #[serde(rename = "item_property", skip_serializing_if = "Option::is_none")]
    pub item_property: Option<String>,
    #[serde(rename = "window_days", skip_serializing_if = "Option::is_none")]
    pub window_days: Option<i64>,
}

/// `UpdateReverseEtlDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateReverseEtlDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"every_5_minutes"`, `"hourly"`, `"daily"`.
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// `UpdateRfmConfigDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateRfmConfigDto {
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu.
    #[serde(rename = "event_names", deserialize_with = "crate::de::null_default")]
    pub event_names: Vec<String>,
    #[serde(rename = "value_property", skip_serializing_if = "Option::is_none")]
    pub value_property: Option<String>,
    #[serde(rename = "use_monetary", skip_serializing_if = "Option::is_none")]
    pub use_monetary: Option<bool>,
    #[serde(rename = "window_days", skip_serializing_if = "Option::is_none")]
    pub window_days: Option<i64>,
}

/// `UpdateSavedReportDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateSavedReportDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "definition", skip_serializing_if = "Option::is_none")]
    pub definition: Option<UpdateSavedReportDtoDefinition>,
}

/// `UpdateSavedReportDtoDefinition` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateSavedReportDtoDefinition {
    /// Zorunlu. Değerler: `"deliveries"`, `"events"`, `"conversions"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu. Değerler: `"sent"`, `"delivered"`, `"failed"`, `"opened"`, `"clicked"`, `"delivery_rate"`, `"open_rate"`, `"click_rate"`, `"failure_rate"`, `"count"`, `"unique_users"`, `"value_sum"`, `"value_avg"`, `"conversions"`, `"revenue"`.
    #[serde(rename = "metrics", deserialize_with = "crate::de::null_default")]
    pub metrics: Vec<String>,
    /// Değerler: `"day"`, `"week"`, `"month"`, `"channel"`, `"provider"`, `"campaign"`, `"status"`, `"name"`, `"source"`, `"event_name"`, `"kind"`.
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<UpdateSavedReportDtoDefinitionSort>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// `UpdateSavedReportDtoDefinitionSort` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateSavedReportDtoDefinitionSort {
    /// Zorunlu.
    #[serde(rename = "by", deserialize_with = "crate::de::null_default")]
    pub by: String,
    /// Değerler: `"asc"`, `"desc"`.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// `UpdateTemplateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<UpdateTemplateDtoContent>,
}

/// `UpdateTemplateDtoContent` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContent {
    #[serde(rename = "headings", skip_serializing_if = "Option::is_none")]
    pub headings: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "subtitle", skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "app_url", skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<UpdateTemplateDtoContentButtonsItem>>,
    #[serde(rename = "big_picture", skip_serializing_if = "Option::is_none")]
    pub big_picture: Option<String>,
    #[serde(rename = "ios_attachments", skip_serializing_if = "Option::is_none")]
    pub ios_attachments: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ios_sound", skip_serializing_if = "Option::is_none")]
    pub ios_sound: Option<String>,
    /// Değerler: `"None"`, `"SetTo"`, `"Increase"`.
    #[serde(rename = "ios_badge_type", skip_serializing_if = "Option::is_none")]
    pub ios_badge_type: Option<String>,
    #[serde(rename = "ios_badge_count", skip_serializing_if = "Option::is_none")]
    pub ios_badge_count: Option<i64>,
    /// Değerler: `"passive"`, `"active"`, `"time-sensitive"`, `"critical"`.
    #[serde(rename = "ios_interruption_level", skip_serializing_if = "Option::is_none")]
    pub ios_interruption_level: Option<String>,
    #[serde(rename = "ios_category", skip_serializing_if = "Option::is_none")]
    pub ios_category: Option<String>,
    #[serde(rename = "android_channel_id", skip_serializing_if = "Option::is_none")]
    pub android_channel_id: Option<String>,
    #[serde(rename = "android_group", skip_serializing_if = "Option::is_none")]
    pub android_group: Option<String>,
    #[serde(rename = "android_sound", skip_serializing_if = "Option::is_none")]
    pub android_sound: Option<String>,
    #[serde(rename = "chrome_web_icon", skip_serializing_if = "Option::is_none")]
    pub chrome_web_icon: Option<String>,
    #[serde(rename = "chrome_web_image", skip_serializing_if = "Option::is_none")]
    pub chrome_web_image: Option<String>,
    #[serde(rename = "chrome_web_badge", skip_serializing_if = "Option::is_none")]
    pub chrome_web_badge: Option<String>,
    #[serde(rename = "web_push_topic", skip_serializing_if = "Option::is_none")]
    pub web_push_topic: Option<String>,
    #[serde(rename = "content_available", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<bool>,
    #[serde(rename = "mutable_content", skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<bool>,
    #[serde(rename = "custom_data", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "email_subject", skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[serde(rename = "email_body", skip_serializing_if = "Option::is_none")]
    pub email_body: Option<String>,
    #[serde(rename = "email_mjml", skip_serializing_if = "Option::is_none")]
    pub email_mjml: Option<String>,
    #[serde(rename = "email_preheader", skip_serializing_if = "Option::is_none")]
    pub email_preheader: Option<String>,
    #[serde(rename = "email_from_name", skip_serializing_if = "Option::is_none")]
    pub email_from_name: Option<String>,
    #[serde(rename = "email_from_address", skip_serializing_if = "Option::is_none")]
    pub email_from_address: Option<String>,
    #[serde(rename = "email_reply_to_address", skip_serializing_if = "Option::is_none")]
    pub email_reply_to_address: Option<String>,
    #[serde(rename = "disable_email_click_tracking", skip_serializing_if = "Option::is_none")]
    pub disable_email_click_tracking: Option<bool>,
    #[serde(rename = "sms_from", skip_serializing_if = "Option::is_none")]
    pub sms_from: Option<String>,
    #[serde(rename = "disable_sms_link_shortening", skip_serializing_if = "Option::is_none")]
    pub disable_sms_link_shortening: Option<bool>,
    #[serde(rename = "add_to_inbox", skip_serializing_if = "Option::is_none")]
    pub add_to_inbox: Option<bool>,
    #[serde(rename = "inbox_ttl", skip_serializing_if = "Option::is_none")]
    pub inbox_ttl: Option<i64>,
    #[serde(rename = "content_card", skip_serializing_if = "Option::is_none")]
    pub content_card: Option<UpdateTemplateDtoContentContentCard>,
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(rename = "ios_relevance_score", skip_serializing_if = "Option::is_none")]
    pub ios_relevance_score: Option<f64>,
    #[serde(rename = "target_content_identifier", skip_serializing_if = "Option::is_none")]
    pub target_content_identifier: Option<String>,
    #[serde(rename = "small_icon", skip_serializing_if = "Option::is_none")]
    pub small_icon: Option<String>,
    #[serde(rename = "large_icon", skip_serializing_if = "Option::is_none")]
    pub large_icon: Option<String>,
    #[serde(rename = "android_accent_color", skip_serializing_if = "Option::is_none")]
    pub android_accent_color: Option<String>,
    /// Değerler: `1`, `0`, `-1`.
    #[serde(rename = "android_visibility", skip_serializing_if = "Option::is_none")]
    pub android_visibility: Option<i64>,
    #[serde(rename = "web_buttons", skip_serializing_if = "Option::is_none")]
    pub web_buttons: Option<Vec<UpdateTemplateDtoContentWebButtonsItem>>,
    #[serde(rename = "require_interaction", skip_serializing_if = "Option::is_none")]
    pub require_interaction: Option<bool>,
    #[serde(rename = "sms_media_urls", skip_serializing_if = "Option::is_none")]
    pub sms_media_urls: Option<Vec<String>>,
    #[serde(rename = "whatsapp_template", skip_serializing_if = "Option::is_none")]
    pub whatsapp_template: Option<UpdateTemplateDtoContentWhatsappTemplate>,
    #[serde(rename = "whatsapp_media", skip_serializing_if = "Option::is_none")]
    pub whatsapp_media: Option<UpdateTemplateDtoContentWhatsappMedia>,
    #[serde(rename = "whatsapp_interactive", skip_serializing_if = "Option::is_none")]
    pub whatsapp_interactive: Option<UpdateTemplateDtoContentWhatsappInteractive>,
    #[serde(rename = "rcs_card", skip_serializing_if = "Option::is_none")]
    pub rcs_card: Option<UpdateTemplateDtoContentRcsCard>,
    #[serde(rename = "rcs_suggestions", skip_serializing_if = "Option::is_none")]
    pub rcs_suggestions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "rcs_content_sid", skip_serializing_if = "Option::is_none")]
    pub rcs_content_sid: Option<String>,
    #[serde(rename = "rcs_content_variables", skip_serializing_if = "Option::is_none")]
    pub rcs_content_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rcs_fallback_text", skip_serializing_if = "Option::is_none")]
    pub rcs_fallback_text: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "messenger_image_url", skip_serializing_if = "Option::is_none")]
    pub messenger_image_url: Option<String>,
    #[serde(rename = "messenger_buttons", skip_serializing_if = "Option::is_none")]
    pub messenger_buttons: Option<Vec<serde_json::Value>>,
    #[serde(rename = "ios_carousel", skip_serializing_if = "Option::is_none")]
    pub ios_carousel: Option<Vec<UpdateTemplateDtoContentIosCarouselItem>>,
    #[serde(rename = "android_live_update", skip_serializing_if = "Option::is_none")]
    pub android_live_update: Option<UpdateTemplateDtoContentAndroidLiveUpdate>,
    #[serde(rename = "voip", skip_serializing_if = "Option::is_none")]
    pub voip: Option<bool>,
    #[serde(rename = "app_clip", skip_serializing_if = "Option::is_none")]
    pub app_clip: Option<bool>,
}

/// `UpdateTemplateDtoContentAndroidLiveUpdate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentAndroidLiveUpdate {
    /// Zorunlu.
    #[serde(rename = "progress", deserialize_with = "crate::de::null_default")]
    pub progress: i64,
    #[serde(rename = "ongoing", skip_serializing_if = "Option::is_none")]
    pub ongoing: Option<bool>,
}

/// `UpdateTemplateDtoContentButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `UpdateTemplateDtoContentContentCard` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentContentCard {
    /// Zorunlu. Değerler: `"classic"`, `"captioned_image"`, `"image_only"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "pinned", skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "button_text", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<std::collections::HashMap<String, String>>,
}

/// `UpdateTemplateDtoContentIosCarouselItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentIosCarouselItem {
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `UpdateTemplateDtoContentRcsCard` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentRcsCard {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "media_url", skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// Değerler: `"SHORT"`, `"MEDIUM"`, `"TALL"`.
    #[serde(rename = "media_height", skip_serializing_if = "Option::is_none")]
    pub media_height: Option<String>,
    /// Değerler: `"VERTICAL"`, `"HORIZONTAL"`.
    #[serde(rename = "orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
}

/// `UpdateTemplateDtoContentWebButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWebButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "text", deserialize_with = "crate::de::null_default")]
    pub text: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `UpdateTemplateDtoContentWhatsappInteractive` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWhatsappInteractive {
    /// Zorunlu. Değerler: `"button"`, `"list"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<UpdateTemplateDtoContentWhatsappInteractiveButtonsItem>>,
    #[serde(rename = "list", skip_serializing_if = "Option::is_none")]
    pub list: Option<UpdateTemplateDtoContentWhatsappInteractiveList>,
}

/// `UpdateTemplateDtoContentWhatsappInteractiveButtonsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWhatsappInteractiveButtonsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
}

/// `UpdateTemplateDtoContentWhatsappInteractiveList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWhatsappInteractiveList {
    /// Zorunlu.
    #[serde(rename = "button", deserialize_with = "crate::de::null_default")]
    pub button: String,
    /// Zorunlu.
    #[serde(rename = "sections", deserialize_with = "crate::de::null_default")]
    pub sections: Vec<UpdateTemplateDtoContentWhatsappInteractiveListSectionsItem>,
}

/// `UpdateTemplateDtoContentWhatsappInteractiveListSectionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWhatsappInteractiveListSectionsItem {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: Vec<UpdateTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem>,
}

/// `UpdateTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWhatsappInteractiveListSectionsItemRowsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `UpdateTemplateDtoContentWhatsappMedia` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWhatsappMedia {
    /// Zorunlu. Değerler: `"image"`, `"video"`, `"document"`, `"audio"`.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// `UpdateTemplateDtoContentWhatsappTemplate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWhatsappTemplate {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<serde_json::Value>,
    #[serde(rename = "body_params", skip_serializing_if = "Option::is_none")]
    pub body_params: Option<Vec<String>>,
    #[serde(rename = "button_params", skip_serializing_if = "Option::is_none")]
    pub button_params: Option<Vec<UpdateTemplateDtoContentWhatsappTemplateButtonParamsItem>>,
}

/// `UpdateTemplateDtoContentWhatsappTemplateButtonParamsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateTemplateDtoContentWhatsappTemplateButtonParamsItem {
    /// Zorunlu.
    #[serde(rename = "index", deserialize_with = "crate::de::null_default")]
    pub index: i64,
    /// Zorunlu. Değerler: `"url"`, `"quick_reply"`, `"copy_code"`.
    #[serde(rename = "sub_type", deserialize_with = "crate::de::null_default")]
    pub sub_type: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `UpdateWalletConfigDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWalletConfigDto {
    #[serde(rename = "apple", skip_serializing_if = "Option::is_none")]
    pub apple: Option<UpdateWalletConfigDtoApple>,
    #[serde(rename = "google", skip_serializing_if = "Option::is_none")]
    pub google: Option<UpdateWalletConfigDtoGoogle>,
}

/// `UpdateWalletConfigDtoApple` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWalletConfigDtoApple {
    /// Zorunlu.
    #[serde(rename = "pass_type_id", deserialize_with = "crate::de::null_default")]
    pub pass_type_id: String,
    /// Zorunlu.
    #[serde(rename = "team_id", deserialize_with = "crate::de::null_default")]
    pub team_id: String,
    /// Zorunlu.
    #[serde(rename = "certificate_pem", deserialize_with = "crate::de::null_default")]
    pub certificate_pem: String,
    /// Zorunlu.
    #[serde(rename = "private_key_pem", deserialize_with = "crate::de::null_default")]
    pub private_key_pem: String,
    #[serde(rename = "key_passphrase", skip_serializing_if = "Option::is_none")]
    pub key_passphrase: Option<String>,
    /// Zorunlu.
    #[serde(rename = "wwdr_pem", deserialize_with = "crate::de::null_default")]
    pub wwdr_pem: String,
}

/// `UpdateWalletConfigDtoGoogle` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWalletConfigDtoGoogle {
    /// Zorunlu.
    #[serde(rename = "issuer_id", deserialize_with = "crate::de::null_default")]
    pub issuer_id: String,
    /// Zorunlu.
    #[serde(rename = "service_account_json", deserialize_with = "crate::de::null_default")]
    pub service_account_json: String,
}

/// `UpdateWalletPassDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWalletPassDto {
    #[serde(rename = "barcode_value", skip_serializing_if = "Option::is_none")]
    pub barcode_value: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<std::collections::HashMap<String, String>>,
}

/// `UpdateWalletTemplateDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWalletTemplateDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"active"`, `"archived"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "design", skip_serializing_if = "Option::is_none")]
    pub design: Option<UpdateWalletTemplateDtoDesign>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<std::collections::HashMap<String, String>>,
}

/// `UpdateWalletTemplateDtoDesign` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWalletTemplateDtoDesign {
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    /// Zorunlu.
    #[serde(rename = "description", deserialize_with = "crate::de::null_default")]
    pub description: String,
    /// Zorunlu.
    #[serde(rename = "organization", deserialize_with = "crate::de::null_default")]
    pub organization: String,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "foreground_color", skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,
    #[serde(rename = "label_color", skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    /// Değerler: `"qr"`, `"pdf417"`, `"aztec"`, `"code128"`.
    #[serde(rename = "barcode", skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<UpdateWalletTemplateDtoDesignFieldsItem>>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "relevant_at", skip_serializing_if = "Option::is_none")]
    pub relevant_at: Option<String>,
}

/// `UpdateWalletTemplateDtoDesignFieldsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWalletTemplateDtoDesignFieldsItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Değerler: `"header"`, `"primary"`, `"secondary"`, `"auxiliary"`, `"back"`.
    #[serde(rename = "section", skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}

/// `UpdateWarehouseExportDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWarehouseExportDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"deliveries"`, `"delivery_events"`, `"events"`, `"conversions"`, `"users"`, `"subscriptions"`.
    #[serde(rename = "tables", skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<String>>,
    /// Değerler: `"every_15_minutes"`, `"hourly"`, `"daily"`.
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// `UpdateWarmupPlanDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWarmupPlanDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Değerler: `"active"`, `"paused"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "daily_limits", skip_serializing_if = "Option::is_none")]
    pub daily_limits: Option<Vec<i64>>,
    #[serde(rename = "provider_shares", skip_serializing_if = "Option::is_none")]
    pub provider_shares: Option<UpdateWarmupPlanDtoProviderShares>,
}

/// `UpdateWarmupPlanDtoProviderShares` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWarmupPlanDtoProviderShares {
    /// Zorunlu.
    #[serde(rename = "gmail", deserialize_with = "crate::de::null_default")]
    pub gmail: f64,
    /// Zorunlu.
    #[serde(rename = "outlook", deserialize_with = "crate::de::null_default")]
    pub outlook: f64,
    /// Zorunlu.
    #[serde(rename = "yahoo", deserialize_with = "crate::de::null_default")]
    pub yahoo: f64,
    /// Zorunlu.
    #[serde(rename = "other", deserialize_with = "crate::de::null_default")]
    pub other: f64,
}

/// `UpdateWebCampaignDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<UpdateWebCampaignDtoDisplay>,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<UpdateWebCampaignDtoVariantsItem>>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<serde_json::Value>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<UpdateWebCampaignDtoConditions>,
    #[serde(rename = "targeting", skip_serializing_if = "Option::is_none")]
    pub targeting: Option<UpdateWebCampaignDtoTargeting>,
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<UpdateWebCampaignDtoFrequency>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
}

/// `UpdateWebCampaignDtoConditions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoConditions {
    #[serde(rename = "url_rules", skip_serializing_if = "Option::is_none")]
    pub url_rules: Option<Vec<UpdateWebCampaignDtoConditionsUrlRulesItem>>,
    #[serde(rename = "utm_rules", skip_serializing_if = "Option::is_none")]
    pub utm_rules: Option<Vec<UpdateWebCampaignDtoConditionsUtmRulesItem>>,
    #[serde(rename = "visits", skip_serializing_if = "Option::is_none")]
    pub visits: Option<UpdateWebCampaignDtoConditionsVisits>,
}

/// `UpdateWebCampaignDtoConditionsUrlRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoConditionsUrlRulesItem {
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"starts_with"`, `"not_contains"`, `"matches"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `UpdateWebCampaignDtoConditionsUtmRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoConditionsUtmRulesItem {
    /// Zorunlu. Değerler: `"utm_source"`, `"utm_medium"`, `"utm_campaign"`, `"utm_term"`, `"utm_content"`.
    #[serde(rename = "param", deserialize_with = "crate::de::null_default")]
    pub param: String,
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// `UpdateWebCampaignDtoConditionsVisits` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoConditionsVisits {
    /// Zorunlu. Değerler: `"gte"`, `"lte"`, `"eq"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: i64,
}

/// `UpdateWebCampaignDtoDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoDisplay {
    /// Zorunlu. Değerler: `"popup"`, `"banner_top"`, `"banner_bottom"`, `"slide_in"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    /// Değerler: `"left"`, `"right"`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    #[serde(rename = "max_width", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "survey_id", skip_serializing_if = "Option::is_none")]
    pub survey_id: Option<String>,
}

/// `UpdateWebCampaignDtoFrequency` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoFrequency {
    #[serde(rename = "max_per_session", skip_serializing_if = "Option::is_none")]
    pub max_per_session: Option<i64>,
    #[serde(rename = "max_per_day", skip_serializing_if = "Option::is_none")]
    pub max_per_day: Option<i64>,
    #[serde(rename = "max_total", skip_serializing_if = "Option::is_none")]
    pub max_total: Option<i64>,
    #[serde(rename = "hide_after_close_days", skip_serializing_if = "Option::is_none")]
    pub hide_after_close_days: Option<i64>,
    #[serde(rename = "stop_after_conversion", skip_serializing_if = "Option::is_none")]
    pub stop_after_conversion: Option<bool>,
}

/// `UpdateWebCampaignDtoTargeting` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoTargeting {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    /// Değerler: `"all"`, `"new"`, `"returning"`.
    #[serde(rename = "visitor", skip_serializing_if = "Option::is_none")]
    pub visitor: Option<String>,
    /// Değerler: `"desktop"`, `"mobile"`, `"tablet"`.
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
}

/// `UpdateWebCampaignDtoVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: UpdateWebCampaignDtoVariantsItemDisplay,
}

/// `UpdateWebCampaignDtoVariantsItemDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebCampaignDtoVariantsItemDisplay {
    /// Zorunlu. Değerler: `"popup"`, `"banner_top"`, `"banner_bottom"`, `"slide_in"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    /// Değerler: `"left"`, `"right"`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    #[serde(rename = "max_width", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "survey_id", skip_serializing_if = "Option::is_none")]
    pub survey_id: Option<String>,
}

/// `UpdateWebhookDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebhookDto {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Değerler: `"message.queued"`, `"message.completed"`, `"message.failed"`, `"delivery.sent"`, `"delivery.delivered"`, `"delivery.failed"`, `"delivery.opened"`, `"delivery.clicked"`, `"delivery.bounced"`, `"delivery.complained"`, `"delivery.unsubscribed"`, `"in_app.impression"`, `"in_app.clicked"`, `"in_app.dismissed"`, `"subscription.created"`, `"subscription.unsubscribed"`, `"subscription.invalidated"`, `"user.created"`, `"user.updated"`, `"user.merged"`, `"user.deleted"`, `"event.received"`, `"conversion.created"`, `"campaign.scheduled"`, `"campaign.started"`, `"campaign.completed"`, `"journey.entered"`, `"journey.step_completed"`, `"journey.completed"`, `"journey.exited"`, `"conversation.created"`, `"conversation.message"`, `"conversation.assigned"`, `"conversation.resolved"`, `"audit.created"`, `"form.submitted"`, `"survey.responded"`, `"ad_audience.synced"`, `"ad_account.revoked"`, `"ad_budget.alert"`, `"ad_insights.imported"`.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "event_filters", skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<UpdateWebhookDtoEventFilters>,
    #[serde(rename = "batch", skip_serializing_if = "Option::is_none")]
    pub batch: Option<bool>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    /// Değerler: `"active"`, `"disabled"`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "body_template", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// Şablon çıktısının içerik türü (varsayılan application/json). Değerler: `"application/json"`, `"text/plain"`.
    #[serde(rename = "body_content_type", skip_serializing_if = "Option::is_none")]
    pub body_content_type: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<UpdateWebhookDtoFilters>,
}

/// `UpdateWebhookDtoEventFilters` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebhookDtoEventFilters {
    #[serde(rename = "event_names", skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
}

/// Olay filtresi: { match: all|any, conditions: [{ field, op, value? }] }.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebhookDtoFilters {
    /// Zorunlu. Değerler: `"all"`, `"any"`.
    #[serde(rename = "match", deserialize_with = "crate::de::null_default")]
    pub r#match: String,
    /// Zorunlu.
    #[serde(rename = "conditions", deserialize_with = "crate::de::null_default")]
    pub conditions: Vec<UpdateWebhookDtoFiltersConditionsItem>,
}

/// `UpdateWebhookDtoFiltersConditionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebhookDtoFiltersConditionsItem {
    /// Zorunlu.
    #[serde(rename = "field", deserialize_with = "crate::de::null_default")]
    pub field: String,
    /// Zorunlu. Değerler: `"eq"`, `"neq"`, `"in"`, `"not_in"`, `"starts_with"`, `"contains"`, `"exists"`, `"not_exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// `UpdateWebPersonalizationDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebPersonalizationDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<serde_json::Value>>,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<UpdateWebPersonalizationDtoVariantsItem>>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<UpdateWebPersonalizationDtoConditions>,
    #[serde(rename = "targeting", skip_serializing_if = "Option::is_none")]
    pub targeting: Option<UpdateWebPersonalizationDtoTargeting>,
    #[serde(rename = "goal", skip_serializing_if = "Option::is_none")]
    pub goal: Option<serde_json::Value>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
}

/// `UpdateWebPersonalizationDtoConditions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebPersonalizationDtoConditions {
    #[serde(rename = "url_rules", skip_serializing_if = "Option::is_none")]
    pub url_rules: Option<Vec<UpdateWebPersonalizationDtoConditionsUrlRulesItem>>,
    #[serde(rename = "utm_rules", skip_serializing_if = "Option::is_none")]
    pub utm_rules: Option<Vec<UpdateWebPersonalizationDtoConditionsUtmRulesItem>>,
    #[serde(rename = "visits", skip_serializing_if = "Option::is_none")]
    pub visits: Option<UpdateWebPersonalizationDtoConditionsVisits>,
}

/// `UpdateWebPersonalizationDtoConditionsUrlRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebPersonalizationDtoConditionsUrlRulesItem {
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"starts_with"`, `"not_contains"`, `"matches"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `UpdateWebPersonalizationDtoConditionsUtmRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebPersonalizationDtoConditionsUtmRulesItem {
    /// Zorunlu. Değerler: `"utm_source"`, `"utm_medium"`, `"utm_campaign"`, `"utm_term"`, `"utm_content"`.
    #[serde(rename = "param", deserialize_with = "crate::de::null_default")]
    pub param: String,
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// `UpdateWebPersonalizationDtoConditionsVisits` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebPersonalizationDtoConditionsVisits {
    /// Zorunlu. Değerler: `"gte"`, `"lte"`, `"eq"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: i64,
}

/// `UpdateWebPersonalizationDtoTargeting` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebPersonalizationDtoTargeting {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    /// Değerler: `"all"`, `"new"`, `"returning"`.
    #[serde(rename = "visitor", skip_serializing_if = "Option::is_none")]
    pub visitor: Option<String>,
    /// Değerler: `"desktop"`, `"mobile"`, `"tablet"`.
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
}

/// `UpdateWebPersonalizationDtoVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpdateWebPersonalizationDtoVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "changes", deserialize_with = "crate::de::null_default")]
    pub changes: Vec<serde_json::Value>,
}

/// `UpsertCatalogItemsDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpsertCatalogItemsDto {
    /// Zorunlu.
    #[serde(rename = "items", deserialize_with = "crate::de::null_default")]
    pub items: Vec<UpsertCatalogItemsDtoItemsItem>,
}

/// `UpsertCatalogItemsDtoItemsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpsertCatalogItemsDtoItemsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "image_url", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "sale_price", skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<f64>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(rename = "in_stock", skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

/// `UpsertUserDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UpsertUserDto {
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "subscriptions", skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<AddSubscriptionDto>>,
}

/// `UsageReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReport {
    /// Zorunlu.
    #[serde(rename = "org_id", deserialize_with = "crate::de::null_default")]
    pub org_id: String,
    /// Zorunlu.
    #[serde(rename = "plan_code", deserialize_with = "crate::de::null_default")]
    pub plan_code: String,
    /// Zorunlu.
    #[serde(rename = "period", deserialize_with = "crate::de::null_default")]
    pub period: UsageReportPeriod,
    /// Zorunlu.
    #[serde(rename = "metrics", deserialize_with = "crate::de::null_default")]
    pub metrics: Vec<UsageReportMetricsItem>,
    /// Zorunlu.
    #[serde(rename = "apps", deserialize_with = "crate::de::null_default")]
    pub apps: Vec<UsageReportAppsItem>,
    /// Zorunlu.
    #[serde(rename = "daily", deserialize_with = "crate::de::null_default")]
    pub daily: Vec<UsageReportDailyItem>,
    /// Zorunlu.
    #[serde(rename = "trialing", deserialize_with = "crate::de::null_default")]
    pub trialing: bool,
    /// Zorunlu.
    #[serde(rename = "trial_ends_at")]
    pub trial_ends_at: Option<String>,
    #[serde(rename = "overage", skip_serializing_if = "Option::is_none")]
    pub overage: Option<UsageReportOverage>,
}

/// `UsageReportAppsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReportAppsItem {
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "messages_sent", deserialize_with = "crate::de::null_default")]
    pub messages_sent: f64,
    /// Zorunlu.
    #[serde(rename = "active_subscriptions", deserialize_with = "crate::de::null_default")]
    pub active_subscriptions: f64,
    /// Zorunlu.
    #[serde(rename = "mau", deserialize_with = "crate::de::null_default")]
    pub mau: f64,
}

/// `UsageReportDailyItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReportDailyItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "messages_sent", deserialize_with = "crate::de::null_default")]
    pub messages_sent: f64,
    /// Zorunlu.
    #[serde(rename = "api_requests", deserialize_with = "crate::de::null_default")]
    pub api_requests: f64,
    /// Zorunlu.
    #[serde(rename = "events_ingested", deserialize_with = "crate::de::null_default")]
    pub events_ingested: f64,
}

/// `UsageReportMetricsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReportMetricsItem {
    /// Zorunlu.
    #[serde(rename = "metric", deserialize_with = "crate::de::null_default")]
    pub metric: String,
    /// Zorunlu.
    #[serde(rename = "used", deserialize_with = "crate::de::null_default")]
    pub used: f64,
    /// Zorunlu.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "percent")]
    pub percent: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "hard", deserialize_with = "crate::de::null_default")]
    pub hard: bool,
}

/// `UsageReportOverage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReportOverage {
    /// Zorunlu.
    #[serde(rename = "lines", deserialize_with = "crate::de::null_default")]
    pub lines: Vec<UsageReportOverageLinesItem>,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: UsageReportOverageTotals,
}

/// `UsageReportOverageLinesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReportOverageLinesItem {
    /// Zorunlu.
    #[serde(rename = "metric", deserialize_with = "crate::de::null_default")]
    pub metric: String,
    /// Zorunlu.
    #[serde(rename = "included", deserialize_with = "crate::de::null_default")]
    pub included: f64,
    /// Zorunlu.
    #[serde(rename = "used", deserialize_with = "crate::de::null_default")]
    pub used: f64,
    /// Zorunlu.
    #[serde(rename = "billable", deserialize_with = "crate::de::null_default")]
    pub billable: f64,
    /// Zorunlu.
    #[serde(rename = "amounts", deserialize_with = "crate::de::null_default")]
    pub amounts: UsageReportOverageLinesItemAmounts,
}

/// `UsageReportOverageLinesItemAmounts` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReportOverageLinesItemAmounts {
    /// Zorunlu.
    #[serde(rename = "USD", deserialize_with = "crate::de::null_default")]
    pub usd: f64,
    /// Zorunlu.
    #[serde(rename = "TRY", deserialize_with = "crate::de::null_default")]
    pub r#try: f64,
}

/// `UsageReportOverageTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReportOverageTotals {
    /// Zorunlu.
    #[serde(rename = "USD", deserialize_with = "crate::de::null_default")]
    pub usd: f64,
    /// Zorunlu.
    #[serde(rename = "TRY", deserialize_with = "crate::de::null_default")]
    pub r#try: f64,
}

/// `UsageReportPeriod` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsageReportPeriod {
    /// Zorunlu.
    #[serde(rename = "start", deserialize_with = "crate::de::null_default")]
    pub start: String,
    /// Zorunlu.
    #[serde(rename = "end", deserialize_with = "crate::de::null_default")]
    pub end: String,
}

/// `User` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct User {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "tags", deserialize_with = "crate::de::null_default")]
    pub tags: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "first_seen_at", deserialize_with = "crate::de::null_default")]
    pub first_seen_at: String,
    /// Zorunlu.
    #[serde(rename = "last_active_at", deserialize_with = "crate::de::null_default")]
    pub last_active_at: String,
    /// Zorunlu.
    #[serde(rename = "session_count", deserialize_with = "crate::de::null_default")]
    pub session_count: i64,
    /// Zorunlu.
    #[serde(rename = "usage_seconds", deserialize_with = "crate::de::null_default")]
    pub usage_seconds: i64,
    /// Zorunlu.
    #[serde(rename = "merged_into")]
    pub merged_into: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_ip")]
    pub last_ip: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_ip_at")]
    pub last_ip_at: Option<String>,
    /// Zorunlu. Değerler: `"positive"`, `"negative"`, `"neutral"`.
    #[serde(rename = "last_sentiment")]
    pub last_sentiment: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_sentiment_at")]
    pub last_sentiment_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu.
    #[serde(rename = "subscription_count", deserialize_with = "crate::de::null_default")]
    pub subscription_count: i64,
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", deserialize_with = "crate::de::null_default")]
    pub channels: Vec<String>,
}

/// `UserDetail` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDetail {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "tags", deserialize_with = "crate::de::null_default")]
    pub tags: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "first_seen_at", deserialize_with = "crate::de::null_default")]
    pub first_seen_at: String,
    /// Zorunlu.
    #[serde(rename = "last_active_at", deserialize_with = "crate::de::null_default")]
    pub last_active_at: String,
    /// Zorunlu.
    #[serde(rename = "session_count", deserialize_with = "crate::de::null_default")]
    pub session_count: i64,
    /// Zorunlu.
    #[serde(rename = "usage_seconds", deserialize_with = "crate::de::null_default")]
    pub usage_seconds: i64,
    /// Zorunlu.
    #[serde(rename = "merged_into")]
    pub merged_into: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_ip")]
    pub last_ip: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_ip_at")]
    pub last_ip_at: Option<String>,
    /// Zorunlu. Değerler: `"positive"`, `"negative"`, `"neutral"`.
    #[serde(rename = "last_sentiment")]
    pub last_sentiment: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_sentiment_at")]
    pub last_sentiment_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu.
    #[serde(rename = "subscription_count", deserialize_with = "crate::de::null_default")]
    pub subscription_count: i64,
    /// Zorunlu. Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`, `"telegram"`, `"viber"`.
    #[serde(rename = "channels", deserialize_with = "crate::de::null_default")]
    pub channels: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "aliases", deserialize_with = "crate::de::null_default")]
    pub aliases: Vec<UserDetailAliasesItem>,
    /// Zorunlu.
    #[serde(rename = "subscriptions", deserialize_with = "crate::de::null_default")]
    pub subscriptions: Vec<Subscription>,
}

/// `UserDetailAliasesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserDetailAliasesItem {
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `UserExport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExport {
    /// Zorunlu.
    #[serde(rename = "exported_at", deserialize_with = "crate::de::null_default")]
    pub exported_at: String,
    /// Zorunlu.
    #[serde(rename = "user", deserialize_with = "crate::de::null_default")]
    pub user: UserDetail,
    /// Zorunlu.
    #[serde(rename = "events", deserialize_with = "crate::de::null_default")]
    pub events: Vec<UserExportEventsItem>,
    /// Zorunlu.
    #[serde(rename = "derived", deserialize_with = "crate::de::null_default")]
    pub derived: UserExportDerived,
}

/// `UserExportDerived` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerived {
    /// Zorunlu.
    #[serde(rename = "wallet_passes", deserialize_with = "crate::de::null_default")]
    pub wallet_passes: Vec<UserExportDerivedWalletPassesItem>,
    /// Zorunlu.
    #[serde(rename = "rfm")]
    pub rfm: Option<UserExportDerivedRfm>,
    /// Zorunlu.
    #[serde(rename = "predictions", deserialize_with = "crate::de::null_default")]
    pub predictions: Vec<UserExportDerivedPredictionsItem>,
    /// Zorunlu.
    #[serde(rename = "channel_scores", deserialize_with = "crate::de::null_default")]
    pub channel_scores: Vec<UserExportDerivedChannelScoresItem>,
    /// Zorunlu.
    #[serde(rename = "decisions", deserialize_with = "crate::de::null_default")]
    pub decisions: Vec<UserExportDerivedDecisionsItem>,
    /// Zorunlu.
    #[serde(rename = "recommendations", deserialize_with = "crate::de::null_default")]
    pub recommendations: Vec<UserExportDerivedRecommendationsItem>,
    /// Zorunlu.
    #[serde(rename = "affinities", deserialize_with = "crate::de::null_default")]
    pub affinities: Vec<UserExportDerivedAffinitiesItem>,
    /// Zorunlu.
    #[serde(rename = "frequency_cap")]
    pub frequency_cap: Option<UserExportDerivedFrequencyCap>,
}

/// `UserExportDerivedAffinitiesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerivedAffinitiesItem {
    /// Zorunlu.
    #[serde(rename = "attribute", deserialize_with = "crate::de::null_default")]
    pub attribute: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
    /// Zorunlu.
    #[serde(rename = "score", deserialize_with = "crate::de::null_default")]
    pub score: f64,
    /// Zorunlu.
    #[serde(rename = "rank", deserialize_with = "crate::de::null_default")]
    pub rank: f64,
    /// Zorunlu.
    #[serde(rename = "computed_at", deserialize_with = "crate::de::null_default")]
    pub computed_at: String,
}

/// `UserExportDerivedChannelScoresItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerivedChannelScoresItem {
    /// Zorunlu.
    #[serde(rename = "channel", deserialize_with = "crate::de::null_default")]
    pub channel: String,
    /// Zorunlu.
    #[serde(rename = "sends", deserialize_with = "crate::de::null_default")]
    pub sends: f64,
    /// Zorunlu.
    #[serde(rename = "engaged", deserialize_with = "crate::de::null_default")]
    pub engaged: f64,
    /// Zorunlu.
    #[serde(rename = "score", deserialize_with = "crate::de::null_default")]
    pub score: f64,
    /// Zorunlu.
    #[serde(rename = "is_best", deserialize_with = "crate::de::null_default")]
    pub is_best: bool,
    /// Zorunlu.
    #[serde(rename = "computed_at", deserialize_with = "crate::de::null_default")]
    pub computed_at: String,
}

/// `UserExportDerivedDecisionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerivedDecisionsItem {
    /// Zorunlu.
    #[serde(rename = "agent", deserialize_with = "crate::de::null_default")]
    pub agent: String,
    /// Zorunlu.
    #[serde(rename = "arm_id", deserialize_with = "crate::de::null_default")]
    pub arm_id: String,
    /// Zorunlu.
    #[serde(rename = "context", deserialize_with = "crate::de::null_default")]
    pub context: String,
    /// Zorunlu.
    #[serde(rename = "assigned_at", deserialize_with = "crate::de::null_default")]
    pub assigned_at: String,
    /// Zorunlu.
    #[serde(rename = "rewarded_at")]
    pub rewarded_at: Option<String>,
}

/// `UserExportDerivedFrequencyCap` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerivedFrequencyCap {
    /// Zorunlu.
    #[serde(rename = "weekly_cap", deserialize_with = "crate::de::null_default")]
    pub weekly_cap: f64,
    /// Zorunlu.
    #[serde(rename = "engagement", deserialize_with = "crate::de::null_default")]
    pub engagement: f64,
    /// Zorunlu.
    #[serde(rename = "computed_at", deserialize_with = "crate::de::null_default")]
    pub computed_at: String,
}

/// `UserExportDerivedPredictionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerivedPredictionsItem {
    /// Zorunlu.
    #[serde(rename = "model", deserialize_with = "crate::de::null_default")]
    pub model: String,
    /// Zorunlu.
    #[serde(rename = "score", deserialize_with = "crate::de::null_default")]
    pub score: f64,
    /// Zorunlu.
    #[serde(rename = "computed_at", deserialize_with = "crate::de::null_default")]
    pub computed_at: String,
}

/// `UserExportDerivedRecommendationsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerivedRecommendationsItem {
    /// Zorunlu.
    #[serde(rename = "catalog_id", deserialize_with = "crate::de::null_default")]
    pub catalog_id: String,
    /// Zorunlu.
    #[serde(rename = "personalized", deserialize_with = "crate::de::null_default")]
    pub personalized: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "recent", deserialize_with = "crate::de::null_default")]
    pub recent: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "purchased", deserialize_with = "crate::de::null_default")]
    pub purchased: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "computed_at", deserialize_with = "crate::de::null_default")]
    pub computed_at: String,
}

/// `UserExportDerivedRfm` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerivedRfm {
    /// Zorunlu.
    #[serde(rename = "segment", deserialize_with = "crate::de::null_default")]
    pub segment: String,
    /// Zorunlu.
    #[serde(rename = "segment_since", deserialize_with = "crate::de::null_default")]
    pub segment_since: String,
    /// Zorunlu.
    #[serde(rename = "r", deserialize_with = "crate::de::null_default")]
    pub r: f64,
    /// Zorunlu.
    #[serde(rename = "f", deserialize_with = "crate::de::null_default")]
    pub f: f64,
    /// Zorunlu.
    #[serde(rename = "m")]
    pub m: Option<f64>,
    /// Zorunlu.
    #[serde(rename = "frequency", deserialize_with = "crate::de::null_default")]
    pub frequency: f64,
    /// Zorunlu.
    #[serde(rename = "monetary", deserialize_with = "crate::de::null_default")]
    pub monetary: f64,
    /// Zorunlu.
    #[serde(rename = "last_event_at", deserialize_with = "crate::de::null_default")]
    pub last_event_at: String,
    /// Zorunlu.
    #[serde(rename = "computed_at", deserialize_with = "crate::de::null_default")]
    pub computed_at: String,
}

/// `UserExportDerivedWalletPassesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportDerivedWalletPassesItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "template_id", deserialize_with = "crate::de::null_default")]
    pub template_id: String,
    /// Zorunlu.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "barcode_value", deserialize_with = "crate::de::null_default")]
    pub barcode_value: String,
    /// Zorunlu.
    #[serde(rename = "values", deserialize_with = "crate::de::null_default")]
    pub values: std::collections::HashMap<String, String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `UserExportEventsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserExportEventsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "subscription_id")]
    pub subscription_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "properties", deserialize_with = "crate::de::null_default")]
    pub properties: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /// Zorunlu. Değerler: `"sdk"`, `"api"`, `"system"`, `"import"`.
    #[serde(rename = "source", deserialize_with = "crate::de::null_default")]
    pub source: String,
    /// Zorunlu.
    #[serde(rename = "occurred_at", deserialize_with = "crate::de::null_default")]
    pub occurred_at: String,
}

/// `UserIdentity` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserIdentity {
    /// Zorunlu.
    #[serde(rename = "identity", deserialize_with = "crate::de::null_default")]
    pub identity: std::collections::HashMap<String, String>,
}

/// `UserJourneyRuns` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserJourneyRuns {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<UserJourneyRunsDataItem>,
}

/// `UserJourneyRunsDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserJourneyRunsDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "journey_id", deserialize_with = "crate::de::null_default")]
    pub journey_id: String,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu. Değerler: `"active"`, `"waiting"`, `"completed"`, `"exited"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "current_node_id")]
    pub current_node_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "next_action_at")]
    pub next_action_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "entered_at", deserialize_with = "crate::de::null_default")]
    pub entered_at: String,
    /// Zorunlu.
    #[serde(rename = "completed_at")]
    pub completed_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "exit_reason")]
    pub exit_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "goal_id")]
    pub goal_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "goal_reached_at")]
    pub goal_reached_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "context", deserialize_with = "crate::de::null_default")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "journey_name", deserialize_with = "crate::de::null_default")]
    pub journey_name: String,
}

/// `UserList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<User>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `UserPreferences` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserPreferences {
    /// Zorunlu.
    #[serde(rename = "user_id", deserialize_with = "crate::de::null_default")]
    pub user_id: String,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<UserPreferencesDataItem>,
    /// Zorunlu.
    #[serde(rename = "categories", deserialize_with = "crate::de::null_default")]
    pub categories: Vec<MessageCategory>,
    /// Zorunlu.
    #[serde(rename = "consents", deserialize_with = "crate::de::null_default")]
    pub consents: Vec<UserPreferencesConsentsItem>,
}

/// `UserPreferencesConsentsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserPreferencesConsentsItem {
    /// Zorunlu. Değerler: `"ads_personalization"`.
    #[serde(rename = "purpose", deserialize_with = "crate::de::null_default")]
    pub purpose: String,
    /// Zorunlu.
    #[serde(rename = "granted", deserialize_with = "crate::de::null_default")]
    pub granted: bool,
    /// Zorunlu.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Zorunlu.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

/// `UserPreferencesDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserPreferencesDataItem {
    /// Zorunlu.
    #[serde(rename = "category", deserialize_with = "crate::de::null_default")]
    pub category: String,
    /// Zorunlu.
    #[serde(rename = "channel")]
    pub channel: Option<String>,
    /// Zorunlu.
    #[serde(rename = "opted_in", deserialize_with = "crate::de::null_default")]
    pub opted_in: bool,
    /// Zorunlu.
    #[serde(rename = "explicit", deserialize_with = "crate::de::null_default")]
    pub explicit: bool,
    /// Zorunlu.
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Zorunlu.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

/// `UsersSearchBody` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsersSearchBody {
    /// Zorunlu. Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
    #[serde(rename = "definition", deserialize_with = "crate::de::null_default")]
    pub definition: UsersSearchBodyDefinition,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Filtre AST: all/any/none grupları ve { field, op, value } koşulları (docs/plan/05 §5.7).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UsersSearchBodyDefinition {
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

/// `VerifyAddressDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VerifyAddressDto {
    /// Zorunlu.
    #[serde(rename = "address", deserialize_with = "crate::de::null_default")]
    pub address: String,
}

/// `WalletConfig` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletConfig {
    /// Zorunlu.
    #[serde(rename = "apple", deserialize_with = "crate::de::null_default")]
    pub apple: WalletConfigApple,
    /// Zorunlu.
    #[serde(rename = "google", deserialize_with = "crate::de::null_default")]
    pub google: WalletConfigGoogle,
}

/// `WalletConfigApple` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletConfigApple {
    /// Zorunlu.
    #[serde(rename = "configured", deserialize_with = "crate::de::null_default")]
    pub configured: bool,
    /// Zorunlu.
    #[serde(rename = "pass_type_id")]
    pub pass_type_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "certificate_expires_at")]
    pub certificate_expires_at: Option<String>,
}

/// `WalletConfigGoogle` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletConfigGoogle {
    /// Zorunlu.
    #[serde(rename = "configured", deserialize_with = "crate::de::null_default")]
    pub configured: bool,
    /// Zorunlu.
    #[serde(rename = "issuer_id")]
    pub issuer_id: Option<String>,
}

/// `WalletPass` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletPass {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "template_id", deserialize_with = "crate::de::null_default")]
    pub template_id: String,
    /// Zorunlu.
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "barcode_value", deserialize_with = "crate::de::null_default")]
    pub barcode_value: String,
    /// Zorunlu.
    #[serde(rename = "values", deserialize_with = "crate::de::null_default")]
    pub values: std::collections::HashMap<String, String>,
    /// Zorunlu. Değerler: `"active"`, `"voided"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    /// Zorunlu.
    #[serde(rename = "apple_url", deserialize_with = "crate::de::null_default")]
    pub apple_url: String,
    /// Zorunlu.
    #[serde(rename = "google_url", deserialize_with = "crate::de::null_default")]
    pub google_url: String,
    /// Zorunlu.
    #[serde(rename = "apple_devices", deserialize_with = "crate::de::null_default")]
    pub apple_devices: i64,
    /// Zorunlu.
    #[serde(rename = "version", deserialize_with = "crate::de::null_default")]
    pub version: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `WalletPassPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletPassPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WalletPass>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
}

/// `WalletTemplate` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletTemplate {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "slug", deserialize_with = "crate::de::null_default")]
    pub slug: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"coupon"`, `"loyalty"`, `"event"`, `"generic"`.
    #[serde(rename = "kind", deserialize_with = "crate::de::null_default")]
    pub kind: String,
    /// Zorunlu. Değerler: `"active"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "design", deserialize_with = "crate::de::null_default")]
    pub design: WalletTemplateDesign,
    /// Zorunlu.
    #[serde(rename = "images", deserialize_with = "crate::de::null_default")]
    pub images: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "passes", deserialize_with = "crate::de::null_default")]
    pub passes: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `WalletTemplateDesign` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletTemplateDesign {
    /// Zorunlu.
    #[serde(rename = "title", deserialize_with = "crate::de::null_default")]
    pub title: String,
    /// Zorunlu.
    #[serde(rename = "description", deserialize_with = "crate::de::null_default")]
    pub description: String,
    /// Zorunlu.
    #[serde(rename = "organization", deserialize_with = "crate::de::null_default")]
    pub organization: String,
    /// Zorunlu.
    #[serde(rename = "background_color", deserialize_with = "crate::de::null_default")]
    pub background_color: String,
    /// Zorunlu.
    #[serde(rename = "foreground_color", deserialize_with = "crate::de::null_default")]
    pub foreground_color: String,
    /// Zorunlu.
    #[serde(rename = "label_color", deserialize_with = "crate::de::null_default")]
    pub label_color: String,
    /// Zorunlu. Değerler: `"qr"`, `"pdf417"`, `"aztec"`, `"code128"`.
    #[serde(rename = "barcode", deserialize_with = "crate::de::null_default")]
    pub barcode: String,
    /// Zorunlu.
    #[serde(rename = "fields", deserialize_with = "crate::de::null_default")]
    pub fields: Vec<WalletTemplateDesignFieldsItem>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "relevant_at", skip_serializing_if = "Option::is_none")]
    pub relevant_at: Option<String>,
}

/// `WalletTemplateDesignFieldsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletTemplateDesignFieldsItem {
    /// Zorunlu.
    #[serde(rename = "key", deserialize_with = "crate::de::null_default")]
    pub key: String,
    /// Zorunlu.
    #[serde(rename = "label", deserialize_with = "crate::de::null_default")]
    pub label: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
    /// Zorunlu. Değerler: `"header"`, `"primary"`, `"secondary"`, `"auxiliary"`, `"back"`.
    #[serde(rename = "section", deserialize_with = "crate::de::null_default")]
    pub section: String,
}

/// `WalletTemplateList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WalletTemplateList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WalletTemplate>,
}

/// `WarehouseConnectionResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseConnectionResult {
    /// Zorunlu.
    #[serde(rename = "ok", deserialize_with = "crate::de::null_default")]
    pub ok: bool,
    /// Zorunlu.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<WarehouseConnectionResultError>,
}

/// `WarehouseConnectionResultError` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseConnectionResultError {
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    /// Zorunlu.
    #[serde(rename = "message", deserialize_with = "crate::de::null_default")]
    pub message: String,
}

/// `WarehouseExport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseExport {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"bigquery"`, `"snowflake"`, `"s3_parquet"`, `"redshift"`, `"databricks"`.
    #[serde(rename = "destination", deserialize_with = "crate::de::null_default")]
    pub destination: String,
    /// Zorunlu.
    #[serde(rename = "config", deserialize_with = "crate::de::null_default")]
    pub config: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "credentials_hint", deserialize_with = "crate::de::null_default")]
    pub credentials_hint: String,
    /// Zorunlu. Değerler: `"deliveries"`, `"delivery_events"`, `"events"`, `"conversions"`, `"users"`, `"subscriptions"`.
    #[serde(rename = "tables", deserialize_with = "crate::de::null_default")]
    pub tables: Vec<String>,
    /// Zorunlu. Değerler: `"every_15_minutes"`, `"hourly"`, `"daily"`.
    #[serde(rename = "schedule", deserialize_with = "crate::de::null_default")]
    pub schedule: String,
    /// Zorunlu.
    #[serde(rename = "enabled", deserialize_with = "crate::de::null_default")]
    pub enabled: bool,
    /// Zorunlu. Değerler: `"active"`, `"paused"`, `"failing"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "consecutive_failures", deserialize_with = "crate::de::null_default")]
    pub consecutive_failures: i64,
    /// Zorunlu.
    #[serde(rename = "last_error")]
    pub last_error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "cursors", deserialize_with = "crate::de::null_default")]
    pub cursors: std::collections::HashMap<String, WarehouseExportCursorsValue>,
    /// Zorunlu.
    #[serde(rename = "schema_versions", deserialize_with = "crate::de::null_default")]
    pub schema_versions: std::collections::HashMap<String, i64>,
    /// Zorunlu.
    #[serde(rename = "last_run_at")]
    pub last_run_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_success_at")]
    pub last_success_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "next_run_at")]
    pub next_run_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `WarehouseExportCursorsValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseExportCursorsValue {
    /// Zorunlu.
    #[serde(rename = "ts", deserialize_with = "crate::de::null_default")]
    pub ts: String,
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
}

/// `WarehouseExportList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseExportList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WarehouseExport>,
}

/// `WarehouseExportRun` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseExportRun {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "export_id", deserialize_with = "crate::de::null_default")]
    pub export_id: String,
    /// Zorunlu. Değerler: `"schedule"`, `"manual"`.
    #[serde(rename = "trigger", deserialize_with = "crate::de::null_default")]
    pub trigger: String,
    /// Zorunlu. Değerler: `"queued"`, `"running"`, `"succeeded"`, `"failed"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "tables", deserialize_with = "crate::de::null_default")]
    pub tables: std::collections::HashMap<String, WarehouseExportRunTablesValue>,
    /// Zorunlu.
    #[serde(rename = "row_count", deserialize_with = "crate::de::null_default")]
    pub row_count: i64,
    /// Zorunlu.
    #[serde(rename = "attempt", deserialize_with = "crate::de::null_default")]
    pub attempt: i64,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "started_at")]
    pub started_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "duration_ms")]
    pub duration_ms: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `WarehouseExportRunPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseExportRunPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WarehouseExportRun>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `WarehouseExportRunTablesValue` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseExportRunTablesValue {
    /// Zorunlu.
    #[serde(rename = "rows", deserialize_with = "crate::de::null_default")]
    pub rows: i64,
    /// Zorunlu.
    #[serde(rename = "schema_version", deserialize_with = "crate::de::null_default")]
    pub schema_version: i64,
    /// Zorunlu.
    #[serde(rename = "from")]
    pub from: Option<WarehouseExportRunTablesValueFrom>,
    /// Zorunlu.
    #[serde(rename = "to")]
    pub to: Option<WarehouseExportRunTablesValueTo>,
    /// Zorunlu.
    #[serde(rename = "objects", deserialize_with = "crate::de::null_default")]
    pub objects: i64,
}

/// `WarehouseExportRunTablesValueFrom` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseExportRunTablesValueFrom {
    /// Zorunlu.
    #[serde(rename = "ts", deserialize_with = "crate::de::null_default")]
    pub ts: String,
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
}

/// `WarehouseExportRunTablesValueTo` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseExportRunTablesValueTo {
    /// Zorunlu.
    #[serde(rename = "ts", deserialize_with = "crate::de::null_default")]
    pub ts: String,
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
}

/// `WarehouseTables` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseTables {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WarehouseTablesDataItem>,
}

/// `WarehouseTablesDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseTablesDataItem {
    /// Zorunlu. Değerler: `"deliveries"`, `"delivery_events"`, `"events"`, `"conversions"`, `"users"`, `"subscriptions"`.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"incremental"`, `"snapshot"`.
    #[serde(rename = "mode", deserialize_with = "crate::de::null_default")]
    pub mode: String,
    /// Zorunlu.
    #[serde(rename = "schema_version", deserialize_with = "crate::de::null_default")]
    pub schema_version: i64,
    /// Zorunlu.
    #[serde(rename = "cursor_column")]
    pub cursor_column: Option<String>,
    /// Zorunlu.
    #[serde(rename = "columns", deserialize_with = "crate::de::null_default")]
    pub columns: Vec<WarehouseTablesDataItemColumnsItem>,
}

/// `WarehouseTablesDataItemColumnsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WarehouseTablesDataItemColumnsItem {
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "nullable", deserialize_with = "crate::de::null_default")]
    pub nullable: bool,
}

/// `WebCampaign` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaign {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "priority", deserialize_with = "crate::de::null_default")]
    pub priority: i64,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: WebCampaignDisplay,
    /// Zorunlu.
    #[serde(rename = "variants", deserialize_with = "crate::de::null_default")]
    pub variants: Vec<WebCampaignVariantsItem>,
    /// Zorunlu.
    #[serde(rename = "ab_test", deserialize_with = "crate::de::null_default")]
    pub ab_test: WebCampaignAbTest,
    /// Zorunlu.
    #[serde(rename = "trigger")]
    pub trigger: Option<serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "conditions", deserialize_with = "crate::de::null_default")]
    pub conditions: WebCampaignConditions,
    /// Zorunlu.
    #[serde(rename = "targeting", deserialize_with = "crate::de::null_default")]
    pub targeting: WebCampaignTargeting,
    /// Zorunlu.
    #[serde(rename = "frequency", deserialize_with = "crate::de::null_default")]
    pub frequency: WebCampaignFrequency,
    /// Zorunlu.
    #[serde(rename = "start_at")]
    pub start_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: WebCampaignStats,
    /// Zorunlu.
    #[serde(rename = "variant_stats", deserialize_with = "crate::de::null_default")]
    pub variant_stats: Vec<WebCampaignVariantStatsItem>,
    /// Zorunlu.
    #[serde(rename = "variant_significance")]
    pub variant_significance: Option<WebCampaignVariantSignificance>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `WebCampaignAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignAbTest {
    /// Zorunlu.
    #[serde(rename = "winner_variant_id")]
    pub winner_variant_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "decided_at")]
    pub decided_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "decided_by")]
    pub decided_by: Option<String>,
}

/// `WebCampaignConditions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignConditions {
    #[serde(rename = "url_rules", skip_serializing_if = "Option::is_none")]
    pub url_rules: Option<Vec<WebCampaignConditionsUrlRulesItem>>,
    #[serde(rename = "utm_rules", skip_serializing_if = "Option::is_none")]
    pub utm_rules: Option<Vec<WebCampaignConditionsUtmRulesItem>>,
    #[serde(rename = "visits", skip_serializing_if = "Option::is_none")]
    pub visits: Option<WebCampaignConditionsVisits>,
}

/// `WebCampaignConditionsUrlRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignConditionsUrlRulesItem {
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"starts_with"`, `"not_contains"`, `"matches"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `WebCampaignConditionsUtmRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignConditionsUtmRulesItem {
    /// Zorunlu. Değerler: `"utm_source"`, `"utm_medium"`, `"utm_campaign"`, `"utm_term"`, `"utm_content"`.
    #[serde(rename = "param", deserialize_with = "crate::de::null_default")]
    pub param: String,
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// `WebCampaignConditionsVisits` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignConditionsVisits {
    /// Zorunlu. Değerler: `"gte"`, `"lte"`, `"eq"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: i64,
}

/// `WebCampaignDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignDisplay {
    /// Zorunlu. Değerler: `"popup"`, `"banner_top"`, `"banner_bottom"`, `"slide_in"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    /// Değerler: `"left"`, `"right"`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    #[serde(rename = "max_width", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "survey_id", skip_serializing_if = "Option::is_none")]
    pub survey_id: Option<String>,
}

/// `WebCampaignFrequency` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignFrequency {
    #[serde(rename = "max_per_session", skip_serializing_if = "Option::is_none")]
    pub max_per_session: Option<i64>,
    #[serde(rename = "max_per_day", skip_serializing_if = "Option::is_none")]
    pub max_per_day: Option<i64>,
    #[serde(rename = "max_total", skip_serializing_if = "Option::is_none")]
    pub max_total: Option<i64>,
    #[serde(rename = "hide_after_close_days", skip_serializing_if = "Option::is_none")]
    pub hide_after_close_days: Option<i64>,
    #[serde(rename = "stop_after_conversion", skip_serializing_if = "Option::is_none")]
    pub stop_after_conversion: Option<bool>,
}

/// `WebCampaignPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WebCampaign>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `WebCampaignReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignReport {
    /// Zorunlu.
    #[serde(rename = "campaign_id", deserialize_with = "crate::de::null_default")]
    pub campaign_id: String,
    /// Zorunlu.
    #[serde(rename = "days", deserialize_with = "crate::de::null_default")]
    pub days: i64,
    /// Zorunlu.
    #[serde(rename = "daily", deserialize_with = "crate::de::null_default")]
    pub daily: Vec<WebCampaignReportDailyItem>,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: WebCampaignReportTotals,
}

/// `WebCampaignReportDailyItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignReportDailyItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: i64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: i64,
    /// Zorunlu.
    #[serde(rename = "closes", deserialize_with = "crate::de::null_default")]
    pub closes: i64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
}

/// `WebCampaignReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignReportTotals {
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: i64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: i64,
    /// Zorunlu.
    #[serde(rename = "closes", deserialize_with = "crate::de::null_default")]
    pub closes: i64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "unique_visitors", deserialize_with = "crate::de::null_default")]
    pub unique_visitors: i64,
    /// Zorunlu.
    #[serde(rename = "ctr", deserialize_with = "crate::de::null_default")]
    pub ctr: f64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
}

/// `WebCampaignStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignStats {
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: i64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: i64,
    /// Zorunlu.
    #[serde(rename = "closes", deserialize_with = "crate::de::null_default")]
    pub closes: i64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "unique_visitors", deserialize_with = "crate::de::null_default")]
    pub unique_visitors: i64,
    /// Zorunlu.
    #[serde(rename = "ctr", deserialize_with = "crate::de::null_default")]
    pub ctr: f64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
}

/// `WebCampaignStatusDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignStatusDto {
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
}

/// `WebCampaignTargeting` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignTargeting {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    /// Değerler: `"all"`, `"new"`, `"returning"`.
    #[serde(rename = "visitor", skip_serializing_if = "Option::is_none")]
    pub visitor: Option<String>,
    /// Değerler: `"desktop"`, `"mobile"`, `"tablet"`.
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
}

/// `WebCampaignVariantSignificance` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignVariantSignificance {
    /// Zorunlu.
    #[serde(rename = "leader")]
    pub leader: Option<String>,
    /// Zorunlu.
    #[serde(rename = "p_value", deserialize_with = "crate::de::null_default")]
    pub p_value: f64,
    /// Zorunlu.
    #[serde(rename = "significant", deserialize_with = "crate::de::null_default")]
    pub significant: bool,
}

/// `WebCampaignVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "display", deserialize_with = "crate::de::null_default")]
    pub display: WebCampaignVariantsItemDisplay,
}

/// `WebCampaignVariantsItemDisplay` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignVariantsItemDisplay {
    /// Zorunlu. Değerler: `"popup"`, `"banner_top"`, `"banner_bottom"`, `"slide_in"`, `"fullscreen"`.
    #[serde(rename = "layout", deserialize_with = "crate::de::null_default")]
    pub layout: String,
    /// Değerler: `"left"`, `"right"`.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(rename = "dismissible", skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    #[serde(rename = "auto_dismiss_seconds", skip_serializing_if = "Option::is_none")]
    pub auto_dismiss_seconds: Option<i64>,
    #[serde(rename = "max_width", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "blocks", deserialize_with = "crate::de::null_default")]
    pub blocks: Vec<serde_json::Value>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "survey_id", skip_serializing_if = "Option::is_none")]
    pub survey_id: Option<String>,
}

/// `WebCampaignVariantStatsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebCampaignVariantStatsItem {
    /// Zorunlu.
    #[serde(rename = "variant_id", deserialize_with = "crate::de::null_default")]
    pub variant_id: String,
    /// Zorunlu.
    #[serde(rename = "impressions", deserialize_with = "crate::de::null_default")]
    pub impressions: i64,
    /// Zorunlu.
    #[serde(rename = "clicks", deserialize_with = "crate::de::null_default")]
    pub clicks: i64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "ctr", deserialize_with = "crate::de::null_default")]
    pub ctr: f64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
}

/// `Webhook` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Webhook {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "events", deserialize_with = "crate::de::null_default")]
    pub events: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "event_filters", deserialize_with = "crate::de::null_default")]
    pub event_filters: WebhookEventFilters,
    /// Zorunlu.
    #[serde(rename = "batch", deserialize_with = "crate::de::null_default")]
    pub batch: bool,
    /// Zorunlu. Değerler: `"active"`, `"failing"`, `"disabled"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "disabled_reason")]
    pub disabled_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "failure_count", deserialize_with = "crate::de::null_default")]
    pub failure_count: i64,
    /// Zorunlu.
    #[serde(rename = "failing_since")]
    pub failing_since: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_success_at")]
    pub last_success_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_failure_at")]
    pub last_failure_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "header_names", deserialize_with = "crate::de::null_default")]
    pub header_names: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "previous_secret_expires_at")]
    pub previous_secret_expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "body_template")]
    pub body_template: Option<String>,
    /// Zorunlu. Değerler: `"application/json"`, `"text/plain"`.
    #[serde(rename = "body_content_type", deserialize_with = "crate::de::null_default")]
    pub body_content_type: String,
    /// Zorunlu.
    #[serde(rename = "filters")]
    pub filters: Option<WebhookFilters>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `WebhookDeliveryDetail` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookDeliveryDetail {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "endpoint_id", deserialize_with = "crate::de::null_default")]
    pub endpoint_id: String,
    /// Zorunlu.
    #[serde(rename = "event_type", deserialize_with = "crate::de::null_default")]
    pub event_type: String,
    /// Zorunlu.
    #[serde(rename = "event_id", deserialize_with = "crate::de::null_default")]
    pub event_id: String,
    /// Zorunlu.
    #[serde(rename = "subject_id")]
    pub subject_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "attempt", deserialize_with = "crate::de::null_default")]
    pub attempt: i64,
    /// Zorunlu. Değerler: `"pending"`, `"success"`, `"failed"`, `"dead"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "response_status")]
    pub response_status: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "response_body_excerpt")]
    pub response_body_excerpt: Option<String>,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "duration_ms")]
    pub duration_ms: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "next_retry_at")]
    pub next_retry_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "delivered_at")]
    pub delivered_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "replay_of")]
    pub replay_of: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "payload", deserialize_with = "crate::de::null_default")]
    pub payload: std::collections::HashMap<String, serde_json::Value>,
}

/// `WebhookDeliveryPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookDeliveryPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WebhookDeliveryPageDataItem>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `WebhookDeliveryPageDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookDeliveryPageDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "endpoint_id", deserialize_with = "crate::de::null_default")]
    pub endpoint_id: String,
    /// Zorunlu.
    #[serde(rename = "event_type", deserialize_with = "crate::de::null_default")]
    pub event_type: String,
    /// Zorunlu.
    #[serde(rename = "event_id", deserialize_with = "crate::de::null_default")]
    pub event_id: String,
    /// Zorunlu.
    #[serde(rename = "subject_id")]
    pub subject_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "attempt", deserialize_with = "crate::de::null_default")]
    pub attempt: i64,
    /// Zorunlu. Değerler: `"pending"`, `"success"`, `"failed"`, `"dead"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "response_status")]
    pub response_status: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "response_body_excerpt")]
    pub response_body_excerpt: Option<String>,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Zorunlu.
    #[serde(rename = "duration_ms")]
    pub duration_ms: Option<i64>,
    /// Zorunlu.
    #[serde(rename = "next_retry_at")]
    pub next_retry_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "delivered_at")]
    pub delivered_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "replay_of")]
    pub replay_of: Option<String>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `WebhookEventFilters` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookEventFilters {
    #[serde(rename = "event_names", skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
}

/// `WebhookFilters` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookFilters {
    /// Zorunlu. Değerler: `"all"`, `"any"`.
    #[serde(rename = "match", deserialize_with = "crate::de::null_default")]
    pub r#match: String,
    /// Zorunlu.
    #[serde(rename = "conditions", deserialize_with = "crate::de::null_default")]
    pub conditions: Vec<WebhookFiltersConditionsItem>,
}

/// `WebhookFiltersConditionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookFiltersConditionsItem {
    /// Zorunlu.
    #[serde(rename = "field", deserialize_with = "crate::de::null_default")]
    pub field: String,
    /// Zorunlu. Değerler: `"eq"`, `"neq"`, `"in"`, `"not_in"`, `"starts_with"`, `"contains"`, `"exists"`, `"not_exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// `WebhookList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<Webhook>,
}

/// `WebhookPreviewDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookPreviewDto {
    /// Zorunlu. Değerler: `"message.queued"`, `"message.completed"`, `"message.failed"`, `"delivery.sent"`, `"delivery.delivered"`, `"delivery.failed"`, `"delivery.opened"`, `"delivery.clicked"`, `"delivery.bounced"`, `"delivery.complained"`, `"delivery.unsubscribed"`, `"in_app.impression"`, `"in_app.clicked"`, `"in_app.dismissed"`, `"subscription.created"`, `"subscription.unsubscribed"`, `"subscription.invalidated"`, `"user.created"`, `"user.updated"`, `"user.merged"`, `"user.deleted"`, `"event.received"`, `"conversion.created"`, `"campaign.scheduled"`, `"campaign.started"`, `"campaign.completed"`, `"journey.entered"`, `"journey.step_completed"`, `"journey.completed"`, `"journey.exited"`, `"conversation.created"`, `"conversation.message"`, `"conversation.assigned"`, `"conversation.resolved"`, `"audit.created"`, `"form.submitted"`, `"survey.responded"`, `"ad_audience.synced"`, `"ad_account.revoked"`, `"ad_budget.alert"`, `"ad_insights.imported"`, `"webhook.test"`.
    #[serde(rename = "event_type", deserialize_with = "crate::de::null_default")]
    pub event_type: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "body_template", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// Şablon çıktısının içerik türü (varsayılan application/json). Değerler: `"application/json"`, `"text/plain"`.
    #[serde(rename = "body_content_type", skip_serializing_if = "Option::is_none")]
    pub body_content_type: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<WebhookPreviewDtoFilters>,
    #[serde(rename = "batch", skip_serializing_if = "Option::is_none")]
    pub batch: Option<bool>,
}

/// Olay filtresi: { match: all|any, conditions: [{ field, op, value? }] }.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookPreviewDtoFilters {
    /// Zorunlu. Değerler: `"all"`, `"any"`.
    #[serde(rename = "match", deserialize_with = "crate::de::null_default")]
    pub r#match: String,
    /// Zorunlu.
    #[serde(rename = "conditions", deserialize_with = "crate::de::null_default")]
    pub conditions: Vec<WebhookPreviewDtoFiltersConditionsItem>,
}

/// `WebhookPreviewDtoFiltersConditionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookPreviewDtoFiltersConditionsItem {
    /// Zorunlu.
    #[serde(rename = "field", deserialize_with = "crate::de::null_default")]
    pub field: String,
    /// Zorunlu. Değerler: `"eq"`, `"neq"`, `"in"`, `"not_in"`, `"starts_with"`, `"contains"`, `"exists"`, `"not_exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// `WebhookPreviewResult` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookPreviewResult {
    /// Zorunlu.
    #[serde(rename = "payload", deserialize_with = "crate::de::null_default")]
    pub payload: std::collections::HashMap<String, serde_json::Value>,
    /// Zorunlu. Olay filtreden geçer mi (geçmezse gönderilmez).
    #[serde(rename = "matched", deserialize_with = "crate::de::null_default")]
    pub matched: bool,
    /// Zorunlu.
    #[serde(rename = "failed_conditions", deserialize_with = "crate::de::null_default")]
    pub failed_conditions: Vec<WebhookPreviewResultFailedConditionsItem>,
    /// Zorunlu.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Zorunlu.
    #[serde(rename = "content_type", deserialize_with = "crate::de::null_default")]
    pub content_type: String,
    /// Zorunlu.
    #[serde(rename = "error")]
    pub error: Option<WebhookPreviewResultError>,
}

/// `WebhookPreviewResultError` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookPreviewResultError {
    /// Zorunlu.
    #[serde(rename = "code", deserialize_with = "crate::de::null_default")]
    pub code: String,
    /// Zorunlu.
    #[serde(rename = "message", deserialize_with = "crate::de::null_default")]
    pub message: String,
}

/// `WebhookPreviewResultFailedConditionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookPreviewResultFailedConditionsItem {
    /// Zorunlu.
    #[serde(rename = "field", deserialize_with = "crate::de::null_default")]
    pub field: String,
    /// Zorunlu.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// `WebhookWithSecret` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookWithSecret {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "url", deserialize_with = "crate::de::null_default")]
    pub url: String,
    /// Zorunlu.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Zorunlu.
    #[serde(rename = "events", deserialize_with = "crate::de::null_default")]
    pub events: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "event_filters", deserialize_with = "crate::de::null_default")]
    pub event_filters: WebhookWithSecretEventFilters,
    /// Zorunlu.
    #[serde(rename = "batch", deserialize_with = "crate::de::null_default")]
    pub batch: bool,
    /// Zorunlu. Değerler: `"active"`, `"failing"`, `"disabled"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "disabled_reason")]
    pub disabled_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "failure_count", deserialize_with = "crate::de::null_default")]
    pub failure_count: i64,
    /// Zorunlu.
    #[serde(rename = "failing_since")]
    pub failing_since: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_success_at")]
    pub last_success_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "last_failure_at")]
    pub last_failure_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "header_names", deserialize_with = "crate::de::null_default")]
    pub header_names: Vec<String>,
    /// Zorunlu.
    #[serde(rename = "previous_secret_expires_at")]
    pub previous_secret_expires_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "body_template")]
    pub body_template: Option<String>,
    /// Zorunlu. Değerler: `"application/json"`, `"text/plain"`.
    #[serde(rename = "body_content_type", deserialize_with = "crate::de::null_default")]
    pub body_content_type: String,
    /// Zorunlu.
    #[serde(rename = "filters")]
    pub filters: Option<WebhookWithSecretFilters>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
    /// Zorunlu. Yalnızca oluşturma ve rotasyon yanıtında bir kez döner.
    #[serde(rename = "secret", deserialize_with = "crate::de::null_default")]
    pub secret: String,
}

/// `WebhookWithSecretEventFilters` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookWithSecretEventFilters {
    #[serde(rename = "event_names", skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
}

/// `WebhookWithSecretFilters` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookWithSecretFilters {
    /// Zorunlu. Değerler: `"all"`, `"any"`.
    #[serde(rename = "match", deserialize_with = "crate::de::null_default")]
    pub r#match: String,
    /// Zorunlu.
    #[serde(rename = "conditions", deserialize_with = "crate::de::null_default")]
    pub conditions: Vec<WebhookWithSecretFiltersConditionsItem>,
}

/// `WebhookWithSecretFiltersConditionsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebhookWithSecretFiltersConditionsItem {
    /// Zorunlu.
    #[serde(rename = "field", deserialize_with = "crate::de::null_default")]
    pub field: String,
    /// Zorunlu. Değerler: `"eq"`, `"neq"`, `"in"`, `"not_in"`, `"starts_with"`, `"contains"`, `"exists"`, `"not_exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// `WebPersonalization` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalization {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "priority", deserialize_with = "crate::de::null_default")]
    pub priority: i64,
    /// Zorunlu.
    #[serde(rename = "changes", deserialize_with = "crate::de::null_default")]
    pub changes: Vec<serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "variants", deserialize_with = "crate::de::null_default")]
    pub variants: Vec<WebPersonalizationVariantsItem>,
    /// Zorunlu.
    #[serde(rename = "ab_test", deserialize_with = "crate::de::null_default")]
    pub ab_test: WebPersonalizationAbTest,
    /// Zorunlu.
    #[serde(rename = "conditions", deserialize_with = "crate::de::null_default")]
    pub conditions: WebPersonalizationConditions,
    /// Zorunlu.
    #[serde(rename = "targeting", deserialize_with = "crate::de::null_default")]
    pub targeting: WebPersonalizationTargeting,
    /// Zorunlu.
    #[serde(rename = "goal")]
    pub goal: Option<serde_json::Value>,
    /// Zorunlu.
    #[serde(rename = "start_at")]
    pub start_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "stats", deserialize_with = "crate::de::null_default")]
    pub stats: WebPersonalizationStats,
    /// Zorunlu.
    #[serde(rename = "variant_stats", deserialize_with = "crate::de::null_default")]
    pub variant_stats: Vec<WebPersonalizationVariantStatsItem>,
    /// Zorunlu.
    #[serde(rename = "variant_significance")]
    pub variant_significance: Option<WebPersonalizationVariantSignificance>,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "updated_at", deserialize_with = "crate::de::null_default")]
    pub updated_at: String,
}

/// `WebPersonalizationAbTest` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationAbTest {
    /// Zorunlu.
    #[serde(rename = "winner_variant_id")]
    pub winner_variant_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "decided_at")]
    pub decided_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "decided_by")]
    pub decided_by: Option<String>,
}

/// `WebPersonalizationConditions` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationConditions {
    #[serde(rename = "url_rules", skip_serializing_if = "Option::is_none")]
    pub url_rules: Option<Vec<WebPersonalizationConditionsUrlRulesItem>>,
    #[serde(rename = "utm_rules", skip_serializing_if = "Option::is_none")]
    pub utm_rules: Option<Vec<WebPersonalizationConditionsUtmRulesItem>>,
    #[serde(rename = "visits", skip_serializing_if = "Option::is_none")]
    pub visits: Option<WebPersonalizationConditionsVisits>,
}

/// `WebPersonalizationConditionsUrlRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationConditionsUrlRulesItem {
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"starts_with"`, `"not_contains"`, `"matches"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: String,
}

/// `WebPersonalizationConditionsUtmRulesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationConditionsUtmRulesItem {
    /// Zorunlu. Değerler: `"utm_source"`, `"utm_medium"`, `"utm_campaign"`, `"utm_term"`, `"utm_content"`.
    #[serde(rename = "param", deserialize_with = "crate::de::null_default")]
    pub param: String,
    /// Zorunlu. Değerler: `"equals"`, `"contains"`, `"exists"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// `WebPersonalizationConditionsVisits` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationConditionsVisits {
    /// Zorunlu. Değerler: `"gte"`, `"lte"`, `"eq"`.
    #[serde(rename = "op", deserialize_with = "crate::de::null_default")]
    pub op: String,
    /// Zorunlu.
    #[serde(rename = "value", deserialize_with = "crate::de::null_default")]
    pub value: i64,
}

/// `WebPersonalizationPage` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationPage {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WebPersonalization>,
    /// Zorunlu.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// Zorunlu.
    #[serde(rename = "has_more", deserialize_with = "crate::de::null_default")]
    pub has_more: bool,
}

/// `WebPersonalizationReport` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationReport {
    /// Zorunlu.
    #[serde(rename = "personalization_id", deserialize_with = "crate::de::null_default")]
    pub personalization_id: String,
    /// Zorunlu.
    #[serde(rename = "days", deserialize_with = "crate::de::null_default")]
    pub days: i64,
    /// Zorunlu.
    #[serde(rename = "daily", deserialize_with = "crate::de::null_default")]
    pub daily: Vec<WebPersonalizationReportDailyItem>,
    /// Zorunlu.
    #[serde(rename = "totals", deserialize_with = "crate::de::null_default")]
    pub totals: WebPersonalizationReportTotals,
}

/// `WebPersonalizationReportDailyItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationReportDailyItem {
    /// Zorunlu.
    #[serde(rename = "date", deserialize_with = "crate::de::null_default")]
    pub date: String,
    /// Zorunlu.
    #[serde(rename = "visitors", deserialize_with = "crate::de::null_default")]
    pub visitors: i64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
}

/// `WebPersonalizationReportTotals` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationReportTotals {
    /// Zorunlu.
    #[serde(rename = "visitors", deserialize_with = "crate::de::null_default")]
    pub visitors: i64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
}

/// `WebPersonalizationStats` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationStats {
    /// Zorunlu.
    #[serde(rename = "visitors", deserialize_with = "crate::de::null_default")]
    pub visitors: i64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
}

/// `WebPersonalizationStatusDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationStatusDto {
    /// Zorunlu. Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
}

/// `WebPersonalizationTargeting` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationTargeting {
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
    /// Değerler: `"all"`, `"new"`, `"returning"`.
    #[serde(rename = "visitor", skip_serializing_if = "Option::is_none")]
    pub visitor: Option<String>,
    /// Değerler: `"desktop"`, `"mobile"`, `"tablet"`.
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
}

/// `WebPersonalizationVariantSignificance` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationVariantSignificance {
    /// Zorunlu.
    #[serde(rename = "leader")]
    pub leader: Option<String>,
    /// Zorunlu.
    #[serde(rename = "p_value", deserialize_with = "crate::de::null_default")]
    pub p_value: f64,
    /// Zorunlu.
    #[serde(rename = "significant", deserialize_with = "crate::de::null_default")]
    pub significant: bool,
}

/// `WebPersonalizationVariantsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationVariantsItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "weight_percent", deserialize_with = "crate::de::null_default")]
    pub weight_percent: i64,
    /// Zorunlu.
    #[serde(rename = "changes", deserialize_with = "crate::de::null_default")]
    pub changes: Vec<serde_json::Value>,
}

/// `WebPersonalizationVariantStatsItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WebPersonalizationVariantStatsItem {
    /// Zorunlu.
    #[serde(rename = "visitors", deserialize_with = "crate::de::null_default")]
    pub visitors: i64,
    /// Zorunlu.
    #[serde(rename = "conversions", deserialize_with = "crate::de::null_default")]
    pub conversions: i64,
    /// Zorunlu.
    #[serde(rename = "conversion_rate", deserialize_with = "crate::de::null_default")]
    pub conversion_rate: f64,
    /// Zorunlu.
    #[serde(rename = "variant_id", deserialize_with = "crate::de::null_default")]
    pub variant_id: String,
}

/// `WhatsAppTemplateList` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WhatsAppTemplateList {
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WhatsAppTemplateListDataItem>,
}

/// `WhatsAppTemplateListDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WhatsAppTemplateListDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    /// Zorunlu.
    #[serde(rename = "category", deserialize_with = "crate::de::null_default")]
    pub category: String,
    /// Zorunlu.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "components", deserialize_with = "crate::de::null_default")]
    pub components: Vec<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "body_params", deserialize_with = "crate::de::null_default")]
    pub body_params: i64,
    /// Zorunlu.
    #[serde(rename = "rejected_reason")]
    pub rejected_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "quality_score")]
    pub quality_score: Option<String>,
    /// Zorunlu.
    #[serde(rename = "synced_at", deserialize_with = "crate::de::null_default")]
    pub synced_at: String,
}

/// `WhatsAppTemplateSync` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WhatsAppTemplateSync {
    /// Zorunlu.
    #[serde(rename = "total", deserialize_with = "crate::de::null_default")]
    pub total: i64,
    /// Zorunlu.
    #[serde(rename = "removed", deserialize_with = "crate::de::null_default")]
    pub removed: i64,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: Vec<WhatsAppTemplateSyncDataItem>,
}

/// `WhatsAppTemplateSyncDataItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WhatsAppTemplateSyncDataItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// Zorunlu.
    #[serde(rename = "name", deserialize_with = "crate::de::null_default")]
    pub name: String,
    /// Zorunlu.
    #[serde(rename = "language", deserialize_with = "crate::de::null_default")]
    pub language: String,
    /// Zorunlu.
    #[serde(rename = "category", deserialize_with = "crate::de::null_default")]
    pub category: String,
    /// Zorunlu.
    #[serde(rename = "status", deserialize_with = "crate::de::null_default")]
    pub status: String,
    /// Zorunlu.
    #[serde(rename = "components", deserialize_with = "crate::de::null_default")]
    pub components: Vec<std::collections::HashMap<String, serde_json::Value>>,
    /// Zorunlu.
    #[serde(rename = "body_params", deserialize_with = "crate::de::null_default")]
    pub body_params: i64,
    /// Zorunlu.
    #[serde(rename = "rejected_reason")]
    pub rejected_reason: Option<String>,
    /// Zorunlu.
    #[serde(rename = "quality_score")]
    pub quality_score: Option<String>,
    /// Zorunlu.
    #[serde(rename = "synced_at", deserialize_with = "crate::de::null_default")]
    pub synced_at: String,
}

/// `ZapierHook` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ZapierHook {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "event", deserialize_with = "crate::de::null_default")]
    pub event: String,
    /// Zorunlu.
    #[serde(rename = "event_name")]
    pub event_name: Option<String>,
    /// Zorunlu.
    #[serde(rename = "target_url", deserialize_with = "crate::de::null_default")]
    pub target_url: String,
    /// Zorunlu.
    #[serde(rename = "last_delivered_at")]
    pub last_delivered_at: Option<String>,
    /// Zorunlu.
    #[serde(rename = "failure_count", deserialize_with = "crate::de::null_default")]
    pub failure_count: i64,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
}

/// `ZapierMe` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ZapierMe {
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "app_name", deserialize_with = "crate::de::null_default")]
    pub app_name: String,
    /// Zorunlu.
    #[serde(rename = "org_id", deserialize_with = "crate::de::null_default")]
    pub org_id: String,
    /// Zorunlu.
    #[serde(rename = "scopes", deserialize_with = "crate::de::null_default")]
    pub scopes: Vec<String>,
}

/// `ZapierSamplesItem` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ZapierSamplesItem {
    /// Zorunlu.
    #[serde(rename = "id", deserialize_with = "crate::de::null_default")]
    pub id: String,
    /// Zorunlu.
    #[serde(rename = "type", deserialize_with = "crate::de::null_default")]
    pub r#type: String,
    /// Zorunlu.
    #[serde(rename = "created_at", deserialize_with = "crate::de::null_default")]
    pub created_at: String,
    /// Zorunlu.
    #[serde(rename = "app_id", deserialize_with = "crate::de::null_default")]
    pub app_id: String,
    /// Zorunlu.
    #[serde(rename = "data", deserialize_with = "crate::de::null_default")]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// `ZapierSubscribeDto` şeması.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ZapierSubscribeDto {
    /// Zorunlu.
    #[serde(rename = "target_url", deserialize_with = "crate::de::null_default")]
    pub target_url: String,
    /// Zorunlu. Değerler: `"subscription.created"`, `"subscription.unsubscribed"`, `"event.received"`, `"delivery.delivered"`, `"delivery.opened"`, `"delivery.clicked"`, `"user.created"`, `"user.updated"`, `"conversion.created"`, `"campaign.completed"`.
    #[serde(rename = "event", deserialize_with = "crate::de::null_default")]
    pub event: String,
    #[serde(rename = "event_name", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
}
