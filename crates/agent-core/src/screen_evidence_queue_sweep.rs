use std::{fs::OpenOptions, io::Write};

use ocentra_parent_agent_protocol::constants;

use crate::JournalError;

use super::{
    screen_evidence_queue_record, ScreenEvidenceExpiredQueueEntry, ScreenEvidenceQueue,
    ScreenEvidenceQueueSweep,
};

pub(crate) fn remove_expired_entries(
    queue: &ScreenEvidenceQueue,
    now: &str,
    deletion_proof_prefix: &str,
) -> Result<ScreenEvidenceQueueSweep, JournalError> {
    let contents = match std::fs::read_to_string(&queue.path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(ScreenEvidenceQueueSweep {
                expired_entries: Vec::new(),
                retained_count: 0,
            });
        }
        Err(error) => return Err(error.into()),
    };
    let mut retained = Vec::new();
    let mut expired_entries = Vec::new();
    for line in contents.lines().filter(|line| !line.trim().is_empty()) {
        let record = screen_evidence_queue_record::decrypted_record_from_line(line)?;
        if screen_evidence_queue_record::queue_record_expired(record.expires_at.as_deref(), now) {
            expired_entries.push(ScreenEvidenceExpiredQueueEntry {
                queue_job_id: record.queue_job_id.clone(),
                image_digest: record.image_digest,
                expires_at: record.expires_at.unwrap_or_default(),
                deletion_proof_ref: screen_evidence_queue_record::prefixed_ref(
                    deletion_proof_prefix,
                    &record.queue_job_id,
                ),
            });
        } else {
            retained.push(line);
        }
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&queue.path)?;
    for line in &retained {
        file.write_all(line.as_bytes())?;
        file.write_all(&[constants::byte::NEWLINE])?;
    }
    file.sync_data()?;
    Ok(ScreenEvidenceQueueSweep {
        expired_entries,
        retained_count: retained.len() as u64,
    })
}
