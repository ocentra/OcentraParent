use ocentra_eventing::{
    EventDeliveryDecisionState, EventDeliveryRequiredArtifact, EventDeliveryRouteKind, ReplayMode,
};
use ocentra_parent_agent_protocol::constants;

use crate::network_event_runtime::{
    prove_network_runtime_remote_delivery_outbox_handoff,
    prove_network_runtime_remote_delivery_receipt_ledger,
    prove_network_runtime_remote_delivery_status, prove_network_runtime_remote_event_chain_journal,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport, NetworkRuntimeRemoteDeliveryOutboxState,
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

    assert_remote_delivery_route_state(&report);
    assert_remote_delivery_refs(&report);
}

fn assert_remote_delivery_route_state(report: &NetworkRuntimeRemoteDeliveryStatusReport) {
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
}

fn assert_remote_delivery_refs(report: &NetworkRuntimeRemoteDeliveryStatusReport) {
    assert_broker_delivery_refs(report);
    assert_remote_lifecycle_refs(report);
    assert_durable_envelope_refs(report);
}

fn assert_broker_delivery_refs(report: &NetworkRuntimeRemoteDeliveryStatusReport) {
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

fn assert_remote_lifecycle_refs(report: &NetworkRuntimeRemoteDeliveryStatusReport) {
    assert_eq!(
        report.cross_process_replay_ref.as_str(),
        constants::network_flow::TEST_REMOTE_LIFECYCLE_CROSS_PROCESS_REPLAY_REF
    );
    assert_eq!(
        report.remote_retention_delete_export_ref.as_str(),
        constants::network_flow::TEST_REMOTE_LIFECYCLE_RETENTION_DELETE_EXPORT_REF
    );
    assert_eq!(
        report.remote_delivery_ack_ref.as_str(),
        constants::network_flow::TEST_REMOTE_LIFECYCLE_DELIVERY_ACK_REF
    );
    assert_eq!(
        report.remote_lifecycle_followup_ref.as_str(),
        constants::network_flow::TEST_REMOTE_LIFECYCLE_FOLLOWUP_REF
    );
}

fn assert_durable_envelope_refs(report: &NetworkRuntimeRemoteDeliveryStatusReport) {
    assert_eq!(
        report.durable_envelope_schema_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_SCHEMA_REF
    );
    assert_eq!(
        report.durable_envelope_journal_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_JOURNAL_REF
    );
    assert_eq!(
        report.durable_envelope_replay_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_REPLAY_REF
    );
    assert_eq!(
        report.durable_envelope_delete_export_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_DELETE_EXPORT_REF
    );
    assert_eq!(
        report.durable_envelope_support_status_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_SUPPORT_STATUS_REF
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
    assert!(report.remote_lifecycle_manual_required);
    assert_eq!(report.remote_lifecycle_missing_artifact_count, 3);
    assert!(report.durable_envelope_ready);
    assert_eq!(report.durable_envelope_missing_artifact_count, 0);
    assert!(!report.policy_authority);
    assert!(!report.side_effect_authority);
    assert_eq!(report.enforcement_command_event_count, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert!(!report.provider_delivery_implemented);
    assert!(!report.child_device_delivery_implemented);
    assert!(!report.product_ready_claimed);
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
    assert!(report.durable_envelope_ready);
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
    assert!(!report.provider_delivery_implemented);
    assert!(!report.child_device_delivery_implemented);
    assert!(!report.product_ready_claimed);
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
        report
            .remote_delivery_status
            .remote_delivery_ack_ref
            .as_str(),
        constants::network_flow::TEST_REMOTE_LIFECYCLE_DELIVERY_ACK_REF
    );
    assert!(!report.broker_delivery_implemented);
    assert!(!report.family_hub_delivery_implemented);
    assert!(!report.provider_delivery_implemented);
    assert!(!report.child_device_delivery_implemented);
    assert!(!report.remote_delivery_ack_implemented);
    assert!(!report.product_ready_claimed);
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
async fn network_runtime_remote_delivery_outbox_handoff_preserves_projection_and_receipt_refs_without_dispatch(
) {
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
        NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    > = prove_network_runtime_remote_delivery_outbox_handoff().await;
    let report =
        proof_result.expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_HANDOFF);

    assert_eq!(
        report.event_chain_export_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF
    );
    assert_eq!(
        report.receipt_ledger_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
    );
    assert_eq!(
        report.outbox_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_REF
    );
    assert_eq!(
        report.handoff_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_HANDOFF_REF
    );
    assert_eq!(
        report.outbox_replay_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_REPLAY_REF
    );
    assert_eq!(
        report.outbox_support_status_ref.as_str(),
        constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_SUPPORT_STATUS_REF
    );
    assert!(report.outbox_candidates_match_projection);
    assert!(report.outbox_candidates_match_receipts);
    assert!(report.outbox_candidate_count > 0);
    assert_eq!(
        report.source_projection_replay_record_count,
        report.outbox_candidate_count
    );
    assert_eq!(report.receipt_record_count, report.outbox_candidate_count);
    assert_eq!(
        report.prepared_not_dispatched_count,
        report.outbox_candidate_count
    );
    assert_eq!(report.unique_event_id_count, report.outbox_candidate_count);
    assert_eq!(
        report.unique_idempotency_key_count,
        report.outbox_candidate_count
    );
    assert_eq!(report.target_handler_count, report.outbox_candidate_count);
    assert_eq!(report.receipt_ref_count, report.outbox_candidate_count);
    assert_eq!(report.broker_requirement_ref_count, 12);
    assert_eq!(report.lifecycle_blocker_ref_count, 4);
    assert_eq!(report.durable_envelope_ref_count, 5);
    assert_eq!(report.projection_replay_mode, ReplayMode::ProjectionOnly);
    assert!(report.remote_delivery_status.durable_envelope_ready);
    for candidate in &report.candidates {
        assert_eq!(
            candidate.state,
            NetworkRuntimeRemoteDeliveryOutboxState::PreparedNotDispatched
        );
        assert_eq!(
            candidate.event_chain_export_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF
        );
        assert_eq!(
            candidate.receipt_ledger_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
        );
        assert_eq!(
            candidate.outbox_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_REF
        );
        assert_eq!(
            candidate.handoff_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_HANDOFF_REF
        );
    }
}

#[tokio::test]
async fn network_runtime_remote_delivery_outbox_handoff_rejects_dispatch_ack_action_and_content_claims(
) {
    let report = prove_network_runtime_remote_delivery_outbox_handoff()
        .await
        .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_HANDOFF);

    assert_eq!(report.dispatch_attempt_count, 0);
    assert_eq!(report.remote_ack_count, 0);
    assert!(!report.broker_delivery_implemented);
    assert!(!report.family_hub_delivery_implemented);
    assert!(!report.provider_delivery_implemented);
    assert!(!report.child_device_delivery_implemented);
    assert!(!report.remote_delivery_ack_implemented);
    assert!(!report.product_ready_claimed);
    assert!(!report.policy_authority);
    assert!(!report.side_effect_authority);
    assert_eq!(report.sequence_gap_count, 0);
    assert_eq!(report.event_id_mismatch_count, 0);
    assert_eq!(report.event_type_mismatch_count, 0);
    assert_eq!(report.correlation_mismatch_count, 0);
    assert_eq!(report.enforcement_command_event_count, 0);
    assert_eq!(report.adapter_action_executed_count, 0);
    assert_eq!(report.exact_url_available_count, 0);
    assert_eq!(report.decrypted_payload_available_count, 0);
    assert_eq!(report.page_content_available_count, 0);
}
