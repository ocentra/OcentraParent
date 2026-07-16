#![forbid(unsafe_code)]

//! Storage custody and retention ownership.
//!
//! This crate owns generic custody/delete/export decisions. Evidence crates own
//! evidence identity; feature crates own feature-specific interpretation.

use serde::{Deserialize, Serialize};

#[path = "storage_custody_decision.rs"]
mod storage_custody_decision;
#[path = "storage_custody_event_impls.rs"]
mod storage_custody_event_impls;
#[path = "storage_custody_events.rs"]
mod storage_custody_events;
#[path = "storage_custody_text_id.rs"]
mod storage_custody_text_id;

pub const CRATE_NAME: &str = "ocentra-storage-custody-core";
const STORAGE_CUSTODY_SCHEMA_VERSION: u16 = 1;
const STORAGE_CUSTODY_DECISION_RECORDED_EVENT_TYPE: &str = "storage-custody.decision.recorded";
const STORAGE_CUSTODY_ACTION_PLANNED_EVENT_TYPE: &str = "storage-custody.action.planned";
const STORAGE_CUSTODY_IDEMPOTENCY_SEPARATOR: &str = ":";
const STORAGE_CUSTODY_ACTION_PREFIX: &str = "storage-custody-action:";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageCustodyLocation {
    #[serde(rename = "child-device-local")]
    ChildDeviceLocal,
    #[serde(rename = "parent-device-local")]
    ParentDeviceLocal,
    #[serde(rename = "parent-owned-remote")]
    ParentOwnedRemote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetentionWindowState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentExportState {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "not-requested")]
    NotRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteSyncState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalPayloadRetentionAction {
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "retain")]
    Retain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentExportPacketState {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "do-not-create")]
    DoNotCreate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteUploadState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageTombstoneState {
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "do-not-write")]
    DoNotWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageAuditState {
    #[serde(rename = "record")]
    Record,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCustodyInput {
    pub location: StorageCustodyLocation,
    pub retention_window_state: RetentionWindowState,
    pub parent_export_state: ParentExportState,
    pub remote_sync_state: RemoteSyncState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCustodyDecision {
    pub local_payload_retention_action: LocalPayloadRetentionAction,
    pub parent_export_packet_state: ParentExportPacketState,
    pub remote_upload_state: RemoteUploadState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCustodyActionPlan {
    pub local_payload_retention_action: LocalPayloadRetentionAction,
    pub tombstone_state: StorageTombstoneState,
    pub parent_export_packet_state: ParentExportPacketState,
    pub remote_upload_state: RemoteUploadState,
    pub audit_state: StorageAuditState,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct StorageCustodyDecisionId(String);

impl StorageCustodyDecisionId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ocentra_eventing::error::EventingError> {
        Ok(Self(storage_custody_text_id::parse_nonempty_text_id(
            "storage_custody.decision_id",
            value,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for StorageCustodyDecisionId {
    type Error = ocentra_eventing::error::EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<StorageCustodyDecisionId> for String {
    fn from(value: StorageCustodyDecisionId) -> Self {
        value.0
    }
}

impl std::fmt::Display for StorageCustodyDecisionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct StorageCustodyActionPlanId(String);

impl StorageCustodyActionPlanId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ocentra_eventing::error::EventingError> {
        Ok(Self(storage_custody_text_id::parse_nonempty_text_id(
            "storage_custody.action_plan_id",
            value,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for StorageCustodyActionPlanId {
    type Error = ocentra_eventing::error::EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<StorageCustodyActionPlanId> for String {
    fn from(value: StorageCustodyActionPlanId) -> Self {
        value.0
    }
}

impl std::fmt::Display for StorageCustodyActionPlanId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct StorageCustodyAggregateId(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCustodyDecisionRecordedEvent {
    pub aggregate_id: StorageCustodyAggregateId,
    pub decision_id: StorageCustodyDecisionId,
    pub input: StorageCustodyInput,
    pub decision: StorageCustodyDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCustodyActionPlannedEvent {
    pub aggregate_id: StorageCustodyAggregateId,
    pub action_plan_id: StorageCustodyActionPlanId,
    pub source_decision_id: StorageCustodyDecisionId,
    pub action_plan: StorageCustodyActionPlan,
}

pub fn evaluate_storage_custody(input: StorageCustodyInput) -> StorageCustodyDecision {
    storage_custody_decision::evaluate_storage_custody(input)
}

pub fn storage_custody_decision_recorded_event(
    aggregate_id: StorageCustodyAggregateId,
    decision_id: StorageCustodyDecisionId,
    input: StorageCustodyInput,
) -> StorageCustodyDecisionRecordedEvent {
    storage_custody_events::storage_custody_decision_recorded_event(
        aggregate_id,
        decision_id,
        input,
    )
}

pub fn plan_storage_custody_actions(input: StorageCustodyInput) -> StorageCustodyActionPlan {
    storage_custody_events::plan_storage_custody_actions(input)
}

pub fn storage_custody_action_planned_event(
    event: StorageCustodyDecisionRecordedEvent,
) -> StorageCustodyActionPlannedEvent {
    storage_custody_events::storage_custody_action_planned_event(event)
}
