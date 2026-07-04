use std::fmt::{Debug, Display};

use ocentra_parent_agent_core::network_event_runtime::remote_delivery_fixture_transport::prove_network_runtime_remote_delivery_fixture_transport;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_provider_child_readiness::{
    prove_network_runtime_remote_delivery_provider_child_readiness,
    prove_network_runtime_remote_delivery_provider_child_readiness_from_fixture_transport,
};
use super::remote_delivery_provider_child_readiness_types::{
    NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
};
use crate::test_text::TestText;

type TestResult = Result<(), TestText>;
type TestResultValue<T> = Result<T, TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> TestResultValue<T> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

#[tokio::test]
async fn preserves_fixture_ack_refs_without_live_delivery() -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_provider_child_readiness().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_PROVIDER_CHILD_READINESS,
    )?;

    assert_eq!(
        report.provider_route_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_PROVIDER_ROUTE_REF
    );
    assert_eq!(
        report.child_device_route_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_ROUTE_REF
    );
    assert_eq!(
        report.provider_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_PROVIDER_READINESS_REF
    );
    assert_eq!(
        report.child_device_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_READINESS_REF
    );
    assert_provider_child_readiness_counts(&report);
    assert_provider_child_readiness_no_delivery_claims(&report);

    Ok(())
}

#[tokio::test]
async fn rejects_live_delivery_claims() -> TestResult {
    let mut fixture_transport = ok(
        prove_network_runtime_remote_delivery_fixture_transport().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_FIXTURE_TRANSPORT,
    )?;
    fixture_transport.provider_delivery_implemented = true;

    let proof_result =
        prove_network_runtime_remote_delivery_provider_child_readiness_from_fixture_transport(
            fixture_transport,
        );

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryProviderChildReadinessError::UnsupportedClaim)
    ));

    Ok(())
}

fn assert_provider_child_readiness_counts(
    report: &NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
) {
    assert_eq!(
        report.provider_state,
        NetworkRuntimeRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable
    );
    assert_eq!(
        report.child_device_state,
        NetworkRuntimeRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable
    );
    assert_eq!(
        report.provider_delivery_readiness_record_count,
        report.source_fixture_ack_count
    );
    assert_eq!(
        report.child_device_delivery_readiness_record_count,
        report.source_fixture_ack_count
    );
    assert_eq!(report.provider_delivery_artifact_count, 0);
    assert_eq!(report.child_device_delivery_artifact_count, 0);
}

fn assert_provider_child_readiness_no_delivery_claims(
    report: &NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
) {
    assert!(report.provider_delivery_records_match_fixture_acks);
    assert!(report.child_device_delivery_records_match_fixture_acks);
    assert!(!report.provider_delivery_implemented);
    assert!(!report.child_device_delivery_implemented);
    assert!(!report.product_ready_remote_delivery);
}
