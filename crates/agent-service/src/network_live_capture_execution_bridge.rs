use ocentra_network_evidence::{
    live_capture::{NetworkLiveCaptureProof, NetworkLiveCaptureProofState},
    live_capture_execution::{NetworkLiveCaptureExecutionInput, NetworkLiveCaptureExecutionState},
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureExecutionStatusState;

use self::execution_source::execution_source;
use self::mapping::execution_ref_index;

#[path = "network_live_capture_execution_bridge/execution_source.rs"]
mod execution_source;
#[path = "network_live_capture_execution_bridge/mapping.rs"]
mod mapping;

const LIVE_CAPTURE_EXECUTION_REFS: [&str; 4] = [
    constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_EXECUTION_REF,
    constants::network_flow::TEST_LIVE_CAPTURE_LINUX_EXECUTION_REF,
    constants::network_flow::TEST_LIVE_CAPTURE_MACOS_EXECUTION_REF,
    constants::network_flow::TEST_LIVE_CAPTURE_MANUAL_EXECUTION_REF,
];

pub(crate) fn execution_input(proof: &NetworkLiveCaptureProof) -> NetworkLiveCaptureExecutionInput {
    let bounded_execution = proof.proof_state == NetworkLiveCaptureProofState::ProofReady;
    NetworkLiveCaptureExecutionInput {
        execution_ref: LIVE_CAPTURE_EXECUTION_REFS[execution_ref_index(proof)].to_string(),
        live_capture_proof: proof.clone(),
        source: execution_source(proof.platform),
        driver_invocation_ref: bounded_execution.then_some(
            constants::network_flow::TEST_LIVE_CAPTURE_DRIVER_INVOCATION_REF.to_string(),
        ),
        interface_observation_ref: bounded_execution.then_some(
            constants::network_flow::TEST_LIVE_CAPTURE_INTERFACE_OBSERVATION_REF.to_string(),
        ),
        permission_ref: bounded_execution.then_some(
            constants::network_flow::TEST_LIVE_CAPTURE_EXECUTION_PERMISSION_REF.to_string(),
        ),
        bounded_window_ref: bounded_execution
            .then_some(constants::network_flow::TEST_LIVE_CAPTURE_BOUNDED_WINDOW_REF.to_string()),
        clean_stop_ref: bounded_execution.then_some(
            constants::network_flow::TEST_LIVE_CAPTURE_EXECUTION_CLEAN_STOP_REF.to_string(),
        ),
        custody_ref: bounded_execution.then_some(
            constants::network_flow::TEST_LIVE_CAPTURE_EXECUTION_CUSTODY_REF.to_string(),
        ),
        retention_delete_export_ref: bounded_execution.then_some(
            constants::network_flow::TEST_LIVE_CAPTURE_EXECUTION_RETENTION_REF.to_string(),
        ),
        metadata_only_sanitization_ref: bounded_execution.then_some(
            constants::network_flow::TEST_LIVE_CAPTURE_METADATA_SANITIZATION_REF.to_string(),
        ),
        private_traffic_exclusion_ref: bounded_execution.then_some(
            constants::network_flow::TEST_LIVE_CAPTURE_EXECUTION_PRIVATE_TRAFFIC_EXCLUSION_REF
                .to_string(),
        ),
        driver_invoked: bounded_execution,
        live_capture_executed: bounded_execution,
        metadata_snapshot_executed: false,
        captured_packet_count: if bounded_execution { 3 } else { 0 },
        raw_artifact_created: false,
        netstat_metadata_substitution_claimed: false,
        unbounded_capture_claimed: false,
        raw_pcap_without_custody_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        host_filtering_claimed: false,
        enforcement_command_claimed: false,
    }
}

pub(crate) fn protocol_execution_state(
    state: NetworkLiveCaptureExecutionState,
) -> NetworkLiveCaptureExecutionStatusState {
    mapping::protocol_execution_state(state)
}
