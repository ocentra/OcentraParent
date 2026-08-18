//! Service handoff for managed-browser screenshots.
//!
//! The service owns queue admission only. Browser trigger/target authority
//! stays with the browser owner, while encrypted image custody stays with the
//! existing agent-core queue. No AI result, OCR, URL, title, debugger endpoint,
//! or raw image is emitted by this handoff.

use std::path::Path;

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

pub(crate) struct ManagedBrowserCdpQueueRequest<'a> {
    pub(crate) queue_dir: &'a Path,
    pub(crate) key: &'a JournalKey,
    pub(crate) queue_job_id: String,
    pub(crate) created_at: String,
    pub(crate) expires_at: String,
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
    if request.queue_job_id.trim().is_empty()
        || request.created_at.trim().is_empty()
        || request.expires_at.trim().is_empty()
        || request.device_ref.trim().is_empty()
        || request.local_user_ref.trim().is_empty()
        || request.parent_setting_ref.trim().is_empty()
        || capture_receipt.raw_image_retained
        || capture_receipt.schema_version != MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION
        || capture_receipt.capture_ref.trim().is_empty()
        || capture_receipt.target_ref.trim().is_empty()
        || capture_receipt.target_ref != capture_receipt.evidence_refs.target_ref
        || capture_receipt.evidence_refs.url_ref.trim().is_empty()
        || capture_receipt.evidence_refs.title_ref.trim().is_empty()
        || capture_receipt.image_digest.trim().is_empty()
        || capture_receipt.width == 0
        || capture_receipt.height == 0
        || capture_receipt.width > MANAGED_BROWSER_CDP_MAX_DIMENSION
        || capture_receipt.height > MANAGED_BROWSER_CDP_MAX_DIMENSION
        || u64::from(capture_receipt.width) * u64::from(capture_receipt.height)
            > MANAGED_BROWSER_CDP_MAX_PIXELS
        || capture_receipt.image_format != MANAGED_BROWSER_CDP_IMAGE_FORMAT
        || capture_receipt.custody_state != SCREEN_CUSTODY_TEMP_QUEUE
        || capture_receipt.image_byte_size != capture_bytes.len() as u64
    {
        return Err(ManagedBrowserCdpQueueError::InvalidRequest);
    }

    let receipt = capture_receipt;
    let job = ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: request.queue_job_id.clone(),
        created_at: request.created_at.clone(),
        not_before: request.created_at.clone(),
        expires_at: request.expires_at,
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

    ScreenEvidenceQueue::open(request.queue_dir, request.key)?
        .append_encrypted_image(&job, capture_bytes)?;
    Ok(ManagedBrowserCdpQueueJobId(request.queue_job_id))
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
