use super::{
    RemoteAccessGrant, RemoteAccessGrantAuditOutcome, RemoteAccessGrantTransition,
    RemoteAccessGrantTransitionReport,
};

pub(super) enum Capacity {
    Attempts,
    TerminalMilestone,
    Exhausted,
}

pub(super) fn prepare(
    grant: &mut RemoteAccessGrant,
    transition: RemoteAccessGrantTransition,
) -> Capacity {
    if grant.attempts.len() < super::MAX_REPLAY_ATTEMPTS {
        return Capacity::Attempts;
    }
    if !matches!(
        transition,
        RemoteAccessGrantTransition::Revoke
            | RemoteAccessGrantTransition::RemoveDevice
            | RemoteAccessGrantTransition::Supersede
    ) {
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
                Capacity::TerminalMilestone
            } else {
                Capacity::Exhausted
            }
        })
}

pub(super) fn record(
    grant: &mut RemoteAccessGrant,
    capacity: &Capacity,
    report: &RemoteAccessGrantTransitionReport,
) {
    match capacity {
        Capacity::Attempts => grant.attempts.push(report.audit.clone()),
        Capacity::TerminalMilestone if report.result.is_ok() => {
            grant.terminal_milestone = Some(report.audit.clone());
        }
        Capacity::TerminalMilestone | Capacity::Exhausted => {}
    }
}
