use ocentra_eventing::expect_value::ExpectValue;
use serde::{Deserialize, Serialize};

use super::config_update_event::{TrackingConfigEffectiveState, TrackingConfigUpdateResponseState};
use super::identifiers::{
    TrackingAcceptedAt, TrackingDurableSettingsStoreRef, TrackingLocalServiceStateSnapshotRef,
    TrackingMutationProofRef, TrackingReadModelProofRef, TrackingRetentionCommandId,
    TrackingRetentionSettingsKind, TrackingRetentionWriteState, TrackingWriterIntentRef,
};
use crate::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};

pub const TRACKING_RETENTION_SETTINGS_WRITE_SCHEMA_VERSION: u16 =
    crate::AGENT_PROTOCOL_SCHEMA_VERSION;

fn parse_or_panic<T, E>(result: Result<T, E>, message: &'static str) -> T {
    result.expect_value(message)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingDeleteAfterAlertResolutionState {
    #[serde(rename = "delete-after-alert-resolved")]
    DeleteAfterAlertResolved,
    #[serde(rename = "retain-after-alert-resolved")]
    RetainAfterAlertResolved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingParentExportState {
    #[serde(rename = "prepared")]
    Prepared,
    #[serde(rename = "not-prepared")]
    NotPrepared,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingRemoteSyncState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingRemoteAiState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingDurableSettingsPersistenceState {
    #[serde(rename = "persisted")]
    Persisted,
    #[serde(rename = "not-persisted")]
    NotPersisted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingConfigAckState {
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingExecutionClaimState {
    #[serde(rename = "claimed")]
    Claimed,
    #[serde(rename = "unclaimed")]
    Unclaimed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRetentionSettingsWriteRequest {
    pub schema_version: u16,
    pub command_id: TrackingRetentionCommandId,
    pub settings_kind: TrackingRetentionSettingsKind,
    pub requested_retention_window_hours: Option<u16>,
    pub requested_delete_after_alert_resolution_state: TrackingDeleteAfterAlertResolutionState,
    pub requested_parent_export_state: TrackingParentExportState,
    pub requested_remote_sync_state: TrackingRemoteSyncState,
    pub requested_remote_ai_state: TrackingRemoteAiState,
    pub source_writer_intent_refs: Vec<TrackingWriterIntentRef>,
    pub source_read_model_proof_refs: Vec<TrackingReadModelProofRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRetentionSettingsWriteResult {
    pub schema_version: u16,
    pub command_id: TrackingRetentionCommandId,
    pub settings_kind: TrackingRetentionSettingsKind,
    pub write_state: TrackingRetentionWriteState,
    pub accepted_at: TrackingAcceptedAt,
    pub source_writer_intent_refs: Vec<TrackingWriterIntentRef>,
    pub source_read_model_proof_refs: Vec<TrackingReadModelProofRef>,
    pub source_mutation_proof_refs: Vec<TrackingMutationProofRef>,
    pub applied_retention_window_hours: Option<u16>,
    pub applied_delete_after_alert_resolution_state: TrackingDeleteAfterAlertResolutionState,
    pub parent_export_state: TrackingParentExportState,
    pub remote_sync_state: TrackingRemoteSyncState,
    pub remote_ai_state: TrackingRemoteAiState,
    pub local_service_state_revision: Option<u64>,
    pub local_service_state_snapshot_ref: TrackingLocalServiceStateSnapshotRef,
    pub durable_settings_store_ref: TrackingDurableSettingsStoreRef,
    pub durable_settings_persistence_state: TrackingDurableSettingsPersistenceState,
    pub child_config_response_state: Option<TrackingConfigUpdateResponseState>,
    pub effective_tracking_state: Option<TrackingConfigEffectiveState>,
    pub child_config_ack_state: TrackingConfigAckState,
    pub command_transport_claim_state: TrackingExecutionClaimState,
    pub service_write_preflight_claim_state: TrackingExecutionClaimState,
    pub service_mutation_execution_state: TrackingExecutionClaimState,
    pub portal_writable_ui_claim_state: TrackingExecutionClaimState,
    pub platform_runtime_claim_state: TrackingExecutionClaimState,
    pub child_device_delivery_claim_state: TrackingExecutionClaimState,
    pub provider_delivery_claim_state: TrackingExecutionClaimState,
    pub notification_receipt_claim_state: TrackingExecutionClaimState,
    pub physical_device_claim_state: TrackingExecutionClaimState,
    pub authority_claim_state: TrackingExecutionClaimState,
    pub product_claim_state: TrackingExecutionClaimState,
}

pub fn default_tracking_retention_settings_write_request() -> TrackingRetentionSettingsWriteRequest
{
    TrackingRetentionSettingsWriteRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: tracking_retention_command_id(),
        settings_kind: tracking_retention_settings_kind(),
        requested_retention_window_hours: Some(168),
        requested_delete_after_alert_resolution_state:
            TrackingDeleteAfterAlertResolutionState::RetainAfterAlertResolved,
        requested_parent_export_state: TrackingParentExportState::NotPrepared,
        requested_remote_sync_state: TrackingRemoteSyncState::Disabled,
        requested_remote_ai_state: TrackingRemoteAiState::Disabled,
        source_writer_intent_refs: vec![tracking_writer_intent_ref()],
        source_read_model_proof_refs: vec![
            tracking_read_model_proof_ref(
                constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF,
            ),
            tracking_read_model_proof_ref(
                constants::tracking_retention_settings_write::JOURNAL_READ_MODEL_PROOF_REF,
            ),
        ],
    }
}

pub fn tracking_retention_command_id() -> TrackingRetentionCommandId {
    parse_or_panic(
        TrackingRetentionCommandId::parse(constants::tracking_retention_settings_write::COMMAND_ID),
        constants::tracking_retention_settings_write::COMMAND_ID,
    )
}

pub fn tracking_retention_settings_kind() -> TrackingRetentionSettingsKind {
    parse_or_panic(
        TrackingRetentionSettingsKind::parse(
            constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW,
        ),
        constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW,
    )
}

pub fn tracking_retention_write_state_accepted() -> TrackingRetentionWriteState {
    parse_or_panic(
        TrackingRetentionWriteState::parse(
            constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED,
        ),
        constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED,
    )
}

pub fn tracking_retention_write_state_rejected() -> TrackingRetentionWriteState {
    parse_or_panic(
        TrackingRetentionWriteState::parse(
            constants::tracking_retention_settings_write::WRITE_STATE_REJECTED,
        ),
        constants::tracking_retention_settings_write::WRITE_STATE_REJECTED,
    )
}

pub fn tracking_retention_accepted_at() -> TrackingAcceptedAt {
    parse_or_panic(
        TrackingAcceptedAt::parse(constants::tracking_retention_settings_write::ACCEPTED_AT),
        constants::tracking_retention_settings_write::ACCEPTED_AT,
    )
}

pub fn tracking_writer_intent_ref() -> TrackingWriterIntentRef {
    parse_or_panic(
        TrackingWriterIntentRef::parse(
            constants::tracking_retention_settings_write::WRITER_INTENT_REF,
        ),
        constants::tracking_retention_settings_write::WRITER_INTENT_REF,
    )
}

pub fn tracking_read_model_proof_ref(value: &'static str) -> TrackingReadModelProofRef {
    parse_or_panic(
        TrackingReadModelProofRef::parse(value),
        constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF,
    )
}

pub fn tracking_mutation_proof_ref() -> TrackingMutationProofRef {
    parse_or_panic(
        TrackingMutationProofRef::parse(
            constants::tracking_retention_settings_write::MUTATION_PROOF_REF,
        ),
        constants::tracking_retention_settings_write::MUTATION_PROOF_REF,
    )
}

pub fn tracking_local_service_state_snapshot_ref() -> TrackingLocalServiceStateSnapshotRef {
    parse_or_panic(
        TrackingLocalServiceStateSnapshotRef::parse(
            constants::tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF,
        ),
        constants::tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF,
    )
}

pub fn tracking_durable_settings_store_ref() -> TrackingDurableSettingsStoreRef {
    parse_or_panic(
        TrackingDurableSettingsStoreRef::parse(
            constants::tracking_retention_settings_write::DURABLE_SETTINGS_STORE_REF,
        ),
        constants::tracking_retention_settings_write::DURABLE_SETTINGS_STORE_REF,
    )
}
