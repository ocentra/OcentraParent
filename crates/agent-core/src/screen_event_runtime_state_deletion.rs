use ocentra_parent_agent_protocol::screen_evidence::{ScreenDeletionState, ScreenRuntimePhase};

pub(crate) fn deletion_state(phase: ScreenRuntimePhase) -> ScreenDeletionState {
    match phase {
        ScreenRuntimePhase::DeletionCommitted | ScreenRuntimePhase::PortalReadModelUpdated => {
            ScreenDeletionState::Committed
        }
        _ => ScreenDeletionState::Pending,
    }
}
