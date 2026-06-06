use crate::{
    materialize_network_live_capture_custody_status, plan_network_live_capture_proof,
    plan_network_raw_capture_storage, NetworkLiveCaptureCustodyStatusError,
    NetworkLiveCaptureCustodyStatusInput, NetworkLiveCaptureCustodyStatusMissingArtifact,
    NetworkLiveCaptureCustodyStatusState, NetworkLiveCapturePlatform, NetworkLiveCaptureProof,
    NetworkLiveCaptureProofInput, NetworkLiveCaptureProofState, NetworkLiveCaptureRequiredArtifact,
    NetworkRawCaptureStorageInput, NetworkRawCaptureStorageRequiredArtifact,
    NetworkRawCaptureStorageState,
};

#[test]
fn live_capture_custody_status_materializes_ready_capture_and_storage_refs_without_execution() {
    let status = materialize_network_live_capture_custody_status(status_ready_input())
        .expect("complete live-capture and raw-storage gates should materialize custody status");

    assert_eq!(
        status.state,
        NetworkLiveCaptureCustodyStatusState::CustodyReady
    );
    assert_eq!(
        status.live_capture_state,
        NetworkLiveCaptureProofState::ProofReady
    );
    assert_eq!(
        status.raw_capture_storage_state,
        NetworkRawCaptureStorageState::CustodyReady
    );
    assert!(status.capture_ready);
    assert!(status.raw_artifact_storage_authorized);
    assert!(status.missing_artifacts.is_empty());
    assert_eq!(status.live_capture_proof_ref, "network-live-capture-row13");
    assert_eq!(
        status.raw_capture_storage_proof_ref,
        "network-live-capture-storage-row03a"
    );
    assert!(!status.driver_invoked);
    assert!(!status.live_capture_executed);
    assert!(!status.raw_artifact_created);
    assert!(!status.remote_upload_enabled);
    assert!(!status.raw_pcap_without_custody_available);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.policy_authority);
    assert!(!status.adapter_authority);
    assert_eq!(status.enforcement_commands_published, 0);
}

#[test]
fn live_capture_custody_status_records_manual_required_capture_and_storage_gaps() {
    let live_capture_proof = plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        permission_granted: false,
        permission_proof_ref: None,
        custody_ref: None,
        ..live_capture_input()
    })
    .expect("manual-required live capture proof should parse");
    let raw_capture_storage_proof =
        plan_network_raw_capture_storage(NetworkRawCaptureStorageInput {
            live_capture_proof: live_capture_proof.clone(),
            delete_export_ref: None,
            delete_export_verified: false,
            ..raw_capture_storage_input(live_capture_proof.clone())
        })
        .expect("manual-required raw storage proof should parse");

    let status =
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            live_capture_proof,
            raw_capture_storage_proof,
            ..status_ready_input()
        })
        .expect("manual-required gates should still materialize status");

    assert_eq!(
        status.state,
        NetworkLiveCaptureCustodyStatusState::ManualRequired
    );
    assert!(!status.capture_ready);
    assert!(!status.raw_artifact_storage_authorized);
    assert_eq!(
        status.missing_artifacts,
        vec![
            NetworkLiveCaptureCustodyStatusMissingArtifact::LiveCapture(
                NetworkLiveCaptureRequiredArtifact::PermissionProof,
            ),
            NetworkLiveCaptureCustodyStatusMissingArtifact::LiveCapture(
                NetworkLiveCaptureRequiredArtifact::CustodyProof,
            ),
            NetworkLiveCaptureCustodyStatusMissingArtifact::RawCaptureStorage(
                NetworkRawCaptureStorageRequiredArtifact::LiveCaptureProof,
            ),
            NetworkLiveCaptureCustodyStatusMissingArtifact::RawCaptureStorage(
                NetworkRawCaptureStorageRequiredArtifact::DeleteExport,
            ),
        ]
    );
    assert!(!status.live_capture_executed);
    assert!(!status.raw_artifact_created);
}

#[test]
fn live_capture_custody_status_preserves_unavailable_and_degraded_states() {
    let unavailable_live_capture = plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        platform_available: false,
        ..live_capture_input()
    })
    .expect("unavailable live capture proof should parse");
    let unavailable_status =
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            raw_capture_storage_proof: plan_network_raw_capture_storage(raw_capture_storage_input(
                unavailable_live_capture.clone(),
            ))
            .expect("unavailable raw storage proof should parse"),
            live_capture_proof: unavailable_live_capture,
            ..status_ready_input()
        })
        .expect("unavailable status should materialize");

    assert_eq!(
        unavailable_status.state,
        NetworkLiveCaptureCustodyStatusState::Unavailable
    );
    assert_eq!(
        unavailable_status.raw_capture_storage_state,
        NetworkRawCaptureStorageState::Unavailable
    );

    let degraded_live_capture = plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        adapter_degraded: true,
        ..live_capture_input()
    })
    .expect("degraded live capture proof should parse");
    let degraded_status =
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            raw_capture_storage_proof: plan_network_raw_capture_storage(raw_capture_storage_input(
                degraded_live_capture.clone(),
            ))
            .expect("degraded raw storage proof should parse"),
            live_capture_proof: degraded_live_capture,
            ..status_ready_input()
        })
        .expect("degraded status should materialize");

    assert_eq!(
        degraded_status.state,
        NetworkLiveCaptureCustodyStatusState::Degraded
    );
    assert_eq!(
        degraded_status.raw_capture_storage_state,
        NetworkRawCaptureStorageState::Degraded
    );
}

#[test]
fn live_capture_custody_status_rejects_mismatched_proof_refs() {
    let mismatched_storage = plan_network_raw_capture_storage(raw_capture_storage_input(
        live_capture_with_ref("other-live-capture-proof-ref"),
    ))
    .expect("mismatched storage proof fixture should parse before composition");

    assert_eq!(
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            raw_capture_storage_proof: mismatched_storage,
            ..status_ready_input()
        }),
        Err(NetworkLiveCaptureCustodyStatusError::MismatchedLiveCaptureProofRef)
    );
}

#[test]
fn live_capture_custody_status_rejects_execution_content_authority_and_remote_claims() {
    assert_eq!(
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            live_capture_execution_claimed: true,
            ..status_ready_input()
        }),
        Err(NetworkLiveCaptureCustodyStatusError::LiveCaptureExecutionClaimRejected)
    );
    assert_eq!(
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            raw_artifact_creation_claimed: true,
            ..status_ready_input()
        }),
        Err(NetworkLiveCaptureCustodyStatusError::RawArtifactCreationClaimRejected)
    );
    assert_eq!(
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            remote_upload_claimed: true,
            ..status_ready_input()
        }),
        Err(NetworkLiveCaptureCustodyStatusError::RemoteUploadClaimRejected)
    );
    assert_eq!(
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            exact_url_claimed: true,
            ..status_ready_input()
        }),
        Err(NetworkLiveCaptureCustodyStatusError::ExactUrlClaimRejected)
    );
    assert_eq!(
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            policy_authority_claimed: true,
            ..status_ready_input()
        }),
        Err(NetworkLiveCaptureCustodyStatusError::PolicyAuthorityClaimRejected)
    );
    assert_eq!(
        materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
            enforcement_command_claimed: true,
            ..status_ready_input()
        }),
        Err(NetworkLiveCaptureCustodyStatusError::EnforcementCommandClaimRejected)
    );
}

fn status_ready_input() -> NetworkLiveCaptureCustodyStatusInput {
    let live_capture_proof = proof_ready_live_capture();
    NetworkLiveCaptureCustodyStatusInput {
        status_ref: "network-live-capture-custody-status-row13a".to_owned(),
        raw_capture_storage_proof: plan_network_raw_capture_storage(raw_capture_storage_input(
            live_capture_proof.clone(),
        ))
        .expect("raw capture storage fixture should parse"),
        live_capture_proof,
        live_capture_execution_claimed: false,
        raw_artifact_creation_claimed: false,
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
        .expect("complete live capture proof refs should parse")
}

fn live_capture_with_ref(capture_proof_ref: &str) -> NetworkLiveCaptureProof {
    plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        capture_proof_ref: capture_proof_ref.to_owned(),
        ..live_capture_input()
    })
    .expect("live capture ref fixture should parse")
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

fn raw_capture_storage_input(
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
