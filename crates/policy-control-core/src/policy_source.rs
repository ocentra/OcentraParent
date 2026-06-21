#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

const POLICY_SOURCE_SCHEMA_VERSION_VALUE: u16 = 1;

pub type PolicySourceWriteSurface = ocentra_parent_agent_protocol::PolicySourceSurface;
pub type PolicySourceDocumentStatus = ocentra_parent_agent_protocol::PolicySourceStatus;

macro_rules! policy_source_text_id {
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

policy_source_text_id!(
    ParentPolicyDocumentId,
    policy_control::source::FIELD_DOCUMENT_ID
);
policy_source_text_id!(
    PolicyHouseholdId,
    policy_control::source::FIELD_HOUSEHOLD_ID
);
policy_source_text_id!(PolicyActorId, policy_control::source::FIELD_ACTOR_ID);
policy_source_text_id!(
    PolicyChildProfileId,
    policy_control::source::FIELD_CHILD_PROFILE_ID
);
policy_source_text_id!(PolicyDeviceId, policy_control::source::FIELD_DEVICE_ID);
policy_source_text_id!(PolicyRuleId, policy_control::source::FIELD_RULE_ID);
policy_source_text_id!(
    PolicyTargetReferenceId,
    policy_control::source::FIELD_TARGET_REFERENCE_ID
);
policy_source_text_id!(PolicyScheduleId, policy_control::source::FIELD_SCHEDULE_ID);
policy_source_text_id!(
    PolicyTimezoneName,
    policy_control::source::FIELD_TIMEZONE_NAME
);
policy_source_text_id!(PolicyReasonCode, policy_control::source::FIELD_REASON_CODE);
policy_source_text_id!(
    PolicyAuditReferenceId,
    policy_control::source::FIELD_AUDIT_REFERENCE_ID
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct PolicyVersion(u64);

impl PolicyVersion {
    pub fn new(value: u64) -> Result<Self, EventingError> {
        if value == 0 {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_POLICY_VERSION,
                value: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for PolicyVersion {
    type Error = EventingError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PolicyVersion> for u64 {
    fn from(value: PolicyVersion) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentPolicyActorRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "co-parent")]
    CoParent,
    #[serde(rename = "observer")]
    Observer,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "support")]
    Support,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicySourceActorState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySourceActorAuthority {
    pub household_id: PolicyHouseholdId,
    pub actor_id: PolicyActorId,
    pub actor_role: ParentPolicyActorRole,
    pub actor_state: PolicySourceActorState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyTargetKind {
    #[serde(rename = "child-profile")]
    ChildProfile,
    #[serde(rename = "device")]
    Device,
    #[serde(rename = "app")]
    App,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "resource")]
    Resource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRuleAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "time-limit")]
    TimeLimit,
    #[serde(rename = "block")]
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyConsumerDomain {
    #[serde(rename = "app")]
    App,
    #[serde(rename = "browser")]
    Browser,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "tracking")]
    Tracking,
    #[serde(rename = "screen")]
    Screen,
    #[serde(rename = "ai")]
    Ai,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEnforcementResultState {
    #[serde(rename = "pending-delivery")]
    PendingDelivery,
    #[serde(rename = "acknowledged")]
    Acknowledged,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "rolled-back")]
    RolledBack,
    #[serde(rename = "stale")]
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRuleTarget {
    pub kind: PolicyTargetKind,
    pub reference_id: PolicyTargetReferenceId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyScheduleDay {
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "wednesday")]
    Wednesday,
    #[serde(rename = "thursday")]
    Thursday,
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "saturday")]
    Saturday,
    #[serde(rename = "sunday")]
    Sunday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyScheduleBudgetResetKind {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyScheduleBudgetCarryoverMode {
    #[serde(rename = "discard-unused")]
    DiscardUnused,
    #[serde(rename = "carry-forward")]
    CarryForward,
    #[serde(rename = "cap-carryover")]
    CapCarryover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyScheduleClockSource {
    #[serde(rename = "child-device")]
    ChildDevice,
    #[serde(rename = "trusted-service")]
    TrustedService,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyScheduleOfflineRecovery {
    #[serde(rename = "resume-remaining")]
    ResumeRemaining,
    #[serde(rename = "recompute-from-journal")]
    RecomputeFromJournal,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyScheduleBudgetResetRule {
    pub kind: PolicyScheduleBudgetResetKind,
    pub local_time: String,
    pub day: Option<PolicyScheduleDay>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyScheduleBudgetCarryoverRule {
    pub mode: PolicyScheduleBudgetCarryoverMode,
    pub max_minutes: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyScheduleTimeBudget {
    pub budget_window_minutes: u16,
    pub reset: PolicyScheduleBudgetResetRule,
    pub carryover: PolicyScheduleBudgetCarryoverRule,
    pub grace_period_minutes: u16,
    pub effective_from: String,
    pub effective_until: Option<String>,
    pub bonus_expiry_minutes: u16,
    pub clock_source: PolicyScheduleClockSource,
    pub offline_recovery: PolicyScheduleOfflineRecovery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyScheduleWindow {
    pub schedule_id: PolicyScheduleId,
    pub timezone_name: PolicyTimezoneName,
    pub starts_at: String,
    pub ends_at: String,
    pub time_budget: PolicyScheduleTimeBudget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentPolicyRule {
    pub rule_id: PolicyRuleId,
    pub target: PolicyRuleTarget,
    pub action: PolicyRuleAction,
    pub schedule_id: Option<PolicyScheduleId>,
    pub priority: u16,
    pub reason_code: PolicyReasonCode,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRetentionMetadata {
    pub export_allowed: bool,
    pub delete_allowed: bool,
    pub sync_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentPolicySourceDocument {
    pub schema_version: SchemaVersion,
    pub document_id: ParentPolicyDocumentId,
    pub household_id: PolicyHouseholdId,
    pub policy_version: PolicyVersion,
    pub source_surface: PolicySourceWriteSurface,
    pub actor_id: PolicyActorId,
    pub actor_role: ParentPolicyActorRole,
    pub status: PolicySourceDocumentStatus,
    pub child_profile_ids: Vec<PolicyChildProfileId>,
    pub device_ids: Vec<PolicyDeviceId>,
    pub rules: Vec<ParentPolicyRule>,
    pub schedules: Vec<PolicyScheduleWindow>,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    #[serde(default)]
    pub superseded_by_policy_version: Option<PolicyVersion>,
    #[serde(default)]
    pub rollback_ref: Option<PolicyRollbackRef>,
    pub retention: PolicyRetentionMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledDomainPolicyArtifact {
    pub household_id: PolicyHouseholdId,
    pub policy_version: PolicyVersion,
    pub source_document_id: ParentPolicyDocumentId,
    pub domain: PolicyConsumerDomain,
    pub rule_count: usize,
    pub schedules: Vec<PolicyScheduleWindow>,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    pub superseded_by_policy_version: Option<PolicyVersion>,
    pub rollback_ref: Option<PolicyRollbackRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEnforcementResultArtifact {
    pub household_id: PolicyHouseholdId,
    pub policy_version: PolicyVersion,
    pub source_document_id: ParentPolicyDocumentId,
    pub state: PolicyEnforcementResultState,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyAuditEvent {
    pub audit_reference_id: PolicyAuditReferenceId,
    pub household_id: PolicyHouseholdId,
    pub policy_version: PolicyVersion,
    pub actor_id: PolicyActorId,
    pub actor_role: ParentPolicyActorRole,
    pub status: PolicySourceDocumentStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRollbackRef {
    pub household_id: PolicyHouseholdId,
    pub rolled_back_document_id: ParentPolicyDocumentId,
    pub rolled_back_policy_version: PolicyVersion,
    pub restored_document_id: ParentPolicyDocumentId,
    pub restored_policy_version: PolicyVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDocumentCompatibilityState {
    #[serde(rename = "compatible")]
    Compatible,
    #[serde(rename = "migration-required")]
    MigrationRequired,
    #[serde(rename = "unsupported")]
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySourceCompatibilityReport {
    pub source_schema_version: SchemaVersion,
    pub supported_schema_version: SchemaVersion,
    pub source_policy_version: PolicyVersion,
    pub minimum_supported_policy_version: PolicyVersion,
    pub schema_state: PolicyDocumentCompatibilityState,
    pub policy_version_state: PolicyDocumentCompatibilityState,
}

pub fn parent_policy_source_schema_version() -> Result<SchemaVersion, EventingError> {
    SchemaVersion::new(POLICY_SOURCE_SCHEMA_VERSION_VALUE)
}

pub fn validate_parent_policy_source_document(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    assert_write_surface_can_author_source_truth(document.source_surface)?;
    assert_actor_role_can_author_source_truth(document.actor_role)?;
    assert_audit_refs_match_status(document)?;
    assert_status_lifecycle_refs(document)?;
    assert_schedule_windows(document)?;
    assert_unique_schedule_ids(document)?;
    assert_unique_rule_ids(document)?;
    assert_rule_schedule_refs(document)?;
    assert_active_policy_has_rules(document)?;
    Ok(())
}

pub fn register_parent_policy_source_document(
    existing: Option<&ParentPolicySourceDocument>,
    candidate: ParentPolicySourceDocument,
) -> Result<ParentPolicySourceDocument, EventingError> {
    validate_parent_policy_source_document(&candidate)?;

    if let Some(current) = existing {
        if current.household_id == candidate.household_id {
            if candidate.policy_version.value() < current.policy_version.value() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_POLICY_VERSION,
                    value: stale_policy_version_value(
                        candidate.policy_version,
                        current.policy_version,
                    ),
                });
            }

            if candidate.policy_version.value() == current.policy_version.value()
                && candidate.document_id != current.document_id
            {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_DOCUMENT_ID,
                    value: duplicate_source_truth_value(
                        &candidate.household_id,
                        candidate.policy_version,
                    ),
                });
            }
        }
    }

    Ok(candidate)
}

pub fn register_parent_policy_source_document_with_authority(
    existing: Option<&ParentPolicySourceDocument>,
    candidate: ParentPolicySourceDocument,
    authority: &PolicySourceActorAuthority,
) -> Result<ParentPolicySourceDocument, EventingError> {
    assert_actor_authority_matches_document(&candidate, authority)?;
    register_parent_policy_source_document(existing, candidate)
}

pub fn mark_parent_policy_source_document_active(
    document: &ParentPolicySourceDocument,
    delivery_results: &[PolicyEnforcementResultArtifact],
) -> Result<ParentPolicySourceDocument, EventingError> {
    validate_parent_policy_source_document(document)?;

    if delivery_results.is_empty() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_STATUS,
            value: policy_control::source::VALUE_ACTIVE_POLICY_REQUIRES_ACKNOWLEDGED_DELIVERY
                .to_string(),
        });
    }

    for delivery_result in delivery_results {
        if delivery_result.household_id != document.household_id {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_HOUSEHOLD_ID,
                value: delivery_result.household_id.as_str().to_string(),
            });
        }
        if delivery_result.source_document_id != document.document_id {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_DOCUMENT_ID,
                value: delivery_result.source_document_id.as_str().to_string(),
            });
        }
        if delivery_result.policy_version != document.policy_version {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_POLICY_VERSION,
                value: delivery_result.policy_version.value().to_string(),
            });
        }
        if delivery_result.state != PolicyEnforcementResultState::Acknowledged {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_STATUS,
                value: policy_control::source::VALUE_ACTIVE_POLICY_REQUIRES_ACKNOWLEDGED_DELIVERY
                    .to_string(),
            });
        }
    }

    let mut activated = document.clone();
    activated.status = PolicySourceDocumentStatus::Active;
    validate_parent_policy_source_document(&activated)?;
    Ok(activated)
}

pub fn supersede_parent_policy_source_document(
    current: &ParentPolicySourceDocument,
    replacement_policy_version: PolicyVersion,
    supersede_audit_reference_id: PolicyAuditReferenceId,
) -> Result<ParentPolicySourceDocument, EventingError> {
    validate_parent_policy_source_document(current)?;

    if replacement_policy_version.value() <= current.policy_version.value() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: replacement_policy_version_must_be_newer_value(
                replacement_policy_version,
                current.policy_version,
            ),
        });
    }

    if current
        .audit_reference_ids
        .contains(&supersede_audit_reference_id)
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_AUDIT_REFERENCE_ID,
            value: supersede_audit_reference_id.as_str().to_string(),
        });
    }

    let mut superseded = current.clone();
    superseded.status = PolicySourceDocumentStatus::Superseded;
    superseded.superseded_by_policy_version = Some(replacement_policy_version);
    superseded.rollback_ref = None;
    superseded
        .audit_reference_ids
        .push(supersede_audit_reference_id);
    validate_parent_policy_source_document(&superseded)?;
    Ok(superseded)
}

pub fn rollback_parent_policy_source_document(
    current: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
    rollback_audit_reference_id: PolicyAuditReferenceId,
) -> Result<ParentPolicySourceDocument, EventingError> {
    validate_parent_policy_source_document(current)?;

    if rollback_ref.household_id != current.household_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_HOUSEHOLD_ID,
            value: rollback_ref.household_id.as_str().to_string(),
        });
    }

    if rollback_ref.rolled_back_document_id != current.document_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_DOCUMENT_ID,
            value: rollback_ref.rolled_back_document_id.as_str().to_string(),
        });
    }

    if rollback_ref.rolled_back_policy_version != current.policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
            value: rollback_ref.rolled_back_policy_version.value().to_string(),
        });
    }

    if rollback_ref.restored_document_id == current.document_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RESTORED_DOCUMENT_ID,
            value: rollback_ref.restored_document_id.as_str().to_string(),
        });
    }

    if rollback_ref.restored_policy_version.value() >= current.policy_version.value() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RESTORED_POLICY_VERSION,
            value: restored_policy_version_must_be_older_value(
                rollback_ref.restored_policy_version,
                current.policy_version,
            ),
        });
    }

    if current
        .audit_reference_ids
        .contains(&rollback_audit_reference_id)
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_AUDIT_REFERENCE_ID,
            value: rollback_audit_reference_id.as_str().to_string(),
        });
    }

    let mut rolled_back = current.clone();
    rolled_back.status = PolicySourceDocumentStatus::RolledBack;
    rolled_back.superseded_by_policy_version = None;
    rolled_back.rollback_ref = Some(rollback_ref.clone());
    rolled_back
        .audit_reference_ids
        .push(rollback_audit_reference_id);
    validate_parent_policy_source_document(&rolled_back)?;
    Ok(rolled_back)
}

pub fn compile_domain_policy_artifact(
    source: &ParentPolicySourceDocument,
    domain: PolicyConsumerDomain,
) -> Result<CompiledDomainPolicyArtifact, EventingError> {
    validate_parent_policy_source_document(source)?;
    assert_source_status_can_compile(source.status)?;
    Ok(CompiledDomainPolicyArtifact {
        household_id: source.household_id.clone(),
        policy_version: source.policy_version,
        source_document_id: source.document_id.clone(),
        domain,
        rule_count: source.rules.len(),
        schedules: source.schedules.clone(),
        audit_reference_ids: source.audit_reference_ids.clone(),
        superseded_by_policy_version: source.superseded_by_policy_version,
        rollback_ref: source.rollback_ref.clone(),
    })
}

pub fn policy_enforcement_result_artifact(
    source: &ParentPolicySourceDocument,
    state: PolicyEnforcementResultState,
) -> Result<PolicyEnforcementResultArtifact, EventingError> {
    validate_parent_policy_source_document(source)?;
    Ok(PolicyEnforcementResultArtifact {
        household_id: source.household_id.clone(),
        policy_version: source.policy_version,
        source_document_id: source.document_id.clone(),
        state,
        audit_reference_ids: source.audit_reference_ids.clone(),
    })
}

pub fn latest_policy_audit_event(
    source: &ParentPolicySourceDocument,
) -> Result<PolicyAuditEvent, EventingError> {
    validate_parent_policy_source_document(source)?;
    let audit_reference_id =
        source
            .audit_reference_ids
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: policy_control::source::FIELD_AUDIT_REFERENCE_IDS,
                value: missing_audit_reference_for_status_value(source.status),
            })?;

    Ok(PolicyAuditEvent {
        audit_reference_id,
        household_id: source.household_id.clone(),
        policy_version: source.policy_version,
        actor_id: source.actor_id.clone(),
        actor_role: source.actor_role,
        status: source.status,
    })
}

pub fn assess_policy_source_compatibility(
    source: &ParentPolicySourceDocument,
    supported_schema_version: SchemaVersion,
    minimum_supported_policy_version: PolicyVersion,
) -> Result<PolicySourceCompatibilityReport, EventingError> {
    validate_parent_policy_source_document(source)?;

    let schema_state = if source.schema_version.value() == supported_schema_version.value() {
        PolicyDocumentCompatibilityState::Compatible
    } else if source.schema_version.value() < supported_schema_version.value() {
        PolicyDocumentCompatibilityState::MigrationRequired
    } else {
        PolicyDocumentCompatibilityState::Unsupported
    };

    let policy_version_state =
        if source.policy_version.value() < minimum_supported_policy_version.value() {
            PolicyDocumentCompatibilityState::MigrationRequired
        } else {
            PolicyDocumentCompatibilityState::Compatible
        };

    Ok(PolicySourceCompatibilityReport {
        source_schema_version: source.schema_version,
        supported_schema_version,
        source_policy_version: source.policy_version,
        minimum_supported_policy_version,
        schema_state,
        policy_version_state,
    })
}

fn assert_write_surface_can_author_source_truth(
    surface: PolicySourceWriteSurface,
) -> Result<(), EventingError> {
    if matches!(
        surface,
        PolicySourceWriteSurface::ParentPortal | PolicySourceWriteSurface::ParentCompanion
    ) {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::source::FIELD_SOURCE_SURFACE,
        value: policy_surface_name(surface).to_string(),
    })
}

fn assert_actor_role_can_author_source_truth(
    role: ParentPolicyActorRole,
) -> Result<(), EventingError> {
    if matches!(
        role,
        ParentPolicyActorRole::Parent | ParentPolicyActorRole::CoParent
    ) {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::source::FIELD_ACTOR_ROLE,
        value: policy_actor_role_name(role).to_string(),
    })
}

fn assert_actor_authority_matches_document(
    document: &ParentPolicySourceDocument,
    authority: &PolicySourceActorAuthority,
) -> Result<(), EventingError> {
    if authority.household_id != document.household_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_HOUSEHOLD_ID,
            value: authority.household_id.as_str().to_string(),
        });
    }

    if authority.actor_id != document.actor_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ACTOR_ID,
            value: authority.actor_id.as_str().to_string(),
        });
    }

    if authority.actor_role != document.actor_role {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ACTOR_ROLE,
            value: policy_actor_role_name(authority.actor_role).to_string(),
        });
    }

    if authority.actor_state != PolicySourceActorState::Active {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ACTOR_STATE,
            value: policy_actor_state_name(authority.actor_state).to_string(),
        });
    }

    Ok(())
}

fn assert_audit_refs_match_status(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    if policy_status_requires_audit_refs(document.status) && document.audit_reference_ids.is_empty()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_AUDIT_REFERENCE_IDS,
            value: missing_audit_references_for_status_value(document.status),
        });
    }

    Ok(())
}

fn assert_status_lifecycle_refs(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    match document.status {
        PolicySourceDocumentStatus::Superseded => {
            let replacement_policy_version =
                document.superseded_by_policy_version.ok_or_else(|| {
                    EventingError::InvalidValue {
                        field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                        value: policy_status_name(document.status).to_string(),
                    }
                })?;

            if replacement_policy_version.value() <= document.policy_version.value() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                    value: replacement_policy_version_must_be_newer_value(
                        replacement_policy_version,
                        document.policy_version,
                    ),
                });
            }

            if let Some(rollback_ref) = &document.rollback_ref {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
                    value: rollback_ref.rolled_back_policy_version.value().to_string(),
                });
            }
        }
        PolicySourceDocumentStatus::RolledBack => {
            let rollback_ref =
                document
                    .rollback_ref
                    .as_ref()
                    .ok_or_else(|| EventingError::InvalidValue {
                        field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
                        value: policy_status_name(document.status).to_string(),
                    })?;

            assert_rollback_ref_matches_document(document, rollback_ref)?;

            if let Some(replacement_policy_version) = document.superseded_by_policy_version {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                    value: replacement_policy_version.value().to_string(),
                });
            }
        }
        _ => {
            if let Some(replacement_policy_version) = document.superseded_by_policy_version {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                    value: replacement_policy_version.value().to_string(),
                });
            }

            if let Some(rollback_ref) = &document.rollback_ref {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
                    value: rollback_ref.rolled_back_policy_version.value().to_string(),
                });
            }
        }
    }

    Ok(())
}

fn assert_rollback_ref_matches_document(
    document: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
) -> Result<(), EventingError> {
    if rollback_ref.household_id != document.household_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_HOUSEHOLD_ID,
            value: rollback_ref.household_id.as_str().to_string(),
        });
    }

    if rollback_ref.rolled_back_document_id != document.document_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_DOCUMENT_ID,
            value: rollback_ref.rolled_back_document_id.as_str().to_string(),
        });
    }

    if rollback_ref.rolled_back_policy_version != document.policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
            value: rollback_ref.rolled_back_policy_version.value().to_string(),
        });
    }

    if rollback_ref.restored_document_id == document.document_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RESTORED_DOCUMENT_ID,
            value: rollback_ref.restored_document_id.as_str().to_string(),
        });
    }

    if rollback_ref.restored_policy_version.value() >= document.policy_version.value() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RESTORED_POLICY_VERSION,
            value: restored_policy_version_must_be_older_value(
                rollback_ref.restored_policy_version,
                document.policy_version,
            ),
        });
    }

    Ok(())
}

fn assert_unique_schedule_ids(document: &ParentPolicySourceDocument) -> Result<(), EventingError> {
    let mut seen = BTreeSet::new();
    for schedule in &document.schedules {
        if !seen.insert(schedule.schedule_id.clone()) {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_SCHEDULE_ID,
                value: schedule.schedule_id.as_str().to_string(),
            });
        }
    }
    Ok(())
}

fn assert_schedule_windows(document: &ParentPolicySourceDocument) -> Result<(), EventingError> {
    for schedule in &document.schedules {
        assert_local_time(
            policy_control::source::FIELD_SCHEDULE_STARTS_AT,
            &schedule.starts_at,
        )?;
        assert_local_time(
            policy_control::source::FIELD_SCHEDULE_ENDS_AT,
            &schedule.ends_at,
        )?;
        assert_schedule_time_budget(&schedule.time_budget)?;
    }

    Ok(())
}

fn assert_unique_rule_ids(document: &ParentPolicySourceDocument) -> Result<(), EventingError> {
    let mut seen = BTreeSet::new();
    for rule in &document.rules {
        if !seen.insert(rule.rule_id.clone()) {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_RULE_ID,
                value: rule.rule_id.as_str().to_string(),
            });
        }
    }
    Ok(())
}

fn assert_rule_schedule_refs(document: &ParentPolicySourceDocument) -> Result<(), EventingError> {
    let schedule_ids = document
        .schedules
        .iter()
        .map(|schedule| schedule.schedule_id.clone())
        .collect::<BTreeSet<_>>();

    for rule in &document.rules {
        if rule.action == PolicyRuleAction::TimeLimit && rule.schedule_id.is_none() {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_RULE_SCHEDULE_ID,
                value: rule.rule_id.as_str().to_string(),
            });
        }
        if let Some(schedule_id) = &rule.schedule_id {
            if !schedule_ids.contains(schedule_id) {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_RULE_SCHEDULE_ID,
                    value: schedule_id.as_str().to_string(),
                });
            }
        }
    }

    Ok(())
}

fn assert_schedule_time_budget(budget: &PolicyScheduleTimeBudget) -> Result<(), EventingError> {
    if budget.budget_window_minutes == 0 {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SCHEDULE_BUDGET_WINDOW_MINUTES,
            value: budget.budget_window_minutes.to_string(),
        });
    }

    if budget.bonus_expiry_minutes == 0 {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SCHEDULE_BONUS_EXPIRY_MINUTES,
            value: budget.bonus_expiry_minutes.to_string(),
        });
    }

    assert_local_time(
        policy_control::source::FIELD_SCHEDULE_RESET_LOCAL_TIME,
        &budget.reset.local_time,
    )?;
    assert_utc_timestamp(
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_FROM,
        &budget.effective_from,
    )?;

    if let Some(effective_until) = &budget.effective_until {
        assert_utc_timestamp(
            policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
            effective_until,
        )?;
        if effective_until <= &budget.effective_from {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
                value: effective_until.clone(),
            });
        }
    }

    match budget.reset.kind {
        PolicyScheduleBudgetResetKind::Weekly => {
            if budget.reset.day.is_none() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_RESET_DAY,
                    value: "missing-weekly-reset-day".to_string(),
                });
            }
        }
        PolicyScheduleBudgetResetKind::Daily | PolicyScheduleBudgetResetKind::Monthly => {
            if budget.reset.day.is_some() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_RESET_DAY,
                    value: "unexpected-reset-day".to_string(),
                });
            }
        }
    }

    match budget.carryover.mode {
        PolicyScheduleBudgetCarryoverMode::DiscardUnused => {
            if budget.carryover.max_minutes.is_some() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
                    value: "discard-unused".to_string(),
                });
            }
        }
        PolicyScheduleBudgetCarryoverMode::CapCarryover => {
            if budget.carryover.max_minutes.unwrap_or(0) == 0 {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
                    value: "cap-carryover".to_string(),
                });
            }
        }
        PolicyScheduleBudgetCarryoverMode::CarryForward => {}
    }

    Ok(())
}

fn assert_local_time(field: &'static str, value: &str) -> Result<(), EventingError> {
    if value.len() != 5 || !value.is_ascii() || value.as_bytes()[2] != b':' {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    let hour = parse_time_component(field, &value[0..2])?;
    let minute = parse_time_component(field, &value[3..5])?;
    if hour > 23 || minute > 59 {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    Ok(())
}

fn assert_utc_timestamp(field: &'static str, value: &str) -> Result<(), EventingError> {
    if value.len() != 20
        || !value.is_ascii()
        || value.as_bytes()[4] != b'-'
        || value.as_bytes()[7] != b'-'
        || value.as_bytes()[10] != b'T'
        || value.as_bytes()[13] != b':'
        || value.as_bytes()[16] != b':'
        || value.as_bytes()[19] != b'Z'
    {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    let month = parse_time_component(field, &value[5..7])?;
    let day = parse_time_component(field, &value[8..10])?;
    if month == 0 || month > 12 || day == 0 || day > 31 {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    assert_local_time(field, &value[11..16])?;
    let seconds = parse_time_component(field, &value[17..19])?;
    if seconds > 59 {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    Ok(())
}

fn parse_time_component(field: &'static str, value: &str) -> Result<u8, EventingError> {
    value
        .parse::<u8>()
        .map_err(|_error| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })
}

fn assert_active_policy_has_rules(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    if document.status == PolicySourceDocumentStatus::Active && document.rules.is_empty() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RULES,
            value: policy_control::source::VALUE_ACTIVE_POLICY_HAS_NO_RULES.to_string(),
        });
    }

    Ok(())
}

fn assert_source_status_can_compile(
    status: PolicySourceDocumentStatus,
) -> Result<(), EventingError> {
    if matches!(
        status,
        PolicySourceDocumentStatus::Draft | PolicySourceDocumentStatus::Preview
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_STATUS,
            value: policy_status_name(status).to_string(),
        });
    }

    Ok(())
}

fn policy_status_requires_audit_refs(status: PolicySourceDocumentStatus) -> bool {
    !matches!(
        status,
        PolicySourceDocumentStatus::Draft | PolicySourceDocumentStatus::Preview
    )
}

fn policy_surface_name(surface: PolicySourceWriteSurface) -> &'static str {
    match surface {
        PolicySourceWriteSurface::ParentPortal => policy_control::source::SURFACE_PARENT_PORTAL,
        PolicySourceWriteSurface::ParentCompanion => {
            policy_control::source::SURFACE_PARENT_COMPANION
        }
        PolicySourceWriteSurface::AiPreview => policy_control::source::SURFACE_AI_PREVIEW,
        PolicySourceWriteSurface::DomainCache => policy_control::source::SURFACE_DOMAIN_CACHE,
    }
}

pub(crate) fn policy_actor_role_name(role: ParentPolicyActorRole) -> &'static str {
    match role {
        ParentPolicyActorRole::Parent => policy_control::source::ROLE_PARENT,
        ParentPolicyActorRole::CoParent => policy_control::source::ROLE_CO_PARENT,
        ParentPolicyActorRole::Observer => policy_control::source::ROLE_OBSERVER,
        ParentPolicyActorRole::Child => policy_control::source::ROLE_CHILD,
        ParentPolicyActorRole::Support => policy_control::source::ROLE_SUPPORT,
    }
}

pub(crate) fn policy_actor_state_name(state: PolicySourceActorState) -> &'static str {
    match state {
        PolicySourceActorState::Active => policy_control::source::ACTOR_STATE_ACTIVE,
        PolicySourceActorState::Revoked => policy_control::source::ACTOR_STATE_REVOKED,
    }
}

pub(crate) fn policy_status_name(status: PolicySourceDocumentStatus) -> &'static str {
    match status {
        PolicySourceDocumentStatus::Draft => policy_control::source::STATUS_DRAFT,
        PolicySourceDocumentStatus::Preview => policy_control::source::STATUS_PREVIEW,
        PolicySourceDocumentStatus::Confirmed => policy_control::source::STATUS_CONFIRMED,
        PolicySourceDocumentStatus::Queued => policy_control::source::STATUS_QUEUED,
        PolicySourceDocumentStatus::Delivered => policy_control::source::STATUS_DELIVERED,
        PolicySourceDocumentStatus::Acknowledged => policy_control::source::STATUS_ACKNOWLEDGED,
        PolicySourceDocumentStatus::Active => policy_control::source::STATUS_ACTIVE,
        PolicySourceDocumentStatus::PartiallyActive => {
            policy_control::source::STATUS_PARTIALLY_ACTIVE
        }
        PolicySourceDocumentStatus::Rejected => policy_control::source::STATUS_REJECTED,
        PolicySourceDocumentStatus::Superseded => policy_control::source::STATUS_SUPERSEDED,
        PolicySourceDocumentStatus::RolledBack => policy_control::source::STATUS_ROLLED_BACK,
        PolicySourceDocumentStatus::Stale => policy_control::source::STATUS_STALE,
        PolicySourceDocumentStatus::Expired => policy_control::source::STATUS_EXPIRED,
        PolicySourceDocumentStatus::ManualRequired => {
            policy_control::source::STATUS_MANUAL_REQUIRED
        }
    }
}

fn stale_policy_version_value(
    candidate_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_STALE_POLICY_VERSION_PREFIX);
    value.push_str(&candidate_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_STALE_POLICY_VERSION_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}

fn duplicate_source_truth_value(
    household_id: &PolicyHouseholdId,
    policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_DUPLICATE_SOURCE_TRUTH_PREFIX);
    value.push_str(household_id.as_str());
    value.push_str(policy_control::source::VALUE_DUPLICATE_SOURCE_TRUTH_VERSION_SEPARATOR);
    value.push_str(&policy_version.value().to_string());
    value
}

fn missing_audit_reference_for_status_value(status: PolicySourceDocumentStatus) -> String {
    let mut value =
        String::from(policy_control::source::VALUE_MISSING_AUDIT_REFERENCE_FOR_STATUS_PREFIX);
    value.push_str(policy_status_name(status));
    value
}

fn missing_audit_references_for_status_value(status: PolicySourceDocumentStatus) -> String {
    let mut value =
        String::from(policy_control::source::VALUE_MISSING_AUDIT_REFERENCES_FOR_STATUS_PREFIX);
    value.push_str(policy_status_name(status));
    value
}

fn replacement_policy_version_must_be_newer_value(
    replacement_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_REPLACEMENT_POLICY_VERSION_PREFIX);
    value.push_str(&replacement_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_MUST_BE_NEWER_THAN_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}

fn restored_policy_version_must_be_older_value(
    restored_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_RESTORED_POLICY_VERSION_PREFIX);
    value.push_str(&restored_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_MUST_BE_OLDER_THAN_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}
