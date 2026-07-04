use std::fmt::{Debug, Display};

use ocentra_parent_agent_core::network_event_runtime::remote_delivery_no_enforcement_invariant::prove_network_runtime_remote_delivery_no_enforcement_invariant;
use ocentra_parent_agent_core::network_event_runtime::remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxState;
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_transport_dispatch_state::{
    prove_network_runtime_remote_delivery_transport_dispatch_state,
    prove_network_runtime_remote_delivery_transport_dispatch_state_from_invariant,
};
use super::remote_delivery_transport_dispatch_state_types::{
    NetworkRuntimeRemoteDeliveryTransportDispatchState,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateError,
};
use crate::test_text::TestText;

type TestResult = Result<(), TestText>;
type TestResultValue<T> = Result<T, TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> TestResultValue<T> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

#[tokio::test]
async fn network_runtime_remote_delivery_transport_dispatch_state_blocks_without_transport(
) -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_transport_dispatch_state().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE,
    )?;

    assert_eq!(
        report.dispatch_state_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF
    );
    assert_eq!(
        report.blocked_dispatch_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF
    );
    assert_eq!(
        report.future_transport_seam_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF
    );
    assert_eq!(
        report.state,
        NetworkRuntimeRemoteDeliveryTransportDispatchState::ManualRequiredBlocked
    );
    assert_eq!(
        report.source_outbox_candidate_count,
        report.blocked_dispatch_record_count
    );
    assert_eq!(
        report.blocked_dispatch_record_count,
        report
            .no_enforcement_invariant
            .manual_required_candidate_count
    );
    assert_eq!(report.dispatch_ready_candidate_count, 0);
    assert_eq!(report.dispatch_attempt_count, 0);
    assert_eq!(report.remote_ack_count, 0);
    assert!(!report.broker_delivery_implemented);
    assert!(!report.family_hub_delivery_implemented);
    assert!(!report.remote_delivery_ack_implemented);
    assert!(!report.provider_delivery_implemented);
    assert!(!report.child_device_delivery_implemented);
    assert!(!report.remote_delete_export_propagation_implemented);
    assert!(!report.product_ready_remote_delivery);
    assert!(!report.policy_authority);
    assert!(!report.side_effect_authority);
    assert_eq!(report.enforcement_command_event_count, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert_eq!(report.raw_pcap_available_count, 0);
    assert_eq!(report.exact_url_available_count, 0);
    assert_eq!(report.decrypted_payload_available_count, 0);
    assert_eq!(report.page_content_available_count, 0);
    assert_eq!(report.video_content_available_count, 0);
    assert_eq!(report.private_message_content_available_count, 0);
    assert_eq!(report.search_query_available_count, 0);
    for record in &report.blocked_dispatch_records {
        assert_eq!(
            record.source_outbox_state,
            NetworkRuntimeRemoteDeliveryOutboxState::PreparedNotDispatched
        );
        assert_eq!(
            record.blocked_state,
            NetworkRuntimeRemoteDeliveryTransportDispatchState::ManualRequiredBlocked
        );
        assert_eq!(record.dispatch_state_ref, report.dispatch_state_ref);
        assert_eq!(record.blocked_dispatch_ref, report.blocked_dispatch_ref);
        assert_eq!(
            record.future_transport_seam_ref,
            report.future_transport_seam_ref
        );
    }

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_transport_dispatch_state_rejects_action_claims(
) -> TestResult {
    let mut no_enforcement_invariant = ok(
        prove_network_runtime_remote_delivery_no_enforcement_invariant().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_NO_ENFORCEMENT_INVARIANT,
    )?;
    no_enforcement_invariant.dispatch_attempt_count = 1;
    no_enforcement_invariant.remote_ack_count = 1;

    let proof_result =
        prove_network_runtime_remote_delivery_transport_dispatch_state_from_invariant(
            no_enforcement_invariant,
        );

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryTransportDispatchStateError::UnsupportedClaim)
    ));

    Ok(())
}
