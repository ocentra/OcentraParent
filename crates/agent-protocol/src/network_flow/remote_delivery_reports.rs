use ocentra_eventing::{
    delivery::validation::EventDeliveryDecisionProof,
    delivery::validation::EventDeliveryRequiredArtifact,
    delivery::validation::EventDeliveryRouteKind,
    ids::{CorrelationId, EventId, EventType, SourceComponent},
    replay::ReplayMode,
};

use super::broker_delivery::NetworkRuntimeBrokerDeliverySemanticsReport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryState {
    FixtureRequirementsRecordedButNotImplemented,
    ManualRequired,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryStatusReport {
    pub broker_semantics: NetworkRuntimeBrokerDeliverySemanticsReport,
    pub broker_status: NetworkRuntimeRemoteDeliveryState,
    pub family_hub_status: NetworkRuntimeRemoteDeliveryState,
    pub family_hub_decision: EventDeliveryDecisionProof,
    pub custody_proof_ref: SourceComponent,
    pub publisher_auth_ref: SourceComponent,
    pub subscriber_auth_ref: SourceComponent,
    pub encryption_ref: SourceComponent,
    pub retention_policy_ref: SourceComponent,
    pub replay_plan_ref: SourceComponent,
    pub deletion_plan_ref: SourceComponent,
    pub offset_policy_ref: SourceComponent,
    pub dedupe_policy_ref: SourceComponent,
    pub transport_config_ref: SourceComponent,
    pub relay_identity_ref: SourceComponent,
    pub relay_policy_ref: SourceComponent,
    pub broker_missing_artifact_count: usize,
    pub family_hub_missing_artifact_count: usize,
    pub accepted_event_type_count: usize,
    pub local_idempotency_queue_proved: bool,
    pub dropped_event_dead_letter_count: usize,
    pub queued_duplicate_rejected: bool,
    pub completed_duplicate_rejected: bool,
    pub external_transport_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub cross_process_replay_implemented: bool,
    pub remote_retention_delete_export_propagation_implemented: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteEventChainJournalReport {
    pub remote_delivery_status: NetworkRuntimeRemoteDeliveryStatusReport,
    pub event_chain_journal_ref: SourceComponent,
    pub event_chain_replay_ref: SourceComponent,
    pub event_chain_export_ref: SourceComponent,
    pub event_chain_support_status_ref: SourceComponent,
    pub stored_event_count: usize,
    pub journal_entry_count: usize,
    pub projection_replay_record_count: usize,
    pub replay_cursor_next_sequence: u64,
    pub exported_event_type_count: usize,
    pub exportable_remote_envelope_count: usize,
    pub unavailable_event_count: usize,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub projection_replay_mode: ReplayMode,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryReceiptRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub event_chain_journal_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryReceiptLedgerReport {
    pub remote_delivery_status: NetworkRuntimeRemoteDeliveryStatusReport,
    pub event_chain_journal_ref: SourceComponent,
    pub event_chain_export_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
    pub receipt_replay_ref: SourceComponent,
    pub receipt_support_status_ref: SourceComponent,
    pub source_projection_replay_record_count: usize,
    pub receipt_record_count: usize,
    pub local_receipt_ack_count: usize,
    pub ordered_sequence_count: usize,
    pub unique_event_id_count: usize,
    pub unique_correlation_id_count: usize,
    pub exported_event_type_count: usize,
    pub replay_cursor_next_sequence: u64,
    pub projection_replay_mode: ReplayMode,
    pub receipt_ledger_ready: bool,
    pub receipt_replay_ready: bool,
    pub receipt_records_match_projection: bool,
    pub receipt_sequence_gap_count: usize,
    pub receipt_event_id_mismatch_count: usize,
    pub receipt_event_type_mismatch_count: usize,
    pub receipt_correlation_mismatch_count: usize,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub receipts: Vec<NetworkRuntimeRemoteDeliveryReceiptRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
    pub delete_export_readiness_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryDurableEnvelopeReport {
    pub receipt_ledger: NetworkRuntimeRemoteDeliveryReceiptLedgerReport,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub durable_replay_ref: SourceComponent,
    pub delete_export_readiness_ref: SourceComponent,
    pub durable_support_status_ref: SourceComponent,
    pub source_receipt_record_count: usize,
    pub durable_envelope_count: usize,
    pub durable_store_write_count: usize,
    pub durable_replay_ready_count: usize,
    pub delete_export_ready_count: usize,
    pub ordered_sequence_count: usize,
    pub unique_event_id_count: usize,
    pub unique_correlation_id_count: usize,
    pub durable_records_match_receipts: bool,
    pub durable_store_ready: bool,
    pub durable_replay_ready: bool,
    pub delete_export_readiness_recorded: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub durable_records: Vec<NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryOutboxState {
    PreparedNotDispatched,
    DispatchBlockedManualRequired,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryOutboxCandidate {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub state: NetworkRuntimeRemoteDeliveryOutboxState,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryOutboxHandoffReport {
    pub durable_envelope: NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub outbox_replay_ref: SourceComponent,
    pub outbox_support_status_ref: SourceComponent,
    pub source_durable_envelope_count: usize,
    pub source_receipt_record_count: usize,
    pub outbox_candidate_count: usize,
    pub prepared_not_dispatched_count: usize,
    pub dispatch_attempt_count: usize,
    pub remote_ack_count: usize,
    pub duplicate_durable_envelope_rejected: bool,
    pub outbox_candidates_match_durable_envelopes: bool,
    pub outbox_candidates_match_receipts: bool,
    pub sequence_gap_count: usize,
    pub event_id_mismatch_count: usize,
    pub event_type_mismatch_count: usize,
    pub correlation_mismatch_count: usize,
    pub unique_event_id_count: usize,
    pub unique_correlation_id_count: usize,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub candidates: Vec<NetworkRuntimeRemoteDeliveryOutboxCandidate>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryFixtureTransportState {
    FixtureAckRecorded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryFixtureTransportRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_outbox_state: NetworkRuntimeRemoteDeliveryOutboxState,
    pub fixture_state: NetworkRuntimeRemoteDeliveryFixtureTransportState,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub fixture_transport_ref: SourceComponent,
    pub fixture_dispatch_attempt_ref: SourceComponent,
    pub fixture_ack_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryFixtureTransportReport {
    pub outbox_handoff: NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    pub fixture_transport_ref: SourceComponent,
    pub fixture_dispatch_attempt_ref: SourceComponent,
    pub fixture_ack_ref: SourceComponent,
    pub source_outbox_candidate_count: usize,
    pub fixture_dispatch_attempt_count: usize,
    pub fixture_remote_ack_count: usize,
    pub fixture_records_match_outbox_candidates: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub records: Vec<NetworkRuntimeRemoteDeliveryFixtureTransportRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryProviderChildReadinessState {
    ManualRequiredUnavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub fixture_ack_ref: SourceComponent,
    pub provider_route_ref: SourceComponent,
    pub child_device_route_ref: SourceComponent,
    pub provider_readiness_ref: SourceComponent,
    pub child_device_readiness_ref: SourceComponent,
    pub provider_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub child_device_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryProviderChildReadinessReport {
    pub fixture_transport: NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    pub provider_route_ref: SourceComponent,
    pub child_device_route_ref: SourceComponent,
    pub provider_readiness_ref: SourceComponent,
    pub child_device_readiness_ref: SourceComponent,
    pub provider_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub child_device_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub source_fixture_ack_count: usize,
    pub provider_delivery_readiness_record_count: usize,
    pub child_device_delivery_readiness_record_count: usize,
    pub provider_delivery_artifact_count: usize,
    pub child_device_delivery_artifact_count: usize,
    pub provider_delivery_records_match_fixture_acks: bool,
    pub child_device_delivery_records_match_fixture_acks: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub records: Vec<NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState {
    ManualRequiredUnavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_provider_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub source_child_device_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub fixture_ack_ref: SourceComponent,
    pub provider_readiness_ref: SourceComponent,
    pub child_device_readiness_ref: SourceComponent,
    pub cross_process_custody_status_ref: SourceComponent,
    pub cross_process_replay_readiness_ref: SourceComponent,
    pub remote_retention_readiness_ref: SourceComponent,
    pub remote_delete_custody_readiness_ref: SourceComponent,
    pub remote_export_custody_readiness_ref: SourceComponent,
    pub custody_state: NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport {
    pub provider_child_readiness: NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    pub cross_process_custody_status_ref: SourceComponent,
    pub cross_process_replay_readiness_ref: SourceComponent,
    pub remote_retention_readiness_ref: SourceComponent,
    pub remote_delete_custody_readiness_ref: SourceComponent,
    pub remote_export_custody_readiness_ref: SourceComponent,
    pub custody_state: NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState,
    pub source_provider_child_readiness_record_count: usize,
    pub cross_process_replay_readiness_record_count: usize,
    pub remote_retention_readiness_record_count: usize,
    pub remote_delete_custody_readiness_record_count: usize,
    pub remote_export_custody_readiness_record_count: usize,
    pub cross_process_replay_artifact_count: usize,
    pub remote_retention_artifact_count: usize,
    pub remote_delete_custody_artifact_count: usize,
    pub remote_export_custody_artifact_count: usize,
    pub custody_records_match_provider_child_readiness: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub cross_process_replay_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub records: Vec<NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryCrossProcessReplayState {
    DurableReplayRecorded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub receipt_ledger_ref: SourceComponent,
    pub local_receipt_ack_ref: SourceComponent,
    pub cross_process_custody_status_ref: SourceComponent,
    pub cross_process_replay_readiness_ref: SourceComponent,
    pub cross_process_replay_ref: SourceComponent,
    pub cross_process_replay_store_ref: SourceComponent,
    pub cross_process_replay_cursor_ref: SourceComponent,
    pub replay_state: NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryCrossProcessReplayReport {
    pub cross_process_custody_readiness:
        NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    pub cross_process_replay_ref: SourceComponent,
    pub cross_process_replay_store_ref: SourceComponent,
    pub cross_process_replay_cursor_ref: SourceComponent,
    pub replay_state: NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
    pub source_durable_envelope_count: usize,
    pub source_custody_readiness_record_count: usize,
    pub cross_process_replay_record_count: usize,
    pub cross_process_replay_store_write_count: usize,
    pub cross_process_replay_cursor_next_sequence: u64,
    pub cross_process_replay_records_match_durable_envelopes: bool,
    pub cross_process_replay_records_match_custody_readiness: bool,
    pub cross_process_replay_implemented: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub records: Vec<NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState {
    DeterministicEnvelopeAckRecorded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_replay_state: NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
    pub durable_envelope_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub cross_process_replay_ref: SourceComponent,
    pub cross_process_replay_store_ref: SourceComponent,
    pub cross_process_replay_cursor_ref: SourceComponent,
    pub external_cross_process_transport_ref: SourceComponent,
    pub external_cross_process_transport_envelope_ref: SourceComponent,
    pub external_cross_process_transport_ack_ref: SourceComponent,
    pub transport_state: NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport {
    pub cross_process_replay: NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    pub external_cross_process_transport_ref: SourceComponent,
    pub external_cross_process_transport_envelope_ref: SourceComponent,
    pub external_cross_process_transport_ack_ref: SourceComponent,
    pub source_replay_record_count: usize,
    pub external_cross_process_transport_record_count: usize,
    pub external_cross_process_transport_envelope_count: usize,
    pub external_cross_process_transport_ack_count: usize,
    pub external_cross_process_transport_records_match_replay_records: bool,
    pub external_cross_process_transport_ack_records_match_envelopes: bool,
    pub external_cross_process_transport_implemented: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub records: Vec<NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryDeleteExportPropagationState {
    ReadinessRecordedNotPropagated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_fixture_state: NetworkRuntimeRemoteDeliveryFixtureTransportState,
    pub propagation_state: NetworkRuntimeRemoteDeliveryDeleteExportPropagationState,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub fixture_ack_ref: SourceComponent,
    pub delete_export_propagation_ref: SourceComponent,
    pub remote_delete_readiness_ref: SourceComponent,
    pub remote_export_readiness_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport {
    pub fixture_transport: NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    pub delete_export_propagation_ref: SourceComponent,
    pub remote_delete_readiness_ref: SourceComponent,
    pub remote_export_readiness_ref: SourceComponent,
    pub source_fixture_record_count: usize,
    pub propagation_readiness_record_count: usize,
    pub remote_delete_ready_count: usize,
    pub remote_export_ready_count: usize,
    pub propagation_records_match_fixture_records: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub records: Vec<NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryDispatchReadinessState {
    ManualRequiredTransportNotImplemented,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryDispatchGate {
    pub gate_ref: SourceComponent,
    pub route_kind: EventDeliveryRouteKind,
    pub required_artifacts: Vec<EventDeliveryRequiredArtifact>,
    pub required_artifact_count: usize,
    pub missing_artifact_count: usize,
    pub fixture_requirements_satisfied: bool,
    pub transport_implemented: bool,
    pub dispatch_ready: bool,
    pub manual_required: bool,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryDispatchReadinessReport {
    pub outbox_handoff: NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    pub dispatch_readiness_ref: SourceComponent,
    pub transport_requirements_ref: SourceComponent,
    pub broker_gate: NetworkRuntimeRemoteDeliveryDispatchGate,
    pub family_hub_gate: NetworkRuntimeRemoteDeliveryDispatchGate,
    pub state: NetworkRuntimeRemoteDeliveryDispatchReadinessState,
    pub source_outbox_candidate_count: usize,
    pub prepared_not_dispatched_count: usize,
    pub manual_required_candidate_count: usize,
    pub dispatch_ready_candidate_count: usize,
    pub dispatch_attempt_count: usize,
    pub remote_ack_count: usize,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState {
    AvailableMetadataNonEnforcing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryNoEnforcementStage {
    RemoteDeliveryStatus,
    EventChainJournal,
    ReceiptLedger,
    DurableEnvelope,
    OutboxHandoff,
    DispatchReadiness,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport {
    pub dispatch_readiness: NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
    pub invariant_ref: SourceComponent,
    pub available_metadata_ref: SourceComponent,
    pub state: NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState,
    pub stages: Vec<NetworkRuntimeRemoteDeliveryNoEnforcementStage>,
    pub remote_metadata_stage_count: usize,
    pub available_metadata_refs: Vec<SourceComponent>,
    pub available_metadata_ref_count: usize,
    pub manual_required_candidate_count: usize,
    pub dispatch_ready_candidate_count: usize,
    pub dispatch_attempt_count: usize,
    pub remote_ack_count: usize,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryTransportDispatchState {
    ManualRequiredBlocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryBlockedDispatchRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_outbox_state: NetworkRuntimeRemoteDeliveryOutboxState,
    pub blocked_state: NetworkRuntimeRemoteDeliveryTransportDispatchState,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub dispatch_state_ref: SourceComponent,
    pub blocked_dispatch_ref: SourceComponent,
    pub future_transport_seam_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryTransportDispatchStateReport {
    pub no_enforcement_invariant: NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    pub dispatch_state_ref: SourceComponent,
    pub blocked_dispatch_ref: SourceComponent,
    pub future_transport_seam_ref: SourceComponent,
    pub state: NetworkRuntimeRemoteDeliveryTransportDispatchState,
    pub source_outbox_candidate_count: usize,
    pub blocked_dispatch_record_count: usize,
    pub dispatch_ready_candidate_count: usize,
    pub dispatch_attempt_count: usize,
    pub remote_ack_count: usize,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
    pub blocked_dispatch_records: Vec<NetworkRuntimeRemoteDeliveryBlockedDispatchRecord>,
}
