use serde::{Deserialize, Serialize};

pub(crate) const SCREEN_FAMILY_AI_HUB_ROUTE_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubRequestedTask {
    GuidedVisionClassification,
    GuidedMultimodalClassification,
    OcrTextFallback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubCapabilityState {
    Available,
    DisabledByParent,
    HubUnavailable,
    LanProofMissing,
    ResourceExhausted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubDegradedState {
    ChildLocalAlreadySelected,
    ParentDisabled,
    HubUnavailable,
    LanProofMissing,
    ResourceExhausted,
    UnsupportedTask,
    CustodyUnsafe,
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubExecutionState {
    Selected,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenFamilyAiHubTransferMode {
    SummaryOnly,
    RedactedCrop,
    NoTransfer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenChildLocalAnalysisAttemptState {
    Selected,
    Degraded,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenEvidenceCustodyState {
    LiveLocalChildAgent,
    LiveLanChildAgent,
    ChildDeviceTempQueue,
    ChildDeviceJournal,
    ChildDeviceQueryStore,
    ParentDeviceCache,
    ParentOwnedExport,
    OcentraHostedNonActivity,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenLocalModelProviderKind {
    DeterministicRules,
    LocalOcr,
    LocalVision,
    LocalMultimodal,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenFamilyAiHubCapability {
    pub schema_version: u16,
    pub hub_id: String,
    pub checked_at: String,
    pub capability_state: ScreenFamilyAiHubCapabilityState,
    pub supported_tasks: Vec<ScreenFamilyAiHubRequestedTask>,
    pub model_runtime_ref: Option<String>,
    pub household_route_ref: Option<String>,
    pub custody_state: ScreenEvidenceCustodyState,
    pub no_retention: bool,
    pub local_household_only: bool,
    pub parent_approval_required: bool,
    pub ocentra_hosted_processing_allowed: bool,
    pub raw_image_retention_allowed: bool,
    pub degraded_states: Vec<ScreenFamilyAiHubDegradedState>,
    pub unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenChildLocalAnalysisAttempt {
    pub attempted: bool,
    pub provider_kind: ScreenLocalModelProviderKind,
    pub execution_state: ScreenChildLocalAnalysisAttemptState,
    pub model_runtime_ref: Option<String>,
    pub degraded_states: Vec<ScreenFamilyAiHubDegradedState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenFamilyAiHubRouteRequest {
    pub route_id: String,
    pub queue_job_id: String,
    pub routed_at: String,
    pub requested_task: ScreenFamilyAiHubRequestedTask,
    pub source_child_local_attempt: ScreenChildLocalAnalysisAttempt,
    pub capability: ScreenFamilyAiHubCapability,
    pub parent_approved_family_hub: bool,
    pub transfer_mode: ScreenFamilyAiHubTransferMode,
    pub source_custody_state: ScreenEvidenceCustodyState,
    pub audit_evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenFamilyAiHubRoute {
    pub schema_version: u16,
    pub route_id: String,
    pub queue_job_id: String,
    pub routed_at: String,
    pub requested_task: ScreenFamilyAiHubRequestedTask,
    pub source_child_local_attempt: ScreenChildLocalAnalysisAttempt,
    pub capability: ScreenFamilyAiHubCapability,
    pub execution_state: ScreenFamilyAiHubExecutionState,
    pub selected_runtime_ref: Option<String>,
    pub transfer_mode: ScreenFamilyAiHubTransferMode,
    pub source_custody_state: ScreenEvidenceCustodyState,
    pub destination_custody_state: ScreenEvidenceCustodyState,
    pub degraded_states: Vec<ScreenFamilyAiHubDegradedState>,
    pub audit_evidence_ids: Vec<String>,
    pub parent_approved_family_hub: bool,
    pub local_provider_attempted: bool,
    pub child_safety_local_fallback_preserved: bool,
    pub summary_first: bool,
    pub redacted_or_cropped_input_required: bool,
    pub raw_full_screenshot_transfer_allowed: bool,
    pub raw_image_retention_allowed: bool,
    pub remote_provider_selected: bool,
    pub remote_api_fallback_allowed: bool,
    pub ocentra_hosted_processing_allowed: bool,
    pub remote_default_for_blocking: bool,
}

pub fn screen_family_ai_hub_capability_is_consistent(value: &ScreenFamilyAiHubCapability) -> bool {
    crate::screen_family_ai_hub_routing_logic::screen_family_ai_hub_capability_is_consistent(value)
}

pub fn screen_child_local_attempt_is_consistent(value: &ScreenChildLocalAnalysisAttempt) -> bool {
    crate::screen_family_ai_hub_routing_logic::screen_child_local_attempt_is_consistent(value)
}

pub fn screen_family_ai_hub_route_is_consistent(value: &ScreenFamilyAiHubRoute) -> bool {
    crate::screen_family_ai_hub_routing_logic::screen_family_ai_hub_route_is_consistent(value)
}

pub fn plan_screen_family_ai_hub_route(
    request: &ScreenFamilyAiHubRouteRequest,
) -> ScreenFamilyAiHubRoute {
    crate::screen_family_ai_hub_routing_logic::plan_screen_family_ai_hub_route(request)
}

const SCREEN_FAMILY_AI_HUB_ROUTING_GENERATED_TYPESCRIPT: &str = r#"/* generated from crates/screen-ai-core/src/screen_family_ai_hub_routing.rs */

export function planScreenFamilyAiHubRouteStub() {
  return 'planScreenFamilyAiHubRouteGenerated';
}
"#;

pub fn screen_family_ai_hub_routing_generated_typescript() -> String {
    SCREEN_FAMILY_AI_HUB_ROUTING_GENERATED_TYPESCRIPT.to_string()
}
