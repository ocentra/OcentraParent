#![forbid(unsafe_code)]

use crate::policy_source::{
    ParentPolicyActorRole, PolicyHouseholdId, PolicySourceActorState, PolicySourceStatus,
    PolicySourceSurface, PolicyVersion,
};
use ocentra_parent_agent_protocol::constants::policy_control;

pub(crate) fn policy_surface_name(surface: PolicySourceSurface) -> &'static str {
    match surface {
        PolicySourceSurface::ParentPortal => policy_control::source::SURFACE_PARENT_PORTAL,
        PolicySourceSurface::ParentCompanion => policy_control::source::SURFACE_PARENT_COMPANION,
        PolicySourceSurface::AiPreview => policy_control::source::SURFACE_AI_PREVIEW,
        PolicySourceSurface::DomainCache => policy_control::source::SURFACE_DOMAIN_CACHE,
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

pub(crate) fn policy_status_name(status: PolicySourceStatus) -> &'static str {
    match status {
        PolicySourceStatus::Draft => policy_control::source::STATUS_DRAFT,
        PolicySourceStatus::Preview => policy_control::source::STATUS_PREVIEW,
        PolicySourceStatus::Confirmed => policy_control::source::STATUS_CONFIRMED,
        PolicySourceStatus::Queued => policy_control::source::STATUS_QUEUED,
        PolicySourceStatus::Delivered => policy_control::source::STATUS_DELIVERED,
        PolicySourceStatus::Acknowledged => policy_control::source::STATUS_ACKNOWLEDGED,
        PolicySourceStatus::Active => policy_control::source::STATUS_ACTIVE,
        PolicySourceStatus::PartiallyActive => policy_control::source::STATUS_PARTIALLY_ACTIVE,
        PolicySourceStatus::Rejected => policy_control::source::STATUS_REJECTED,
        PolicySourceStatus::Superseded => policy_control::source::STATUS_SUPERSEDED,
        PolicySourceStatus::RolledBack => policy_control::source::STATUS_ROLLED_BACK,
        PolicySourceStatus::Stale => policy_control::source::STATUS_STALE,
        PolicySourceStatus::Expired => policy_control::source::STATUS_EXPIRED,
        PolicySourceStatus::ManualRequired => policy_control::source::STATUS_MANUAL_REQUIRED,
    }
}

pub(crate) fn stale_policy_version_value(
    candidate_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_STALE_POLICY_VERSION_PREFIX);
    value.push_str(&candidate_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_STALE_POLICY_VERSION_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}

pub(crate) fn duplicate_source_truth_value(
    household_id: &PolicyHouseholdId,
    policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_DUPLICATE_SOURCE_TRUTH_PREFIX);
    value.push_str(household_id.as_str());
    value.push_str(policy_control::source::VALUE_DUPLICATE_SOURCE_TRUTH_VERSION_SEPARATOR);
    value.push_str(&policy_version.value().to_string());
    value
}

pub(crate) fn missing_audit_reference_for_status_value(status: PolicySourceStatus) -> String {
    let mut value =
        String::from(policy_control::source::VALUE_MISSING_AUDIT_REFERENCE_FOR_STATUS_PREFIX);
    value.push_str(policy_status_name(status));
    value
}

pub(crate) fn missing_audit_references_for_status_value(status: PolicySourceStatus) -> String {
    let mut value =
        String::from(policy_control::source::VALUE_MISSING_AUDIT_REFERENCES_FOR_STATUS_PREFIX);
    value.push_str(policy_status_name(status));
    value
}

pub(crate) fn replacement_policy_version_must_be_newer_value(
    replacement_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_REPLACEMENT_POLICY_VERSION_PREFIX);
    value.push_str(&replacement_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_MUST_BE_NEWER_THAN_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}

pub(crate) fn restored_policy_version_must_be_older_value(
    restored_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_RESTORED_POLICY_VERSION_PREFIX);
    value.push_str(&restored_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_MUST_BE_OLDER_THAN_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}
