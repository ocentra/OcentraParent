use serde::{Deserialize, Serialize};

use crate::{
    BrowserBoundaryState, BrowserChannel, BrowserCustodyLabel, BrowserExactUrlClaimState,
    BrowserFamily, BrowserInterventionAction, BrowserInterventionCapabilityState,
    BrowserInterventionDecisionSource, BrowserInterventionDeliveryState,
    BrowserInterventionMechanism, BrowserInterventionOutcome, BrowserInterventionTargetType,
    BrowserQueryVisibilityLabel, BrowserUnmanagedDetectionState, BrowserUnmanagedEnforcementState,
    BrowserUnmanagedFallbackActionState,
};

pub const BROWSER_INTERVENTION_SCHEMA_VERSION: u16 = crate::BROWSER_INTERVENTION_SCHEMA_VERSION;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserInterventionRow {
    pub schema_version: u16,
    pub browser_intervention_id: String,
    pub observed_at: String,
    pub source_id: String,
    pub device_id: String,
    pub browser_family: Option<BrowserFamily>,
    pub browser_channel: Option<BrowserChannel>,
    pub managed_browser_session_id: Option<String>,
    pub profile_id: Option<String>,
    pub process_id: Option<u32>,
    pub intervention_action_id: Option<String>,
    pub intervention_audit_id: Option<String>,
    #[serde(default)]
    pub evidence_reference_ids: Vec<String>,
    pub policy_decision_id: Option<String>,
    pub decision_source: BrowserInterventionDecisionSource,
    pub intervention_action: BrowserInterventionAction,
    pub intervention_target_type: BrowserInterventionTargetType,
    pub intervention_target_value: String,
    pub requested_url: Option<String>,
    pub observed_url: Option<String>,
    pub intervention_mechanism: BrowserInterventionMechanism,
    pub intervention_outcome: BrowserInterventionOutcome,
    pub browser_boundary_state: BrowserBoundaryState,
    pub exact_url_claim_state: BrowserExactUrlClaimState,
    pub unmanaged_detection_state: BrowserUnmanagedDetectionState,
    #[serde(default)]
    pub unmanaged_fallback_action: BrowserUnmanagedFallbackActionState,
    #[serde(default)]
    pub child_delivery_state: BrowserInterventionDeliveryState,
    pub reason: Option<String>,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserInterventionReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub limit: u64,
    pub returned: u64,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub managed_session_intervention_capability: BrowserInterventionCapabilityState,
    pub unmanaged_browser_enforcement: BrowserUnmanagedEnforcementState,
    #[serde(default)]
    pub unmanaged_fallback_action: BrowserUnmanagedFallbackActionState,
    pub rows: Vec<BrowserInterventionRow>,
}
