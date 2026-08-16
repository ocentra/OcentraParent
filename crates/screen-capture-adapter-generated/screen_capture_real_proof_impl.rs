use std::{env, fs::create_dir_all, path::PathBuf};

use crate::screen_capture_real_proof_support::{
    proof_scope_label, run_id, write_captured_artifacts, write_degraded_artifacts,
    write_run_metadata, write_trigger_input, ProofResult, ScreenCaptureProofError,
    ScreenCaptureProofPath, ScreenCaptureProofText, DEFAULT_OUTPUT_DIR_PATH,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_screen_capture_adapter::{
    capture_active_window_png, capture_primary_display_png, capture_window_title_contains_png,
    ScreenCaptureAttempt, ScreenCaptureScope, ScreenCaptureWindowTitleQuery,
};

const ENV_WINDOW_TITLE_CONTAINS: &str = "OCENTRA_SCREEN_CAPTURE_WINDOW_TITLE_CONTAINS";
const ENV_CAPTURE_SCOPE: &str = "OCENTRA_SCREEN_CAPTURE_SCOPE";
const ENV_KEEP_RAW_UNTIL_ANALYSIS: &str = "OCENTRA_SCREEN_CAPTURE_KEEP_RAW_UNTIL_ANALYSIS";
const KEEP_RAW_ENABLED_VALUE: &str = "1";
const SCOPE_SELECTED_WINDOW_INPUT: &str = "selected-window";
const SCOPE_PRIMARY_DISPLAY_INPUT: &str = "primary-display";

pub fn main() -> ProofResult<()> {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT_DIR_PATH));
    create_dir_all(&output_dir).map_err(|error| {
        ScreenCaptureProofError(format!("{}: {error:?}", constants::error::JOURNAL_OPENS))
    })?;

    let run_id = run_id()?;
    let target_title = env::var(ENV_WINDOW_TITLE_CONTAINS)
        .ok()
        .and_then(|value| ScreenCaptureWindowTitleQuery::try_from(value).ok());
    let requested_scope = env::var(ENV_CAPTURE_SCOPE)
        .ok()
        .map(ScreenCaptureProofText)
        .map(|value| requested_scope(&value))
        .unwrap_or_else(|| {
            if target_title.is_some() {
                ScreenCaptureScope::SelectedWindow
            } else {
                ScreenCaptureScope::ActiveWindow
            }
        });
    let keep_raw_until_analysis =
        env::var(ENV_KEEP_RAW_UNTIL_ANALYSIS).is_ok_and(|value| value == KEEP_RAW_ENABLED_VALUE);
    let attempt = match requested_scope {
        ScreenCaptureScope::ActiveWindow => capture_active_window_png(),
        ScreenCaptureScope::SelectedWindow => target_title
            .as_ref()
            .map(capture_window_title_contains_png)
            .unwrap_or_else(capture_active_window_png),
        ScreenCaptureScope::PrimaryDisplay => capture_primary_display_png(),
    };
    let status = attempt.status();
    let requested_scope_label = proof_scope_label(requested_scope);

    write_run_metadata(
        ScreenCaptureProofPath(&output_dir),
        &run_id,
        &status,
        target_title.as_ref(),
        requested_scope_label,
        keep_raw_until_analysis,
    )?;
    write_trigger_input(ScreenCaptureProofPath(&output_dir), requested_scope_label)?;

    match attempt {
        ScreenCaptureAttempt::Captured(image) => {
            write_captured_artifacts(
                ScreenCaptureProofPath(&output_dir),
                &run_id,
                &image,
                requested_scope_label,
                keep_raw_until_analysis,
            )?;
        }
        ScreenCaptureAttempt::Degraded(metadata) => {
            write_degraded_artifacts(ScreenCaptureProofPath(&output_dir), &metadata.status)?;
        }
    }

    Ok(())
}

fn requested_scope(value: &ScreenCaptureProofText) -> ScreenCaptureScope {
    match value.0.as_str() {
        SCOPE_SELECTED_WINDOW_INPUT => ScreenCaptureScope::SelectedWindow,
        SCOPE_PRIMARY_DISPLAY_INPUT => ScreenCaptureScope::PrimaryDisplay,
        _ => ScreenCaptureScope::ActiveWindow,
    }
}
