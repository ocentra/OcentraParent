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

const POLICY_EVENT_SCHEMA_VERSION_VALUE: u16 = 1;
const POLICY_EVENT_NAMESPACE_VALUE: &str = "policy";

macro_rules! policy_event_kind_enum {
    ($( $variant:ident => $event_type:literal, )+) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub enum PolicyEventKind {
            $(#[serde(rename = $event_type)] $variant,)+
        }

        pub const POLICY_EVENT_KINDS: &[PolicyEventKind] = &[
            $(PolicyEventKind::$variant,)+
        ];

        impl PolicyEventKind {
            pub fn event_type_name(self) -> &'static str {
                match self {
                    $(Self::$variant => $event_type,)+
                }
            }
        }
    };
}

policy_event_kind_enum!(
    DraftCreated => "policy.draft.created",
    PreviewRequested => "policy.preview.requested",
    PreviewGenerated => "policy.preview.generated",
    Confirmed => "policy.confirmed",
    VersionSuperseded => "policy.version.superseded",
    CompilerRequested => "policy.compiler.requested",
    CompilerCompleted => "policy.compiler.completed",
    DeliveryQueued => "policy.delivery.queued",
    DeliverySent => "policy.delivery.sent",
    DeliveryAcknowledged => "policy.delivery.acknowledged",
    DeliveryRejected => "policy.delivery.rejected",
    DeliveryExpired => "policy.delivery.expired",
    DeliveryRetryScheduled => "policy.delivery.retry-scheduled",
    DomainApplied => "policy.domain.applied",
    DomainPartial => "policy.domain.partial",
    RollbackRequested => "policy.rollback.requested",
    RollbackApplied => "policy.rollback.applied",
    AskParentRequested => "policy.ask-parent.requested",
    AskParentApproved => "policy.ask-parent.approved",
    AskParentDenied => "policy.ask-parent.denied",
    OverrideCreated => "policy.override.created",
    OverrideExpired => "policy.override.expired",
    AuditRecorded => "policy.audit.recorded",
    DeadLetterRecorded => "policy.dead-letter.recorded",
    ManualRequired => "policy.manual-required",
);

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
        match self {
            Self::SourceDocument { .. } => "source-document",
            Self::Request { .. } => "request",
            Self::Approval { .. } => "approval",
            Self::Override { .. } => "override",
            Self::Delivery { .. } => "delivery",
            Self::Rollback { .. } => "rollback",
            Self::Audit { .. } => "audit",
        }
    }

    pub fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_key_value())
    }

    fn aggregate_key_value(&self) -> String {
        match self {
            Self::SourceDocument {
                household_id,
                source_document_id,
                policy_version,
            } => aggregate_key_value(&[
                "policy-source",
                household_id.as_str(),
                source_document_id.as_str(),
                &policy_version.value().to_string(),
            ]),
            Self::Request {
                household_id,
                request_id,
                policy_version,
                ..
            } => aggregate_key_value(&[
                "policy-request",
                household_id.as_str(),
                request_id.as_str(),
                &policy_version.value().to_string(),
            ]),
            Self::Approval {
                household_id,
                approval_id,
                request_id,
                policy_version,
                ..
            } => aggregate_key_value(&[
                "policy-approval",
                household_id.as_str(),
                approval_id.as_str(),
                request_id.as_str(),
                &policy_version.value().to_string(),
            ]),
            Self::Override {
                household_id,
                override_id,
                approval_id,
                request_id,
                policy_version,
                ..
            } => aggregate_key_value(&[
                "policy-override",
                household_id.as_str(),
                override_id.as_str(),
                approval_id.as_str(),
                request_id.as_str(),
                &policy_version.value().to_string(),
            ]),
            Self::Delivery {
                household_id,
                delivery_id,
                child_profile_id,
                device_id,
                domain,
                policy_version,
                ..
            } => aggregate_key_value(&[
                "policy-delivery",
                household_id.as_str(),
                delivery_id.as_str(),
                child_profile_id.as_str(),
                device_id.as_str(),
                policy_event_domain_name(*domain),
                &policy_version.value().to_string(),
            ]),
            Self::Rollback {
                household_id,
                rollback_ref,
            } => aggregate_key_value(&[
                "policy-rollback",
                household_id.as_str(),
                rollback_ref.rolled_back_document_id.as_str(),
                &rollback_ref.rolled_back_policy_version.value().to_string(),
                rollback_ref.restored_document_id.as_str(),
                &rollback_ref.restored_policy_version.value().to_string(),
            ]),
            Self::Audit {
                household_id,
                audit_reference_id,
                source_document_id,
                policy_version,
            } => aggregate_key_value(&[
                "policy-audit",
                household_id.as_str(),
                audit_reference_id.as_str(),
                source_document_id.as_str(),
                &policy_version.value().to_string(),
            ]),
        }
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
    pub fn redacted_summary(&self) -> String {
        let mut value = String::from("policy-event kind=");
        value.push_str(self.kind.event_type_name());
        value.push_str(" scope=");
        value.push_str(self.scope.family_label());
        value.push_str(" sequence=");
        value.push_str(&self.sequence.value().to_string());
        if matches!(self.kind, PolicyEventKind::ManualRequired) {
            value.push_str(" manual-required");
        }
        if matches!(self.kind, PolicyEventKind::DeadLetterRecorded) {
            value.push_str(" dead-lettered");
        }
        value
    }

    pub fn event_type(&self) -> Result<EventType, EventingError> {
        EventType::parse(self.kind.event_type_name())
    }

    pub fn contract(&self) -> Result<ocentra_eventing::envelope::EventContract, EventingError> {
        Ok(ocentra_eventing::envelope::EventContract::new(
            self.event_type()?,
            self.schema_version,
        ))
    }

    pub fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        self.scope.aggregate_key()
    }

    pub fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(self.idempotency_key_value()?)
    }

    pub fn replay_record(&self) -> Result<PolicyEventReplayRecord, EventingError> {
        Ok(PolicyEventReplayRecord {
            aggregate_key: self.aggregate_key()?,
            last_sequence: self.sequence,
            last_event_type: self.event_type()?,
            last_idempotency_key: self.idempotency_key()?,
        })
    }

    fn idempotency_key_value(&self) -> Result<String, EventingError> {
        let aggregate_key = self.aggregate_key()?;
        let mut value = String::from("policy-event:");
        value.push_str(self.kind.event_type_name());
        value.push('|');
        value.push_str(aggregate_key.as_str());
        value.push('|');
        value.push_str(&self.sequence.value().to_string());
        value.push('|');
        value.push_str(self.scope.family_label());
        value.push('|');
        value.push_str(&join_audit_reference_ids(&self.audit_reference_ids));
        value.push('|');
        value.push_str(
            self.reason_code
                .as_ref()
                .map_or("none", PolicyReasonCode::as_str),
        );
        value.push('|');
        value.push_str(
            self.dead_letter_reason
                .as_ref()
                .map_or("none", policy_event_dead_letter_reason_name),
        );
        Ok(value)
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
    SchemaVersion::new(POLICY_EVENT_SCHEMA_VERSION_VALUE)
}

pub fn policy_event_family_namespace() -> Result<EventNamespace, EventingError> {
    EventNamespace::parse(POLICY_EVENT_NAMESPACE_VALUE)
}

pub fn policy_event_family_variants() -> Result<Vec<EventTopologyFamilyVariant>, EventingError> {
    let family = policy_event_family_namespace()?;
    POLICY_EVENT_KINDS
        .iter()
        .copied()
        .map(|kind| {
            Ok(EventTopologyFamilyVariant {
                family: family.clone(),
                event_type: EventType::parse(kind.event_type_name())?,
            })
        })
        .collect()
}

pub fn policy_event_contract_registry() -> Result<EventContractRegistry, EventingError> {
    let mut registry = EventContractRegistry::new();
    for kind in POLICY_EVENT_KINDS.iter().copied() {
        let event = sample_policy_event(kind)?;
        registry.register_event(&event)?;
    }
    Ok(registry)
}

pub fn apply_policy_event_replay(
    current: &PolicyEventReplayRecord,
    next: &PolicyEvent,
) -> Result<PolicyEventApplyOutcome, EventingError> {
    let next_aggregate_key = next.aggregate_key()?;
    let next_idempotency_key = next.idempotency_key()?;

    if next_aggregate_key != current.aggregate_key {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_HOUSEHOLD_ID,
            value: next_aggregate_key.as_str().to_string(),
        });
    }

    match next.sequence.value().cmp(&current.last_sequence.value()) {
        std::cmp::Ordering::Less => Ok(PolicyEventApplyOutcome::Stale(current.clone())),
        std::cmp::Ordering::Equal => {
            if next_idempotency_key == current.last_idempotency_key
                && next.event_type()? == current.last_event_type
            {
                return Ok(PolicyEventApplyOutcome::Duplicate(current.clone()));
            }

            Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_SEQUENCE,
                value: conflicting_replay_value(next.sequence, &current.last_event_type),
            })
        }
        std::cmp::Ordering::Greater => {
            Ok(PolicyEventApplyOutcome::Advanced(PolicyEventReplayRecord {
                aggregate_key: next_aggregate_key,
                last_sequence: next.sequence,
                last_event_type: next.event_type()?,
                last_idempotency_key: next_idempotency_key,
            }))
        }
    }
}

fn sample_policy_event(kind: PolicyEventKind) -> Result<PolicyEvent, EventingError> {
    let scope = sample_policy_event_scope(kind)?;
    let audit_reference_ids = vec![PolicyAuditReferenceId::parse("audit-policy-event")?];
    let reason_code = if kind_requires_reason(kind) {
        Some(PolicyReasonCode::parse(kind.reason_code_value())?)
    } else {
        None
    };
    let dead_letter_reason = if matches!(kind, PolicyEventKind::DeadLetterRecorded) {
        Some(PolicyEventDeadLetterReason::ReplayRejected)
    } else {
        None
    };
    Ok(PolicyEvent {
        schema_version: policy_event_schema_version()?,
        kind,
        sequence: PolicyEventSequence::new(1)?,
        scope,
        audit_reference_ids,
        reason_code,
        dead_letter_reason,
    })
}

fn sample_policy_event_scope(kind: PolicyEventKind) -> Result<PolicyEventScope, EventingError> {
    let household_id = PolicyHouseholdId::parse("household-default")?;
    let source_document_id = ParentPolicyDocumentId::parse("policy-source-default")?;
    let policy_version = PolicyVersion::new(5)?;

    match kind {
        PolicyEventKind::DraftCreated
        | PolicyEventKind::PreviewRequested
        | PolicyEventKind::PreviewGenerated
        | PolicyEventKind::Confirmed
        | PolicyEventKind::VersionSuperseded
        | PolicyEventKind::CompilerRequested
        | PolicyEventKind::CompilerCompleted
        | PolicyEventKind::AuditRecorded
        | PolicyEventKind::DeadLetterRecorded
        | PolicyEventKind::ManualRequired => Ok(PolicyEventScope::SourceDocument {
            household_id,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::AskParentRequested
        | PolicyEventKind::AskParentApproved
        | PolicyEventKind::AskParentDenied => Ok(PolicyEventScope::Request {
            household_id,
            request_id: PolicyRequestId::parse("policy-request-default")?,
            child_profile_id: PolicyChildProfileId::parse("child-primary")?,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::OverrideCreated | PolicyEventKind::OverrideExpired => {
            Ok(PolicyEventScope::Override {
                household_id,
                override_id: PolicyOverrideId::parse("policy-override-default")?,
                approval_id: PolicyApprovalId::parse("policy-approval-default")?,
                request_id: PolicyRequestId::parse("policy-request-default")?,
                source_document_id,
                policy_version,
            })
        }
        PolicyEventKind::DeliveryQueued
        | PolicyEventKind::DeliverySent
        | PolicyEventKind::DeliveryAcknowledged
        | PolicyEventKind::DeliveryRejected
        | PolicyEventKind::DeliveryExpired
        | PolicyEventKind::DeliveryRetryScheduled
        | PolicyEventKind::DomainApplied
        | PolicyEventKind::DomainPartial => Ok(PolicyEventScope::Delivery {
            household_id,
            delivery_id: PolicyDeliveryId::parse("policy-delivery-default")?,
            child_profile_id: PolicyChildProfileId::parse("child-primary")?,
            device_id: PolicyDeviceId::parse("device-laptop")?,
            domain: PolicyConsumerDomain::Tracking,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::RollbackRequested | PolicyEventKind::RollbackApplied => {
            Ok(PolicyEventScope::Rollback {
                household_id,
                rollback_ref: PolicyRollbackRef {
                    household_id: PolicyHouseholdId::parse("household-default")?,
                    rolled_back_document_id: ParentPolicyDocumentId::parse(
                        "policy-source-default",
                    )?,
                    rolled_back_policy_version: PolicyVersion::new(5)?,
                    restored_document_id: ParentPolicyDocumentId::parse("policy-source-previous")?,
                    restored_policy_version: PolicyVersion::new(4)?,
                },
            })
        }
    }
}

fn kind_requires_reason(kind: PolicyEventKind) -> bool {
    matches!(
        kind,
        PolicyEventKind::DeliveryRejected
            | PolicyEventKind::DeliveryExpired
            | PolicyEventKind::DeliveryRetryScheduled
            | PolicyEventKind::DomainPartial
            | PolicyEventKind::AskParentDenied
            | PolicyEventKind::OverrideExpired
            | PolicyEventKind::ManualRequired
            | PolicyEventKind::RollbackApplied
    )
}

impl PolicyEventKind {
    pub fn reason_code_value(self) -> &'static str {
        match self {
            Self::DeliveryRejected => "delivery-rejected",
            Self::DeliveryExpired => "delivery-expired",
            Self::DeliveryRetryScheduled => "delivery-retry-scheduled",
            Self::DomainPartial => "domain-partial",
            Self::AskParentDenied => "ask-parent-denied",
            Self::OverrideExpired => "override-expired",
            Self::ManualRequired => "manual-required",
            Self::RollbackApplied => "rollback-applied",
            _ => "policy-event",
        }
    }
}

fn join_audit_reference_ids(audit_reference_ids: &[PolicyAuditReferenceId]) -> String {
    audit_reference_ids
        .iter()
        .map(PolicyAuditReferenceId::as_str)
        .collect::<Vec<_>>()
        .join(",")
}

fn policy_event_dead_letter_reason_name(reason: &PolicyEventDeadLetterReason) -> &'static str {
    match reason {
        PolicyEventDeadLetterReason::DuplicateIdempotency => "duplicate-idempotency",
        PolicyEventDeadLetterReason::ReplayRejected => "replay-rejected",
        PolicyEventDeadLetterReason::StaleSequence => "stale-sequence",
        PolicyEventDeadLetterReason::UnsupportedTarget => "unsupported-target",
        PolicyEventDeadLetterReason::MissingSubscriber => "missing-subscriber",
        PolicyEventDeadLetterReason::ManualRequired => "manual-required",
    }
}

fn policy_event_domain_name(domain: PolicyConsumerDomain) -> &'static str {
    match domain {
        PolicyConsumerDomain::App => "app",
        PolicyConsumerDomain::Browser => "browser",
        PolicyConsumerDomain::Network => "network",
        PolicyConsumerDomain::Tracking => "tracking",
        PolicyConsumerDomain::Screen => "screen",
        PolicyConsumerDomain::Ai => "ai",
    }
}

fn aggregate_key_value(parts: &[&str]) -> String {
    let mut value = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            value.push(':');
        }
        value.push_str(part);
    }
    value
}

fn conflicting_replay_value(sequence: PolicyEventSequence, last_event_type: &EventType) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_CONFLICTING_REPLAY_FOR_SEQUENCE_PREFIX);
    value.push_str(&sequence.value().to_string());
    value.push_str(policy_control::delivery::VALUE_CONFLICTING_REPLAY_ON_SEPARATOR);
    value.push_str(last_event_type.as_str());
    value
}
