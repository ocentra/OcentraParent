mod screen_capture_real_proof_support;

use std::{env, fs::create_dir_all, path::PathBuf};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_screen_capture_adapter::{
    capture_active_window_png, capture_primary_display_png, capture_window_title_contains_png,
    ScreenCaptureAttempt, ScreenCaptureScope, ScreenCaptureWindowTitleQuery,
};
use screen_capture_real_proof_support::{
    proof_scope_label, run_id, write_captured_artifacts, write_degraded_artifacts,
    write_run_metadata, write_trigger_input, DEFAULT_DIR,
};

fn main() {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_DIR));
    create_dir_all(&output_dir).expect(constants::error::JOURNAL_OPENS);

    let run_id = run_id();
    let target_title = env::var("OCENTRA_SCREEN_CAPTURE_WINDOW_TITLE_CONTAINS")
        .ok()
        .and_then(|value| ScreenCaptureWindowTitleQuery::try_from(value).ok());
    let requested_scope = env::var("OCENTRA_SCREEN_CAPTURE_SCOPE")
        .ok()
        .as_deref()
        .map(requested_scope)
        .unwrap_or_else(|| {
            if target_title.is_some() {
                ScreenCaptureScope::SelectedWindow
            } else {
                ScreenCaptureScope::ActiveWindow
            }
        });
    let keep_raw_until_analysis =
        env::var("OCENTRA_SCREEN_CAPTURE_KEEP_RAW_UNTIL_ANALYSIS").is_ok_and(|value| value == "1");
    let attempt = match requested_scope {
        ScreenCaptureScope::ActiveWindow => capture_active_window_png(),
        ScreenCaptureScope::SelectedWindow => target_title
            .as_ref()
            .map(capture_window_title_contains_png)
            .unwrap_or_else(capture_active_window_png),
        ScreenCaptureScope::PrimaryDisplay => capture_primary_display_png(),
    };
    let requested_scope_label = proof_scope_label(requested_scope);

    write_run_metadata(
        &output_dir,
        &run_id,
        attempt.status(),
        target_title.as_ref(),
        requested_scope_label,
        keep_raw_until_analysis,
    );
    write_trigger_input(&output_dir, requested_scope_label);

    match attempt {
        ScreenCaptureAttempt::Captured(image) => {
            write_captured_artifacts(
                &output_dir,
                &run_id,
                image,
                requested_scope_label,
                keep_raw_until_analysis,
            );
        }
        ScreenCaptureAttempt::Degraded(metadata) => {
            write_degraded_artifacts(&output_dir, metadata.status);
        }
    }
}

fn requested_scope(value: &str) -> ScreenCaptureScope {
    match value {
        "selected-window" => ScreenCaptureScope::SelectedWindow,
        "primary-display" => ScreenCaptureScope::PrimaryDisplay,
        _ => ScreenCaptureScope::ActiveWindow,
    }
}
