use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateKind;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateResponse;

use crate::time::timestamp_now;

use super::{
    accepted_response, default_disabled_setting, rejected_response, ScreenSettingsRequestId,
    ScreenSettingsRuntime, ScreenSettingsText, ScreenSettingsTimestamp,
};

pub(super) async fn handle_get(
    runtime: &ScreenSettingsRuntime,
    request_id: ScreenSettingsRequestId,
) -> ScreenSettingsUpdateResponse {
    let generated_at: String = timestamp_now();
    match runtime.read_state().await {
        Ok(state) => {
            let setting = state
                .active_setting()
                .map(|record| record.setting.clone())
                .unwrap_or_else(|| default_disabled_setting(ScreenSettingsTimestamp(generated_at)));
            accepted_response(
                request_id,
                ScreenSettingsUpdateKind::Get,
                setting,
                None,
                ScreenSettingsText(constants::screen_settings::MESSAGE_REPORTED.to_string()),
            )
        }
        Err(_) => rejected_response(
            request_id,
            ScreenSettingsUpdateKind::Get,
            ScreenSettingsRejectionReason::StorageUnavailable,
            ScreenSettingsText(constants::screen_settings::MESSAGE_STORAGE_UNAVAILABLE.to_string()),
        ),
    }
}
