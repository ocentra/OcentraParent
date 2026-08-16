use super::{
    AppGameSourceGatedPolicyPreviewReadModelRow, AppGameSourceGatedPolicyPreviewTimerStatusOptions,
};

const READ_MODEL_PREVIEW_READY_VISIBLE: &str = "preview-ready-visible";
const READ_MODEL_SOURCE_MANUAL_REQUIRED_VISIBLE: &str = "source-manual-required-visible";
const READ_MODEL_COMPILER_MANUAL_REQUIRED_VISIBLE: &str = "compiler-manual-required-visible";

const TIMER_STATUS_TIMER_RUNTIME_PROOF_REQUIRED: &str = "timer-runtime-proof-required";
const TIMER_STATUS_SOURCE_FRESHNESS_PROOF_REQUIRED: &str = "source-freshness-proof-required";
const TIMER_STATUS_COMPILER_DECISION_PROOF_REQUIRED: &str = "compiler-decision-proof-required";

pub(super) fn projection_state_for_gate_state(gate_state: &str) -> &'static str {
    match gate_state {
        "source-fresh" => READ_MODEL_PREVIEW_READY_VISIBLE,
        "source-manual-required" => READ_MODEL_SOURCE_MANUAL_REQUIRED_VISIBLE,
        _ => READ_MODEL_COMPILER_MANUAL_REQUIRED_VISIBLE,
    }
}

pub(super) fn timer_status_state_for_handoff_state(timer_handoff_state: &str) -> &'static str {
    match timer_handoff_state {
        "ready-for-timer-sequencing" => TIMER_STATUS_TIMER_RUNTIME_PROOF_REQUIRED,
        "source-manual-required-before-timer" => TIMER_STATUS_SOURCE_FRESHNESS_PROOF_REQUIRED,
        _ => TIMER_STATUS_COMPILER_DECISION_PROOF_REQUIRED,
    }
}

pub(super) fn required_proof_refs_for_timer_status(
    options: &AppGameSourceGatedPolicyPreviewTimerStatusOptions,
    timer_status_state: &str,
) -> Vec<String> {
    match timer_status_state {
        TIMER_STATUS_TIMER_RUNTIME_PROOF_REQUIRED => {
            vec![options.timer_runtime_proof_ref.clone()]
        }
        TIMER_STATUS_SOURCE_FRESHNESS_PROOF_REQUIRED => {
            vec![options.source_freshness_proof_ref.clone()]
        }
        _ => vec![options.compiler_decision_proof_ref.clone()],
    }
}

pub(super) fn count_rows(
    rows: &[AppGameSourceGatedPolicyPreviewReadModelRow],
    target_domain: &str,
) -> usize {
    rows.iter()
        .filter(|row| row.target_domain == target_domain)
        .count()
}

pub(super) fn count_projection_rows(
    rows: &[AppGameSourceGatedPolicyPreviewReadModelRow],
    projection_state: &str,
) -> usize {
    rows.iter()
        .filter(|row| row.projection_state == projection_state)
        .count()
}
