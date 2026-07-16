use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateKind;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateStatus;
use ocentra_parent_agent_protocol::transport::AgentEventName;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ScreenSettingsEventId(pub(super) &'static str);

pub(super) fn event_id_for_response(
    kind: ScreenSettingsUpdateKind,
    status: ScreenSettingsUpdateStatus,
) -> ScreenSettingsEventId {
    match (kind, status) {
        (ScreenSettingsUpdateKind::Replace, ScreenSettingsUpdateStatus::Accepted) => {
            ScreenSettingsEventId(constants::event_id::SCREEN_SETTINGS_REPLACE_ACCEPTED)
        }
        (ScreenSettingsUpdateKind::Replace, ScreenSettingsUpdateStatus::Rejected) => {
            ScreenSettingsEventId(constants::event_id::SCREEN_SETTINGS_REPLACE_REJECTED)
        }
        _ => ScreenSettingsEventId(constants::event_id::SCREEN_SETTINGS_REPORTED),
    }
}

pub(super) fn event_name_for_response(
    kind: ScreenSettingsUpdateKind,
    status: ScreenSettingsUpdateStatus,
) -> AgentEventName {
    match (kind, status) {
        (ScreenSettingsUpdateKind::Replace, ScreenSettingsUpdateStatus::Accepted) => {
            AgentEventName::AgentScreenSettingsReplaceAccepted
        }
        (ScreenSettingsUpdateKind::Replace, ScreenSettingsUpdateStatus::Rejected) => {
            AgentEventName::AgentScreenSettingsReplaceRejected
        }
        _ => AgentEventName::AgentScreenSettingsReported,
    }
}
