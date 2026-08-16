use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

use crate::{
    degraded_capture, CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureMetadata,
    ScreenCaptureScope, ScreenCaptureWindowTitleQuery,
};

pub(super) fn capture_active_window_png() -> ScreenCaptureAttempt {
    capture_window_png(
        |window| {
            matches!(window.is_focused(), Ok(true)) && !matches!(window.is_minimized(), Ok(true))
        },
        ScreenCaptureScope::ActiveWindow,
    )
}

pub(super) fn capture_window_title_contains_png(
    title_query: &ScreenCaptureWindowTitleQuery,
) -> ScreenCaptureAttempt {
    capture_window_png(
        |window| {
            !matches!(window.is_minimized(), Ok(true))
                && window
                    .title()
                    .is_ok_and(|title| title.contains(title_query.as_str()))
        },
        ScreenCaptureScope::SelectedWindow,
    )
}

fn capture_window_png(
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

fn encode_png(image: xcap::image::RgbaImage) -> Result<Vec<u8>, xcap::image::ImageError> {
    let mut writer = std::io::Cursor::new(Vec::new());
    let dynamic_image = xcap::image::DynamicImage::ImageRgba8(image);
    dynamic_image.write_to(&mut writer, xcap::image::ImageFormat::Png)?;
    Ok(writer.into_inner())
}
