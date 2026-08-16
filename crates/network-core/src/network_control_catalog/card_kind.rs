use super::{NetworkControlCardKind, NetworkControlKind, NetworkControlSelectionMode};

pub fn card_kind_for(
    control_kind: NetworkControlKind,
    selection_mode: NetworkControlSelectionMode,
    option_count: usize,
) -> NetworkControlCardKind {
    match control_kind {
        NetworkControlKind::Toggle => NetworkControlCardKind::Toggle,
        NetworkControlKind::Schedule => NetworkControlCardKind::ScheduleCard,
        NetworkControlKind::RuleList => NetworkControlCardKind::RuleListCard,
        NetworkControlKind::TargetList => NetworkControlCardKind::TargetListCard,
        NetworkControlKind::Retention => NetworkControlCardKind::RetentionCard,
        NetworkControlKind::ReadOnlyStatus => NetworkControlCardKind::StatusCard,
        NetworkControlKind::Number | NetworkControlKind::Duration => {
            NetworkControlCardKind::NumberCard
        }
        _ if selection_mode == NetworkControlSelectionMode::Multi => {
            if option_count > 4 {
                NetworkControlCardKind::MultiChoiceMany
            } else {
                NetworkControlCardKind::MultiChoiceNormal
            }
        }
        _ => {
            if option_count > 4 {
                NetworkControlCardKind::SingleChoiceMany
            } else {
                NetworkControlCardKind::SingleChoiceCompact
            }
        }
    }
}
