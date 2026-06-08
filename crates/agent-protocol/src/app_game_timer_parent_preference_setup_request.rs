use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameTimerParentPreferenceSetupRequest {
    pub request_id: String,
    pub requested_at: String,
    pub parent_surface_intent_reference_id: String,
    pub parent_preference_setup_reference_id: String,
    pub request_reference_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameTimerParentPreferenceSetupRequestResult {
    pub schema_version: String,
    pub request_id: String,
    pub requested_at: String,
    pub accepted_at: String,
    pub request_status: String,
    pub parent_surface_intent_reference_id: String,
    pub parent_preference_setup_reference_id: String,
    pub request_reference_ids: Vec<String>,
    pub action_result_reference_id: String,
    pub action_result_reference_ids: Vec<String>,
    pub command_boundary_claimed: bool,
    pub action_result_handoff_claimed: bool,
    pub action_result_persistence_claimed: bool,
    pub parent_preference_mutation_claimed: bool,
    pub notification_rule_mutation_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub durable_outbox_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
}
