use ocentra_eventing::{
    EventDeliveryDecisionState, EventDeliveryRequiredArtifact, EventDeliveryRouteKind,
};
use ocentra_parent_agent_protocol::constants;

use crate::network_event_runtime::{
    prove_network_runtime_remote_delivery_status, NetworkRuntimeRemoteDeliveryState,
    NetworkRuntimeRemoteDeliveryStatusError, NetworkRuntimeRemoteDeliveryStatusReport,
};

#[tokio::test]
async fn network_runtime_remote_delivery_status_preserves_broker_family_hub_refs_without_transport()
{
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryStatusReport,
        NetworkRuntimeRemoteDeliveryStatusError,
    > = prove_network_runtime_remote_delivery_status().await;
    let report =
        proof_result.expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DELIVERY_STATUS);

    assert_eq!(
        report.broker_status,
        NetworkRuntimeRemoteDeliveryState::RequirementsSatisfiedButNotImplemented
    );
    assert_eq!(
        report.family_hub_status,
        NetworkRuntimeRemoteDeliveryState::RequirementsSatisfiedButNotImplemented
    );
    assert_eq!(
        report.family_hub_decision.route_kind,
        EventDeliveryRouteKind::ExternalRelay
    );
    assert_eq!(
        report.family_hub_decision.decision_state,
        EventDeliveryDecisionState::ExternalRelayRouteRequirementsSatisfied
    );
    assert_eq!(report.broker_missing_artifact_count, 0);
    assert_eq!(report.family_hub_missing_artifact_count, 0);
    assert_eq!(report.accepted_event_type_count, 3);
    assert_eq!(
        report.custody_proof_ref.as_str(),
        constants::network_flow::TEST_BROKER_CUSTODY_PROOF_REF
    );
    assert_eq!(
        report.publisher_auth_ref.as_str(),
        constants::network_flow::TEST_BROKER_PUBLISHER_AUTH_REF
    );
    assert_eq!(
        report.subscriber_auth_ref.as_str(),
        constants::network_flow::TEST_BROKER_SUBSCRIBER_AUTH_REF
    );
    assert_eq!(
        report.encryption_ref.as_str(),
        constants::network_flow::TEST_BROKER_ENCRYPTION_REF
    );
    assert_eq!(
        report.retention_policy_ref.as_str(),
        constants::network_flow::TEST_BROKER_RETENTION_POLICY_REF
    );
    assert_eq!(
        report.replay_plan_ref.as_str(),
        constants::network_flow::TEST_BROKER_REPLAY_PLAN_REF
    );
    assert_eq!(
        report.deletion_plan_ref.as_str(),
        constants::network_flow::TEST_BROKER_DELETION_PLAN_REF
    );
    assert_eq!(
        report.offset_policy_ref.as_str(),
        constants::network_flow::TEST_BROKER_OFFSET_POLICY_REF
    );
    assert_eq!(
        report.dedupe_policy_ref.as_str(),
        constants::network_flow::TEST_BROKER_DEDUPE_POLICY_REF
    );
    assert_eq!(
        report.transport_config_ref.as_str(),
        constants::network_flow::TEST_BROKER_CONFIG_REF
    );
    assert_eq!(
        report.relay_identity_ref.as_str(),
        constants::network_flow::TEST_FAMILY_HUB_IDENTITY_REF
    );
    assert_eq!(
        report.relay_policy_ref.as_str(),
        constants::network_flow::TEST_FAMILY_HUB_RELAY_POLICY_REF
    );
}

#[tokio::test]
async fn network_runtime_remote_delivery_status_rejects_authority_and_side_effect_claims() {
    let report = prove_network_runtime_remote_delivery_status()
        .await
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DELIVERY_STATUS);

    assert!(report.local_idempotency_queue_proved);
    assert!(report.queued_duplicate_rejected);
    assert!(report.completed_duplicate_rejected);
    assert_eq!(report.dropped_event_dead_letter_count, 1);
    assert!(!report.external_transport_delivery_implemented);
    assert!(!report.family_hub_delivery_implemented);
    assert!(!report.cross_process_replay_implemented);
    assert!(!report.remote_retention_delete_export_propagation_implemented);
    assert!(!report.policy_authority);
    assert!(!report.side_effect_authority);
    assert_eq!(report.enforcement_command_event_count, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert_eq!(
        report.broker_semantics.delivery_decision.required_artifacts,
        vec![
            EventDeliveryRequiredArtifact::CustodyProof,
            EventDeliveryRequiredArtifact::PublisherAuthProof,
            EventDeliveryRequiredArtifact::SubscriberAuthProof,
            EventDeliveryRequiredArtifact::EncryptionProof,
            EventDeliveryRequiredArtifact::RetentionPolicy,
            EventDeliveryRequiredArtifact::ReplayPlan,
            EventDeliveryRequiredArtifact::DeletionPlan,
            EventDeliveryRequiredArtifact::BackpressurePolicy,
            EventDeliveryRequiredArtifact::OffsetPolicy,
            EventDeliveryRequiredArtifact::DedupePolicy,
            EventDeliveryRequiredArtifact::TransportConfig,
        ]
    );
    assert!(report
        .family_hub_decision
        .required_artifacts
        .contains(&EventDeliveryRequiredArtifact::ExternalRelayIdentity));
    assert!(report
        .family_hub_decision
        .required_artifacts
        .contains(&EventDeliveryRequiredArtifact::ExternalRelayPolicy));
}
