use std::fmt::{Debug, Display};

use ocentra_eventing::{
    delivery::validation::EventDeliveryDecisionState,
    delivery::validation::EventDeliveryRequiredArtifact,
    delivery::validation::EventDeliveryRouteKind, replay::ReplayMode,
};
use ocentra_parent_agent_protocol::constants;

use crate::test_text::TestText;
use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_dispatch_readiness::prove_network_runtime_remote_delivery_dispatch_readiness,
    remote_delivery_dispatch_readiness_types::{
        NetworkRuntimeRemoteDeliveryDispatchReadinessError,
        NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
        NetworkRuntimeRemoteDeliveryDispatchReadinessState,
    },
    remote_delivery_durable_envelope::prove_network_runtime_remote_delivery_durable_envelope,
    remote_delivery_durable_envelope_types::{
        NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
        NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    },
    remote_delivery_event_chain_journal::prove_network_runtime_remote_event_chain_journal,
    remote_delivery_event_chain_journal_types::{
        NetworkRuntimeRemoteEventChainJournalError, NetworkRuntimeRemoteEventChainJournalReport,
    },
    remote_delivery_no_enforcement_invariant::{
        prove_network_runtime_remote_delivery_no_enforcement_invariant,
        prove_network_runtime_remote_delivery_no_enforcement_invariant_from_dispatch_readiness,
    },
    remote_delivery_no_enforcement_invariant_types::{
        NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
        NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
        NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState,
        NetworkRuntimeRemoteDeliveryNoEnforcementStage,
    },
    remote_delivery_outbox_handoff::prove_network_runtime_remote_delivery_outbox_handoff,
    remote_delivery_outbox_handoff_types::{
        NetworkRuntimeRemoteDeliveryOutboxHandoffError,
        NetworkRuntimeRemoteDeliveryOutboxHandoffReport, NetworkRuntimeRemoteDeliveryOutboxState,
    },
    remote_delivery_receipt_ledger::prove_network_runtime_remote_delivery_receipt_ledger,
    remote_delivery_receipt_ledger_types::{
        NetworkRuntimeRemoteDeliveryReceiptLedgerError,
        NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
    },
    remote_delivery_status::{
        prove_network_runtime_remote_delivery_status, NetworkRuntimeRemoteDeliveryState,
        NetworkRuntimeRemoteDeliveryStatusError, NetworkRuntimeRemoteDeliveryStatusReport,
    },
};

type TestResult = Result<(), TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
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
async fn network_runtime_remote_event_chain_journal_preserves_export_boundary_without_transport(
) -> TestResult {
    let proof_result: Result<
        NetworkRuntimeRemoteEventChainJournalReport,
        NetworkRuntimeRemoteEventChainJournalError,
    > = prove_network_runtime_remote_event_chain_journal().await;
    let report = ok(
        proof_result,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_EVENT_CHAIN_JOURNAL,
    )?;

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
    let owned_phase_count = ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase::ordered_chain()
        .iter()
        .filter(|phase| {
            matches!(
                phase,
                ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase::FlowObserved
                    | ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase::DomainObserved
                    | ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase::ActivityClassified
            )
        })
        .count();
    assert_eq!(report.stored_event_count, owned_phase_count);
    assert_eq!(report.exported_event_type_count, owned_phase_count);
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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_event_chain_journal_rejects_delivery_and_content_claims(
) -> TestResult {
    let report = ok(
        prove_network_runtime_remote_event_chain_journal().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_EVENT_CHAIN_JOURNAL,
    )?;

    assert_eq!(
        report.remote_delivery_status.broker_status,
        NetworkRuntimeRemoteDeliveryState::FixtureRequirementsRecordedButNotImplemented
    );
    assert!(!report.broker_delivery_implemented);
    assert!(!report.family_hub_delivery_implemented);
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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_no_enforcement_invariant_accepts_available_metadata(
) -> TestResult {
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
        NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
    > = prove_network_runtime_remote_delivery_no_enforcement_invariant().await;
    let report = ok(
        proof_result,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_NO_ENFORCEMENT_INVARIANT,
    )?;

    assert_eq!(
        report.invariant_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_NO_ENFORCEMENT_INVARIANT_REF
    );
    assert_eq!(
        report.available_metadata_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_AVAILABLE_METADATA_REF
    );
    assert_eq!(
        report.state,
        NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState::AvailableMetadataNonEnforcing
    );
    assert_eq!(report.remote_metadata_stage_count, 6);
    assert_eq!(report.stages.len(), report.remote_metadata_stage_count);
    assert!(report
        .stages
        .contains(&NetworkRuntimeRemoteDeliveryNoEnforcementStage::RemoteDeliveryStatus));
    assert!(report
        .stages
        .contains(&NetworkRuntimeRemoteDeliveryNoEnforcementStage::EventChainJournal));
    assert!(report
        .stages
        .contains(&NetworkRuntimeRemoteDeliveryNoEnforcementStage::ReceiptLedger));
    assert!(report
        .stages
        .contains(&NetworkRuntimeRemoteDeliveryNoEnforcementStage::DurableEnvelope));
    assert!(report
        .stages
        .contains(&NetworkRuntimeRemoteDeliveryNoEnforcementStage::OutboxHandoff));
    assert!(report
        .stages
        .contains(&NetworkRuntimeRemoteDeliveryNoEnforcementStage::DispatchReadiness));
    assert_eq!(
        report.available_metadata_ref_count,
        report.available_metadata_refs.len()
    );
    assert!(report.available_metadata_ref_count >= 31);
    assert_eq!(
        report.manual_required_candidate_count,
        report
            .dispatch_readiness
            .outbox_handoff
            .outbox_candidate_count
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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_no_enforcement_invariant_rejects_remote_action_claims(
) -> TestResult {
    let mut dispatch_readiness = ok(
        prove_network_runtime_remote_delivery_dispatch_readiness().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DISPATCH_READINESS,
    )?;
    dispatch_readiness.enforcement_command_event_count = 1;
    dispatch_readiness.dispatch_attempt_count = 1;

    let proof_result =
        prove_network_runtime_remote_delivery_no_enforcement_invariant_from_dispatch_readiness(
            dispatch_readiness,
        );

    assert!(matches!(
        proof_result,
        Err(NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError::UnsupportedClaim)
    ));

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_receipt_ledger_preserves_local_ack_boundary_without_transport(
) -> TestResult {
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
        NetworkRuntimeRemoteDeliveryReceiptLedgerError,
    > = prove_network_runtime_remote_delivery_receipt_ledger().await;
    let report = ok(
        proof_result,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_RECEIPT_LEDGER,
    )?;

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
        NetworkRuntimeRemoteDeliveryState::FixtureRequirementsRecordedButNotImplemented
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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_receipt_ledger_rejects_transport_action_and_content_claims(
) -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_receipt_ledger().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_RECEIPT_LEDGER,
    )?;

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
    assert_eq!(report.raw_pcap_available_count, 0);
    assert_eq!(report.exact_url_available_count, 0);
    assert_eq!(report.decrypted_payload_available_count, 0);
    assert_eq!(report.page_content_available_count, 0);
    assert_eq!(report.video_content_available_count, 0);
    assert_eq!(report.private_message_content_available_count, 0);
    assert_eq!(report.search_query_available_count, 0);

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_durable_envelope_preserves_receipt_refs_without_transport(
) -> TestResult {
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
        NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    > = prove_network_runtime_remote_delivery_durable_envelope().await;
    let report = ok(
        proof_result,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DURABLE_ENVELOPE,
    )?;

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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_durable_envelope_rejects_delivery_action_and_content_claims(
) -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_durable_envelope().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DURABLE_ENVELOPE,
    )?;

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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_outbox_handoff_preserves_durable_refs_without_dispatch(
) -> TestResult {
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
        NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    > = prove_network_runtime_remote_delivery_outbox_handoff().await;
    let report = ok(
        proof_result,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_HANDOFF,
    )?;

    assert_outbox_handoff_refs(&report);
    assert_outbox_handoff_counts(&report);
    assert_outbox_handoff_candidates(&report);

    Ok(())
}

fn assert_outbox_handoff_refs(report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport) {
    assert_eq!(
        report.durable_envelope_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF
    );
    assert_eq!(
        report.durable_store_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF
    );
    assert_eq!(
        report.outbox_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REF
    );
    assert_eq!(
        report.handoff_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF
    );
    assert_eq!(
        report.outbox_replay_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REPLAY_REF
    );
    assert_eq!(
        report.outbox_support_status_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_SUPPORT_STATUS_REF
    );
    assert_eq!(
        report.durable_envelope.durable_envelope_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF
    );
}

fn assert_outbox_handoff_counts(report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport) {
    assert_eq!(
        report.source_durable_envelope_count,
        report.outbox_candidate_count
    );
    assert_eq!(
        report.source_receipt_record_count,
        report.outbox_candidate_count
    );
    assert_eq!(
        report.prepared_not_dispatched_count,
        report.outbox_candidate_count
    );
    assert_eq!(report.dispatch_attempt_count, 0);
    assert_eq!(report.remote_ack_count, 0);
    assert!(report.duplicate_durable_envelope_rejected);
    assert!(report.outbox_candidates_match_durable_envelopes);
    assert!(report.outbox_candidates_match_receipts);
    assert_eq!(report.sequence_gap_count, 0);
    assert_eq!(report.event_id_mismatch_count, 0);
    assert_eq!(report.event_type_mismatch_count, 0);
    assert_eq!(report.correlation_mismatch_count, 0);
    assert_eq!(report.unique_event_id_count, report.outbox_candidate_count);
    assert_eq!(report.unique_correlation_id_count, 1);
}

fn assert_outbox_handoff_candidates(report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport) {
    for candidate in &report.candidates {
        assert_eq!(
            candidate.state,
            NetworkRuntimeRemoteDeliveryOutboxState::PreparedNotDispatched
        );
        assert_eq!(
            candidate.durable_envelope_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF
        );
        assert_eq!(
            candidate.durable_store_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF
        );
        assert_eq!(
            candidate.receipt_ledger_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
        );
        assert_eq!(
            candidate.local_receipt_ack_ref.as_str(),
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
        );
        assert_eq!(
            candidate.outbox_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REF
        );
        assert_eq!(
            candidate.handoff_ref.as_str(),
            constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF
        );
    }
}

#[tokio::test]
async fn network_runtime_remote_delivery_outbox_handoff_rejects_dispatch_ack_action_and_content_claims(
) -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_outbox_handoff().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_HANDOFF,
    )?;

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

    Ok(())
}

#[tokio::test]
async fn network_runtime_remote_delivery_dispatch_readiness_blocks_without_transport() -> TestResult
{
    let proof_result: Result<
        NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
        NetworkRuntimeRemoteDeliveryDispatchReadinessError,
    > = prove_network_runtime_remote_delivery_dispatch_readiness().await;
    let report = ok(
        proof_result,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DISPATCH_READINESS,
    )?;

    assert_eq!(
        report.dispatch_readiness_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_DISPATCH_READINESS_REF
    );
    assert_eq!(
        report.transport_requirements_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_TRANSPORT_REQUIREMENTS_REF
    );
    assert_eq!(
        report.state,
        NetworkRuntimeRemoteDeliveryDispatchReadinessState::ManualRequiredTransportNotImplemented
    );
    assert_eq!(
        report.source_outbox_candidate_count,
        report.prepared_not_dispatched_count
    );
    assert_eq!(
        report.outbox_handoff.outbox_candidate_count,
        report.source_outbox_candidate_count
    );
    assert_eq!(
        report.manual_required_candidate_count,
        report.source_outbox_candidate_count
    );
    assert_eq!(report.dispatch_ready_candidate_count, 0);
    assert_eq!(report.dispatch_attempt_count, 0);
    assert_eq!(report.remote_ack_count, 0);
    assert_dispatch_gate_blocks_transport(&report);

    Ok(())
}

fn assert_dispatch_gate_blocks_transport(
    report: &NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
) {
    assert_eq!(
        report.broker_gate.gate_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_BROKER_DISPATCH_GATE_REF
    );
    assert_eq!(
        report.family_hub_gate.gate_ref.as_str(),
        constants::network_flow::TEST_REMOTE_DELIVERY_FAMILY_HUB_DISPATCH_GATE_REF
    );
    assert_eq!(
        report.broker_gate.route_kind,
        EventDeliveryRouteKind::ExternalTransport
    );
    assert_eq!(
        report.family_hub_gate.route_kind,
        EventDeliveryRouteKind::ExternalRelay
    );
    assert!(report.broker_gate.fixture_requirements_satisfied);
    assert!(report.family_hub_gate.fixture_requirements_satisfied);
    assert!(report.broker_gate.required_artifact_count > 0);
    assert!(report.family_hub_gate.required_artifact_count > 0);
    assert_eq!(
        report.broker_gate.required_artifacts.len(),
        report.broker_gate.required_artifact_count
    );
    assert_eq!(
        report.family_hub_gate.required_artifacts.len(),
        report.family_hub_gate.required_artifact_count
    );
    assert_eq!(report.broker_gate.missing_artifact_count, 0);
    assert_eq!(report.family_hub_gate.missing_artifact_count, 0);
    assert!(!report.broker_gate.transport_implemented);
    assert!(!report.family_hub_gate.transport_implemented);
    assert!(!report.broker_gate.dispatch_ready);
    assert!(!report.family_hub_gate.dispatch_ready);
    assert!(report.broker_gate.manual_required);
    assert!(report.family_hub_gate.manual_required);
}

#[tokio::test]
async fn network_runtime_remote_delivery_dispatch_readiness_rejects_authority_and_content_claims(
) -> TestResult {
    let report = ok(
        prove_network_runtime_remote_delivery_dispatch_readiness().await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_DISPATCH_READINESS,
    )?;

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

    Ok(())
}
