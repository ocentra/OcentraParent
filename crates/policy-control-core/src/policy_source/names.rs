#![forbid(unsafe_code)]

use crate::policy_source::{
    ParentPolicyActorRole, PolicyHouseholdId, PolicySourceActorState, PolicySourceStatus,
    PolicySourceSurface, PolicyVersion,
};

mod labels;
mod values;

pub(crate) fn policy_surface_name(surface: PolicySourceSurface) -> &'static str {
    labels::policy_surface_name(surface)
}

pub(crate) fn policy_actor_role_name(role: ParentPolicyActorRole) -> &'static str {
    labels::policy_actor_role_name(role)
}

pub(crate) fn policy_actor_state_name(state: PolicySourceActorState) -> &'static str {
    labels::policy_actor_state_name(state)
}

pub(crate) fn policy_status_name(status: PolicySourceStatus) -> &'static str {
    labels::policy_status_name(status)
}

pub(crate) fn stale_policy_version_value(
    candidate_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    values::stale_policy_version_value(candidate_policy_version, current_policy_version)
}

pub(crate) fn duplicate_source_truth_value(
    household_id: &PolicyHouseholdId,
    policy_version: PolicyVersion,
) -> String {
    values::duplicate_source_truth_value(household_id, policy_version)
}

pub(crate) fn missing_audit_reference_for_status_value(status: PolicySourceStatus) -> String {
    values::missing_audit_reference_for_status_value(status)
}

pub(crate) fn missing_audit_references_for_status_value(status: PolicySourceStatus) -> String {
    values::missing_audit_references_for_status_value(status)
}

pub(crate) fn replacement_policy_version_must_be_newer_value(
    replacement_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    values::replacement_policy_version_must_be_newer_value(
        replacement_policy_version,
        current_policy_version,
    )
}

pub(crate) fn restored_policy_version_must_be_older_value(
    restored_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    values::restored_policy_version_must_be_older_value(
        restored_policy_version,
        current_policy_version,
    )
}
