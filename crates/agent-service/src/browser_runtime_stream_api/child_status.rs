use ocentra_parent_agent_core::browser_event_runtime::BrowserRuntimeActionIntentHandoffResponse;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct BrowserRuntimeActionIntentChildStatusResponse {
    pub(crate) accepted_row_count: usize,
    pub(crate) child_command_ref: Option<String>,
    pub(crate) child_accepted_event_ref: Option<String>,
    pub(crate) parent_read_model_ref: Option<String>,
    pub(crate) dispatch_attempt_count: u8,
    pub(crate) adapter_execution_count: u8,
    pub(crate) child_intervention_execution_count: u8,
    pub(crate) enforcement_execution_count: u8,
}

pub(crate) async fn action_intent_child_status_from_handoff(
    handoff: &BrowserRuntimeActionIntentHandoffResponse,
) -> Option<BrowserRuntimeActionIntentChildStatusResponse> {
    if !handoff_is_child_status_candidate(handoff) {
        return Some(BrowserRuntimeActionIntentChildStatusResponse::default());
    }

    // This handoff contains a policy preview and action-intent identity, but
    // no trusted parent profile, device, or observation context. Keep child
    // delivery unavailable/manual-required until that typed authority arrives;
    // synthesizing parent-child events here would falsely claim acceptance.
    None
}

fn handoff_is_child_status_candidate(handoff: &BrowserRuntimeActionIntentHandoffResponse) -> bool {
    handoff.candidate_count > 0
        && handoff.dispatch_attempt_count == 0
        && handoff.adapter_execution_count == 0
        && handoff.browser_mutation_count == 0
        && handoff.child_intervention_execution_count == 0
        && handoff.enforcement_execution_count == 0
}
