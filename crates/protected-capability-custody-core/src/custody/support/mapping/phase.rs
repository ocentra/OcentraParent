use super::super::super::{Decision, TransitionPhase};

pub(super) fn intent(decision: Decision) -> TransitionPhase {
    if decision == Decision::Commit {
        TransitionPhase::CommitIntent
    } else {
        TransitionPhase::AbortIntent
    }
}

pub(super) fn terminal(decision: Decision) -> TransitionPhase {
    if decision == Decision::Commit {
        TransitionPhase::CommitTerminal
    } else {
        TransitionPhase::AbortTerminal
    }
}
