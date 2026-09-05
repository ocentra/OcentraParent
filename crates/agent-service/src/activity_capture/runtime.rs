use ocentra_parent_agent_protocol::constants;

pub(super) fn startup_activity_capture_enabled_for_value(
    value: &super::StartupActivityCaptureDisabledValue<'_>,
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
