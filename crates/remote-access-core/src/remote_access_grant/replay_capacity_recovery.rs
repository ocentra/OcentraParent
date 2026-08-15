use super::{
    replay_capacity::Capacity, RemoteAccessGrant, RemoteAccessGrantContext, RemoteAccessGrantState,
    RemoteAccessGrantTransition, RemoteAccessGrantTransitionAuthority,
};

pub(super) fn system_failure_stop_capacity(
    grant: &mut RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Option<Capacity> {
    if transition == RemoteAccessGrantTransition::Stop
        && context.transition_authority == RemoteAccessGrantTransitionAuthority::SystemFailure
    {
        recycle_completed_recovery_sequence(grant);
        return Some(if grant.stop_recovery_milestone.is_none() {
            Capacity::StopRecoveryMilestone
        } else {
            Capacity::Exhausted
        });
    }
    None
}

fn recycle_completed_recovery_sequence(grant: &mut RemoteAccessGrant) {
    if grant.state != RemoteAccessGrantState::Active
        || grant.stop_recovery != super::RemoteAccessGrantStopRecoveryState::NotRequired
        || grant.stop_recovery_milestone.is_none()
        || grant.reconnect_request_recovery_milestone.is_none()
        || grant.restart_recovery_milestone.is_none()
    {
        return;
    }
    grant.stop_recovery_milestone = None;
    grant.reconnect_request_recovery_milestone = None;
    grant.restart_recovery_milestone = None;
}

pub(super) fn is_restart_reconnect(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
) -> bool {
    transition == RemoteAccessGrantTransition::Reconnect
        && grant.state == RemoteAccessGrantState::ReconnectPending
        && (grant.restart_recovery_at == Some(grant.attempts.len())
            || grant.reconnect_request_recovery_milestone.is_some())
}

pub(super) fn is_system_recovery_reconnect_request(
    grant: &RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
) -> bool {
    transition == RemoteAccessGrantTransition::RequestReconnect
        && grant.state == RemoteAccessGrantState::Stopped
        && grant.stop_recovery == super::RemoteAccessGrantStopRecoveryState::Pending
}

pub(super) fn reserved_capacity(
    grant: &mut RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Option<Capacity> {
    if let Some(capacity) = system_failure_stop_capacity(grant, transition, context) {
        return Some(capacity);
    }
    if is_system_recovery_reconnect_request(grant, transition) {
        return Some(if grant.reconnect_request_recovery_milestone.is_none() {
            Capacity::ReconnectRequestRecoveryMilestone
        } else {
            Capacity::Exhausted
        });
    }
    if !is_restart_reconnect(grant, transition) {
        return None;
    }
    Some(
        if grant.restart_recovery_milestone.is_none()
            || (grant.reconnect_request_recovery_milestone.is_some()
                && grant.restart_recovery_at == Some(grant.attempts.len()))
        {
            Capacity::RestartRecoveryMilestone
        } else {
            Capacity::Exhausted
        },
    )
}
