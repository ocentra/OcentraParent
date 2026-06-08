use super::{
    constants, NetworkLiveCaptureProofStatusState, NetworkLiveCaptureStatus,
    NetworkLiveCaptureStatusPlatform, NetworkLiveCaptureStatusRow,
    NetworkRawCaptureStorageStatusState,
};

use constants::network_flow as flow;

#[test]
fn network_live_capture_status_serializes_row13_service_readiness_without_live_capture_claims() {
    let serialized = serde_json::to_value(live_capture_status_fixture())
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["statusRef"], flow::TEST_LIVE_CAPTURE_STATUS_REF);
    assert_eq!(
        serialized["row13StatusRef"],
        flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF
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
    assert_eq!(serialized["driverInvokedCount"], 0);
    assert_eq!(serialized["liveCaptureExecutedCount"], 0);
    assert_eq!(serialized["remoteUploadEnabledCount"], 0);
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
    assert_eq!(serialized["rows"][0]["proofState"], "proof-ready");
    assert_eq!(serialized["rows"][0]["storageState"], "custody-ready");
    assert_eq!(serialized["rows"][1]["proofState"], "manual-required");
    assert_eq!(serialized["rows"][2]["proofState"], "unavailable");
    assert_eq!(serialized["rows"][3]["proofState"], "degraded");
}

fn live_capture_status_fixture() -> NetworkLiveCaptureStatus {
    NetworkLiveCaptureStatus {
        status_ref: flow::TEST_LIVE_CAPTURE_STATUS_REF.to_string(),
        row13_status_ref: flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF.to_string(),
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
        capture_ready_count: 1,
        raw_artifact_storage_authorized_count: 1,
        rows: vec![
            live_capture_status_row(
                NetworkLiveCaptureStatusPlatform::WindowsNpcap,
                flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF,
                NetworkLiveCaptureProofStatusState::ProofReady,
                NetworkRawCaptureStorageStatusState::CustodyReady,
                (0, 0),
                (true, true),
            ),
            live_capture_status_row(
                NetworkLiveCaptureStatusPlatform::WindowsNpcap,
                flow::TEST_LIVE_CAPTURE_MANUAL_PROOF_REF,
                NetworkLiveCaptureProofStatusState::ManualRequired,
                NetworkRawCaptureStorageStatusState::ManualRequired,
                (9, 9),
                (false, false),
            ),
            live_capture_status_row(
                NetworkLiveCaptureStatusPlatform::LinuxLibpcap,
                flow::TEST_LIVE_CAPTURE_LINUX_PROOF_REF,
                NetworkLiveCaptureProofStatusState::Unavailable,
                NetworkRawCaptureStorageStatusState::Unavailable,
                (9, 9),
                (false, false),
            ),
            live_capture_status_row(
                NetworkLiveCaptureStatusPlatform::MacosBpfLibpcap,
                flow::TEST_LIVE_CAPTURE_MACOS_PROOF_REF,
                NetworkLiveCaptureProofStatusState::Degraded,
                NetworkRawCaptureStorageStatusState::Degraded,
                (0, 3),
                (false, false),
            ),
        ],
        ..NetworkLiveCaptureStatus::default()
    }
}

fn live_capture_status_row(
    platform: NetworkLiveCaptureStatusPlatform,
    proof_ref: &str,
    proof_state: NetworkLiveCaptureProofStatusState,
    storage_state: NetworkRawCaptureStorageStatusState,
    missing_artifact_counts: (u64, u64),
    readiness: (bool, bool),
) -> NetworkLiveCaptureStatusRow {
    NetworkLiveCaptureStatusRow {
        platform,
        capture_proof_ref: proof_ref.to_string(),
        proof_state,
        storage_proof_ref: flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF.to_string(),
        storage_state,
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
        missing_artifact_count: missing_artifact_counts.0,
        storage_missing_artifact_count: missing_artifact_counts.1,
        capture_ready: readiness.0,
        raw_artifact_storage_authorized: readiness.1,
        ..NetworkLiveCaptureStatusRow::default()
    }
}
