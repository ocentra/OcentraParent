use crate::{
    evaluate_network_raw_capture_storage_proof, plan_network_live_capture_proof,
    NetworkLiveCapturePlatform, NetworkLiveCaptureProof, NetworkLiveCaptureProofInput,
    NetworkLiveCaptureProofState, NetworkRawCaptureArtifactKind,
    NetworkRawCaptureStorageProofError, NetworkRawCaptureStorageProofInput,
    NetworkRawCaptureStorageRequiredArtifact, NetworkRawCaptureStorageState,
    NetworkRetentionReadinessProof,
};

#[test]
fn raw_capture_storage_accepts_custody_ready_artifact_governance_refs() {
    let proof = evaluate_network_raw_capture_storage_proof(storage_input(live_capture_ready()))
        .expect("complete raw capture storage custody refs should pass");

    assert_eq!(
        proof.storage_state,
        NetworkRawCaptureStorageState::CustodyReady
    );
    assert!(proof.missing_required_artifacts.is_empty());
    assert_eq!(proof.live_capture_proof_ref, "network-live-capture-row03a");
    assert_eq!(
        proof.live_capture_state,
        NetworkLiveCaptureProofState::ProofReady
    );
    assert_eq!(
        proof.raw_artifact_manifest_ref,
        Some("raw-capture-artifact-manifest-row03a".to_owned())
    );
    assert_eq!(
        proof.storage_location_ref,
        Some("encrypted-local-storage-location-row03a".to_owned())
    );
    assert_eq!(proof.retention_refs.len(), 6);
    assert!(proof.raw_artifact_stored);
    assert!(proof.encrypted_at_rest);
    assert!(proof.quota_rotation_governed);
    assert!(proof.retention_policy_governed);
    assert!(proof.delete_export_governed);
    assert!(proof.custody_governed);
    assert!(proof.private_traffic_exclusion_governed);
    assert!(!proof.remote_upload_enabled);
    assert!(!proof.live_capture_executed);
    assert!(!proof.raw_pcap_without_custody_available);
    assert_eq!(proof.enforcement_commands_published, 0);
}

#[test]
fn raw_capture_storage_records_manual_required_custody_gaps_when_artifact_is_touched() {
    let proof = evaluate_network_raw_capture_storage_proof(NetworkRawCaptureStorageProofInput {
        raw_artifact_manifest_ref: None,
        storage_location_ref: None,
        encryption_at_rest_verified: false,
        delete_export_verified: false,
        custody_chain_verified: false,
        ..storage_input(live_capture_ready())
    })
    .expect("missing storage custody refs should remain reportable");

    assert_eq!(
        proof.storage_state,
        NetworkRawCaptureStorageState::ManualRequired
    );
    assert!(!proof.raw_artifact_stored);
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkRawCaptureStorageRequiredArtifact::RawArtifactManifest,
            NetworkRawCaptureStorageRequiredArtifact::StorageLocation,
            NetworkRawCaptureStorageRequiredArtifact::EncryptionAtRest,
            NetworkRawCaptureStorageRequiredArtifact::DeleteExport,
            NetworkRawCaptureStorageRequiredArtifact::Custody,
        ]
    );
    assert!(!proof.encrypted_at_rest);
    assert!(!proof.delete_export_governed);
    assert!(!proof.custody_governed);
    assert!(!proof.raw_pcap_without_custody_available);
}

#[test]
fn raw_capture_storage_preserves_live_capture_unavailable_and_degraded_states() {
    let unavailable = evaluate_network_raw_capture_storage_proof(storage_input(
        live_capture_with_state(NetworkLiveCaptureProofState::Unavailable),
    ))
    .expect("unavailable live capture state should stay reportable");
    assert_eq!(
        unavailable.storage_state,
        NetworkRawCaptureStorageState::Unavailable
    );
    assert!(!unavailable.raw_artifact_stored);
    assert!(unavailable
        .missing_required_artifacts
        .contains(&NetworkRawCaptureStorageRequiredArtifact::LiveCaptureProofReady));

    let degraded = evaluate_network_raw_capture_storage_proof(storage_input(
        live_capture_with_state(NetworkLiveCaptureProofState::Degraded),
    ))
    .expect("degraded live capture state should stay reportable");
    assert_eq!(
        degraded.storage_state,
        NetworkRawCaptureStorageState::Degraded
    );
    assert!(!degraded.raw_artifact_stored);
}

#[test]
fn raw_capture_storage_rejects_remote_upload_content_authority_and_execution_claims() {
    assert_eq!(
        evaluate_network_raw_capture_storage_proof(NetworkRawCaptureStorageProofInput {
            remote_upload_claimed: true,
            ..storage_input(live_capture_ready())
        }),
        Err(NetworkRawCaptureStorageProofError::RemoteUploadClaimRejected)
    );
    assert_eq!(
        evaluate_network_raw_capture_storage_proof(NetworkRawCaptureStorageProofInput {
            raw_pcap_without_custody_claimed: true,
            ..storage_input(live_capture_ready())
        }),
        Err(NetworkRawCaptureStorageProofError::RawPcapWithoutCustodyClaimRejected)
    );
    assert_eq!(
        evaluate_network_raw_capture_storage_proof(NetworkRawCaptureStorageProofInput {
            exact_url_claimed: true,
            ..storage_input(live_capture_ready())
        }),
        Err(NetworkRawCaptureStorageProofError::ExactUrlClaimRejected)
    );
    assert_eq!(
        evaluate_network_raw_capture_storage_proof(NetworkRawCaptureStorageProofInput {
            enforcement_command_claimed: true,
            ..storage_input(live_capture_ready())
        }),
        Err(NetworkRawCaptureStorageProofError::EnforcementCommandClaimRejected)
    );
}

fn storage_input(
    live_capture_proof: NetworkLiveCaptureProof,
) -> NetworkRawCaptureStorageProofInput {
    NetworkRawCaptureStorageProofInput {
        storage_proof_ref: "raw-capture-storage-proof-row03a".to_owned(),
        artifact_kind: NetworkRawCaptureArtifactKind::Pcap,
        live_capture_proof,
        retention: retention_ready(),
        raw_artifact_manifest_ref: Some("raw-capture-artifact-manifest-row03a".to_owned()),
        storage_location_ref: Some("encrypted-local-storage-location-row03a".to_owned()),
        raw_artifact_touched: true,
        encryption_at_rest_verified: true,
        quota_rotation_verified: true,
        retention_policy_verified: true,
        delete_export_verified: true,
        custody_chain_verified: true,
        private_traffic_exclusion_verified: true,
        remote_upload_claimed: false,
        live_capture_execution_claimed: false,
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

fn live_capture_ready() -> NetworkLiveCaptureProof {
    plan_network_live_capture_proof(live_capture_input())
        .expect("complete live capture proof refs should pass")
}

fn live_capture_with_state(state: NetworkLiveCaptureProofState) -> NetworkLiveCaptureProof {
    let input = match state {
        NetworkLiveCaptureProofState::Unavailable => NetworkLiveCaptureProofInput {
            platform_available: false,
            ..live_capture_input()
        },
        NetworkLiveCaptureProofState::Degraded => NetworkLiveCaptureProofInput {
            adapter_degraded: true,
            ..live_capture_input()
        },
        NetworkLiveCaptureProofState::ManualRequired => NetworkLiveCaptureProofInput {
            permission_granted: false,
            permission_proof_ref: None,
            ..live_capture_input()
        },
        NetworkLiveCaptureProofState::ProofReady => live_capture_input(),
    };
    plan_network_live_capture_proof(input).expect("live capture state fixture should build")
}

fn live_capture_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: "network-live-capture-row03a".to_owned(),
        platform: NetworkLiveCapturePlatform::WindowsNpcap,
        interface_ref: Some("npcap-interface-row03a".to_owned()),
        driver_proof_ref: Some("npcap-driver-proof-row03a".to_owned()),
        permission_proof_ref: Some("npcap-permission-proof-row03a".to_owned()),
        bounded_capture_ref: Some("bounded-capture-proof-row03a".to_owned()),
        clean_stop_ref: Some("clean-stop-proof-row03a".to_owned()),
        quota_rotation_ref: Some("quota-rotation-proof-row03a".to_owned()),
        retention_delete_export_ref: Some("retention-delete-export-row03a".to_owned()),
        custody_ref: Some("capture-custody-row03a".to_owned()),
        private_traffic_exclusion_ref: Some("private-traffic-exclusion-row03a".to_owned()),
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

fn retention_ready() -> NetworkRetentionReadinessProof {
    NetworkRetentionReadinessProof {
        encryption_at_rest_ref: "capture-encryption-at-rest-row03a".to_owned(),
        quota_rotation_ref: "capture-quota-rotation-row03a".to_owned(),
        retention_policy_ref: "capture-retention-policy-row03a".to_owned(),
        delete_export_ref: "capture-delete-export-row03a".to_owned(),
        custody_ref: "capture-custody-chain-row03a".to_owned(),
        private_family_traffic_exclusion_ref: "capture-private-traffic-row03a".to_owned(),
    }
}
