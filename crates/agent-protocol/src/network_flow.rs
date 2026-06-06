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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRemoteDeliveryStatusState {
    RequirementsSatisfiedButNotImplemented,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub external_transport_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub cross_process_replay_implemented: bool,
    pub remote_retention_delete_export_propagation_implemented: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub enforcement_command_event_count: u64,
    pub adapter_action_executed_count: u64,
}

#[path = "network_flow_events.rs"]
mod network_flow_events;

pub use network_flow_events::*;
