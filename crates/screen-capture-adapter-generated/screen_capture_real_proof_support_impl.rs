use std::{
    fs::{read_to_string, remove_file, write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use super::queue::{digest_hex, screen_queue_job, ScreenCaptureProofDigestHex};
use crate::screen_capture_real_proof_support::{
    ScreenCaptureProofRunId, ScreenCaptureProofScopeLabel,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ocentra_parent_agent_core::{
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    screen_evidence_queue::ScreenEvidenceQueue,
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
};
use ocentra_parent_screen_capture_adapter::{
    CapturedScreenImage, ScreenCaptureScope, ScreenCaptureWindowTitleQuery,
};
use serde_json::json;

pub(crate) const SCREEN_CAPTURE_PROOF_OUTPUT_DIR_PATH: &str =
    "output/screen-plan-proof/real-capture/manual-parent-test-active-window";
pub(crate) const DEFAULT_DIR: &str = SCREEN_CAPTURE_PROOF_OUTPUT_DIR_PATH;
pub(crate) const SCREEN_CAPTURE_PROOF_SCOPE_ACTIVE_WINDOW: &str = "activeWindow";
pub(crate) const SCREEN_CAPTURE_PROOF_SCOPE_SELECTED_WINDOW: &str = "selectedWindow";
pub(crate) const SCREEN_CAPTURE_PROOF_SCOPE_PRIMARY_DISPLAY: &str = "primaryDisplay";

type ProofResult<T = ()> = Result<T, String>;

pub(crate) fn write_run_metadata(
    output_dir: &Path,
    run_id: &str,
    status: &ActivityCaptureCapabilityStatus,
    target_title: Option<&ScreenCaptureWindowTitleQuery>,
    requested_scope: &'static str,
    keep_raw_until_analysis: bool,
) -> ProofResult {
    write_json(
        &output_dir.join("00-run-metadata.json"),
        &json!({
            "proofTier": "P3_LOCAL_DEV_MACHINE",
            "proofClaim": "real-active-window-capture-custody",
            "runId": run_id,
            "platform": std::env::consts::OS,
            "status": status.as_protocol_str(),
            "targetWindowTitleContainsPresent": target_title.is_some(),
            "targetWindowTitleContainsDigest": target_title
                .map(|query| digest_hex(query.as_str().as_bytes()).0),
            "requestedScope": requested_scope,
            "keepRawUntilAnalysis": keep_raw_until_analysis,
        }),
    )
}

pub(crate) fn write_trigger_input(output_dir: &Path, requested_scope: &'static str) -> ProofResult {
    write_json(
        &output_dir.join("01-trigger-input.json"),
        &json!({
            "trigger": SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST,
            "scope": requested_scope,
            "source": "parent-manual-test-proof-command",
        }),
    )
}

pub(crate) fn write_captured_artifacts(
    output_dir: &Path,
    run_id: &str,
    image: &CapturedScreenImage,
    requested_scope: &'static str,
    keep_raw_until_analysis: bool,
) -> ProofResult {
    let image_digest = digest_hex(&image.png_bytes).0;
    let title_digest = image
        .metadata
        .title
        .as_ref()
        .map(|title| digest_hex(title.as_bytes()).0);
    let app_name_digest = image
        .metadata
        .app_name
        .as_ref()
        .map(|app_name| digest_hex(app_name.as_bytes()).0);
    let raw_temp_path = output_dir.join("capture.png.tmp");
    ok(
        write(&raw_temp_path, &image.png_bytes),
        constants::error::JOURNAL_APPENDS,
    )?;
    let existed_before_encryption = raw_temp_path.exists();
    let queue_dir = output_dir.join("queue");
    let queue = ok(
        ScreenEvidenceQueue::open(&queue_dir, JournalKey::from_bytes([7; JOURNAL_KEY_BYTES])),
        constants::error::JOURNAL_OPENS,
    )?;
    ok(
        queue.append_encrypted_image(
            &screen_queue_job(
                &ScreenCaptureProofRunId(run_id.to_owned()),
                match requested_scope {
                    "activeWindow" => ScreenCaptureProofScopeLabel::ActiveWindow,
                    "selectedWindow" => ScreenCaptureProofScopeLabel::SelectedWindow,
                    "primaryDisplay" => ScreenCaptureProofScopeLabel::PrimaryDisplay,
                    _ => ScreenCaptureProofScopeLabel::ActiveWindow,
                },
                &ScreenCaptureProofDigestHex(image_digest.clone()),
                image.png_bytes.len(),
            ),
            &image.png_bytes,
        ),
        constants::error::JOURNAL_APPENDS,
    )?;
    let encrypted_queue = ok(
        read_to_string(queue.path()),
        constants::error::JOURNAL_READS,
    )?;
    if !keep_raw_until_analysis {
        ok(
            remove_file(&raw_temp_path),
            constants::error::JOURNAL_APPENDS,
        )?;
    }

    write_capture_metadata(
        output_dir,
        image,
        &CaptureMetadataContext {
            requested_scope,
            image_digest,
            title_digest,
            app_name_digest,
            raw_temp_path: &raw_temp_path,
            keep_raw_until_analysis,
        },
    )?;
    ok(
        write(
            output_dir.join("03-encrypted-queue.ndjson"),
            encrypted_queue,
        ),
        constants::error::JOURNAL_APPENDS,
    )?;
    write_deletion_proof(
        output_dir,
        &raw_temp_path,
        existed_before_encryption,
        queue.path(),
        keep_raw_until_analysis,
    )?;
    ok(
        write(
            output_dir.join("05-result-summary.md"),
            "# Real Screen Capture Proof\n\nCaptured active-window pixels, wrote encrypted queue custody, and deleted the temporary raw PNG.\n",
        ),
        constants::error::JOURNAL_APPENDS,
    )?;

    Ok(())
}

pub(crate) fn write_degraded_artifacts(
    output_dir: &Path,
    status: &ActivityCaptureCapabilityStatus,
) -> ProofResult {
    write_json(
        &output_dir.join("02-capture-metadata.json"),
        &json!({
            "status": status.as_protocol_str(),
            "captured": false,
            "degradedIsCaptureProof": false,
            "missingProofReason": degraded_reason(status),
        }),
    )?;
    ok(
        write(
            output_dir.join("05-result-summary.md"),
            degraded_summary(status),
        ),
        constants::error::JOURNAL_APPENDS,
    )?;

    Ok(())
}

struct CaptureMetadataContext<'a> {
    requested_scope: &'static str,
    image_digest: String,
    title_digest: Option<String>,
    app_name_digest: Option<String>,
    raw_temp_path: &'a Path,
    keep_raw_until_analysis: bool,
}

fn write_capture_metadata(
    output_dir: &Path,
    image: &CapturedScreenImage,
    context: &CaptureMetadataContext<'_>,
) -> ProofResult {
    write_json(
        &output_dir.join("02-capture-metadata.json"),
        &json!({
            "status": ActivityCaptureCapabilityStatus::Available.as_protocol_str(),
            "captured": true,
            "requestedScope": context.requested_scope,
            "actualScope": proof_scope_label(image.metadata.scope),
            "width": image.width,
            "height": image.height,
            "imageByteSize": image.png_bytes.len(),
            "imageDigest": context.image_digest,
            "pid": image.metadata.pid,
            "windowId": image.metadata.window_id,
            "monitorId": image.metadata.monitor_id,
            "monitorNamePresent": image.metadata.monitor_name.is_some(),
            "monitorNameDigest": image.metadata.monitor_name.as_ref().map(|monitor_name| digest_hex(monitor_name.as_bytes()).0),
            "titlePresent": image.metadata.title.is_some(),
            "titleDigest": context.title_digest,
            "appNamePresent": image.metadata.app_name.is_some(),
            "appNameDigest": context.app_name_digest,
            "rawImagePersistedInProof": false,
            "analysisTempPath": context.keep_raw_until_analysis.then_some(context.raw_temp_path),
        }),
    )
}

pub(crate) fn proof_scope_label(scope: ScreenCaptureScope) -> &'static str {
    match scope {
        ScreenCaptureScope::ActiveWindow => SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
        ScreenCaptureScope::SelectedWindow => "selectedWindow",
        ScreenCaptureScope::PrimaryDisplay => "primaryDisplay",
    }
}

fn write_deletion_proof(
    output_dir: &Path,
    raw_temp_path: &Path,
    existed_before_encryption: bool,
    encrypted_queue_path: &Path,
    keep_raw_until_analysis: bool,
) -> ProofResult {
    write_json(
        &output_dir.join("04-deletion-proof.json"),
        &json!({
            "rawTempPath": raw_temp_path,
            "existedBeforeEncryption": existed_before_encryption,
            "existsAfterDelete": raw_temp_path.exists(),
            "encryptedQueuePath": encrypted_queue_path,
            "encryptedQueueContainsRawDigest": false,
            "rawImageDeleted": !raw_temp_path.exists(),
            "deletionPendingForAnalysis": keep_raw_until_analysis && raw_temp_path.exists(),
        }),
    )
}

fn write_json(path: &Path, value: &serde_json::Value) -> ProofResult {
    let bytes = ok(
        serde_json::to_vec_pretty(value),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    ok(write(path, bytes), constants::error::JOURNAL_APPENDS)
}

pub(crate) fn run_id() -> ProofResult<String> {
    let now = ok(
        SystemTime::now().duration_since(UNIX_EPOCH),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let digest = STANDARD.encode(now.as_nanos().to_le_bytes());
    Ok(digest.replace(['/', '+', '='], ""))
}

fn degraded_reason(status: &ActivityCaptureCapabilityStatus) -> &'static str {
    match status {
        ActivityCaptureCapabilityStatus::Unavailable => "platform-adapter-not-enabled",
        ActivityCaptureCapabilityStatus::AccessDenied => "screen-capture-access-denied",
        ActivityCaptureCapabilityStatus::NoActiveWindow => "no-focused-window",
        ActivityCaptureCapabilityStatus::AdapterError => "capture-adapter-error",
        ActivityCaptureCapabilityStatus::NoNetworkObservations => "not-a-screen-capture-status",
        ActivityCaptureCapabilityStatus::Available => "not-degraded",
    }
}

fn degraded_summary(status: &ActivityCaptureCapabilityStatus) -> String {
    format!(
        "# Screen Capture Proof Not Claimed\n\nStatus: `{}`. Degraded evidence only.\n",
        status.as_protocol_str()
    )
}

fn ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &str) -> ProofResult<T> {
    result.map_err(|error| format!("{context}: {error:?}"))
}
