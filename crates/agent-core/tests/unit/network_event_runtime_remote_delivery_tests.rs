use std::fmt::{Debug, Display};

use ocentra_eventing::{
    delivery::validation::EventDeliveryDecisionState,
    delivery::validation::EventDeliveryRequiredArtifact,
    delivery::validation::EventDeliveryRouteKind,
};
use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;
use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_dispatch_readiness::prove_network_runtime_remote_delivery_dispatch_readiness,
    remote_delivery_dispatch_readiness_types::NetworkRuntimeRemoteDeliveryDispatchReadinessError,
    remote_delivery_durable_envelope::prove_network_runtime_remote_delivery_durable_envelope,
    remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    remote_delivery_event_chain_journal::prove_network_runtime_remote_event_chain_journal,
    remote_delivery_event_chain_journal_types::NetworkRuntimeRemoteEventChainJournalError,
    remote_delivery_no_enforcement_invariant::prove_network_runtime_remote_delivery_no_enforcement_invariant,
    remote_delivery_no_enforcement_invariant_types::NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
    remote_delivery_outbox_handoff::prove_network_runtime_remote_delivery_outbox_handoff,
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    remote_delivery_receipt_ledger::prove_network_runtime_remote_delivery_receipt_ledger,
    remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptLedgerError,
    remote_delivery_status::{
        prove_network_runtime_remote_delivery_status, NetworkRuntimeRemoteDeliveryState,
        NetworkRuntimeRemoteDeliveryStatusError, NetworkRuntimeRemoteDeliveryStatusReport,
    },
};

type TestResult = Result<(), TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

fn expect_event_chain_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteEventChainJournalError>,
    context: impl Display,
) -> TestResult {
    if result.is_err_and(|error| {
        matches!(
            error,
            NetworkRuntimeRemoteEventChainJournalError::RuntimeOwnerUnavailable
        )
    }) {
        Ok(())
    } else {
        Err(TestText::from_display(format!(
            "{context}: expected RuntimeOwnerUnavailable"
        )))
    }
}

fn expect_receipt_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryReceiptLedgerError>,
    context: impl Display,
) -> TestResult {
    if result.is_err_and(|error| {
        matches!(
            error,
            NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable
        )
    }) {
        Ok(())
    } else {
        Err(TestText::from_display(format!(
            "{context}: expected RuntimeOwnerUnavailable"
        )))
    }
}

fn expect_durable_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryDurableEnvelopeError>,
    context: impl Display,
) -> TestResult {
    if result.is_err_and(|error| {
        matches!(
            error,
            NetworkRuntimeRemoteDeliveryDurableEnvelopeError::ReceiptLedger(
                NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable,
            )
        )
    }) {
        Ok(())
    } else {
        Err(TestText::from_display(format!(
            "{context}: expected ReceiptLedger(RuntimeOwnerUnavailable)"
        )))
    }
}

fn expect_outbox_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryOutboxHandoffError>,
    context: impl Display,
) -> TestResult {
    if result.is_err_and(|error| {
        matches!(
            error,
            NetworkRuntimeRemoteDeliveryOutboxHandoffError::DurableEnvelope(
                NetworkRuntimeRemoteDeliveryDurableEnvelopeError::ReceiptLedger(
                    NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable,
                ),
            )
        )
    }) {
        Ok(())
    } else {
        Err(TestText::from_display(format!(
            "{context}: expected DurableEnvelope(...RuntimeOwnerUnavailable)"
        )))
    }
}

fn expect_dispatch_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryDispatchReadinessError>,
    context: impl Display,
) -> TestResult {
    if result.is_err_and(|error| {
        matches!(
            error,
            NetworkRuntimeRemoteDeliveryDispatchReadinessError::OutboxHandoff(
                NetworkRuntimeRemoteDeliveryOutboxHandoffError::DurableEnvelope(
                    NetworkRuntimeRemoteDeliveryDurableEnvelopeError::ReceiptLedger(
                        NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable,
                    ),
                ),
            )
        )
    }) {
        Ok(())
    } else {
        Err(TestText::from_display(format!(
            "{context}: expected OutboxHandoff(...RuntimeOwnerUnavailable)"
        )))
    }
}

fn expect_no_enforcement_owner_unavailable<T>(
    result: Result<T, NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError>,
    context: impl Display,
) -> TestResult {
    if result.is_err_and(|error| {
        matches!(
            error,
            NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError::DispatchReadiness(
                NetworkRuntimeRemoteDeliveryDispatchReadinessError::OutboxHandoff(
                    NetworkRuntimeRemoteDeliveryOutboxHandoffError::DurableEnvelope(
                        NetworkRuntimeRemoteDeliveryDurableEnvelopeError::ReceiptLedger(
                            NetworkRuntimeRemoteDeliveryReceiptLedgerError::RuntimeOwnerUnavailable,
                        ),
                    ),
                ),
            )
        )
    }) {
        Ok(())
    } else {
        Err(TestText::from_display(format!(
            "{context}: expected DispatchReadiness(...RuntimeOwnerUnavailable)"
        )))
    }
}

#[tokio::test]
async fn network_runtime_remote_delivery_status_preserves_broker_family_hub_refs_without_transport(
) -> TestResult {
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryStatusReport,
        NetworkRuntimeRemoteDeliveryStatusError,
    > = prove_network_runtime_remote_delivery_status().await;
    let report = ok(
        proof_result,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DELIVERY_STATUS,
    )?;

    assert_eq!(
        report.broker_status,
        NetworkRuntimeRemoteDeliveryState::FixtureRequirementsRecordedButNotImplemented
    );
    assert_eq!(
        report.family_hub_status,
        NetworkRuntimeRemoteDeliveryState::FixtureRequirementsRecordedButNotImplemented
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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_status_rejects_authority_and_side_effect_claims(
) -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_status().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DELIVERY_STATUS,
    )?;

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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_event_chain_journal_rejects_without_runtime_owner() -> TestResult {
    expect_event_chain_owner_unavailable(
        prove_network_runtime_remote_event_chain_journal().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_EVENT_CHAIN_JOURNAL,
    )
}

#[tokio::test]
async fn network_runtime_remote_delivery_no_enforcement_invariant_rejects_without_runtime_owner(
) -> TestResult {
    expect_no_enforcement_owner_unavailable(
        prove_network_runtime_remote_delivery_no_enforcement_invariant().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_NO_ENFORCEMENT_INVARIANT,
    )
}

#[tokio::test]
async fn network_runtime_remote_delivery_receipt_ledger_rejects_without_runtime_owner() -> TestResult
{
    expect_receipt_owner_unavailable(
        prove_network_runtime_remote_delivery_receipt_ledger().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_RECEIPT_LEDGER,
    )
}

#[tokio::test]
async fn network_runtime_remote_delivery_durable_envelope_rejects_without_runtime_owner(
) -> TestResult {
    expect_durable_owner_unavailable(
        prove_network_runtime_remote_delivery_durable_envelope().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DURABLE_ENVELOPE,
    )
}

#[tokio::test]
async fn network_runtime_remote_delivery_outbox_handoff_rejects_without_runtime_owner() -> TestResult
{
    expect_outbox_owner_unavailable(
        prove_network_runtime_remote_delivery_outbox_handoff().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_HANDOFF,
    )
}

#[tokio::test]
async fn network_runtime_remote_delivery_dispatch_readiness_rejects_without_runtime_owner(
) -> TestResult {
    expect_dispatch_owner_unavailable(
        prove_network_runtime_remote_delivery_dispatch_readiness().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DISPATCH_READINESS,
    )
}
