use super::*;

pub(super) fn app_game_adapter_dispatch_execute_summary_details(
    execute_result: Option<&Value>,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    let Some(execute_result) = execute_result else {
        return Vec::new();
    };
    vec![
        app_game_detail(
            "Execute command",
            execute_result
                .get("commandId")
                .and_then(Value::as_str)
                .unwrap_or("Not reported"),
        ),
        app_game_detail(
            "Execute status",
            execute_result
                .get("executionStatus")
                .and_then(Value::as_str)
                .unwrap_or("Not reported"),
        ),
        app_game_detail(
            "Execute result",
            execute_result
                .get("executionResultId")
                .and_then(Value::as_str)
                .unwrap_or("Not reported"),
        ),
        app_game_detail(
            "Adapter execution status",
            execute_result
                .get("executionAdapterResultCode")
                .and_then(Value::as_str)
                .unwrap_or("Not reported"),
        ),
        app_game_detail(
            "Execute audit",
            execute_result
                .get("executionAuditEventId")
                .and_then(Value::as_str)
                .unwrap_or("Not reported"),
        ),
        app_game_detail(
            "Execute readback",
            execute_result
                .get("readbackCommandName")
                .and_then(Value::as_str)
                .unwrap_or("Not reported"),
        ),
        app_game_detail(
            "Adapter dispatch",
            app_game_claimed_value(
                execute_result
                    .get("adapterDispatchExecutedClaimed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        app_game_detail(
            "Platform state",
            app_game_claimed_value(
                execute_result
                    .get("platformEnforcementClaimed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        app_game_detail(
            "Child delivery",
            app_game_claimed_value(
                execute_result
                    .get("childDeviceDeliveryClaimed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
    ]
}
