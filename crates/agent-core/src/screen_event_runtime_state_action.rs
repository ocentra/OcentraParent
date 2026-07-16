use ocentra_parent_agent_protocol::screen_evidence::{ScreenActionState, ScreenRuntimePhase};

pub(crate) fn action_state(phase: ScreenRuntimePhase) -> ScreenActionState {
    match phase {
        ScreenRuntimePhase::ActionDryRunRecorded
        | ScreenRuntimePhase::DeletionCommitted
        | ScreenRuntimePhase::PortalReadModelUpdated => ScreenActionState::DryRunRecorded,
        _ => ScreenActionState::NotReady,
    }
}
