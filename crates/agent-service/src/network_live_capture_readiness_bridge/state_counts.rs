use ocentra_parent_agent_protocol::network_flow::{
    NetworkLiveCaptureStatus, NetworkLiveCaptureStatusRow,
};

use super::execution_counts::apply_execution_counts;
use super::proof_counts::apply_proof_counts;
use super::storage_counts::apply_storage_counts;

pub(super) fn apply_state_counts(
    status: &mut NetworkLiveCaptureStatus,
    row: &NetworkLiveCaptureStatusRow,
) {
    apply_proof_counts(status, &row.proof_state);
    apply_storage_counts(status, &row.storage_state);
    apply_execution_counts(status, &row.execution_state);
    status.missing_artifact_count += row.missing_artifact_count;
    status.storage_missing_artifact_count += row.storage_missing_artifact_count;
    status.execution_missing_artifact_count += row.execution_missing_artifact_count;
    status.captured_packet_count += row.captured_packet_count;
    status.enforcement_command_event_count += row.enforcement_commands_published;
}
