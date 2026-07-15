use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::live_capture::{
    plan_network_live_capture_proof, NetworkLiveCapturePlatform, NetworkLiveCaptureProof,
    NetworkLiveCaptureProofInput, NetworkLiveCaptureProofState,
};
use ocentra_network_evidence::raw_capture_storage::types::*;
use ocentra_network_evidence::raw_capture_storage::*;

#[test]
fn raw_capture_storage_accepts_custody_ready_artifact_governance_refs() {
    let proof = plan_network_raw_capture_storage(storage_ready_input(proof_ready_live_capture()))
        .expect_value("complete raw capture custody refs should be storage ready");

    assert_eq!(
        proof.storage_state,
        NetworkRawCaptureStorageState::CustodyReady
    );
    assert!(proof.raw_artifact_storage_authorized);
    assert!(proof.missing_artifacts.is_empty());
    assert_eq!(proof.live_capture_proof_ref, "network-live-capture-row13");
    assert_eq!(
        proof.raw_artifact_manifest_ref,
        Some("raw-artifact-manifest-row03a".to_owned())
    );
    assert_eq!(
        proof.encryption_at_rest_ref,
        Some("encrypted-local-custody-row03a".to_owned())
    );
    assert_eq!(
        proof.delete_export_ref,
        Some("delete-export-proof-row03a".to_owned())
    );
    assert!(!proof.live_capture_executed);
    assert!(!proof.remote_upload_enabled);
    assert!(!proof.raw_pcap_without_custody_available);
    assert_eq!(proof.enforcement_commands_published, 0);
}

#[test]
fn raw_capture_storage_records_manual_required_custody_gaps_when_artifact_is_touched() {
    let proof = plan_network_raw_capture_storage(NetworkRawCaptureStorageInput {
        live_capture_proof: manual_required_live_capture(),
        delete_export_ref: None,
        custody_chain_ref: None,
        delete_export_verified: false,
        custody_chain_verified: false,
        ..storage_ready_input(proof_ready_live_capture())
    })
    .expect_value("missing custody refs should become manual-required");

    assert_eq!(
        proof.storage_state,
        NetworkRawCaptureStorageState::ManualRequired
    );
    assert!(!proof.raw_artifact_storage_authorized);
    assert_eq!(
        proof.missing_artifacts,
        vec![
            NetworkRawCaptureStorageRequiredArtifact::LiveCaptureProof,
            NetworkRawCaptureStorageRequiredArtifact::DeleteExport,
            NetworkRawCaptureStorageRequiredArtifact::CustodyChain,
        ]
    );
    assert!(!proof.live_capture_executed);
    assert!(!proof.remote_upload_enabled);
}

#[test]
fn raw_capture_storage_preserves_live_capture_unavailable_and_degraded_states() {
    let unavailable = plan_network_raw_capture_storage(storage_ready_input(
        live_capture_with_state(NetworkLiveCaptureProofState::Unavailable),
    ))
    .expect_value("unavailable capture state should remain visible");
    assert_eq!(
        unavailable.storage_state,
        NetworkRawCaptureStorageState::Unavailable
    );
    assert!(!unavailable.raw_artifact_storage_authorized);

    let degraded = plan_network_raw_capture_storage(storage_ready_input(live_capture_with_state(
        NetworkLiveCaptureProofState::Degraded,
    )))
    .expect_value("degraded capture state should remain visible");
    assert_eq!(
        degraded.storage_state,
        NetworkRawCaptureStorageState::Degraded
    );
    assert!(!degraded.raw_artifact_storage_authorized);
}

#[test]
fn raw_capture_storage_rejects_remote_upload_content_authority_and_execution_claims() {
    assert_eq!(
        plan_network_raw_capture_storage(NetworkRawCaptureStorageInput {
            remote_upload_claimed: true,
            ..storage_ready_input(proof_ready_live_capture())
        }),
        Err(NetworkRawCaptureStorageError::RemoteUploadClaimRejected)
    );
    assert_eq!(
        plan_network_raw_capture_storage(NetworkRawCaptureStorageInput {
            raw_pcap_without_custody_claimed: true,
            ..storage_ready_input(proof_ready_live_capture())
        }),
        Err(NetworkRawCaptureStorageError::RawPcapWithoutCustodyClaimRejected)
    );
    assert_eq!(
        plan_network_raw_capture_storage(NetworkRawCaptureStorageInput {
            live_capture_execution_claimed: true,
            ..storage_ready_input(proof_ready_live_capture())
        }),
        Err(NetworkRawCaptureStorageError::LiveCaptureExecutionClaimRejected)
    );
    assert_eq!(
        plan_network_raw_capture_storage(NetworkRawCaptureStorageInput {
            exact_url_claimed: true,
            ..storage_ready_input(proof_ready_live_capture())
        }),
        Err(NetworkRawCaptureStorageError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_raw_capture_storage(NetworkRawCaptureStorageInput {
            policy_authority_claimed: true,
            ..storage_ready_input(proof_ready_live_capture())
        }),
        Err(NetworkRawCaptureStorageError::PolicyAuthorityClaimRejected)
    );
    assert_eq!(
        plan_network_raw_capture_storage(NetworkRawCaptureStorageInput {
            enforcement_command_claimed: true,
            ..storage_ready_input(proof_ready_live_capture())
        }),
        Err(NetworkRawCaptureStorageError::EnforcementCommandClaimRejected)
    );
}

fn storage_ready_input(
    live_capture_proof: NetworkLiveCaptureProof,
) -> NetworkRawCaptureStorageInput {
    NetworkRawCaptureStorageInput {
        storage_proof_ref: "network-live-capture-storage-row03a".to_owned(),
        live_capture_proof,
        raw_artifact_manifest_ref: Some("raw-artifact-manifest-row03a".to_owned()),
        storage_location_ref: Some("local-encrypted-storage-row03a".to_owned()),
        encryption_at_rest_ref: Some("encrypted-local-custody-row03a".to_owned()),
        quota_rotation_ref: Some("quota-rotation-row03a".to_owned()),
        retention_policy_ref: Some("retention-policy-row03a".to_owned()),
        delete_export_ref: Some("delete-export-proof-row03a".to_owned()),
        custody_chain_ref: Some("custody-chain-row03a".to_owned()),
        private_traffic_exclusion_ref: Some("private-traffic-exclusion-row03a".to_owned()),
        raw_artifact_manifest_available: true,
        storage_location_available: true,
        encryption_at_rest_verified: true,
        quota_rotation_verified: true,
        retention_policy_verified: true,
        delete_export_verified: true,
        custody_chain_verified: true,
        private_traffic_exclusion_verified: true,
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

fn proof_ready_live_capture() -> NetworkLiveCaptureProof {
    plan_network_live_capture_proof(live_capture_input())
        .expect_value("complete live capture proof refs should parse")
}

fn manual_required_live_capture() -> NetworkLiveCaptureProof {
    plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        retention_delete_export_ref: None,
        custody_ref: None,
        ..live_capture_input()
    })
    .expect_value("missing live capture custody refs should parse")
}

fn live_capture_with_state(state: NetworkLiveCaptureProofState) -> NetworkLiveCaptureProof {
    match state {
        NetworkLiveCaptureProofState::Unavailable => {
            plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
                platform_available: false,
                ..live_capture_input()
            })
        }
        NetworkLiveCaptureProofState::Degraded => {
            plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
                adapter_degraded: true,
                ..live_capture_input()
            })
        }
        NetworkLiveCaptureProofState::ProofReady | NetworkLiveCaptureProofState::ManualRequired => {
            plan_network_live_capture_proof(live_capture_input())
        }
    }
    .expect_value("live capture state fixture should parse")
}

fn live_capture_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: "network-live-capture-row13".to_owned(),
        platform: NetworkLiveCapturePlatform::WindowsNpcap,
        interface_ref: Some("npcap-interface-row13".to_owned()),
        driver_proof_ref: Some("npcap-driver-proof-row13".to_owned()),
        permission_proof_ref: Some("npcap-permission-proof-row13".to_owned()),
        bounded_capture_ref: Some("bounded-capture-proof-row13".to_owned()),
        clean_stop_ref: Some("clean-stop-proof-row13".to_owned()),
        quota_rotation_ref: Some("quota-rotation-proof-row13".to_owned()),
        retention_delete_export_ref: Some("live-capture-retention-delete-export-row13".to_owned()),
        custody_ref: Some("live-capture-custody-row13".to_owned()),
        private_traffic_exclusion_ref: Some("private-family-traffic-exclusion-row13".to_owned()),
        platform_available: true,
        driver_available: true,
        permission_granted: true,
        interface_enumerated: true,
        bounded_capture_succeeded: true,
        clean_stop_succeeded: true,
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
