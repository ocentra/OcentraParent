use super::{
    validation::is_terminal, RemoteAccessGrant, RemoteAccessGrantAuditOutcome,
    RemoteAccessGrantDisclosureState, RemoteAccessGrantError, RemoteAccessGrantParentGrant,
    RemoteAccessGrantState,
};

pub(super) fn validate(grant: &RemoteAccessGrant) -> Result<(), RemoteAccessGrantError> {
    let (expected_disclosure, expected_parent_grant) =
        lifecycle_evidence(lifecycle_state_for_evidence(grant))?;
    if grant.disclosure_state != expected_disclosure || grant.parent_grant != expected_parent_grant
    {
        return Err(RemoteAccessGrantError::InvalidSerializedState);
    }
    Ok(())
}

fn lifecycle_state_for_evidence(grant: &RemoteAccessGrant) -> RemoteAccessGrantState {
    if !is_terminal(grant.state) {
        return grant.state;
    }
    grant
        .attempts
        .iter()
        .rev()
        .find(|attempt| {
            attempt.outcome == RemoteAccessGrantAuditOutcome::Accepted
                && !is_terminal(attempt.resulting_state)
        })
        .map_or(RemoteAccessGrantState::Requested, |attempt| {
            attempt.resulting_state
        })
}

fn lifecycle_evidence(
    state: RemoteAccessGrantState,
) -> Result<
    (
        RemoteAccessGrantDisclosureState,
        RemoteAccessGrantParentGrant,
    ),
    RemoteAccessGrantError,
> {
    match state {
        RemoteAccessGrantState::Requested => Ok((
            RemoteAccessGrantDisclosureState::Undisclosed,
            RemoteAccessGrantParentGrant::NotGranted,
        )),
        RemoteAccessGrantState::ParentConfirmed => Ok((
            RemoteAccessGrantDisclosureState::Undisclosed,
            RemoteAccessGrantParentGrant::Granted,
        )),
        RemoteAccessGrantState::Paired
        | RemoteAccessGrantState::Active
        | RemoteAccessGrantState::Paused
        | RemoteAccessGrantState::Stopped
        | RemoteAccessGrantState::ReconnectPending => Ok((
            RemoteAccessGrantDisclosureState::Disclosed,
            RemoteAccessGrantParentGrant::Granted,
        )),
        _ => Err(RemoteAccessGrantError::InvalidSerializedState),
    }
}
