use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteSettings {
    pub id: i64,
    pub brand: String,
    #[serde(alias = "hero_label")]
    pub hero_label: String,
    #[serde(alias = "hero_title")]
    pub hero_title: String,
    #[serde(alias = "hero_subtitle")]
    pub hero_subtitle: String,
    #[serde(alias = "cta_primary")]
    pub cta_primary: String,
    #[serde(alias = "cta_secondary")]
    pub cta_secondary: String,
    #[serde(alias = "updated_at")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteSettingsDto {
    pub id: i64,
    pub brand: String,
    pub hero_label: String,
    pub hero_title: String,
    pub hero_subtitle: String,
    pub cta_primary: String,
    pub cta_secondary: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMetrics {
    pub total_leads: i64,
    pub total_media: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicBootstrap {
    pub settings: SiteSettingsDto,
    pub metrics: PublicMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadSubmissionInput {
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub company: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadRow {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub company: Option<String>,
    pub message: Option<String>,
    pub status: String,
    #[serde(alias = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageSummary {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub status: String,
    #[serde(alias = "published_at")]
    pub published_at: Option<String>,
    #[serde(alias = "updated_at")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: i64,
    pub slug: String,
    pub title: String,
    #[serde(alias = "content_md")]
    pub content_md: String,
    #[serde(alias = "content_html")]
    pub content_html: String,
    pub status: String,
    #[serde(alias = "published_at")]
    pub published_at: Option<String>,
    #[serde(alias = "created_at")]
    pub created_at: String,
    #[serde(alias = "updated_at")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountRow {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionClaims {
    pub email: String,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardResponse {
    pub stats: DashboardStats,
    pub recent_leads: Vec<LeadRow>,
    pub recent_pages: Vec<PageSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    pub total_leads: i64,
    pub total_media: i64,
    pub total_pages: i64,
    pub latest_update_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadTag {
    pub id: i64,
    #[serde(alias = "lead_id")]
    pub lead_id: i64,
    pub tag: String,
    #[serde(alias = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadNote {
    pub id: i64,
    #[serde(alias = "lead_id")]
    pub lead_id: i64,
    pub content: String,
    #[serde(alias = "created_by")]
    pub created_by: String,
    #[serde(alias = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadDetail {
    #[serde(flatten)]
    pub lead: LeadRow,
    pub tags: Vec<LeadTag>,
    pub notes: Vec<LeadNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusInput {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInput {
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteInput {
    pub content: String,
    #[serde(alias = "createdBy")]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminUserRow {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: String,
    #[serde(alias = "avatar_url")]
    pub avatar_url: Option<String>,
    #[serde(alias = "github_login")]
    pub github_login: Option<String>,
    #[serde(alias = "last_login_at")]
    pub last_login_at: Option<String>,
    #[serde(alias = "is_active")]
    pub is_active: i64,
    #[serde(alias = "created_at")]
    pub created_at: String,
    #[serde(alias = "updated_at")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserInput {
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserUpdateInput {
    pub name: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessLogRow {
    pub id: i64,
    #[serde(alias = "user_email")]
    pub user_email: String,
    pub action: String,
    pub path: String,
    pub method: String,
    #[serde(alias = "status_code")]
    pub status_code: Option<i64>,
    #[serde(alias = "ip_address")]
    pub ip_address: Option<String>,
    #[serde(alias = "user_agent")]
    pub user_agent: Option<String>,
    #[serde(alias = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLogRow {
    pub id: i64,
    pub method: String,
    pub path: String,
    #[serde(alias = "status_code")]
    pub status_code: i64,
    #[serde(alias = "duration_ms")]
    pub duration_ms: Option<i64>,
    #[serde(alias = "request_body")]
    pub request_body: Option<String>,
    #[serde(alias = "response_size")]
    pub response_size: Option<i64>,
    #[serde(alias = "ip_address")]
    pub ip_address: Option<String>,
    #[serde(alias = "created_at")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStats {
    pub total_users: i64,
    pub active_users: i64,
    pub total_leads: i64,
    pub total_media: i64,
    pub total_pages: i64,
    pub total_emails: i64,
    pub total_api_requests: i64,
    pub recent_access_logs: Vec<AccessLogRow>,
    pub recent_api_logs: Vec<ApiLogRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLead {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub company: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMedia {
    #[serde(alias = "image_id")]
    pub image_id: String,
    pub title: String,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub leads: Vec<SearchLead>,
    pub media: Vec<SearchMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInput {
    pub slug: String,
    pub title: String,
    #[serde(alias = "contentMd")]
    pub content_md: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplateInput {
    pub name: String,
    pub subject: String,
    #[serde(alias = "bodyHtml")]
    pub body_html: String,
    #[serde(alias = "bodyText")]
    pub body_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailTemplate {
    pub id: i64,
    pub name: String,
    pub subject: String,
    #[serde(alias = "body_html")]
    pub body_html: String,
    #[serde(alias = "body_text")]
    pub body_text: String,
    #[serde(alias = "created_at")]
    pub created_at: String,
    #[serde(alias = "updated_at")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailInput {
    #[serde(alias = "leadId")]
    pub lead_id: i64,
    #[serde(alias = "templateId")]
    pub template_id: Option<i64>,
    pub subject: Option<String>,
    #[serde(alias = "bodyHtml")]
    pub body_html: Option<String>,
    #[serde(alias = "bodyText")]
    pub body_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailLog {
    pub id: i64,
    #[serde(alias = "lead_id")]
    pub lead_id: i64,
    #[serde(alias = "template_id")]
    pub template_id: Option<i64>,
    pub subject: String,
    pub status: String,
    #[serde(alias = "sent_at")]
    pub sent_at: String,
    #[serde(alias = "lead_name")]
    pub lead_name: Option<String>,
    #[serde(alias = "lead_email")]
    pub lead_email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCopySuggestionRequest {
    pub objective: String,
    pub audience: String,
    pub tone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCopySuggestion {
    #[serde(alias = "heroTitle")]
    pub hero_title: String,
    #[serde(alias = "heroSubtitle")]
    pub hero_subtitle: String,
    #[serde(alias = "ctaPrimary")]
    pub cta_primary: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectUploadInput {
    pub title: String,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaAsset {
    pub id: i64,
    #[serde(alias = "image_id")]
    pub image_id: String,
    pub title: String,
    pub alt: Option<String>,
    pub status: String,
    #[serde(alias = "delivery_url")]
    pub delivery_url: Option<String>,
    #[serde(alias = "preview_url")]
    pub preview_url: Option<String>,
    #[serde(alias = "uploaded_at")]
    pub uploaded_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMediaInput {
    pub title: String,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAsset {
    pub name: String,
    pub url: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseSummary {
    pub tag: String,
    pub name: String,
    pub published_at: String,
    pub body: String,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchInput {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorReindexResponse {
    pub count: usize,
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvValueInput {
    pub value: String,
    #[serde(alias = "expirationTtl")]
    pub expiration_ttl: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTaskInput {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNoteInput {
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTask {
    pub id: String,
    pub title: String,
    pub done: bool,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpsAgentState {
    pub tasks: Vec<AgentTask>,
    pub notes: Vec<String>,
}
