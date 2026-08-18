//! Service handoff for managed-browser screenshots.
//!
//! The service owns queue admission only. Browser trigger/target authority
//! stays with the browser owner, while encrypted image custody stays with the
//! existing agent-core queue. No AI result, OCR, URL, title, debugger endpoint,
//! or raw image is emitted by this handoff.

use std::path::Path;

use chrono::{DateTime, Duration, SecondsFormat, Utc};
use ocentra_parent_agent_core::{
    browser_bridge_capture::{
        ManagedBrowserCdpCaptureError, ManagedBrowserCdpStructuredExtraction,
        ManagedBrowserCdpTargetAuthority,
    },
    journal_crypto::JournalKey,
    journal_error::JournalError,
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
use ocentra_screen_ai_core::screen_intelligence_router::extraction::owner::handoff::ScreenManagedBrowserStructuredExtractionHandoffError;
use ocentra_screen_ai_core::screen_intelligence_router::extraction::owner::{
    ManagedBrowserStructuredExtractionObservation, ManagedBrowserStructuredExtractionOwner,
};
use ocentra_screen_ai_core::screen_intelligence_router::{
    plan_screen_intelligence_route, ActivityEvidenceRef as ScreenActivityEvidenceRef,
    ScreenCaptureScope, ScreenEvidenceCustodyState, ScreenIntelligencePolicySensitivity,
    ScreenIntelligenceRouteDecision, ScreenIntelligenceRouteRequest, ScreenIntelligenceSourceKind,
    ScreenManagedBrowserStructuredExtraction,
};
use sha2::{Digest, Sha256};

const MANAGED_BROWSER_STRUCTURED_EVIDENCE_KIND: &str = "managed-browser-structured";

/// Inputs owned by the service composition boundary. They contain policy and
/// capture intent only; the browser target and structured evidence are always
/// obtained from the real managed-browser authority below.
pub struct ManagedBrowserScreenIntelligenceRequest {
    pub schema_version: u16,
    pub request_id: String,
    pub route_id: String,
    pub requested_at: String,
    pub device_ref: String,
    pub capture_reason: String,
    pub policy_question: String,
    pub policy_sensitivity: ScreenIntelligencePolicySensitivity,
    pub existing_evidence_refs: Vec<ScreenActivityEvidenceRef>,
    pub parent_allows_managed_browser_structured_extraction: bool,
    pub parent_allows_screen_capture: bool,
    pub allowed_capture_scopes: Vec<ScreenCaptureScope>,
    pub protected_surface_suspected: bool,
    pub credential_prompt_suspected: bool,
}

#[derive(Debug)]
pub enum ManagedBrowserScreenIntelligenceRouteError {
    Browser(ManagedBrowserCdpCaptureError),
    InvalidOwnerHandoff(ScreenManagedBrowserStructuredExtractionHandoffError),
    InconsistentDecision,
}

impl From<ManagedBrowserCdpCaptureError> for ManagedBrowserScreenIntelligenceRouteError {
    fn from(error: ManagedBrowserCdpCaptureError) -> Self {
        Self::Browser(error)
    }
}

struct ManagedBrowserStructuredExtractionHandoff {
    extraction: ManagedBrowserCdpStructuredExtraction,
}

impl ManagedBrowserStructuredExtractionOwner for ManagedBrowserStructuredExtractionHandoff {
    fn observation(&self) -> ManagedBrowserStructuredExtractionObservation {
        let refs = self.extraction.evidence_refs();
        let evidence_refs = [
            refs.target_ref.as_str(),
            refs.url_ref.as_str(),
            refs.title_ref.as_str(),
        ]
        .into_iter()
        .map(|evidence_id| ScreenActivityEvidenceRef {
            evidence_id: evidence_id.to_owned(),
            kind: String::from(MANAGED_BROWSER_STRUCTURED_EVIDENCE_KIND),
            digest: self.extraction.evidence_digest().to_owned(),
            uri: None,
        })
        .collect();
        ManagedBrowserStructuredExtractionObservation {
            source_id: self.extraction.source_id().to_owned(),
            extraction_id: self.extraction.extraction_id().to_owned(),
            captured_at: self.extraction.captured_at().to_owned(),
            managed_browser_session_ref: self.extraction.managed_browser_session_ref().to_owned(),
            target_ref: self.extraction.target_ref().to_owned(),
            evidence_refs,
            structured_evidence_digest: self.extraction.evidence_digest().to_owned(),
            structured_signal_digest: self.extraction.structured_signal_digest().to_owned(),
            structured_body_digest: self.extraction.structured_body_digest().to_owned(),
            document_frame_id: self.extraction.document_frame_id().map(str::to_owned),
            document_loader_id: self.extraction.document_loader_id().map(str::to_owned),
            document_url_digest: self.extraction.document_url_digest().map(str::to_owned),
            authority_digest: self.extraction.authority_digest().to_owned(),
            dom_overflow_redacted: self.extraction.dom_overflow_redacted(),
            private_content_redacted: self.extraction.private_content_redacted(),
            protected_content_skipped: self.extraction.protected_content_skipped(),
            fresh: self.extraction.is_fresh(),
            unavailable: self.extraction.is_unavailable(),
            custody_state: ScreenEvidenceCustodyState::Unavailable,
        }
    }
}

/// Compose the real managed-browser producer into the screen router. The
/// extraction is performed before the caller can request a screenshot. This
/// neutral observation is intentionally non-authorizing, so the current
/// result remains `Unavailable` until a domain-owned producer/policy handoff
/// exists; it never promotes ReviewRequired/unknown/redacted evidence to
/// `NoScreenNeeded`.
pub fn plan_managed_browser_screen_route(
    authority: &ManagedBrowserCdpTargetAuthority,
    input: ManagedBrowserScreenIntelligenceRequest,
) -> Result<ScreenIntelligenceRouteDecision, ManagedBrowserScreenIntelligenceRouteError> {
    let extraction = authority.extract_structured()?;
    let handoff = ScreenManagedBrowserStructuredExtraction::from_untrusted_observation(Box::new(
        ManagedBrowserStructuredExtractionHandoff { extraction },
    ))
    .map_err(ManagedBrowserScreenIntelligenceRouteError::InvalidOwnerHandoff)?;
    let request = ScreenIntelligenceRouteRequest {
        schema_version: input.schema_version,
        request_id: input.request_id,
        requested_at: input.requested_at,
        device_ref: input.device_ref,
        source_kind: ScreenIntelligenceSourceKind::ManagedBrowser,
        capture_reason: input.capture_reason,
        policy_question: input.policy_question,
        policy_sensitivity: input.policy_sensitivity,
        existing_evidence_refs: input.existing_evidence_refs,
        structured_extraction: Some(handoff),
        parent_allows_managed_browser_structured_extraction: input
            .parent_allows_managed_browser_structured_extraction,
        parent_allows_screen_capture: input.parent_allows_screen_capture,
        allowed_capture_scopes: input.allowed_capture_scopes,
        protected_surface_suspected: input.protected_surface_suspected,
        credential_prompt_suspected: input.credential_prompt_suspected,
    };
    let decision = plan_screen_intelligence_route(&request, input.route_id);
    if !ocentra_screen_ai_core::screen_intelligence_router::screen_intelligence_route_decision_is_consistent(&decision) {
        return Err(ManagedBrowserScreenIntelligenceRouteError::InconsistentDecision);
    }
    Ok(decision)
}

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
        && DateTime::parse_from_rfc3339(&receipt.captured_at).is_ok()
        && !receipt.structured_extraction_id.trim().is_empty()
        && valid_sha256_digest(receipt.structured_evidence_digest.as_bytes())
        && valid_sha256_digest(receipt.structured_signal_digest.as_bytes())
        && receipt
            .structured_body_digest
            .as_bytes()
            .strip_prefix(
                ocentra_schema::managed_browser_cdp_capture::
                    MANAGED_BROWSER_CDP_STRUCTURED_BODY_DIGEST_PREFIX
                    .as_bytes(),
            )
            .is_some_and(valid_sha256_digest)
        && !receipt.document_frame_id.trim().is_empty()
        && !receipt.document_loader_id.trim().is_empty()
        && valid_sha256_digest(receipt.document_url_digest.as_bytes())
        && valid_sha256_digest(receipt.authority_digest.as_bytes())
        && valid_sha256_digest(receipt.capture_context_digest.as_bytes())
        && receipt.width > 0
        && receipt.height > 0
        && receipt.width <= MANAGED_BROWSER_CDP_MAX_DIMENSION
        && receipt.height <= MANAGED_BROWSER_CDP_MAX_DIMENSION
        && u64::from(receipt.width) * u64::from(receipt.height) <= MANAGED_BROWSER_CDP_MAX_PIXELS
        && receipt.image_format == MANAGED_BROWSER_CDP_IMAGE_FORMAT
        && receipt.custody_state == SCREEN_CUSTODY_TEMP_QUEUE
        && receipt.image_byte_size == image_bytes.len() as u64
}

fn valid_sha256_digest(value: &[u8]) -> bool {
    value.len() == 64 && value.iter().all(u8::is_ascii_hexdigit)
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
