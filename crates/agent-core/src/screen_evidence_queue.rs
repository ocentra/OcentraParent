use std::path::{Path, PathBuf};

use crate::journal_crypto::JournalKey;

#[path = "screen_evidence_queue_read.rs"]
mod screen_evidence_queue_read;
#[path = "screen_evidence_queue_record.rs"]
mod screen_evidence_queue_record;
#[path = "screen_evidence_queue_remove.rs"]
mod screen_evidence_queue_remove;
#[path = "screen_evidence_queue_sweep.rs"]
mod screen_evidence_queue_sweep;

pub struct ScreenEvidenceQueue {
    path: PathBuf,
    key: JournalKey,
}

pub struct DecryptedScreenEvidenceQueueEntry {
    pub schema_version: u16,
    pub queue_job_id: String,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
    pub status: String,
    pub deletion_required: bool,
    pub deletion_status: String,
    pub deletion_proof_ref: Option<String>,
    pub custody_state: String,
    pub image_digest: String,
    pub image_bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenEvidenceExpiredQueueEntry {
    pub queue_job_id: String,
    pub image_digest: String,
    pub expires_at: String,
    pub deletion_proof_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenEvidenceQueueSweep {
    pub expired_entries: Vec<ScreenEvidenceExpiredQueueEntry>,
    pub retained_count: u64,
}

impl ScreenEvidenceQueue {
    pub fn open(directory: impl AsRef<Path>, key: JournalKey) -> Result<Self, crate::JournalError> {
        screen_evidence_queue_read::open(directory, key)
    }

    pub fn append_encrypted_image(
        &self,
        job: &ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisQueueJob,
        image_bytes: &[u8],
    ) -> Result<(), crate::JournalError> {
        screen_evidence_queue_read::append_encrypted_image(self, job, image_bytes)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read_decrypted_entries(
        &self,
        max_entries: usize,
    ) -> Result<Vec<DecryptedScreenEvidenceQueueEntry>, crate::JournalError> {
        screen_evidence_queue_read::read_decrypted_entries(self, max_entries)
    }

    pub fn remove_entries(&self, queue_job_ids: &[String]) -> Result<u64, crate::JournalError> {
        screen_evidence_queue_remove::remove_entries(self, queue_job_ids)
    }

    pub fn remove_expired_entries(
        &self,
        now: &str,
        deletion_proof_prefix: &str,
    ) -> Result<ScreenEvidenceQueueSweep, crate::JournalError> {
        screen_evidence_queue_sweep::remove_expired_entries(self, now, deletion_proof_prefix)
    }
}
