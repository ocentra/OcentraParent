use ocentra_schema::parent_ui_bridge::ParentUiActionKind;

pub(super) fn action_result_message(action: &ParentUiActionKind) -> &'static str {
    match action {
        ParentUiActionKind::AppGameAdapterDispatchExecuteRequested => {
            "parent Rust facade requested app/game adapter dispatch execution"
        }
        ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested => {
            "parent Rust facade requested app/game timer parent preference setup"
        }
        _ => "parent Rust facade app/game action is unavailable",
    }
}
