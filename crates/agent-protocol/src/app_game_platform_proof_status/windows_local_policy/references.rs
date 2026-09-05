use std::collections::HashSet;

use super::{text, AppGameWindowsLocalPolicyEvidence, AppGameWindowsLocalPolicyEvidenceError};
use crate::app_game_platform_proof_status::{
    AppGameWindowsLocalPolicyEvidenceGap, APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT,
};

const REQUIRED_NONCLAIM_GAPS: [AppGameWindowsLocalPolicyEvidenceGap; 4] = [
    AppGameWindowsLocalPolicyEvidenceGap::BroadBlockingNotProved,
    AppGameWindowsLocalPolicyEvidenceGap::SystemAllowlistNotProved,
    AppGameWindowsLocalPolicyEvidenceGap::RollbackNotProved,
    AppGameWindowsLocalPolicyEvidenceGap::AuditCustodyNotProved,
];

pub(super) fn validate(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    validate_proof_refs(evidence)?;
    validate_open_gaps(evidence)
}

fn validate_proof_refs(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    if evidence.proof_refs.is_empty()
        || evidence.proof_refs.len() > APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT as usize
    {
        return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidProofReference);
    }
    let mut unique = HashSet::with_capacity(evidence.proof_refs.len());
    for proof_ref in &evidence.proof_refs {
        if !text::is_opaque_local_policy_reference(proof_ref) {
            return Err(AppGameWindowsLocalPolicyEvidenceError::InvalidProofReference);
        }
        if !unique.insert(proof_ref.as_str()) {
            return Err(AppGameWindowsLocalPolicyEvidenceError::DuplicateProofReference);
        }
    }
    Ok(())
}

fn validate_open_gaps(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    if evidence.open_gaps.len() > APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT as usize {
        return Err(AppGameWindowsLocalPolicyEvidenceError::MissingRequiredGap);
    }
    let unique = evidence.open_gaps.iter().copied().collect::<HashSet<_>>();
    if unique.len() != evidence.open_gaps.len() {
        return Err(AppGameWindowsLocalPolicyEvidenceError::DuplicateGap);
    }
    if REQUIRED_NONCLAIM_GAPS
        .into_iter()
        .any(|required| !unique.contains(&required))
    {
        return Err(AppGameWindowsLocalPolicyEvidenceError::MissingRequiredGap);
    }
    validate_observation_gaps(evidence, &unique)?;
    Ok(())
}

fn validate_observation_gaps(
    evidence: &AppGameWindowsLocalPolicyEvidence,
    gaps: &HashSet<AppGameWindowsLocalPolicyEvidenceGap>,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    let expected = [
        (
            !evidence.app_id_service_query_succeeded,
            AppGameWindowsLocalPolicyEvidenceGap::AppIdServiceUnavailable,
        ),
        (
            !evidence.app_locker_policy_readable,
            AppGameWindowsLocalPolicyEvidenceGap::AppLockerPolicyUnreadable,
        ),
        (
            !evidence.device_guard_query_succeeded,
            AppGameWindowsLocalPolicyEvidenceGap::DeviceGuardUnavailable,
        ),
        (
            !evidence.app_control_configured,
            AppGameWindowsLocalPolicyEvidenceGap::AppControlNotConfigured,
        ),
    ];
    if expected
        .into_iter()
        .any(|(required, gap)| required && !gaps.contains(&gap))
    {
        return Err(AppGameWindowsLocalPolicyEvidenceError::MissingRequiredGap);
    }
    Ok(())
}
