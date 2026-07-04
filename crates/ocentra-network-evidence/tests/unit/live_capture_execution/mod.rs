mod fixtures;

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::live_capture::*;
use ocentra_network_evidence::live_capture_execution::*;

use self::fixtures::{executed_input, live_capture_with_state, proof_ready_live_capture};

#[test]
fn live_capture_execution_accepts_bounded_driver_run_with_custody_refs() {
    let proof = prove_network_live_capture_execution(executed_input(proof_ready_live_capture()))
        .expect_value("bounded driver execution with custody refs should parse");

    assert_eq!(
        proof.execution_state,
        NetworkLiveCaptureExecutionState::BoundedExecuted
    );
    assert_eq!(proof.execution_ref, "network-live-capture-execution-row13b");
    assert_eq!(proof.capture_proof_ref, "network-live-capture-row13");
    assert_eq!(
        proof.source,
        NetworkLiveCaptureExecutionSource::WindowsNpcapDriver
    );
    assert!(proof.missing_artifacts.is_empty());
    assert!(proof.driver_invoked);
    assert!(proof.live_capture_executed);
    assert_eq!(proof.captured_packet_count, 3);
    assert!(!proof.raw_artifact_created);
    assert!(!proof.raw_pcap_without_custody_available);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.policy_authority);
    assert!(!proof.adapter_authority);
    assert!(!proof.host_filtering_claimed);
    assert_eq!(proof.enforcement_commands_published, 0);
    assert!(!proof.netstat_metadata_substituted_for_live_capture);
}

#[test]
fn live_capture_execution_keeps_metadata_snapshot_separate_from_packet_capture() {
    let proof = prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
        source: NetworkLiveCaptureExecutionSource::MetadataSnapshotOnly,
        driver_invoked: false,
        live_capture_executed: false,
        metadata_snapshot_executed: true,
        captured_packet_count: 0,
        driver_invocation_ref: None,
        interface_observation_ref: None,
        permission_ref: None,
        bounded_window_ref: None,
        clean_stop_ref: None,
        custody_ref: None,
        retention_delete_export_ref: None,
        metadata_only_sanitization_ref: None,
        private_traffic_exclusion_ref: None,
        ..executed_input(proof_ready_live_capture())
    })
    .expect_value("metadata snapshot should remain a non-capture observation");

    assert_eq!(
        proof.execution_state,
        NetworkLiveCaptureExecutionState::ManualRequired
    );
    assert!(proof.metadata_snapshot_executed);
    assert!(!proof.driver_invoked);
    assert!(!proof.live_capture_executed);
    assert_eq!(proof.captured_packet_count, 0);
    assert!(!proof.netstat_metadata_substituted_for_live_capture);
    assert_eq!(
        proof.missing_artifacts,
        vec![
            NetworkLiveCaptureExecutionRequiredArtifact::DriverInvocation,
            NetworkLiveCaptureExecutionRequiredArtifact::InterfaceObservation,
            NetworkLiveCaptureExecutionRequiredArtifact::Permission,
            NetworkLiveCaptureExecutionRequiredArtifact::BoundedWindow,
            NetworkLiveCaptureExecutionRequiredArtifact::CleanStop,
            NetworkLiveCaptureExecutionRequiredArtifact::Custody,
            NetworkLiveCaptureExecutionRequiredArtifact::RetentionDeleteExport,
            NetworkLiveCaptureExecutionRequiredArtifact::MetadataOnlySanitization,
            NetworkLiveCaptureExecutionRequiredArtifact::PrivateTrafficExclusion,
        ]
    );
}

#[test]
fn live_capture_execution_stays_manual_required_when_clean_stop_or_custody_refs_are_missing() {
    let proof = prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
        clean_stop_ref: None,
        custody_ref: None,
        ..executed_input(proof_ready_live_capture())
    })
    .expect_value("missing custody refs should parse as manual-required");

    assert_eq!(
        proof.execution_state,
        NetworkLiveCaptureExecutionState::ManualRequired
    );
    assert!(!proof.driver_invoked);
    assert!(!proof.live_capture_executed);
    assert_eq!(proof.captured_packet_count, 0);
    assert_eq!(
        proof.missing_artifacts,
        vec![
            NetworkLiveCaptureExecutionRequiredArtifact::CleanStop,
            NetworkLiveCaptureExecutionRequiredArtifact::Custody,
        ]
    );
}

#[test]
fn live_capture_execution_preserves_unavailable_and_degraded_capture_states() {
    let unavailable = prove_network_live_capture_execution(executed_input(
        live_capture_with_state(NetworkLiveCaptureProofState::Unavailable),
    ))
    .expect_value("unavailable capture proof should remain visible");
    assert_eq!(
        unavailable.execution_state,
        NetworkLiveCaptureExecutionState::Unavailable
    );
    assert!(!unavailable.live_capture_executed);

    let degraded = prove_network_live_capture_execution(executed_input(live_capture_with_state(
        NetworkLiveCaptureProofState::Degraded,
    )))
    .expect_value("degraded capture proof should remain visible");
    assert_eq!(
        degraded.execution_state,
        NetworkLiveCaptureExecutionState::Degraded
    );
    assert!(!degraded.live_capture_executed);
}

#[test]
fn live_capture_execution_rejects_shape_drift_and_unsupported_claims() {
    assert_eq!(
        prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
            live_capture_executed: true,
            driver_invoked: false,
            ..executed_input(proof_ready_live_capture())
        }),
        Err(NetworkLiveCaptureExecutionError::DriverExecutionRequiresPacketObservation)
    );
    assert_eq!(
        prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
            source: NetworkLiveCaptureExecutionSource::MetadataSnapshotOnly,
            driver_invoked: true,
            live_capture_executed: true,
            ..executed_input(proof_ready_live_capture())
        }),
        Err(NetworkLiveCaptureExecutionError::MetadataSnapshotCannotClaimDriverExecution)
    );
    assert_eq!(
        prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
            source: NetworkLiveCaptureExecutionSource::LinuxLibpcapDriver,
            ..executed_input(proof_ready_live_capture())
        }),
        Err(NetworkLiveCaptureExecutionError::SourcePlatformMismatch)
    );
    assert_eq!(
        prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
            netstat_metadata_substitution_claimed: true,
            ..executed_input(proof_ready_live_capture())
        }),
        Err(NetworkLiveCaptureExecutionError::NetstatSubstitutionClaimRejected)
    );
    assert_eq!(
        prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
            raw_artifact_created: true,
            ..executed_input(proof_ready_live_capture())
        }),
        Err(NetworkLiveCaptureExecutionError::RawArtifactCreationRejected)
    );
    assert_eq!(
        prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
            exact_url_claimed: true,
            ..executed_input(proof_ready_live_capture())
        }),
        Err(NetworkLiveCaptureExecutionError::ExactUrlClaimRejected)
    );
    assert_eq!(
        prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
            host_filtering_claimed: true,
            ..executed_input(proof_ready_live_capture())
        }),
        Err(NetworkLiveCaptureExecutionError::HostFilteringClaimRejected)
    );
    assert_eq!(
        prove_network_live_capture_execution(NetworkLiveCaptureExecutionInput {
            enforcement_command_claimed: true,
            ..executed_input(proof_ready_live_capture())
        }),
        Err(NetworkLiveCaptureExecutionError::EnforcementCommandClaimRejected)
    );
}
