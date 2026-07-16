use std::{
    fs::{read_to_string, remove_file, write},
    time::{SystemTime, UNIX_EPOCH},
};

use super::queue::{digest_hex, screen_queue_job, ScreenCaptureProofDigestHex};
use crate::screen_capture_real_proof_support::{
    degraded_reason, proof_scope_label, scope_wire_label, ProofResult, ScreenCaptureProofError,
    ScreenCaptureProofPath, ScreenCaptureProofRunId, ScreenCaptureProofScopeLabel,
    ScreenCaptureProofText,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ocentra_parent_agent_core::{
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    screen_evidence_queue::ScreenEvidenceQueue,
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST;
use ocentra_parent_screen_capture_adapter::{CapturedScreenImage, ScreenCaptureWindowTitleQuery};
use serde_json::json;

pub const OUT: &str = "output/screen-plan-proof/real-capture/manual-parent-test-active-window";
pub const DEFAULT_DIR: &str = OUT;
pub const SCREEN_CAPTURE_PROOF_SCOPE_ACTIVE_WINDOW: &str = "activeWindow";
pub const SCREEN_CAPTURE_PROOF_SCOPE_SELECTED_WINDOW: &str = "selectedWindow";
pub const SCREEN_CAPTURE_PROOF_SCOPE_PRIMARY_DISPLAY: &str = "primaryDisplay";
pub const RUN_METADATA_FILE_NAME: &str = "00-run-metadata.json";
pub const TRIGGER_INPUT_FILE_NAME: &str = "01-trigger-input.json";
pub const CAPTURE_METADATA_FILE_NAME: &str = "02-capture-metadata.json";
pub const ENCRYPTED_QUEUE_FILE_NAME: &str = "03-encrypted-queue.ndjson";
pub const DELETION_PROOF_FILE_NAME: &str = "04-deletion-proof.json";
pub const RESULT_SUMMARY_FILE_NAME: &str = "05-result-summary.md";
pub const RAW_CAPTURE_FILE_NAME: &str = "capture.png.tmp";
pub const QUEUE_DIRECTORY_NAME: &str = "queue";
pub const PROOF_CLAIM_REAL_ACTIVE_WINDOW: &str = "real-active-window-capture-custody";
pub const PROOF_SOURCE_PARENT_MANUAL_TEST: &str = "parent-manual-test-proof-command";
pub const RESULT_SUMMARY_CAPTURED: &str = "# Real Screen Capture Proof\n\nCaptured active-window pixels, wrote encrypted queue custody, and deleted the temporary raw PNG.\n";
pub const DEGRADED_REASON_PLATFORM_ADAPTER: &str = "platform-adapter-not-enabled";
pub const DEGRADED_REASON_ACCESS_DENIED: &str = "screen-capture-access-denied";
pub const DEGRADED_REASON_NO_ACTIVE_WINDOW: &str = "no-focused-window";
pub const DEGRADED_REASON_ADAPTER_ERROR: &str = "capture-adapter-error";
pub const DEGRADED_REASON_NOT_SCREEN_CAPTURE: &str = "not-a-screen-capture-status";
pub const DEGRADED_REASON_NOT_DEGRADED: &str = "not-degraded";
pub const EMPTY_TEXT: &str = "";
pub const FIELD_PROOF_TIER: &str = "proofTier";
pub const FIELD_PROOF_CLAIM: &str = "proofClaim";
pub const FIELD_RUN_ID: &str = "runId";
pub const FIELD_PLATFORM: &str = "platform";
pub const FIELD_STATUS: &str = "status";
pub const FIELD_TARGET_TITLE_PRESENT: &str = "targetWindowTitleContainsPresent";
pub const FIELD_TARGET_TITLE_DIGEST: &str = "targetWindowTitleContainsDigest";
pub const FIELD_REQUESTED_SCOPE: &str = "requestedScope";
pub const FIELD_KEEP_RAW_UNTIL_ANALYSIS: &str = "keepRawUntilAnalysis";
pub const FIELD_TRIGGER: &str = "trigger";
pub const FIELD_SCOPE: &str = "scope";
pub const FIELD_SOURCE: &str = "source";
pub const FIELD_CAPTURED: &str = "captured";
pub const FIELD_DEGRADED_IS_CAPTURE_PROOF: &str = "degradedIsCaptureProof";
pub const FIELD_MISSING_PROOF_REASON: &str = "missingProofReason";
pub const FIELD_ACTUAL_SCOPE: &str = "actualScope";
pub const FIELD_WIDTH: &str = "width";
pub const FIELD_HEIGHT: &str = "height";
pub const FIELD_IMAGE_BYTE_SIZE: &str = "imageByteSize";
pub const FIELD_IMAGE_DIGEST: &str = "imageDigest";
pub const FIELD_PID: &str = "pid";
pub const FIELD_WINDOW_ID: &str = "windowId";
pub const FIELD_MONITOR_ID: &str = "monitorId";
pub const FIELD_MONITOR_NAME_PRESENT: &str = "monitorNamePresent";
pub const FIELD_MONITOR_NAME_DIGEST: &str = "monitorNameDigest";
pub const FIELD_TITLE_PRESENT: &str = "titlePresent";
pub const FIELD_TITLE_DIGEST: &str = "titleDigest";
pub const FIELD_APP_NAME_PRESENT: &str = "appNamePresent";
pub const FIELD_APP_NAME_DIGEST: &str = "appNameDigest";
pub const FIELD_RAW_IMAGE_PERSISTED: &str = "rawImagePersistedInProof";
pub const FIELD_ANALYSIS_TEMP_PATH: &str = "analysisTempPath";
pub const FIELD_RAW_TEMP_PATH: &str = "rawTempPath";
pub const FIELD_EXISTED_BEFORE_ENCRYPTION: &str = "existedBeforeEncryption";
pub const FIELD_EXISTS_AFTER_DELETE: &str = "existsAfterDelete";
pub const FIELD_ENCRYPTED_QUEUE_PATH: &str = "encryptedQueuePath";
pub const FIELD_ENCRYPTED_QUEUE_CONTAINS_RAW_DIGEST: &str = "encryptedQueueContainsRawDigest";
pub const FIELD_RAW_IMAGE_DELETED: &str = "rawImageDeleted";
pub const FIELD_DELETION_PENDING_FOR_ANALYSIS: &str = "deletionPendingForAnalysis";
pub const PROOF_TIER_LOCAL_DEV: &str = "P3_LOCAL_DEV_MACHINE";
pub const DEGRADED_SUMMARY_PREFIX: &str = "# Screen Capture Proof Not Claimed\n\nStatus: `";
pub const DEGRADED_SUMMARY_SUFFIX: &str = "`. Degraded evidence only.\n";

pub(crate) fn write_run_metadata(
    output_dir: ScreenCaptureProofPath<'_>,
    run_id: &ScreenCaptureProofRunId,
    status: &ActivityCaptureCapabilityStatus,
    target_title: Option<&ScreenCaptureWindowTitleQuery>,
    requested_scope: ScreenCaptureProofScopeLabel,
    keep_raw_until_analysis: bool,
) -> ProofResult {
    let requested_scope_label = scope_wire_label(requested_scope);
    write_json(
        ScreenCaptureProofPath(&output_dir.0.join(RUN_METADATA_FILE_NAME)),
        &json!({
            (FIELD_PROOF_TIER): PROOF_TIER_LOCAL_DEV,
            (FIELD_PROOF_CLAIM): PROOF_CLAIM_REAL_ACTIVE_WINDOW,
            (FIELD_RUN_ID): run_id.0,
            (FIELD_PLATFORM): std::env::consts::OS,
            (FIELD_STATUS): status.as_protocol_str(),
            (FIELD_TARGET_TITLE_PRESENT): target_title.is_some(),
            (FIELD_TARGET_TITLE_DIGEST): target_title
                .map(|query| digest_hex(query.as_str().as_bytes()).0),
            (FIELD_REQUESTED_SCOPE): requested_scope_label.0,
            (FIELD_KEEP_RAW_UNTIL_ANALYSIS): keep_raw_until_analysis,
        }),
    )
}

pub(crate) fn write_trigger_input(
    output_dir: ScreenCaptureProofPath<'_>,
    requested_scope: ScreenCaptureProofScopeLabel,
) -> ProofResult {
    let requested_scope_label = scope_wire_label(requested_scope);
    write_json(
        ScreenCaptureProofPath(&output_dir.0.join(TRIGGER_INPUT_FILE_NAME)),
        &json!({
            (FIELD_TRIGGER): SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST,
            (FIELD_SCOPE): requested_scope_label.0,
            (FIELD_SOURCE): PROOF_SOURCE_PARENT_MANUAL_TEST,
        }),
    )
}

pub(crate) fn write_captured_artifacts(
    output_dir: ScreenCaptureProofPath<'_>,
    run_id: &ScreenCaptureProofRunId,
    image: &CapturedScreenImage,
    requested_scope: ScreenCaptureProofScopeLabel,
    keep_raw_until_analysis: bool,
) -> ProofResult {
    let image_digest = digest_hex(&image.png_bytes);
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
    let queue_artifacts = persist_capture_queue(
        output_dir,
        run_id,
        image,
        requested_scope,
        &image_digest,
        keep_raw_until_analysis,
    )?;

    write_capture_metadata(
        output_dir,
        image,
        &CaptureMetadataContext {
            requested_scope,
            image_digest: image_digest.0,
            title_digest,
            app_name_digest,
            raw_temp_path: ScreenCaptureProofPath(&queue_artifacts.raw_temp_path),
            keep_raw_until_analysis,
        },
    )?;
    ok(
        write(
            output_dir.0.join(ENCRYPTED_QUEUE_FILE_NAME),
            queue_artifacts.encrypted_queue,
        ),
        ScreenCaptureProofError(constants::error::JOURNAL_APPENDS.to_owned()),
    )?;
    write_deletion_proof(
        output_dir,
        ScreenCaptureProofPath(&queue_artifacts.raw_temp_path),
        queue_artifacts.existed_before_encryption,
        ScreenCaptureProofPath(&queue_artifacts.queue_path),
        keep_raw_until_analysis,
    )?;
    ok(
        write(
            output_dir.0.join(RESULT_SUMMARY_FILE_NAME),
            RESULT_SUMMARY_CAPTURED,
        ),
        ScreenCaptureProofError(constants::error::JOURNAL_APPENDS.to_owned()),
    )?;

    Ok(())
}

struct CaptureQueueArtifacts {
    raw_temp_path: std::path::PathBuf,
    queue_path: std::path::PathBuf,
    encrypted_queue: String,
    existed_before_encryption: bool,
}

fn persist_capture_queue(
    output_dir: ScreenCaptureProofPath<'_>,
    run_id: &ScreenCaptureProofRunId,
    image: &CapturedScreenImage,
    requested_scope: ScreenCaptureProofScopeLabel,
    image_digest: &ScreenCaptureProofDigestHex,
    keep_raw_until_analysis: bool,
) -> ProofResult<CaptureQueueArtifacts> {
    let raw_temp_path = output_dir.0.join(RAW_CAPTURE_FILE_NAME);
    ok(
        write(&raw_temp_path, &image.png_bytes),
        ScreenCaptureProofError(constants::error::JOURNAL_APPENDS.to_owned()),
    )?;
    let existed_before_encryption = raw_temp_path.exists();
    let queue_path = output_dir.0.join(QUEUE_DIRECTORY_NAME);
    let queue = ok(
        ScreenEvidenceQueue::open(&queue_path, JournalKey::from_bytes([7; JOURNAL_KEY_BYTES])),
        ScreenCaptureProofError(constants::error::JOURNAL_OPENS.to_owned()),
    )?;
    ok(
        queue.append_encrypted_image(
            &screen_queue_job(run_id, requested_scope, image_digest, image.png_bytes.len()),
            &image.png_bytes,
        ),
        ScreenCaptureProofError(constants::error::JOURNAL_APPENDS.to_owned()),
    )?;
    let encrypted_queue = ok(
        read_to_string(queue.path()),
        ScreenCaptureProofError(constants::error::JOURNAL_READS.to_owned()),
    )?;
    if !keep_raw_until_analysis {
        ok(
            remove_file(&raw_temp_path),
            ScreenCaptureProofError(constants::error::JOURNAL_APPENDS.to_owned()),
        )?;
    }
    Ok(CaptureQueueArtifacts {
        raw_temp_path,
        queue_path,
        encrypted_queue,
        existed_before_encryption,
    })
}

pub(crate) fn write_degraded_artifacts(
    output_dir: ScreenCaptureProofPath<'_>,
    status: &ActivityCaptureCapabilityStatus,
) -> ProofResult {
    write_json(
        ScreenCaptureProofPath(&output_dir.0.join(CAPTURE_METADATA_FILE_NAME)),
        &json!({
            (FIELD_STATUS): status.as_protocol_str(),
            (FIELD_CAPTURED): false,
            (FIELD_DEGRADED_IS_CAPTURE_PROOF): false,
            (FIELD_MISSING_PROOF_REASON): degraded_reason(status).0,
        }),
    )?;
    ok(
        write(
            output_dir.0.join(RESULT_SUMMARY_FILE_NAME),
            degraded_summary(status).0,
        ),
        ScreenCaptureProofError(constants::error::JOURNAL_APPENDS.to_owned()),
    )?;

    Ok(())
}

struct CaptureMetadataContext<'a> {
    requested_scope: ScreenCaptureProofScopeLabel,
    image_digest: String,
    title_digest: Option<String>,
    app_name_digest: Option<String>,
    raw_temp_path: ScreenCaptureProofPath<'a>,
    keep_raw_until_analysis: bool,
}

fn write_capture_metadata(
    output_dir: ScreenCaptureProofPath<'_>,
    image: &CapturedScreenImage,
    context: &CaptureMetadataContext<'_>,
) -> ProofResult {
    write_json(
        ScreenCaptureProofPath(&output_dir.0.join(CAPTURE_METADATA_FILE_NAME)),
        &json!({
            (FIELD_STATUS): ActivityCaptureCapabilityStatus::Available.as_protocol_str(),
            (FIELD_CAPTURED): true,
            (FIELD_REQUESTED_SCOPE): scope_wire_label(context.requested_scope).0,
            (FIELD_ACTUAL_SCOPE): scope_wire_label(proof_scope_label(image.metadata.scope)).0,
            (FIELD_WIDTH): image.width,
            (FIELD_HEIGHT): image.height,
            (FIELD_IMAGE_BYTE_SIZE): image.png_bytes.len(),
            (FIELD_IMAGE_DIGEST): context.image_digest,
            (FIELD_PID): image.metadata.pid,
            (FIELD_WINDOW_ID): image.metadata.window_id,
            (FIELD_MONITOR_ID): image.metadata.monitor_id,
            (FIELD_MONITOR_NAME_PRESENT): image.metadata.monitor_name.is_some(),
            (FIELD_MONITOR_NAME_DIGEST): image.metadata.monitor_name.as_ref().map(|monitor_name| digest_hex(monitor_name.as_bytes()).0),
            (FIELD_TITLE_PRESENT): image.metadata.title.is_some(),
            (FIELD_TITLE_DIGEST): context.title_digest,
            (FIELD_APP_NAME_PRESENT): image.metadata.app_name.is_some(),
            (FIELD_APP_NAME_DIGEST): context.app_name_digest,
            (FIELD_RAW_IMAGE_PERSISTED): false,
            (FIELD_ANALYSIS_TEMP_PATH): context.keep_raw_until_analysis.then_some(context.raw_temp_path.0),
        }),
    )
}

fn write_deletion_proof(
    output_dir: ScreenCaptureProofPath<'_>,
    raw_temp_path: ScreenCaptureProofPath<'_>,
    existed_before_encryption: bool,
    encrypted_queue_path: ScreenCaptureProofPath<'_>,
    keep_raw_until_analysis: bool,
) -> ProofResult {
    write_json(
        ScreenCaptureProofPath(&output_dir.0.join(DELETION_PROOF_FILE_NAME)),
        &json!({
            (FIELD_RAW_TEMP_PATH): raw_temp_path.0,
            (FIELD_EXISTED_BEFORE_ENCRYPTION): existed_before_encryption,
            (FIELD_EXISTS_AFTER_DELETE): raw_temp_path.0.exists(),
            (FIELD_ENCRYPTED_QUEUE_PATH): encrypted_queue_path.0,
            (FIELD_ENCRYPTED_QUEUE_CONTAINS_RAW_DIGEST): false,
            (FIELD_RAW_IMAGE_DELETED): !raw_temp_path.0.exists(),
            (FIELD_DELETION_PENDING_FOR_ANALYSIS): keep_raw_until_analysis && raw_temp_path.0.exists(),
        }),
    )
}

fn write_json(path: ScreenCaptureProofPath<'_>, value: &serde_json::Value) -> ProofResult {
    let bytes = ok(
        serde_json::to_vec_pretty(value),
        ScreenCaptureProofError(constants::error::AGENT_EVENT_SERIALIZES.to_owned()),
    )?;
    ok(
        write(path.0, bytes),
        ScreenCaptureProofError(constants::error::JOURNAL_APPENDS.to_owned()),
    )
}

pub(crate) fn run_id() -> ProofResult<ScreenCaptureProofRunId> {
    let now = ok(
        SystemTime::now().duration_since(UNIX_EPOCH),
        ScreenCaptureProofError(constants::error::AGENT_EVENT_SERIALIZES.to_owned()),
    )?;
    let digest = STANDARD.encode(now.as_nanos().to_le_bytes());
    Ok(ScreenCaptureProofRunId(
        digest.replace(['/', '+', '='], EMPTY_TEXT),
    ))
}

fn degraded_summary(status: &ActivityCaptureCapabilityStatus) -> ScreenCaptureProofText {
    let mut summary = String::from(DEGRADED_SUMMARY_PREFIX);
    summary.push_str(status.as_protocol_str());
    summary.push_str(DEGRADED_SUMMARY_SUFFIX);
    ScreenCaptureProofText(summary)
}

fn ok<T, E: std::fmt::Debug>(
    result: Result<T, E>,
    context: ScreenCaptureProofError,
) -> ProofResult<T> {
    let context = context.0;
    result.map_err(|error| ScreenCaptureProofError(format!("{context}: {error:?}")))
}
