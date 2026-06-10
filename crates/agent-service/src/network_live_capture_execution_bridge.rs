use ocentra_network_evidence::{
    NetworkLiveCaptureExecutionInput, NetworkLiveCaptureExecutionSource,
    NetworkLiveCaptureExecutionState, NetworkLiveCapturePlatform, NetworkLiveCaptureProof,
    NetworkLiveCaptureProofState,
};
use ocentra_parent_agent_protocol::{constants, NetworkLiveCaptureExecutionStatusState};

pub(crate) fn execution_input(proof: &NetworkLiveCaptureProof) -> NetworkLiveCaptureExecutionInput {
    let bounded_execution = proof.proof_state == NetworkLiveCaptureProofState::ProofReady;
    NetworkLiveCaptureExecutionInput {
        execution_ref: execution_ref(proof).to_string(),
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
    match state {
        NetworkLiveCaptureExecutionState::BoundedExecuted => {
            NetworkLiveCaptureExecutionStatusState::BoundedExecuted
        }
        NetworkLiveCaptureExecutionState::ManualRequired => {
            NetworkLiveCaptureExecutionStatusState::ManualRequired
        }
        NetworkLiveCaptureExecutionState::Unavailable => {
            NetworkLiveCaptureExecutionStatusState::Unavailable
        }
        NetworkLiveCaptureExecutionState::Degraded => {
            NetworkLiveCaptureExecutionStatusState::Degraded
        }
    }
}

fn execution_ref(proof: &NetworkLiveCaptureProof) -> &'static str {
    match proof.capture_proof_ref.as_str() {
        constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF => {
            constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_EXECUTION_REF
        }
        constants::network_flow::TEST_LIVE_CAPTURE_LINUX_PROOF_REF => {
            constants::network_flow::TEST_LIVE_CAPTURE_LINUX_EXECUTION_REF
        }
        constants::network_flow::TEST_LIVE_CAPTURE_MACOS_PROOF_REF => {
            constants::network_flow::TEST_LIVE_CAPTURE_MACOS_EXECUTION_REF
        }
        _ => constants::network_flow::TEST_LIVE_CAPTURE_MANUAL_EXECUTION_REF,
    }
}

fn execution_source(platform: NetworkLiveCapturePlatform) -> NetworkLiveCaptureExecutionSource {
    match platform {
        NetworkLiveCapturePlatform::WindowsNpcap => {
            NetworkLiveCaptureExecutionSource::WindowsNpcapDriver
        }
        NetworkLiveCapturePlatform::LinuxLibpcap => {
            NetworkLiveCaptureExecutionSource::LinuxLibpcapDriver
        }
        NetworkLiveCapturePlatform::MacosBpfLibpcap => {
            NetworkLiveCaptureExecutionSource::MacosBpfLibpcapDriver
        }
    }
}
