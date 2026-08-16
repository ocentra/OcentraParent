use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

#[path = "activity_surface/source_status.rs"]
pub mod source_status;

use source_status::ActivityAppGameSourceStatusRow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivitySurfaceScopeKind {
    #[serde(rename = "family")]
    Family,
    #[serde(rename = "device")]
    Device,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityReportFrequency {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityReportSectionKind {
    #[serde(rename = "summary")]
    Summary,
    #[serde(rename = "screen")]
    Screen,
    #[serde(rename = "app-use")]
    AppUse,
    #[serde(rename = "browser")]
    Browser,
    #[serde(rename = "games")]
    Games,
    #[serde(rename = "network")]
    Network,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityReadModelState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "permission-required")]
    PermissionRequired,
    #[serde(rename = "scaffold-only")]
    ScaffoldOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityReportSourceReachabilityState {
    #[serde(rename = "reachable")]
    Reachable,
    #[serde(rename = "unreachable")]
    Unreachable,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "error")]
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivitySavedReportState {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "saved")]
    Saved,
    #[serde(rename = "storage-unavailable")]
    StorageUnavailable,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "scaffold-only")]
    ScaffoldOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityReportCustodyLabel {
    #[serde(rename = "child-device-local-summary")]
    ChildDeviceLocalSummary,
    #[serde(rename = "parent-device-local-report-json")]
    ParentDeviceLocalReportJson,
    #[serde(rename = "parent-device-local-history")]
    ParentDeviceLocalHistory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityReportSourceLabel {
    #[serde(rename = "activity-query-store-summary")]
    ActivityQueryStoreSummary,
    #[serde(rename = "family-fanout-source-state")]
    FamilyFanoutSourceState,
    #[serde(rename = "saved-report-json")]
    SavedReportJson,
    #[serde(rename = "saved-report-history")]
    SavedReportHistory,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySurfaceScope {
    pub scope_kind: ActivitySurfaceScopeKind,
    pub family_id: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySurfaceRequest {
    pub schema_version: u16,
    pub scope: ActivitySurfaceScope,
    pub requested_at: String,
    pub range_start: String,
    pub range_end: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReportRequest {
    pub schema_version: u16,
    pub frequency: ActivityReportFrequency,
    pub scope: ActivitySurfaceScope,
    pub requested_at: String,
    pub range_start: String,
    pub range_end: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReportSourceState {
    pub device_id: String,
    pub reachability_state: ActivityReportSourceReachabilityState,
    pub state: ActivityReadModelState,
    pub reason: Option<String>,
    pub last_updated_at: Option<String>,
    #[serde(default = "default_source_state_custody_label")]
    pub custody_label: ActivityReportCustodyLabel,
    #[serde(default = "default_source_state_source_label")]
    pub source_label: ActivityReportSourceLabel,
    #[serde(default)]
    pub raw_child_evidence_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReportSection {
    pub section_kind: ActivityReportSectionKind,
    pub title: String,
    pub state: ActivityReadModelState,
    pub summary: String,
    pub item_count: u64,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySavedReportMetadata {
    pub report_id: String,
    pub file_name: String,
    pub saved_state: ActivitySavedReportState,
    pub saved_at: Option<String>,
    pub storage_reason: Option<String>,
    #[serde(default = "default_saved_metadata_custody_label")]
    pub custody_label: ActivityReportCustodyLabel,
    #[serde(default = "default_saved_metadata_source_label")]
    pub source_label: ActivityReportSourceLabel,
    #[serde(default)]
    pub raw_child_evidence_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReportSourceStateSummary {
    pub total_sources: u64,
    pub ready_sources: u64,
    pub offline_sources: u64,
    pub stale_sources: u64,
    pub unavailable_sources: u64,
    pub unreachable_sources: u64,
    pub error_sources: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReportDocument {
    pub schema_version: u16,
    pub report_id: String,
    pub frequency: ActivityReportFrequency,
    pub scope: ActivitySurfaceScope,
    pub requested_at: String,
    pub range_start: String,
    pub range_end: String,
    pub generated_at: String,
    pub saved_metadata: Option<ActivitySavedReportMetadata>,
    pub source_states: Vec<ActivityReportSourceState>,
    pub sections: Vec<ActivityReportSection>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistoricalReportListItem {
    pub schema_version: u16,
    pub report_id: String,
    pub file_name: String,
    pub report_date: String,
    pub range_start: String,
    pub range_end: String,
    pub summary: String,
    pub saved_state: ActivitySavedReportState,
    pub saved_at: Option<String>,
    pub source_state_summary: ActivityReportSourceStateSummary,
    pub parsed_report: ActivityReportDocument,
    #[serde(default = "default_history_item_custody_label")]
    pub custody_label: ActivityReportCustodyLabel,
    #[serde(default = "default_history_item_source_label")]
    pub source_label: ActivityReportSourceLabel,
    #[serde(default)]
    pub raw_child_evidence_included: bool,
}

fn default_source_state_custody_label() -> ActivityReportCustodyLabel {
    ActivityReportCustodyLabel::ChildDeviceLocalSummary
}

fn default_source_state_source_label() -> ActivityReportSourceLabel {
    ActivityReportSourceLabel::ActivityQueryStoreSummary
}

fn default_saved_metadata_custody_label() -> ActivityReportCustodyLabel {
    ActivityReportCustodyLabel::ParentDeviceLocalReportJson
}

fn default_saved_metadata_source_label() -> ActivityReportSourceLabel {
    ActivityReportSourceLabel::SavedReportJson
}

fn default_history_item_custody_label() -> ActivityReportCustodyLabel {
    ActivityReportCustodyLabel::ParentDeviceLocalHistory
}

fn default_history_item_source_label() -> ActivityReportSourceLabel {
    ActivityReportSourceLabel::SavedReportHistory
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistoricalReportList {
    pub schema_version: u16,
    pub request: ActivitySurfaceRequest,
    pub state: ActivityReadModelState,
    pub storage_state: ActivitySavedReportState,
    pub storage_reason: Option<String>,
    pub reports: Vec<ActivityHistoricalReportListItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityTabReadModel<Row> {
    pub schema_version: u16,
    pub request: ActivitySurfaceRequest,
    pub state: ActivityReadModelState,
    pub generated_at: String,
    pub summary: String,
    pub rows: Vec<Row>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityScreenReadModelRow {
    pub row_id: String,
    pub label: String,
    pub device_id: String,
    pub state: ActivityReadModelState,
    pub total_ms: u64,
    pub foreground_ms: u64,
    pub background_ms: u64,
    pub capture_reason: String,
    pub capture_scope: String,
    pub capability_status: String,
    pub queue_job_id: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub provider_kind: String,
    pub prompt_or_template_version: String,
    pub primary_category: Option<String>,
    pub confidence: f64,
    pub image_deletion_state: String,
    pub raw_image_retained: bool,
    pub policy_eligible: bool,
    pub image_digest: String,
    pub custody_state: String,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub policy_decision_ref: Option<String>,
    pub policy_action: Option<String>,
    #[serde(default)]
    pub policy_reason_codes: Vec<String>,
    #[serde(default)]
    pub parent_rule_refs: Vec<String>,
    #[serde(default)]
    pub local_model_runtime_refs: Vec<String>,
    #[serde(default)]
    pub parent_explanation_refs: Vec<String>,
    #[serde(default)]
    pub explanation_reasons: Vec<String>,
    #[serde(default)]
    pub deletion_reasons: Vec<String>,
    #[serde(default)]
    pub ocr_text_snippets: Vec<String>,
    #[serde(default)]
    pub redaction_notes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAppUseReadModelRow {
    pub row_id: String,
    pub app_name: String,
    pub device_id: String,
    pub state: ActivityReadModelState,
    pub product_kind: String,
    pub classification_state: String,
    pub inventory_state: String,
    pub runtime_state: String,
    pub foreground_state: String,
    pub capability_status: String,
    pub last_observed_at: Option<String>,
    pub total_ms: u64,
    pub launch_count: u64,
    pub inventory_row_count: u64,
    pub running_row_count: u64,
    pub foreground_row_count: u64,
    pub daily_rollup_count: u64,
    pub evidence_claim_row_count: u64,
    pub identity_row_count: u64,
    pub approval_authority_row_count: u64,
    pub approval_action_result_row_count: u64,
    pub platform_authority_matrix_count: u64,
    pub platform_authority_row_count: u64,
    pub ai_classifier_result_row_count: u64,
    pub source_status_rows: Vec<ActivityAppGameSourceStatusRow>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityBrowserReadModelRow {
    pub row_id: String,
    pub domain_label: String,
    pub device_id: String,
    pub state: ActivityReadModelState,
    pub visit_count: u64,
    pub total_ms: u64,
    pub evidence_digest: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityGamesReadModelRow {
    pub row_id: String,
    pub display_name: String,
    pub device_id: String,
    pub state: ActivityReadModelState,
    pub product_kind: String,
    pub classification_state: String,
    pub inventory_state: String,
    pub runtime_state: String,
    pub foreground_state: String,
    pub capability_status: String,
    pub last_observed_at: Option<String>,
    pub total_ms: u64,
    pub session_count: u64,
    pub launcher_row_count: u64,
    pub running_row_count: u64,
    pub foreground_row_count: u64,
    pub daily_rollup_count: u64,
    pub evidence_claim_row_count: u64,
    pub identity_row_count: u64,
    pub approval_authority_row_count: u64,
    pub approval_action_result_row_count: u64,
    pub platform_authority_matrix_count: u64,
    pub platform_authority_row_count: u64,
    pub ai_classifier_result_row_count: u64,
    pub source_status_rows: Vec<ActivityAppGameSourceStatusRow>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkReadModelRow {
    pub row_id: String,
    pub destination_label: String,
    pub device_id: String,
    pub state: ActivityReadModelState,
    pub connection_count: u64,
    pub total_bytes: u64,
    pub evidence_digest: Option<String>,
}

pub type ActivityScreenReadModel = ActivityTabReadModel<ActivityScreenReadModelRow>;
pub type ActivityAppUseReadModel = ActivityTabReadModel<ActivityAppUseReadModelRow>;
pub type ActivityBrowserReadModel = ActivityTabReadModel<ActivityBrowserReadModelRow>;
pub type ActivityGamesReadModel = ActivityTabReadModel<ActivityGamesReadModelRow>;
pub type ActivityNetworkReadModel = ActivityTabReadModel<ActivityNetworkReadModelRow>;
