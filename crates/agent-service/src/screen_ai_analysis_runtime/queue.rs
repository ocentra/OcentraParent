use std::{fs, path::Path};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    screen_evidence_queue::ScreenEvidenceQueue,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult;

use crate::activity_capture::ActivityCaptureError;

use super::ScreenAiAnalysisCycleClock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct QueuedScreenImage {
    pub(super) queue_job_id: String,
    pub(super) custody_state: String,
    pub(super) image_digest: String,
    pub(super) image_bytes: Vec<u8>,
}

pub(super) fn first_queued_screen_image(
    queue: &ScreenEvidenceQueue,
    max_queue_scan: usize,
) -> Result<Option<QueuedScreenImage>, ActivityCaptureError> {
    Ok(queue
        .read_decrypted_entries(max_queue_scan)?
        .into_iter()
        .next()
        .map(|entry| QueuedScreenImage {
            queue_job_id: entry.queue_job_id,
            custody_state: entry.custody_state,
            image_digest: entry.image_digest,
            image_bytes: entry.image_bytes,
        }))
}

pub(super) fn metadata_result_for_queue_job(
    store_path: &Path,
    queue_job_id: &str,
    clock: &ScreenAiAnalysisCycleClock,
) -> Result<Option<ScreenAnalysisResult>, ActivityCaptureError> {
    let store = ActivityStore::open(store_path)?;
    let summary = store.screen_evidence_recent_summary(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        &clock.timestamp,
    )?;
    Ok(summary
        .results
        .into_iter()
        .find(|result| result.queue_job_id == queue_job_id))
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
