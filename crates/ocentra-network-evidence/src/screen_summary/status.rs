use super::{
    NetworkScreenSummaryPrivacyMode, NetworkScreenSummaryTriggerInput,
    NetworkScreenSummaryTriggerStatus,
};

pub(super) fn screen_summary_status(
    input: &NetworkScreenSummaryTriggerInput,
    screen_summary_recommended: bool,
) -> NetworkScreenSummaryTriggerStatus {
    if !screen_summary_recommended {
        return NetworkScreenSummaryTriggerStatus::NotRecommended;
    }
    if !input.screen_summary_enabled {
        return NetworkScreenSummaryTriggerStatus::DisabledByParent;
    }
    if input.protected_surface_detected {
        return NetworkScreenSummaryTriggerStatus::ProtectedSurfaceUnavailable;
    }
    if !input.debounce_clear {
        return NetworkScreenSummaryTriggerStatus::Debounced;
    }
    if !input.queue_available {
        return NetworkScreenSummaryTriggerStatus::QueueUnavailable;
    }
    if !input.encrypted_temporary_custody_available
        || !input.delete_after_analysis_available
        || !input.local_only_runtime_available
    {
        return NetworkScreenSummaryTriggerStatus::CustodyManualRequired;
    }
    NetworkScreenSummaryTriggerStatus::Queued
}

pub(super) fn privacy_mode_for(
    status: NetworkScreenSummaryTriggerStatus,
    screen_summary_recommended: bool,
) -> NetworkScreenSummaryPrivacyMode {
    if !screen_summary_recommended {
        return NetworkScreenSummaryPrivacyMode::NetworkOnly;
    }
    match status {
        NetworkScreenSummaryTriggerStatus::Queued => {
            NetworkScreenSummaryPrivacyMode::ActiveWindowScreenIfEnabled
        }
        _ => NetworkScreenSummaryPrivacyMode::ScreenManualRequired,
    }
}
