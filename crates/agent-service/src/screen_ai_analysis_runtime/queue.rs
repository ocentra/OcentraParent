use std::{fs, path::Path};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    screen_evidence_queue::ScreenEvidenceQueue,
};
use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult;

use crate::activity_capture::ActivityCaptureError;

use super::ScreenAiAnalysisCycleClock;

const SCREEN_ANALYSIS_LEASE_SECONDS: i64 = 30 * 60;

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
) -> Result<Option<QueuedScreenImage>, ActivityCaptureError> {
    let lease_expires_at = chrono::DateTime::parse_from_rfc3339(&clock.timestamp)
        .map_err(|_parse_error| ActivityCaptureError::Io)?
        .checked_add_signed(chrono::Duration::seconds(SCREEN_ANALYSIS_LEASE_SECONDS))
        .ok_or(ActivityCaptureError::Io)?
        .to_rfc3339();
    Ok(queue
        .claim_first_decrypted_entry(
            max_queue_scan,
            clock.timestamp.as_str(),
            lease_expires_at.as_str(),
        )?
        .map(|entry| QueuedScreenImage {
            queue_job_id: entry.queue_job_id,
            custody_state: entry.custody_state,
            image_digest: entry.image_digest,
            image_bytes: entry.image_bytes,
        }))
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
