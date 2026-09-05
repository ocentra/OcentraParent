use super::{AppGameWindowsLocalPolicyEvidence, AppGameWindowsLocalPolicyEvidenceError};
use crate::app_game_platform_proof_status::{
    APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT,
    APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT,
};

pub(super) fn validate(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    validate_app_id_service(evidence)?;
    validate_app_locker(evidence)?;
    validate_device_guard(evidence)?;
    validate_app_control(evidence)
}

fn validate_app_id_service(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    let invalid_without_query = !evidence.app_id_service_query_succeeded
        && (evidence.app_id_service_present || evidence.app_id_service_running);
    let invalid_without_service =
        !evidence.app_id_service_present && evidence.app_id_service_running;
    if invalid_without_query || invalid_without_service {
        return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidAppIdServiceState);
    }
    Ok(())
}

fn validate_app_locker(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    let counts_without_policy = !evidence.app_locker_policy_readable
        && (evidence.app_locker_collection_count != 0 || evidence.app_locker_rule_count != 0);
    let counts_out_of_bounds = evidence.app_locker_collection_count
        > APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT
        || evidence.app_locker_rule_count > APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT;
    if counts_without_policy || counts_out_of_bounds {
        return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidAppLockerCounts);
    }
    Ok(())
}

fn validate_device_guard(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    let state_without_query = !evidence.device_guard_query_succeeded
        && (evidence.device_guard_configured || evidence.device_guard_running);
    let running_without_configuration =
        !evidence.device_guard_configured && evidence.device_guard_running;
    if state_without_query || running_without_configuration {
        return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidDeviceGuardState);
    }
    Ok(())
}

fn validate_app_control(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    let state_without_configuration = !evidence.app_control_configured
        && (evidence.app_control_audit_only || evidence.app_control_policy_reports_enforced);
    let contradictory_state =
        evidence.app_control_audit_only && evidence.app_control_policy_reports_enforced;
    if state_without_configuration || contradictory_state {
        return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidAppControlState);
    }
    Ok(())
}
