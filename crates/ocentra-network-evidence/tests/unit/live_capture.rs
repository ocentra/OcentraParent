use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::live_capture::*;

#[test]
fn live_capture_gate_allows_proof_ready_with_driver_permission_custody_and_quota_refs() {
    let proof = plan_network_live_capture_proof(proof_ready_input())
        .expect_value("complete local capture artifacts should be proof-ready");

    assert_eq!(proof.proof_state, NetworkLiveCaptureProofState::ProofReady);
    assert!(proof.capture_ready);
    assert!(proof.missing_artifacts.is_empty());
    assert_eq!(
        proof.interface_ref,
        Some("npcap-interface-row13".to_owned())
    );
    assert_eq!(
        proof.driver_proof_ref,
        Some("npcap-driver-proof-row13".to_owned())
    );
    assert_eq!(
        proof.retention_delete_export_ref,
        Some("live-capture-retention-delete-export-row13".to_owned())
    );
    assert!(!proof.driver_invoked);
    assert!(!proof.live_capture_executed);
    assert!(!proof.raw_pcap_without_custody_available);
    assert_eq!(proof.enforcement_commands_published, 0);
}

#[test]
fn live_capture_gate_stays_manual_required_without_permission_quota_and_custody() {
    let proof = plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        permission_granted: false,
        permission_proof_ref: None,
        quota_rotation_ref: None,
        custody_ref: None,
        ..proof_ready_input()
    })
    .expect_value("missing artifacts should produce manual-required proof state");

    assert_eq!(
        proof.proof_state,
        NetworkLiveCaptureProofState::ManualRequired
    );
    assert!(!proof.capture_ready);
    assert_eq!(
        proof.missing_artifacts,
        vec![
            NetworkLiveCaptureRequiredArtifact::PermissionProof,
            NetworkLiveCaptureRequiredArtifact::QuotaRotationProof,
            NetworkLiveCaptureRequiredArtifact::CustodyProof,
        ]
    );
    assert!(!proof.live_capture_executed);
}

#[test]
fn live_capture_gate_preserves_unavailable_and_degraded_states() {
    let unavailable = plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        platform_available: false,
        ..proof_ready_input()
    })
    .expect_value("unavailable platform should still report proof state");
    assert_eq!(
        unavailable.proof_state,
        NetworkLiveCaptureProofState::Unavailable
    );
    assert!(!unavailable.capture_ready);

    let degraded = plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        adapter_degraded: true,
        ..proof_ready_input()
    })
    .expect_value("degraded adapter should still report proof state");
    assert_eq!(degraded.proof_state, NetworkLiveCaptureProofState::Degraded);
    assert!(!degraded.capture_ready);
}

#[test]
fn live_capture_gate_rejects_live_execution_content_and_authority_claims() {
    assert_eq!(
        plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
            live_capture_execution_claimed: true,
            ..proof_ready_input()
        }),
        Err(NetworkLiveCaptureProofError::LiveCaptureExecutionClaimRejected)
    );
    assert_eq!(
        plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
            unbounded_capture_claimed: true,
            ..proof_ready_input()
        }),
        Err(NetworkLiveCaptureProofError::UnboundedCaptureClaimRejected)
    );
    assert_eq!(
        plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
            exact_url_claimed: true,
            ..proof_ready_input()
        }),
        Err(NetworkLiveCaptureProofError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
            adapter_authority_claimed: true,
            ..proof_ready_input()
        }),
        Err(NetworkLiveCaptureProofError::AdapterAuthorityClaimRejected)
    );
}

fn proof_ready_input() -> NetworkLiveCaptureProofInput {
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
