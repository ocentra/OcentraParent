use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

use crate::{
    degraded_capture, CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureMetadata,
    ScreenCaptureScope,
};

pub(super) fn capture_primary_display_png() -> ScreenCaptureAttempt {
    let monitors = match xcap::Monitor::all() {
        Ok(monitors) => monitors,
        Err(_) => {
            return degraded_capture(
                ActivityCaptureCapabilityStatus::AdapterError,
                ScreenCaptureScope::PrimaryDisplay,
            );
        }
    };

    let Some(monitor) = monitors
        .iter()
        .find(|monitor| matches!(monitor.is_primary(), Ok(true)))
        .or_else(|| monitors.first())
    else {
        return degraded_capture(
            ActivityCaptureCapabilityStatus::NoActiveWindow,
            ScreenCaptureScope::PrimaryDisplay,
        );
    };

    let image = match monitor.capture_image() {
        Ok(image) => image,
        Err(_) => {
            return degraded_capture(
                ActivityCaptureCapabilityStatus::AccessDenied,
                ScreenCaptureScope::PrimaryDisplay,
            );
        }
    };

    let width = image.width();
    let height = image.height();
    let png_bytes = match encode_png(image) {
        Ok(png_bytes) => png_bytes,
        Err(_) => {
            return degraded_capture(
                ActivityCaptureCapabilityStatus::AdapterError,
                ScreenCaptureScope::PrimaryDisplay,
            );
        }
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

fn encode_png(image: xcap::image::RgbaImage) -> Result<Vec<u8>, xcap::image::ImageError> {
    let mut writer = std::io::Cursor::new(Vec::new());
    let dynamic_image = xcap::image::DynamicImage::ImageRgba8(image);
    dynamic_image.write_to(&mut writer, xcap::image::ImageFormat::Png)?;
    Ok(writer.into_inner())
}
