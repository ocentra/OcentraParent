use super::{
    constants, NetworkLiveCaptureExecutionStatusState, NetworkLiveCaptureProofStatusState,
    NetworkLiveCaptureStatus, NetworkLiveCaptureStatusPlatform, NetworkLiveCaptureStatusRow,
    NetworkRawCaptureStorageStatusState,
};
use ocentra_eventing::expect_value::ExpectValue;

use constants::network_flow as flow;

#[test]
fn network_live_capture_status_serializes_row13_service_readiness_without_live_capture_claims() {
    let serialized = serde_json::to_value(live_capture_status_fixture())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["statusRef"], flow::TEST_LIVE_CAPTURE_STATUS_REF);
    assert_eq!(
        serialized["row13StatusRef"],
        flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF
    );
    assert_eq!(
        serialized["executionStatusRef"],
        flow::TEST_LIVE_CAPTURE_EXECUTION_STATUS_REF
    );
    assert_eq!(
        serialized["rawStorageStatusRef"],
        flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF
    );
    assert_eq!(serialized["platformRowCount"], 4);
    assert_eq!(serialized["proofReadyCount"], 1);
    assert_eq!(serialized["manualRequiredCount"], 1);
    assert_eq!(serialized["unavailableCount"], 1);
    assert_eq!(serialized["degradedCount"], 1);
    assert_eq!(serialized["captureReadyCount"], 1);
    assert_eq!(serialized["storageCustodyReadyCount"], 1);
    assert_eq!(serialized["boundedExecutedCount"], 1);
    assert_eq!(serialized["executionManualRequiredCount"], 1);
    assert_eq!(serialized["executionUnavailableCount"], 1);
    assert_eq!(serialized["executionDegradedCount"], 1);
    assert_eq!(serialized["executionMissingArtifactCount"], 30);
    assert_eq!(serialized["capturedPacketCount"], 3);
    assert_eq!(serialized["driverInvokedCount"], 1);
    assert_eq!(serialized["liveCaptureExecutedCount"], 1);
    assert_eq!(serialized["remoteUploadEnabledCount"], 0);
    assert_eq!(serialized["rawArtifactCreatedCount"], 0);
    assert_eq!(serialized["rawPcapWithoutCustodyAvailableCount"], 0);
    assert_eq!(serialized["exactUrlAvailableCount"], 0);
    assert_eq!(serialized["policyAuthorityCount"], 0);
    assert_eq!(serialized["adapterAuthorityCount"], 0);
    assert_eq!(serialized["enforcementCommandEventCount"], 0);
    assert_eq!(serialized["netstatMetadataSubstitutionCount"], 0);
    assert_eq!(serialized["hostFilteringClaimCount"], 0);
    assert_eq!(
        serialized["rows"][0]["captureProofRef"],
        flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF
    );
    assert_eq!(
        serialized["rows"][0]["executionRef"],
        flow::TEST_LIVE_CAPTURE_WINDOWS_EXECUTION_REF
    );
    assert_eq!(serialized["rows"][0]["proofState"], "proof-ready");
    assert_eq!(serialized["rows"][0]["executionState"], "bounded-executed");
    assert_eq!(serialized["rows"][0]["storageState"], "custody-ready");
    assert_eq!(serialized["rows"][1]["proofState"], "manual-required");
    assert_eq!(serialized["rows"][2]["proofState"], "unavailable");
    assert_eq!(serialized["rows"][3]["proofState"], "degraded");
}

fn live_capture_status_fixture() -> NetworkLiveCaptureStatus {
    NetworkLiveCaptureStatus {
        status_ref: flow::TEST_LIVE_CAPTURE_STATUS_REF.to_string(),
        row13_status_ref: flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF.to_string(),
        execution_status_ref: flow::TEST_LIVE_CAPTURE_EXECUTION_STATUS_REF.to_string(),
        raw_storage_status_ref: flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF.to_string(),
        platform_row_count: 4,
        proof_ready_count: 1,
        manual_required_count: 1,
        unavailable_count: 1,
        degraded_count: 1,
        required_artifact_count: 36,
        missing_artifact_count: 18,
        storage_custody_ready_count: 1,
        storage_manual_required_count: 1,
        storage_unavailable_count: 1,
        storage_degraded_count: 1,
        storage_missing_artifact_count: 21,
        bounded_executed_count: 1,
        execution_manual_required_count: 1,
        execution_unavailable_count: 1,
        execution_degraded_count: 1,
        execution_missing_artifact_count: 30,
        captured_packet_count: 3,
        driver_invoked_count: 1,
        live_capture_executed_count: 1,
        capture_ready_count: 1,
        raw_artifact_storage_authorized_count: 1,
        rows: live_capture_status_rows(),
        ..NetworkLiveCaptureStatus::default()
    }
}

fn live_capture_status_rows() -> Vec<NetworkLiveCaptureStatusRow> {
    vec![
        live_capture_status_row(LiveCaptureStatusRowFixture {
            platform: NetworkLiveCaptureStatusPlatform::WindowsNpcap,
            proof_ref: flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF,
            execution_ref: flow::TEST_LIVE_CAPTURE_WINDOWS_EXECUTION_REF,
            proof_state: NetworkLiveCaptureProofStatusState::ProofReady,
            execution_state: NetworkLiveCaptureExecutionStatusState::BoundedExecuted,
            storage_state: NetworkRawCaptureStorageStatusState::CustodyReady,
            missing_artifact_count: 0,
            storage_missing_artifact_count: 0,
            execution_missing_artifact_count: 0,
            capture_ready: true,
            raw_artifact_storage_authorized: true,
            driver_invoked: true,
            live_capture_executed: true,
            captured_packet_count: 3,
        }),
        manual_live_capture_status_row(),
        unavailable_live_capture_status_row(),
        degraded_live_capture_status_row(),
    ]
}

fn manual_live_capture_status_row() -> NetworkLiveCaptureStatusRow {
    live_capture_status_row(LiveCaptureStatusRowFixture {
        platform: NetworkLiveCaptureStatusPlatform::WindowsNpcap,
        proof_ref: flow::TEST_LIVE_CAPTURE_MANUAL_PROOF_REF,
        execution_ref: flow::TEST_LIVE_CAPTURE_MANUAL_EXECUTION_REF,
        proof_state: NetworkLiveCaptureProofStatusState::ManualRequired,
        execution_state: NetworkLiveCaptureExecutionStatusState::ManualRequired,
        storage_state: NetworkRawCaptureStorageStatusState::ManualRequired,
        missing_artifact_count: 9,
        storage_missing_artifact_count: 9,
        execution_missing_artifact_count: 10,
        capture_ready: false,
        raw_artifact_storage_authorized: false,
        driver_invoked: false,
        live_capture_executed: false,
        captured_packet_count: 0,
    })
}

fn unavailable_live_capture_status_row() -> NetworkLiveCaptureStatusRow {
    live_capture_status_row(LiveCaptureStatusRowFixture {
        platform: NetworkLiveCaptureStatusPlatform::LinuxLibpcap,
        proof_ref: flow::TEST_LIVE_CAPTURE_LINUX_PROOF_REF,
        execution_ref: flow::TEST_LIVE_CAPTURE_LINUX_EXECUTION_REF,
        proof_state: NetworkLiveCaptureProofStatusState::Unavailable,
        execution_state: NetworkLiveCaptureExecutionStatusState::Unavailable,
        storage_state: NetworkRawCaptureStorageStatusState::Unavailable,
        missing_artifact_count: 9,
        storage_missing_artifact_count: 9,
        execution_missing_artifact_count: 10,
        capture_ready: false,
        raw_artifact_storage_authorized: false,
        driver_invoked: false,
        live_capture_executed: false,
        captured_packet_count: 0,
    })
}

fn degraded_live_capture_status_row() -> NetworkLiveCaptureStatusRow {
    live_capture_status_row(LiveCaptureStatusRowFixture {
        platform: NetworkLiveCaptureStatusPlatform::MacosBpfLibpcap,
        proof_ref: flow::TEST_LIVE_CAPTURE_MACOS_PROOF_REF,
        execution_ref: flow::TEST_LIVE_CAPTURE_MACOS_EXECUTION_REF,
        proof_state: NetworkLiveCaptureProofStatusState::Degraded,
        execution_state: NetworkLiveCaptureExecutionStatusState::Degraded,
        storage_state: NetworkRawCaptureStorageStatusState::Degraded,
        missing_artifact_count: 0,
        storage_missing_artifact_count: 3,
        execution_missing_artifact_count: 10,
        capture_ready: false,
        raw_artifact_storage_authorized: false,
        driver_invoked: false,
        live_capture_executed: false,
        captured_packet_count: 0,
    })
}

struct LiveCaptureStatusRowFixture {
    platform: NetworkLiveCaptureStatusPlatform,
    proof_ref: &'static str,
    execution_ref: &'static str,
    proof_state: NetworkLiveCaptureProofStatusState,
    execution_state: NetworkLiveCaptureExecutionStatusState,
    storage_state: NetworkRawCaptureStorageStatusState,
    missing_artifact_count: u64,
    storage_missing_artifact_count: u64,
    execution_missing_artifact_count: u64,
    capture_ready: bool,
    raw_artifact_storage_authorized: bool,
    driver_invoked: bool,
    live_capture_executed: bool,
    captured_packet_count: u64,
}

fn live_capture_status_row(fixture: LiveCaptureStatusRowFixture) -> NetworkLiveCaptureStatusRow {
    NetworkLiveCaptureStatusRow {
        platform: fixture.platform,
        capture_proof_ref: fixture.proof_ref.to_string(),
        proof_state: fixture.proof_state,
        storage_proof_ref: flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF.to_string(),
        storage_state: fixture.storage_state,
        interface_ref: Some(flow::TEST_LIVE_CAPTURE_INTERFACE_REF.to_string()),
        driver_proof_ref: Some(flow::TEST_LIVE_CAPTURE_DRIVER_REF.to_string()),
        permission_proof_ref: Some(flow::TEST_LIVE_CAPTURE_PERMISSION_REF.to_string()),
        bounded_capture_ref: Some(flow::TEST_LIVE_CAPTURE_BOUNDED_REF.to_string()),
        clean_stop_ref: Some(flow::TEST_LIVE_CAPTURE_CLEAN_STOP_REF.to_string()),
        quota_rotation_ref: Some(flow::TEST_LIVE_CAPTURE_QUOTA_REF.to_string()),
        retention_delete_export_ref: Some(flow::TEST_LIVE_CAPTURE_RETENTION_REF.to_string()),
        custody_ref: Some(flow::TEST_LIVE_CAPTURE_CUSTODY_REF.to_string()),
        private_traffic_exclusion_ref: Some(
            flow::TEST_LIVE_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF.to_string(),
        ),
        raw_artifact_manifest_ref: Some(flow::TEST_RAW_CAPTURE_MANIFEST_REF.to_string()),
        storage_location_ref: Some(flow::TEST_RAW_CAPTURE_STORAGE_LOCATION_REF.to_string()),
        encryption_at_rest_ref: Some(flow::TEST_RAW_CAPTURE_ENCRYPTION_REF.to_string()),
        storage_quota_rotation_ref: Some(flow::TEST_RAW_CAPTURE_QUOTA_REF.to_string()),
        retention_policy_ref: Some(flow::TEST_RAW_CAPTURE_RETENTION_REF.to_string()),
        storage_delete_export_ref: Some(flow::TEST_RAW_CAPTURE_DELETE_EXPORT_REF.to_string()),
        custody_chain_ref: Some(flow::TEST_RAW_CAPTURE_CUSTODY_CHAIN_REF.to_string()),
        storage_private_traffic_exclusion_ref: Some(
            flow::TEST_RAW_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF.to_string(),
        ),
        execution_ref: Some(fixture.execution_ref.to_string()),
        execution_state: fixture.execution_state,
        execution_missing_artifact_count: fixture.execution_missing_artifact_count,
        driver_invocation_ref: fixture
            .driver_invoked
            .then_some(flow::TEST_LIVE_CAPTURE_DRIVER_INVOCATION_REF.to_string()),
        interface_observation_ref: fixture
            .live_capture_executed
            .then_some(flow::TEST_LIVE_CAPTURE_INTERFACE_OBSERVATION_REF.to_string()),
        execution_permission_ref: fixture
            .live_capture_executed
            .then_some(flow::TEST_LIVE_CAPTURE_EXECUTION_PERMISSION_REF.to_string()),
        bounded_window_ref: fixture
            .live_capture_executed
            .then_some(flow::TEST_LIVE_CAPTURE_BOUNDED_WINDOW_REF.to_string()),
        execution_clean_stop_ref: fixture
            .live_capture_executed
            .then_some(flow::TEST_LIVE_CAPTURE_EXECUTION_CLEAN_STOP_REF.to_string()),
        execution_custody_ref: fixture
            .live_capture_executed
            .then_some(flow::TEST_LIVE_CAPTURE_EXECUTION_CUSTODY_REF.to_string()),
        execution_retention_delete_export_ref: fixture
            .live_capture_executed
            .then_some(flow::TEST_LIVE_CAPTURE_EXECUTION_RETENTION_REF.to_string()),
        metadata_only_sanitization_ref: fixture
            .live_capture_executed
            .then_some(flow::TEST_LIVE_CAPTURE_METADATA_SANITIZATION_REF.to_string()),
        execution_private_traffic_exclusion_ref: fixture
            .live_capture_executed
            .then_some(flow::TEST_LIVE_CAPTURE_EXECUTION_PRIVATE_TRAFFIC_EXCLUSION_REF.to_string()),
        driver_invoked: fixture.driver_invoked,
        live_capture_executed: fixture.live_capture_executed,
        captured_packet_count: fixture.captured_packet_count,
        missing_artifact_count: fixture.missing_artifact_count,
        storage_missing_artifact_count: fixture.storage_missing_artifact_count,
        capture_ready: fixture.capture_ready,
        raw_artifact_storage_authorized: fixture.raw_artifact_storage_authorized,
        ..NetworkLiveCaptureStatusRow::default()
    }
}
