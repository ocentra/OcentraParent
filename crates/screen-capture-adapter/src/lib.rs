use ocentra_parent_agent_protocol::ActivityCaptureCapabilityStatus;

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

#[cfg(not(windows))]
fn degraded_selected_window(status: ActivityCaptureCapabilityStatus) -> ScreenCaptureAttempt {
    degraded_capture(status, ScreenCaptureScope::SelectedWindow)
}

fn degraded_primary_display(status: ActivityCaptureCapabilityStatus) -> ScreenCaptureAttempt {
    degraded_capture(status, ScreenCaptureScope::PrimaryDisplay)
}

#[cfg(windows)]
fn platform_capture_active_window_png() -> ScreenCaptureAttempt {
    platform_capture_window_png(
        |window| {
            matches!(window.is_focused(), Ok(true)) && !matches!(window.is_minimized(), Ok(true))
        },
        ScreenCaptureScope::ActiveWindow,
    )
}

#[cfg(windows)]
fn platform_capture_window_title_contains_png(title_contains: &str) -> ScreenCaptureAttempt {
    platform_capture_window_png(
        |window| {
            !matches!(window.is_minimized(), Ok(true))
                && window
                    .title()
                    .is_ok_and(|title| title.contains(title_contains))
        },
        ScreenCaptureScope::SelectedWindow,
    )
}

#[cfg(windows)]
fn platform_capture_window_png(
    matches_window: impl Fn(&xcap::Window) -> bool,
    scope: ScreenCaptureScope,
) -> ScreenCaptureAttempt {
    let windows = match xcap::Window::all() {
        Ok(windows) => windows,
        Err(_) => return degraded_capture(ActivityCaptureCapabilityStatus::AdapterError, scope),
    };

    let Some(window) = windows.into_iter().find(matches_window) else {
        return degraded_capture(ActivityCaptureCapabilityStatus::NoActiveWindow, scope);
    };

    let image = match window.capture_image() {
        Ok(image) => image,
        Err(_) => return degraded_capture(ActivityCaptureCapabilityStatus::AccessDenied, scope),
    };

    let width = image.width();
    let height = image.height();
    let png_bytes = match encode_png(image) {
        Ok(png_bytes) => png_bytes,
        Err(_) => return degraded_capture(ActivityCaptureCapabilityStatus::AdapterError, scope),
    };

    ScreenCaptureAttempt::Captured(CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope,
            pid: window.pid().ok(),
            app_name: window.app_name().ok(),
            title: window.title().ok(),
            window_id: window.id().ok(),
            monitor_id: window
                .current_monitor()
                .ok()
                .and_then(|monitor| monitor.id().ok()),
            monitor_name: window
                .current_monitor()
                .ok()
                .and_then(|monitor| monitor.name().ok()),
        },
        width,
        height,
        png_bytes,
    })
}

#[cfg(windows)]
fn platform_capture_primary_display_png() -> ScreenCaptureAttempt {
    let monitors = match xcap::Monitor::all() {
        Ok(monitors) => monitors,
        Err(_) => return degraded_primary_display(ActivityCaptureCapabilityStatus::AdapterError),
    };

    let Some(monitor) = monitors
        .iter()
        .find(|monitor| matches!(monitor.is_primary(), Ok(true)))
        .or_else(|| monitors.first())
    else {
        return degraded_primary_display(ActivityCaptureCapabilityStatus::NoActiveWindow);
    };

    let image = match monitor.capture_image() {
        Ok(image) => image,
        Err(_) => return degraded_primary_display(ActivityCaptureCapabilityStatus::AccessDenied),
    };

    let width = image.width();
    let height = image.height();
    let png_bytes = match encode_png(image) {
        Ok(png_bytes) => png_bytes,
        Err(_) => return degraded_primary_display(ActivityCaptureCapabilityStatus::AdapterError),
    };

    ScreenCaptureAttempt::Captured(CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope: ScreenCaptureScope::PrimaryDisplay,
            pid: None,
            app_name: None,
            title: None,
            window_id: None,
            monitor_id: monitor.id().ok(),
            monitor_name: monitor.name().ok(),
        },
        width,
        height,
        png_bytes,
    })
}

#[cfg(windows)]
fn encode_png(image: xcap::image::RgbaImage) -> Result<Vec<u8>, xcap::image::ImageError> {
    let mut writer = std::io::Cursor::new(Vec::new());
    let dynamic_image = xcap::image::DynamicImage::ImageRgba8(image);
    dynamic_image.write_to(&mut writer, xcap::image::ImageFormat::Png)?;
    Ok(writer.into_inner())
}

#[cfg(not(windows))]
fn platform_capture_active_window_png() -> ScreenCaptureAttempt {
    degraded_capture(
        ActivityCaptureCapabilityStatus::Unavailable,
        ScreenCaptureScope::ActiveWindow,
    )
}

#[cfg(not(windows))]
fn platform_capture_window_title_contains_png(_title_contains: &str) -> ScreenCaptureAttempt {
    degraded_selected_window(ActivityCaptureCapabilityStatus::Unavailable)
}

#[cfg(not(windows))]
fn platform_capture_primary_display_png() -> ScreenCaptureAttempt {
    degraded_primary_display(ActivityCaptureCapabilityStatus::Unavailable)
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
