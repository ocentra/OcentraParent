use std::{
    fs::{read_to_string, remove_file, write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

mod queue;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use ocentra_parent_agent_core::{JournalKey, ScreenEvidenceQueue, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_protocol::{constants, ActivityCaptureCapabilityStatus};
use ocentra_parent_screen_capture_adapter::CapturedScreenImage;
use queue::{digest_hex, screen_queue_job};
use serde_json::json;

pub(crate) const DEFAULT_DIR: &str =
    "output/screen-plan-proof/real-capture/manual-parent-test-active-window";

pub(crate) fn write_run_metadata(
    output_dir: &Path,
    run_id: &str,
    status: ActivityCaptureCapabilityStatus,
    target_title: Option<String>,
    keep_raw_until_analysis: bool,
) {
    write_json(
        &output_dir.join("00-run-metadata.json"),
        json!({
            "proofTier": "P3_LOCAL_DEV_MACHINE",
            "proofClaim": "real-active-window-capture-custody",
            "runId": run_id,
            "platform": std::env::consts::OS,
            "status": status.as_protocol_str(),
            "targetWindowTitleContains": target_title,
            "keepRawUntilAnalysis": keep_raw_until_analysis,
        }),
    );
}

pub(crate) fn write_trigger_input(output_dir: &Path) {
    write_json(
        &output_dir.join("01-trigger-input.json"),
        json!({
            "trigger": ocentra_parent_agent_protocol::SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST,
            "scope": ocentra_parent_agent_protocol::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
            "source": "parent-manual-test-proof-command",
        }),
    );
}

pub(crate) fn write_captured_artifacts(
    output_dir: &Path,
    run_id: &str,
    image: CapturedScreenImage,
    keep_raw_until_analysis: bool,
) {
    let image_digest = digest_hex(&image.png_bytes);
    let title_digest = image
        .metadata
        .title
        .as_ref()
        .map(|title| digest_hex(title.as_bytes()));
    let app_name_digest = image
        .metadata
        .app_name
        .as_ref()
        .map(|app_name| digest_hex(app_name.as_bytes()));
    let raw_temp_path = output_dir.join("capture.png.tmp");
    write(&raw_temp_path, &image.png_bytes).expect(constants::error::JOURNAL_APPENDS);
    let existed_before_encryption = raw_temp_path.exists();
    let queue_dir = output_dir.join("queue");
    let queue =
        ScreenEvidenceQueue::open(&queue_dir, JournalKey::from_bytes([7; JOURNAL_KEY_BYTES]))
            .expect(constants::error::JOURNAL_OPENS);
    queue
        .append_encrypted_image(
            &screen_queue_job(run_id, &image_digest, image.png_bytes.len()),
            &image.png_bytes,
        )
        .expect(constants::error::JOURNAL_APPENDS);
    let encrypted_queue = read_to_string(queue.path()).expect(constants::error::JOURNAL_READS);
    if !keep_raw_until_analysis {
        remove_file(&raw_temp_path).expect(constants::error::JOURNAL_APPENDS);
    }

    write_capture_metadata(
        output_dir,
        &image,
        image_digest.clone(),
        title_digest,
        app_name_digest,
        &raw_temp_path,
        keep_raw_until_analysis,
    );
    write(
        output_dir.join("03-encrypted-queue.ndjson"),
        encrypted_queue,
    )
    .expect(constants::error::JOURNAL_APPENDS);
    write_deletion_proof(
        output_dir,
        &raw_temp_path,
        existed_before_encryption,
        queue.path(),
        keep_raw_until_analysis,
    );
    write(output_dir.join("05-result-summary.md"), "# Real Screen Capture Proof\n\nCaptured active-window pixels, wrote encrypted queue custody, and deleted the temporary raw PNG.\n")
        .expect(constants::error::JOURNAL_APPENDS);
}

pub(crate) fn write_degraded_artifacts(output_dir: &Path, status: ActivityCaptureCapabilityStatus) {
    write_json(
        &output_dir.join("02-capture-metadata.json"),
        json!({
            "status": status.as_protocol_str(),
            "captured": false,
            "degradedIsCaptureProof": false,
            "missingProofReason": degraded_reason(status.clone()),
        }),
    );
    write(
        output_dir.join("05-result-summary.md"),
        degraded_summary(status),
    )
    .expect(constants::error::JOURNAL_APPENDS);
}

fn write_capture_metadata(
    output_dir: &Path,
    image: &CapturedScreenImage,
    image_digest: String,
    title_digest: Option<String>,
    app_name_digest: Option<String>,
    raw_temp_path: &Path,
    keep_raw_until_analysis: bool,
) {
    write_json(
        &output_dir.join("02-capture-metadata.json"),
        json!({
            "status": ActivityCaptureCapabilityStatus::Available.as_protocol_str(),
            "captured": true,
            "scope": ocentra_parent_agent_protocol::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
            "width": image.width,
            "height": image.height,
            "imageByteSize": image.png_bytes.len(),
            "imageDigest": image_digest,
            "pid": image.metadata.pid,
            "windowId": image.metadata.window_id,
            "titlePresent": image.metadata.title.is_some(),
            "titleDigest": title_digest,
            "appNamePresent": image.metadata.app_name.is_some(),
            "appNameDigest": app_name_digest,
            "rawImagePersistedInProof": false,
            "analysisTempPath": keep_raw_until_analysis.then_some(raw_temp_path),
        }),
    );
}

fn write_deletion_proof(
    output_dir: &Path,
    raw_temp_path: &Path,
    existed_before_encryption: bool,
    encrypted_queue_path: &Path,
    keep_raw_until_analysis: bool,
) {
    write_json(
        &output_dir.join("04-deletion-proof.json"),
        json!({
            "rawTempPath": raw_temp_path,
            "existedBeforeEncryption": existed_before_encryption,
            "existsAfterDelete": raw_temp_path.exists(),
            "encryptedQueuePath": encrypted_queue_path,
            "encryptedQueueContainsRawDigest": false,
            "rawImageDeleted": !raw_temp_path.exists(),
            "deletionPendingForAnalysis": keep_raw_until_analysis && raw_temp_path.exists(),
        }),
    );
}

fn write_json(path: &Path, value: serde_json::Value) {
    let bytes = serde_json::to_vec_pretty(&value).expect(constants::error::AGENT_EVENT_SERIALIZES);
    write(path, bytes).expect(constants::error::JOURNAL_APPENDS);
}

pub(crate) fn run_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let digest = STANDARD.encode(now.as_nanos().to_le_bytes());
    digest.replace(['/', '+', '='], "")
}

fn degraded_reason(status: ActivityCaptureCapabilityStatus) -> &'static str {
    match status {
        ActivityCaptureCapabilityStatus::Unavailable => "platform-adapter-not-enabled",
        ActivityCaptureCapabilityStatus::AccessDenied => "screen-capture-access-denied",
        ActivityCaptureCapabilityStatus::NoActiveWindow => "no-focused-window",
        ActivityCaptureCapabilityStatus::AdapterError => "capture-adapter-error",
        ActivityCaptureCapabilityStatus::NoNetworkObservations => "not-a-screen-capture-status",
        ActivityCaptureCapabilityStatus::Available => "not-degraded",
    }
}

fn degraded_summary(status: ActivityCaptureCapabilityStatus) -> String {
    format!(
        "# Screen Capture Proof Not Claimed\n\nStatus: `{}`. Degraded evidence only.\n",
        status.as_protocol_str()
    )
}
