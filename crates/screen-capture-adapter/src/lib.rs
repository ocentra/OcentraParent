use ocentra_parent_agent_protocol::ActivityCaptureCapabilityStatus;

#[cfg(any(target_os = "windows", target_os = "macos"))]
mod desktop_xcap;
#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
mod linux_x11;
pub mod trigger_scheduler;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenCaptureScope {
    ActiveWindow,
    SelectedWindow,
    PrimaryDisplay,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenCaptureMetadata {
    pub status: ActivityCaptureCapabilityStatus,
    pub scope: ScreenCaptureScope,
    pub pid: Option<u32>,
    pub app_name: Option<String>,
    pub title: Option<String>,
    pub window_id: Option<u32>,
    pub monitor_id: Option<u32>,
    pub monitor_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapturedScreenImage {
    pub metadata: ScreenCaptureMetadata,
    pub width: u32,
    pub height: u32,
    pub png_bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScreenCaptureAttempt {
    Captured(CapturedScreenImage),
    Degraded(ScreenCaptureMetadata),
}

impl ScreenCaptureAttempt {
    pub fn status(&self) -> ActivityCaptureCapabilityStatus {
        match self {
            Self::Captured(image) => image.metadata.status.clone(),
            Self::Degraded(metadata) => metadata.status.clone(),
        }
    }
}

pub fn capture_active_window_png() -> ScreenCaptureAttempt {
    platform_capture_active_window_png()
}

pub fn capture_window_title_contains_png(title_contains: &str) -> ScreenCaptureAttempt {
    platform_capture_window_title_contains_png(title_contains)
}

pub fn capture_primary_display_png() -> ScreenCaptureAttempt {
    platform_capture_primary_display_png()
}

fn degraded_capture(
    status: ActivityCaptureCapabilityStatus,
    scope: ScreenCaptureScope,
) -> ScreenCaptureAttempt {
    ScreenCaptureAttempt::Degraded(ScreenCaptureMetadata {
        status,
        scope,
        pid: None,
        app_name: None,
        title: None,
        window_id: None,
        monitor_id: None,
        monitor_name: None,
    })
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    all(target_os = "linux", not(target_env = "ohos"))
)))]
fn degraded_selected_window(status: ActivityCaptureCapabilityStatus) -> ScreenCaptureAttempt {
    degraded_capture(status, ScreenCaptureScope::SelectedWindow)
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn platform_capture_active_window_png() -> ScreenCaptureAttempt {
    desktop_xcap::capture_active_window_png()
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn platform_capture_window_title_contains_png(title_contains: &str) -> ScreenCaptureAttempt {
    desktop_xcap::capture_window_title_contains_png(title_contains)
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn platform_capture_primary_display_png() -> ScreenCaptureAttempt {
    desktop_xcap::capture_primary_display_png()
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    all(target_os = "linux", not(target_env = "ohos"))
)))]
fn platform_capture_active_window_png() -> ScreenCaptureAttempt {
    degraded_capture(
        ActivityCaptureCapabilityStatus::Unavailable,
        ScreenCaptureScope::ActiveWindow,
    )
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    all(target_os = "linux", not(target_env = "ohos"))
)))]
fn platform_capture_window_title_contains_png(_title_contains: &str) -> ScreenCaptureAttempt {
    degraded_selected_window(ActivityCaptureCapabilityStatus::Unavailable)
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    all(target_os = "linux", not(target_env = "ohos"))
)))]
fn platform_capture_primary_display_png() -> ScreenCaptureAttempt {
    degraded_capture(
        ActivityCaptureCapabilityStatus::Unavailable,
        ScreenCaptureScope::PrimaryDisplay,
    )
}

#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
fn platform_capture_active_window_png() -> ScreenCaptureAttempt {
    linux_x11::capture_active_window_png()
}

#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
fn platform_capture_window_title_contains_png(title_contains: &str) -> ScreenCaptureAttempt {
    linux_x11::capture_window_title_contains_png(title_contains)
}

#[cfg(all(target_os = "linux", not(target_env = "ohos")))]
fn platform_capture_primary_display_png() -> ScreenCaptureAttempt {
    linux_x11::capture_primary_display_png()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degraded_attempt_reports_status_and_active_window_scope() {
        let attempt = degraded_capture(
            ActivityCaptureCapabilityStatus::AccessDenied,
            ScreenCaptureScope::ActiveWindow,
        );

        assert_eq!(
            attempt.status(),
            ActivityCaptureCapabilityStatus::AccessDenied
        );
        assert!(matches!(
            attempt,
            ScreenCaptureAttempt::Degraded(ScreenCaptureMetadata {
                scope: ScreenCaptureScope::ActiveWindow,
                ..
            })
        ));
    }

    #[test]
    fn captured_attempt_reports_available_status() {
        let attempt = ScreenCaptureAttempt::Captured(CapturedScreenImage {
            metadata: ScreenCaptureMetadata {
                status: ActivityCaptureCapabilityStatus::Available,
                scope: ScreenCaptureScope::ActiveWindow,
                pid: None,
                app_name: None,
                title: None,
                window_id: None,
                monitor_id: None,
                monitor_name: None,
            },
            width: 1,
            height: 1,
            png_bytes: vec![1],
        });

        assert_eq!(attempt.status(), ActivityCaptureCapabilityStatus::Available);
    }
}
