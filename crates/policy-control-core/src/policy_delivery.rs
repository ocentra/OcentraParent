#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_source::{
    CompiledDomainPolicyArtifact, ParentPolicyDocumentId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId, PolicyHouseholdId,
    PolicyReasonCode, PolicyRollbackRef, PolicyVersion,
};

mod state_context;
mod state_values;
mod transition_rules;
mod transitions;
mod validation;

const POLICY_DELIVERY_SCHEMA_VERSION_VALUE: u16 = 1;
const POLICY_DELIVERY_INITIAL_SEQUENCE_VALUE: u64 = 1;

fn parse_delivery_text_id(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    Ok(value)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyDeliveryId(String);

impl PolicyDeliveryId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_delivery_text_id(value, policy_control::delivery::FIELD_DELIVERY_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyDeliveryId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyDeliveryId> for String {
    fn from(value: PolicyDeliveryId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyDeliveryAttemptId(String);

impl PolicyDeliveryAttemptId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_delivery_text_id(value, policy_control::delivery::FIELD_ATTEMPT_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyDeliveryAttemptId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyDeliveryAttemptId> for String {
    fn from(value: PolicyDeliveryAttemptId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct PolicyDeliverySequence(u64);

impl PolicyDeliverySequence {
    pub fn new(value: u64) -> Result<Self, EventingError> {
        if value == 0 {
            return Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_SEQUENCE,
                value: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for PolicyDeliverySequence {
    type Error = EventingError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PolicyDeliverySequence> for u64 {
    fn from(value: PolicyDeliverySequence) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDeliveryState {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "delivering")]
    Delivering,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "acknowledged")]
    Acknowledged,
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "superseded")]
    Superseded,
    #[serde(rename = "rolled-back")]
    RolledBack,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "expired-before-delivery")]
    ExpiredBeforeDelivery,
    #[serde(rename = "retry-scheduled")]
    RetryScheduled,
    #[serde(rename = "partial-domain-apply")]
    PartialDomainApply,
    #[serde(rename = "blocked-by-permission")]
    BlockedByPermission,
    #[serde(rename = "blocked-by-capability")]
    BlockedByCapability,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDeliveryParentVisibleState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "superseded")]
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDeliveryTarget {
    pub child_profile_id: PolicyChildProfileId,
    pub device_id: PolicyDeviceId,
    pub domain: PolicyConsumerDomain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDeliveryRecord {
    pub schema_version: SchemaVersion,
    pub delivery_id: PolicyDeliveryId,
    pub household_id: PolicyHouseholdId,
    pub policy_version: PolicyVersion,
    pub source_document_id: ParentPolicyDocumentId,
    pub target: PolicyDeliveryTarget,
    pub state: PolicyDeliveryState,
    pub last_sequence: PolicyDeliverySequence,
    pub last_attempt_id: PolicyDeliveryAttemptId,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    #[serde(default)]
    pub source_audit_reference_ids: Vec<PolicyAuditReferenceId>,
    #[serde(default)]
    pub source_superseded_by_policy_version: Option<PolicyVersion>,
    #[serde(default)]
    pub source_rollback_ref: Option<PolicyRollbackRef>,
    pub reason_code: Option<PolicyReasonCode>,
    pub superseded_by_policy_version: Option<PolicyVersion>,
    pub rollback_reference_state: Option<PolicyDeliveryState>,
}

impl PolicyDeliveryRecord {
    pub fn parent_visible_state(&self) -> PolicyDeliveryParentVisibleState {
        state_values::policy_delivery_parent_visible_state(self.state)
    }

    pub fn is_active(&self) -> bool {
        self.state == PolicyDeliveryState::Applied
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDeliveryTransition {
    pub attempt_id: PolicyDeliveryAttemptId,
    pub sequence: PolicyDeliverySequence,
    pub state: PolicyDeliveryState,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    pub reason_code: Option<PolicyReasonCode>,
    pub superseded_by_policy_version: Option<PolicyVersion>,
    pub rollback_reference_state: Option<PolicyDeliveryState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDeliveryApplyOutcome {
    Advanced(PolicyDeliveryRecord),
    Duplicate(PolicyDeliveryRecord),
    Stale(PolicyDeliveryRecord),
}

impl PolicyDeliveryApplyOutcome {
    pub fn into_record(self) -> PolicyDeliveryRecord {
        match self {
            Self::Advanced(record) | Self::Duplicate(record) | Self::Stale(record) => record,
        }
    }
}

pub fn policy_delivery_schema_version() -> Result<SchemaVersion, EventingError> {
    validation::policy_delivery_schema_version()
}

pub fn queue_policy_delivery(
    artifact: &CompiledDomainPolicyArtifact,
    target: PolicyDeliveryTarget,
    delivery_id: PolicyDeliveryId,
    attempt_id: PolicyDeliveryAttemptId,
    audit_reference_ids: Vec<PolicyAuditReferenceId>,
) -> Result<PolicyDeliveryRecord, EventingError> {
    transitions::queue_policy_delivery(
        artifact,
        target,
        delivery_id,
        attempt_id,
        audit_reference_ids,
    )
}

pub fn validate_policy_delivery_record(record: &PolicyDeliveryRecord) -> Result<(), EventingError> {
    validation::validate_policy_delivery_record(record)
}

pub fn apply_policy_delivery_transition(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    transitions::apply_policy_delivery_transition(current, transition)
}
