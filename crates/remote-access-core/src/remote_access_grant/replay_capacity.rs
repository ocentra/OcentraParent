use super::{
    RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantContext,
    RemoteAccessGrantTransition, RemoteAccessGrantTransitionReport,
};

pub(super) enum Capacity {
    Attempts,
    StopRecoveryMilestone,
    ReconnectRequestRecoveryMilestone,
    RestartRecoveryMilestone,
    ReservedMilestone,
    Exhausted,
}

pub(super) fn prepare(
    grant: &mut RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> Capacity {
    if grant.attempts.len() < super::MAX_REPLAY_ATTEMPTS {
        return Capacity::Attempts;
    }
    if let Some(index) = grant.attempts.iter().position(|attempt| {
        super::replay::is_child_device_retry(grant, attempt, transition, context)
    }) {
        grant.attempts.remove(index);
        return Capacity::Attempts;
    }
    if let Some(capacity) =
        super::replay_capacity_recovery::reserved_capacity(grant, transition, context)
    {
        return capacity;
    }
    if !can_use_reserved_milestone(transition) {
        return Capacity::Exhausted;
    }
    grant
        .attempts
        .iter()
        .position(|attempt| attempt.outcome == RemoteAccessGrantAuditOutcome::Denied)
        .map(|index| {
            grant.attempts.remove(index);
            Capacity::Attempts
        })
        .unwrap_or_else(|| {
            if grant.terminal_milestone.is_none() {
                Capacity::ReservedMilestone
            } else {
                Capacity::Exhausted
            }
        })
}

fn can_use_reserved_milestone(transition: RemoteAccessGrantTransition) -> bool {
    matches!(
        transition,
        RemoteAccessGrantTransition::Revoke
            | RemoteAccessGrantTransition::RemoveDevice
            | RemoteAccessGrantTransition::Supersede
    )
}

pub(super) fn record(
    grant: &mut RemoteAccessGrant,
    capacity: &Capacity,
    report: &RemoteAccessGrantTransitionReport,
) {
    match capacity {
        Capacity::Attempts => grant.attempts.push(report.audit.clone()),
        Capacity::StopRecoveryMilestone if report.result.is_ok() => {
            grant.stop_recovery_milestone = Some(report.audit.clone());
        }
        Capacity::ReconnectRequestRecoveryMilestone if report.result.is_ok() => {
            grant.reconnect_request_recovery_milestone = Some(report.audit.clone());
        }
        Capacity::RestartRecoveryMilestone if report.result.is_ok() => {
            grant.restart_recovery_milestone = Some(report.audit.clone());
        }
        Capacity::ReservedMilestone if report.result.is_ok() => {
            grant.terminal_milestone = Some(report.audit.clone());
        }
        Capacity::StopRecoveryMilestone
        | Capacity::ReconnectRequestRecoveryMilestone
        | Capacity::RestartRecoveryMilestone
        | Capacity::ReservedMilestone
        | Capacity::Exhausted => {}
    }
}
