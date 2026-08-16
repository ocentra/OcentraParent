use super::SurfaceText;
use ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState;
use ocentra_parent_agent_protocol::constants;

pub(super) fn read_model_state_value(state: ActivityReadModelState) -> SurfaceText {
    match state {
        ActivityReadModelState::Ready => SurfaceText(constants::activity_surface::STATE_READY),
        ActivityReadModelState::Empty => SurfaceText(constants::activity_surface::STATE_EMPTY),
        ActivityReadModelState::Unavailable => {
            SurfaceText(constants::activity_surface::STATE_UNAVAILABLE)
        }
        ActivityReadModelState::Offline => SurfaceText(constants::activity_surface::STATE_OFFLINE),
        ActivityReadModelState::Stale => SurfaceText(constants::activity_surface::STATE_STALE),
        ActivityReadModelState::PermissionRequired => {
            SurfaceText(constants::activity_surface::STATE_PERMISSION_REQUIRED)
        }
        ActivityReadModelState::ScaffoldOnly => {
            SurfaceText(constants::activity_surface::STATE_SCAFFOLD_ONLY)
        }
    }
}
