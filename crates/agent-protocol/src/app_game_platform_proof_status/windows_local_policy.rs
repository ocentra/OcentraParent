use super::{
    AppGameWindowsLocalPolicyEvidence, AppGameWindowsLocalPolicyEvidenceError,
    AppGameWindowsLocalPolicyEvidenceState,
};
use crate::app_game::APP_GAME_SCHEMA_VERSION;

#[path = "windows_local_policy/nonclaims.rs"]
mod nonclaims;
#[path = "windows_local_policy/observation.rs"]
mod observation;
#[path = "windows_local_policy/references.rs"]
mod references;
#[path = "windows_local_policy/text.rs"]
mod text;

pub(super) fn validate_schema_and_state(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    if evidence.schema_version != APP_GAME_SCHEMA_VERSION {
        return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidSchemaVersion);
    }
    if !text::is_canonical_utc_timestamp(&evidence.observed_at) {
        return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidObservedAt);
    }
    let succeeded = u8::from(evidence.app_id_service_query_succeeded)
        + u8::from(evidence.app_locker_policy_readable)
        + u8::from(evidence.device_guard_query_succeeded);
    let state_is_valid = match evidence.state {
        AppGameWindowsLocalPolicyEvidenceState::Ready => evidence.probe_supported && succeeded == 3,
        AppGameWindowsLocalPolicyEvidenceState::Partial => {
            evidence.probe_supported && (1..3).contains(&succeeded)
        }
        AppGameWindowsLocalPolicyEvidenceState::Unavailable => succeeded == 0,
    };
    if !state_is_valid {
        return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidState);
    }
    Ok(())
}

pub(super) fn validate_observations(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    observation::validate(evidence)
}

pub(super) fn validate_redaction_and_nonclaims(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    nonclaims::validate(evidence)
}

pub(super) fn validate_references_and_gaps(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    references::validate(evidence)
}
