use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_settings::ScreenAnalysisParentSetting;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateKind;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateResponse;

use crate::screen_settings_store::{ScreenSettingsAuditRecord, ScreenSettingsRevisionRecord};
use crate::time::timestamp_now;

use super::{
    accepted_response, next_audit_event_id, next_revision_id, rejected_response, validation,
    ScreenSettingsId, ScreenSettingsRequestId, ScreenSettingsRuntime, ScreenSettingsText,
};

pub(super) async fn handle_replace(
    runtime: &ScreenSettingsRuntime,
    request_id: ScreenSettingsRequestId,
    base_setting_version: Option<u64>,
    setting: ScreenAnalysisParentSetting,
) -> ScreenSettingsUpdateResponse {
    let generated_at: String = timestamp_now();
    if let Err(reason) = validation::validate_screen_setting(&setting) {
        return rejected_response(
            request_id,
            ScreenSettingsUpdateKind::Replace,
            reason,
            ScreenSettingsText(constants::screen_settings::MESSAGE_INVALID_SETTING.to_string()),
        );
    }
    let mut state = match runtime.read_state().await {
        Ok(state) => state,
        Err(_) => {
            return rejected_response(
                request_id,
                ScreenSettingsUpdateKind::Replace,
                ScreenSettingsRejectionReason::StorageUnavailable,
                ScreenSettingsText(
                    constants::screen_settings::MESSAGE_STORAGE_UNAVAILABLE.to_string(),
                ),
            );
        }
    };
    if state.active_setting_version != base_setting_version {
        return rejected_response(
            request_id,
            ScreenSettingsUpdateKind::Replace,
            ScreenSettingsRejectionReason::StaleRevision,
            ScreenSettingsText(constants::screen_settings::MESSAGE_STALE_REVISION.to_string()),
        );
    }
    let audit_event_id = next_audit_event_id(&state);
    state.active_setting_version = Some(setting.setting_version);
    state.settings.push(ScreenSettingsRevisionRecord {
        revision_id: next_revision_id(&state).0,
        setting: setting.clone(),
        created_at: generated_at.clone(),
        audit_event_id: audit_event_id.clone().0,
    });
    state.audit_events.push(ScreenSettingsAuditRecord {
        audit_event_id: audit_event_id.clone().0,
        request_id: request_id.clone().0,
        kind: ScreenSettingsUpdateKind::Replace,
        setting_version: setting.setting_version,
        created_at: generated_at,
    });
    if runtime.write_state(&state).await.is_err() {
        return rejected_response(
            request_id,
            ScreenSettingsUpdateKind::Replace,
            ScreenSettingsRejectionReason::StorageUnavailable,
            ScreenSettingsText(constants::screen_settings::MESSAGE_STORAGE_UNAVAILABLE.to_string()),
        );
    }
    accepted_response(
        request_id,
        ScreenSettingsUpdateKind::Replace,
        setting,
        Some(ScreenSettingsId(audit_event_id.0)),
        ScreenSettingsText(constants::screen_settings::MESSAGE_ACCEPTED.to_string()),
    )
}
