use ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState;
use ocentra_parent_agent_protocol::constants;

pub(super) fn read_model_state_value(state: ActivityReadModelState) -> &'static str {
    match state {
        ActivityReadModelState::Ready => constants::activity_surface::STATE_READY,
        ActivityReadModelState::Empty => constants::activity_surface::STATE_EMPTY,
        ActivityReadModelState::Unavailable => constants::activity_surface::STATE_UNAVAILABLE,
        ActivityReadModelState::Offline => constants::activity_surface::STATE_OFFLINE,
        ActivityReadModelState::Stale => constants::activity_surface::STATE_STALE,
        ActivityReadModelState::PermissionRequired => {
            constants::activity_surface::STATE_PERMISSION_REQUIRED
        }
        ActivityReadModelState::ScaffoldOnly => constants::activity_surface::STATE_SCAFFOLD_ONLY,
    }
}
