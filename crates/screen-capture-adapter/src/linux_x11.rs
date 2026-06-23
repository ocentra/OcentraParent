use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

use crate::{
    degraded_capture, CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureMetadata,
    ScreenCaptureScope, ScreenCaptureWindowTitleQuery,
};

pub(super) fn capture_active_window_png() -> ScreenCaptureAttempt {
    let Some(window_id) = active_x11_window_id() else {
        return degraded_capture(
            ActivityCaptureCapabilityStatus::NoActiveWindow,
            ScreenCaptureScope::ActiveWindow,
        );
    };
    capture_x11_window_png(&window_id, ScreenCaptureScope::ActiveWindow, None)
}

pub(super) fn capture_window_title_contains_png(
    title_query: &ScreenCaptureWindowTitleQuery,
) -> ScreenCaptureAttempt {
    let Some(selection) = find_x11_window_by_title(title_query) else {
        return degraded_capture(
            ActivityCaptureCapabilityStatus::NoActiveWindow,
            ScreenCaptureScope::SelectedWindow,
        );
    };
    capture_x11_window_png(
        &selection.window_id,
        ScreenCaptureScope::SelectedWindow,
        selection.title,
    )
}

pub(super) fn capture_primary_display_png() -> ScreenCaptureAttempt {
    capture_x11_root_png()
}

struct X11WindowSelection {
    window_id: String,
    title: Option<String>,
}

fn active_x11_window_id() -> Option<String> {
    let output = std::process::Command::new("xprop")
        .args(["-root", "_NET_ACTIVE_WINDOW"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    let window_id = stdout.split_whitespace().last()?.trim();
    if window_id == "0x0" {
        None
    } else {
        Some(window_id.to_owned())
    }
}

fn find_x11_window_by_title(
    title_query: &ScreenCaptureWindowTitleQuery,
) -> Option<X11WindowSelection> {
    let output = std::process::Command::new("xwininfo")
        .args(["-root", "-tree"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    stdout.lines().find_map(|line| {
        if !line.contains(title_query.as_str()) {
            return None;
        }
        let window_id = line.split_whitespace().next()?.to_owned();
        Some(X11WindowSelection {
            window_id,
            title: quoted_x11_title(line),
        })
    })
}

fn quoted_x11_title(line: &str) -> Option<String> {
    let start = line.find('"')? + 1;
    let rest = line.get(start..)?;
    let end = rest.find('"')?;
    Some(rest.get(..end)?.to_owned())
}

fn capture_x11_root_png() -> ScreenCaptureAttempt {
    capture_x11_png(
        ["-root"].as_slice(),
        ScreenCaptureScope::PrimaryDisplay,
        None,
        None,
    )
}

fn capture_x11_window_png(
    window_id: &str,
    scope: ScreenCaptureScope,
    title: Option<String>,
) -> ScreenCaptureAttempt {
    capture_x11_png(["-id", window_id].as_slice(), scope, Some(window_id), title)
}

fn capture_x11_png(
    selector_args: &[&str],
    scope: ScreenCaptureScope,
    window_id: Option<&str>,
    title: Option<String>,
) -> ScreenCaptureAttempt {
    let temp_stem = x11_temp_stem();
    let xwd_path = std::env::temp_dir().join(format!("{temp_stem}.xwd"));
    let png_path = std::env::temp_dir().join(format!("{temp_stem}.png"));
    let mut xwd_args = selector_args.to_vec();
    xwd_args.push("-silent");
    xwd_args.push("-out");
    let xwd_status = std::process::Command::new("xwd")
        .args(&xwd_args)
        .arg(&xwd_path)
        .status();
    let Ok(xwd_status) = xwd_status else {
        return degraded_capture(ActivityCaptureCapabilityStatus::AdapterError, scope);
    };
    if !xwd_status.success() {
        let _ = std::fs::remove_file(&xwd_path);
        return degraded_capture(ActivityCaptureCapabilityStatus::AccessDenied, scope);
    }

    let convert_status = std::process::Command::new("convert")
        .arg(&xwd_path)
        .arg(&png_path)
        .status();
    let _ = std::fs::remove_file(&xwd_path);
    let Ok(convert_status) = convert_status else {
        return degraded_capture(ActivityCaptureCapabilityStatus::AdapterError, scope);
    };
    if !convert_status.success() {
        let _ = std::fs::remove_file(&png_path);
        return degraded_capture(ActivityCaptureCapabilityStatus::AdapterError, scope);
    }

    let png_bytes = match std::fs::read(&png_path) {
        Ok(bytes) => bytes,
        Err(_) => return degraded_capture(ActivityCaptureCapabilityStatus::AdapterError, scope),
    };
    let dimensions = x11_png_dimensions(&png_path);
    let _ = std::fs::remove_file(&png_path);
    let Some((width, height)) = dimensions else {
        return degraded_capture(ActivityCaptureCapabilityStatus::AdapterError, scope);
    };

    ScreenCaptureAttempt::Captured(CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope,
            pid: None,
            app_name: Some("x11-window".to_owned()),
            title,
            window_id: window_id.and_then(parse_x11_window_id),
            monitor_id: None,
            monitor_name: std::env::var("XDG_SESSION_TYPE").ok(),
        },
        width,
        height,
        png_bytes,
    })
}

fn x11_png_dimensions(path: &std::path::Path) -> Option<(u32, u32)> {
    let output = std::process::Command::new("identify")
        .args(["-format", "%w %h"])
        .arg(path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    let mut parts = stdout.split_whitespace();
    let width = parts.next()?.parse().ok()?;
    let height = parts.next()?.parse().ok()?;
    Some((width, height))
}

fn parse_x11_window_id(value: &str) -> Option<u32> {
    u32::from_str_radix(value.trim_start_matches("0x"), 16).ok()
}

fn x11_temp_stem() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("ocentra-screen-x11-{}-{now}", std::process::id())
}
