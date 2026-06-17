#![forbid(unsafe_code)]

//! Storage custody and retention ownership.
//!
//! This crate owns generic custody/delete/export decisions. Evidence crates own
//! evidence identity; feature crates own feature-specific interpretation.

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-storage-custody-core";
const STORAGE_CUSTODY_SCHEMA_VERSION: u16 = 1;
const STORAGE_CUSTODY_DECISION_RECORDED_EVENT_TYPE: &str = "storage-custody.decision.recorded";
const STORAGE_CUSTODY_ACTION_PLANNED_EVENT_TYPE: &str = "storage-custody.action.planned";
const STORAGE_CUSTODY_IDEMPOTENCY_SEPARATOR: &str = ":";
const STORAGE_CUSTODY_ACTION_PREFIX: &str = "storage-custody-action:";
const ERROR_STORAGE_CUSTODY_ACTION_ID: &str = "storage custody action id";

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

macro_rules! storage_custody_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

storage_custody_text_id!(StorageCustodyDecisionId, "storage_custody.decision_id");
storage_custody_text_id!(StorageCustodyActionPlanId, "storage_custody.action_plan_id");
storage_custody_text_id!(StorageCustodyAggregateId, "storage_custody.aggregate_id");

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

impl DomainEvent for StorageCustodyDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        storage_custody_event_contract(STORAGE_CUSTODY_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        storage_custody_idempotency_key(
            STORAGE_CUSTODY_DECISION_RECORDED_EVENT_TYPE,
            &self.decision_id,
        )
    }
}

impl DomainEvent for StorageCustodyActionPlannedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        storage_custody_event_contract(STORAGE_CUSTODY_ACTION_PLANNED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        storage_custody_idempotency_key(
            STORAGE_CUSTODY_ACTION_PLANNED_EVENT_TYPE,
            &self.action_plan_id,
        )
    }
}

pub fn evaluate_storage_custody(input: StorageCustodyInput) -> StorageCustodyDecision {
    let delete_local_payload = input.retention_window_state == RetentionWindowState::Expired;
    let create_parent_export_packet = input.parent_export_state == ParentExportState::Requested;
    let remote_upload_allowed = input.remote_sync_state == RemoteSyncState::Enabled
        && input.location == StorageCustodyLocation::ParentOwnedRemote;

    StorageCustodyDecision {
        local_payload_retention_action: if delete_local_payload {
            LocalPayloadRetentionAction::Delete
        } else {
            LocalPayloadRetentionAction::Retain
        },
        parent_export_packet_state: if create_parent_export_packet {
            ParentExportPacketState::Create
        } else {
            ParentExportPacketState::DoNotCreate
        },
        remote_upload_state: if remote_upload_allowed {
            RemoteUploadState::Allowed
        } else {
            RemoteUploadState::Blocked
        },
    }
}

pub fn storage_custody_decision_recorded_event(
    aggregate_id: StorageCustodyAggregateId,
    decision_id: StorageCustodyDecisionId,
    input: StorageCustodyInput,
) -> StorageCustodyDecisionRecordedEvent {
    StorageCustodyDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input,
        decision: evaluate_storage_custody(input),
    }
}

pub fn plan_storage_custody_actions(input: StorageCustodyInput) -> StorageCustodyActionPlan {
    let decision = evaluate_storage_custody(input);
    StorageCustodyActionPlan {
        local_payload_retention_action: decision.local_payload_retention_action,
        tombstone_state: if decision.local_payload_retention_action
            == LocalPayloadRetentionAction::Delete
        {
            StorageTombstoneState::Write
        } else {
            StorageTombstoneState::DoNotWrite
        },
        parent_export_packet_state: decision.parent_export_packet_state,
        remote_upload_state: decision.remote_upload_state,
        audit_state: StorageAuditState::Record,
    }
}

pub fn storage_custody_action_planned_event(
    event: StorageCustodyDecisionRecordedEvent,
) -> StorageCustodyActionPlannedEvent {
    StorageCustodyActionPlannedEvent {
        aggregate_id: event.aggregate_id,
        action_plan_id: StorageCustodyActionPlanId::parse(storage_custody_action_ref(
            &event.decision_id,
        ))
        .expect(ERROR_STORAGE_CUSTODY_ACTION_ID),
        source_decision_id: event.decision_id,
        action_plan: plan_storage_custody_actions(event.input),
    }
}

fn storage_custody_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(STORAGE_CUSTODY_SCHEMA_VERSION)?,
    ))
}

fn storage_custody_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, STORAGE_CUSTODY_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn storage_custody_action_ref(decision_id: &StorageCustodyDecisionId) -> String {
    let mut value = String::from(STORAGE_CUSTODY_ACTION_PREFIX);
    value.push_str(decision_id.as_str());
    value
}
