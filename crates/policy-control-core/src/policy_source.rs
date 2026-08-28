#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use serde::{Deserialize, Serialize};

mod lifecycle;
mod names;
mod primitives;
mod validation;

const POLICY_SOURCE_SCHEMA_VERSION_VALUE: u16 = 1;

pub type ParentPolicyDocumentId = primitives::ParentPolicyDocumentId;
pub type PolicyHouseholdId = primitives::PolicyHouseholdId;
pub type PolicyActorId = primitives::PolicyActorId;
pub type PolicyChildProfileId = primitives::PolicyChildProfileId;
pub type PolicyDeviceId = primitives::PolicyDeviceId;
pub type PolicyRuleId = primitives::PolicyRuleId;
pub type PolicyTargetReferenceId = primitives::PolicyTargetReferenceId;
pub type PolicyScheduleId = primitives::PolicyScheduleId;
pub type PolicyTimezoneName = primitives::PolicyTimezoneName;
pub type PolicyReasonCode = primitives::PolicyReasonCode;
pub type PolicyAuditReferenceId = primitives::PolicyAuditReferenceId;
pub type PolicyVersion = primitives::PolicyVersion;

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
    pub source_surface: PolicySourceSurface,
    pub actor_id: PolicyActorId,
    pub actor_role: ParentPolicyActorRole,
    pub status: PolicySourceStatus,
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
    pub status: PolicySourceStatus,
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
    validation::validate_parent_policy_source_document(document)
}

pub(crate) fn assert_policy_utc_timestamp(
    field: &'static str,
    value: &str,
) -> Result<(), EventingError> {
    validation::time::format::assert_utc_timestamp(field, value)
}

pub fn register_parent_policy_source_document(
    existing: Option<&ParentPolicySourceDocument>,
    candidate: ParentPolicySourceDocument,
) -> Result<ParentPolicySourceDocument, EventingError> {
    lifecycle::register_parent_policy_source_document(existing, candidate)
}

pub fn register_parent_policy_source_document_with_authority(
    existing: Option<&ParentPolicySourceDocument>,
    candidate: ParentPolicySourceDocument,
    authority: &PolicySourceActorAuthority,
) -> Result<ParentPolicySourceDocument, EventingError> {
    lifecycle::register_parent_policy_source_document_with_authority(existing, candidate, authority)
}

pub fn mark_parent_policy_source_document_active(
    document: &ParentPolicySourceDocument,
    delivery_results: &[PolicyEnforcementResultArtifact],
) -> Result<ParentPolicySourceDocument, EventingError> {
    lifecycle::mark_parent_policy_source_document_active(document, delivery_results)
}

pub fn supersede_parent_policy_source_document(
    current: &ParentPolicySourceDocument,
    replacement_policy_version: PolicyVersion,
    supersede_audit_reference_id: PolicyAuditReferenceId,
) -> Result<ParentPolicySourceDocument, EventingError> {
    lifecycle::supersede_parent_policy_source_document(
        current,
        replacement_policy_version,
        supersede_audit_reference_id,
    )
}

pub fn rollback_parent_policy_source_document(
    current: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
    rollback_audit_reference_id: PolicyAuditReferenceId,
) -> Result<ParentPolicySourceDocument, EventingError> {
    lifecycle::rollback_parent_policy_source_document(
        current,
        rollback_ref,
        rollback_audit_reference_id,
    )
}

pub fn compile_domain_policy_artifact(
    source: &ParentPolicySourceDocument,
    domain: PolicyConsumerDomain,
) -> Result<CompiledDomainPolicyArtifact, EventingError> {
    lifecycle::compile_domain_policy_artifact(source, domain)
}

pub fn policy_enforcement_result_artifact(
    source: &ParentPolicySourceDocument,
    state: PolicyEnforcementResultState,
) -> Result<PolicyEnforcementResultArtifact, EventingError> {
    lifecycle::policy_enforcement_result_artifact(source, state)
}

pub fn latest_policy_audit_event(
    source: &ParentPolicySourceDocument,
) -> Result<PolicyAuditEvent, EventingError> {
    lifecycle::latest_policy_audit_event(source)
}

pub fn assess_policy_source_compatibility(
    source: &ParentPolicySourceDocument,
    supported_schema_version: SchemaVersion,
    minimum_supported_policy_version: PolicyVersion,
) -> Result<PolicySourceCompatibilityReport, EventingError> {
    lifecycle::assess_policy_source_compatibility(
        source,
        supported_schema_version,
        minimum_supported_policy_version,
    )
}

pub(crate) fn policy_actor_role_name(role: ParentPolicyActorRole) -> &'static str {
    names::policy_actor_role_name(role)
}

pub(crate) fn policy_actor_state_name(state: PolicySourceActorState) -> &'static str {
    names::policy_actor_state_name(state)
}

pub(crate) fn policy_status_name(status: PolicySourceStatus) -> &'static str {
    names::policy_status_name(status)
}
