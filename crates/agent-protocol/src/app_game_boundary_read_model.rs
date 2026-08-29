use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const APP_GAME_BOUNDARY_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str =
    "child-device-query-store";
pub const APP_GAME_BOUNDARY_READ_MODEL_STATUS_NO_ROWS: &str = "no-boundary-rows";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameHealthStatus {
    Healthy,
    Degraded,
    Unavailable,
    ManualRequired,
    NotClaimed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppGamePerformanceHealthReadModel {
    pub status: AppGameHealthStatus,
    pub limit: u64,
    pub returned: u64,
    pub inventory_returned: u64,
    pub running_now_returned: u64,
    pub foreground_now_returned: u64,
    pub launcher_returned: u64,
    pub daily_rollup_returned: u64,
    pub custody_label: String,
    pub replay_state: String,
}
pub const APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM: &str = "evidenceClaim";
pub const APP_GAME_BOUNDARY_KIND_IDENTITY: &str = "identity";
pub const APP_GAME_BOUNDARY_KIND_APPROVAL_AUTHORITY: &str = "approvalAuthority";
pub const APP_GAME_BOUNDARY_KIND_APPROVAL_ACTION_RESULT: &str = "approvalActionResult";
pub const APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_MATRIX: &str = "platformAuthorityMatrix";
pub const APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_ROW: &str = "platformAuthorityRow";
pub const APP_GAME_BOUNDARY_KIND_AI_CLASSIFIER_RESULT: &str = "aiClassifierResult";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameBoundaryReadModelRow {
    pub schema_version: u16,
    pub row_id: String,
    pub boundary_kind: String,
    pub row_count: u64,
    pub evidence_reference_ids: Vec<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameBoundaryReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_label: String,
    pub capability_status: String,
    pub performance_health: AppGamePerformanceHealthReadModel,
    pub returned: u64,
    pub evidence_claim_row_count: u64,
    pub identity_row_count: u64,
    pub approval_authority_row_count: u64,
    pub approval_action_result_row_count: u64,
    pub platform_authority_matrix_count: u64,
    pub platform_authority_row_count: u64,
    pub ai_classifier_result_row_count: u64,
    pub rows: Vec<AppGameBoundaryReadModelRow>,
}
