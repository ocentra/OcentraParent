use super::{
    replay_capacity::Capacity, RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantState,
    RemoteAccessGrantTransition, RemoteAccessGrantTransitionAuthority,
};

pub(super) fn system_failure_stop_capacity(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Option<Capacity> {
    if transition == RemoteAccessGrantTransition::Stop
        && context.transition_authority == RemoteAccessGrantTransitionAuthority::SystemFailure
    {
        return Some(if grant.stop_recovery_milestone.is_none() {
            Capacity::StopRecoveryMilestone
        } else {
            Capacity::Exhausted
        });
    }
    None
}

pub(super) fn is_restart_reconnect(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
) -> bool {
    transition == RemoteAccessGrantTransition::Reconnect
        && grant.state == RemoteAccessGrantState::ReconnectPending
        && grant.restart_recovery_at == Some(grant.attempts.len())
}
