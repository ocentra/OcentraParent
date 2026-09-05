use ocentra_schema::parent_ui_bridge::ParentUiActionKind;

pub(super) fn action_result_message(action: &ParentUiActionKind) -> &'static str {
    match action {
        ParentUiActionKind::PolicyPreviewAuthoringDraftStaged => {
            "parent Rust facade staged a policy preview draft"
        }
        ParentUiActionKind::PolicyPreviewAuthoringDraftCancelled => {
            "parent Rust facade invalidated a policy preview draft"
        }
        ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested => {
            "parent Rust facade requested policy preview parent confirmation"
        }
        ParentUiActionKind::PolicyRequestParentResolutionRequested => {
            "parent Rust facade requested parent policy request resolution"
        }
        _ => "parent Rust facade policy action is unavailable",
    }
}
