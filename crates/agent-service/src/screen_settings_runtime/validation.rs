use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_settings::ScreenAnalysisParentSetting;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsRejectionReason;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;

pub(super) fn validate_screen_setting(
    setting: &ScreenAnalysisParentSetting,
) -> Result<(), ScreenSettingsRejectionReason> {
    if setting.schema_version != SCREEN_EVIDENCE_SCHEMA_VERSION
        || setting.setting_version == 0
        || setting.cadence_seconds < constants::screen_settings::MIN_CADENCE_SECONDS
        || setting.cadence_seconds > constants::screen_settings::MAX_CADENCE_SECONDS
        || setting.temporary_image_ttl_seconds < constants::screen_settings::MIN_TTL_SECONDS
        || setting.temporary_image_ttl_seconds > constants::screen_settings::MAX_TTL_SECONDS
        || setting.max_retry_count > constants::screen_settings::MAX_RETRY_COUNT
        || setting.ocr_text_snippet_limit > constants::screen_settings::MAX_OCR_SNIPPET_LIMIT
        || !setting.credential_suppression_enabled
        || !setting.delete_after_success
        || !setting.delete_after_expiry
    {
        return Err(ScreenSettingsRejectionReason::InvalidSetting);
    }
    if setting.retain_raw_image && !raw_retention_local_ttl_allowed(setting) {
        return Err(ScreenSettingsRejectionReason::RawRetentionForbidden);
    }
    if !setting.screen_analysis_enabled
        && (setting.cadence_capture_enabled
            || setting.strict_mode_enabled
            || setting.trigger_capture_enabled
            || setting.policy_use_enabled)
    {
        return Err(ScreenSettingsRejectionReason::DisabledSettingInconsistent);
    }
    if setting.policy_use_enabled
        && (!setting.screen_analysis_enabled
            || setting.analysis_mode == constants::screen_settings::ANALYSIS_MODE_OBSERVE_ONLY)
    {
        return Err(ScreenSettingsRejectionReason::PolicyModeInconsistent);
    }
    if setting.strict_mode_enabled
        && (!setting.screen_analysis_enabled
            || !setting.cadence_capture_enabled
            || setting.cadence_seconds != constants::screen_settings::STRICT_CADENCE_SECONDS)
    {
        return Err(ScreenSettingsRejectionReason::StrictModeInconsistent);
    }
    if setting.trigger_capture_enabled
        && (!setting.screen_analysis_enabled || setting.enabled_triggers.is_empty())
    {
        return Err(ScreenSettingsRejectionReason::TriggerModeInconsistent);
    }
    if !setting.ocr_text_enabled
        && (setting.ocr_text_snippet_limit != 0
            || setting.redaction_mode != constants::screen_settings::REDACTION_MODE_DISABLED
            || setting.ocr_text_retention_mode
                != constants::screen_settings::OCR_TEXT_RETENTION_DISABLED
            || setting.pii_redaction_enabled)
    {
        return Err(ScreenSettingsRejectionReason::OcrModeInconsistent);
    }
    if setting.ocr_text_enabled
        && (setting.ocr_text_snippet_limit == 0
            || setting.redaction_mode == constants::screen_settings::REDACTION_MODE_DISABLED
            || setting.ocr_text_retention_mode
                == constants::screen_settings::OCR_TEXT_RETENTION_DISABLED)
    {
        return Err(ScreenSettingsRejectionReason::OcrModeInconsistent);
    }
    Ok(())
}

fn raw_retention_local_ttl_allowed(setting: &ScreenAnalysisParentSetting) -> bool {
    setting.screen_analysis_enabled
        && setting.temporary_image_ttl_seconds
            <= constants::screen_settings::RAW_RETENTION_MAX_TTL_SECONDS
        && setting.delete_after_success
        && setting.delete_after_expiry
}
