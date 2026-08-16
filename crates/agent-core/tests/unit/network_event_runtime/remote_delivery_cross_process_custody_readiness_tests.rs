use std::fmt::{Debug, Display};

use ocentra_parent_agent_core::network_event_runtime::remote_delivery_provider_child_readiness::prove_network_runtime_remote_delivery_provider_child_readiness;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_cross_process_custody_readiness::{
    prove_network_runtime_remote_delivery_cross_process_custody_readiness,
    prove_network_runtime_remote_delivery_cross_process_custody_readiness_from_provider_child_readiness,
};
use super::remote_delivery_cross_process_custody_readiness_types::{
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState,
};
use crate::test_text::TestText;

type TestResult = Result<(), TestText>;
type TestResultValue<T> = Result<T, TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> TestResultValue<T> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

#[tokio::test]
async fn preserves_provider_child_readiness_refs_without_cross_process_claims() -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_cross_process_custody_readiness().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_CUSTODY_READINESS,
    )?;

    assert_eq!(
        report.cross_process_custody_status_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_CUSTODY_STATUS_REF
    );
    assert_eq!(
        report.cross_process_replay_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_READINESS_REF
    );
    assert_eq!(
        report.remote_retention_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_RETENTION_READINESS_REF
    );
    assert_eq!(
        report.remote_delete_custody_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_CUSTODY_REF
    );
    assert_eq!(
        report.remote_export_custody_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_CUSTODY_REF
    );
    assert_cross_process_custody_readiness_counts(&report);
    assert_cross_process_custody_readiness_no_claims(&report);

    Ok(())
}

#[tokio::test]
async fn rejects_cross_process_replay_claims() -> TestResult {
    let mut provider_child_readiness = ok(
        prove_network_runtime_remote_delivery_provider_child_readiness().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_PROVIDER_CHILD_READINESS,
    )?;
    provider_child_readiness
        .fixture_transport
        .outbox_handoff
        .durable_envelope
        .receipt_ledger
        .remote_delivery_status
        .cross_process_replay_implemented = true;

    let proof_result =
        prove_network_runtime_remote_delivery_cross_process_custody_readiness_from_provider_child_readiness(
            provider_child_readiness,
        );

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError::UnsupportedClaim)
    ));

    Ok(())
}

fn assert_cross_process_custody_readiness_counts(
    report: &NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
) {
    assert_eq!(
        report.custody_state,
        NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable
    );
    assert_eq!(
        report.cross_process_replay_readiness_record_count,
        report.source_provider_child_readiness_record_count
    );
    assert_eq!(
        report.remote_retention_readiness_record_count,
        report.source_provider_child_readiness_record_count
    );
    assert_eq!(
        report.remote_delete_custody_readiness_record_count,
        report.source_provider_child_readiness_record_count
    );
    assert_eq!(
        report.remote_export_custody_readiness_record_count,
        report.source_provider_child_readiness_record_count
    );
    assert_eq!(report.cross_process_replay_artifact_count, 0);
    assert_eq!(report.remote_retention_artifact_count, 0);
    assert_eq!(report.remote_delete_custody_artifact_count, 0);
    assert_eq!(report.remote_export_custody_artifact_count, 0);
}

fn assert_cross_process_custody_readiness_no_claims(
    report: &NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
) {
    assert!(report.custody_records_match_provider_child_readiness);
    assert!(!report.cross_process_replay_implemented);
    assert!(!report.remote_delete_export_propagation_implemented);
    assert!(!report.provider_delivery_implemented);
    assert!(!report.child_device_delivery_implemented);
    assert!(!report.product_ready_remote_delivery);
}
