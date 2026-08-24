use std::{
    ffi::OsString,
    io::{Cursor, Read, Seek, SeekFrom},
    path::Path,
    time::Instant,
};

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use png::{Decoder, Limits};
use tempfile::NamedTempFile;

use super::super::linux_foreground_source::{
    linux_process::{executable_path, run_child},
    linux_tools::{capture_failure_status, LinuxWindowSelector},
};
use super::super::{
    degraded_capture, CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureMetadata,
    ScreenCaptureScope,
};

const MAX_CAPTURE_ARTIFACT_BYTES: u64 = 32 * 1024 * 1024;
const MAX_CAPTURE_DECODED_BYTES: usize = 128 * 1024 * 1024;

pub(super) fn capture_x11_window_png(
    window: &LinuxWindowSelector,
    scope: ScreenCaptureScope,
    deadline: Instant,
) -> ScreenCaptureAttempt {
    capture_x11(
        vec![OsString::from("-id"), window.xwd_argument()],
        scope,
        deadline,
    )
}

pub(super) fn capture_x11_png(
    scope: ScreenCaptureScope,
    deadline: Instant,
) -> ScreenCaptureAttempt {
    capture_x11(vec![OsString::from("-root")], scope, deadline)
}

fn capture_x11(
    selector_args: Vec<OsString>,
    scope: ScreenCaptureScope,
    deadline: Instant,
) -> ScreenCaptureAttempt {
    let Some(xwd_program) = executable_path("xwd") else {
        return degraded_capture(ActivityCaptureCapabilityStatus::Unavailable, scope);
    };
    let Some(convert_program) = executable_path("convert") else {
        return degraded_capture(ActivityCaptureCapabilityStatus::Unavailable, scope);
    };

    let captured = capture_artifact(selector_args, &xwd_program, &convert_program, deadline);
    let (png_bytes, width, height) = match captured {
        Ok(value) => value,
        Err(status) => return degraded_capture(status, scope),
    };

    ScreenCaptureAttempt::Captured(CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope,
            pid: None,
            app_name: None,
            title: None,
            window_id: None,
            monitor_id: None,
            monitor_name: None,
        },
        width,
        height,
        png_bytes,
    })
}

fn capture_artifact(
    selector_args: Vec<OsString>,
    xwd_program: &Path,
    convert_program: &Path,
    deadline: Instant,
) -> Result<(Vec<u8>, u32, u32), ActivityCaptureCapabilityStatus> {
    let xwd_file = tempfile::Builder::new()
        .prefix("ocentra-screen-x11-")
        .suffix(".xwd")
        .tempfile()
        .map_err(|_| ActivityCaptureCapabilityStatus::AdapterError)?;
    let mut png_file = tempfile::Builder::new()
        .prefix("ocentra-screen-x11-")
        .suffix(".png")
        .tempfile()
        .map_err(|_| ActivityCaptureCapabilityStatus::AdapterError)?;
    let xwd_path = xwd_file.path();
    let png_path = png_file.path();

    let mut xwd_args = selector_args;
    xwd_args.extend([OsString::from("-silent"), OsString::from("-out")]);
    xwd_args.push(xwd_path.as_os_str().to_owned());
    let xwd_result = run_child(xwd_program, &xwd_args, deadline);
    if !xwd_result.succeeded() {
        return Err(capture_failure_status(&xwd_result));
    }
    if !artifact_within_limit(&xwd_file) {
        return Err(ActivityCaptureCapabilityStatus::AdapterError);
    }

    let convert_result = run_child(
        convert_program,
        &[
            xwd_path.as_os_str().to_owned(),
            png_path.as_os_str().to_owned(),
        ],
        deadline,
    );
    if !convert_result.succeeded() {
        return Err(capture_failure_status(&convert_result));
    }

    let png_bytes =
        read_capped_artifact(&mut png_file).ok_or(ActivityCaptureCapabilityStatus::AdapterError)?;
    let (width, height) =
        png_dimensions(&png_bytes).ok_or(ActivityCaptureCapabilityStatus::AdapterError)?;
    Ok((png_bytes, width, height))
}

fn artifact_within_limit(file: &NamedTempFile) -> bool {
    file.as_file()
        .metadata()
        .map(|metadata| metadata.len() > 0 && metadata.len() <= MAX_CAPTURE_ARTIFACT_BYTES)
        .unwrap_or(false)
}

fn read_capped_artifact(file: &mut NamedTempFile) -> Option<Vec<u8>> {
    file.as_file_mut().seek(SeekFrom::Start(0)).ok()?;
    let metadata = file.as_file().metadata().ok()?;
    if metadata.len() == 0 || metadata.len() > MAX_CAPTURE_ARTIFACT_BYTES {
        return None;
    }
    let mut bytes = Vec::new();
    let bytes_read = file
        .as_file_mut()
        .take(MAX_CAPTURE_ARTIFACT_BYTES + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes_read == 0 || bytes_read as u64 > MAX_CAPTURE_ARTIFACT_BYTES {
        return None;
    }
    Some(bytes)
}

fn png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    let decoder = Decoder::new_with_limits(
        Cursor::new(bytes),
        Limits {
            bytes: usize::try_from(MAX_CAPTURE_ARTIFACT_BYTES).ok()?,
        },
    );
    let mut reader = decoder.read_info().ok()?;
    let decoded_bytes = reader.output_buffer_size()?;
    if decoded_bytes == 0 || decoded_bytes > MAX_CAPTURE_DECODED_BYTES {
        return None;
    }
    let mut decoded = vec![0; decoded_bytes];
    let output = reader.next_frame(&mut decoded).ok()?;
    (output.width > 0 && output.height > 0).then_some((output.width, output.height))
}
