use ocentra_parent_agent_protocol::network_flow::{
    NetworkLiveCaptureExecutionStatusState, NetworkLiveCaptureStatus,
};

pub(super) fn apply_execution_counts(
    status: &mut NetworkLiveCaptureStatus,
    execution_state: &NetworkLiveCaptureExecutionStatusState,
) {
    match execution_state {
        NetworkLiveCaptureExecutionStatusState::BoundedExecuted => {
            status.bounded_executed_count += 1
        }
        NetworkLiveCaptureExecutionStatusState::ManualRequired => {
            status.execution_manual_required_count += 1
        }
        NetworkLiveCaptureExecutionStatusState::Unavailable => {
            status.execution_unavailable_count += 1
        }
        NetworkLiveCaptureExecutionStatusState::Degraded => status.execution_degraded_count += 1,
    }
}
