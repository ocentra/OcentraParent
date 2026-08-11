use super::{
    RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantContext,
    RemoteAccessGrantTransition, RemoteAccessGrantTransitionAuthority,
    RemoteAccessGrantTransitionReport,
};

pub(super) enum Capacity {
    Attempts,
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
    if !can_use_reserved_milestone(transition, context) {
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

fn can_use_reserved_milestone(
    transition: RemoteAccessGrantTransition,
    context: &RemoteAccessGrantContext<'_>,
) -> bool {
    matches!(
        transition,
        RemoteAccessGrantTransition::Revoke
            | RemoteAccessGrantTransition::RemoveDevice
            | RemoteAccessGrantTransition::Supersede
    ) || (transition == RemoteAccessGrantTransition::Stop
        && context.transition_authority == RemoteAccessGrantTransitionAuthority::SystemFailure)
}

pub(super) fn record(
    grant: &mut RemoteAccessGrant,
    capacity: &Capacity,
    report: &RemoteAccessGrantTransitionReport,
) {
    match capacity {
        Capacity::Attempts => grant.attempts.push(report.audit.clone()),
        Capacity::ReservedMilestone if report.result.is_ok() => {
            grant.terminal_milestone = Some(report.audit.clone());
        }
        Capacity::ReservedMilestone | Capacity::Exhausted => {}
    }
}
