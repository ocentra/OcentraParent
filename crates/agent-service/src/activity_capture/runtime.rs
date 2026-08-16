use crate::activity_capture::StartupActivityCaptureDisabledValue;
use ocentra_parent_agent_protocol::constants;

pub(super) fn startup_activity_capture_enabled() -> bool {
    startup_activity_capture_enabled_for_value(&StartupActivityCaptureDisabledValue(
        std::env::var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED)
            .ok()
            .as_deref(),
    ))
}

pub(super) fn startup_activity_capture_enabled_for_value(
    value: &StartupActivityCaptureDisabledValue<'_>,
) -> bool {
    windows_activity_capture_supported() && value.0 != Some(constants::value::TRUE)
}

#[cfg(windows)]
fn windows_activity_capture_supported() -> bool {
    true
}

#[cfg(not(windows))]
fn windows_activity_capture_supported() -> bool {
    false
}
