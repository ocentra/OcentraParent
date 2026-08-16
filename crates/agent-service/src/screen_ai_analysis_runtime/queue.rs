use std::{fs, path::Path};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    screen_evidence_queue::ScreenEvidenceQueue,
};
use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult;

use crate::activity_capture::ActivityCaptureError;

use super::ScreenAiAnalysisCycleClock;

const SCREEN_ANALYSIS_LEASE_MINIMUM_MS: u64 = 5 * 60 * 1_000;
const SCREEN_ANALYSIS_LEASE_SAFETY_MS: u64 = 60 * 1_000;

pub(super) struct AnalysisLeaseExpiresAt(pub(super) String);
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct QueuedScreenImage {
    pub(crate) queue_job_id: String,
    pub(crate) custody_state: String,
    pub(crate) image_digest: String,
    pub(crate) image_bytes: Vec<u8>,
}

pub(super) fn first_queued_screen_image(
    queue: &ScreenEvidenceQueue,
    max_queue_scan: usize,
    clock: &ScreenAiAnalysisCycleClock,
    adapter_timeout_ms: u64,
) -> Result<Option<QueuedScreenImage>, ActivityCaptureError> {
    let lease_expires_at = analysis_lease_expires_at(clock, adapter_timeout_ms)?;
    Ok(queue
        .claim_first_decrypted_entry(
            max_queue_scan,
            clock.timestamp.as_str(),
            &lease_expires_at.0,
        )?
        .map(|entry| QueuedScreenImage {
            queue_job_id: entry.queue_job_id,
            custody_state: entry.custody_state,
            image_digest: entry.image_digest,
            image_bytes: entry.image_bytes,
        }))
}

pub(super) fn analysis_lease_expires_at(
    clock: &ScreenAiAnalysisCycleClock,
    adapter_timeout_ms: u64,
) -> Result<AnalysisLeaseExpiresAt, ActivityCaptureError> {
    let lease_duration_ms = adapter_timeout_ms
        .saturating_add(SCREEN_ANALYSIS_LEASE_SAFETY_MS)
        .max(SCREEN_ANALYSIS_LEASE_MINIMUM_MS);
    let lease_duration_ms =
        i64::try_from(lease_duration_ms).map_err(|_overflow| ActivityCaptureError::Io)?;
    chrono::DateTime::parse_from_rfc3339(&clock.timestamp)
        .map_err(|_parse_error| ActivityCaptureError::Io)?
        .checked_add_signed(chrono::Duration::milliseconds(lease_duration_ms))
        .ok_or(ActivityCaptureError::Io)
        .map(|timestamp| AnalysisLeaseExpiresAt(timestamp.to_rfc3339()))
}

pub(super) fn metadata_result_for_queue_job(
    store_path: &Path,
    image: &QueuedScreenImage,
    clock: &ScreenAiAnalysisCycleClock,
) -> Result<Option<ScreenAnalysisResult>, ActivityCaptureError> {
    let store = ActivityStore::open(store_path)?;
    let _ = clock;
    Ok(store.screen_evidence_result_for_queue_job(&image.queue_job_id)?)
}

pub(super) fn load_existing_screen_key(
    path: &Path,
) -> Result<Option<JournalKey>, ActivityCaptureError> {
    match fs::read(path) {
        Ok(bytes) => journal_key_from_bytes(&bytes).map(Some),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err(ActivityCaptureError::Io),
    }
}

fn journal_key_from_bytes(bytes: &[u8]) -> Result<JournalKey, ActivityCaptureError> {
    if bytes.len() != JOURNAL_KEY_BYTES {
        return Err(ActivityCaptureError::InvalidKeyLength);
    }
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(bytes);
    Ok(JournalKey::from_bytes(key))
}
