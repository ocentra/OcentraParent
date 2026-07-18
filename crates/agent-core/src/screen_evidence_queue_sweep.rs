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
    super::with_exclusive_queue_lock(queue, || {
        remove_expired_entries_locked(queue, now, deletion_proof_prefix)
    })
}

fn remove_expired_entries_locked(
    queue: &ScreenEvidenceQueue,
    now: &str,
    deletion_proof_prefix: &str,
) -> Result<ScreenEvidenceQueueSweep, JournalError> {
    let contents = match super::read_queue_contents(queue)? {
        Some(contents) => contents,
        None => {
            return Ok(ScreenEvidenceQueueSweep {
                expired_entries: Vec::new(),
                retained_count: 0,
            });
        }
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
    super::replace_queue_lines(queue, &retained)?;
    Ok(ScreenEvidenceQueueSweep {
        expired_entries,
        retained_count: retained.len() as u64,
    })
}
