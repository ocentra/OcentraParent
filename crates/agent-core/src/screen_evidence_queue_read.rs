use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisQueueJob;

use crate::{
    journal_crypto::{decrypt_payload, encrypt_payload, JournalKey},
    JournalError,
};

use super::{DecryptedScreenEvidenceQueueEntry, ScreenEvidenceQueue};

pub(crate) fn open(
    directory: impl AsRef<Path>,
    key: JournalKey,
) -> Result<ScreenEvidenceQueue, JournalError> {
    create_dir_all(directory.as_ref())?;
    let path = directory
        .as_ref()
        .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
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
    let mut file = OpenOptions::new().append(true).open(&queue.path)?;
    serde_json::to_writer(&mut file, &record)?;
    file.write_all(&[constants::byte::NEWLINE])?;
    file.sync_data()?;
    Ok(())
}

pub(crate) fn read_decrypted_entries(
    queue: &ScreenEvidenceQueue,
    max_entries: usize,
) -> Result<Vec<DecryptedScreenEvidenceQueueEntry>, JournalError> {
    let contents = match std::fs::read_to_string(&queue.path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
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
