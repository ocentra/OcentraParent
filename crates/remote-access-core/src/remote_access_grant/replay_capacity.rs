use super::{RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantTransition};

pub(super) fn prepare(
    grant: &mut RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
) -> bool {
    if grant.attempts.len() < super::MAX_REPLAY_ATTEMPTS {
        return true;
    }
    if !matches!(
        transition,
        RemoteAccessGrantTransition::Revoke
            | RemoteAccessGrantTransition::RemoveDevice
            | RemoteAccessGrantTransition::Supersede
    ) {
        return false;
    }
    grant
        .attempts
        .iter()
        .position(|attempt| attempt.outcome == RemoteAccessGrantAuditOutcome::Denied)
        .map(|index| {
            grant.attempts.remove(index);
            true
        })
        .unwrap_or(false)
}
