use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;

use crate::journal_crypto::JournalKey;

#[path = "screen_evidence_queue_outbox.rs"]
mod screen_evidence_queue_outbox;
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    pub outbox_failures: Vec<ScreenEvidenceOutboxFailure>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenEvidenceOutboxFailure {
    pub queue_job_id: String,
    pub malformed_record_digest: String,
    pub deletion_proof_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct ScreenEvidenceQueueLease {
    pub(crate) queue_job_id: String,
    pub(crate) lease_expires_at: String,
}

pub(crate) fn with_exclusive_queue_lock<T>(
    queue: &ScreenEvidenceQueue,
    operation: impl FnOnce() -> Result<T, crate::JournalError>,
) -> Result<T, crate::JournalError> {
    let lock = queue_lock_file(queue)?;
    lock.lock_exclusive()?;
    let result = operation();
    match result {
        Ok(value) => {
            FileExt::unlock(&lock)?;
            Ok(value)
        }
        Err(error) => {
            let _ = FileExt::unlock(&lock);
            Err(error)
        }
    }
}

pub(crate) fn replace_queue_lines(
    queue: &ScreenEvidenceQueue,
    lines: &[&str],
) -> Result<(), crate::JournalError> {
    AtomicFile::new(&queue.path, AllowOverwrite)
        .write(|file| write_queue_lines(file, lines))
        .map_err(|error| io::Error::other(error.to_string()))?;
    sync_parent_directory(&queue.path)?;
    Ok(())
}

pub(crate) fn read_queue_contents(
    queue: &ScreenEvidenceQueue,
) -> Result<Option<String>, crate::JournalError> {
    match std::fs::read_to_string(&queue.path) {
        Ok(contents) => Ok(Some(contents)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn queue_lock_file(queue: &ScreenEvidenceQueue) -> Result<File, crate::JournalError> {
    let lock_path = queue.path.with_extension("lock");
    Ok(OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(lock_path)?)
}

fn write_queue_lines(file: &mut File, lines: &[&str]) -> io::Result<()> {
    for line in lines {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }
    file.sync_all()
}

#[cfg(not(windows))]
pub(crate) fn sync_parent_directory(path: &Path) -> io::Result<()> {
    File::open(path.parent().unwrap_or_else(|| Path::new(".")))?.sync_all()
}

#[cfg(windows)]
pub(crate) fn sync_parent_directory(_path: &Path) -> io::Result<()> {
    Ok(())
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

    pub fn claim_first_decrypted_entry(
        &self,
        max_entries: usize,
        now: &str,
        lease_expires_at: &str,
    ) -> Result<Option<DecryptedScreenEvidenceQueueEntry>, crate::JournalError> {
        screen_evidence_queue_read::claim_first_decrypted_entry(
            self,
            max_entries,
            now,
            lease_expires_at,
        )
    }

    pub fn complete_claimed_entry(&self, queue_job_id: &str) -> Result<(), crate::JournalError> {
        screen_evidence_queue_remove::complete_claimed_entry(self, queue_job_id)
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

    pub fn acknowledge_expired_entries(
        &self,
        queue_job_ids: &[String],
    ) -> Result<u64, crate::JournalError> {
        screen_evidence_queue_sweep::acknowledge_expired_entries(self, queue_job_ids)
    }
}
