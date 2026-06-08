use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str = "child-device-query-store";
pub const NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT: &str = "parent-owned-export";
pub const NETWORK_FLOW_CUSTODY_UNAVAILABLE: &str = "unavailable";
pub const NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS: &str = "activeRows";
pub const NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS: &str = "tombstoneRows";
pub const NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS: &str = "exportableRows";
pub const NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY: &str = "exportCustody";
pub const NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID: &str = "latestTombstoneEventId";
pub const NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT: &str =
    "latestTombstoneObservedAt";
pub const NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS: &str =
    "deletedEvidenceReferenceIds";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkEndpoint {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowCounters {
    pub connection_count: u64,
    pub bytes_sent: Option<u64>,
    pub bytes_received: Option<u64>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowObservation {
    pub schema_version: u16,
    pub event_id: String,
    pub observed_at: String,
    pub observer: String,
    pub capability_status: String,
    pub adapter_id: String,
    pub protocol: Option<String>,
    pub tcp_state: Option<String>,
    pub local_endpoint: ActivityNetworkEndpoint,
    pub destination_endpoint: ActivityNetworkEndpoint,
    pub destination_domain: Option<String>,
    pub domain_attribution_status: String,
    pub process_attribution_status: String,
    pub process_id: Option<u64>,
    pub process_name: Option<String>,
    pub counters: ActivityNetworkFlowCounters,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody: String,
    pub limit: u64,
    pub returned: u64,
    pub active_rows: u64,
    pub tombstone_rows: u64,
    pub exportable_rows: u64,
    pub capability_status: String,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub latest_tombstone_event_id: Option<String>,
    pub latest_tombstone_observed_at: Option<String>,
    pub deleted_evidence_reference_ids: Vec<String>,
    pub rows: Vec<ActivityNetworkFlowObservation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowRollup {
    pub key: String,
    pub label: String,
    pub connection_count: u64,
    pub bytes_sent: Option<u64>,
    pub bytes_received: Option<u64>,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowIndicator {
    pub kind: String,
    pub label: String,
    pub observed_at: String,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowDigest {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody: String,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub top_processes: Vec<ActivityNetworkFlowRollup>,
    pub top_destinations: Vec<ActivityNetworkFlowRollup>,
    pub unusual_indicators: Vec<ActivityNetworkFlowIndicator>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRemoteDeliveryStatusState {
    #[default]
    #[serde(rename = "fixture-requirements-recorded-but-not-implemented")]
    FixtureRequirementsRecordedButNotImplemented,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRemoteDeliveryTransportDispatchState {
    #[default]
    #[serde(rename = "manual-required-blocked")]
    ManualRequiredBlocked,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRemoteDeliveryProviderChildReadinessState {
    #[default]
    #[serde(rename = "manual-required-unavailable")]
    ManualRequiredUnavailable,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkRemoteDeliveryStatus {
    pub status_ref: String,
    pub broker_status: NetworkRemoteDeliveryStatusState,
    pub family_hub_status: NetworkRemoteDeliveryStatusState,
    pub custody_proof_ref: String,
    pub publisher_auth_ref: String,
    pub subscriber_auth_ref: String,
    pub encryption_ref: String,
    pub retention_policy_ref: String,
    pub replay_plan_ref: String,
    pub deletion_plan_ref: String,
    pub offset_policy_ref: String,
    pub dedupe_policy_ref: String,
    pub transport_config_ref: String,
    pub relay_identity_ref: String,
    pub relay_policy_ref: String,
    pub broker_missing_artifact_count: u64,
    pub family_hub_missing_artifact_count: u64,
    pub accepted_event_type_count: u64,
    pub local_idempotency_queue_proved: bool,
    pub dropped_event_dead_letter_count: u64,
    pub queued_duplicate_rejected: bool,
    pub completed_duplicate_rejected: bool,
    pub event_chain_journal_ref: String,
    pub receipt_ledger_ref: String,
    pub local_receipt_ack_ref: String,
    pub durable_envelope_ref: String,
    pub durable_store_ref: String,
    pub durable_replay_ref: String,
    pub durable_delete_export_ref: String,
    pub durable_support_status_ref: String,
    pub durable_envelope_ready: bool,
    pub durable_envelope_missing_artifact_count: u64,
    pub outbox_ref: String,
    pub outbox_handoff_ref: String,
    pub outbox_replay_ref: String,
    pub outbox_support_status_ref: String,
    pub transport_dispatch_state_ref: String,
    pub blocked_dispatch_ref: String,
    pub future_transport_seam_ref: String,
    pub fixture_transport_ref: String,
    pub fixture_dispatch_attempt_ref: String,
    pub fixture_ack_ref: String,
    pub delete_export_propagation_ref: String,
    pub remote_delete_readiness_ref: String,
    pub remote_export_readiness_ref: String,
    pub provider_route_ref: String,
    pub child_device_route_ref: String,
    pub provider_delivery_readiness_ref: String,
    pub child_device_delivery_readiness_ref: String,
    pub transport_dispatch_state: NetworkRemoteDeliveryTransportDispatchState,
    pub provider_delivery_readiness_state: NetworkRemoteDeliveryProviderChildReadinessState,
    pub child_device_delivery_readiness_state: NetworkRemoteDeliveryProviderChildReadinessState,
    pub outbox_candidate_count: u64,
    pub source_outbox_candidate_count: u64,
    pub prepared_not_dispatched_count: u64,
    pub blocked_dispatch_record_count: u64,
    pub blocked_dispatch_records_match_outbox_candidates: bool,
    pub fixture_source_outbox_candidate_count: u64,
    pub fixture_dispatch_attempt_count: u64,
    pub fixture_remote_ack_count: u64,
    pub fixture_records_match_outbox_candidates: bool,
    pub delete_export_readiness_record_count: u64,
    pub remote_delete_ready_count: u64,
    pub remote_export_ready_count: u64,
    pub delete_export_records_match_fixture_acks: bool,
    pub provider_delivery_readiness_record_count: u64,
    pub child_device_delivery_readiness_record_count: u64,
    pub provider_delivery_artifact_count: u64,
    pub child_device_delivery_artifact_count: u64,
    pub provider_delivery_records_match_fixture_acks: bool,
    pub child_device_delivery_records_match_fixture_acks: bool,
    pub dispatch_ready_candidate_count: u64,
    pub dispatch_attempt_count: u64,
    pub remote_ack_count: u64,
    pub duplicate_durable_envelope_rejected: bool,
    pub outbox_candidates_match_durable_envelopes: bool,
    pub outbox_candidates_match_receipts: bool,
    pub sequence_gap_count: u64,
    pub event_id_mismatch_count: u64,
    pub event_type_mismatch_count: u64,
    pub correlation_mismatch_count: u64,
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
    pub enforcement_command_event_count: u64,
    pub adapter_action_executed_count: u64,
    pub raw_pcap_available_count: u64,
    pub exact_url_available_count: u64,
    pub decrypted_payload_available_count: u64,
    pub page_content_available_count: u64,
    pub video_content_available_count: u64,
    pub private_message_content_available_count: u64,
    pub search_query_available_count: u64,
}

#[path = "network_flow_events.rs"]
mod network_flow_events;

pub use network_flow_events::*;
