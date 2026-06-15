#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use ocentra_eventing::{error::EventingError, ids::SchemaVersion};
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_source::{
    CompiledDomainPolicyArtifact, ParentPolicyDocumentId, PolicyAuditReferenceId,
    PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId, PolicyHouseholdId,
    PolicyReasonCode, PolicyRollbackRef, PolicyVersion,
};

const POLICY_DELIVERY_SCHEMA_VERSION_VALUE: u16 = 1;
const POLICY_DELIVERY_INITIAL_SEQUENCE_VALUE: u64 = 1;

macro_rules! policy_delivery_text_id {
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
    };
}

policy_delivery_text_id!(
    PolicyDeliveryId,
    policy_control::delivery::FIELD_DELIVERY_ID
);
policy_delivery_text_id!(
    PolicyDeliveryAttemptId,
    policy_control::delivery::FIELD_ATTEMPT_ID
);

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
        parent_visible_state(self.state)
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
    SchemaVersion::new(POLICY_DELIVERY_SCHEMA_VERSION_VALUE)
}

pub fn queue_policy_delivery(
    artifact: &CompiledDomainPolicyArtifact,
    target: PolicyDeliveryTarget,
    delivery_id: PolicyDeliveryId,
    attempt_id: PolicyDeliveryAttemptId,
    audit_reference_ids: Vec<PolicyAuditReferenceId>,
) -> Result<PolicyDeliveryRecord, EventingError> {
    let record = PolicyDeliveryRecord {
        schema_version: policy_delivery_schema_version()?,
        delivery_id,
        household_id: artifact.household_id.clone(),
        policy_version: artifact.policy_version,
        source_document_id: artifact.source_document_id.clone(),
        target,
        state: PolicyDeliveryState::Queued,
        last_sequence: PolicyDeliverySequence::new(POLICY_DELIVERY_INITIAL_SEQUENCE_VALUE)?,
        last_attempt_id: attempt_id,
        audit_reference_ids,
        source_audit_reference_ids: artifact.audit_reference_ids.clone(),
        source_superseded_by_policy_version: artifact.superseded_by_policy_version,
        source_rollback_ref: artifact.rollback_ref.clone(),
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    };
    validate_policy_delivery_record(&record)?;
    Ok(record)
}

pub fn validate_policy_delivery_record(record: &PolicyDeliveryRecord) -> Result<(), EventingError> {
    assert_audit_refs(&record.audit_reference_ids)?;
    assert_state_context(
        record.state,
        record.reason_code.as_ref(),
        record.superseded_by_policy_version,
        record.rollback_reference_state,
        record.policy_version,
    )?;
    Ok(())
}

pub fn apply_policy_delivery_transition(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    validate_policy_delivery_record(current)?;
    validate_policy_delivery_transition(&transition, current.policy_version)?;

    match transition
        .sequence
        .value()
        .cmp(&current.last_sequence.value())
    {
        std::cmp::Ordering::Less => return Ok(PolicyDeliveryApplyOutcome::Stale(current.clone())),
        std::cmp::Ordering::Equal => {
            if transition_matches_record(current, &transition) {
                return Ok(PolicyDeliveryApplyOutcome::Duplicate(current.clone()));
            }

            return Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_SEQUENCE,
                value: conflicting_replay_value(transition.sequence, &current.delivery_id),
            });
        }
        std::cmp::Ordering::Greater => {}
    }

    if !transition_allowed(current.state, transition.state) {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_STATE,
            value: invalid_transition_value(current.state, transition.state),
        });
    }

    let next = PolicyDeliveryRecord {
        schema_version: current.schema_version,
        delivery_id: current.delivery_id.clone(),
        household_id: current.household_id.clone(),
        policy_version: current.policy_version,
        source_document_id: current.source_document_id.clone(),
        target: current.target.clone(),
        state: transition.state,
        last_sequence: transition.sequence,
        last_attempt_id: transition.attempt_id,
        audit_reference_ids: transition.audit_reference_ids,
        source_audit_reference_ids: current.source_audit_reference_ids.clone(),
        source_superseded_by_policy_version: current.source_superseded_by_policy_version,
        source_rollback_ref: current.source_rollback_ref.clone(),
        reason_code: transition.reason_code,
        superseded_by_policy_version: transition.superseded_by_policy_version,
        rollback_reference_state: transition.rollback_reference_state,
    };
    validate_policy_delivery_record(&next)?;
    Ok(PolicyDeliveryApplyOutcome::Advanced(next))
}

fn validate_policy_delivery_transition(
    transition: &PolicyDeliveryTransition,
    current_policy_version: PolicyVersion,
) -> Result<(), EventingError> {
    assert_audit_refs(&transition.audit_reference_ids)?;
    assert_state_context(
        transition.state,
        transition.reason_code.as_ref(),
        transition.superseded_by_policy_version,
        transition.rollback_reference_state,
        current_policy_version,
    )?;
    Ok(())
}

fn assert_audit_refs(audit_reference_ids: &[PolicyAuditReferenceId]) -> Result<(), EventingError> {
    if audit_reference_ids.is_empty() {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_AUDIT_REFERENCE_IDS,
            value: policy_control::delivery::VALUE_MISSING_AUDIT_REFERENCES.to_string(),
        });
    }

    let mut seen = BTreeSet::new();
    for audit_reference_id in audit_reference_ids {
        if !seen.insert(audit_reference_id.clone()) {
            return Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_AUDIT_REFERENCE_IDS,
                value: audit_reference_id.as_str().to_string(),
            });
        }
    }

    Ok(())
}

fn assert_state_context(
    state: PolicyDeliveryState,
    reason_code: Option<&PolicyReasonCode>,
    superseded_by_policy_version: Option<PolicyVersion>,
    rollback_reference_state: Option<PolicyDeliveryState>,
    current_policy_version: PolicyVersion,
) -> Result<(), EventingError> {
    match state {
        PolicyDeliveryState::Queued
        | PolicyDeliveryState::Delivering
        | PolicyDeliveryState::Delivered
        | PolicyDeliveryState::Acknowledged
        | PolicyDeliveryState::Applied => {
            if let Some(reason_code) = reason_code {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_REASON_CODE,
                    value: unexpected_reason_code_value(reason_code, state),
                });
            }
            if let Some(superseded_by_policy_version) = superseded_by_policy_version {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                    value: unexpected_replacement_policy_version_value(
                        superseded_by_policy_version,
                        state,
                    ),
                });
            }
            if let Some(rollback_reference_state) = rollback_reference_state {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
                    value: unexpected_rollback_reference_state_value(
                        rollback_reference_state,
                        state,
                    ),
                });
            }
        }
        PolicyDeliveryState::Rejected
        | PolicyDeliveryState::Degraded
        | PolicyDeliveryState::Offline
        | PolicyDeliveryState::ExpiredBeforeDelivery
        | PolicyDeliveryState::RetryScheduled
        | PolicyDeliveryState::PartialDomainApply
        | PolicyDeliveryState::BlockedByPermission
        | PolicyDeliveryState::BlockedByCapability
        | PolicyDeliveryState::ManualRequired => {
            if reason_code.is_none() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_REASON_CODE,
                    value: missing_reason_code_value(state),
                });
            }
            if let Some(superseded_by_policy_version) = superseded_by_policy_version {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                    value: unexpected_replacement_policy_version_value(
                        superseded_by_policy_version,
                        state,
                    ),
                });
            }
            if let Some(rollback_reference_state) = rollback_reference_state {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
                    value: unexpected_rollback_reference_state_value(
                        rollback_reference_state,
                        state,
                    ),
                });
            }
        }
        PolicyDeliveryState::Superseded => {
            if let Some(reason_code) = reason_code {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_REASON_CODE,
                    value: unexpected_reason_code_value(reason_code, state),
                });
            }
            if let Some(rollback_reference_state) = rollback_reference_state {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
                    value: unexpected_rollback_reference_state_value(
                        rollback_reference_state,
                        state,
                    ),
                });
            }

            let superseded_by_policy_version =
                superseded_by_policy_version.ok_or_else(|| EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                    value: missing_replacement_policy_version_value(state),
                })?;

            if superseded_by_policy_version.value() <= current_policy_version.value() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                    value: replacement_policy_version_must_be_newer_value(
                        superseded_by_policy_version,
                        current_policy_version,
                    ),
                });
            }
        }
        PolicyDeliveryState::RolledBack => {
            if reason_code.is_none() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_REASON_CODE,
                    value: missing_reason_code_value(state),
                });
            }
            if let Some(superseded_by_policy_version) = superseded_by_policy_version {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                    value: unexpected_replacement_policy_version_value(
                        superseded_by_policy_version,
                        state,
                    ),
                });
            }

            let rollback_reference_state =
                rollback_reference_state.ok_or_else(|| EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
                    value: missing_rollback_reference_state_value(state),
                })?;

            if !matches!(
                rollback_reference_state,
                PolicyDeliveryState::Delivered
                    | PolicyDeliveryState::Acknowledged
                    | PolicyDeliveryState::Applied
                    | PolicyDeliveryState::PartialDomainApply
                    | PolicyDeliveryState::Degraded
                    | PolicyDeliveryState::Offline
            ) {
                return Err(EventingError::InvalidValue {
                    field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
                    value: policy_delivery_state_name(rollback_reference_state).to_string(),
                });
            }
        }
    }

    Ok(())
}

fn transition_matches_record(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
) -> bool {
    current.state == transition.state
        && current.last_attempt_id == transition.attempt_id
        && current.audit_reference_ids == transition.audit_reference_ids
        && current.reason_code == transition.reason_code
        && current.superseded_by_policy_version == transition.superseded_by_policy_version
        && current.rollback_reference_state == transition.rollback_reference_state
}

fn transition_allowed(current: PolicyDeliveryState, next: PolicyDeliveryState) -> bool {
    if current == next {
        return true;
    }

    match current {
        PolicyDeliveryState::Queued => matches!(
            next,
            PolicyDeliveryState::Delivering
                | PolicyDeliveryState::Delivered
                | PolicyDeliveryState::Acknowledged
                | PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::Offline
                | PolicyDeliveryState::ExpiredBeforeDelivery
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::PartialDomainApply
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::Delivering => matches!(
            next,
            PolicyDeliveryState::Delivered
                | PolicyDeliveryState::Acknowledged
                | PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::Offline
                | PolicyDeliveryState::ExpiredBeforeDelivery
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::PartialDomainApply
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::Delivered => matches!(
            next,
            PolicyDeliveryState::Acknowledged
                | PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::Offline
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::PartialDomainApply
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::Acknowledged => matches!(
            next,
            PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::Offline
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::PartialDomainApply
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::Applied => matches!(
            next,
            PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::Rejected => next == PolicyDeliveryState::Superseded,
        PolicyDeliveryState::Superseded => false,
        PolicyDeliveryState::RolledBack => next == PolicyDeliveryState::Superseded,
        PolicyDeliveryState::Degraded => matches!(
            next,
            PolicyDeliveryState::Delivering
                | PolicyDeliveryState::Delivered
                | PolicyDeliveryState::Acknowledged
                | PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Offline
                | PolicyDeliveryState::ExpiredBeforeDelivery
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::PartialDomainApply
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::Offline => matches!(
            next,
            PolicyDeliveryState::Delivering
                | PolicyDeliveryState::Delivered
                | PolicyDeliveryState::Acknowledged
                | PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::ExpiredBeforeDelivery
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::PartialDomainApply
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::ExpiredBeforeDelivery => next == PolicyDeliveryState::Superseded,
        PolicyDeliveryState::RetryScheduled => matches!(
            next,
            PolicyDeliveryState::Delivering
                | PolicyDeliveryState::Delivered
                | PolicyDeliveryState::Acknowledged
                | PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::Offline
                | PolicyDeliveryState::ExpiredBeforeDelivery
                | PolicyDeliveryState::PartialDomainApply
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::PartialDomainApply => matches!(
            next,
            PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::Offline
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
        PolicyDeliveryState::BlockedByPermission
        | PolicyDeliveryState::BlockedByCapability
        | PolicyDeliveryState::ManualRequired => matches!(
            next,
            PolicyDeliveryState::Delivering
                | PolicyDeliveryState::Delivered
                | PolicyDeliveryState::Acknowledged
                | PolicyDeliveryState::Applied
                | PolicyDeliveryState::Rejected
                | PolicyDeliveryState::Superseded
                | PolicyDeliveryState::RolledBack
                | PolicyDeliveryState::Degraded
                | PolicyDeliveryState::Offline
                | PolicyDeliveryState::ExpiredBeforeDelivery
                | PolicyDeliveryState::RetryScheduled
                | PolicyDeliveryState::PartialDomainApply
                | PolicyDeliveryState::BlockedByPermission
                | PolicyDeliveryState::BlockedByCapability
                | PolicyDeliveryState::ManualRequired
        ),
    }
}

fn parent_visible_state(state: PolicyDeliveryState) -> PolicyDeliveryParentVisibleState {
    match state {
        PolicyDeliveryState::Queued
        | PolicyDeliveryState::Delivering
        | PolicyDeliveryState::Delivered
        | PolicyDeliveryState::Acknowledged => PolicyDeliveryParentVisibleState::Pending,
        PolicyDeliveryState::Applied => PolicyDeliveryParentVisibleState::Applied,
        PolicyDeliveryState::Rejected
        | PolicyDeliveryState::RolledBack
        | PolicyDeliveryState::BlockedByPermission
        | PolicyDeliveryState::BlockedByCapability
        | PolicyDeliveryState::ManualRequired => PolicyDeliveryParentVisibleState::ManualRequired,
        PolicyDeliveryState::Superseded => PolicyDeliveryParentVisibleState::Superseded,
        PolicyDeliveryState::Degraded
        | PolicyDeliveryState::Offline
        | PolicyDeliveryState::ExpiredBeforeDelivery
        | PolicyDeliveryState::RetryScheduled
        | PolicyDeliveryState::PartialDomainApply => PolicyDeliveryParentVisibleState::Degraded,
    }
}

fn policy_delivery_state_name(state: PolicyDeliveryState) -> &'static str {
    match state {
        PolicyDeliveryState::Queued => policy_control::delivery::STATUS_QUEUED,
        PolicyDeliveryState::Delivering => "delivering",
        PolicyDeliveryState::Delivered => policy_control::delivery::STATUS_DELIVERED,
        PolicyDeliveryState::Acknowledged => policy_control::delivery::STATUS_ACKNOWLEDGED,
        PolicyDeliveryState::Applied => policy_control::delivery::STATUS_APPLIED,
        PolicyDeliveryState::Rejected => policy_control::delivery::STATUS_REJECTED,
        PolicyDeliveryState::Superseded => policy_control::delivery::STATUS_SUPERSEDED,
        PolicyDeliveryState::RolledBack => policy_control::delivery::STATUS_ROLLED_BACK,
        PolicyDeliveryState::Degraded => policy_control::delivery::STATUS_DEGRADED,
        PolicyDeliveryState::Offline => policy_control::delivery::STATUS_OFFLINE,
        PolicyDeliveryState::ExpiredBeforeDelivery => "expired-before-delivery",
        PolicyDeliveryState::RetryScheduled => "retry-scheduled",
        PolicyDeliveryState::PartialDomainApply => "partial-domain-apply",
        PolicyDeliveryState::BlockedByPermission => "blocked-by-permission",
        PolicyDeliveryState::BlockedByCapability => "blocked-by-capability",
        PolicyDeliveryState::ManualRequired => "manual-required",
    }
}

fn conflicting_replay_value(
    sequence: PolicyDeliverySequence,
    delivery_id: &PolicyDeliveryId,
) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_CONFLICTING_REPLAY_FOR_SEQUENCE_PREFIX);
    value.push_str(&sequence.value().to_string());
    value.push_str(policy_control::delivery::VALUE_CONFLICTING_REPLAY_ON_SEPARATOR);
    value.push_str(delivery_id.as_str());
    value
}

fn invalid_transition_value(current: PolicyDeliveryState, next: PolicyDeliveryState) -> String {
    let mut value = String::from(policy_control::delivery::VALUE_INVALID_TRANSITION_PREFIX);
    value.push_str(policy_delivery_state_name(current));
    value.push_str(policy_control::delivery::VALUE_INVALID_TRANSITION_SEPARATOR);
    value.push_str(policy_delivery_state_name(next));
    value
}

fn unexpected_reason_code_value(
    reason_code: &PolicyReasonCode,
    state: PolicyDeliveryState,
) -> String {
    let mut value = String::from(policy_control::delivery::VALUE_UNEXPECTED_REASON_CODE_PREFIX);
    value.push_str(reason_code.as_str());
    value.push_str(policy_control::delivery::VALUE_FOR_STATE_SEPARATOR);
    value.push_str(policy_delivery_state_name(state));
    value
}

fn unexpected_replacement_policy_version_value(
    policy_version: PolicyVersion,
    state: PolicyDeliveryState,
) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_UNEXPECTED_REPLACEMENT_POLICY_VERSION_PREFIX);
    value.push_str(&policy_version.value().to_string());
    value.push_str(policy_control::delivery::VALUE_FOR_STATE_SEPARATOR);
    value.push_str(policy_delivery_state_name(state));
    value
}

fn unexpected_rollback_reference_state_value(
    rollback_reference_state: PolicyDeliveryState,
    state: PolicyDeliveryState,
) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_UNEXPECTED_ROLLBACK_REFERENCE_STATE_PREFIX);
    value.push_str(policy_delivery_state_name(rollback_reference_state));
    value.push_str(policy_control::delivery::VALUE_FOR_STATE_SEPARATOR);
    value.push_str(policy_delivery_state_name(state));
    value
}

fn missing_reason_code_value(state: PolicyDeliveryState) -> String {
    let mut value = String::from(policy_control::delivery::VALUE_MISSING_REASON_CODE_FOR_PREFIX);
    value.push_str(policy_delivery_state_name(state));
    value
}

fn missing_replacement_policy_version_value(state: PolicyDeliveryState) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_MISSING_REPLACEMENT_POLICY_VERSION_FOR_PREFIX);
    value.push_str(policy_delivery_state_name(state));
    value
}

fn replacement_policy_version_must_be_newer_value(
    superseded_by_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::delivery::VALUE_REPLACEMENT_POLICY_VERSION_PREFIX);
    value.push_str(&superseded_by_policy_version.value().to_string());
    value.push_str(policy_control::delivery::VALUE_MUST_BE_NEWER_THAN_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}

fn missing_rollback_reference_state_value(state: PolicyDeliveryState) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_MISSING_ROLLBACK_REFERENCE_STATE_FOR_PREFIX);
    value.push_str(policy_delivery_state_name(state));
    value
}
