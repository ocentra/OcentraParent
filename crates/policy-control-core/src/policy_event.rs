#![forbid(unsafe_code)]

use ocentra_eventing::contract_registry::EventContractRegistry;
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventNamespace, EventType, IdempotencyKey, SchemaVersion,
};
use ocentra_eventing::topology::EventTopologyFamilyVariant;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_delivery::PolicyDeliveryId;
use crate::policy_request::{PolicyApprovalId, PolicyOverrideId, PolicyRequestId};
use crate::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRollbackRef, PolicyVersion,
};

#[path = "policy_event/kind.rs"]
mod kind;
mod replay;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PolicyEventKind {
    #[serde(rename = "policy.draft.created")]
    DraftCreated,
    #[serde(rename = "policy.preview.requested")]
    PreviewRequested,
    #[serde(rename = "policy.preview.generated")]
    PreviewGenerated,
    #[serde(rename = "policy.confirmed")]
    Confirmed,
    #[serde(rename = "policy.version.superseded")]
    VersionSuperseded,
    #[serde(rename = "policy.compiler.requested")]
    CompilerRequested,
    #[serde(rename = "policy.compiler.completed")]
    CompilerCompleted,
    #[serde(rename = "policy.delivery.queued")]
    DeliveryQueued,
    #[serde(rename = "policy.delivery.sent")]
    DeliverySent,
    #[serde(rename = "policy.delivery.acknowledged")]
    DeliveryAcknowledged,
    #[serde(rename = "policy.delivery.rejected")]
    DeliveryRejected,
    #[serde(rename = "policy.delivery.expired")]
    DeliveryExpired,
    #[serde(rename = "policy.delivery.retry-scheduled")]
    DeliveryRetryScheduled,
    #[serde(rename = "policy.domain.applied")]
    DomainApplied,
    #[serde(rename = "policy.domain.partial")]
    DomainPartial,
    #[serde(rename = "policy.rollback.requested")]
    RollbackRequested,
    #[serde(rename = "policy.rollback.applied")]
    RollbackApplied,
    #[serde(rename = "policy.ask-parent.requested")]
    AskParentRequested,
    #[serde(rename = "policy.ask-parent.approved")]
    AskParentApproved,
    #[serde(rename = "policy.ask-parent.denied")]
    AskParentDenied,
    #[serde(rename = "policy.override.created")]
    OverrideCreated,
    #[serde(rename = "policy.override.expired")]
    OverrideExpired,
    #[serde(rename = "policy.audit.recorded")]
    AuditRecorded,
    #[serde(rename = "policy.dead-letter.recorded")]
    DeadLetterRecorded,
    #[serde(rename = "policy.manual-required")]
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PolicyEventDeadLetterReason {
    DuplicateIdempotency,
    ReplayRejected,
    StaleSequence,
    UnsupportedTarget,
    MissingSubscriber,
    ManualRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct PolicyEventSequence(u64);

impl PolicyEventSequence {
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

impl TryFrom<u64> for PolicyEventSequence {
    type Error = EventingError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PolicyEventSequence> for u64 {
    fn from(value: PolicyEventSequence) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "kebab-case")]
pub enum PolicyEventScope {
    SourceDocument {
        household_id: PolicyHouseholdId,
        source_document_id: ParentPolicyDocumentId,
        policy_version: PolicyVersion,
    },
    Request {
        household_id: PolicyHouseholdId,
        request_id: PolicyRequestId,
        child_profile_id: PolicyChildProfileId,
        source_document_id: ParentPolicyDocumentId,
        policy_version: PolicyVersion,
    },
    Approval {
        household_id: PolicyHouseholdId,
        approval_id: PolicyApprovalId,
        request_id: PolicyRequestId,
        source_document_id: ParentPolicyDocumentId,
        policy_version: PolicyVersion,
    },
    Override {
        household_id: PolicyHouseholdId,
        override_id: PolicyOverrideId,
        approval_id: PolicyApprovalId,
        request_id: PolicyRequestId,
        source_document_id: ParentPolicyDocumentId,
        policy_version: PolicyVersion,
    },
    Delivery {
        household_id: PolicyHouseholdId,
        delivery_id: PolicyDeliveryId,
        child_profile_id: PolicyChildProfileId,
        device_id: PolicyDeviceId,
        domain: PolicyConsumerDomain,
        source_document_id: ParentPolicyDocumentId,
        policy_version: PolicyVersion,
    },
    Rollback {
        household_id: PolicyHouseholdId,
        rollback_ref: PolicyRollbackRef,
    },
    Audit {
        household_id: PolicyHouseholdId,
        audit_reference_id: PolicyAuditReferenceId,
        source_document_id: ParentPolicyDocumentId,
        policy_version: PolicyVersion,
    },
}

impl PolicyEventScope {
    pub fn family_label(&self) -> &'static str {
        replay::policy_event_scope_family_label(self)
    }

    pub fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        replay::policy_event_scope_aggregate_key(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEvent {
    pub schema_version: SchemaVersion,
    pub kind: PolicyEventKind,
    pub sequence: PolicyEventSequence,
    pub scope: PolicyEventScope,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    #[serde(default)]
    pub reason_code: Option<PolicyReasonCode>,
    #[serde(default)]
    pub dead_letter_reason: Option<PolicyEventDeadLetterReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyEventReplayRecord {
    pub aggregate_key: AggregateKey,
    pub last_sequence: PolicyEventSequence,
    pub last_event_type: EventType,
    pub last_idempotency_key: IdempotencyKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEventApplyOutcome {
    Advanced(PolicyEventReplayRecord),
    Duplicate(PolicyEventReplayRecord),
    Stale(PolicyEventReplayRecord),
}

impl PolicyEventApplyOutcome {
    pub fn into_record(self) -> PolicyEventReplayRecord {
        match self {
            Self::Advanced(record) | Self::Duplicate(record) | Self::Stale(record) => record,
        }
    }
}

impl PolicyEvent {
    fn scope(&self) -> &PolicyEventScope {
        &self.scope
    }

    pub fn redacted_summary(&self) -> String {
        replay::policy_event_redacted_summary(self)
    }

    pub fn event_type(&self) -> Result<EventType, EventingError> {
        replay::policy_event_event_type(self)
    }

    pub fn contract(&self) -> Result<ocentra_eventing::envelope::EventContract, EventingError> {
        replay::policy_event_contract(self)
    }

    pub fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        replay::policy_event_aggregate_key(self)
    }

    pub fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        replay::policy_event_idempotency_key(self)
    }

    pub fn replay_record(&self) -> Result<PolicyEventReplayRecord, EventingError> {
        replay::policy_event_replay_record(self)
    }
}

impl DomainEvent for PolicyEvent {
    fn contract(&self) -> Result<ocentra_eventing::envelope::EventContract, EventingError> {
        self.contract()
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        self.aggregate_key()
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        self.idempotency_key()
    }
}

pub fn policy_event_schema_version() -> Result<SchemaVersion, EventingError> {
    replay::policy_event_schema_version()
}

pub fn policy_event_family_namespace() -> Result<EventNamespace, EventingError> {
    replay::policy_event_family_namespace()
}

pub fn policy_event_family_variants() -> Result<Vec<EventTopologyFamilyVariant>, EventingError> {
    replay::policy_event_family_variants()
}

pub fn policy_event_contract_registry() -> Result<EventContractRegistry, EventingError> {
    replay::policy_event_contract_registry()
}

pub fn apply_policy_event_replay(
    current: &PolicyEventReplayRecord,
    next: &PolicyEvent,
) -> Result<PolicyEventApplyOutcome, EventingError> {
    replay::apply_policy_event_replay(current, next)
}

impl PolicyEventKind {
    pub fn reason_code_value(self) -> &'static str {
        replay::policy_event_kind_reason_code_value(self)
    }

    pub fn event_type_name(self) -> &'static str {
        kind::policy_event_kind_name(self)
    }
}

pub const POLICY_EVENT_KINDS: &[PolicyEventKind] = kind::POLICY_EVENT_KINDS;

pub(crate) fn policy_event_kinds() -> &'static [PolicyEventKind] {
    kind::policy_event_kinds()
}
