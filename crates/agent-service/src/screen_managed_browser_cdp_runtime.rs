//! Service handoff for managed-browser screenshots.
//!
//! The service owns queue admission only. Browser trigger/target authority
//! stays with the browser owner, while encrypted image custody stays with the
//! existing agent-core queue. No AI result, OCR, URL, title, debugger endpoint,
//! or raw image is emitted by this handoff.

use std::path::Path;

use chrono::{DateTime, Duration, SecondsFormat, Utc};
use ocentra_parent_agent_core::{
    journal_crypto::JournalKey, journal_error::JournalError,
    screen_evidence_queue::ScreenEvidenceQueue,
};
use ocentra_parent_agent_protocol::{
    activity::{ActivityEvidenceKind, ActivityEvidenceRef},
    screen_evidence::{
        ScreenAnalysisQueueJob, SCREEN_CUSTODY_TEMP_QUEUE, SCREEN_DELETION_REQUIRED,
        SCREEN_QUEUE_STATUS_QUEUED,
    },
    SCREEN_EVIDENCE_SCHEMA_VERSION,
};
use ocentra_parent_screen_capture_adapter::managed_browser_cdp::ManagedBrowserCdpScreenCapture;
use ocentra_schema::managed_browser_cdp_capture::{
    MANAGED_BROWSER_CDP_ADAPTER_ID, MANAGED_BROWSER_CDP_CAPTURE_REASON,
    MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION, MANAGED_BROWSER_CDP_ENCRYPTED_IMAGE_REF_PREFIX,
    MANAGED_BROWSER_CDP_IMAGE_FORMAT, MANAGED_BROWSER_CDP_MAX_DIMENSION,
    MANAGED_BROWSER_CDP_MAX_PIXELS, MANAGED_BROWSER_CDP_SOURCE_ID,
};
use sha2::{Digest, Sha256};

pub(crate) struct ManagedBrowserCdpQueueRequest<'a> {
    pub(crate) queue_dir: &'a Path,
    pub(crate) key: &'a JournalKey,
    pub(crate) queue_job_id: String,
    pub(crate) device_ref: String,
    pub(crate) local_user_ref: String,
    pub(crate) parent_setting_ref: String,
    pub(crate) setting_version: u64,
    pub(crate) max_retry_count: u64,
    pub(crate) capture: &'a ManagedBrowserCdpScreenCapture,
}

pub(crate) struct ManagedBrowserCdpQueueJobId(String);

#[derive(Debug)]
pub enum ManagedBrowserCdpQueueError {
    InvalidRequest,
    Queue(JournalError),
}

impl From<JournalError> for ManagedBrowserCdpQueueError {
    fn from(error: JournalError) -> Self {
        Self::Queue(error)
    }
}

pub(crate) fn enqueue_managed_browser_cdp_capture(
    request: ManagedBrowserCdpQueueRequest<'_>,
) -> Result<ManagedBrowserCdpQueueJobId, ManagedBrowserCdpQueueError> {
    let capture_receipt = request.capture.receipt();
    let capture_bytes = request.capture.png_bytes();
    let created_at = Utc::now();
    let expires_at = created_at
        .checked_add_signed(Duration::seconds(
            ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_MAX_QUEUE_TTL_SECONDS,
        ))
        .ok_or(ManagedBrowserCdpQueueError::InvalidRequest)?;
    if !queue_request_is_valid(
        &request,
        capture_receipt,
        capture_bytes,
        &created_at,
        &expires_at,
    ) {
        return Err(ManagedBrowserCdpQueueError::InvalidRequest);
    }
    let created_at_text = created_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at_text = expires_at.to_rfc3339_opts(SecondsFormat::Millis, true);

    let receipt = capture_receipt;
    let job = ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: request.queue_job_id.clone(),
        created_at: created_at_text.clone(),
        not_before: created_at_text,
        expires_at: expires_at_text,
        last_attempt_at: None,
        capture_reason: MANAGED_BROWSER_CDP_CAPTURE_REASON.to_owned(),
        capture_scope: receipt.capture_mode.as_protocol_str().to_owned(),
        source_id: MANAGED_BROWSER_CDP_SOURCE_ID.to_owned(),
        adapter_id: MANAGED_BROWSER_CDP_ADAPTER_ID.to_owned(),
        device_ref: request.device_ref,
        local_user_ref: request.local_user_ref,
        parent_setting_ref: request.parent_setting_ref,
        setting_version: request.setting_version,
        related_evidence_refs: evidence_refs(receipt),
        encrypted_image_ref: {
            let mut value = String::from(MANAGED_BROWSER_CDP_ENCRYPTED_IMAGE_REF_PREFIX);
            value.push_str(&request.queue_job_id);
            value
        },
        image_digest: receipt.image_digest.clone(),
        image_byte_size: receipt.image_byte_size,
        image_format: MANAGED_BROWSER_CDP_IMAGE_FORMAT.to_owned(),
        status: SCREEN_QUEUE_STATUS_QUEUED.to_owned(),
        attempt_count: 0,
        max_retry_count: request.max_retry_count,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: SCREEN_DELETION_REQUIRED.to_owned(),
        deletion_proof_ref: None,
        custody_state: SCREEN_CUSTODY_TEMP_QUEUE.to_owned(),
    };
    if !queue_job_custody_is_valid(&job) {
        return Err(ManagedBrowserCdpQueueError::InvalidRequest);
    }

    ScreenEvidenceQueue::open(request.queue_dir, request.key)?
        .append_encrypted_image(&job, capture_bytes)?;
    Ok(ManagedBrowserCdpQueueJobId(request.queue_job_id))
}

fn queue_request_is_valid(
    request: &ManagedBrowserCdpQueueRequest<'_>,
    receipt: &ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpCaptureReceipt,
    image_bytes: &[u8],
    created_at: &DateTime<Utc>,
    expires_at: &DateTime<Utc>,
) -> bool {
    let queue_window_is_valid = expires_at > created_at
        && expires_at.signed_duration_since(created_at).num_seconds()
            <= ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_MAX_QUEUE_TTL_SECONDS;
    let mut digest = String::new();
    for byte in Sha256::digest(image_bytes) {
        digest.push_str(&format!("{byte:02x}"));
    }
    !request.queue_job_id.trim().is_empty()
        && !request.device_ref.trim().is_empty()
        && !request.local_user_ref.trim().is_empty()
        && !request.parent_setting_ref.trim().is_empty()
        && queue_window_is_valid
        && !receipt.raw_image_retained
        && receipt.schema_version == MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION
        && !receipt.capture_ref.trim().is_empty()
        && !receipt.target_ref.trim().is_empty()
        && receipt.target_ref == receipt.evidence_refs.target_ref
        && !receipt.evidence_refs.url_ref.trim().is_empty()
        && !receipt.evidence_refs.title_ref.trim().is_empty()
        && receipt.image_digest == digest
        && receipt.width > 0
        && receipt.height > 0
        && receipt.width <= MANAGED_BROWSER_CDP_MAX_DIMENSION
        && receipt.height <= MANAGED_BROWSER_CDP_MAX_DIMENSION
        && u64::from(receipt.width) * u64::from(receipt.height) <= MANAGED_BROWSER_CDP_MAX_PIXELS
        && receipt.image_format == MANAGED_BROWSER_CDP_IMAGE_FORMAT
        && receipt.custody_state == SCREEN_CUSTODY_TEMP_QUEUE
        && receipt.image_byte_size == image_bytes.len() as u64
}

fn queue_job_custody_is_valid(job: &ScreenAnalysisQueueJob) -> bool {
    job.status == SCREEN_QUEUE_STATUS_QUEUED
        && job.attempt_count == 0
        && job.last_attempt_at.is_none()
        && job.failure_reason.is_none()
        && job.unavailable_reason.is_none()
        && job.deletion_required
        && job.deleted_at.is_none()
        && job.deletion_status == SCREEN_DELETION_REQUIRED
        && job.deletion_proof_ref.is_none()
        && job.custody_state == SCREEN_CUSTODY_TEMP_QUEUE
        && job.created_at == job.not_before
        && !job.expires_at.trim().is_empty()
}

fn evidence_refs(
    receipt: &ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpCaptureReceipt,
) -> Vec<ActivityEvidenceRef> {
    [
        ActivityEvidenceRef {
            evidence_id: receipt.evidence_refs.target_ref.clone(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: receipt.evidence_refs.url_ref.clone(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: receipt.evidence_refs.title_ref.clone(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: receipt.capture_ref.clone(),
            kind: ActivityEvidenceKind::Screenshot,
            digest: Some(receipt.image_digest.clone()),
            uri: None,
        },
    ]
    .into_iter()
    .collect()
}
