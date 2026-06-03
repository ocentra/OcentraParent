mod screen_capture_real_proof_support;

use std::{env, fs::create_dir_all, path::PathBuf};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_screen_capture_adapter::{
    capture_active_window_png, capture_window_title_contains_png, ScreenCaptureAttempt,
};
use screen_capture_real_proof_support::{
    run_id, write_captured_artifacts, write_degraded_artifacts, write_run_metadata,
    write_trigger_input, DEFAULT_DIR,
};

fn main() {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_DIR));
    create_dir_all(&output_dir).expect(constants::error::JOURNAL_OPENS);

    let run_id = run_id();
    let target_title = env::var("OCENTRA_SCREEN_CAPTURE_WINDOW_TITLE_CONTAINS").ok();
    let keep_raw_until_analysis =
        env::var("OCENTRA_SCREEN_CAPTURE_KEEP_RAW_UNTIL_ANALYSIS").is_ok_and(|value| value == "1");
    let attempt = target_title
        .as_deref()
        .map(capture_window_title_contains_png)
        .unwrap_or_else(capture_active_window_png);

    write_run_metadata(
        &output_dir,
        &run_id,
        attempt.status(),
        target_title,
        keep_raw_until_analysis,
    );
    write_trigger_input(&output_dir);

    match attempt {
        ScreenCaptureAttempt::Captured(image) => {
            write_captured_artifacts(&output_dir, &run_id, image, keep_raw_until_analysis);
        }
        ScreenCaptureAttempt::Degraded(metadata) => {
            write_degraded_artifacts(&output_dir, metadata.status);
        }
    }
}
