use super::*;

const APP_GAME_PREFLIGHT_ROW_STATUS_LABELS: &[(&str, &str)] = &[
    ("dispatch-eligible", "Dispatch eligible"),
    ("blocked-before-dispatch", "Blocked before dispatch"),
    ("manual-required", "Manual proof required"),
    ("unavailable", "Adapter unavailable"),
    ("unsupported", "Platform unsupported"),
    ("degraded", "Adapter degraded"),
];
const APP_GAME_PREFLIGHT_DECISION_LABELS: &[(&str, &str)] =
    &[("dispatch-eligible", "Dispatch eligible")];
const APP_GAME_PREFLIGHT_OUTCOME_LABELS: &[(&str, &str)] = &[("dispatch-ready", "Dispatch ready")];
const APP_GAME_RESULT_ROW_STATUS_LABELS: &[(&str, &str)] = &[
    ("command-accepted", "Command accepted"),
    ("blocked-before-command", "Blocked before command"),
    ("manual-required", "Manual proof required"),
    ("unavailable", "Adapter unavailable"),
    ("unsupported", "Platform unsupported"),
    ("degraded", "Adapter degraded"),
];
const APP_GAME_RESULT_DECISION_LABELS: &[(&str, &str)] =
    &[("command-accepted", "Command accepted")];
const APP_GAME_EXECUTION_AUDIT_LABELS: &[(&str, &str)] = &[
    (
        "service-local-audit-recorded",
        "Service-local audit recorded",
    ),
    (
        "blocked-before-execution-audit",
        "Blocked before execution audit",
    ),
];
const APP_GAME_ADAPTER_EXECUTION_LABELS: &[(&str, &str)] = &[
    ("adapter-execution-reported", "Adapter execution reported"),
    (
        "adapter-execution-evidence-missing",
        "Execution evidence missing",
    ),
    (
        "blocked-before-adapter-execution",
        "Blocked before adapter execution",
    ),
];
pub(super) fn app_game_panel_unavailable(
    eyebrow: &str,
    title: &str,
    body: &str,
    empty_message: &str,
    product_claim: &str,
) -> ParentAppGamePanelSnapshot {
    ParentAppGamePanelSnapshot {
        eyebrow: eyebrow.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        load_state: "unavailable".to_string(),
        summary_details: vec![
            app_game_detail("Status", "unavailable"),
            app_game_detail("Product claim", product_claim),
        ],
        rows: Vec::new(),
        empty_message: empty_message.to_string(),
        product_claim: product_claim.to_string(),
    }
}

pub(super) fn app_game_panel_row(
    title: impl Into<String>,
    details: Vec<ParentAppGamePanelDetailSnapshot>,
) -> ParentAppGamePanelRowSnapshot {
    ParentAppGamePanelRowSnapshot {
        title: title.into(),
        details,
    }
}

pub(super) fn app_game_detail(
    label: impl Into<String>,
    value: impl Into<String>,
) -> ParentAppGamePanelDetailSnapshot {
    ParentAppGamePanelDetailSnapshot {
        label: label.into(),
        value: value.into(),
    }
}

pub(super) fn app_game_optional_string(value: Option<&str>) -> String {
    value.unwrap_or("Not reported").to_string()
}

pub(super) fn app_game_adapter_dispatch_preflight_load_state(
    read_model: &AppGameAdapterDispatchPreflightReadModel,
) -> String {
    if read_model.returned == 0 {
        "unavailable".to_string()
    } else if read_model.dispatch_eligible_count > 0 && read_model.blocked_before_dispatch_count > 0
    {
        "warn".to_string()
    } else {
        "ready".to_string()
    }
}

pub(super) fn app_game_adapter_dispatch_preflight_row_status(state: &str) -> String {
    APP_GAME_PREFLIGHT_ROW_STATUS_LABELS
        .iter()
        .find(|(raw, _)| *raw == state)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| state.to_string())
}

pub(super) fn app_game_adapter_dispatch_preflight_decision_label(decision: &str) -> String {
    APP_GAME_PREFLIGHT_DECISION_LABELS
        .iter()
        .find(|(raw, _)| *raw == decision)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| "Blocked before dispatch".to_string())
}

pub(super) fn app_game_adapter_dispatch_preflight_outcome_label(outcome: &str) -> String {
    APP_GAME_PREFLIGHT_OUTCOME_LABELS
        .iter()
        .find(|(raw, _)| *raw == outcome)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| outcome.to_string())
}

pub(super) fn app_game_adapter_dispatch_result_load_state(
    read_model: &AppGameAdapterDispatchResultReadModel,
) -> String {
    if read_model.returned == 0 {
        "unavailable".to_string()
    } else if read_model.command_accepted_count > 0 && read_model.blocked_before_command_count > 0 {
        "warn".to_string()
    } else {
        "ready".to_string()
    }
}

pub(super) fn app_game_adapter_dispatch_result_row_status(state: &str) -> String {
    APP_GAME_RESULT_ROW_STATUS_LABELS
        .iter()
        .find(|(raw, _)| *raw == state)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| state.to_string())
}

pub(super) fn app_game_adapter_dispatch_result_decision_label(decision: &str) -> String {
    APP_GAME_RESULT_DECISION_LABELS
        .iter()
        .find(|(raw, _)| *raw == decision)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| "Blocked before command".to_string())
}

pub(super) fn app_game_adapter_dispatch_execution_audit_label(state: &str) -> String {
    APP_GAME_EXECUTION_AUDIT_LABELS
        .iter()
        .find(|(raw, _)| *raw == state)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| state.to_string())
}

pub(super) fn app_game_adapter_dispatch_adapter_execution_label(state: &str) -> String {
    APP_GAME_ADAPTER_EXECUTION_LABELS
        .iter()
        .find(|(raw, _)| *raw == state)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| state.to_string())
}
