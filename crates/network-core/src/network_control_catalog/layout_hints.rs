use super::{NetworkControlKind, NetworkControlLayoutHints, NetworkControlSelectionMode};

pub fn layout_hints_for(
    control_kind: NetworkControlKind,
    selection_mode: NetworkControlSelectionMode,
    option_count: usize,
) -> NetworkControlLayoutHints {
    let many_options = option_count > 4;
    let list_like = selection_mode == NetworkControlSelectionMode::Multi
        || matches!(
            control_kind,
            NetworkControlKind::ActionList | NetworkControlKind::TargetList
        );
    NetworkControlLayoutHints {
        preferred_column_span: if many_options
            || matches!(
                control_kind,
                NetworkControlKind::Retention | NetworkControlKind::ReadOnlyStatus
            ) {
            2
        } else {
            1
        },
        collapsible: many_options
            || list_like
            || control_kind == NetworkControlKind::ReadOnlyStatus,
        searchable_options: many_options,
        option_group_count: if many_options {
            option_count.div_ceil(4)
        } else {
            1
        },
        show_as_matrix_when_large: many_options && list_like,
        show_selected_count: list_like,
    }
}
