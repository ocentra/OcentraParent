#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserRuntimeActionIntentChildStatusReadModelState {
    ChildAcceptedNotExecuted,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserRuntimeActionIntentChildStatusRecord {
    pub policy_preview_id: String,
    pub action_intent_id: String,
    pub durable_result_ref: String,
    pub durable_read_model_ref: String,
    pub outbox_ref: String,
    pub handoff_ref: String,
    pub child_command_ref: String,
    pub child_command_received_event_ref: String,
    pub child_command_accepted_event_ref: String,
    pub parent_read_model_ref: String,
    pub parent_read_model_projected_event_ref: String,
    pub state: BrowserRuntimeActionIntentChildStatusReadModelState,
}

#[derive(Clone, Debug)]
pub struct BrowserRuntimeActionIntentChildStatusReport {
    pub handoff_candidate_count: usize,
    pub child_command_received_count: usize,
    pub child_command_accepted_count: usize,
    pub parent_read_model_row_count: usize,
    pub child_accepted_not_executed_count: usize,
    pub handoff_refs_match_durable_record: bool,
    pub child_command_matches_handoff: bool,
    pub parent_read_model_visible: bool,
    pub dispatch_attempt_count: usize,
    pub adapter_execution_count: usize,
    pub browser_mutation_count: usize,
    pub child_intervention_execution_count: usize,
    pub final_policy_execution_count: usize,
    pub enforcement_execution_count: usize,
    pub public_stream_field_registry_ready: bool,
    pub external_transport_implemented: bool,
    pub adapter_dispatch_claimed: bool,
    pub browser_mutation_claimed: bool,
    pub child_intervention_execution_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub enforcement_claimed: bool,
    pub rows: Vec<BrowserRuntimeActionIntentChildStatusRecord>,
}
