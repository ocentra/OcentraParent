use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisQueueJob;

use crate::{
    journal_crypto::{decrypt_payload, encrypt_payload, JournalKey},
    JournalError,
};

use super::{DecryptedScreenEvidenceQueueEntry, ScreenEvidenceQueue, ScreenEvidenceQueueLease};

pub(crate) fn open(
    directory: impl AsRef<Path>,
    key: JournalKey,
) -> Result<ScreenEvidenceQueue, JournalError> {
    create_dir_all(directory.as_ref())?;
    let metadata = std::fs::symlink_metadata(directory.as_ref())?;
    if metadata.file_type().is_symlink() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "screen evidence queue directory must not be a symlink",
        )
        .into());
    }
    let directory = directory.as_ref().canonicalize()?;
    let path = directory.join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    OpenOptions::new().create(true).append(true).open(&path)?;
    Ok(ScreenEvidenceQueue { path, key })
}

pub(crate) fn append_encrypted_image(
    queue: &ScreenEvidenceQueue,
    job: &ScreenAnalysisQueueJob,
    image_bytes: &[u8],
) -> Result<(), JournalError> {
    let encrypted = encrypt_payload(&queue.key, image_bytes)?;
    let record = super::screen_evidence_queue_record::encrypted_record_from_job(job, encrypted);
    super::with_exclusive_queue_lock(queue, || {
        let mut file = OpenOptions::new().append(true).open(&queue.path)?;
        serde_json::to_writer(&mut file, &record)?;
        file.write_all(&[constants::byte::NEWLINE])?;
        file.sync_all()?;
        Ok(())
    })
}

pub(crate) fn read_decrypted_entries(
    queue: &ScreenEvidenceQueue,
    max_entries: usize,
) -> Result<Vec<DecryptedScreenEvidenceQueueEntry>, JournalError> {
    let contents = super::with_exclusive_queue_lock(queue, || {
        Ok(super::read_queue_contents(queue)?.unwrap_or_default())
    })?;
    let mut entries = Vec::new();
    for line in contents.lines().filter(|line| !line.trim().is_empty()) {
        if entries.len() >= max_entries {
            break;
        }
        let record = super::screen_evidence_queue_record::decrypted_record_from_line(line)?;
        let image_bytes = decrypt_payload(&queue.key, &record.nonce, &record.ciphertext)?;
        entries.push(DecryptedScreenEvidenceQueueEntry {
            schema_version: record.schema_version,
            queue_job_id: record.queue_job_id,
            created_at: record.created_at,
            expires_at: record.expires_at,
            status: record.status,
            deletion_required: record.deletion_required,
            deletion_status: record.deletion_status,
            deletion_proof_ref: record.deletion_proof_ref,
            custody_state: record.custody_state,
            image_digest: record.image_digest,
            image_bytes,
        });
    }
    Ok(entries)
}

pub(crate) fn claim_first_decrypted_entry(
    queue: &ScreenEvidenceQueue,
    max_entries: usize,
    now: &str,
    lease_expires_at: &str,
) -> Result<Option<DecryptedScreenEvidenceQueueEntry>, JournalError> {
    super::with_exclusive_queue_lock(queue, || {
        let contents = super::read_queue_contents(queue)?.unwrap_or_default();
        let mut leases = read_leases(queue)?;
        let original_lease_count = leases.len();
        leases.retain(|lease| {
            super::screen_evidence_queue_record::timestamp_is_after(&lease.lease_expires_at, now)
        });
        for line in contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .take(max_entries)
        {
            let record = super::screen_evidence_queue_record::decrypted_record_from_line(line)?;
            if leases
                .iter()
                .any(|lease| lease.queue_job_id == record.queue_job_id)
            {
                continue;
            }
            let image_bytes = decrypt_payload(&queue.key, &record.nonce, &record.ciphertext)?;
            leases.push(ScreenEvidenceQueueLease {
                queue_job_id: record.queue_job_id.clone(),
                lease_expires_at: lease_expires_at.to_string(),
            });
            write_leases(queue, &leases)?;
            return Ok(Some(decrypted_entry(record, image_bytes)));
        }
        if leases.len() != original_lease_count {
            write_leases(queue, &leases)?;
        }
        Ok(None)
    })
}

pub(crate) fn read_leases(
    queue: &ScreenEvidenceQueue,
) -> Result<Vec<ScreenEvidenceQueueLease>, JournalError> {
    match std::fs::read_to_string(lease_path(queue)) {
        Ok(contents) => contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(serde_json::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error.into()),
    }
}

pub(crate) fn write_leases(
    queue: &ScreenEvidenceQueue,
    leases: &[ScreenEvidenceQueueLease],
) -> Result<(), JournalError> {
    let path = lease_path(queue);
    let body = leases
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");
    AtomicFile::new(&path, AllowOverwrite)
        .write(|file| {
            file.write_all(body.as_bytes())?;
            if !body.is_empty() {
                file.write_all(b"\n")?;
            }
            file.sync_all()
        })
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    super::sync_parent_directory(&path)?;
    Ok(())
}

fn lease_path(queue: &ScreenEvidenceQueue) -> std::path::PathBuf {
    queue.path().with_extension("analysis-leases")
}

fn decrypted_entry(
    record: super::screen_evidence_queue_record::EncryptedScreenEvidenceQueueRecord,
    image_bytes: Vec<u8>,
) -> DecryptedScreenEvidenceQueueEntry {
    DecryptedScreenEvidenceQueueEntry {
        schema_version: record.schema_version,
        queue_job_id: record.queue_job_id,
        created_at: record.created_at,
        expires_at: record.expires_at,
        status: record.status,
        deletion_required: record.deletion_required,
        deletion_status: record.deletion_status,
        deletion_proof_ref: record.deletion_proof_ref,
        custody_state: record.custody_state,
        image_digest: record.image_digest,
        image_bytes,
    }
}
