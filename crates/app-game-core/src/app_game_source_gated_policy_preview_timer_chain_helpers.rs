const TIMER_HANDOFF_READY_FOR_TIMER_SEQUENCING: &str = "ready-for-timer-sequencing";
const TIMER_HANDOFF_SOURCE_MANUAL_REQUIRED_BEFORE_TIMER: &str =
    "source-manual-required-before-timer";
const TIMER_HANDOFF_COMPILER_MANUAL_REQUIRED_BEFORE_TIMER: &str =
    "compiler-manual-required-before-timer";
const TIMER_STATUS_SOURCE_FRESHNESS_PROOF_REQUIRED: &str = "source-freshness-proof-required";
const TIMER_STATUS_RUNTIME_PROOF_REQUIRED: &str = "timer-runtime-proof-required";
const TIMER_RUNTIME_PROOF_REQUIRED: &str = "runtime-proof-required";
const TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS: &str = "blocked-by-source-freshness";
const TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION: &str = "blocked-by-compiler-decision";

pub(super) fn timer_handoff_state_for_projection(projection_state: &str) -> &'static str {
    match projection_state {
        "preview-ready-visible" => TIMER_HANDOFF_READY_FOR_TIMER_SEQUENCING,
        "source-manual-required-visible" => TIMER_HANDOFF_SOURCE_MANUAL_REQUIRED_BEFORE_TIMER,
        _ => TIMER_HANDOFF_COMPILER_MANUAL_REQUIRED_BEFORE_TIMER,
    }
}

pub(super) fn runtime_readiness_state_for_timer_status(timer_status_state: &str) -> &'static str {
    match timer_status_state {
        TIMER_STATUS_RUNTIME_PROOF_REQUIRED => TIMER_RUNTIME_PROOF_REQUIRED,
        TIMER_STATUS_SOURCE_FRESHNESS_PROOF_REQUIRED => TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS,
        _ => TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION,
    }
}
