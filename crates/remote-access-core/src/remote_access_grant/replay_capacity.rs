use super::{
    RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantContext,
    RemoteAccessGrantTransition, RemoteAccessGrantTransitionAuthority,
    RemoteAccessGrantTransitionReport,
};

pub(super) enum Capacity {
    Attempts,
    StopRecoveryMilestone,
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
    if is_system_failure_stop(transition, context) {
        return if grant.stop_recovery_milestone.is_none() {
            Capacity::StopRecoveryMilestone
        } else {
            Capacity::Exhausted
        };
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

fn is_system_failure_stop(
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> bool {
    transition == RemoteAccessGrantTransition::Stop
        && context.transition_authority == RemoteAccessGrantTransitionAuthority::SystemFailure
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
        Capacity::ReservedMilestone if report.result.is_ok() => {
            grant.terminal_milestone = Some(report.audit.clone());
        }
        Capacity::StopRecoveryMilestone | Capacity::ReservedMilestone | Capacity::Exhausted => {}
    }
}
