use ocentra_parent_agent_protocol::ActivityCaptureCapabilityStatus;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenCaptureScope {
    ActiveWindow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenCaptureMetadata {
    pub status: ActivityCaptureCapabilityStatus,
    pub scope: ScreenCaptureScope,
    pub pid: Option<u32>,
    pub app_name: Option<String>,
    pub title: Option<String>,
    pub window_id: Option<u32>,
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

fn degraded_active_window(status: ActivityCaptureCapabilityStatus) -> ScreenCaptureAttempt {
    ScreenCaptureAttempt::Degraded(ScreenCaptureMetadata {
        status,
        scope: ScreenCaptureScope::ActiveWindow,
        pid: None,
        app_name: None,
        title: None,
        window_id: None,
    })
}

#[cfg(windows)]
fn platform_capture_active_window_png() -> ScreenCaptureAttempt {
    platform_capture_window_png(|window| {
        matches!(window.is_focused(), Ok(true)) && !matches!(window.is_minimized(), Ok(true))
    })
}

#[cfg(windows)]
fn platform_capture_window_title_contains_png(title_contains: &str) -> ScreenCaptureAttempt {
    platform_capture_window_png(|window| {
        !matches!(window.is_minimized(), Ok(true))
            && window
                .title()
                .is_ok_and(|title| title.contains(title_contains))
    })
}

#[cfg(windows)]
fn platform_capture_window_png(
    matches_window: impl Fn(&xcap::Window) -> bool,
) -> ScreenCaptureAttempt {
    let windows = match xcap::Window::all() {
        Ok(windows) => windows,
        Err(_) => return degraded_active_window(ActivityCaptureCapabilityStatus::AdapterError),
    };

    let Some(window) = windows.into_iter().find(matches_window) else {
        return degraded_active_window(ActivityCaptureCapabilityStatus::NoActiveWindow);
    };

    let image = match window.capture_image() {
        Ok(image) => image,
        Err(_) => return degraded_active_window(ActivityCaptureCapabilityStatus::AccessDenied),
    };

    let width = image.width();
    let height = image.height();
    let png_bytes = match encode_png(image) {
        Ok(png_bytes) => png_bytes,
        Err(_) => return degraded_active_window(ActivityCaptureCapabilityStatus::AdapterError),
    };

    ScreenCaptureAttempt::Captured(CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope: ScreenCaptureScope::ActiveWindow,
            pid: window.pid().ok(),
            app_name: window.app_name().ok(),
            title: window.title().ok(),
            window_id: window.id().ok(),
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
    degraded_active_window(ActivityCaptureCapabilityStatus::Unavailable)
}

#[cfg(not(windows))]
fn platform_capture_window_title_contains_png(_title_contains: &str) -> ScreenCaptureAttempt {
    degraded_active_window(ActivityCaptureCapabilityStatus::Unavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degraded_attempt_reports_status_and_active_window_scope() {
        let attempt = degraded_active_window(ActivityCaptureCapabilityStatus::AccessDenied);

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
            },
            width: 1,
            height: 1,
            png_bytes: vec![1],
        });

        assert_eq!(attempt.status(), ActivityCaptureCapabilityStatus::Available);
    }
}
