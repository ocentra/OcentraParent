use serde::{Deserialize, Serialize};

pub const BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserAiUxReadModelRow {
    pub schema_version: u16,
    pub row_id: String,
    pub source_evidence_ids: Vec<String>,
    pub child_snapshot_id: String,
    pub child_state: String,
    pub child_primary_text_token: String,
    pub child_delivery_state: String,
    pub adapter_proof_ref: Option<String>,
    pub parent_explanation_id: String,
    pub parent_explanation_state: String,
    pub parent_title_text_token: String,
    pub explanation_audit_refs: Vec<String>,
    pub model_runtime_visible: bool,
    pub policy_rule_visible: bool,
    pub action_visible: bool,
    pub child_experience_visible: bool,
    pub degraded_state_visible: bool,
    pub manual_fallback_visible: bool,
    pub runtime_delivery_claimed: bool,
    pub rendered_ui_claimed: bool,
    pub direct_enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserAiUxReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub latest_event_id: Option<String>,
    pub rows: Vec<BrowserAiUxReadModelRow>,
}
