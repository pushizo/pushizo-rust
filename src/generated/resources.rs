// Bu dosya api/openapi.yaml'dan sdk/scripts/generate-server-sdks.mjs ile üretilmiştir; elle düzenlemeyin.

//! API kaynak servisleri (`Client::notifications()`, `Client::segments()` …).

#![allow(clippy::all, missing_docs, unused_mut, unused_imports)]

use crate::client::Operation;
use crate::models::*;
use crate::multipart::Multipart;
use crate::{Client, FileUpload, Paginator, RequestOptions, Result};

/// SDK'nın üretildiği OpenAPI sözleşmesinin sürümü.
pub const API_VERSION: &str = "1.1.0";

impl Client {
    /// Bildirim gönderme, durum, iptal, teslimatlar ve geçmiş dışa aktarımı.
    pub fn notifications(&self) -> NotificationsService<'_> {
        NotificationsService::new(self)
    }

    /// Kullanıcı oluşturma/güncelleme (tag, özellik), arama ve abonelik ekleme.
    pub fn users(&self) -> UsersService<'_> {
        UsersService::new(self)
    }

    /// Kullanıcı kimlikleri (alias): ekleme ve kaldırma.
    pub fn aliases(&self) -> AliasesService<'_> {
        AliasesService::new(self)
    }

    /// Abonelik (cihaz/e-posta/SMS) okuma, güncelleme, silme ve sahip atama.
    pub fn subscriptions(&self) -> SubscriptionsService<'_> {
        SubscriptionsService::new(self)
    }

    /// Özel olay gönderimi ve olay tanımları.
    pub fn events(&self) -> EventsService<'_> {
        EventsService::new(self)
    }

    /// Segment yönetimi, önizleme ve kitle tahmini.
    pub fn segments(&self) -> SegmentsService<'_> {
        SegmentsService::new(self)
    }

    /// Şablon yönetimi, sürümler ve önizleme.
    pub fn templates(&self) -> TemplatesService<'_> {
        TemplatesService::new(self)
    }

    /// Kampanya yaşam döngüsü (zamanlama, gönderim, duraklatma, onay) ve istatistikler.
    pub fn campaigns(&self) -> CampaignsService<'_> {
        CampaignsService::new(self)
    }

    /// Uygulama içi (in-app) mesaj yönetimi ve durum değişiklikleri.
    pub fn in_app_messages(&self) -> InAppMessagesService<'_> {
        InAppMessagesService::new(self)
    }

    /// Kullanıcı bildirim kutusu (inbox): listeleme, okundu işaretleme ve arşivleme.
    pub fn inbox(&self) -> InboxService<'_> {
        InboxService::new(self)
    }

    /// E-posta/SMS gönderim engeli (suppression) listesi.
    pub fn suppressions(&self) -> SuppressionsService<'_> {
        SuppressionsService::new(self)
    }

    /// Veri dışa aktarma işleri.
    pub fn exports(&self) -> ExportsService<'_> {
        ExportsService::new(self)
    }

    /// Veri içe aktarma işleri (CSV, multipart).
    pub fn imports(&self) -> ImportsService<'_> {
        ImportsService::new(self)
    }

    /// Giden webhook uç noktaları, teslimatlar ve yeniden gönderim.
    pub fn webhooks(&self) -> WebhooksService<'_> {
        WebhooksService::new(self)
    }

    /// Kanal (APNs, FCM, Web Push, e-posta, SMS) yapılandırması ve doğrulaması.
    pub fn channels(&self) -> ChannelsService<'_> {
        ChannelsService::new(self)
    }

    /// Analitik raporları.
    pub fn analytics(&self) -> AnalyticsService<'_> {
        AnalyticsService::new(self)
    }

    /// App listesi ve ayarları.
    pub fn apps(&self) -> AppsService<'_> {
        AppsService::new(self)
    }

    /// App API anahtarları: oluşturma, kapsam/IP izin listesi güncelleme, rotasyon ve iptal.
    pub fn api_keys(&self) -> ApiKeysService<'_> {
        ApiKeysService::new(self)
    }

    /// Organizasyon, üyeler ve denetim kayıtları.
    pub fn organization(&self) -> OrganizationService<'_> {
        OrganizationService::new(self)
    }

    /// `ads` kaynağı.
    pub fn ads(&self) -> AdsService<'_> {
        AdsService::new(self)
    }

    /// `cannedResponses` kaynağı.
    pub fn canned_responses(&self) -> CannedResponsesService<'_> {
        CannedResponsesService::new(self)
    }

    /// `catalogs` kaynağı.
    pub fn catalogs(&self) -> CatalogsService<'_> {
        CatalogsService::new(self)
    }

    /// `categories` kaynağı.
    pub fn categories(&self) -> CategoriesService<'_> {
        CategoriesService::new(self)
    }

    /// `channelScores` kaynağı.
    pub fn channel_scores(&self) -> ChannelScoresService<'_> {
        ChannelScoresService::new(self)
    }

    /// `contentTables` kaynağı.
    pub fn content_tables(&self) -> ContentTablesService<'_> {
        ContentTablesService::new(self)
    }

    /// `conversations` kaynağı.
    pub fn conversations(&self) -> ConversationsService<'_> {
        ConversationsService::new(self)
    }

    /// `dataFeeds` kaynağı.
    pub fn data_feeds(&self) -> DataFeedsService<'_> {
        DataFeedsService::new(self)
    }

    /// `decisionAgents` kaynağı.
    pub fn decision_agents(&self) -> DecisionAgentsService<'_> {
        DecisionAgentsService::new(self)
    }

    /// `emailDeliverability` kaynağı.
    pub fn email_deliverability(&self) -> EmailDeliverabilityService<'_> {
        EmailDeliverabilityService::new(self)
    }

    /// `featureFlags` kaynağı.
    pub fn feature_flags(&self) -> FeatureFlagsService<'_> {
        FeatureFlagsService::new(self)
    }

    /// `forms` kaynağı.
    pub fn forms(&self) -> FormsService<'_> {
        FormsService::new(self)
    }

    /// `geofences` kaynağı.
    pub fn geofences(&self) -> GeofencesService<'_> {
        GeofencesService::new(self)
    }

    /// `globalControl` kaynağı.
    pub fn global_control(&self) -> GlobalControlService<'_> {
        GlobalControlService::new(self)
    }

    /// `identityKeys` kaynağı.
    pub fn identity_keys(&self) -> IdentityKeysService<'_> {
        IdentityKeysService::new(self)
    }

    /// `integrations` kaynağı.
    pub fn integrations(&self) -> IntegrationsService<'_> {
        IntegrationsService::new(self)
    }

    /// `journeys` kaynağı.
    pub fn journeys(&self) -> JourneysService<'_> {
        JourneysService::new(self)
    }

    /// `landingPages` kaynağı.
    pub fn landing_pages(&self) -> LandingPagesService<'_> {
        LandingPagesService::new(self)
    }

    /// `liveActivities` kaynağı.
    pub fn live_activities(&self) -> LiveActivitiesService<'_> {
        LiveActivitiesService::new(self)
    }

    /// `mcp` kaynağı.
    pub fn mcp(&self) -> McpService<'_> {
        McpService::new(self)
    }

    /// `predictions` kaynağı.
    pub fn predictions(&self) -> PredictionsService<'_> {
        PredictionsService::new(self)
    }

    /// `preferences` kaynağı.
    pub fn preferences(&self) -> PreferencesService<'_> {
        PreferencesService::new(self)
    }

    /// `promoCodeLists` kaynağı.
    pub fn promo_code_lists(&self) -> PromoCodeListsService<'_> {
        PromoCodeListsService::new(self)
    }

    /// `recommendations` kaynağı.
    pub fn recommendations(&self) -> RecommendationsService<'_> {
        RecommendationsService::new(self)
    }

    /// `reverseEtl` kaynağı.
    pub fn reverse_etl(&self) -> ReverseEtlService<'_> {
        ReverseEtlService::new(self)
    }

    /// `rfm` kaynağı.
    pub fn rfm(&self) -> RfmService<'_> {
        RfmService::new(self)
    }

    /// `smsKeywords` kaynağı.
    pub fn sms_keywords(&self) -> SmsKeywordsService<'_> {
        SmsKeywordsService::new(self)
    }

    /// `surveys` kaynağı.
    pub fn surveys(&self) -> SurveysService<'_> {
        SurveysService::new(self)
    }

    /// `verify` kaynağı.
    pub fn verify(&self) -> VerifyService<'_> {
        VerifyService::new(self)
    }

    /// `wallet` kaynağı.
    pub fn wallet(&self) -> WalletService<'_> {
        WalletService::new(self)
    }

    /// `warehouseExports` kaynağı.
    pub fn warehouse_exports(&self) -> WarehouseExportsService<'_> {
        WarehouseExportsService::new(self)
    }

    /// `webCampaigns` kaynağı.
    pub fn web_campaigns(&self) -> WebCampaignsService<'_> {
        WebCampaignsService::new(self)
    }

    /// `webPersonalizations` kaynağı.
    pub fn web_personalizations(&self) -> WebPersonalizationsService<'_> {
        WebPersonalizationsService::new(self)
    }

    /// `whatsapp` kaynağı.
    pub fn whatsapp(&self) -> WhatsappService<'_> {
        WhatsappService::new(self)
    }
}

/// Bildirim gönderme, durum, iptal, teslimatlar ve geçmiş dışa aktarımı.
/// `Client::notifications()` ile alınır.
#[derive(Debug, Clone)]
pub struct NotificationsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `NotificationsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NotificationsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"campaign"`, `"api"`, `"journey"`, `"test"`.
    pub source: Option<String>,
    /// Değerler: `"campaign"`, `"api"`, `"journey"`, `"test"`.
    pub kind: Option<String>,
    /// Değerler: `"queued"`, `"processing"`, `"completed"`, `"cancelled"`, `"failed"`.
    pub status: Option<String>,
    /// Değerler: `"created_at"`, `"-created_at"`.
    pub sort: Option<String>,
    pub label: Option<String>,
}

impl NotificationsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.source {
            op.query("source", v);
        }
        if let Some(v) = &self.kind {
            op.query("kind", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.sort {
            op.query("sort", v);
        }
        if let Some(v) = &self.label {
            op.query("label", v);
        }
    }
}

/// `NotificationsService::deliveries` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NotificationsDeliveriesParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"pending"`, `"sent"`, `"delivered"`, `"failed"`, `"bounced"`, `"suppressed"`, `"expired"`, `"cancelled"`.
    pub status: Option<String>,
}

impl NotificationsDeliveriesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

impl<'a> NotificationsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Bildirim oluştur (202, asenkron teslimat)
    ///
    /// `POST /v1/notifications`
    ///
    /// `Idempotency-Key` başlığı gönderilir: `RequestOptions::idempotency_key`, gövdedeki `idempotency_key` veya otomatik UUID.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateNotificationDto) -> Result<CreateNotificationResult> {
        let mut op = Operation::new("POST", "/v1/notifications");
        op.idempotent = true;
        op.body_app_id = true;
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Mesaj listesi (app_id zorunlu)
    ///
    /// `GET /v1/notifications`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&NotificationsListParams>) -> Result<MessageList> {
        let mut op = Operation::new("GET", "/v1/notifications");
        op.query_app_id = true;
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&NotificationsListParams>) -> Paginator<'a, Message> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Kaydedilmemiş hedef için alıcı sayısı (gönderim yapmaz)
    ///
    /// `POST /v1/notifications/estimate`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn estimate(&self, body: &EstimateAudienceDto) -> Result<EstimateAudienceResult> {
        let mut op = Operation::new("POST", "/v1/notifications/estimate");
        op.body_app_id = true;
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Mesaj durumu ve sayaçları
    ///
    /// `GET /v1/notifications/{messageId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, message_id: &str) -> Result<Message> {
        let mut op = Operation::new("GET", "/v1/notifications/{messageId}");
        op.query_app_id = true;
        op.path_param("messageId", message_id);
        self.client.execute(op, &self.opts)
    }

    /// Zamanlanmış/kuyruktaki mesajı iptal et
    ///
    /// `DELETE /v1/notifications/{messageId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn cancel(&self, message_id: &str) -> Result<Message> {
        let mut op = Operation::new("DELETE", "/v1/notifications/{messageId}");
        op.query_app_id = true;
        op.path_param("messageId", message_id);
        self.client.execute(op, &self.opts)
    }

    /// Abone bazlı teslimat sonuçları (cursor)
    ///
    /// `GET /v1/notifications/{messageId}/deliveries`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn deliveries(&self, message_id: &str, params: Option<&NotificationsDeliveriesParams>) -> Result<DeliveryList> {
        let mut op = Operation::new("GET", "/v1/notifications/{messageId}/deliveries");
        op.query_app_id = true;
        op.path_param("messageId", message_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `deliveries` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn deliveries_all(&self, message_id: &str, params: Option<&NotificationsDeliveriesParams>) -> Paginator<'a, Delivery> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let message_id = message_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.deliveries(&message_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Bildirim geçmişi CSV işi (teslimatlar; tamamlanınca download_url)
    ///
    /// `GET /v1/notifications/{messageId}/history`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn history(&self, message_id: &str) -> Result<Export> {
        let mut op = Operation::new("GET", "/v1/notifications/{messageId}/history");
        op.query_app_id = true;
        op.path_param("messageId", message_id);
        self.client.execute(op, &self.opts)
    }

    /// Mesaj olay CSV işi (gönderildi/teslim/açıldı/tıklandı/bounce…; tamamlanınca download_url)
    ///
    /// `POST /v1/notifications/{messageId}/export_events`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn export_events(&self, message_id: &str) -> Result<Export> {
        let mut op = Operation::new("POST", "/v1/notifications/{messageId}/export_events");
        op.query_app_id = true;
        op.path_param("messageId", message_id);
        self.client.execute(op, &self.opts)
    }
}

/// Kullanıcı oluşturma/güncelleme (tag, özellik), arama ve abonelik ekleme.
/// `Client::users()` ile alınır.
#[derive(Debug, Clone)]
pub struct UsersService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `UsersService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UsersListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"last_active_at"`, `"-last_active_at"`.
    pub sort: Option<String>,
    pub q: Option<String>,
    pub ip: Option<String>,
    pub tag: Option<String>,
    pub last_active_after: Option<String>,
    pub last_active_before: Option<String>,
}

impl UsersListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.sort {
            op.query("sort", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
        if let Some(v) = &self.ip {
            op.query("ip", v);
        }
        if let Some(v) = &self.tag {
            op.query("tag", v);
        }
        if let Some(v) = &self.last_active_after {
            op.query("last_active_after", v);
        }
        if let Some(v) = &self.last_active_before {
            op.query("last_active_before", v);
        }
    }
}

impl<'a> UsersService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Kullanıcı listesi
    ///
    /// `GET /v1/apps/{appId}/users`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&UsersListParams>) -> Result<UserList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/users");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&UsersListParams>) -> Paginator<'a, User> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Kullanıcı oluştur/güncelle (external_id ile, isteğe bağlı aboneliklerle)
    ///
    /// `POST /v1/apps/{appId}/users`
    ///
    /// `Idempotency-Key` başlığı gönderilir: `RequestOptions::idempotency_key`, gövdedeki `idempotency_key` veya otomatik UUID.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn upsert(&self, body: &UpsertUserDto) -> Result<UserDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/users");
        op.idempotent = true;
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Segment tanımı ile kullanıcı arama
    ///
    /// `POST /v1/apps/{appId}/users/search`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn search(&self, body: &UsersSearchBody) -> Result<UserList> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/users/search");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Alias ile kullanıcı (hs_id, external_id veya özel etiket)
    ///
    /// `GET /v1/apps/{appId}/users/by/{label}/{value}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_by_alias(&self, label: &str, value: &str) -> Result<UserDetail> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/users/by/{label}/{value}");
        op.path_param("label", label);
        op.path_param("value", value);
        self.client.execute(op, &self.opts)
    }

    /// Tag/property merge-patch ve deltas
    ///
    /// `PATCH /v1/apps/{appId}/users/by/{label}/{value}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, label: &str, value: &str, body: &PatchUserDto) -> Result<UserDetail> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/users/by/{label}/{value}");
        op.path_param("label", label);
        op.path_param("value", value);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcıyı sil (abonelikler silinir, olaylar anonimleşir)
    ///
    /// `DELETE /v1/apps/{appId}/users/by/{label}/{value}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, label: &str, value: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/users/by/{label}/{value}");
        op.path_param("label", label);
        op.path_param("value", value);
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcının tüm kimlikleri (hs_id, external_id, özel alias'lar)
    ///
    /// `GET /v1/apps/{appId}/users/by/{label}/{value}/identity`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_identity(&self, label: &str, value: &str) -> Result<UserIdentity> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/users/by/{label}/{value}/identity");
        op.path_param("label", label);
        op.path_param("value", value);
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcı verisini dışa aktar (KVKK/GDPR taşınabilirlik): profil, alias, abonelik, son olaylar
    ///
    /// `GET /v1/apps/{appId}/users/by/{label}/{value}/export`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn export(&self, label: &str, value: &str) -> Result<UserExport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/users/by/{label}/{value}/export");
        op.path_param("label", label);
        op.path_param("value", value);
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcıya abonelik ekle
    ///
    /// `POST /v1/apps/{appId}/users/by/{label}/{value}/subscriptions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn add_subscription(&self, label: &str, value: &str, body: &AddSubscriptionDto) -> Result<Subscription> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/users/by/{label}/{value}/subscriptions");
        op.path_param("label", label);
        op.path_param("value", value);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// Kullanıcı kimlikleri (alias): ekleme ve kaldırma.
/// `Client::aliases()` ile alınır.
#[derive(Debug, Clone)]
pub struct AliasesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> AliasesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Alias ekle
    ///
    /// `PATCH /v1/apps/{appId}/users/by/{label}/{value}/identity`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn add(&self, label: &str, value: &str, body: &AddAliasDto) -> Result<UserDetail> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/users/by/{label}/{value}/identity");
        op.path_param("label", label);
        op.path_param("value", value);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Alias kaldır
    ///
    /// `DELETE /v1/apps/{appId}/users/by/{label}/{value}/identity/{aliasLabel}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, label: &str, value: &str, alias_label: &str) -> Result<UserDetail> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/users/by/{label}/{value}/identity/{aliasLabel}");
        op.path_param("label", label);
        op.path_param("value", value);
        op.path_param("aliasLabel", alias_label);
        self.client.execute(op, &self.opts)
    }
}

/// Abonelik (cihaz/e-posta/SMS) okuma, güncelleme, silme ve sahip atama.
/// `Client::subscriptions()` ile alınır.
#[derive(Debug, Clone)]
pub struct SubscriptionsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> SubscriptionsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Abonelik detayı
    ///
    /// `GET /v1/apps/{appId}/subscriptions/{subscriptionId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, subscription_id: &str) -> Result<Subscription> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/subscriptions/{subscriptionId}");
        op.path_param("subscriptionId", subscription_id);
        self.client.execute(op, &self.opts)
    }

    /// Abonelik güncelle
    ///
    /// `PATCH /v1/apps/{appId}/subscriptions/{subscriptionId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, subscription_id: &str, body: &PatchSubscriptionDto) -> Result<Subscription> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/subscriptions/{subscriptionId}");
        op.path_param("subscriptionId", subscription_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Aboneliği sil
    ///
    /// `DELETE /v1/apps/{appId}/subscriptions/{subscriptionId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, subscription_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/subscriptions/{subscriptionId}");
        op.path_param("subscriptionId", subscription_id);
        self.client.execute(op, &self.opts)
    }

    /// Aboneliğin sahibi kullanıcının kimlikleri
    ///
    /// `GET /v1/apps/{appId}/subscriptions/{subscriptionId}/user/identity`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_user_identity(&self, subscription_id: &str) -> Result<UserIdentity> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/subscriptions/{subscriptionId}/user/identity");
        op.path_param("subscriptionId", subscription_id);
        self.client.execute(op, &self.opts)
    }

    /// Token ile abonelik güncelle (token_type: ios_push, android_push, hms_push, web_push, email, sms veya eski sağlayıcı adları iOSPush, AndroidPush, HuaweiPush, ChromePush, FirefoxPush, SafariPush, Email, SMS)
    ///
    /// `PATCH /v1/apps/{appId}/subscriptions_by_token/{tokenType}/{token}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_by_token(&self, token_type: &str, token: &str, body: &PatchSubscriptionDto) -> Result<Subscription> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/subscriptions_by_token/{tokenType}/{token}");
        op.path_param("tokenType", token_type);
        op.path_param("token", token);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Aboneliği external_id ile kullanıcıya bağla
    ///
    /// `POST /v1/apps/{appId}/subscriptions/{subscriptionId}/owner`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn set_owner(&self, subscription_id: &str, body: &SetOwnerDto) -> Result<Subscription> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/subscriptions/{subscriptionId}/owner");
        op.path_param("subscriptionId", subscription_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// Özel olay gönderimi ve olay tanımları.
/// `Client::events()` ile alınır.
#[derive(Debug, Clone)]
pub struct EventsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `EventsService::list_definitions` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventsListDefinitionsParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl EventsListDefinitionsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> EventsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Toplu olay gönder (en fazla 500)
    ///
    /// `POST /v1/apps/{appId}/events`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn send(&self, body: &ApiEventsDto) -> Result<EventsAccepted> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/events");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Olay tanımları
    ///
    /// `GET /v1/apps/{appId}/events/definitions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_definitions(&self, params: Option<&EventsListDefinitionsParams>) -> Result<EventDefinitionList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/events/definitions");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list_definitions` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_definitions_all(&self, params: Option<&EventsListDefinitionsParams>) -> Paginator<'a, EventDefinition> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list_definitions(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Olay detayı
    ///
    /// `GET /v1/apps/{appId}/events/definitions/{name}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_definition(&self, name: &str) -> Result<EventDefinitionDetail> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/events/definitions/{name}");
        op.path_param("name", name);
        self.client.execute(op, &self.opts)
    }

    /// Görünen ad / dönüşüm işareti
    ///
    /// `PATCH /v1/apps/{appId}/events/definitions/{name}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_definition(&self, name: &str, body: &PatchEventDefinitionDto) -> Result<EventDefinition> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/events/definitions/{name}");
        op.path_param("name", name);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// Segment yönetimi, önizleme ve kitle tahmini.
/// `Client::segments()` ile alınır.
#[derive(Debug, Clone)]
pub struct SegmentsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `SegmentsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SegmentsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub q: Option<String>,
}

impl SegmentsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
    }
}

/// `SegmentsService::estimate` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SegmentsEstimateParams {
    /// true: önbelleği atla, tam sayım Değerler: `"true"`, `"false"`.
    pub force: Option<String>,
}

impl SegmentsEstimateParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.force {
            op.query("force", v);
        }
    }
}

impl<'a> SegmentsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Segment listesi
    ///
    /// `GET /v1/apps/{appId}/segments`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&SegmentsListParams>) -> Result<SegmentPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/segments");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&SegmentsListParams>) -> Paginator<'a, Segment> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Segment oluştur
    ///
    /// `POST /v1/apps/{appId}/segments`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &SegmentsCreateBody) -> Result<Segment> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/segments");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Tanım ile sayım ve örnek kullanıcılar
    ///
    /// `POST /v1/apps/{appId}/segments/preview`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn preview(&self, body: &SegmentsPreviewBody) -> Result<SegmentPreview> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/segments/preview");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Segment detayı
    ///
    /// `GET /v1/apps/{appId}/segments/{segmentId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, segment_id: &str) -> Result<Segment> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/segments/{segmentId}");
        op.path_param("segmentId", segment_id);
        self.client.execute(op, &self.opts)
    }

    /// Segment güncelle
    ///
    /// `PATCH /v1/apps/{appId}/segments/{segmentId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, segment_id: &str, body: &SegmentsUpdateBody) -> Result<Segment> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/segments/{segmentId}");
        op.path_param("segmentId", segment_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Segment sil
    ///
    /// `DELETE /v1/apps/{appId}/segments/{segmentId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, segment_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/segments/{segmentId}");
        op.path_param("segmentId", segment_id);
        self.client.execute(op, &self.opts)
    }

    /// Segmenti kullanan kampanyalar
    ///
    /// `GET /v1/apps/{appId}/segments/{segmentId}/campaigns`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn used_by(&self, segment_id: &str) -> Result<SegmentCampaigns> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/segments/{segmentId}/campaigns");
        op.path_param("segmentId", segment_id);
        self.client.execute(op, &self.opts)
    }

    /// Tahmini sayım
    ///
    /// `POST /v1/apps/{appId}/segments/{segmentId}/estimate`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn estimate(&self, segment_id: &str, params: Option<&SegmentsEstimateParams>) -> Result<SegmentEstimate> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/segments/{segmentId}/estimate");
        op.path_param("segmentId", segment_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// Şablon yönetimi, sürümler ve önizleme.
/// `Client::templates()` ile alınır.
#[derive(Debug, Clone)]
pub struct TemplatesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `TemplatesService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemplatesListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub q: Option<String>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`.
    pub channel: Option<String>,
}

impl TemplatesListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
        if let Some(v) = &self.channel {
            op.query("channel", v);
        }
    }
}

impl<'a> TemplatesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Şablon listesi
    ///
    /// `GET /v1/apps/{appId}/templates`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&TemplatesListParams>) -> Result<TemplatePage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/templates");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&TemplatesListParams>) -> Paginator<'a, Template> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Şablon oluştur
    ///
    /// `POST /v1/apps/{appId}/templates`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateTemplateDto) -> Result<Template> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/templates");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Şablon detayı
    ///
    /// `GET /v1/apps/{appId}/templates/{templateId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, template_id: &str) -> Result<Template> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/templates/{templateId}");
        op.path_param("templateId", template_id);
        self.client.execute(op, &self.opts)
    }

    /// Şablon güncelle
    ///
    /// `PATCH /v1/apps/{appId}/templates/{templateId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, template_id: &str, body: &UpdateTemplateDto) -> Result<Template> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/templates/{templateId}");
        op.path_param("templateId", template_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Şablon sil
    ///
    /// `DELETE /v1/apps/{appId}/templates/{templateId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, template_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/templates/{templateId}");
        op.path_param("templateId", template_id);
        self.client.execute(op, &self.opts)
    }

    /// Şablon analitiği
    ///
    /// `GET /v1/apps/{appId}/templates/{templateId}/stats`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn stats(&self, template_id: &str) -> Result<TemplateStats> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/templates/{templateId}/stats");
        op.path_param("templateId", template_id);
        self.client.execute(op, &self.opts)
    }

    /// Şablonu aynı organizasyondaki başka bir app'e kopyala
    ///
    /// `POST /v1/apps/{appId}/templates/{templateId}/copy_to_app`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn copy_to_app(&self, template_id: &str, body: &CopyTemplateDto) -> Result<Template> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/templates/{templateId}/copy_to_app");
        op.path_param("templateId", template_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Sürüm geçmişi
    ///
    /// `GET /v1/apps/{appId}/templates/{templateId}/versions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn versions(&self, template_id: &str) -> Result<TemplateVersionList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/templates/{templateId}/versions");
        op.path_param("templateId", template_id);
        self.client.execute(op, &self.opts)
    }

    /// Şablonu örnek verilerle render et
    ///
    /// `POST /v1/apps/{appId}/templates/{templateId}/render`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn render(&self, template_id: &str, body: &RenderTemplateDto) -> Result<RenderPreview> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/templates/{templateId}/render");
        op.path_param("templateId", template_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// Kampanya yaşam döngüsü (zamanlama, gönderim, duraklatma, onay) ve istatistikler.
/// `Client::campaigns()` ile alınır.
#[derive(Debug, Clone)]
pub struct CampaignsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `CampaignsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CampaignsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub q: Option<String>,
    /// Değerler: `"draft"`, `"scheduled"`, `"running"`, `"paused"`, `"completed"`, `"cancelled"`, `"failed"`.
    pub status: Option<String>,
    /// Değerler: `"one_time"`, `"scheduled"`, `"recurring"`, `"automated"`, `"transactional"`.
    pub r#type: Option<String>,
    /// Değerler: `"ios_push"`, `"android_push"`, `"hms_push"`, `"adm_push"`, `"wns_push"`, `"web_push"`, `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`.
    pub channel: Option<String>,
    pub label: Option<String>,
}

impl CampaignsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.r#type {
            op.query("type", v);
        }
        if let Some(v) = &self.channel {
            op.query("channel", v);
        }
        if let Some(v) = &self.label {
            op.query("label", v);
        }
    }
}

/// `CampaignsService::schedule` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CampaignsScheduleParams {
    /// Değerler: `"true"`, `"false"`.
    pub force: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub preflight_ack: Option<String>,
}

impl CampaignsScheduleParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.force {
            op.query("force", v);
        }
        if let Some(v) = &self.preflight_ack {
            op.query("preflight_ack", v);
        }
    }
}

/// `CampaignsService::send` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CampaignsSendParams {
    /// Değerler: `"true"`, `"false"`.
    pub force: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub preflight_ack: Option<String>,
}

impl CampaignsSendParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.force {
            op.query("force", v);
        }
        if let Some(v) = &self.preflight_ack {
            op.query("preflight_ack", v);
        }
    }
}

/// `CampaignsService::preflight` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CampaignsPreflightParams {
    /// Değerler: `"true"`, `"false"`.
    pub check_links: Option<String>,
}

impl CampaignsPreflightParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.check_links {
            op.query("check_links", v);
        }
    }
}

/// `CampaignsService::approve` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CampaignsApproveParams {
    /// Değerler: `"true"`, `"false"`.
    pub force: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub preflight_ack: Option<String>,
}

impl CampaignsApproveParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.force {
            op.query("force", v);
        }
        if let Some(v) = &self.preflight_ack {
            op.query("preflight_ack", v);
        }
    }
}

/// `CampaignsService::deliveries` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CampaignsDeliveriesParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"pending"`, `"sent"`, `"delivered"`, `"failed"`, `"bounced"`, `"suppressed"`, `"expired"`, `"cancelled"`.
    pub status: Option<String>,
}

impl CampaignsDeliveriesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

impl<'a> CampaignsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Kampanya listesi
    ///
    /// `GET /v1/apps/{appId}/campaigns`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&CampaignsListParams>) -> Result<CampaignPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/campaigns");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&CampaignsListParams>) -> Paginator<'a, Campaign> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Kampanya oluştur
    ///
    /// `POST /v1/apps/{appId}/campaigns`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CampaignsCreateBody) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kampanya detayı
    ///
    /// `GET /v1/apps/{appId}/campaigns/{campaignId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, campaign_id: &str) -> Result<Campaign> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/campaigns/{campaignId}");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// Kampanya güncelle
    ///
    /// `PATCH /v1/apps/{appId}/campaigns/{campaignId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, campaign_id: &str, body: &CampaignsUpdateBody) -> Result<Campaign> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/campaigns/{campaignId}");
        op.path_param("campaignId", campaign_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kampanya sil
    ///
    /// `DELETE /v1/apps/{appId}/campaigns/{campaignId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, campaign_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/campaigns/{campaignId}");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// Zamanla; e-posta liste koruması 409 → force=true (admin kapsamı)
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/schedule`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn schedule(&self, campaign_id: &str, body: &ScheduleCampaignDto, params: Option<&CampaignsScheduleParams>) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/schedule");
        op.path_param("campaignId", campaign_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Hemen gönder; e-posta liste koruması 409 → force=true (admin kapsamı)
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/send`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn send(&self, campaign_id: &str, params: Option<&CampaignsSendParams>) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/send");
        op.path_param("campaignId", campaign_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Yayın öncesi denetim raporu (marka, yasaklı ifade, bağlantı, kişiselleştirme, KVKK/İYS)
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/preflight`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn preflight(&self, campaign_id: &str, params: Option<&CampaignsPreflightParams>) -> Result<CampaignPreflightReport> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/preflight");
        op.path_param("campaignId", campaign_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// E-posta liste riski (tahmini geçersiz oranı, 7/30 gün bounce oranı)
    ///
    /// `GET /v1/apps/{appId}/campaigns/{campaignId}/email-risk`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn email_risk(&self, campaign_id: &str) -> Result<CampaignEmailRisk> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/campaigns/{campaignId}/email-risk");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// Duraklat
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/pause`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn pause(&self, campaign_id: &str) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/pause");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// Sürdür
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/resume`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn resume(&self, campaign_id: &str) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/resume");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// İptal
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/cancel`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn cancel(&self, campaign_id: &str) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/cancel");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// Çoğalt
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/duplicate`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn duplicate(&self, campaign_id: &str) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/duplicate");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// Onayla
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/approve`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn approve(&self, campaign_id: &str, params: Option<&CampaignsApproveParams>) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/approve");
        op.path_param("campaignId", campaign_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// İstatistikler
    ///
    /// `GET /v1/apps/{appId}/campaigns/{campaignId}/stats`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn stats(&self, campaign_id: &str) -> Result<CampaignStats> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/campaigns/{campaignId}/stats");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// A/B varyant karşılaştırması
    ///
    /// `GET /v1/apps/{appId}/campaigns/{campaignId}/variants/stats`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn variant_stats(&self, campaign_id: &str) -> Result<CampaignVariantStats> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/campaigns/{campaignId}/variants/stats");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// A/B kazananını seç
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/winner`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn select_winner(&self, campaign_id: &str, body: &SelectWinnerDto) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/winner");
        op.path_param("campaignId", campaign_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Abone bazlı teslimatlar
    ///
    /// `GET /v1/apps/{appId}/campaigns/{campaignId}/deliveries`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn deliveries(&self, campaign_id: &str, params: Option<&CampaignsDeliveriesParams>) -> Result<DeliveryList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/campaigns/{campaignId}/deliveries");
        op.path_param("campaignId", campaign_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `deliveries` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn deliveries_all(&self, campaign_id: &str, params: Option<&CampaignsDeliveriesParams>) -> Paginator<'a, Delivery> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let campaign_id = campaign_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.deliveries(&campaign_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Başarısız teslimatlara yeniden gönder
    ///
    /// `POST /v1/apps/{appId}/campaigns/{campaignId}/resend-failed`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn resend_failed(&self, campaign_id: &str) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/campaigns/{campaignId}/resend-failed");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// Kampanya çalıştırmaları
    ///
    /// `GET /v1/apps/{appId}/campaigns/{campaignId}/messages`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn messages(&self, campaign_id: &str) -> Result<MessageList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/campaigns/{campaignId}/messages");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }
}

/// Uygulama içi (in-app) mesaj yönetimi ve durum değişiklikleri.
/// `Client::in_app_messages()` ile alınır.
#[derive(Debug, Clone)]
pub struct InAppMessagesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `InAppMessagesService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InAppMessagesListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    pub status: Option<String>,
}

impl InAppMessagesListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

impl<'a> InAppMessagesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// In-app mesajlar (durum filtresi; istatistik dahil)
    ///
    /// `GET /v1/apps/{appId}/in-app-messages`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&InAppMessagesListParams>) -> Result<InAppMessagePage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/in-app-messages");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&InAppMessagesListParams>) -> Paginator<'a, InAppMessage> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// In-app mesaj oluştur (taslak)
    ///
    /// `POST /v1/apps/{appId}/in-app-messages`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateInAppDto) -> Result<InAppMessage> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/in-app-messages");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// In-app mesaj detayı ve istatistikleri
    ///
    /// `GET /v1/apps/{appId}/in-app-messages/{messageId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, message_id: &str) -> Result<InAppMessage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/in-app-messages/{messageId}");
        op.path_param("messageId", message_id);
        self.client.execute(op, &self.opts)
    }

    /// In-app mesajı düzenle
    ///
    /// `PATCH /v1/apps/{appId}/in-app-messages/{messageId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, message_id: &str, body: &UpdateInAppDto) -> Result<InAppMessage> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/in-app-messages/{messageId}");
        op.path_param("messageId", message_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// In-app mesajı sil
    ///
    /// `DELETE /v1/apps/{appId}/in-app-messages/{messageId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, message_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/in-app-messages/{messageId}");
        op.path_param("messageId", message_id);
        self.client.execute(op, &self.opts)
    }

    /// Yayınla / duraklat / arşivle
    ///
    /// `POST /v1/apps/{appId}/in-app-messages/{messageId}/status`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn status(&self, message_id: &str, body: &InAppStatusDto) -> Result<InAppMessage> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/in-app-messages/{messageId}/status");
        op.path_param("messageId", message_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// A/B kazananını elle seç
    ///
    /// `POST /v1/apps/{appId}/in-app-messages/{messageId}/winner`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn select_winner(&self, message_id: &str, body: &InAppWinnerDto) -> Result<InAppMessage> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/in-app-messages/{messageId}/winner");
        op.path_param("messageId", message_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// A/B kazananını elle seç
    ///
    /// `POST /v1/apps/{appId}/in-app-messages/{messageId}/winner`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    #[deprecated(note = "select_winner kullanın; bu ad geriye dönük uyumluluk için korunur")]
    pub fn winner(&self, message_id: &str, body: &InAppWinnerDto) -> Result<InAppMessage> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/in-app-messages/{messageId}/winner");
        op.path_param("messageId", message_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// Kullanıcı bildirim kutusu (inbox): listeleme, okundu işaretleme ve arşivleme.
/// `Client::inbox()` ile alınır.
#[derive(Debug, Clone)]
pub struct InboxService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `InboxService::metrics` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InboxMetricsParams {
    pub from: Option<String>,
    pub to: Option<String>,
    /// Değerler: `"sms"`, `"whatsapp"`, `"rcs"`, `"email"`, `"chat"`.
    pub channel: Option<String>,
}

impl InboxMetricsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.channel {
            op.query("channel", v);
        }
    }
}

/// `InboxService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InboxListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub include_archived: Option<String>,
}

impl InboxListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.include_archived {
            op.query("include_archived", v);
        }
    }
}

impl<'a> InboxService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Takımlar
    ///
    /// `GET /v1/apps/{appId}/inbox/teams`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn teams(&self) -> Result<InboxTeamList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/inbox/teams");
        self.client.execute(op, &self.opts)
    }

    /// Temsilciler ve varlık durumu
    ///
    /// `GET /v1/apps/{appId}/inbox/agents`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn agents(&self) -> Result<InboxAgentList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/inbox/agents");
        self.client.execute(op, &self.opts)
    }

    /// Gelen kutusu ayarları
    ///
    /// `GET /v1/apps/{appId}/inbox/settings`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn settings(&self) -> Result<InboxSettings> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/inbox/settings");
        self.client.execute(op, &self.opts)
    }

    /// Gelen kutusu ayarlarını güncelle
    ///
    /// `PUT /v1/apps/{appId}/inbox/settings`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_settings(&self, body: &InboxSettingsUpdateDto) -> Result<InboxSettings> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/inbox/settings");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Gelen kutusu metrikleri (ilk yanıt/çözüm ort. ve p90, SLA, temsilci tablosu)
    ///
    /// `GET /v1/apps/{appId}/inbox/metrics`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn metrics(&self, params: Option<&InboxMetricsParams>) -> Result<InboxMetrics> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/inbox/metrics");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcının bildirim kutusu (okunmamış sayısı dahil)
    ///
    /// `GET /v1/apps/{appId}/users/by/{label}/{value}/inbox`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, label: &str, value: &str, params: Option<&InboxListParams>) -> Result<InboxPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/users/by/{label}/{value}/inbox");
        op.path_param("label", label);
        op.path_param("value", value);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, label: &str, value: &str, params: Option<&InboxListParams>) -> Paginator<'a, InboxItem> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let label = label.to_owned();
        let value = value.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(&label, &value, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Tümünü okundu işaretle
    ///
    /// `POST /v1/apps/{appId}/users/by/{label}/{value}/inbox/read-all`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn read_all(&self, label: &str, value: &str) -> Result<InboxReadAllResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/users/by/{label}/{value}/inbox/read-all");
        op.path_param("label", label);
        op.path_param("value", value);
        self.client.execute(op, &self.opts)
    }

    /// Okundu işaretle
    ///
    /// `POST /v1/apps/{appId}/users/by/{label}/{value}/inbox/{itemId}/read`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn read(&self, label: &str, value: &str, item_id: &str) -> Result<InboxItem> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/users/by/{label}/{value}/inbox/{itemId}/read");
        op.path_param("label", label);
        op.path_param("value", value);
        op.path_param("itemId", item_id);
        self.client.execute(op, &self.opts)
    }

    /// Kutudan kaldır (arşivle)
    ///
    /// `DELETE /v1/apps/{appId}/users/by/{label}/{value}/inbox/{itemId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn archive(&self, label: &str, value: &str, item_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/users/by/{label}/{value}/inbox/{itemId}");
        op.path_param("label", label);
        op.path_param("value", value);
        op.path_param("itemId", item_id);
        self.client.execute(op, &self.opts)
    }
}

/// E-posta/SMS gönderim engeli (suppression) listesi.
/// `Client::suppressions()` ile alınır.
#[derive(Debug, Clone)]
pub struct SuppressionsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `SuppressionsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SuppressionsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"email"`, `"sms"`, `"whatsapp"`, `"rcs"`.
    pub channel: Option<String>,
    /// Değerler: `"bounce"`, `"complaint"`, `"manual"`, `"unsubscribe_link"`, `"stop"`.
    pub reason: Option<String>,
    pub value: Option<String>,
}

impl SuppressionsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.channel {
            op.query("channel", v);
        }
        if let Some(v) = &self.reason {
            op.query("reason", v);
        }
        if let Some(v) = &self.value {
            op.query("value", v);
        }
    }
}

impl<'a> SuppressionsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Suppression list (e-posta/SMS gönderilmeyecek adresler; kanal/neden/değer filtresi)
    ///
    /// `GET /v1/apps/{appId}/suppressions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&SuppressionsListParams>) -> Result<SuppressionPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/suppressions");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&SuppressionsListParams>) -> Paginator<'a, Suppression> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Adres ekle (manuel engel; değer saklanmaz, özet + maskeli gösterim)
    ///
    /// `POST /v1/apps/{appId}/suppressions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateSuppressionDto) -> Result<Suppression> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/suppressions");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kaydı kaldır (adres yeniden gönderim alabilir)
    ///
    /// `DELETE /v1/apps/{appId}/suppressions/{suppressionId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, suppression_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/suppressions/{suppressionId}");
        op.path_param("suppressionId", suppression_id);
        self.client.execute(op, &self.opts)
    }
}

/// Veri dışa aktarma işleri.
/// `Client::exports()` ile alınır.
#[derive(Debug, Clone)]
pub struct ExportsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `ExportsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExportsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl ExportsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> ExportsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Export işi başlat
    ///
    /// `POST /v1/apps/{appId}/exports`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateExportDto) -> Result<Export> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/exports");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Export işleri
    ///
    /// `GET /v1/apps/{appId}/exports`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&ExportsListParams>) -> Result<ExportList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/exports");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&ExportsListParams>) -> Paginator<'a, Export> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Export durumu (download_url)
    ///
    /// `GET /v1/apps/{appId}/exports/{exportId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, export_id: &str) -> Result<Export> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/exports/{exportId}");
        op.path_param("exportId", export_id);
        self.client.execute(op, &self.opts)
    }
}

/// Veri içe aktarma işleri (CSV, multipart).
/// `Client::imports()` ile alınır.
#[derive(Debug, Clone)]
pub struct ImportsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `ImportsService::create` için multipart/form-data alanları.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ImportsCreateForm {
    /// Zorunlu. Değerler: `"legacy_players"`, `"csv_users"`.
    pub r#type: String,
    /// Zorunlu. CSV dosyası (UTF-8, en fazla 50 MB)
    pub file: Option<FileUpload>,
}

/// `ImportsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ImportsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl ImportsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> ImportsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Import işi başlat (multipart: type, file)
    ///
    /// `POST /v1/apps/{appId}/imports`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, form: &ImportsCreateForm) -> Result<Import> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/imports");
        let mut mp = Multipart::new();
        mp.field("type", &form.r#type);
        if let Some(v) = &form.file {
            mp.file("file", v);
        }
        op.multipart = Some(mp);
        self.client.execute(op, &self.opts)
    }

    /// Import işleri
    ///
    /// `GET /v1/apps/{appId}/imports`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&ImportsListParams>) -> Result<ImportList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/imports");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&ImportsListParams>) -> Paginator<'a, Import> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Import durumu
    ///
    /// `GET /v1/apps/{appId}/imports/{importId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, import_id: &str) -> Result<Import> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/imports/{importId}");
        op.path_param("importId", import_id);
        self.client.execute(op, &self.opts)
    }
}

/// Giden webhook uç noktaları, teslimatlar ve yeniden gönderim.
/// `Client::webhooks()` ile alınır.
#[derive(Debug, Clone)]
pub struct WebhooksService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `WebhooksService::deliveries` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WebhooksDeliveriesParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"pending"`, `"success"`, `"failed"`, `"dead"`.
    pub status: Option<String>,
    pub event_type: Option<String>,
}

impl WebhooksDeliveriesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.event_type {
            op.query("event_type", v);
        }
    }
}

impl<'a> WebhooksService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Webhook endpoint'leri
    ///
    /// `GET /v1/apps/{appId}/webhooks`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<WebhookList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/webhooks");
        self.client.execute(op, &self.opts)
    }

    /// Endpoint oluştur (secret yalnızca bu yanıtta döner)
    ///
    /// `POST /v1/apps/{appId}/webhooks`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateWebhookDto) -> Result<WebhookWithSecret> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/webhooks");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Event Streams önizlemesi: örnek olay + şablon + filtre → gövde/hata (gönderim yapmaz)
    ///
    /// `POST /v1/apps/{appId}/webhooks/preview`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn preview(&self, body: &WebhookPreviewDto) -> Result<WebhookPreviewResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/webhooks/preview");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Endpoint detayı
    ///
    /// `GET /v1/apps/{appId}/webhooks/{webhookId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, webhook_id: &str) -> Result<Webhook> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/webhooks/{webhookId}");
        op.path_param("webhookId", webhook_id);
        self.client.execute(op, &self.opts)
    }

    /// Endpoint güncelle (URL, olaylar, batch, başlıklar, etkinleştir/durdur)
    ///
    /// `PATCH /v1/apps/{appId}/webhooks/{webhookId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, webhook_id: &str, body: &UpdateWebhookDto) -> Result<Webhook> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/webhooks/{webhookId}");
        op.path_param("webhookId", webhook_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Endpoint sil
    ///
    /// `DELETE /v1/apps/{appId}/webhooks/{webhookId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, webhook_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/webhooks/{webhookId}");
        op.path_param("webhookId", webhook_id);
        self.client.execute(op, &self.opts)
    }

    /// Secret rotasyonu (eski secret 24 saat ikinci imza olarak gönderilir)
    ///
    /// `POST /v1/apps/{appId}/webhooks/{webhookId}/rotate-secret`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn rotate_secret(&self, webhook_id: &str) -> Result<WebhookWithSecret> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/webhooks/{webhookId}/rotate-secret");
        op.path_param("webhookId", webhook_id);
        self.client.execute(op, &self.opts)
    }

    /// Test olayı gönder (anında, sonuç döner)
    ///
    /// `POST /v1/apps/{appId}/webhooks/{webhookId}/test`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn test(&self, webhook_id: &str) -> Result<WebhookDeliveryDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/webhooks/{webhookId}/test");
        op.path_param("webhookId", webhook_id);
        self.client.execute(op, &self.opts)
    }

    /// Teslim geçmişi (cursor; durum/olay türü filtresi)
    ///
    /// `GET /v1/apps/{appId}/webhooks/{webhookId}/deliveries`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn deliveries(&self, webhook_id: &str, params: Option<&WebhooksDeliveriesParams>) -> Result<WebhookDeliveryPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/webhooks/{webhookId}/deliveries");
        op.path_param("webhookId", webhook_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `deliveries` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn deliveries_all(&self, webhook_id: &str, params: Option<&WebhooksDeliveriesParams>) -> Paginator<'a, WebhookDeliveryPageDataItem> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let webhook_id = webhook_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.deliveries(&webhook_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Teslimi yeniden gönder (aynı olay kimliğiyle)
    ///
    /// `POST /v1/apps/{appId}/webhooks/deliveries/{deliveryId}/replay`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn replay(&self, delivery_id: &str) -> Result<WebhookDeliveryDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/webhooks/deliveries/{deliveryId}/replay");
        op.path_param("deliveryId", delivery_id);
        self.client.execute(op, &self.opts)
    }
}

/// Kanal (APNs, FCM, Web Push, e-posta, SMS) yapılandırması ve doğrulaması.
/// `Client::channels()` ile alınır.
#[derive(Debug, Clone)]
pub struct ChannelsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> ChannelsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Kanal yapılandırmaları
    ///
    /// `GET /v1/apps/{appId}/channels`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<ChannelList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/channels");
        self.client.execute(op, &self.opts)
    }

    /// Kimlik bilgisi yükle (APNs p8, FCM servis hesabı, VAPID, mock)
    ///
    /// `PUT /v1/apps/{appId}/channels/{channel}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn upsert(&self, channel: &str, body: &serde_json::Value) -> Result<ChannelConfig> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/channels/{channel}");
        op.path_param("channel", channel);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Sağlayıcıya test bağlantısı
    ///
    /// `POST /v1/apps/{appId}/channels/{channel}/validate`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn validate(&self, channel: &str) -> Result<ChannelValidation> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/channels/{channel}/validate");
        op.path_param("channel", channel);
        self.client.execute(op, &self.opts)
    }
}

/// Analitik raporları.
/// `Client::analytics()` ile alınır.
#[derive(Debug, Clone)]
pub struct AnalyticsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `AnalyticsService::overview` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsOverviewParams {
    /// Değerler: `"7d"`, `"30d"`.
    pub range: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

impl AnalyticsOverviewParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.range {
            op.query("range", v);
        }
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
    }
}

/// `AnalyticsService::messages` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsMessagesParams {
    pub from: Option<String>,
    pub to: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub compare: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    pub format: Option<String>,
    /// Değerler: `"day"`, `"channel"`, `"campaign"`.
    pub group_by: Option<String>,
}

impl AnalyticsMessagesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.compare {
            op.query("compare", v);
        }
        if let Some(v) = &self.format {
            op.query("format", v);
        }
        if let Some(v) = &self.group_by {
            op.query("group_by", v);
        }
    }
}

/// `AnalyticsService::subscriptions` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsSubscriptionsParams {
    pub from: Option<String>,
    pub to: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub compare: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    pub format: Option<String>,
}

impl AnalyticsSubscriptionsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.compare {
            op.query("compare", v);
        }
        if let Some(v) = &self.format {
            op.query("format", v);
        }
    }
}

/// `AnalyticsService::uninstall_report` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsUninstallReportParams {
    pub from: Option<String>,
    pub to: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub compare: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    pub format: Option<String>,
}

impl AnalyticsUninstallReportParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.compare {
            op.query("compare", v);
        }
        if let Some(v) = &self.format {
            op.query("format", v);
        }
    }
}

/// `AnalyticsService::events` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsEventsParams {
    pub from: Option<String>,
    pub to: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub compare: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    pub format: Option<String>,
    pub name: Option<String>,
    /// Değerler: `"day"`, `"name"`.
    pub group_by: Option<String>,
}

impl AnalyticsEventsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.compare {
            op.query("compare", v);
        }
        if let Some(v) = &self.format {
            op.query("format", v);
        }
        if let Some(v) = &self.name {
            op.query("name", v);
        }
        if let Some(v) = &self.group_by {
            op.query("group_by", v);
        }
    }
}

/// `AnalyticsService::conversions` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsConversionsParams {
    pub from: Option<String>,
    pub to: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub compare: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    pub format: Option<String>,
    pub campaign_id: Option<String>,
}

impl AnalyticsConversionsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.compare {
            op.query("compare", v);
        }
        if let Some(v) = &self.format {
            op.query("format", v);
        }
        if let Some(v) = &self.campaign_id {
            op.query("campaign_id", v);
        }
    }
}

/// `AnalyticsService::providers` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsProvidersParams {
    pub from: Option<String>,
    pub to: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub compare: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    pub format: Option<String>,
}

impl AnalyticsProvidersParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.compare {
            op.query("compare", v);
        }
        if let Some(v) = &self.format {
            op.query("format", v);
        }
    }
}

/// `AnalyticsService::retention` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsRetentionParams {
    pub weeks: Option<i64>,
    /// Değerler: `"json"`, `"csv"`.
    pub format: Option<String>,
}

impl AnalyticsRetentionParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.weeks {
            op.query("weeks", v);
        }
        if let Some(v) = &self.format {
            op.query("format", v);
        }
    }
}

/// `AnalyticsService::engagement` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnalyticsEngagementParams {
    pub from: Option<String>,
    pub to: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub compare: Option<String>,
    /// Değerler: `"json"`, `"csv"`.
    pub format: Option<String>,
}

impl AnalyticsEngagementParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.compare {
            op.query("compare", v);
        }
        if let Some(v) = &self.format {
            op.query("format", v);
        }
    }
}

impl<'a> AnalyticsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Genel bakış KPI
    ///
    /// `GET /v1/apps/{appId}/analytics/overview`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn overview(&self, params: Option<&AnalyticsOverviewParams>) -> Result<OverviewStats> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/overview");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Teslimat raporu
    ///
    /// `GET /v1/apps/{appId}/analytics/messages`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn messages(&self, params: Option<&AnalyticsMessagesParams>) -> Result<MessagesReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/messages");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Abonelik büyümesi
    ///
    /// `GET /v1/apps/{appId}/analytics/subscriptions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn subscriptions(&self, params: Option<&AnalyticsSubscriptionsParams>) -> Result<SubscriptionsReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/subscriptions");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Uygulama kaldırma raporu
    ///
    /// `GET /v1/apps/{appId}/analytics/uninstalls`
    ///
    /// Mobil push sağlayıcısının "cihaz kayıtlı değil" yanıtından sayılır.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn uninstall_report(&self, params: Option<&AnalyticsUninstallReportParams>) -> Result<UninstallReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/uninstalls");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Olay hacmi
    ///
    /// `GET /v1/apps/{appId}/analytics/events`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn events(&self, params: Option<&AnalyticsEventsParams>) -> Result<EventsReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/events");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Dönüşüm/atribüsyon
    ///
    /// `GET /v1/apps/{appId}/analytics/conversions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn conversions(&self, params: Option<&AnalyticsConversionsParams>) -> Result<ConversionsReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/conversions");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Rapor çalıştır
    ///
    /// `POST /v1/apps/{appId}/analytics/reports/run`
    ///
    /// Kaynak, metrik, boyut ve filtre seçilerek rapor üretir; alanlar izinli listeyle sınırlıdır.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn run_report(&self, body: &ReportRunBodyDto) -> Result<ReportResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/analytics/reports/run");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Yol analizi
    ///
    /// `POST /v1/apps/{appId}/analytics/paths`
    ///
    /// Başlangıç olayından sonra veya önce kullanıcıların izlediği yollar.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn paths(&self, body: &PathBodyDto) -> Result<PathReport> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/analytics/paths");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Funnel
    ///
    /// `POST /v1/apps/{appId}/analytics/funnels`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn funnel(&self, body: &FunnelBodyDto) -> Result<FunnelReport> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/analytics/funnels");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Sağlayıcı sağlığı
    ///
    /// `GET /v1/apps/{appId}/analytics/providers`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn providers(&self, params: Option<&AnalyticsProvidersParams>) -> Result<ProvidersReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/providers");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Retention kohortu
    ///
    /// `GET /v1/apps/{appId}/analytics/retention`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn retention(&self, params: Option<&AnalyticsRetentionParams>) -> Result<RetentionReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/retention");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Etkileşim ısı haritası
    ///
    /// `GET /v1/apps/{appId}/analytics/engagement`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn engagement(&self, params: Option<&AnalyticsEngagementParams>) -> Result<EngagementReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/engagement");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Kaydedilmiş raporlar
    ///
    /// `GET /v1/apps/{appId}/analytics/reports`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<SavedReportList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/reports");
        self.client.execute(op, &self.opts)
    }

    /// Raporu kaydet
    ///
    /// `POST /v1/apps/{appId}/analytics/reports`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateSavedReportDto) -> Result<SavedReport> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/analytics/reports");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kaydedilmiş rapor
    ///
    /// `GET /v1/apps/{appId}/analytics/reports/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, id: &str) -> Result<SavedReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/analytics/reports/{id}");
        op.path_param("id", id);
        self.client.execute(op, &self.opts)
    }

    /// Raporu güncelle
    ///
    /// `PATCH /v1/apps/{appId}/analytics/reports/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, id: &str, body: &UpdateSavedReportDto) -> Result<SavedReport> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/analytics/reports/{id}");
        op.path_param("id", id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Raporu sil
    ///
    /// `DELETE /v1/apps/{appId}/analytics/reports/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, id: &str) -> Result<()> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/analytics/reports/{id}");
        op.path_param("id", id);
        self.client.execute_empty(op, &self.opts)
    }
}

/// App listesi ve ayarları.
/// `Client::apps()` ile alınır.
#[derive(Debug, Clone)]
pub struct AppsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> AppsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// App listesi (org geneli admin anahtarı)
    ///
    /// `GET /v1/apps`
    pub fn list(&self) -> Result<AppList> {
        let mut op = Operation::new("GET", "/v1/apps");
        self.client.execute(op, &self.opts)
    }

    /// App oluştur (org geneli admin anahtarı)
    ///
    /// `POST /v1/apps`
    pub fn create(&self, body: &CreateAppDto) -> Result<App> {
        let mut op = Operation::new("POST", "/v1/apps");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// App detayı
    ///
    /// `GET /v1/apps/{appId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self) -> Result<App> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}");
        self.client.execute(op, &self.opts)
    }

    /// App ayarlarını güncelle
    ///
    /// `PATCH /v1/apps/{appId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, body: &UpdateAppDto) -> Result<App> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// App API anahtarları: oluşturma, kapsam/IP izin listesi güncelleme, rotasyon ve iptal.
/// `Client::api_keys()` ile alınır.
#[derive(Debug, Clone)]
pub struct ApiKeysService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> ApiKeysService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// App'e bağlı API anahtarları
    ///
    /// `GET /v1/apps/{appId}/api-keys`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<ApiKeyList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/api-keys");
        self.client.execute(op, &self.opts)
    }

    /// App'e bağlı API anahtarı oluştur (ham anahtar yalnızca bu yanıtta)
    ///
    /// `POST /v1/apps/{appId}/api-keys`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateAppApiKeyDto) -> Result<CreatedApiKey> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/api-keys");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Anahtarı güncelle (ad, kapsam, IP izin listesi, bitiş)
    ///
    /// `PATCH /v1/apps/{appId}/api-keys/{keyId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, key_id: &str, body: &UpdateApiKeyDto) -> Result<ApiKey> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/api-keys/{keyId}");
        op.path_param("keyId", key_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Anahtarı iptal et
    ///
    /// `DELETE /v1/apps/{appId}/api-keys/{keyId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn revoke(&self, key_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/api-keys/{keyId}");
        op.path_param("keyId", key_id);
        self.client.execute(op, &self.opts)
    }

    /// Anahtarı döndür (eski değer anında geçersiz)
    ///
    /// `POST /v1/apps/{appId}/api-keys/{keyId}/rotate`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn rotate(&self, key_id: &str) -> Result<CreatedApiKey> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/api-keys/{keyId}/rotate");
        op.path_param("keyId", key_id);
        self.client.execute(op, &self.opts)
    }
}

/// Organizasyon, üyeler ve denetim kayıtları.
/// `Client::organization()` ile alınır.
#[derive(Debug, Clone)]
pub struct OrganizationService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `OrganizationService::audit_logs` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrganizationAuditLogsParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub action: Option<String>,
    pub actor_id: Option<String>,
    pub app_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

impl OrganizationAuditLogsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.action {
            op.query("action", v);
        }
        if let Some(v) = &self.actor_id {
            op.query("actor_id", v);
        }
        if let Some(v) = &self.app_id {
            op.query("app_id", v);
        }
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
    }
}

/// `OrganizationService::usage` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrganizationUsageParams {
    pub days: Option<i64>,
}

impl OrganizationUsageParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.days {
            op.query("days", v);
        }
    }
}

impl<'a> OrganizationService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Anahtarın ait olduğu organizasyon (org geneli admin anahtarı)
    ///
    /// `GET /v1/organization`
    pub fn get(&self) -> Result<Org> {
        let mut op = Operation::new("GET", "/v1/organization");
        self.client.execute(op, &self.opts)
    }

    /// Organizasyon üyeleri (org geneli admin anahtarı)
    ///
    /// `GET /v1/organization/members`
    pub fn members(&self) -> Result<MemberList> {
        let mut op = Operation::new("GET", "/v1/organization/members");
        self.client.execute(op, &self.opts)
    }

    /// Audit log (org geneli admin anahtarı; action/actor/app/tarih filtreleri, cursor)
    ///
    /// `GET /v1/organization/audit-logs`
    pub fn audit_logs(&self, params: Option<&OrganizationAuditLogsParams>) -> Result<AuditPage> {
        let mut op = Operation::new("GET", "/v1/organization/audit-logs");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `audit_logs` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn audit_logs_all(&self, params: Option<&OrganizationAuditLogsParams>) -> Paginator<'a, AuditEntry> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.audit_logs(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Organizasyon kullanımı ve plan kotaları (org geneli anahtar)
    ///
    /// `GET /v1/organization/usage`
    pub fn usage(&self, params: Option<&OrganizationUsageParams>) -> Result<UsageReport> {
        let mut op = Operation::new("GET", "/v1/organization/usage");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// `ads` kaynağı.
/// `Client::ads()` ile alınır.
#[derive(Debug, Clone)]
pub struct AdsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `AdsService::list_audiences` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsListAudiencesParams {
    pub ad_account_id: Option<String>,
}

impl AdsListAudiencesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.ad_account_id {
            op.query("ad_account_id", v);
        }
    }
}

/// `AdsService::remove_audience` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsRemoveAudienceParams {
    /// Değerler: `"true"`, `"false"`.
    pub delete_on_platform: Option<String>,
}

impl AdsRemoveAudienceParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.delete_on_platform {
            op.query("delete_on_platform", v);
        }
    }
}

/// `AdsService::syncs` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsSyncsParams {
    pub limit: Option<i64>,
}

impl AdsSyncsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
    }
}

/// `AdsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsListParams {
    pub ad_account_id: Option<String>,
    /// Değerler: `"campaign"`, `"ad_set"`, `"ad"`.
    pub level: Option<String>,
    pub parent_id: Option<String>,
    /// Değerler: `"active"`, `"paused"`, `"pending"`, `"rejected"`, `"archived"`, `"deleted"`, `"unknown"`.
    pub status: Option<String>,
    pub q: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<i64>,
}

impl AdsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.ad_account_id {
            op.query("ad_account_id", v);
        }
        if let Some(v) = &self.level {
            op.query("level", v);
        }
        if let Some(v) = &self.parent_id {
            op.query("parent_id", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
    }
}

/// `AdsService::spend` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsSpendParams {
    pub currency: Option<String>,
    /// Değerler: `"day"`, `"platform"`, `"account"`, `"campaign"`, `"ad_set"`.
    pub group_by: Option<String>,
    pub ad_account_id: Option<String>,
    pub campaign_id: Option<String>,
}

impl AdsSpendParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.currency {
            op.query("currency", v);
        }
        if let Some(v) = &self.group_by {
            op.query("group_by", v);
        }
        if let Some(v) = &self.ad_account_id {
            op.query("ad_account_id", v);
        }
        if let Some(v) = &self.campaign_id {
            op.query("campaign_id", v);
        }
    }
}

/// `AdsService::attribution` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsAttributionParams {
    /// Değerler: `"last_click"`, `"first_click"`, `"linear"`, `"time_decay"`.
    pub model: Option<String>,
    pub window_days: Option<i64>,
    pub half_life_days: Option<i64>,
    pub currency: Option<String>,
    /// Değerler: `"channel"`, `"campaign"`, `"ad_set"`.
    pub group_by: Option<String>,
    pub conversion_events: Option<String>,
}

impl AdsAttributionParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.model {
            op.query("model", v);
        }
        if let Some(v) = &self.window_days {
            op.query("window_days", v);
        }
        if let Some(v) = &self.half_life_days {
            op.query("half_life_days", v);
        }
        if let Some(v) = &self.currency {
            op.query("currency", v);
        }
        if let Some(v) = &self.group_by {
            op.query("group_by", v);
        }
        if let Some(v) = &self.conversion_events {
            op.query("conversion_events", v);
        }
    }
}

/// `AdsService::ltv` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsLtvParams {
    pub months: Option<i64>,
    pub cohorts: Option<i64>,
    pub currency: Option<String>,
}

impl AdsLtvParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.months {
            op.query("months", v);
        }
        if let Some(v) = &self.cohorts {
            op.query("cohorts", v);
        }
        if let Some(v) = &self.currency {
            op.query("currency", v);
        }
    }
}

/// `AdsService::capi_deliveries` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsCapiDeliveriesParams {
    pub config_id: Option<String>,
    /// Değerler: `"pending"`, `"sent"`, `"failed"`, `"skipped"`.
    pub status: Option<String>,
    pub limit: Option<i64>,
}

impl AdsCapiDeliveriesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.config_id {
            op.query("config_id", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
    }
}

/// `AdsService::alert_history` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdsAlertHistoryParams {
    pub rule_id: Option<String>,
    pub limit: Option<i64>,
}

impl AdsAlertHistoryParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.rule_id {
            op.query("rule_id", v);
        }
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
    }
}

impl<'a> AdsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Bağlı reklam hesapları
    ///
    /// `GET /v1/apps/{appId}/ads/accounts`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_accounts(&self) -> Result<AdAccountList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/accounts");
        self.client.execute(op, &self.opts)
    }

    /// Reklam kitleleri
    ///
    /// `GET /v1/apps/{appId}/ads/audiences`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_audiences(&self, params: Option<&AdsListAudiencesParams>) -> Result<AdAudienceList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/audiences");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Reklam kitlesi oluştur
    ///
    /// `POST /v1/apps/{appId}/ads/audiences`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_audience(&self, body: &CreateAdAudienceDto) -> Result<AdAudience> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/ads/audiences");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Reklam kitlesi ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/ads/audiences/{audienceId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_audience(&self, audience_id: &str) -> Result<AdAudience> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/audiences/{audienceId}");
        op.path_param("audienceId", audience_id);
        self.client.execute(op, &self.opts)
    }

    /// Reklam kitlesini güncelle
    ///
    /// `PATCH /v1/apps/{appId}/ads/audiences/{audienceId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_audience(&self, audience_id: &str, body: &UpdateAdAudienceDto) -> Result<AdAudience> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/ads/audiences/{audienceId}");
        op.path_param("audienceId", audience_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Reklam kitlesini sil
    ///
    /// `DELETE /v1/apps/{appId}/ads/audiences/{audienceId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn remove_audience(&self, audience_id: &str, params: Option<&AdsRemoveAudienceParams>) -> Result<AdOk> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/ads/audiences/{audienceId}");
        op.path_param("audienceId", audience_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Senkronu şimdi çalıştır
    ///
    /// `POST /v1/apps/{appId}/ads/audiences/{audienceId}/sync`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn sync(&self, audience_id: &str, body: &SyncAdAudienceDto) -> Result<AdAudienceSyncQueued> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/ads/audiences/{audienceId}/sync");
        op.path_param("audienceId", audience_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Senkron geçmişi
    ///
    /// `GET /v1/apps/{appId}/ads/audiences/{audienceId}/syncs`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn syncs(&self, audience_id: &str, params: Option<&AdsSyncsParams>) -> Result<AdAudienceSyncList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/audiences/{audienceId}/syncs");
        op.path_param("audienceId", audience_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Benzer kitle (lookalike) oluştur
    ///
    /// `POST /v1/apps/{appId}/ads/audiences/{audienceId}/lookalikes`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn lookalike(&self, audience_id: &str, body: &CreateLookalikeDto) -> Result<AdAudience> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/ads/audiences/{audienceId}/lookalikes");
        op.path_param("audienceId", audience_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kampanya / reklam seti / reklam listesi ve metrikler
    ///
    /// `GET /v1/apps/{appId}/ads/campaigns`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&AdsListParams>) -> Result<AdEntityList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/campaigns");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Kampanya / reklam seti / reklam ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/ads/campaigns/{entityId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, entity_id: &str) -> Result<AdEntity> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/campaigns/{entityId}");
        op.path_param("entityId", entity_id);
        self.client.execute(op, &self.opts)
    }

    /// Durum ve günlük bütçe değişikliği
    ///
    /// `PATCH /v1/apps/{appId}/ads/campaigns/{entityId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, entity_id: &str, body: &UpdateAdCampaignDto) -> Result<AdEntity> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/ads/campaigns/{entityId}");
        op.path_param("entityId", entity_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kampanyaları ve metrikleri şimdi yenile
    ///
    /// `POST /v1/apps/{appId}/ads/accounts/{accountId}/refresh`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn refresh(&self, account_id: &str) -> Result<AdRefreshQueued> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/ads/accounts/{accountId}/refresh");
        op.path_param("accountId", account_id);
        self.client.execute(op, &self.opts)
    }

    /// Harcama ve performans raporu
    ///
    /// `GET /v1/apps/{appId}/ads/reports/spend`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn spend(&self, from: &str, to: &str, params: Option<&AdsSpendParams>) -> Result<AdSpendReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/reports/spend");
        op.query("from", from);
        op.query("to", to);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Birleşik atribüsyon raporu (ROAS, CAC)
    ///
    /// `GET /v1/apps/{appId}/ads/reports/attribution`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn attribution(&self, from: &str, to: &str, params: Option<&AdsAttributionParams>) -> Result<AdAttributionReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/reports/attribution");
        op.query("from", from);
        op.query("to", to);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Kohort LTV raporu
    ///
    /// `GET /v1/apps/{appId}/ads/reports/ltv`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn ltv(&self, params: Option<&AdsLtvParams>) -> Result<AdLtvReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/reports/ltv");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Conversions API yapılandırmaları
    ///
    /// `GET /v1/apps/{appId}/ads/conversions-api/configs`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn capi_configs(&self) -> Result<AdCapiConfigList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/conversions-api/configs");
        self.client.execute(op, &self.opts)
    }

    /// Conversions API teslim geçmişi
    ///
    /// `GET /v1/apps/{appId}/ads/conversions-api/deliveries`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn capi_deliveries(&self, params: Option<&AdsCapiDeliveriesParams>) -> Result<AdCapiDeliveryList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/conversions-api/deliveries");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Bütçe uyarı kuralları
    ///
    /// `GET /v1/apps/{appId}/ads/alert-rules`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn rules(&self) -> Result<AdBudgetAlertRuleList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/alert-rules");
        self.client.execute(op, &self.opts)
    }

    /// Bütçe uyarı kuralı oluştur
    ///
    /// `POST /v1/apps/{appId}/ads/alert-rules`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_rule(&self, body: &CreateBudgetAlertRuleDto) -> Result<AdBudgetAlertRule> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/ads/alert-rules");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Bütçe uyarı kuralını güncelle
    ///
    /// `PATCH /v1/apps/{appId}/ads/alert-rules/{ruleId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_rule(&self, rule_id: &str, body: &UpdateBudgetAlertRuleDto) -> Result<AdBudgetAlertRule> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/ads/alert-rules/{ruleId}");
        op.path_param("ruleId", rule_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Bütçe uyarı kuralını sil
    ///
    /// `DELETE /v1/apps/{appId}/ads/alert-rules/{ruleId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn remove_rule(&self, rule_id: &str) -> Result<AdOk> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/ads/alert-rules/{ruleId}");
        op.path_param("ruleId", rule_id);
        self.client.execute(op, &self.opts)
    }

    /// Tetiklenen bütçe uyarıları
    ///
    /// `GET /v1/apps/{appId}/ads/alerts`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn alert_history(&self, params: Option<&AdsAlertHistoryParams>) -> Result<AdBudgetAlertList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/ads/alerts");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// `cannedResponses` kaynağı.
/// `Client::canned_responses()` ile alınır.
#[derive(Debug, Clone)]
pub struct CannedResponsesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `CannedResponsesService::canned` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CannedResponsesCannedParams {
    pub q: Option<String>,
}

impl CannedResponsesCannedParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.q {
            op.query("q", v);
        }
    }
}

impl<'a> CannedResponsesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Hazır yanıtlar
    ///
    /// `GET /v1/apps/{appId}/canned-responses`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn canned(&self, params: Option<&CannedResponsesCannedParams>) -> Result<CannedResponseList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/canned-responses");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Hazır yanıt oluştur
    ///
    /// `POST /v1/apps/{appId}/canned-responses`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_canned(&self, body: &CannedInputDto) -> Result<CannedResponse> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/canned-responses");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Hazır yanıtı güncelle
    ///
    /// `PATCH /v1/apps/{appId}/canned-responses/{cannedId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_canned(&self, canned_id: &str, body: &CannedUpdateDto) -> Result<CannedResponse> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/canned-responses/{cannedId}");
        op.path_param("cannedId", canned_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Hazır yanıtı sil
    ///
    /// `DELETE /v1/apps/{appId}/canned-responses/{cannedId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete_canned(&self, canned_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/canned-responses/{cannedId}");
        op.path_param("cannedId", canned_id);
        self.client.execute(op, &self.opts)
    }
}

/// `catalogs` kaynağı.
/// `Client::catalogs()` ile alınır.
#[derive(Debug, Clone)]
pub struct CatalogsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `CatalogsService::items` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CatalogsItemsParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub q: Option<String>,
    pub category: Option<String>,
}

impl CatalogsItemsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.offset {
            op.query("offset", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
        if let Some(v) = &self.category {
            op.query("category", v);
        }
    }
}

/// `CatalogsService::upload` için multipart/form-data alanları.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CatalogsUploadForm {
    /// Zorunlu. UTF-8 CSV. Zorunlu sütunlar: id, title. Bilinen: description, image_url, url, price, sale_price, currency, category, brand, in_stock. Diğerleri özellik olur. En fazla 100.000 satır, 20 MB.
    pub file: Option<FileUpload>,
}

impl<'a> CatalogsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Ürün katalogları
    ///
    /// `GET /v1/apps/{appId}/catalogs`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<CatalogList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/catalogs");
        self.client.execute(op, &self.opts)
    }

    /// Katalog oluştur
    ///
    /// `POST /v1/apps/{appId}/catalogs`
    ///
    /// Şablonda `{% catalog_item '<ad>' id: custom_data.product_id as urun %}{{ urun.title }}`.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateCatalogDto) -> Result<Catalog> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/catalogs");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Katalog ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/catalogs/{catalogId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, catalog_id: &str) -> Result<Catalog> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/catalogs/{catalogId}");
        op.path_param("catalogId", catalog_id);
        self.client.execute(op, &self.opts)
    }

    /// Kataloğu güncelle
    ///
    /// `PATCH /v1/apps/{appId}/catalogs/{catalogId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, catalog_id: &str, body: &UpdateCatalogDto) -> Result<Catalog> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/catalogs/{catalogId}");
        op.path_param("catalogId", catalog_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kataloğu sil
    ///
    /// `DELETE /v1/apps/{appId}/catalogs/{catalogId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, catalog_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/catalogs/{catalogId}");
        op.path_param("catalogId", catalog_id);
        self.client.execute(op, &self.opts)
    }

    /// Ürünler
    ///
    /// `GET /v1/apps/{appId}/catalogs/{catalogId}/items`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn items(&self, catalog_id: &str, params: Option<&CatalogsItemsParams>) -> Result<CatalogItemsPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/catalogs/{catalogId}/items");
        op.path_param("catalogId", catalog_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Ürünleri ekle veya güncelle (en fazla 1000)
    ///
    /// `PUT /v1/apps/{appId}/catalogs/{catalogId}/items`
    ///
    /// Listede olmayan ürünlere dokunulmaz. Listedeki ürünün verilmeyen alanları temizlenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn upsert(&self, catalog_id: &str, body: &UpsertCatalogItemsDto) -> Result<CatalogUpsertResult> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/catalogs/{catalogId}/items");
        op.path_param("catalogId", catalog_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// CSV ile ürün ekle veya güncelle
    ///
    /// `POST /v1/apps/{appId}/catalogs/{catalogId}/items/upload`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn upload(&self, catalog_id: &str, form: &CatalogsUploadForm) -> Result<CatalogUpsertResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/catalogs/{catalogId}/items/upload");
        op.path_param("catalogId", catalog_id);
        let mut mp = Multipart::new();
        if let Some(v) = &form.file {
            mp.file("file", v);
        }
        op.multipart = Some(mp);
        self.client.execute(op, &self.opts)
    }

    /// Ürün ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/catalogs/{catalogId}/items/{itemId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn item(&self, catalog_id: &str, item_id: &str) -> Result<CatalogItem> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/catalogs/{catalogId}/items/{itemId}");
        op.path_param("catalogId", catalog_id);
        op.path_param("itemId", item_id);
        self.client.execute(op, &self.opts)
    }

    /// Ürünü sil
    ///
    /// `DELETE /v1/apps/{appId}/catalogs/{catalogId}/items/{itemId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn remove_item(&self, catalog_id: &str, item_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/catalogs/{catalogId}/items/{itemId}");
        op.path_param("catalogId", catalog_id);
        op.path_param("itemId", item_id);
        self.client.execute(op, &self.opts)
    }
}

/// `categories` kaynağı.
/// `Client::categories()` ile alınır.
#[derive(Debug, Clone)]
pub struct CategoriesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> CategoriesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Mesaj kategorileri
    ///
    /// `GET /v1/apps/{appId}/categories`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<MessageCategoryList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/categories");
        self.client.execute(op, &self.opts)
    }

    /// Kategori oluştur
    ///
    /// `POST /v1/apps/{appId}/categories`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateCategoryDto) -> Result<MessageCategory> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/categories");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kategori güncelle
    ///
    /// `PATCH /v1/apps/{appId}/categories/{categoryId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, category_id: &str, body: &UpdateCategoryDto) -> Result<MessageCategory> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/categories/{categoryId}");
        op.path_param("categoryId", category_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kategori sil
    ///
    /// `DELETE /v1/apps/{appId}/categories/{categoryId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, category_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/categories/{categoryId}");
        op.path_param("categoryId", category_id);
        self.client.execute(op, &self.opts)
    }
}

/// `channelScores` kaynağı.
/// `Client::channel_scores()` ile alınır.
#[derive(Debug, Clone)]
pub struct ChannelScoresService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `ChannelScoresService::users` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChannelScoresUsersParams {
    /// Değerler: `"push"`, `"email"`, `"whatsapp"`, `"rcs"`, `"sms"`.
    pub channel: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl ChannelScoresUsersParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.channel {
            op.query("channel", v);
        }
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> ChannelScoresService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Kanal skoru ayarı
    ///
    /// `GET /v1/apps/{appId}/channel-scores/config`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn config(&self) -> Result<ChannelScoreConfig> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/channel-scores/config");
        self.client.execute(op, &self.opts)
    }

    /// Kanal skorunu ayarla
    ///
    /// `PUT /v1/apps/{appId}/channel-scores/config`
    ///
    /// Açıkken skorlar kayıttan sonra yeniden hesaplanır; kapatınca silinir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, body: &UpdateChannelScoreConfigDto) -> Result<ChannelScoreConfig> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/channel-scores/config");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kanal skorlarını yeniden hesapla
    ///
    /// `POST /v1/apps/{appId}/channel-scores/compute`
    ///
    /// İş kuyruğa alınır; ilerleme `status` alanından izlenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn compute(&self) -> Result<ChannelScoreConfig> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/channel-scores/compute");
        self.client.execute(op, &self.opts)
    }

    /// En iyi kanal dağılımı
    ///
    /// `GET /v1/apps/{appId}/channel-scores/overview`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn overview(&self) -> Result<ChannelScoreOverview> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/channel-scores/overview");
        self.client.execute(op, &self.opts)
    }

    /// En iyi kanalı belli kullanıcılar
    ///
    /// `GET /v1/apps/{appId}/channel-scores/users`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn users(&self, params: Option<&ChannelScoresUsersParams>) -> Result<ChannelScoreUsersPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/channel-scores/users");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcının kanal skorları
    ///
    /// `GET /v1/apps/{appId}/channel-scores/users/{userId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn user(&self, user_id: &str) -> Result<ChannelScoreUser> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/channel-scores/users/{userId}");
        op.path_param("userId", user_id);
        self.client.execute(op, &self.opts)
    }
}

/// `contentTables` kaynağı.
/// `Client::content_tables()` ile alınır.
#[derive(Debug, Clone)]
pub struct ContentTablesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `ContentTablesService::create_table_upload` için multipart/form-data alanları.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ContentTablesCreateTableUploadForm {
    /// Zorunlu. UTF-8 CSV; ilk satır başlık. En fazla 100.000 satır, 20 MB.
    pub file: Option<FileUpload>,
    /// Zorunlu. Liquid tablo adı (küçük harf, rakam, alt çizgi).
    pub name: String,
    /// Zorunlu. Satır seçiminde kullanılan anahtar sütunu (değerleri benzersiz).
    pub key_column: String,
    pub description: Option<String>,
}

/// `ContentTablesService::replace_table_upload` için multipart/form-data alanları.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ContentTablesReplaceTableUploadForm {
    /// Zorunlu. Yeni CSV (tablonun tüm satırları değiştirilir, sürüm artar).
    pub file: Option<FileUpload>,
    /// İsteğe bağlı yeni anahtar sütunu.
    pub key_column: Option<String>,
}

/// `ContentTablesService::table_rows` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ContentTablesTableRowsParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub q: Option<String>,
}

impl ContentTablesTableRowsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.offset {
            op.query("offset", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
    }
}

impl<'a> ContentTablesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// İçerik tabloları
    ///
    /// `GET /v1/apps/{appId}/content-tables`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_tables(&self) -> Result<ContentTableList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/content-tables");
        self.client.execute(op, &self.opts)
    }

    /// CSV ile içerik tablosu oluştur
    ///
    /// `POST /v1/apps/{appId}/content-tables`
    ///
    /// Şablonda `{% lookup '<ad>' key: user.tags.<etiket> as satir %}{{ satir.<sütun> }}`.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_table_upload(&self, form: &ContentTablesCreateTableUploadForm) -> Result<ContentTable> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/content-tables");
        let mut mp = Multipart::new();
        if let Some(v) = &form.file {
            mp.file("file", v);
        }
        mp.field("name", &form.name);
        mp.field("key_column", &form.key_column);
        if let Some(v) = &form.description {
            mp.field("description", v);
        }
        op.multipart = Some(mp);
        self.client.execute(op, &self.opts)
    }

    /// İçerik tablosu ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/content-tables/{tableId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_table(&self, table_id: &str) -> Result<ContentTable> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/content-tables/{tableId}");
        op.path_param("tableId", table_id);
        self.client.execute(op, &self.opts)
    }

    /// İçerik tablosu açıklamasını güncelle
    ///
    /// `PATCH /v1/apps/{appId}/content-tables/{tableId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_table(&self, table_id: &str, body: &UpdateContentTableDto) -> Result<ContentTable> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/content-tables/{tableId}");
        op.path_param("tableId", table_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// İçerik tablosunu sil
    ///
    /// `DELETE /v1/apps/{appId}/content-tables/{tableId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete_table(&self, table_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/content-tables/{tableId}");
        op.path_param("tableId", table_id);
        self.client.execute(op, &self.opts)
    }

    /// İçerik tablosunu yeniden yükle (sürüm artar)
    ///
    /// `PUT /v1/apps/{appId}/content-tables/{tableId}/rows`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn replace_table_upload(&self, table_id: &str, form: &ContentTablesReplaceTableUploadForm) -> Result<ContentTable> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/content-tables/{tableId}/rows");
        op.path_param("tableId", table_id);
        let mut mp = Multipart::new();
        if let Some(v) = &form.file {
            mp.file("file", v);
        }
        if let Some(v) = &form.key_column {
            mp.field("key_column", v);
        }
        op.multipart = Some(mp);
        self.client.execute(op, &self.opts)
    }

    /// İçerik tablosu satırları
    ///
    /// `GET /v1/apps/{appId}/content-tables/{tableId}/rows`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn table_rows(&self, table_id: &str, params: Option<&ContentTablesTableRowsParams>) -> Result<ContentTableRows> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/content-tables/{tableId}/rows");
        op.path_param("tableId", table_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// `conversations` kaynağı.
/// `Client::conversations()` ile alınır.
#[derive(Debug, Clone)]
pub struct ConversationsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `ConversationsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConversationsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"sms"`, `"whatsapp"`, `"rcs"`, `"email"`, `"chat"`.
    pub channel: Option<String>,
    /// Değerler: `"open"`, `"pending"`, `"resolved"`, `"closed"`.
    pub status: Option<String>,
    /// Değerler: `"true"`, `"false"`.
    pub unread: Option<String>,
    pub q: Option<String>,
    pub assignee: Option<String>,
    pub team_id: Option<String>,
}

impl ConversationsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.channel {
            op.query("channel", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.unread {
            op.query("unread", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
        if let Some(v) = &self.assignee {
            op.query("assignee", v);
        }
        if let Some(v) = &self.team_id {
            op.query("team_id", v);
        }
    }
}

/// `ConversationsService::messages` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConversationsMessagesParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub after_seq: Option<i64>,
}

impl ConversationsMessagesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.after_seq {
            op.query("after_seq", v);
        }
    }
}

impl<'a> ConversationsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Konuşmalar (SMS, WhatsApp, RCS, e-posta, canlı sohbet; son mesaja göre; kanal/durum/okunmamış/atanan/takım/kişi filtresi)
    ///
    /// `GET /v1/apps/{appId}/conversations`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&ConversationsListParams>) -> Result<ConversationPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/conversations");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&ConversationsListParams>) -> Paginator<'a, Conversation> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Konuşma (atama, SLA, bot durumu, WhatsApp oturum penceresi ve açık rıza dahil)
    ///
    /// `GET /v1/apps/{appId}/conversations/{conversationId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, conversation_id: &str) -> Result<Conversation> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/conversations/{conversationId}");
        op.path_param("conversationId", conversation_id);
        self.client.execute(op, &self.opts)
    }

    /// Konuşma durumu: open / pending / resolved (closed = resolved)
    ///
    /// `PATCH /v1/apps/{appId}/conversations/{conversationId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn set_status(&self, conversation_id: &str, body: &ConversationUpdateDto) -> Result<Conversation> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/conversations/{conversationId}");
        op.path_param("conversationId", conversation_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Konuşma mesajları (yeniden eskiye; `after_seq` ile eskiden yeniye; giden mesaj durumu teslimattan veya okundu işaretçisinden)
    ///
    /// `GET /v1/apps/{appId}/conversations/{conversationId}/messages`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn messages(&self, conversation_id: &str, params: Option<&ConversationsMessagesParams>) -> Result<ConversationMessagePage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/conversations/{conversationId}/messages");
        op.path_param("conversationId", conversation_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `messages` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn messages_all(&self, conversation_id: &str, params: Option<&ConversationsMessagesParams>) -> Paginator<'a, ConversationMessagePageDataItem> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let conversation_id = conversation_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.messages(&conversation_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Yanıt gönder (client_message_id ile tekilleşir). Sohbet: doğrudan; e-posta: yanıt adresli işlemsel e-posta; WhatsApp: açık rıza zorunlu, son gelen mesajdan 24 saat içinde metin/medya, pencere dışında yalnızca onaylı şablon (aksi 422 WHATSAPP_WINDOW_CLOSED).
    ///
    /// `POST /v1/apps/{appId}/conversations/{conversationId}/messages`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn reply(&self, conversation_id: &str, body: &ConversationReplyDto) -> Result<ConversationReplyResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/conversations/{conversationId}/messages");
        op.path_param("conversationId", conversation_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Gelen mesajları okundu işaretle (sohbette ziyaretçiye okundu bilgisi gider)
    ///
    /// `POST /v1/apps/{appId}/conversations/{conversationId}/read`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn read(&self, conversation_id: &str) -> Result<Conversation> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/conversations/{conversationId}/read");
        op.path_param("conversationId", conversation_id);
        self.client.execute(op, &self.opts)
    }

    /// Konuşmayı temsilciye ve/veya takıma ata (inbox.manage olmadan yalnızca kendine)
    ///
    /// `POST /v1/apps/{appId}/conversations/{conversationId}/assign`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn assign(&self, conversation_id: &str, body: &ConversationAssignDto) -> Result<Conversation> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/conversations/{conversationId}/assign");
        op.path_param("conversationId", conversation_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Sohbetten kişiye etiket ekle/kaldır
    ///
    /// `POST /v1/apps/{appId}/conversations/{conversationId}/tags`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn tags(&self, conversation_id: &str, body: &ConversationTagsDto) -> Result<ConversationTagsResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/conversations/{conversationId}/tags");
        op.path_param("conversationId", conversation_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Konuşma zaman çizelgesi (atama, durum, ilk yanıt, bot devri)
    ///
    /// `GET /v1/apps/{appId}/conversations/{conversationId}/activity`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn activity(&self, conversation_id: &str) -> Result<ConversationActivity> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/conversations/{conversationId}/activity");
        op.path_param("conversationId", conversation_id);
        self.client.execute(op, &self.opts)
    }
}

/// `dataFeeds` kaynağı.
/// `Client::data_feeds()` ile alınır.
#[derive(Debug, Clone)]
pub struct DataFeedsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> DataFeedsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Data feed listesi
    ///
    /// `GET /v1/apps/{appId}/data-feeds`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_feeds(&self) -> Result<DataFeedList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/data-feeds");
        self.client.execute(op, &self.opts)
    }

    /// Data feed oluştur
    ///
    /// `POST /v1/apps/{appId}/data-feeds`
    ///
    /// URL şablonu Liquid ile kişiselleştirilir (ör. `https://api.ornek.com/u/{{ user.external_id }}`); şablonda `{{ feed.<ad>.alan }}`.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_feed(&self, body: &CreateDataFeedDto) -> Result<DataFeed> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/data-feeds");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Data feed ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/data-feeds/{feedId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_feed(&self, feed_id: &str) -> Result<DataFeed> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/data-feeds/{feedId}");
        op.path_param("feedId", feed_id);
        self.client.execute(op, &self.opts)
    }

    /// Data feed güncelle
    ///
    /// `PATCH /v1/apps/{appId}/data-feeds/{feedId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_feed(&self, feed_id: &str, body: &UpdateDataFeedDto) -> Result<DataFeed> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/data-feeds/{feedId}");
        op.path_param("feedId", feed_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Data feed sil
    ///
    /// `DELETE /v1/apps/{appId}/data-feeds/{feedId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete_feed(&self, feed_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/data-feeds/{feedId}");
        op.path_param("feedId", feed_id);
        self.client.execute(op, &self.opts)
    }

    /// Data feed'i örnek kullanıcıyla dene
    ///
    /// `POST /v1/apps/{appId}/data-feeds/{feedId}/test`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn test_feed(&self, feed_id: &str, body: &TestDataFeedDto) -> Result<DataFeedTestResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/data-feeds/{feedId}/test");
        op.path_param("feedId", feed_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `decisionAgents` kaynağı.
/// `Client::decision_agents()` ile alınır.
#[derive(Debug, Clone)]
pub struct DecisionAgentsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> DecisionAgentsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Karar ajanları
    ///
    /// `GET /v1/apps/{appId}/decision-agents`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<DecisionAgentList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/decision-agents");
        self.client.execute(op, &self.opts)
    }

    /// Karar ajanı oluştur
    ///
    /// `POST /v1/apps/{appId}/decision-agents`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateDecisionAgentDto) -> Result<DecisionAgent> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/decision-agents");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Karar ajanı
    ///
    /// `GET /v1/apps/{appId}/decision-agents/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, id: &str) -> Result<DecisionAgent> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/decision-agents/{id}");
        op.path_param("id", id);
        self.client.execute(op, &self.opts)
    }

    /// Karar ajanını güncelle
    ///
    /// `PATCH /v1/apps/{appId}/decision-agents/{id}`
    ///
    /// Seçenekler değiştirilemez: geçmiş öğrenme onlara bağlıdır. Yeni seçenekler için yeni ajan oluşturun.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, id: &str, body: &UpdateDecisionAgentDto) -> Result<DecisionAgent> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/decision-agents/{id}");
        op.path_param("id", id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Karar ajanını sil
    ///
    /// `DELETE /v1/apps/{appId}/decision-agents/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, id: &str) -> Result<()> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/decision-agents/{id}");
        op.path_param("id", id);
        self.client.execute_empty(op, &self.opts)
    }

    /// Kazanç raporu
    ///
    /// `GET /v1/apps/{appId}/decision-agents/{id}/report`
    ///
    /// Öğrenen politika ve kontrol grubu karşılaştırması.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn report(&self, id: &str) -> Result<DecisionReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/decision-agents/{id}/report");
        op.path_param("id", id);
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcı için karar al
    ///
    /// `POST /v1/apps/{appId}/decision-agents/{id}/decide`
    ///
    /// Karar kaydedilir, ödül olayı geldikçe ajan öğrenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn decide(&self, id: &str, body: &DecideDto) -> Result<DecisionResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/decision-agents/{id}/decide");
        op.path_param("id", id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `emailDeliverability` kaynağı.
/// `Client::email_deliverability()` ile alınır.
#[derive(Debug, Clone)]
pub struct EmailDeliverabilityService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `EmailDeliverabilityService::usage` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmailDeliverabilityUsageParams {
    pub days: Option<i64>,
}

impl EmailDeliverabilityUsageParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.days {
            op.query("days", v);
        }
    }
}

impl<'a> EmailDeliverabilityService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// E-posta teslim ayarları (BCC/arşiv, gönderim öncesi adres doğrulama, üçüncü taraf doğrulayıcı)
    ///
    /// `GET /v1/apps/{appId}/email-deliverability`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn settings(&self) -> Result<EmailDeliverabilitySettings> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/email-deliverability");
        self.client.execute(op, &self.opts)
    }

    /// E-posta teslim ayarlarını güncelle (doğrulayıcı anahtarı zarf şifreli saklanır, geri okunmaz)
    ///
    /// `PUT /v1/apps/{appId}/email-deliverability`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, body: &DeliverabilitySettingsDto) -> Result<EmailDeliverabilitySettings> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/email-deliverability");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Adres doğrulama testi (sözdizimi, MX, tek kullanımlık alan adı, rol hesabı, üçüncü taraf)
    ///
    /// `POST /v1/apps/{appId}/email-deliverability/verify-address`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn verify(&self, body: &VerifyAddressDto) -> Result<EmailAddressCheck> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/email-deliverability/verify-address");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Isındırma planları (bugünkü limit/kullanım ve ileri günlere ayrılmış gönderimler)
    ///
    /// `GET /v1/apps/{appId}/email-deliverability/warmup-plans`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn plans(&self) -> Result<EmailWarmupPlanList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/email-deliverability/warmup-plans");
        self.client.execute(op, &self.opts)
    }

    /// Isındırma planı oluştur (günlük limit eğrisi veya başlangıç/çarpan/hedef; sağlayıcı payları)
    ///
    /// `POST /v1/apps/{appId}/email-deliverability/warmup-plans`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateWarmupPlanDto) -> Result<EmailWarmupPlan> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/email-deliverability/warmup-plans");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Isındırma planı
    ///
    /// `GET /v1/apps/{appId}/email-deliverability/warmup-plans/{planId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, plan_id: &str) -> Result<EmailWarmupPlan> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/email-deliverability/warmup-plans/{planId}");
        op.path_param("planId", plan_id);
        self.client.execute(op, &self.opts)
    }

    /// Isındırma planını güncelle (duraklat/sürdür, eğri, paylar)
    ///
    /// `PATCH /v1/apps/{appId}/email-deliverability/warmup-plans/{planId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_plan(&self, plan_id: &str, body: &UpdateWarmupPlanDto) -> Result<EmailWarmupPlan> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/email-deliverability/warmup-plans/{planId}");
        op.path_param("planId", plan_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Isındırma planını sil
    ///
    /// `DELETE /v1/apps/{appId}/email-deliverability/warmup-plans/{planId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, plan_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/email-deliverability/warmup-plans/{planId}");
        op.path_param("planId", plan_id);
        self.client.execute(op, &self.opts)
    }

    /// Isındırma planı gün × sağlayıcı kullanımı
    ///
    /// `GET /v1/apps/{appId}/email-deliverability/warmup-plans/{planId}/usage`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn usage(&self, plan_id: &str, params: Option<&EmailDeliverabilityUsageParams>) -> Result<EmailWarmupUsage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/email-deliverability/warmup-plans/{planId}/usage");
        op.path_param("planId", plan_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Google Postmaster itibar kayıtları
    ///
    /// `GET /v1/apps/{appId}/email-deliverability/postmaster`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn postmaster(&self) -> Result<PostmasterStats> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/email-deliverability/postmaster");
        self.client.execute(op, &self.opts)
    }

    /// Google Postmaster dünkü itibarını oku
    ///
    /// `POST /v1/apps/{appId}/email-deliverability/postmaster/sync`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn sync_postmaster(&self) -> Result<PostmasterStats> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/email-deliverability/postmaster/sync");
        self.client.execute(op, &self.opts)
    }
}

/// `featureFlags` kaynağı.
/// `Client::feature_flags()` ile alınır.
#[derive(Debug, Clone)]
pub struct FeatureFlagsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> FeatureFlagsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Özellik bayrakları
    ///
    /// `GET /v1/apps/{appId}/feature-flags`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<AppFlagList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/feature-flags");
        self.client.execute(op, &self.opts)
    }

    /// Bayrak oluştur
    ///
    /// `POST /v1/apps/{appId}/feature-flags`
    ///
    /// Değerler uygulama kimliğini bilen herkesçe okunabilir; gizli bilgi yazmayın.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateAppFlagDto) -> Result<AppFlag> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/feature-flags");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Bayrakları bir kullanıcı için değerlendir
    ///
    /// `POST /v1/apps/{appId}/feature-flags/evaluate`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn evaluate(&self, body: &EvaluateAppFlagsDto) -> Result<AppFlagEvaluation> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/feature-flags/evaluate");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Bayrak ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/feature-flags/{flagId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, flag_id: &str) -> Result<AppFlag> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/feature-flags/{flagId}");
        op.path_param("flagId", flag_id);
        self.client.execute(op, &self.opts)
    }

    /// Bayrağı güncelle
    ///
    /// `PATCH /v1/apps/{appId}/feature-flags/{flagId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, flag_id: &str, body: &UpdateAppFlagDto) -> Result<AppFlag> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/feature-flags/{flagId}");
        op.path_param("flagId", flag_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Bayrağı sil
    ///
    /// `DELETE /v1/apps/{appId}/feature-flags/{flagId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, flag_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/feature-flags/{flagId}");
        op.path_param("flagId", flag_id);
        self.client.execute(op, &self.opts)
    }
}

/// `forms` kaynağı.
/// `Client::forms()` ile alınır.
#[derive(Debug, Clone)]
pub struct FormsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `FormsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"active"`, `"archived"`.
    pub status: Option<String>,
}

impl FormsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

/// `FormsService::submissions` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormsSubmissionsParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"accepted"`, `"pending_confirmation"`, `"confirmed"`.
    pub status: Option<String>,
    pub since: Option<String>,
}

impl FormsSubmissionsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.since {
            op.query("since", v);
        }
    }
}

impl<'a> FormsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Formlar
    ///
    /// `GET /v1/apps/{appId}/forms`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&FormsListParams>) -> Result<FormPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/forms");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&FormsListParams>) -> Paginator<'a, Form> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Form oluştur
    ///
    /// `POST /v1/apps/{appId}/forms`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateFormDto) -> Result<Form> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/forms");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Form detayı
    ///
    /// `GET /v1/apps/{appId}/forms/{formId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, form_id: &str) -> Result<Form> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/forms/{formId}");
        op.path_param("formId", form_id);
        self.client.execute(op, &self.opts)
    }

    /// Form gönderimleri
    ///
    /// `GET /v1/apps/{appId}/forms/{formId}/submissions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn submissions(&self, form_id: &str, params: Option<&FormsSubmissionsParams>) -> Result<FormSubmissionPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/forms/{formId}/submissions");
        op.path_param("formId", form_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `submissions` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn submissions_all(&self, form_id: &str, params: Option<&FormsSubmissionsParams>) -> Paginator<'a, FormSubmission> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let form_id = form_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.submissions(&form_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Sunucu tarafı form gönderimi (`external_id` verilirse o kullanıcının profili güncellenir)
    ///
    /// `POST /v1/apps/{appId}/forms/{formId}/submissions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn submit(&self, form_id: &str, body: &PublicFormSubmitDto) -> Result<FormSubmitResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/forms/{formId}/submissions");
        op.path_param("formId", form_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `geofences` kaynağı.
/// `Client::geofences()` ile alınır.
#[derive(Debug, Clone)]
pub struct GeofencesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> GeofencesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Bölgeler
    ///
    /// `GET /v1/apps/{appId}/geofences`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<GeofenceList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/geofences");
        self.client.execute(op, &self.opts)
    }

    /// Bölge oluştur
    ///
    /// `POST /v1/apps/{appId}/geofences`
    ///
    /// Konum paylaşan abonelik bölgeye girip çıktığında `geofence_enter` ve `geofence_exit` olayları oluşur.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateGeofenceDto) -> Result<Geofence> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/geofences");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Bölge ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/geofences/{geofenceId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, geofence_id: &str) -> Result<Geofence> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/geofences/{geofenceId}");
        op.path_param("geofenceId", geofence_id);
        self.client.execute(op, &self.opts)
    }

    /// Bölgeyi güncelle
    ///
    /// `PATCH /v1/apps/{appId}/geofences/{geofenceId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, geofence_id: &str, body: &UpdateGeofenceDto) -> Result<Geofence> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/geofences/{geofenceId}");
        op.path_param("geofenceId", geofence_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Bölgeyi sil
    ///
    /// `DELETE /v1/apps/{appId}/geofences/{geofenceId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, geofence_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/geofences/{geofenceId}");
        op.path_param("geofenceId", geofence_id);
        self.client.execute(op, &self.opts)
    }
}

/// `globalControl` kaynağı.
/// `Client::global_control()` ile alınır.
#[derive(Debug, Clone)]
pub struct GlobalControlService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `GlobalControlService::report` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GlobalControlReportParams {
    pub days: Option<i64>,
    pub event: Option<String>,
}

impl GlobalControlReportParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.days {
            op.query("days", v);
        }
        if let Some(v) = &self.event {
            op.query("event", v);
        }
    }
}

impl<'a> GlobalControlService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Genel kontrol grubu ayarı
    ///
    /// `GET /v1/apps/{appId}/global-control`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self) -> Result<GlobalControl> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/global-control");
        self.client.execute(op, &self.opts)
    }

    /// Genel kontrol grubunu ayarla
    ///
    /// `PUT /v1/apps/{appId}/global-control`
    ///
    /// Gruptaki kullanıcılar kampanya, otomasyon ve uygulama içi mesajları almaz. API ve işlemsel gönderimler etkilenmez.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, body: &UpdateGlobalControlDto) -> Result<GlobalControl> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/global-control");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Grubu yeniden kur (üyelik değişir, ölçüm baştan başlar)
    ///
    /// `POST /v1/apps/{appId}/global-control/reset`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn reset(&self) -> Result<GlobalControl> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/global-control/reset");
        self.client.execute(op, &self.opts)
    }

    /// Kontrol ve mesaj alan grubun dönüşüm karşılaştırması
    ///
    /// `GET /v1/apps/{appId}/global-control/report`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn report(&self, params: Option<&GlobalControlReportParams>) -> Result<GlobalControlReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/global-control/report");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// `identityKeys` kaynağı.
/// `Client::identity_keys()` ile alınır.
#[derive(Debug, Clone)]
pub struct IdentityKeysService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> IdentityKeysService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// SDK kimlik doğrulama (JWT) genel anahtarları
    ///
    /// `GET /v1/apps/{appId}/identity-keys`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<IdentityKeyList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/identity-keys");
        self.client.execute(op, &self.opts)
    }

    /// Genel anahtar ekle (ES256)
    ///
    /// `POST /v1/apps/{appId}/identity-keys`
    ///
    /// SDK JWT identity verification için ES256 (EC P-256) genel anahtarı ekler; özel anahtar gönderilmez. Belirteç sunucunuzda imzalanır (`sub` = external_id, `aud` = app_id, `exp` ≤ 24 saat) ve SDK isteklerinde `Authorization: Bearer <jwt>` başlığıyla taşınır. Kip app ayarında `identity_verification_mode` ile seçilir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateIdentityKeyDto) -> Result<IdentityKey> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/identity-keys");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Genel anahtarı iptal et
    ///
    /// `DELETE /v1/apps/{appId}/identity-keys/{keyId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn revoke(&self, key_id: &str) -> Result<IdentityKey> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/identity-keys/{keyId}");
        op.path_param("keyId", key_id);
        self.client.execute(op, &self.opts)
    }
}

/// `integrations` kaynağı.
/// `Client::integrations()` ile alınır.
#[derive(Debug, Clone)]
pub struct IntegrationsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `IntegrationsService::samples` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IntegrationsSamplesParams {
    pub event_name: Option<String>,
}

impl IntegrationsSamplesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.event_name {
            op.query("event_name", v);
        }
    }
}

impl<'a> IntegrationsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Zapier bağlantı testi: anahtarın app’i ve kapsamları
    ///
    /// `GET /v1/apps/{appId}/integrations/zapier/me`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn me(&self) -> Result<ZapierMe> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/integrations/zapier/me");
        self.client.execute(op, &self.opts)
    }

    /// REST Hook aboneliği (Zap açıldı)
    ///
    /// `POST /v1/apps/{appId}/integrations/zapier/hooks`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn subscribe(&self, body: &ZapierSubscribeDto) -> Result<ZapierHook> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/integrations/zapier/hooks");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// REST Hook aboneliğini sil (Zap kapatıldı)
    ///
    /// `DELETE /v1/apps/{appId}/integrations/zapier/hooks/{hookId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn unsubscribe(&self, hook_id: &str) -> Result<IntegrationOk> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/integrations/zapier/hooks/{hookId}");
        op.path_param("hookId", hook_id);
        self.client.execute(op, &self.opts)
    }

    /// Tetik örnekleri (performList): son kayıtlar, abonelik gövdesiyle aynı biçim
    ///
    /// `GET /v1/apps/{appId}/integrations/zapier/samples`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn samples(&self, event: &str, params: Option<&IntegrationsSamplesParams>) -> Result<Vec<ZapierSamplesItem>> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/integrations/zapier/samples");
        op.query("event", event);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// `journeys` kaynağı.
/// `Client::journeys()` ile alınır.
#[derive(Debug, Clone)]
pub struct JourneysService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `JourneysService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JourneysListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    pub status: Option<String>,
    pub q: Option<String>,
}

impl JourneysListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
    }
}

/// `JourneysService::stats` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JourneysStatsParams {
    pub version: Option<i64>,
    /// Değerler: `"true"`, `"false"`.
    pub all_versions: Option<String>,
}

impl JourneysStatsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.version {
            op.query("version", v);
        }
        if let Some(v) = &self.all_versions {
            op.query("all_versions", v);
        }
    }
}

/// `JourneysService::runs` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JourneysRunsParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"active"`, `"waiting"`, `"completed"`, `"exited"`, `"failed"`.
    pub status: Option<String>,
    pub user_id: Option<String>,
    pub version: Option<i64>,
}

impl JourneysRunsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
        if let Some(v) = &self.user_id {
            op.query("user_id", v);
        }
        if let Some(v) = &self.version {
            op.query("version", v);
        }
    }
}

impl<'a> JourneysService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Journey listesi
    ///
    /// `GET /v1/apps/{appId}/journeys`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&JourneysListParams>) -> Result<JourneyPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/journeys");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&JourneysListParams>) -> Paginator<'a, Journey> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Journey oluştur (taslak)
    ///
    /// `POST /v1/apps/{appId}/journeys`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &JourneysCreateBody) -> Result<JourneyDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/journeys");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Journey detayı
    ///
    /// `GET /v1/apps/{appId}/journeys/{journeyId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, journey_id: &str) -> Result<JourneyDetail> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/journeys/{journeyId}");
        op.path_param("journeyId", journey_id);
        self.client.execute(op, &self.opts)
    }

    /// Ad / açıklama
    ///
    /// `PATCH /v1/apps/{appId}/journeys/{journeyId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, journey_id: &str, body: &UpdateJourneyDto) -> Result<JourneyDetail> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/journeys/{journeyId}");
        op.path_param("journeyId", journey_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Hiç yayınlanmamış journey'i sil
    ///
    /// `DELETE /v1/apps/{appId}/journeys/{journeyId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, journey_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/journeys/{journeyId}");
        op.path_param("journeyId", journey_id);
        self.client.execute(op, &self.opts)
    }

    /// Taslağı kaydet
    ///
    /// `PUT /v1/apps/{appId}/journeys/{journeyId}/draft`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn save_draft(&self, journey_id: &str, body: &JourneysSaveDraftBody) -> Result<JourneyDetail> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/journeys/{journeyId}/draft");
        op.path_param("journeyId", journey_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Taslaktaki düğümü güncelle
    ///
    /// `PATCH /v1/apps/{appId}/journeys/{journeyId}/nodes/{nodeId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_node(&self, journey_id: &str, node_id: &str, body: &UpdateNodeDto) -> Result<JourneyDetail> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/journeys/{journeyId}/nodes/{nodeId}");
        op.path_param("journeyId", journey_id);
        op.path_param("nodeId", node_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Sürümler
    ///
    /// `GET /v1/apps/{appId}/journeys/{journeyId}/versions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn versions(&self, journey_id: &str) -> Result<JourneyVersionList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/journeys/{journeyId}/versions");
        op.path_param("journeyId", journey_id);
        self.client.execute(op, &self.opts)
    }

    /// Eski sürümü yeni taslak olarak geri yükle (yayındaki sürüm değişmez)
    ///
    /// `POST /v1/apps/{appId}/journeys/{journeyId}/versions/{version}/restore`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn restore_version(&self, journey_id: &str, version: &str) -> Result<JourneyDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/journeys/{journeyId}/versions/{version}/restore");
        op.path_param("journeyId", journey_id);
        op.path_param("version", version);
        self.client.execute(op, &self.opts)
    }

    /// Yayınla
    ///
    /// `POST /v1/apps/{appId}/journeys/{journeyId}/publish`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn publish(&self, journey_id: &str) -> Result<JourneyPublishResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/journeys/{journeyId}/publish");
        op.path_param("journeyId", journey_id);
        self.client.execute(op, &self.opts)
    }

    /// Duraklat
    ///
    /// `POST /v1/apps/{appId}/journeys/{journeyId}/pause`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn pause(&self, journey_id: &str) -> Result<JourneyDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/journeys/{journeyId}/pause");
        op.path_param("journeyId", journey_id);
        self.client.execute(op, &self.opts)
    }

    /// Devam ettir
    ///
    /// `POST /v1/apps/{appId}/journeys/{journeyId}/resume`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn resume(&self, journey_id: &str) -> Result<JourneyDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/journeys/{journeyId}/resume");
        op.path_param("journeyId", journey_id);
        self.client.execute(op, &self.opts)
    }

    /// Arşivle
    ///
    /// `POST /v1/apps/{appId}/journeys/{journeyId}/archive`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn archive(&self, journey_id: &str) -> Result<JourneyDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/journeys/{journeyId}/archive");
        op.path_param("journeyId", journey_id);
        self.client.execute(op, &self.opts)
    }

    /// Kopyala
    ///
    /// `POST /v1/apps/{appId}/journeys/{journeyId}/duplicate`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn duplicate(&self, journey_id: &str) -> Result<JourneyDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/journeys/{journeyId}/duplicate");
        op.path_param("journeyId", journey_id);
        self.client.execute(op, &self.opts)
    }

    /// Düğüm bazlı istatistik
    ///
    /// `GET /v1/apps/{appId}/journeys/{journeyId}/stats`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn stats(&self, journey_id: &str, params: Option<&JourneysStatsParams>) -> Result<JourneyStats> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/journeys/{journeyId}/stats");
        op.path_param("journeyId", journey_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Run listesi
    ///
    /// `GET /v1/apps/{appId}/journeys/{journeyId}/runs`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn runs(&self, journey_id: &str, params: Option<&JourneysRunsParams>) -> Result<JourneyRunPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/journeys/{journeyId}/runs");
        op.path_param("journeyId", journey_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `runs` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn runs_all(&self, journey_id: &str, params: Option<&JourneysRunsParams>) -> Paginator<'a, JourneyRun> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let journey_id = journey_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.runs(&journey_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Kullanıcıyı journey'e sok (external_id, user_id veya alias; context Liquid custom_data)
    ///
    /// `POST /v1/apps/{appId}/journeys/{journeyId}/runs`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn trigger(&self, journey_id: &str, body: &TriggerRunDto) -> Result<JourneyTriggerResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/journeys/{journeyId}/runs");
        op.path_param("journeyId", journey_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Run detayı ve adımlar
    ///
    /// `GET /v1/apps/{appId}/journeys/{journeyId}/runs/{runId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn run(&self, journey_id: &str, run_id: &str) -> Result<JourneyRunDetail> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/journeys/{journeyId}/runs/{runId}");
        op.path_param("journeyId", journey_id);
        op.path_param("runId", run_id);
        self.client.execute(op, &self.opts)
    }

    /// Run'ı sonlandır
    ///
    /// `DELETE /v1/apps/{appId}/journeys/{journeyId}/runs/{runId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn exit_run(&self, journey_id: &str, run_id: &str) -> Result<JourneyRunDetail> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/journeys/{journeyId}/runs/{runId}");
        op.path_param("journeyId", journey_id);
        op.path_param("runId", run_id);
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcının journey run'ları
    ///
    /// `GET /v1/apps/{appId}/users/by/{label}/{value}/journeys`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_for_user(&self, label: &str, value: &str) -> Result<UserJourneyRuns> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/users/by/{label}/{value}/journeys");
        op.path_param("label", label);
        op.path_param("value", value);
        self.client.execute(op, &self.opts)
    }
}

/// `landingPages` kaynağı.
/// `Client::landing_pages()` ile alınır.
#[derive(Debug, Clone)]
pub struct LandingPagesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `LandingPagesService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LandingPagesListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"draft"`, `"published"`, `"archived"`.
    pub status: Option<String>,
}

impl LandingPagesListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

impl<'a> LandingPagesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Landing sayfaları
    ///
    /// `GET /v1/apps/{appId}/landing-pages`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&LandingPagesListParams>) -> Result<LandingPageList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/landing-pages");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&LandingPagesListParams>) -> Paginator<'a, LandingPageListDataItem> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Landing sayfası detayı
    ///
    /// `GET /v1/apps/{appId}/landing-pages/{pageId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, page_id: &str) -> Result<LandingPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/landing-pages/{pageId}");
        op.path_param("pageId", page_id);
        self.client.execute(op, &self.opts)
    }

    /// Taslağı yayınla
    ///
    /// `POST /v1/apps/{appId}/landing-pages/{pageId}/publish`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn publish(&self, page_id: &str) -> Result<LandingPage> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/landing-pages/{pageId}/publish");
        op.path_param("pageId", page_id);
        self.client.execute(op, &self.opts)
    }

    /// Yayından kaldır
    ///
    /// `POST /v1/apps/{appId}/landing-pages/{pageId}/unpublish`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn unpublish(&self, page_id: &str) -> Result<LandingPage> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/landing-pages/{pageId}/unpublish");
        op.path_param("pageId", page_id);
        self.client.execute(op, &self.opts)
    }
}

/// `liveActivities` kaynağı.
/// `Client::live_activities()` ile alınır.
#[derive(Debug, Clone)]
pub struct LiveActivitiesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> LiveActivitiesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Live Activity türleri
    ///
    /// `GET /v1/apps/{appId}/live-activities/types`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_types(&self) -> Result<LiveActivityTypeList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/live-activities/types");
        self.client.execute(op, &self.opts)
    }

    /// Live Activity türü oluştur
    ///
    /// `POST /v1/apps/{appId}/live-activities/types`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_type(&self, body: &CreateLiveActivityTypeDto) -> Result<LiveActivityType> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/live-activities/types");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Live Activity türünü güncelle
    ///
    /// `PATCH /v1/apps/{appId}/live-activities/types/{activityType}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_type(&self, activity_type: &str, body: &UpdateLiveActivityTypeDto) -> Result<LiveActivityType> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/live-activities/types/{activityType}");
        op.path_param("activityType", activity_type);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Live Activity türünü sil
    ///
    /// `DELETE /v1/apps/{appId}/live-activities/types/{activityType}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete_type(&self, activity_type: &str) -> Result<LiveActivityTokenResult> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/live-activities/types/{activityType}");
        op.path_param("activityType", activity_type);
        self.client.execute(op, &self.opts)
    }

    /// Live Activity başlat (push-to-start)
    ///
    /// `POST /v1/apps/{appId}/live-activities/{activityType}/start`
    ///
    /// Hedefteki iOS aboneliklerinin push-to-start belirteçleriyle etkinliği uzaktan başlatır (iOS 17.2+). `attributes` ve `content_state` türün şemasına uymalıdır. Dönen `activity_id` ile sonraki update/end yapılır.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn start(&self, activity_type: &str, body: &StartLiveActivityDto) -> Result<LiveActivitySendResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/live-activities/{activityType}/start");
        op.path_param("activityType", activity_type);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Live Activity güncelle
    ///
    /// `POST /v1/apps/{appId}/live-activities/{activityType}/update`
    ///
    /// Etkinliğin cihazlardan bildirilen güncelleme belirteçlerine yeni içerik durumu gönderir. Sessiz saat ve frekans sınırı uygulanmaz; priority 5 Apple bütçesini korur.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, activity_type: &str, body: &UpdateLiveActivityDto) -> Result<LiveActivitySendResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/live-activities/{activityType}/update");
        op.path_param("activityType", activity_type);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Live Activity bitir
    ///
    /// `POST /v1/apps/{appId}/live-activities/{activityType}/end`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn end(&self, activity_type: &str, body: &EndLiveActivityDto) -> Result<LiveActivitySendResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/live-activities/{activityType}/end");
        op.path_param("activityType", activity_type);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `mcp` kaynağı.
/// `Client::mcp()` ile alınır.
#[derive(Debug, Clone)]
pub struct McpService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> McpService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Kampanya taslağı oluştur (MCP onaylı mod; gönderim yapmaz)
    ///
    /// `POST /v1/apps/{appId}/mcp/campaign-drafts`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_campaign_draft(&self, body: &McpCreateCampaignDraftBody) -> Result<Campaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/mcp/campaign-drafts");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Segment taslağı oluştur (MCP onaylı mod; dinamik segment)
    ///
    /// `POST /v1/apps/{appId}/mcp/segment-drafts`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_segment_draft(&self, body: &McpCreateSegmentDraftBody) -> Result<Segment> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/mcp/segment-drafts");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Şablon oluştur (MCP onaylı mod; şablon tek başına gönderim yapmaz)
    ///
    /// `POST /v1/apps/{appId}/mcp/template-drafts`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_template_draft(&self, body: &CreateTemplateDto) -> Result<Template> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/mcp/template-drafts");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Journey taslağı oluştur (MCP onaylı mod; yayımlanmaz)
    ///
    /// `POST /v1/apps/{appId}/mcp/journey-drafts`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_journey_draft(&self, body: &McpCreateJourneyDraftBody) -> Result<JourneyDetail> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/mcp/journey-drafts");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// In-app mesaj taslağı oluştur (MCP onaylı mod; etkinleştirilmez)
    ///
    /// `POST /v1/apps/{appId}/mcp/in-app-drafts`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_in_app_draft(&self, body: &CreateInAppDto) -> Result<InAppMessage> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/mcp/in-app-drafts");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Test gönderimi (yalnızca app’in test cihazlarına; MCP onaylı mod)
    ///
    /// `POST /v1/apps/{appId}/mcp/test-sends`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn test_send(&self, body: &TestPushDto) -> Result<CreateNotificationResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/mcp/test-sends");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `predictions` kaynağı.
/// `Client::predictions()` ile alınır.
#[derive(Debug, Clone)]
pub struct PredictionsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `PredictionsService::users` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PredictionsUsersParams {
    /// Değerler: `"low"`, `"medium"`, `"high"`.
    pub bucket: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl PredictionsUsersParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.bucket {
            op.query("bucket", v);
        }
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> PredictionsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Öngörü modelleri
    ///
    /// `GET /v1/apps/{appId}/predictions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<PredictionModelList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/predictions");
        self.client.execute(op, &self.opts)
    }

    /// Öngörü modeli oluştur
    ///
    /// `POST /v1/apps/{appId}/predictions`
    ///
    /// Eğitim kuyruğa alınır; sonuç `outcome` alanından izlenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreatePredictionDto) -> Result<PredictionModel> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/predictions");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Öngörü modeli
    ///
    /// `GET /v1/apps/{appId}/predictions/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, id: &str) -> Result<PredictionModel> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/predictions/{id}");
        op.path_param("id", id);
        self.client.execute(op, &self.opts)
    }

    /// Öngörü modelini güncelle
    ///
    /// `PATCH /v1/apps/{appId}/predictions/{id}`
    ///
    /// Olay veya ufuk değişirse yeniden eğitilir; kapatınca skorlar silinir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, id: &str, body: &UpdatePredictionDto) -> Result<PredictionModel> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/predictions/{id}");
        op.path_param("id", id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Öngörü modelini sil
    ///
    /// `DELETE /v1/apps/{appId}/predictions/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, id: &str) -> Result<()> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/predictions/{id}");
        op.path_param("id", id);
        self.client.execute_empty(op, &self.opts)
    }

    /// Modeli yeniden eğit
    ///
    /// `POST /v1/apps/{appId}/predictions/{id}/compute`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn compute(&self, id: &str) -> Result<PredictionModel> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/predictions/{id}/compute");
        op.path_param("id", id);
        self.client.execute(op, &self.opts)
    }

    /// Skoru olan kullanıcılar
    ///
    /// `GET /v1/apps/{appId}/predictions/{id}/users`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn users(&self, id: &str, params: Option<&PredictionsUsersParams>) -> Result<PredictionUsersPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/predictions/{id}/users");
        op.path_param("id", id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// `preferences` kaynağı.
/// `Client::preferences()` ile alınır.
#[derive(Debug, Clone)]
pub struct PreferencesService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> PreferencesService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Kullanıcının tercihleri (alias ile)
    ///
    /// `GET /v1/apps/{appId}/users/by/{label}/{value}/preferences`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, label: &str, value: &str) -> Result<UserPreferences> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/users/by/{label}/{value}/preferences");
        op.path_param("label", label);
        op.path_param("value", value);
        self.client.execute(op, &self.opts)
    }

    /// Kullanıcı tercihlerini yaz (alias ile)
    ///
    /// `PUT /v1/apps/{appId}/users/by/{label}/{value}/preferences`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn set(&self, label: &str, value: &str, body: &SetPreferencesDto) -> Result<UserPreferences> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/users/by/{label}/{value}/preferences");
        op.path_param("label", label);
        op.path_param("value", value);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `promoCodeLists` kaynağı.
/// `Client::promo_code_lists()` ile alınır.
#[derive(Debug, Clone)]
pub struct PromoCodeListsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `PromoCodeListsService::upload_promo_list` için multipart/form-data alanları.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PromoCodeListsUploadPromoListForm {
    /// Zorunlu. UTF-8 CSV; kodlar ilk sütunda, başlık isteğe bağlı. En fazla 1.000.000 kod, 20 MB.
    pub file: Option<FileUpload>,
}

/// `PromoCodeListsService::promo_codes` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PromoCodeListsPromoCodesParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub q: Option<String>,
    /// Değerler: `"available"`, `"assigned"`.
    pub status: Option<String>,
}

impl PromoCodeListsPromoCodesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.offset {
            op.query("offset", v);
        }
        if let Some(v) = &self.q {
            op.query("q", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

impl<'a> PromoCodeListsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Promosyon kodu listeleri
    ///
    /// `GET /v1/apps/{appId}/promo-code-lists`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_promo_lists(&self) -> Result<PromoCodeListList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/promo-code-lists");
        self.client.execute(op, &self.opts)
    }

    /// Promosyon kodu listesi oluştur
    ///
    /// `POST /v1/apps/{appId}/promo-code-lists`
    ///
    /// Şablonda `{% promo_code '<ad>' %}` teslimat başına tek kullanımlık kod verir. Kod kalmazsa teslimat `PromoCodeUnavailable` hatasıyla düşer.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_promo_list(&self, body: &CreatePromoCodeListDto) -> Result<PromoCodeList> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/promo-code-lists");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Promosyon kodu listesi ayrıntısı
    ///
    /// `GET /v1/apps/{appId}/promo-code-lists/{listId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_promo_list(&self, list_id: &str) -> Result<PromoCodeList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/promo-code-lists/{listId}");
        op.path_param("listId", list_id);
        self.client.execute(op, &self.opts)
    }

    /// Promosyon kodu listesini güncelle (açıklama, son kullanma)
    ///
    /// `PATCH /v1/apps/{appId}/promo-code-lists/{listId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_promo_list(&self, list_id: &str, body: &UpdatePromoCodeListDto) -> Result<PromoCodeList> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/promo-code-lists/{listId}");
        op.path_param("listId", list_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Promosyon kodu listesini sil
    ///
    /// `DELETE /v1/apps/{appId}/promo-code-lists/{listId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete_promo_list(&self, list_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/promo-code-lists/{listId}");
        op.path_param("listId", list_id);
        self.client.execute(op, &self.opts)
    }

    /// CSV ile kod ekle (zaten olanlar atlanır)
    ///
    /// `POST /v1/apps/{appId}/promo-code-lists/{listId}/codes`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn upload_promo_list(&self, list_id: &str, form: &PromoCodeListsUploadPromoListForm) -> Result<PromoCodeUploadResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/promo-code-lists/{listId}/codes");
        op.path_param("listId", list_id);
        let mut mp = Multipart::new();
        if let Some(v) = &form.file {
            mp.file("file", v);
        }
        op.multipart = Some(mp);
        self.client.execute(op, &self.opts)
    }

    /// Listedeki kodlar ve atamaları
    ///
    /// `GET /v1/apps/{appId}/promo-code-lists/{listId}/codes`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn promo_codes(&self, list_id: &str, params: Option<&PromoCodeListsPromoCodesParams>) -> Result<PromoCodesPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/promo-code-lists/{listId}/codes");
        op.path_param("listId", list_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// `recommendations` kaynağı.
/// `Client::recommendations()` ile alınır.
#[derive(Debug, Clone)]
pub struct RecommendationsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `RecommendationsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecommendationsListParams {
    pub limit: Option<i64>,
    pub user_id: Option<String>,
    pub external_id: Option<String>,
    pub item_id: Option<String>,
    pub category: Option<String>,
}

impl RecommendationsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.user_id {
            op.query("user_id", v);
        }
        if let Some(v) = &self.external_id {
            op.query("external_id", v);
        }
        if let Some(v) = &self.item_id {
            op.query("item_id", v);
        }
        if let Some(v) = &self.category {
            op.query("category", v);
        }
    }
}

impl<'a> RecommendationsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Ürün önerileri
    ///
    /// `GET /v1/apps/{appId}/catalogs/{catalogId}/recommendations`
    ///
    /// Stokta olmayan ve kullanıcının satın aldığı ürünler çıkmaz. Sonuç yetmezse yedek kaynakla tamamlanır.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, catalog_id: &str, strategy: &str, params: Option<&RecommendationsListParams>) -> Result<Recommendations> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/catalogs/{catalogId}/recommendations");
        op.path_param("catalogId", catalog_id);
        op.query("strategy", strategy);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Öneri modeli
    ///
    /// `GET /v1/apps/{appId}/catalogs/{catalogId}/recommendations/config`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_config(&self, catalog_id: &str) -> Result<RecommendationConfig> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/catalogs/{catalogId}/recommendations/config");
        op.path_param("catalogId", catalog_id);
        self.client.execute(op, &self.opts)
    }

    /// Öneri modelini ayarla
    ///
    /// `PUT /v1/apps/{appId}/catalogs/{catalogId}/recommendations/config`
    ///
    /// Model açıksa öneriler kayıttan sonra yeniden hesaplanır.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_config(&self, catalog_id: &str, body: &UpdateRecommendationConfigDto) -> Result<RecommendationConfig> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/catalogs/{catalogId}/recommendations/config");
        op.path_param("catalogId", catalog_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Önerileri yeniden hesapla
    ///
    /// `POST /v1/apps/{appId}/catalogs/{catalogId}/recommendations/compute`
    ///
    /// İş kuyruğa alınır; ilerleme `status` alanından izlenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn compute(&self, catalog_id: &str) -> Result<RecommendationConfig> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/catalogs/{catalogId}/recommendations/compute");
        op.path_param("catalogId", catalog_id);
        self.client.execute(op, &self.opts)
    }
}

/// `reverseEtl` kaynağı.
/// `Client::reverse_etl()` ile alınır.
#[derive(Debug, Clone)]
pub struct ReverseEtlService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `ReverseEtlService::runs` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReverseEtlRunsParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl ReverseEtlRunsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> ReverseEtlService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Depodan içeri aktarımlar
    ///
    /// `GET /v1/apps/{appId}/reverse-etl`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<ReverseEtlSyncList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/reverse-etl");
        self.client.execute(op, &self.opts)
    }

    /// İçeri aktarım oluştur
    ///
    /// `POST /v1/apps/{appId}/reverse-etl`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &serde_json::Value) -> Result<ReverseEtlSync> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/reverse-etl");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kaydetmeden bağlantı testi
    ///
    /// `POST /v1/apps/{appId}/reverse-etl/test-connection`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn test(&self, body: &serde_json::Value) -> Result<WarehouseConnectionResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/reverse-etl/test-connection");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// İçeri aktarım detayı
    ///
    /// `GET /v1/apps/{appId}/reverse-etl/{syncId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, sync_id: &str) -> Result<ReverseEtlSync> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/reverse-etl/{syncId}");
        op.path_param("syncId", sync_id);
        self.client.execute(op, &self.opts)
    }

    /// İçeri aktarımı güncelle
    ///
    /// `PATCH /v1/apps/{appId}/reverse-etl/{syncId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, sync_id: &str, body: &UpdateReverseEtlDto) -> Result<ReverseEtlSync> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/reverse-etl/{syncId}");
        op.path_param("syncId", sync_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// İçeri aktarımı sil
    ///
    /// `DELETE /v1/apps/{appId}/reverse-etl/{syncId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, sync_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/reverse-etl/{syncId}");
        op.path_param("syncId", sync_id);
        self.client.execute(op, &self.opts)
    }

    /// Kayıtlı içeri aktarımın bağlantı testi
    ///
    /// `POST /v1/apps/{appId}/reverse-etl/{syncId}/test`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn test_saved(&self, sync_id: &str) -> Result<WarehouseConnectionResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/reverse-etl/{syncId}/test");
        op.path_param("syncId", sync_id);
        self.client.execute(op, &self.opts)
    }

    /// Şimdi içeri aktar
    ///
    /// `POST /v1/apps/{appId}/reverse-etl/{syncId}/run`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn run(&self, sync_id: &str) -> Result<ReverseEtlRun> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/reverse-etl/{syncId}/run");
        op.path_param("syncId", sync_id);
        self.client.execute(op, &self.opts)
    }

    /// İçeri aktarım geçmişi
    ///
    /// `GET /v1/apps/{appId}/reverse-etl/{syncId}/runs`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn runs(&self, sync_id: &str, params: Option<&ReverseEtlRunsParams>) -> Result<ReverseEtlRunPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/reverse-etl/{syncId}/runs");
        op.path_param("syncId", sync_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `runs` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn runs_all(&self, sync_id: &str, params: Option<&ReverseEtlRunsParams>) -> Paginator<'a, ReverseEtlRun> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let sync_id = sync_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.runs(&sync_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }
}

/// `rfm` kaynağı.
/// `Client::rfm()` ile alınır.
#[derive(Debug, Clone)]
pub struct RfmService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `RfmService::transitions` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RfmTransitionsParams {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl RfmTransitionsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.from {
            op.query("from", v);
        }
        if let Some(v) = &self.to {
            op.query("to", v);
        }
    }
}

/// `RfmService::users` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RfmUsersParams {
    /// Değerler: `"champions"`, `"loyal"`, `"potential"`, `"new"`, `"promising"`, `"need_attention"`, `"about_to_sleep"`, `"at_risk"`, `"cant_lose"`, `"hibernating"`, `"lost"`.
    pub segment: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl RfmUsersParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.segment {
            op.query("segment", v);
        }
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> RfmService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// RFM modeli
    ///
    /// `GET /v1/apps/{appId}/rfm/config`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn config(&self) -> Result<RfmConfig> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/rfm/config");
        self.client.execute(op, &self.opts)
    }

    /// RFM modelini ayarla
    ///
    /// `PUT /v1/apps/{appId}/rfm/config`
    ///
    /// Model açıksa skorlar kayıttan sonra yeniden hesaplanır.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, body: &UpdateRfmConfigDto) -> Result<RfmConfig> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/rfm/config");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Skorları yeniden hesapla
    ///
    /// `POST /v1/apps/{appId}/rfm/compute`
    ///
    /// İş kuyruğa alınır; ilerleme `status` alanından izlenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn compute(&self) -> Result<RfmConfig> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/rfm/compute");
        self.client.execute(op, &self.opts)
    }

    /// Segment dağılımı
    ///
    /// `GET /v1/apps/{appId}/rfm/overview`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn overview(&self) -> Result<RfmOverview> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/rfm/overview");
        self.client.execute(op, &self.opts)
    }

    /// İki tarih arası segment geçişleri
    ///
    /// `GET /v1/apps/{appId}/rfm/transitions`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn transitions(&self, params: Option<&RfmTransitionsParams>) -> Result<RfmTransitions> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/rfm/transitions");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Skoru olan kullanıcılar
    ///
    /// `GET /v1/apps/{appId}/rfm/users`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn users(&self, params: Option<&RfmUsersParams>) -> Result<RfmUsersPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/rfm/users");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }
}

/// `smsKeywords` kaynağı.
/// `Client::sms_keywords()` ile alınır.
#[derive(Debug, Clone)]
pub struct SmsKeywordsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> SmsKeywordsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// İki yönlü mesaj anahtar sözcükleri (org geneli + bu app) ve yasal (ayrılmış) sözcükler
    ///
    /// `GET /v1/apps/{appId}/sms-keywords`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn keywords(&self) -> Result<SmsKeywordList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/sms-keywords");
        self.client.execute(op, &self.opts)
    }

    /// Anahtar sözcük oluştur (tag ekle/kaldır, olay → journey tetiği, otomatik yanıt, abonelik)
    ///
    /// `POST /v1/apps/{appId}/sms-keywords`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_keyword(&self, body: &CreateKeywordDto) -> Result<SmsKeyword> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/sms-keywords");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Anahtar sözcüğü güncelle
    ///
    /// `PATCH /v1/apps/{appId}/sms-keywords/{keywordId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_keyword(&self, keyword_id: &str, body: &UpdateKeywordDto) -> Result<SmsKeyword> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/sms-keywords/{keywordId}");
        op.path_param("keywordId", keyword_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Anahtar sözcüğü sil
    ///
    /// `DELETE /v1/apps/{appId}/sms-keywords/{keywordId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete_keyword(&self, keyword_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/sms-keywords/{keywordId}");
        op.path_param("keywordId", keyword_id);
        self.client.execute(op, &self.opts)
    }
}

/// `surveys` kaynağı.
/// `Client::surveys()` ile alınır.
#[derive(Debug, Clone)]
pub struct SurveysService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `SurveysService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurveysListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"draft"`, `"active"`, `"closed"`.
    pub status: Option<String>,
}

impl SurveysListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

/// `SurveysService::report` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurveysReportParams {
    pub days: Option<i64>,
}

impl SurveysReportParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.days {
            op.query("days", v);
        }
    }
}

/// `SurveysService::responses` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurveysResponsesParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub text_question: Option<String>,
    /// Değerler: `"promoter"`, `"passive"`, `"detractor"`.
    pub nps_category: Option<String>,
}

impl SurveysResponsesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.text_question {
            op.query("text_question", v);
        }
        if let Some(v) = &self.nps_category {
            op.query("nps_category", v);
        }
    }
}

impl<'a> SurveysService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Anketler
    ///
    /// `GET /v1/apps/{appId}/surveys`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&SurveysListParams>) -> Result<SurveyPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/surveys");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&SurveysListParams>) -> Paginator<'a, Survey> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Anket oluştur (taslak)
    ///
    /// `POST /v1/apps/{appId}/surveys`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateSurveyDto) -> Result<Survey> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/surveys");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Anket detayı
    ///
    /// `GET /v1/apps/{appId}/surveys/{surveyId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, survey_id: &str) -> Result<Survey> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/surveys/{surveyId}");
        op.path_param("surveyId", survey_id);
        self.client.execute(op, &self.opts)
    }

    /// Etkinleştir / kapat
    ///
    /// `POST /v1/apps/{appId}/surveys/{surveyId}/status`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn status(&self, survey_id: &str, body: &SurveyStatusDto) -> Result<Survey> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/surveys/{surveyId}/status");
        op.path_param("surveyId", survey_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// NPS / CSAT raporu
    ///
    /// `GET /v1/apps/{appId}/surveys/{surveyId}/report`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn report(&self, survey_id: &str, params: Option<&SurveysReportParams>) -> Result<SurveyReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/surveys/{surveyId}/report");
        op.path_param("surveyId", survey_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Anket yanıtları
    ///
    /// `GET /v1/apps/{appId}/surveys/{surveyId}/responses`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn responses(&self, survey_id: &str, params: Option<&SurveysResponsesParams>) -> Result<SurveyResponsePage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/surveys/{surveyId}/responses");
        op.path_param("surveyId", survey_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `responses` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn responses_all(&self, survey_id: &str, params: Option<&SurveysResponsesParams>) -> Paginator<'a, SurveyResponse> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let survey_id = survey_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.responses(&survey_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Sunucu tarafı anket yanıtı (`external_id` ile kullanıcıya bağlanır)
    ///
    /// `POST /v1/apps/{appId}/surveys/{surveyId}/responses`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn respond(&self, survey_id: &str, body: &PublicSurveyResponseDto) -> Result<SurveyResponseResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/surveys/{surveyId}/responses");
        op.path_param("surveyId", survey_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `verify` kaynağı.
/// `Client::verify()` ile alınır.
#[derive(Debug, Clone)]
pub struct VerifyService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

impl<'a> VerifyService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// SMS doğrulama kodu gönder
    ///
    /// `POST /v1/apps/{appId}/verify/sms`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn start(&self, body: &StartVerifyDto) -> Result<SmsVerification> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/verify/sms");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// SMS doğrulama kodunu kontrol et
    ///
    /// `POST /v1/apps/{appId}/verify/sms/check`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn check(&self, body: &CheckVerifyDto) -> Result<SmsVerificationCheck> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/verify/sms/check");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `wallet` kaynağı.
/// `Client::wallet()` ile alınır.
#[derive(Debug, Clone)]
pub struct WalletService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `WalletService::list_passes` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WalletListPassesParams {
    pub template_id: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl WalletListPassesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.template_id {
            op.query("template_id", v);
        }
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> WalletService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Wallet yapılandırması
    ///
    /// `GET /v1/apps/{appId}/wallet/config`
    ///
    /// Kimlik bilgisi geri okunamaz, yalnızca durum döner.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_config(&self) -> Result<WalletConfig> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/wallet/config");
        self.client.execute(op, &self.opts)
    }

    /// Wallet kimlik bilgilerini ayarla
    ///
    /// `PUT /v1/apps/{appId}/wallet/config`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_config(&self, body: &UpdateWalletConfigDto) -> Result<WalletConfig> {
        let mut op = Operation::new("PUT", "/v1/apps/{appId}/wallet/config");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Wallet şablonları
    ///
    /// `GET /v1/apps/{appId}/wallet/templates`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_templates(&self) -> Result<WalletTemplateList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/wallet/templates");
        self.client.execute(op, &self.opts)
    }

    /// Wallet şablonu oluştur
    ///
    /// `POST /v1/apps/{appId}/wallet/templates`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create_template(&self, body: &CreateWalletTemplateDto) -> Result<WalletTemplate> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/wallet/templates");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Wallet şablonu
    ///
    /// `GET /v1/apps/{appId}/wallet/templates/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_template(&self, id: &str) -> Result<WalletTemplate> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/wallet/templates/{id}");
        op.path_param("id", id);
        self.client.execute(op, &self.opts)
    }

    /// Wallet şablonunu güncelle
    ///
    /// `PATCH /v1/apps/{appId}/wallet/templates/{id}`
    ///
    /// Tasarım değişirse verilmiş kartlar da güncellenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_template(&self, id: &str, body: &UpdateWalletTemplateDto) -> Result<WalletTemplate> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/wallet/templates/{id}");
        op.path_param("id", id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Wallet şablonunu sil
    ///
    /// `DELETE /v1/apps/{appId}/wallet/templates/{id}`
    ///
    /// Verilmiş kartı olan şablon silinemez, arşivlenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete_template(&self, id: &str) -> Result<()> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/wallet/templates/{id}");
        op.path_param("id", id);
        self.client.execute_empty(op, &self.opts)
    }

    /// Verilmiş kartlar
    ///
    /// `GET /v1/apps/{appId}/wallet/passes`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list_passes(&self, params: Option<&WalletListPassesParams>) -> Result<WalletPassPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/wallet/passes");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Kart ver
    ///
    /// `POST /v1/apps/{appId}/wallet/passes`
    ///
    /// Kullanıcılı kart şablon başına tektir; tekrar istek aynı kartı döndürür.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn issue_pass(&self, body: &IssueWalletPassDto) -> Result<WalletPass> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/wallet/passes");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kart
    ///
    /// `GET /v1/apps/{appId}/wallet/passes/{id}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get_pass(&self, id: &str) -> Result<WalletPass> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/wallet/passes/{id}");
        op.path_param("id", id);
        self.client.execute(op, &self.opts)
    }

    /// Kartı güncelle
    ///
    /// `PATCH /v1/apps/{appId}/wallet/passes/{id}`
    ///
    /// Cihazdaki kart arka planda yenilenir.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update_pass(&self, id: &str, body: &UpdateWalletPassDto) -> Result<WalletPass> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/wallet/passes/{id}");
        op.path_param("id", id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kartı iptal et
    ///
    /// `POST /v1/apps/{appId}/wallet/passes/{id}/void`
    ///
    /// Geri alınmaz; cihazdaki kart geçersiz görünür.
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn void_pass(&self, id: &str) -> Result<WalletPass> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/wallet/passes/{id}/void");
        op.path_param("id", id);
        self.client.execute(op, &self.opts)
    }
}

/// `warehouseExports` kaynağı.
/// `Client::warehouse_exports()` ile alınır.
#[derive(Debug, Clone)]
pub struct WarehouseExportsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `WarehouseExportsService::runs` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WarehouseExportsRunsParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

impl WarehouseExportsRunsParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
    }
}

impl<'a> WarehouseExportsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Dışa aktarılabilir tablolar, kolonlar ve şema sürümleri
    ///
    /// `GET /v1/apps/{appId}/warehouse-exports/tables`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn tables(&self) -> Result<WarehouseTables> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/warehouse-exports/tables");
        self.client.execute(op, &self.opts)
    }

    /// Kaydetmeden bağlantı testi
    ///
    /// `POST /v1/apps/{appId}/warehouse-exports/test-connection`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn test(&self, body: &serde_json::Value) -> Result<WarehouseConnectionResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/warehouse-exports/test-connection");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Veri ambarı dışa aktarımları
    ///
    /// `GET /v1/apps/{appId}/warehouse-exports`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self) -> Result<WarehouseExportList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/warehouse-exports");
        self.client.execute(op, &self.opts)
    }

    /// Dışa aktarım oluştur (kimlik bilgisi şifreli saklanır, yanıtta dönmez)
    ///
    /// `POST /v1/apps/{appId}/warehouse-exports`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &serde_json::Value) -> Result<WarehouseExport> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/warehouse-exports");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Dışa aktarım detayı
    ///
    /// `GET /v1/apps/{appId}/warehouse-exports/{exportId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, export_id: &str) -> Result<WarehouseExport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/warehouse-exports/{exportId}");
        op.path_param("exportId", export_id);
        self.client.execute(op, &self.opts)
    }

    /// Dışa aktarımı güncelle (ad, tablolar, zamanlama, etkin/duraklat, hedef ayarı, kimlik bilgisi)
    ///
    /// `PATCH /v1/apps/{appId}/warehouse-exports/{exportId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, export_id: &str, body: &UpdateWarehouseExportDto) -> Result<WarehouseExport> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/warehouse-exports/{exportId}");
        op.path_param("exportId", export_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Dışa aktarımı sil (hedefteki veriler silinmez)
    ///
    /// `DELETE /v1/apps/{appId}/warehouse-exports/{exportId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn delete(&self, export_id: &str) -> Result<OkModel> {
        let mut op = Operation::new("DELETE", "/v1/apps/{appId}/warehouse-exports/{exportId}");
        op.path_param("exportId", export_id);
        self.client.execute(op, &self.opts)
    }

    /// Kayıtlı dışa aktarımın bağlantı testi
    ///
    /// `POST /v1/apps/{appId}/warehouse-exports/{exportId}/test`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn test_saved(&self, export_id: &str) -> Result<WarehouseConnectionResult> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/warehouse-exports/{exportId}/test");
        op.path_param("exportId", export_id);
        self.client.execute(op, &self.opts)
    }

    /// Şimdi çalıştır (son başarılı imleçten itibaren)
    ///
    /// `POST /v1/apps/{appId}/warehouse-exports/{exportId}/run`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn run(&self, export_id: &str) -> Result<WarehouseExportRun> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/warehouse-exports/{exportId}/run");
        op.path_param("exportId", export_id);
        self.client.execute(op, &self.opts)
    }

    /// Çalıştırma geçmişi (cursor)
    ///
    /// `GET /v1/apps/{appId}/warehouse-exports/{exportId}/runs`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn runs(&self, export_id: &str, params: Option<&WarehouseExportsRunsParams>) -> Result<WarehouseExportRunPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/warehouse-exports/{exportId}/runs");
        op.path_param("exportId", export_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `runs` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn runs_all(&self, export_id: &str, params: Option<&WarehouseExportsRunsParams>) -> Paginator<'a, WarehouseExportRun> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        let export_id = export_id.to_owned();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.runs(&export_id, Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }
}

/// `webCampaigns` kaynağı.
/// `Client::web_campaigns()` ile alınır.
#[derive(Debug, Clone)]
pub struct WebCampaignsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `WebCampaignsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WebCampaignsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    pub status: Option<String>,
}

impl WebCampaignsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

/// `WebCampaignsService::report` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WebCampaignsReportParams {
    pub days: Option<i64>,
}

impl WebCampaignsReportParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.days {
            op.query("days", v);
        }
    }
}

impl<'a> WebCampaignsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Web kampanyaları
    ///
    /// `GET /v1/apps/{appId}/web-campaigns`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&WebCampaignsListParams>) -> Result<WebCampaignPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/web-campaigns");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&WebCampaignsListParams>) -> Paginator<'a, WebCampaign> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Web kampanyası oluştur (taslak)
    ///
    /// `POST /v1/apps/{appId}/web-campaigns`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateWebCampaignDto) -> Result<WebCampaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/web-campaigns");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Web kampanyası detayı ve istatistikleri
    ///
    /// `GET /v1/apps/{appId}/web-campaigns/{campaignId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, campaign_id: &str) -> Result<WebCampaign> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/web-campaigns/{campaignId}");
        op.path_param("campaignId", campaign_id);
        self.client.execute(op, &self.opts)
    }

    /// Web kampanyasını düzenle
    ///
    /// `PATCH /v1/apps/{appId}/web-campaigns/{campaignId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, campaign_id: &str, body: &UpdateWebCampaignDto) -> Result<WebCampaign> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/web-campaigns/{campaignId}");
        op.path_param("campaignId", campaign_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Günlük seri
    ///
    /// `GET /v1/apps/{appId}/web-campaigns/{campaignId}/report`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn report(&self, campaign_id: &str, params: Option<&WebCampaignsReportParams>) -> Result<WebCampaignReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/web-campaigns/{campaignId}/report");
        op.path_param("campaignId", campaign_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Yayınla / duraklat / arşivle
    ///
    /// `POST /v1/apps/{appId}/web-campaigns/{campaignId}/status`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn status(&self, campaign_id: &str, body: &WebCampaignStatusDto) -> Result<WebCampaign> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/web-campaigns/{campaignId}/status");
        op.path_param("campaignId", campaign_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `webPersonalizations` kaynağı.
/// `Client::web_personalizations()` ile alınır.
#[derive(Debug, Clone)]
pub struct WebPersonalizationsService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `WebPersonalizationsService::list` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WebPersonalizationsListParams {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    /// Değerler: `"draft"`, `"active"`, `"paused"`, `"archived"`.
    pub status: Option<String>,
}

impl WebPersonalizationsListParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.limit {
            op.query("limit", v);
        }
        if let Some(v) = &self.cursor {
            op.query("cursor", v);
        }
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

/// `WebPersonalizationsService::report` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WebPersonalizationsReportParams {
    pub days: Option<i64>,
}

impl WebPersonalizationsReportParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.days {
            op.query("days", v);
        }
    }
}

impl<'a> WebPersonalizationsService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Web kişiselleştirme deneyimleri
    ///
    /// `GET /v1/apps/{appId}/web-personalizations`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn list(&self, params: Option<&WebPersonalizationsListParams>) -> Result<WebPersonalizationPage> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/web-personalizations");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// `list` sonuçlarını `next_cursor` ile tüm sayfalar boyunca tek tek döndürür.
    /// Hata olursa yineleyici `Err` verir ve durur.
    pub fn list_all(&self, params: Option<&WebPersonalizationsListParams>) -> Paginator<'a, WebPersonalization> {
        let service = self.clone();
        let mut p = params.cloned().unwrap_or_default();
        Paginator::new(move |cursor| {
            if let Some(c) = cursor {
                p.cursor = Some(c);
            }
            let page = service.list(Some(&p))?;
            Ok((page.data, page.next_cursor, page.has_more))
        })
    }

    /// Kişiselleştirme deneyimi oluştur (taslak)
    ///
    /// `POST /v1/apps/{appId}/web-personalizations`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn create(&self, body: &CreateWebPersonalizationDto) -> Result<WebPersonalization> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/web-personalizations");
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Kişiselleştirme deneyimi detayı ve istatistikleri
    ///
    /// `GET /v1/apps/{appId}/web-personalizations/{personalizationId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn get(&self, personalization_id: &str) -> Result<WebPersonalization> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/web-personalizations/{personalizationId}");
        op.path_param("personalizationId", personalization_id);
        self.client.execute(op, &self.opts)
    }

    /// Kişiselleştirme deneyimini düzenle
    ///
    /// `PATCH /v1/apps/{appId}/web-personalizations/{personalizationId}`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn update(&self, personalization_id: &str, body: &UpdateWebPersonalizationDto) -> Result<WebPersonalization> {
        let mut op = Operation::new("PATCH", "/v1/apps/{appId}/web-personalizations/{personalizationId}");
        op.path_param("personalizationId", personalization_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }

    /// Günlük seri
    ///
    /// `GET /v1/apps/{appId}/web-personalizations/{personalizationId}/report`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn report(&self, personalization_id: &str, params: Option<&WebPersonalizationsReportParams>) -> Result<WebPersonalizationReport> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/web-personalizations/{personalizationId}/report");
        op.path_param("personalizationId", personalization_id);
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// Yayınla / duraklat / arşivle
    ///
    /// `POST /v1/apps/{appId}/web-personalizations/{personalizationId}/status`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn status(&self, personalization_id: &str, body: &WebPersonalizationStatusDto) -> Result<WebPersonalization> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/web-personalizations/{personalizationId}/status");
        op.path_param("personalizationId", personalization_id);
        op.json(body)?;
        self.client.execute(op, &self.opts)
    }
}

/// `whatsapp` kaynağı.
/// `Client::whatsapp()` ile alınır.
#[derive(Debug, Clone)]
pub struct WhatsappService<'a> {
    client: &'a Client,
    opts: RequestOptions,
}

/// `WhatsappService::templates` için isteğe bağlı sorgu parametreleri.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WhatsappTemplatesParams {
    pub status: Option<String>,
}

impl WhatsappTemplatesParams {
    fn append_query(&self, op: &mut Operation) {
        if let Some(v) = &self.status {
            op.query("status", v);
        }
    }
}

impl<'a> WhatsappService<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, opts: RequestOptions::default() }
    }

    /// Bu servisten yapılacak çağrıların ayarları (app, Idempotency-Key, başlık, zaman aşımı).
    pub fn with_options(mut self, opts: RequestOptions) -> Self {
        self.opts = opts;
        self
    }

    /// WhatsApp şablonları (Meta onay durumu ile)
    ///
    /// `GET /v1/apps/{appId}/whatsapp/templates`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn templates(&self, params: Option<&WhatsappTemplatesParams>) -> Result<WhatsAppTemplateList> {
        let mut op = Operation::new("GET", "/v1/apps/{appId}/whatsapp/templates");
        if let Some(p) = params {
            p.append_query(&mut op);
        }
        self.client.execute(op, &self.opts)
    }

    /// WhatsApp şablonlarını Meta Graph API’den senkronla
    ///
    /// `POST /v1/apps/{appId}/whatsapp/templates/sync`
    ///
    /// App kapsamı istemcinin varsayılan app’i veya `RequestOptions::app_id` değeridir.
    pub fn sync(&self) -> Result<WhatsAppTemplateSync> {
        let mut op = Operation::new("POST", "/v1/apps/{appId}/whatsapp/templates/sync");
        self.client.execute(op, &self.opts)
    }
}
