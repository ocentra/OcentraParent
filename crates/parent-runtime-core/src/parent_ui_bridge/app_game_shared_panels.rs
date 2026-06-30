use super::*;

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
    match state {
        "dispatch-eligible" => "Dispatch eligible".to_string(),
        "blocked-before-dispatch" => "Blocked before dispatch".to_string(),
        "manual-required" => "Manual proof required".to_string(),
        "unavailable" => "Adapter unavailable".to_string(),
        "unsupported" => "Platform unsupported".to_string(),
        "degraded" => "Adapter degraded".to_string(),
        _ => state.to_string(),
    }
}

pub(super) fn app_game_adapter_dispatch_preflight_decision_label(decision: &str) -> String {
    match decision {
        "dispatch-eligible" => "Dispatch eligible".to_string(),
        _ => "Blocked before dispatch".to_string(),
    }
}

pub(super) fn app_game_adapter_dispatch_preflight_outcome_label(outcome: &str) -> String {
    match outcome {
        "dispatch-ready" => "Dispatch ready".to_string(),
        _ => outcome.to_string(),
    }
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
    match state {
        "command-accepted" => "Command accepted".to_string(),
        "blocked-before-command" => "Blocked before command".to_string(),
        "manual-required" => "Manual proof required".to_string(),
        "unavailable" => "Adapter unavailable".to_string(),
        "unsupported" => "Platform unsupported".to_string(),
        "degraded" => "Adapter degraded".to_string(),
        _ => state.to_string(),
    }
}

pub(super) fn app_game_adapter_dispatch_result_decision_label(decision: &str) -> String {
    match decision {
        "command-accepted" => "Command accepted".to_string(),
        _ => "Blocked before command".to_string(),
    }
}

pub(super) fn app_game_adapter_dispatch_execution_audit_label(state: &str) -> String {
    match state {
        "service-local-audit-recorded" => "Service-local audit recorded".to_string(),
        "blocked-before-execution-audit" => "Blocked before execution audit".to_string(),
        _ => state.to_string(),
    }
}

pub(super) fn app_game_adapter_dispatch_adapter_execution_label(state: &str) -> String {
    match state {
        "adapter-execution-reported" => "Adapter execution reported".to_string(),
        "adapter-execution-evidence-missing" => "Execution evidence missing".to_string(),
        "blocked-before-adapter-execution" => "Blocked before adapter execution".to_string(),
        _ => state.to_string(),
    }
}
