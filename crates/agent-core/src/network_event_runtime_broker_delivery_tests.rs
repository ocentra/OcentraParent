use ocentra_eventing::delivery::{EventDeliveryDecisionState, EventDeliveryRequiredArtifact};
use ocentra_parent_agent_protocol::constants;

use crate::network_event_runtime::broker_delivery::{
    prove_network_runtime_broker_delivery_semantics, NetworkRuntimeBrokerDeliveryProofError,
    NetworkRuntimeBrokerDeliverySemantics, NetworkRuntimeBrokerDeliverySemanticsReport,
};

#[tokio::test]
async fn network_runtime_broker_delivery_semantics_preserve_refs_without_live_broker() {
    let proof_result: Result<
        NetworkRuntimeBrokerDeliverySemanticsReport,
        NetworkRuntimeBrokerDeliveryProofError,
    > = prove_network_runtime_broker_delivery_semantics().await;
    let report = proof_result
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_BROKER_DELIVERY_SEMANTICS);

    assert_eq!(
        report.delivery_decision.decision_state,
        EventDeliveryDecisionState::ExternalTransportRouteRequirementsSatisfied
    );
    assert_eq!(
        report.delivery_semantics,
        NetworkRuntimeBrokerDeliverySemantics::LocalIdempotencyQueueProof
    );
    assert_eq!(
        report
            .delivery_decision
            .retention_policy_ref
            .as_ref()
            .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_BROKER_DELIVERY_SEMANTICS)
            .as_str(),
        constants::network_flow::TEST_BROKER_RETENTION_POLICY_REF
    );
    assert_eq!(
        report.replay_plan_ref.as_str(),
        constants::network_flow::TEST_BROKER_REPLAY_PLAN_REF
    );
    assert_eq!(
        report.dropped_event_audit_ref.as_str(),
        constants::network_flow::TEST_BROKER_DROPPED_EVENT_AUDIT_REF
    );
    assert_eq!(
        report.adapter_action_ledger_ref.as_str(),
        constants::network_flow::TEST_BROKER_ADAPTER_ACTION_LEDGER_REF
    );
    assert!(report.queued_duplicate_rejected);
    assert!(report.completed_duplicate_rejected);
    assert_eq!(report.duplicate_stored_event_count, 1);
    assert_eq!(report.dropped_event_dead_letter_count, 1);
    assert_eq!(report.enforcement_command_event_count, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert!(!report.external_transport_delivery_implemented);
    assert!(!report.external_relay_delivery_implemented);
    assert!(report
        .delivery_decision
        .required_artifacts
        .contains(&EventDeliveryRequiredArtifact::ReplayPlan));
    assert!(report
        .delivery_decision
        .required_artifacts
        .contains(&EventDeliveryRequiredArtifact::DedupePolicy));
    assert!(report
        .delivery_decision
        .required_artifacts
        .contains(&EventDeliveryRequiredArtifact::TransportConfig));
}
