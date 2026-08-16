use ocentra_network_evidence::{
    live_capture::{
        NetworkLiveCapturePlatform, NetworkLiveCaptureProof, NetworkLiveCaptureProofInput,
        NetworkLiveCaptureProofState,
    },
    raw_capture_storage::types::NetworkRawCaptureStorageInput,
};
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Copy)]
struct CaptureProofRef(&'static str);

pub(super) fn proof_ready_input() -> NetworkLiveCaptureProofInput {
    complete_live_capture_input(
        CaptureProofRef(constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF),
        NetworkLiveCapturePlatform::WindowsNpcap,
        true,
        false,
    )
}

pub(super) fn manual_required_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: constants::network_flow::TEST_LIVE_CAPTURE_MANUAL_PROOF_REF.to_string(),
        platform: NetworkLiveCapturePlatform::WindowsNpcap,
        interface_ref: None,
        driver_proof_ref: None,
        permission_proof_ref: None,
        bounded_capture_ref: None,
        clean_stop_ref: None,
        quota_rotation_ref: None,
        retention_delete_export_ref: None,
        custody_ref: None,
        private_traffic_exclusion_ref: None,
        platform_available: true,
        driver_available: false,
        permission_granted: false,
        interface_enumerated: false,
        bounded_capture_succeeded: false,
        clean_stop_succeeded: false,
        adapter_degraded: false,
        ..no_claim_live_capture_input()
    }
}

pub(super) fn unavailable_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: constants::network_flow::TEST_LIVE_CAPTURE_LINUX_PROOF_REF.to_string(),
        platform: NetworkLiveCapturePlatform::LinuxLibpcap,
        platform_available: false,
        ..manual_required_input()
    }
}

pub(super) fn degraded_input() -> NetworkLiveCaptureProofInput {
    complete_live_capture_input(
        CaptureProofRef(constants::network_flow::TEST_LIVE_CAPTURE_MACOS_PROOF_REF),
        NetworkLiveCapturePlatform::MacosBpfLibpcap,
        true,
        true,
    )
}

fn complete_live_capture_input(
    capture_proof_ref: CaptureProofRef,
    platform: NetworkLiveCapturePlatform,
    platform_available: bool,
    adapter_degraded: bool,
) -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: capture_proof_ref.0.to_string(),
        platform,
        interface_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_INTERFACE_REF.to_string()),
        driver_proof_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_DRIVER_REF.to_string()),
        permission_proof_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_PERMISSION_REF.to_string(),
        ),
        bounded_capture_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_BOUNDED_REF.to_string(),
        ),
        clean_stop_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_CLEAN_STOP_REF.to_string()),
        quota_rotation_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_QUOTA_REF.to_string()),
        retention_delete_export_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_RETENTION_REF.to_string(),
        ),
        custody_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_CUSTODY_REF.to_string()),
        private_traffic_exclusion_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF.to_string(),
        ),
        platform_available,
        driver_available: true,
        permission_granted: true,
        interface_enumerated: true,
        bounded_capture_succeeded: true,
        clean_stop_succeeded: true,
        adapter_degraded,
        ..no_claim_live_capture_input()
    }
}

fn no_claim_live_capture_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: String::new(),
        platform: NetworkLiveCapturePlatform::WindowsNpcap,
        interface_ref: None,
        driver_proof_ref: None,
        permission_proof_ref: None,
        bounded_capture_ref: None,
        clean_stop_ref: None,
        quota_rotation_ref: None,
        retention_delete_export_ref: None,
        custody_ref: None,
        private_traffic_exclusion_ref: None,
        platform_available: false,
        driver_available: false,
        permission_granted: false,
        interface_enumerated: false,
        bounded_capture_succeeded: false,
        clean_stop_succeeded: false,
        adapter_degraded: false,
        live_capture_execution_claimed: false,
        unbounded_capture_claimed: false,
        raw_pcap_without_custody_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}

pub(super) fn storage_input(proof: &NetworkLiveCaptureProof) -> NetworkRawCaptureStorageInput {
    let ready = proof.proof_state == NetworkLiveCaptureProofState::ProofReady;
    NetworkRawCaptureStorageInput {
        storage_proof_ref: constants::network_flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF
            .to_string(),
        live_capture_proof: proof.clone(),
        raw_artifact_manifest_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_MANIFEST_REF.to_string()),
        storage_location_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_STORAGE_LOCATION_REF.to_string()),
        encryption_at_rest_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_ENCRYPTION_REF.to_string()),
        quota_rotation_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_QUOTA_REF.to_string()),
        retention_policy_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_RETENTION_REF.to_string()),
        delete_export_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_DELETE_EXPORT_REF.to_string()),
        custody_chain_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_CUSTODY_CHAIN_REF.to_string()),
        private_traffic_exclusion_ref: ready.then_some(
            constants::network_flow::TEST_RAW_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF.to_string(),
        ),
        raw_artifact_manifest_available: ready,
        storage_location_available: ready,
        encryption_at_rest_verified: ready,
        quota_rotation_verified: ready,
        retention_policy_verified: ready,
        delete_export_verified: ready,
        custody_chain_verified: ready,
        private_traffic_exclusion_verified: ready,
        live_capture_execution_claimed: false,
        remote_upload_claimed: false,
        raw_pcap_without_custody_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}
