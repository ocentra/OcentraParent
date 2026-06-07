use ocentra_eventing::{
    EventDeliveryDecisionState, EventDeliveryRequiredArtifact, EventDeliveryRouteKind, ReplayMode,
};
use ocentra_parent_agent_protocol::constants;

use crate::network_event_runtime::{
    prove_network_runtime_remote_delivery_durable_envelope,
    prove_network_runtime_remote_delivery_receipt_ledger,
    prove_network_runtime_remote_delivery_status, prove_network_runtime_remote_event_chain_journal,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    NetworkRuntimeRemoteDeliveryReceiptLedgerError,
    NetworkRuntimeRemoteDeliveryReceiptLedgerReport, NetworkRuntimeRemoteDeliveryState,
    NetworkRuntimeRemoteDeliveryStatusError, NetworkRuntimeRemoteDeliveryStatusReport,
    NetworkRuntimeRemoteEventChainJournalError, NetworkRuntimeRemoteEventChainJournalReport,
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

#[tokio::test]
async fn network_runtime_remote_event_chain_journal_preserves_export_boundary_without_transport() {
    let proof_result: Result<
        NetworkRuntimeRemoteEventChainJournalReport,
        NetworkRuntimeRemoteEventChainJournalError,
    > = prove_network_runtime_remote_event_chain_journal().await;
    let report = proof_result
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_EVENT_CHAIN_JOURNAL);

    assert_eq!(
        report.event_chain_journal_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF
    );
    assert_eq!(
        report.event_chain_replay_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_REPLAY_REF
    );
    assert_eq!(
        report.event_chain_export_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF
    );
    assert_eq!(
        report.event_chain_support_status_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_SUPPORT_STATUS_REF
    );
    assert!(report.stored_event_count > 0);
    assert_eq!(report.journal_entry_count, report.stored_event_count);
    assert_eq!(
        report.projection_replay_record_count,
        report.journal_entry_count
    );
    assert_eq!(
        report.replay_cursor_next_sequence,
        report.journal_entry_count as u64 + 1
    );
    assert_eq!(report.projection_replay_mode, ReplayMode::ProjectionOnly);
    assert_eq!(
        report.exportable_remote_envelope_count,
        report.journal_entry_count
    );
    assert_eq!(report.exported_event_type_count, report.journal_entry_count);
    assert_eq!(report.unavailable_event_count, report.journal_entry_count);
}

#[tokio::test]
async fn network_runtime_remote_event_chain_journal_rejects_delivery_and_content_claims() {
    let report = prove_network_runtime_remote_event_chain_journal()
        .await
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_EVENT_CHAIN_JOURNAL);

    assert_eq!(
        report.remote_delivery_status.broker_status,
        NetworkRuntimeRemoteDeliveryState::RequirementsSatisfiedButNotImplemented
    );
    assert!(!report.broker_delivery_implemented);
    assert!(!report.family_hub_delivery_implemented);
    assert!(!report.policy_authority);
    assert!(!report.side_effect_authority);
    assert_eq!(report.enforcement_command_event_count, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert_eq!(report.exact_url_available_count, 0);
    assert_eq!(report.decrypted_payload_available_count, 0);
    assert_eq!(report.page_content_available_count, 0);
}

#[tokio::test]
async fn network_runtime_remote_delivery_receipt_ledger_preserves_local_ack_boundary_without_transport(
) {
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
        NetworkRuntimeRemoteDeliveryReceiptLedgerError,
    > = prove_network_runtime_remote_delivery_receipt_ledger().await;
    let report =
        proof_result.expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_RECEIPT_LEDGER);

    assert_eq!(
        report.event_chain_journal_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF
    );
    assert_eq!(
        report.event_chain_export_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF
    );
    assert_eq!(
        report.receipt_ledger_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
    );
    assert_eq!(
        report.local_receipt_ack_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
    );
    assert_eq!(
        report.receipt_replay_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_REPLAY_REF
    );
    assert_eq!(
        report.receipt_support_status_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_SUPPORT_STATUS_REF
    );
    assert_eq!(
        report.remote_delivery_status.family_hub_status,
        NetworkRuntimeRemoteDeliveryState::RequirementsSatisfiedButNotImplemented
    );
    assert!(report.receipt_ledger_ready);
    assert!(report.receipt_replay_ready);
    assert!(report.receipt_records_match_projection);
    assert!(report.receipt_record_count > 0);
    assert_eq!(
        report.source_projection_replay_record_count,
        report.receipt_record_count
    );
    assert_eq!(report.local_receipt_ack_count, report.receipt_record_count);
    assert_eq!(report.ordered_sequence_count, report.receipt_record_count);
    assert_eq!(report.unique_event_id_count, report.receipt_record_count);
    assert_eq!(report.unique_correlation_id_count, 1);
    assert_eq!(
        report.exported_event_type_count,
        report.receipt_record_count
    );
    assert_eq!(report.projection_replay_mode, ReplayMode::ProjectionOnly);
    assert_eq!(
        report.replay_cursor_next_sequence,
        report.receipt_record_count as u64 + 1
    );
    for receipt in &report.receipts {
        assert_eq!(
            receipt.event_chain_journal_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF
        );
        assert_eq!(
            receipt.local_receipt_ack_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
        );
    }
}

#[tokio::test]
async fn network_runtime_remote_delivery_receipt_ledger_rejects_transport_action_and_content_claims(
) {
    let report = prove_network_runtime_remote_delivery_receipt_ledger()
        .await
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_RECEIPT_LEDGER);

    assert_eq!(
        report.local_receipt_ack_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
    );
    assert!(!report.broker_delivery_implemented);
    assert!(!report.family_hub_delivery_implemented);
    assert!(!report.remote_delivery_ack_implemented);
    assert!(!report.policy_authority);
    assert!(!report.side_effect_authority);
    assert_eq!(report.receipt_sequence_gap_count, 0);
    assert_eq!(report.receipt_event_id_mismatch_count, 0);
    assert_eq!(report.receipt_event_type_mismatch_count, 0);
    assert_eq!(report.receipt_correlation_mismatch_count, 0);
    assert_eq!(report.enforcement_command_event_count, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert_eq!(report.exact_url_available_count, 0);
    assert_eq!(report.decrypted_payload_available_count, 0);
    assert_eq!(report.page_content_available_count, 0);
}

#[tokio::test]
async fn network_runtime_remote_delivery_durable_envelope_preserves_receipt_refs_without_transport()
{
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
        NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    > = prove_network_runtime_remote_delivery_durable_envelope().await;
    let report = proof_result
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DURABLE_ENVELOPE);

    assert_eq!(
        report.durable_envelope_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF
    );
    assert_eq!(
        report.durable_store_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF
    );
    assert_eq!(
        report.durable_replay_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_REPLAY_REF
    );
    assert_eq!(
        report.delete_export_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF
    );
    assert_eq!(
        report.durable_support_status_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_SUPPORT_STATUS_REF
    );
    assert_eq!(
        report.receipt_ledger.receipt_ledger_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
    );
    assert_eq!(
        report.receipt_ledger.local_receipt_ack_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
    );
    assert!(report.durable_store_ready);
    assert!(report.durable_replay_ready);
    assert!(report.delete_export_readiness_recorded);
    assert!(report.durable_records_match_receipts);
    assert!(report.durable_envelope_count > 0);
    assert_eq!(
        report.source_receipt_record_count,
        report.durable_envelope_count
    );
    assert_eq!(
        report.durable_store_write_count,
        report.durable_envelope_count
    );
    assert_eq!(
        report.durable_replay_ready_count,
        report.durable_envelope_count
    );
    assert_eq!(
        report.delete_export_ready_count,
        report.durable_envelope_count
    );
    assert_eq!(report.ordered_sequence_count, report.durable_envelope_count);
    assert_eq!(report.unique_event_id_count, report.durable_envelope_count);
    assert_eq!(report.unique_correlation_id_count, 1);
    for durable_record in &report.durable_records {
        assert_eq!(
            durable_record.durable_envelope_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF
        );
        assert_eq!(
            durable_record.receipt_ledger_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
        );
        assert_eq!(
            durable_record.local_receipt_ack_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
        );
        assert_eq!(
            durable_record.delete_export_readiness_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF
        );
    }
}

#[tokio::test]
async fn network_runtime_remote_delivery_durable_envelope_rejects_delivery_action_and_content_claims(
) {
    let report = prove_network_runtime_remote_delivery_durable_envelope()
        .await
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DURABLE_ENVELOPE);

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
    assert_eq!(report.exact_url_available_count, 0);
    assert_eq!(report.decrypted_payload_available_count, 0);
    assert_eq!(report.page_content_available_count, 0);
}
