use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;

use crate::screen_event_runtime_input::ScreenRuntimeInput;

pub(crate) fn action_ref(phase: ScreenRuntimePhase, input: &ScreenRuntimeInput) -> Option<String> {
    match phase {
        ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => Some(input.action_ref.clone()),
        _ => None,
    }
}

pub(crate) fn deletion_proof_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    match phase {
        ScreenRuntimePhase::DeletionCommitted | ScreenRuntimePhase::PortalReadModelUpdated => {
            Some(input.deletion_proof_ref.clone())
        }
        _ => None,
    }
}

pub(crate) fn portal_read_model_ref(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
) -> Option<String> {
    match phase {
        ScreenRuntimePhase::PortalReadModelUpdated => Some(input.portal_read_model_ref.clone()),
        _ => None,
    }
}
