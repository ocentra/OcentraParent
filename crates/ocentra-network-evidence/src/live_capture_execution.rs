use crate::live_capture::{NetworkLiveCaptureProof, NetworkLiveCaptureProofState};

mod types;
mod validation;

pub type NetworkLiveCaptureExecutionError = types::NetworkLiveCaptureExecutionError;
pub type NetworkLiveCaptureExecutionInput = types::NetworkLiveCaptureExecutionInput;
pub type NetworkLiveCaptureExecutionProof = types::NetworkLiveCaptureExecutionProof;
pub type NetworkLiveCaptureExecutionRequiredArtifact =
    types::NetworkLiveCaptureExecutionRequiredArtifact;
pub type NetworkLiveCaptureExecutionSource = types::NetworkLiveCaptureExecutionSource;
pub type NetworkLiveCaptureExecutionState = types::NetworkLiveCaptureExecutionState;

use validation::{missing_artifacts, validate_input};

pub fn prove_network_live_capture_execution(
    input: NetworkLiveCaptureExecutionInput,
) -> Result<NetworkLiveCaptureExecutionProof, NetworkLiveCaptureExecutionError> {
    validate_input(&input)?;

    let missing_artifacts = missing_artifacts(&input);
    let execution_state = execution_state(&input.live_capture_proof, &missing_artifacts);
    let live_capture_executed =
        execution_state == NetworkLiveCaptureExecutionState::BoundedExecuted;

    Ok(NetworkLiveCaptureExecutionProof {
        execution_ref: input.execution_ref,
        capture_proof_ref: input.live_capture_proof.capture_proof_ref,
        platform: input.live_capture_proof.platform,
        source: input.source,
        execution_state,
        missing_artifacts,
        driver_invocation_ref: input.driver_invocation_ref,
        interface_observation_ref: input.interface_observation_ref,
        permission_ref: input.permission_ref,
        bounded_window_ref: input.bounded_window_ref,
        clean_stop_ref: input.clean_stop_ref,
        custody_ref: input.custody_ref,
        retention_delete_export_ref: input.retention_delete_export_ref,
        metadata_only_sanitization_ref: input.metadata_only_sanitization_ref,
        private_traffic_exclusion_ref: input.private_traffic_exclusion_ref,
        driver_invoked: live_capture_executed && input.driver_invoked,
        live_capture_executed,
        metadata_snapshot_executed: input.metadata_snapshot_executed,
        captured_packet_count: if live_capture_executed {
            input.captured_packet_count
        } else {
            0
        },
        raw_artifact_created: false,
        raw_pcap_without_custody_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        policy_authority: false,
        adapter_authority: false,
        host_filtering_claimed: false,
        enforcement_commands_published: 0,
        netstat_metadata_substituted_for_live_capture: false,
    })
}

fn execution_state(
    live_capture_proof: &NetworkLiveCaptureProof,
    missing_artifacts: &[NetworkLiveCaptureExecutionRequiredArtifact],
) -> NetworkLiveCaptureExecutionState {
    match live_capture_proof.proof_state {
        NetworkLiveCaptureProofState::Unavailable => NetworkLiveCaptureExecutionState::Unavailable,
        NetworkLiveCaptureProofState::Degraded => NetworkLiveCaptureExecutionState::Degraded,
        NetworkLiveCaptureProofState::ProofReady if missing_artifacts.is_empty() => {
            NetworkLiveCaptureExecutionState::BoundedExecuted
        }
        NetworkLiveCaptureProofState::ProofReady | NetworkLiveCaptureProofState::ManualRequired => {
            NetworkLiveCaptureExecutionState::ManualRequired
        }
    }
}
