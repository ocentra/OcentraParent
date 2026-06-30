use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateResponse;

use crate::{fields::fields_from_pairs, json_contract::serialize_json_string};

pub(crate) fn screen_settings_response_payload(
    response: &ScreenSettingsUpdateResponse,
) -> LogFields {
    let mut fields = fields_from_pairs(vec![
        (
            constants::field::SCREEN_SETTINGS_RESPONSE,
            LogFieldValue::String(serialize_json_string(response)),
        ),
        (
            constants::field::SCREEN_SETTINGS_UPDATE_KIND,
            LogFieldValue::String(response.kind.as_protocol_str().to_string()),
        ),
    ]);
    if let Some(reason) = response.rejection_reason {
        fields.insert(
            constants::field::SCREEN_SETTINGS_REJECTION_REASON.to_string(),
            LogFieldValue::String(rejection_reason_protocol_str(reason).to_string()),
        );
    }
    fields
}

fn rejection_reason_protocol_str(reason: ScreenSettingsRejectionReason) -> &'static str {
    match reason {
        ScreenSettingsRejectionReason::StorageUnavailable => {
            constants::screen_settings::REJECTION_STORAGE_UNAVAILABLE
        }
        ScreenSettingsRejectionReason::InvalidSetting => {
            constants::screen_settings::REJECTION_INVALID_SETTING
        }
        ScreenSettingsRejectionReason::StaleRevision => {
            constants::screen_settings::REJECTION_STALE_REVISION
        }
        ScreenSettingsRejectionReason::RawRetentionForbidden => {
            constants::screen_settings::REJECTION_RAW_RETENTION_FORBIDDEN
        }
        ScreenSettingsRejectionReason::DisabledSettingInconsistent => {
            constants::screen_settings::REJECTION_DISABLED_SETTING_INCONSISTENT
        }
        ScreenSettingsRejectionReason::PolicyModeInconsistent => {
            constants::screen_settings::REJECTION_POLICY_MODE_INCONSISTENT
        }
        ScreenSettingsRejectionReason::StrictModeInconsistent => {
            constants::screen_settings::REJECTION_STRICT_MODE_INCONSISTENT
        }
        ScreenSettingsRejectionReason::TriggerModeInconsistent => {
            constants::screen_settings::REJECTION_TRIGGER_MODE_INCONSISTENT
        }
        ScreenSettingsRejectionReason::OcrModeInconsistent => {
            constants::screen_settings::REJECTION_OCR_MODE_INCONSISTENT
        }
    }
}
