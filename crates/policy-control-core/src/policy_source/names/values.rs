#![forbid(unsafe_code)]

use crate::policy_source::{PolicyHouseholdId, PolicySourceStatus, PolicyVersion};
use ocentra_parent_agent_protocol::constants::policy_control;

use super::policy_status_name;

pub(super) fn stale_policy_version_value(
    candidate_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_STALE_POLICY_VERSION_PREFIX);
    value.push_str(&candidate_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_STALE_POLICY_VERSION_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}

pub(super) fn duplicate_source_truth_value(
    household_id: &PolicyHouseholdId,
    policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_DUPLICATE_SOURCE_TRUTH_PREFIX);
    value.push_str(household_id.as_str());
    value.push_str(policy_control::source::VALUE_DUPLICATE_SOURCE_TRUTH_VERSION_SEPARATOR);
    value.push_str(&policy_version.value().to_string());
    value
}

pub(super) fn missing_audit_reference_for_status_value(status: PolicySourceStatus) -> String {
    let mut value =
        String::from(policy_control::source::VALUE_MISSING_AUDIT_REFERENCE_FOR_STATUS_PREFIX);
    value.push_str(policy_status_name(status));
    value
}

pub(super) fn missing_audit_references_for_status_value(status: PolicySourceStatus) -> String {
    let mut value =
        String::from(policy_control::source::VALUE_MISSING_AUDIT_REFERENCES_FOR_STATUS_PREFIX);
    value.push_str(policy_status_name(status));
    value
}

pub(super) fn replacement_policy_version_must_be_newer_value(
    replacement_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_REPLACEMENT_POLICY_VERSION_PREFIX);
    value.push_str(&replacement_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_MUST_BE_NEWER_THAN_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}

pub(super) fn restored_policy_version_must_be_older_value(
    restored_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::source::VALUE_RESTORED_POLICY_VERSION_PREFIX);
    value.push_str(&restored_policy_version.value().to_string());
    value.push_str(policy_control::source::VALUE_MUST_BE_OLDER_THAN_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}
