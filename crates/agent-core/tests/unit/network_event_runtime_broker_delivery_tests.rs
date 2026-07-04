use std::fmt::{Debug, Display};

use ocentra_eventing::{
    delivery::validation::EventDeliveryDecisionState,
    delivery::validation::EventDeliveryRequiredArtifact,
};
use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;
use ocentra_parent_agent_core::network_event_runtime::broker_delivery::{
    prove_network_runtime_broker_delivery_semantics, NetworkRuntimeBrokerDeliverySemantics,
    NetworkRuntimeBrokerDeliverySemanticsReport,
};

type TestResult = Result<(), TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

fn some<T>(value: Option<T>, context: impl Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}

#[tokio::test]
async fn network_runtime_broker_delivery_semantics_preserve_refs_without_live_broker() -> TestResult
{
    let report: NetworkRuntimeBrokerDeliverySemanticsReport = ok(
        prove_network_runtime_broker_delivery_semantics().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_BROKER_DELIVERY_SEMANTICS,
    )?;

    assert_eq!(
        report.delivery_decision.decision_state,
        EventDeliveryDecisionState::ExternalTransportRouteRequirementsSatisfied
    );
    assert_eq!(
        report.delivery_semantics,
        NetworkRuntimeBrokerDeliverySemantics::LocalIdempotencyQueueProof
    );
    let retention_policy_ref = some(
        report.delivery_decision.retention_policy_ref.as_ref(),
        constants::network_flow::ERROR_NETWORK_RUNTIME_BROKER_DELIVERY_SEMANTICS,
    )?;
    assert_eq!(
        retention_policy_ref.as_str(),
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
    Ok(())
}
