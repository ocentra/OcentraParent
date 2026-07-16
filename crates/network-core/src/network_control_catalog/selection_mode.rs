use super::{NetworkControlKind, NetworkControlSelectionMode};

pub fn selection_mode_for(
    control_kind: NetworkControlKind,
    option_count: usize,
) -> NetworkControlSelectionMode {
    if matches!(
        control_kind,
        NetworkControlKind::MultiChoice
            | NetworkControlKind::ActionList
            | NetworkControlKind::TargetList
    ) {
        return NetworkControlSelectionMode::Multi;
    }
    if option_count > 4 && control_kind != NetworkControlKind::ReadOnlyStatus {
        return NetworkControlSelectionMode::Multi;
    }
    NetworkControlSelectionMode::Single
}
