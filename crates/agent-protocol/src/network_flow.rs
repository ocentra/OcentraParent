use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RuntimeRole, SchemaVersion};
use serde::{Deserialize, Serialize};

use crate::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityEvidenceRef, ActivityNetworkProtocol, ActivityNetworkTcpState,
    ActivityProcessAttributionStatus,
};

pub mod broker_delivery;
pub mod remote_delivery_reports;
pub mod review;

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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRemoteDeliveryCrossProcessCustodyReadinessState {
    #[default]
    #[serde(rename = "manual-required-unavailable")]
    ManualRequiredUnavailable,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRemoteDeliveryExternalCrossProcessTransportState {
    #[default]
    #[serde(rename = "deterministic-envelope-ack-recorded")]
    DeterministicEnvelopeAckRecorded,
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
    pub cross_process_custody_status_ref: String,
    pub cross_process_replay_readiness_ref: String,
    pub remote_retention_readiness_ref: String,
    pub remote_delete_custody_readiness_ref: String,
    pub remote_export_custody_readiness_ref: String,
    pub cross_process_replay_ref: String,
    pub cross_process_replay_store_ref: String,
    pub cross_process_replay_cursor_ref: String,
    pub external_cross_process_transport_ref: String,
    pub external_cross_process_transport_envelope_ref: String,
    pub external_cross_process_transport_ack_ref: String,
    pub transport_dispatch_state: NetworkRemoteDeliveryTransportDispatchState,
    pub provider_delivery_readiness_state: NetworkRemoteDeliveryProviderChildReadinessState,
    pub child_device_delivery_readiness_state: NetworkRemoteDeliveryProviderChildReadinessState,
    pub cross_process_custody_readiness_state:
        NetworkRemoteDeliveryCrossProcessCustodyReadinessState,
    pub external_cross_process_transport_state:
        NetworkRemoteDeliveryExternalCrossProcessTransportState,
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
    pub cross_process_replay_readiness_record_count: u64,
    pub remote_retention_readiness_record_count: u64,
    pub remote_delete_custody_readiness_record_count: u64,
    pub remote_export_custody_readiness_record_count: u64,
    pub cross_process_custody_records_match_provider_child_readiness: bool,
    pub cross_process_replay_artifact_count: u64,
    pub remote_retention_artifact_count: u64,
    pub remote_delete_custody_artifact_count: u64,
    pub remote_export_custody_artifact_count: u64,
    pub cross_process_replay_record_count: u64,
    pub cross_process_replay_store_write_count: u64,
    pub cross_process_replay_cursor_next_sequence: u64,
    pub cross_process_replay_records_match_durable_envelopes: bool,
    pub cross_process_replay_records_match_custody_readiness: bool,
    pub external_cross_process_transport_record_count: u64,
    pub external_cross_process_transport_envelope_count: u64,
    pub external_cross_process_transport_ack_count: u64,
    pub external_cross_process_transport_records_match_replay_records: bool,
    pub external_cross_process_transport_ack_records_match_envelopes: bool,
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
    pub external_cross_process_transport_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
    pub host_filtering_claimed: bool,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCaptureStatusPlatform {
    #[default]
    #[serde(rename = "windows-npcap")]
    WindowsNpcap,
    #[serde(rename = "linux-libpcap")]
    LinuxLibpcap,
    #[serde(rename = "macos-bpf-libpcap")]
    MacosBpfLibpcap,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCaptureProofStatusState {
    #[default]
    #[serde(rename = "proof-ready")]
    ProofReady,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "degraded")]
    Degraded,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRawCaptureStorageStatusState {
    #[default]
    #[serde(rename = "custody-ready")]
    CustodyReady,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "degraded")]
    Degraded,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCaptureExecutionStatusState {
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "bounded-executed")]
    BoundedExecuted,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "degraded")]
    Degraded,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkLiveCaptureStatusRow {
    pub platform: NetworkLiveCaptureStatusPlatform,
    pub capture_proof_ref: String,
    pub proof_state: NetworkLiveCaptureProofStatusState,
    pub storage_proof_ref: String,
    pub storage_state: NetworkRawCaptureStorageStatusState,
    pub interface_ref: Option<String>,
    pub driver_proof_ref: Option<String>,
    pub permission_proof_ref: Option<String>,
    pub bounded_capture_ref: Option<String>,
    pub clean_stop_ref: Option<String>,
    pub quota_rotation_ref: Option<String>,
    pub retention_delete_export_ref: Option<String>,
    pub custody_ref: Option<String>,
    pub private_traffic_exclusion_ref: Option<String>,
    pub raw_artifact_manifest_ref: Option<String>,
    pub storage_location_ref: Option<String>,
    pub encryption_at_rest_ref: Option<String>,
    pub storage_quota_rotation_ref: Option<String>,
    pub retention_policy_ref: Option<String>,
    pub storage_delete_export_ref: Option<String>,
    pub custody_chain_ref: Option<String>,
    pub storage_private_traffic_exclusion_ref: Option<String>,
    pub execution_ref: Option<String>,
    pub execution_state: NetworkLiveCaptureExecutionStatusState,
    pub execution_missing_artifact_count: u64,
    pub driver_invocation_ref: Option<String>,
    pub interface_observation_ref: Option<String>,
    pub execution_permission_ref: Option<String>,
    pub bounded_window_ref: Option<String>,
    pub execution_clean_stop_ref: Option<String>,
    pub execution_custody_ref: Option<String>,
    pub execution_retention_delete_export_ref: Option<String>,
    pub metadata_only_sanitization_ref: Option<String>,
    pub execution_private_traffic_exclusion_ref: Option<String>,
    pub metadata_snapshot_executed: bool,
    pub captured_packet_count: u64,
    pub raw_artifact_created: bool,
    pub missing_artifact_count: u64,
    pub storage_missing_artifact_count: u64,
    pub capture_ready: bool,
    pub raw_artifact_storage_authorized: bool,
    pub driver_invoked: bool,
    pub live_capture_executed: bool,
    pub remote_upload_enabled: bool,
    pub raw_pcap_without_custody_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: u64,
    pub netstat_metadata_substituted_for_live_capture: bool,
    pub host_filtering_claimed: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkLiveCaptureStatus {
    pub status_ref: String,
    pub row13_status_ref: String,
    pub execution_status_ref: String,
    pub raw_storage_status_ref: String,
    pub platform_row_count: u64,
    pub proof_ready_count: u64,
    pub manual_required_count: u64,
    pub unavailable_count: u64,
    pub degraded_count: u64,
    pub required_artifact_count: u64,
    pub missing_artifact_count: u64,
    pub storage_custody_ready_count: u64,
    pub storage_manual_required_count: u64,
    pub storage_unavailable_count: u64,
    pub storage_degraded_count: u64,
    pub storage_missing_artifact_count: u64,
    pub bounded_executed_count: u64,
    pub execution_manual_required_count: u64,
    pub execution_unavailable_count: u64,
    pub execution_degraded_count: u64,
    pub execution_missing_artifact_count: u64,
    pub metadata_snapshot_executed_count: u64,
    pub captured_packet_count: u64,
    pub raw_artifact_created_count: u64,
    pub capture_ready_count: u64,
    pub raw_artifact_storage_authorized_count: u64,
    pub driver_invoked_count: u64,
    pub live_capture_executed_count: u64,
    pub remote_upload_enabled_count: u64,
    pub raw_pcap_without_custody_available_count: u64,
    pub exact_url_available_count: u64,
    pub decrypted_payload_available_count: u64,
    pub page_content_available_count: u64,
    pub private_message_available_count: u64,
    pub search_query_available_count: u64,
    pub policy_authority_count: u64,
    pub adapter_authority_count: u64,
    pub enforcement_command_event_count: u64,
    pub netstat_metadata_substitution_count: u64,
    pub host_filtering_claim_count: u64,
    pub rows: Vec<NetworkLiveCaptureStatusRow>,
}

pub trait NetworkRuntimeEventContract {
    const EVENT_TYPE: &'static str;
    const SCHEMA_VERSION: u16 = crate::constants::network_flow::EVENT_SCHEMA_VERSION;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkClaimBoundary {
    pub exact_url_available: bool,
    pub decrypted_https_payload_available: bool,
    pub message_content_available: bool,
    pub search_query_available: bool,
    pub adapter_action_executed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkFlowObservedEvent {
    pub schema_version: u16,
    pub flow_event_ref: String,
    pub observed_at: String,
    pub device_ref: String,
    pub flow_evidence_ref: String,
    pub custody: String,
    pub evidence_grade: NetworkEvidenceGrade,
    pub claim_boundary: NetworkClaimBoundary,
}

impl NetworkRuntimeEventContract for NetworkFlowObservedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDomainObservedEvent {
    pub schema_version: u16,
    pub domain_event_ref: String,
    pub previous_event_ref: String,
    pub flow_evidence_ref: String,
    pub domain_evidence_ref: String,
    pub attribution: NetworkDomainAttributionKind,
    pub evidence_grade: NetworkEvidenceGrade,
    pub uncertainty_codes: Vec<String>,
    pub claim_boundary: NetworkClaimBoundary,
}

impl NetworkRuntimeEventContract for NetworkDomainObservedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkActivityClassifiedEvent {
    pub schema_version: u16,
    pub classification_event_ref: String,
    pub previous_event_ref: String,
    pub evidence_refs: Vec<String>,
    pub activity_kind: NetworkActivityKind,
    pub confidence: f32,
    pub evidence_grade: NetworkEvidenceGrade,
    pub uncertainty_codes: Vec<String>,
}

impl NetworkRuntimeEventContract for NetworkActivityClassifiedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAiAnalysisRequestedEvent {
    pub schema_version: u16,
    pub ai_request_ref: String,
    pub previous_event_ref: String,
    pub evidence_refs: Vec<String>,
    pub prompt_template_ref: String,
    pub custody: String,
    pub raw_packet_payload_included: bool,
}

impl NetworkRuntimeEventContract for NetworkAiAnalysisRequestedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAiAnalysisCompletedEvent {
    pub schema_version: u16,
    pub ai_analysis_ref: String,
    pub ai_request_ref: String,
    pub previous_event_ref: String,
    pub advisory_state: NetworkAiAdvisoryState,
    pub evidence_refs: Vec<String>,
    pub unsupported_claims: Vec<String>,
}

impl NetworkRuntimeEventContract for NetworkAiAnalysisCompletedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyEvaluationRequestedEvent {
    pub schema_version: u16,
    pub policy_evaluation_ref: String,
    pub previous_event_ref: String,
    pub evidence_refs: Vec<String>,
    pub ai_analysis_ref: Option<String>,
    pub parent_rule_refs: Vec<String>,
    pub dry_run: bool,
}

impl NetworkRuntimeEventContract for NetworkPolicyEvaluationRequestedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyDecisionCompletedEvent {
    pub schema_version: u16,
    pub policy_decision_ref: String,
    pub policy_evaluation_ref: String,
    pub previous_event_ref: String,
    pub decision_action: NetworkPolicyDecisionAction,
    pub evidence_refs: Vec<String>,
    pub parent_rule_refs: Vec<String>,
    pub adapter_capability_required: bool,
}

impl NetworkRuntimeEventContract for NetworkPolicyDecisionCompletedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_POLICY_DECISION_COMPLETED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkEnforcementCommandIssuedEvent {
    pub schema_version: u16,
    pub enforcement_command_ref: String,
    pub previous_event_ref: String,
    pub policy_decision_ref: String,
    pub adapter_capability_ref: String,
    pub enforcement_mode: NetworkEnforcementMode,
    pub evidence_refs: Vec<String>,
    pub rollback_ref: Option<String>,
}

impl NetworkRuntimeEventContract for NetworkEnforcementCommandIssuedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkEnforcementResultObservedEvent {
    pub schema_version: u16,
    pub enforcement_result_ref: String,
    pub enforcement_command_ref: String,
    pub previous_event_ref: String,
    pub result_status: NetworkEnforcementResultStatus,
    pub adapter_action_executed: bool,
    pub rollback_ref: Option<String>,
    pub unavailable_reason_code: Option<String>,
}

impl NetworkRuntimeEventContract for NetworkEnforcementResultObservedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAuditEntryCommittedEvent {
    pub schema_version: u16,
    pub audit_entry_ref: String,
    pub previous_event_ref: String,
    pub policy_decision_ref: String,
    pub enforcement_command_ref: Option<String>,
    pub enforcement_result_ref: Option<String>,
    pub evidence_refs: Vec<String>,
    pub audit_outcome: NetworkAuditOutcome,
}

impl NetworkRuntimeEventContract for NetworkAuditEntryCommittedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPortalReadModelUpdatedEvent {
    pub schema_version: u16,
    pub read_model_ref: String,
    pub previous_event_ref: String,
    pub audit_entry_ref: String,
    pub update_kind: NetworkPortalUpdateKind,
    pub visible_manual_required: bool,
    pub visible_unavailable: bool,
}

impl NetworkRuntimeEventContract for NetworkPortalReadModelUpdatedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvidenceGrade {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "D")]
    D,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkDomainAttributionKind {
    DnsAnswer,
    SniVisible,
    HttpHost,
    ReverseLookup,
    IpOnly,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkActivityKind {
    SocialCandidate,
    VideoCandidate,
    GameCandidate,
    VpnProxyTunnelCandidate,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkAiAdvisoryState {
    Requested,
    Completed,
    ManualReviewRequired,
    ProviderUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkPolicyDecisionAction {
    Observe,
    Warn,
    AskParent,
    Limit,
    Block,
    ManualReview,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkEnforcementMode {
    DryRun,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkEnforcementResultStatus {
    DryRun,
    ManualRequired,
    Unavailable,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkAuditOutcome {
    Committed,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkPortalUpdateKind {
    NetworkReadModel,
    CapabilityState,
    ManualRequiredState,
}

const NETWORK_RUNTIME_PHASES: [NetworkRuntimePhase; 11] = [
    NetworkRuntimePhase::FlowObserved,
    NetworkRuntimePhase::DomainObserved,
    NetworkRuntimePhase::ActivityClassified,
    NetworkRuntimePhase::AiAnalysisRequested,
    NetworkRuntimePhase::AiAnalysisCompleted,
    NetworkRuntimePhase::PolicyEvaluationRequested,
    NetworkRuntimePhase::PolicyDecisionCompleted,
    NetworkRuntimePhase::EnforcementCommandIssued,
    NetworkRuntimePhase::EnforcementResultObserved,
    NetworkRuntimePhase::AuditEntryCommitted,
    NetworkRuntimePhase::PortalReadModelUpdated,
];

const NETWORK_RUNTIME_PHASE_EVENT_TYPES: [&str; 11] = [
    constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
    constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED,
    constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED,
    constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED,
    constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED,
    constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED,
    constants::network_flow::EVENT_POLICY_DECISION_COMPLETED,
    constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED,
    constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED,
    constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED,
    constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED,
];

const NETWORK_RUNTIME_PHASE_SUBSCRIBER_IDS: [&str; 11] = [
    constants::network_flow::SUBSCRIBER_NETWORK_OBSERVER,
    constants::network_flow::SUBSCRIBER_DOMAIN_OBSERVER,
    constants::network_flow::SUBSCRIBER_ACTIVITY_CLASSIFIER,
    constants::network_flow::SUBSCRIBER_AI_REQUEST,
    constants::network_flow::SUBSCRIBER_AI_COMPLETE,
    constants::network_flow::SUBSCRIBER_POLICY_REQUEST,
    constants::network_flow::SUBSCRIBER_POLICY_DECISION,
    constants::network_flow::SUBSCRIBER_ENFORCEMENT_COMMAND,
    constants::network_flow::SUBSCRIBER_ENFORCEMENT_RESULT,
    constants::network_flow::SUBSCRIBER_AUDIT_ENTRY,
    constants::network_flow::SUBSCRIBER_PORTAL_READ_MODEL,
];

const NETWORK_RUNTIME_PHASE_TARGET_HANDLERS: [&str; 11] = [
    constants::network_flow::TARGET_NETWORK_OBSERVER,
    constants::network_flow::TARGET_DOMAIN_OBSERVER,
    constants::network_flow::TARGET_ACTIVITY_CLASSIFIER,
    constants::network_flow::TARGET_AI_ANALYZER,
    constants::network_flow::TARGET_AI_ANALYZER,
    constants::network_flow::TARGET_POLICY_ENGINE,
    constants::network_flow::TARGET_POLICY_ENGINE,
    constants::network_flow::TARGET_ENFORCEMENT_DRY_RUN,
    constants::network_flow::TARGET_ENFORCEMENT_DRY_RUN,
    constants::network_flow::TARGET_AUDIT_WRITER,
    constants::network_flow::TARGET_PORTAL_READ_MODEL,
];

const NETWORK_RUNTIME_PHASE_RUNTIME_ROLES: [&str; 11] = [
    constants::eventing_source::ROLE_AGENT,
    constants::eventing_source::ROLE_AGENT,
    constants::eventing_source::ROLE_AGENT,
    constants::eventing_source::ROLE_ANALYZER,
    constants::eventing_source::ROLE_ANALYZER,
    constants::eventing_source::ROLE_DECISION_ENGINE,
    constants::eventing_source::ROLE_DECISION_ENGINE,
    constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
    constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
    constants::eventing_source::ROLE_AUDIT_WRITER,
    constants::eventing_source::ROLE_READ_MODEL,
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRuntimePhase {
    FlowObserved,
    DomainObserved,
    ActivityClassified,
    AiAnalysisRequested,
    AiAnalysisCompleted,
    PolicyEvaluationRequested,
    PolicyDecisionCompleted,
    EnforcementCommandIssued,
    EnforcementResultObserved,
    AuditEntryCommitted,
    PortalReadModelUpdated,
}

impl NetworkRuntimePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &NETWORK_RUNTIME_PHASES
    }

    pub fn event_type(self) -> &'static str {
        NETWORK_RUNTIME_PHASE_EVENT_TYPES[self as usize]
    }

    pub fn subscriber_id(self) -> &'static str {
        NETWORK_RUNTIME_PHASE_SUBSCRIBER_IDS[self as usize]
    }

    pub fn target_handler(self) -> &'static str {
        NETWORK_RUNTIME_PHASE_TARGET_HANDLERS[self as usize]
    }

    pub fn runtime_role(self) -> Result<RuntimeRole, EventingError> {
        RuntimeRole::parse(NETWORK_RUNTIME_PHASE_RUNTIME_ROLES[self as usize])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvidenceScope {
    MetadataOnly,
    AdapterUnavailable,
}

// Runtime evidence grading is distinct from the A/B/C/D wire contract in network_flow_events.rs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRuntimeEvidenceGrade {
    DomainAndProcessMetadata,
    IpOrProcessPartialMetadata,
    AdapterUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiAuditState {
    NotRequested,
    Requested,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRiskBudgetState {
    ObserveOnly,
    ManualReviewRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkInterventionState {
    DryRunOnly,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkRuntimeClaimBoundary {
    pub raw_pcap_available: bool,
    pub decrypted_https_payload_available: bool,
    pub exact_url_available: bool,
    pub page_content_available: bool,
    pub video_content_available: bool,
    pub private_message_content_available: bool,
    pub search_query_available: bool,
    pub adapter_action_executed: bool,
}

impl NetworkRuntimeClaimBoundary {
    pub fn metadata_only() -> Self {
        Self {
            raw_pcap_available: false,
            decrypted_https_payload_available: false,
            exact_url_available: false,
            page_content_available: false,
            video_content_available: false,
            private_message_content_available: false,
            search_query_available: false,
            adapter_action_executed: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkRuntimeEventPayload {
    pub phase: NetworkRuntimePhase,
    pub capability_status: ActivityCaptureCapabilityStatus,
    pub domain_attribution_status: ActivityDomainAttributionStatus,
    pub process_attribution_status: ActivityProcessAttributionStatus,
    pub protocol: Option<ActivityNetworkProtocol>,
    pub tcp_state: Option<ActivityNetworkTcpState>,
    pub local_ip: Option<String>,
    pub local_port: Option<u16>,
    pub destination_ip: Option<String>,
    pub destination_port: Option<u16>,
    pub destination_domain: Option<String>,
    pub process_id: Option<u32>,
    pub process_name: Option<String>,
    pub evidence_scope: NetworkEvidenceScope,
    pub evidence_grade: NetworkRuntimeEvidenceGrade,
    /// Canonical A-D grade carried across the runtime event boundary.
    pub evidence_grade_contract: NetworkEvidenceGrade,
    pub ai_audit_state: NetworkAiAuditState,
    pub risk_budget_state: NetworkRiskBudgetState,
    pub intervention_state: NetworkInterventionState,
    /// Canonical policy intent; the evidence crate maps its local action into this boundary.
    pub policy_action: NetworkPolicyDecisionAction,
    pub claim_boundary: NetworkRuntimeClaimBoundary,
    pub previous_phase_ref: Option<String>,
    pub evidence_ref: String,
    pub ai_request_ref: Option<String>,
    pub ai_analysis_ref: Option<String>,
    pub policy_evaluation_ref: Option<String>,
    pub policy_decision_ref: Option<String>,
    pub adapter_capability_ref: Option<String>,
    pub enforcement_command_ref: Option<String>,
    pub enforcement_result_ref: Option<String>,
    pub audit_entry_ref: Option<String>,
    pub observed_at: String,
}

impl DomainEvent for NetworkRuntimeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        self.validate_semantics()?;
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::network_flow::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(network_runtime_aggregate_key(self))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::network_flow::IDEMPOTENCY_NETWORK_RUNTIME_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&network_runtime_aggregate_key(self));
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

impl NetworkRuntimeEventPayload {
    pub fn validate_semantics(&self) -> Result<(), EventingError> {
        let expected = match self.evidence_grade {
            NetworkRuntimeEvidenceGrade::DomainAndProcessMetadata => (
                NetworkEvidenceGrade::B,
                NetworkRiskBudgetState::ObserveOnly,
                NetworkInterventionState::DryRunOnly,
                NetworkPolicyDecisionAction::Observe,
            ),
            NetworkRuntimeEvidenceGrade::IpOrProcessPartialMetadata => (
                NetworkEvidenceGrade::C,
                NetworkRiskBudgetState::ManualReviewRequired,
                NetworkInterventionState::ManualRequired,
                NetworkPolicyDecisionAction::AskParent,
            ),
            NetworkRuntimeEvidenceGrade::AdapterUnavailable => (
                NetworkEvidenceGrade::D,
                NetworkRiskBudgetState::Unavailable,
                NetworkInterventionState::Unavailable,
                NetworkPolicyDecisionAction::ManualReview,
            ),
        };
        if (
            self.evidence_grade_contract,
            self.risk_budget_state,
            self.intervention_state,
            self.policy_action,
        ) != expected
        {
            return Err(EventingError::InvalidValue {
                field: "network_runtime_payload_semantics",
                value: "evidence/risk/intervention/policy tuple is inconsistent".to_string(),
            });
        }
        Ok(())
    }
}

fn network_runtime_aggregate_key(payload: &NetworkRuntimeEventPayload) -> String {
    payload.destination_domain.as_ref().map_or_else(
        || {
            payload.destination_ip.as_ref().map_or_else(
                || {
                    let mut value =
                        String::from(constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX);
                    value.push_str(payload.capability_status.as_protocol_str());
                    value
                },
                |ip| {
                    let mut value =
                        String::from(constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX);
                    value.push_str(ip);
                    let port_suffix = payload.destination_port.map_or_else(String::new, |port| {
                        format!("{}{}", constants::delimiter::HYPHEN, port)
                    });
                    value.push_str(&port_suffix);
                    value
                },
            )
        },
        |domain| {
            let mut value = String::from(constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX);
            value.push_str(domain);
            value
        },
    )
}
