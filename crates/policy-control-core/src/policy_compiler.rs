#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_source::{
    validate_parent_policy_source_document, ParentPolicyDocumentId, ParentPolicyRule,
    ParentPolicySourceDocument, PolicyAuditReferenceId, PolicyChildProfileId, PolicyDeviceId,
    PolicyHouseholdId, PolicyReasonCode, PolicyRetentionMetadata, PolicyRollbackRef,
    PolicyRuleAction, PolicyRuleId, PolicyRuleTarget, PolicyScheduleId, PolicyScheduleWindow,
    PolicySourceDocumentStatus, PolicyTargetKind, PolicyVersion,
};

const POLICY_COMPILER_SCHEMA_VERSION_VALUE: u16 = 1;

macro_rules! policy_compiler_text_id {
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

policy_compiler_text_id!(
    PolicyCompiledArtifactId,
    policy_control::compiler::FIELD_COMPILED_ARTIFACT_ID
);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyCompilerRuleStatus {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unsupported")]
    Unsupported,
}

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
    pub source_status: PolicySourceDocumentStatus,
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
    let rows = match domain {
        PolicyCompilerDomain::AppGame => vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::Unsupported,
            ),
        ],
        PolicyCompilerDomain::Browser => vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
        ],
        PolicyCompilerDomain::Network => vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::Supported,
            ),
        ],
        PolicyCompilerDomain::Tracking => vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::Supported,
            ),
        ],
        PolicyCompilerDomain::Screen => vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::Unsupported,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::ManualRequired,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::Unsupported,
            ),
        ],
        PolicyCompilerDomain::Ai => vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::Supported,
            ),
        ],
        PolicyCompilerDomain::Enforcement => vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::Supported,
            ),
        ],
        PolicyCompilerDomain::NotificationAskParent => vec![
            support_matrix_row(
                PolicyTargetKind::ChildProfile,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Device,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::App,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Site,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Category,
                PolicyCompilerCapabilityState::Supported,
            ),
            support_matrix_row(
                PolicyTargetKind::Resource,
                PolicyCompilerCapabilityState::Supported,
            ),
        ],
    };

    PolicyCompilerSupportMatrix { domain, rows }
}

fn support_matrix_row(
    target_kind: PolicyTargetKind,
    capability_state: PolicyCompilerCapabilityState,
) -> PolicyCompilerSupportMatrixRow {
    PolicyCompilerSupportMatrixRow {
        target_kind,
        capability_state,
    }
}

fn assert_support_matrix_matches_domain(
    support_matrix: &PolicyCompilerSupportMatrix,
    domain: PolicyCompilerDomain,
) -> Result<(), EventingError> {
    if support_matrix.domain != domain {
        return Err(EventingError::InvalidValue {
            field: policy_control::compiler::FIELD_SUPPORT_MATRIX_DOMAIN,
            value: compiler_domain_name(support_matrix.domain).to_string(),
        });
    }

    for target_kind in all_policy_target_kinds() {
        let match_count = support_matrix
            .rows
            .iter()
            .filter(|row| row.target_kind == target_kind)
            .count();
        if match_count != 1 {
            return Err(EventingError::InvalidValue {
                field: policy_control::compiler::FIELD_SUPPORT_MATRIX_TARGET_KIND,
                value: policy_target_kind_name(target_kind).to_string(),
            });
        }
    }

    Ok(())
}

fn all_policy_target_kinds() -> [PolicyTargetKind; 6] {
    [
        PolicyTargetKind::ChildProfile,
        PolicyTargetKind::Device,
        PolicyTargetKind::App,
        PolicyTargetKind::Site,
        PolicyTargetKind::Category,
        PolicyTargetKind::Resource,
    ]
}

fn assert_source_status_can_compile(
    surface: crate::policy_source::PolicySourceWriteSurface,
    status: PolicySourceDocumentStatus,
) -> Result<(), EventingError> {
    if matches!(
        surface,
        crate::policy_source::PolicySourceWriteSurface::DomainCache
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SOURCE_SURFACE,
            value: policy_control::source::SURFACE_DOMAIN_CACHE.to_string(),
        });
    }

    if matches!(
        status,
        PolicySourceDocumentStatus::Draft | PolicySourceDocumentStatus::Preview
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::compiler::FIELD_SOURCE_STATUS,
            value: source_status_name(status).to_string(),
        });
    }

    Ok(())
}

fn assert_compiler_version_compatible(
    source_policy_version: PolicyVersion,
    consumer_policy_version: PolicyVersion,
) -> Result<(), EventingError> {
    if source_policy_version == consumer_policy_version {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
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

    match domain {
        PolicyCompilerDomain::Enforcement => Ok(
            if matrix_capability_state == PolicyCompilerCapabilityState::Unsupported {
                PolicyCompilerCapabilityState::Unsupported
            } else {
                PolicyCompilerCapabilityState::ManualRequired
            },
        ),
        PolicyCompilerDomain::NotificationAskParent => Ok(match matrix_capability_state {
            PolicyCompilerCapabilityState::Unsupported => {
                PolicyCompilerCapabilityState::Unsupported
            }
            PolicyCompilerCapabilityState::ManualRequired => {
                PolicyCompilerCapabilityState::ManualRequired
            }
            PolicyCompilerCapabilityState::Supported => {
                if action == PolicyRuleAction::AskParent {
                    PolicyCompilerCapabilityState::Supported
                } else {
                    PolicyCompilerCapabilityState::ManualRequired
                }
            }
        }),
        _ => Ok(matrix_capability_state),
    }
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
    match capability_state {
        PolicyCompilerCapabilityState::Supported => PolicyCompilerRuleStatus::Ready,
        PolicyCompilerCapabilityState::ManualRequired => PolicyCompilerRuleStatus::ManualRequired,
        PolicyCompilerCapabilityState::Unsupported => PolicyCompilerRuleStatus::Unsupported,
    }
}

fn reason_code_for_capability_state(
    capability_state: PolicyCompilerCapabilityState,
    domain: PolicyCompilerDomain,
    action: PolicyRuleAction,
) -> Result<Option<PolicyReasonCode>, EventingError> {
    match capability_state {
        PolicyCompilerCapabilityState::Supported => Ok(None),
        PolicyCompilerCapabilityState::ManualRequired => {
            let reason = if domain == PolicyCompilerDomain::Enforcement
                && action != PolicyRuleAction::AskParent
            {
                policy_control::compiler::REASON_ENFORCEMENT_HANDOFF_REQUIRED
            } else {
                policy_control::compiler::REASON_MANUAL_REQUIRED_TARGET
            };
            Ok(Some(parse_reason(reason)?))
        }
        PolicyCompilerCapabilityState::Unsupported => Ok(Some(parse_reason(
            policy_control::compiler::REASON_UNSUPPORTED_TARGET,
        )?)),
    }
}

fn parse_reason(value: &str) -> Result<PolicyReasonCode, EventingError> {
    PolicyReasonCode::parse(value)
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
    match domain {
        PolicyCompilerDomain::AppGame => "app-game",
        PolicyCompilerDomain::Browser => "browser",
        PolicyCompilerDomain::Network => "network",
        PolicyCompilerDomain::Tracking => "tracking",
        PolicyCompilerDomain::Screen => "screen",
        PolicyCompilerDomain::Ai => "ai",
        PolicyCompilerDomain::Enforcement => "enforcement",
        PolicyCompilerDomain::NotificationAskParent => "notification-ask-parent",
    }
}

fn policy_target_kind_name(target_kind: PolicyTargetKind) -> &'static str {
    match target_kind {
        PolicyTargetKind::ChildProfile => "child-profile",
        PolicyTargetKind::Device => "device",
        PolicyTargetKind::App => "app",
        PolicyTargetKind::Site => "site",
        PolicyTargetKind::Category => "category",
        PolicyTargetKind::Resource => "resource",
    }
}

fn source_status_name(status: PolicySourceDocumentStatus) -> &'static str {
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
