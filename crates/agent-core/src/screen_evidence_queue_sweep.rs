use crate::JournalError;
use std::collections::HashSet;

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
                expired_entries: read_outbox(queue)?,
                retained_count: 0,
            });
        }
    };
    let mut retained = Vec::new();
    let mut newly_expired = Vec::new();
    for line in contents.lines().filter(|line| !line.trim().is_empty()) {
        let record = screen_evidence_queue_record::decrypted_record_from_line(line)?;
        if screen_evidence_queue_record::queue_record_expired(record.expires_at.as_deref(), now) {
            newly_expired.push(ScreenEvidenceExpiredQueueEntry {
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
    // The deletion outbox is the durable intent. It is synced before the queue
    // is mutated, so a process crash or later publication failure cannot lose a
    // deletion that still needs a terminal read-model/event projection.
    let mut pending = read_outbox(queue)?;
    let known = pending
        .iter()
        .map(|entry| entry.queue_job_id.clone())
        .collect::<HashSet<_>>();
    pending.extend(
        newly_expired
            .into_iter()
            .filter(|entry| !known.contains(&entry.queue_job_id)),
    );
    write_outbox(queue, &pending)?;
    super::replace_queue_lines(queue, &retained)?;
    Ok(ScreenEvidenceQueueSweep {
        expired_entries: pending,
        retained_count: retained.len() as u64,
    })
}

pub(crate) fn acknowledge_expired_entries(
    queue: &ScreenEvidenceQueue,
    queue_job_ids: &[String],
) -> Result<u64, JournalError> {
    super::with_exclusive_queue_lock(queue, || {
        let ids = queue_job_ids.iter().collect::<HashSet<_>>();
        let pending = read_outbox(queue)?;
        let retained = pending
            .iter()
            .filter(|entry| !ids.contains(&entry.queue_job_id))
            .cloned()
            .collect::<Vec<_>>();
        let removed = pending.len().saturating_sub(retained.len()) as u64;
        write_outbox(queue, &retained)?;
        Ok(removed)
    })
}

fn outbox_path(queue: &ScreenEvidenceQueue) -> std::path::PathBuf {
    queue.path().with_extension("deletion-outbox")
}

fn read_outbox(
    queue: &ScreenEvidenceQueue,
) -> Result<Vec<ScreenEvidenceExpiredQueueEntry>, JournalError> {
    let path = outbox_path(queue);
    match std::fs::read_to_string(path) {
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

fn write_outbox(
    queue: &ScreenEvidenceQueue,
    entries: &[ScreenEvidenceExpiredQueueEntry],
) -> Result<(), JournalError> {
    let path = outbox_path(queue);
    let body = entries
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");
    let temp = path.with_extension("deletion-outbox.tmp");
    std::fs::write(
        &temp,
        if body.is_empty() {
            body
        } else {
            format!("{body}\n")
        },
    )?;
    std::fs::rename(temp, path)?;
    Ok(())
}
