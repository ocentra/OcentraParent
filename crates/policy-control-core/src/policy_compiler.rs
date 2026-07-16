#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_source::{
    policy_status_name, validate_parent_policy_source_document, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyAuditReferenceId, PolicyChildProfileId,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata,
    PolicyRollbackRef, PolicyRuleAction, PolicyRuleId, PolicyRuleTarget, PolicyScheduleId,
    PolicyScheduleWindow, PolicyTargetKind, PolicyVersion,
};

const POLICY_COMPILER_SCHEMA_VERSION_VALUE: u16 = 1;
const POLICY_TARGET_KIND_COUNT: usize = 6;
const POLICY_COMPILER_DOMAIN_COUNT: usize = 8;
const POLICY_COMPILER_CAPABILITY_STATE_COUNT: usize = 3;
const UNKNOWN_POLICY_TARGET_KIND_NAME: &str = "unknown";

const POLICY_TARGET_KINDS: [PolicyTargetKind; POLICY_TARGET_KIND_COUNT] = [
    PolicyTargetKind::ChildProfile,
    PolicyTargetKind::Device,
    PolicyTargetKind::App,
    PolicyTargetKind::Site,
    PolicyTargetKind::Category,
    PolicyTargetKind::Resource,
];

const POLICY_TARGET_KIND_NAMES: [&str; POLICY_TARGET_KIND_COUNT] = [
    "child-profile",
    "device",
    "app",
    "site",
    "category",
    "resource",
];

const POLICY_COMPILER_DOMAIN_NAMES: [&str; POLICY_COMPILER_DOMAIN_COUNT] = [
    "app-game",
    "browser",
    "network",
    "tracking",
    "screen",
    "ai",
    "enforcement",
    "notification-ask-parent",
];

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyCompiledArtifactId(String);

impl PolicyCompiledArtifactId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        (!value.trim().is_empty())
            .then_some(Self(value))
            .ok_or(EventingError::EmptyValue {
                field: policy_control::compiler::FIELD_COMPILED_ARTIFACT_ID,
            })
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyCompiledArtifactId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyCompiledArtifactId> for String {
    fn from(value: PolicyCompiledArtifactId) -> Self {
        value.0
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyCompilerDomain {
    #[serde(rename = "app-game")]
    AppGame,
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
    #[serde(rename = "enforcement")]
    Enforcement,
    #[serde(rename = "notification-ask-parent")]
    NotificationAskParent,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyCompilerRuleStatus {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unsupported")]
    Unsupported,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyCompilerCapabilityState {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unsupported")]
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyCompilerSupportMatrixRow {
    pub target_kind: PolicyTargetKind,
    pub capability_state: PolicyCompilerCapabilityState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyCompilerSupportMatrix {
    pub domain: PolicyCompilerDomain,
    pub rows: Vec<PolicyCompilerSupportMatrixRow>,
}

type DomainSupportMatrix = [PolicyCompilerCapabilityState; POLICY_TARGET_KIND_COUNT];
type CapabilityStateResolver =
    fn(PolicyCompilerCapabilityState, PolicyRuleAction) -> PolicyCompilerCapabilityState;

const RULE_STATUS_BY_CAPABILITY_STATE: [PolicyCompilerRuleStatus;
    POLICY_COMPILER_CAPABILITY_STATE_COUNT] = [
    PolicyCompilerRuleStatus::Ready,
    PolicyCompilerRuleStatus::ManualRequired,
    PolicyCompilerRuleStatus::Unsupported,
];

const APP_GAME_SUPPORT_MATRIX: DomainSupportMatrix = [
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::ManualRequired,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Unsupported,
];

const BROWSER_SUPPORT_MATRIX: DomainSupportMatrix = [
    PolicyCompilerCapabilityState::ManualRequired,
    PolicyCompilerCapabilityState::ManualRequired,
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::ManualRequired,
];

const NETWORK_SUPPORT_MATRIX: DomainSupportMatrix = [
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::ManualRequired,
    PolicyCompilerCapabilityState::Supported,
];

const TRACKING_SUPPORT_MATRIX: DomainSupportMatrix = [
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Supported,
];

const SCREEN_SUPPORT_MATRIX: DomainSupportMatrix = [
    PolicyCompilerCapabilityState::Unsupported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::ManualRequired,
    PolicyCompilerCapabilityState::ManualRequired,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Unsupported,
];

const FULLY_SUPPORTED_MATRIX: DomainSupportMatrix = [
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Supported,
    PolicyCompilerCapabilityState::Supported,
];

const DOMAIN_SUPPORT_MATRICES: [DomainSupportMatrix; POLICY_COMPILER_DOMAIN_COUNT] = [
    APP_GAME_SUPPORT_MATRIX,
    BROWSER_SUPPORT_MATRIX,
    NETWORK_SUPPORT_MATRIX,
    TRACKING_SUPPORT_MATRIX,
    SCREEN_SUPPORT_MATRIX,
    FULLY_SUPPORTED_MATRIX,
    FULLY_SUPPORTED_MATRIX,
    FULLY_SUPPORTED_MATRIX,
];

const CAPABILITY_STATE_RESOLVERS: [CapabilityStateResolver; POLICY_COMPILER_DOMAIN_COUNT] = [
    passthrough_capability_state,
    passthrough_capability_state,
    passthrough_capability_state,
    passthrough_capability_state,
    passthrough_capability_state,
    passthrough_capability_state,
    enforcement_capability_state,
    notification_ask_parent_capability_state,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainCompiledPolicyRule {
    pub rule_id: PolicyRuleId,
    pub target: PolicyRuleTarget,
    pub action: PolicyRuleAction,
    pub schedule_id: Option<PolicyScheduleId>,
    pub capability_state: PolicyCompilerCapabilityState,
    pub status: PolicyCompilerRuleStatus,
    pub reason_code: Option<PolicyReasonCode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyCompilerDeliveryTarget {
    pub child_profile_ids: Vec<PolicyChildProfileId>,
    pub device_ids: Vec<PolicyDeviceId>,
    pub domain: PolicyCompilerDomain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainCompiledPolicyArtifact {
    pub compiled_artifact_id: PolicyCompiledArtifactId,
    pub compiler_schema_version: SchemaVersion,
    pub household_id: PolicyHouseholdId,
    pub policy_version: PolicyVersion,
    pub consumer_policy_version: PolicyVersion,
    pub source_document_id: ParentPolicyDocumentId,
    pub source_status: PolicySourceStatus,
    pub domain: PolicyCompilerDomain,
    pub delivery_target: PolicyCompilerDeliveryTarget,
    pub support_matrix: PolicyCompilerSupportMatrix,
    pub evidence_custody_requirements: PolicyRetentionMetadata,
    pub no_claim_labels: Vec<String>,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    pub superseded_by_policy_version: Option<PolicyVersion>,
    pub rollback_ref: Option<PolicyRollbackRef>,
    pub schedules: Vec<PolicyScheduleWindow>,
    pub rules: Vec<DomainCompiledPolicyRule>,
}

pub fn policy_compiler_schema_version() -> Result<SchemaVersion, EventingError> {
    SchemaVersion::new(POLICY_COMPILER_SCHEMA_VERSION_VALUE)
}

pub fn compile_app_game_policy(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(
        source,
        consumer_policy_version,
        PolicyCompilerDomain::AppGame,
        default_support_matrix_for_domain(PolicyCompilerDomain::AppGame),
    )
}

pub fn compile_browser_policy(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(
        source,
        consumer_policy_version,
        PolicyCompilerDomain::Browser,
        default_support_matrix_for_domain(PolicyCompilerDomain::Browser),
    )
}

pub fn compile_network_policy(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(
        source,
        consumer_policy_version,
        PolicyCompilerDomain::Network,
        default_support_matrix_for_domain(PolicyCompilerDomain::Network),
    )
}

pub fn compile_tracking_policy(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(
        source,
        consumer_policy_version,
        PolicyCompilerDomain::Tracking,
        default_support_matrix_for_domain(PolicyCompilerDomain::Tracking),
    )
}

pub fn compile_screen_policy(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(
        source,
        consumer_policy_version,
        PolicyCompilerDomain::Screen,
        default_support_matrix_for_domain(PolicyCompilerDomain::Screen),
    )
}

pub fn compile_ai_policy_context(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(
        source,
        consumer_policy_version,
        PolicyCompilerDomain::Ai,
        default_support_matrix_for_domain(PolicyCompilerDomain::Ai),
    )
}

pub fn compile_enforcement_policy_hints(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(
        source,
        consumer_policy_version,
        PolicyCompilerDomain::Enforcement,
        default_support_matrix_for_domain(PolicyCompilerDomain::Enforcement),
    )
}

pub fn compile_notification_ask_parent_policy(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(
        source,
        consumer_policy_version,
        PolicyCompilerDomain::NotificationAskParent,
        default_support_matrix_for_domain(PolicyCompilerDomain::NotificationAskParent),
    )
}

pub fn compile_domain_policy_with_support_matrix(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
    domain: PolicyCompilerDomain,
    support_matrix: PolicyCompilerSupportMatrix,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    compile_domain_policy(source, consumer_policy_version, domain, support_matrix)
}

fn compile_domain_policy(
    source: &ParentPolicySourceDocument,
    consumer_policy_version: PolicyVersion,
    domain: PolicyCompilerDomain,
    support_matrix: PolicyCompilerSupportMatrix,
) -> Result<DomainCompiledPolicyArtifact, EventingError> {
    validate_parent_policy_source_document(source)?;
    assert_source_status_can_compile(source.source_surface, source.status)?;
    assert_compiler_version_compatible(source.policy_version, consumer_policy_version)?;
    assert_support_matrix_matches_domain(&support_matrix, domain)?;

    let rules = source
        .rules
        .iter()
        .cloned()
        .map(|rule| compile_rule_for_domain(rule, domain, &support_matrix))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DomainCompiledPolicyArtifact {
        compiled_artifact_id: compiled_artifact_id_for(source, domain)?,
        compiler_schema_version: policy_compiler_schema_version()?,
        household_id: source.household_id.clone(),
        policy_version: source.policy_version,
        consumer_policy_version,
        source_document_id: source.document_id.clone(),
        source_status: source.status,
        domain,
        delivery_target: PolicyCompilerDeliveryTarget {
            child_profile_ids: source.child_profile_ids.clone(),
            device_ids: source.device_ids.clone(),
            domain,
        },
        support_matrix,
        evidence_custody_requirements: source.retention.clone(),
        no_claim_labels: no_claim_labels_for_domain(domain),
        audit_reference_ids: source.audit_reference_ids.clone(),
        superseded_by_policy_version: source.superseded_by_policy_version,
        rollback_ref: source.rollback_ref.clone(),
        schedules: source.schedules.clone(),
        rules,
    })
}

fn compiled_artifact_id_for(
    source: &ParentPolicySourceDocument,
    domain: PolicyCompilerDomain,
) -> Result<PolicyCompiledArtifactId, EventingError> {
    PolicyCompiledArtifactId::parse(format!(
        "policy-compiler:{}:{}:{}",
        compiler_domain_name(domain),
        source.document_id.as_str(),
        source.policy_version.value()
    ))
}

fn no_claim_labels_for_domain(_domain: PolicyCompilerDomain) -> Vec<String> {
    vec![
        policy_control::compiler::NO_CLAIM_COMPILED_ARTIFACT_NOT_SOURCE_TRUTH.to_string(),
        policy_control::compiler::NO_CLAIM_RUNTIME_MUTATION.to_string(),
        policy_control::compiler::NO_CLAIM_ENFORCEMENT.to_string(),
        policy_control::compiler::NO_CLAIM_UI_DELIVERY.to_string(),
        policy_control::compiler::NO_CLAIM_PLATFORM_SUPPORT.to_string(),
    ]
}

fn default_support_matrix_for_domain(domain: PolicyCompilerDomain) -> PolicyCompilerSupportMatrix {
    let rows = POLICY_TARGET_KINDS
        .into_iter()
        .zip(domain_support_matrix(domain))
        .map(
            |(target_kind, capability_state)| PolicyCompilerSupportMatrixRow {
                target_kind,
                capability_state,
            },
        )
        .collect();

    PolicyCompilerSupportMatrix { domain, rows }
}

fn assert_support_matrix_matches_domain(
    support_matrix: &PolicyCompilerSupportMatrix,
    domain: PolicyCompilerDomain,
) -> Result<(), EventingError> {
    (support_matrix.domain == domain)
        .then_some(())
        .ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::compiler::FIELD_SUPPORT_MATRIX_DOMAIN,
            value: compiler_domain_name(support_matrix.domain).to_string(),
        })?;

    POLICY_TARGET_KINDS
        .into_iter()
        .find(|target_kind| {
            support_matrix
                .rows
                .iter()
                .filter(|row| row.target_kind == *target_kind)
                .count()
                != 1
        })
        .map_or(Ok(()), |target_kind| {
            Err(EventingError::InvalidValue {
                field: policy_control::compiler::FIELD_SUPPORT_MATRIX_TARGET_KIND,
                value: policy_target_kind_name(target_kind).to_string(),
            })
        })
}

fn assert_source_status_can_compile(
    surface: PolicySourceSurface,
    status: PolicySourceStatus,
) -> Result<(), EventingError> {
    (!matches!(surface, PolicySourceSurface::DomainCache))
        .then_some(())
        .ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::source::FIELD_SOURCE_SURFACE,
            value: policy_control::source::SURFACE_DOMAIN_CACHE.to_string(),
        })?;

    (!matches!(
        status,
        PolicySourceStatus::Draft | PolicySourceStatus::Preview
    ))
    .then_some(())
    .ok_or_else(|| EventingError::InvalidValue {
        field: policy_control::compiler::FIELD_SOURCE_STATUS,
        value: policy_status_name(status).to_string(),
    })
}

fn assert_compiler_version_compatible(
    source_policy_version: PolicyVersion,
    consumer_policy_version: PolicyVersion,
) -> Result<(), EventingError> {
    (source_policy_version == consumer_policy_version)
        .then_some(())
        .ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::compiler::FIELD_CONSUMER_POLICY_VERSION,
            value: compiler_version_mismatch_value(source_policy_version, consumer_policy_version),
        })
}

fn compile_rule_for_domain(
    rule: ParentPolicyRule,
    domain: PolicyCompilerDomain,
    support_matrix: &PolicyCompilerSupportMatrix,
) -> Result<DomainCompiledPolicyRule, EventingError> {
    let capability_state =
        capability_state_for_rule(domain, support_matrix, rule.target.kind, rule.action)?;
    let status = rule_status_for_capability_state(capability_state);
    let reason_code = reason_code_for_capability_state(capability_state, domain, rule.action)?;

    Ok(DomainCompiledPolicyRule {
        rule_id: rule.rule_id,
        target: rule.target,
        action: rule.action,
        schedule_id: rule.schedule_id,
        capability_state,
        status,
        reason_code,
    })
}

fn capability_state_for_rule(
    domain: PolicyCompilerDomain,
    support_matrix: &PolicyCompilerSupportMatrix,
    target_kind: PolicyTargetKind,
    action: PolicyRuleAction,
) -> Result<PolicyCompilerCapabilityState, EventingError> {
    let matrix_capability_state = support_matrix_capability_state(support_matrix, target_kind)?;
    let resolver = CAPABILITY_STATE_RESOLVERS[compiler_domain_index(domain)];

    Ok(resolver(matrix_capability_state, action))
}

fn support_matrix_capability_state(
    support_matrix: &PolicyCompilerSupportMatrix,
    target_kind: PolicyTargetKind,
) -> Result<PolicyCompilerCapabilityState, EventingError> {
    support_matrix
        .rows
        .iter()
        .find(|row| row.target_kind == target_kind)
        .map(|row| row.capability_state)
        .ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::compiler::FIELD_SUPPORT_MATRIX_TARGET_KIND,
            value: policy_target_kind_name(target_kind).to_string(),
        })
}

fn rule_status_for_capability_state(
    capability_state: PolicyCompilerCapabilityState,
) -> PolicyCompilerRuleStatus {
    RULE_STATUS_BY_CAPABILITY_STATE[capability_state_index(capability_state)]
}

fn reason_code_for_capability_state(
    capability_state: PolicyCompilerCapabilityState,
    domain: PolicyCompilerDomain,
    action: PolicyRuleAction,
) -> Result<Option<PolicyReasonCode>, EventingError> {
    [
        None,
        Some(manual_required_reason(domain, action)),
        Some(policy_control::compiler::REASON_UNSUPPORTED_TARGET),
    ][capability_state_index(capability_state)]
    .map(parse_reason)
    .transpose()
}

fn parse_reason(value: &str) -> Result<PolicyReasonCode, EventingError> {
    PolicyReasonCode::parse(value)
}

const fn compiler_domain_index(domain: PolicyCompilerDomain) -> usize {
    domain as usize
}

const fn capability_state_index(capability_state: PolicyCompilerCapabilityState) -> usize {
    capability_state as usize
}

fn policy_target_kind_index(target_kind: PolicyTargetKind) -> Option<usize> {
    POLICY_TARGET_KINDS
        .iter()
        .position(|candidate| *candidate == target_kind)
}

fn domain_support_matrix(domain: PolicyCompilerDomain) -> DomainSupportMatrix {
    DOMAIN_SUPPORT_MATRICES[compiler_domain_index(domain)]
}

fn passthrough_capability_state(
    capability_state: PolicyCompilerCapabilityState,
    _action: PolicyRuleAction,
) -> PolicyCompilerCapabilityState {
    capability_state
}

fn enforcement_capability_state(
    capability_state: PolicyCompilerCapabilityState,
    _action: PolicyRuleAction,
) -> PolicyCompilerCapabilityState {
    [
        PolicyCompilerCapabilityState::ManualRequired,
        PolicyCompilerCapabilityState::Unsupported,
    ][usize::from(capability_state == PolicyCompilerCapabilityState::Unsupported)]
}

fn notification_ask_parent_capability_state(
    capability_state: PolicyCompilerCapabilityState,
    action: PolicyRuleAction,
) -> PolicyCompilerCapabilityState {
    [
        [
            PolicyCompilerCapabilityState::ManualRequired,
            PolicyCompilerCapabilityState::Supported,
        ][usize::from(action == PolicyRuleAction::AskParent)],
        PolicyCompilerCapabilityState::ManualRequired,
        PolicyCompilerCapabilityState::Unsupported,
    ][capability_state_index(capability_state)]
}

fn manual_required_reason(domain: PolicyCompilerDomain, action: PolicyRuleAction) -> &'static str {
    [
        policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET,
        policy_control::compiler::REASON_ENFORCEMENT_HANDOFF_REQUIRED,
    ][usize::from(
        domain == PolicyCompilerDomain::Enforcement && action != PolicyRuleAction::AskParent,
    )]
}

fn compiler_version_mismatch_value(
    source_policy_version: PolicyVersion,
    consumer_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::compiler::VALUE_SOURCE_POLICY_VERSION_PREFIX);
    value.push_str(&source_policy_version.value().to_string());
    value.push_str(policy_control::compiler::VALUE_CONSUMER_POLICY_VERSION_SEPARATOR);
    value.push_str(&consumer_policy_version.value().to_string());
    value
}

fn compiler_domain_name(domain: PolicyCompilerDomain) -> &'static str {
    POLICY_COMPILER_DOMAIN_NAMES[compiler_domain_index(domain)]
}

fn policy_target_kind_name(target_kind: PolicyTargetKind) -> &'static str {
    policy_target_kind_index(target_kind)
        .and_then(|index| POLICY_TARGET_KIND_NAMES.get(index).copied())
        .unwrap_or(UNKNOWN_POLICY_TARGET_KIND_NAME)
}
