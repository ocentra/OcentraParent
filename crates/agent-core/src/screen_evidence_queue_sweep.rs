use crate::JournalError;
use std::collections::HashSet;

use super::{
    screen_evidence_queue_outbox, screen_evidence_queue_record, ScreenEvidenceExpiredQueueEntry,
    ScreenEvidenceQueue, ScreenEvidenceQueueSweep,
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
    let contents = super::read_queue_contents(queue)?.unwrap_or_default();
    let leases = super::screen_evidence_queue_leases::read_leases(queue)?;
    let mut retained = Vec::new();
    let mut newly_expired = Vec::new();
    for line in contents.lines().filter(|line| !line.trim().is_empty()) {
        let record = screen_evidence_queue_record::decrypted_record_from_line(line)?;
        let actively_leased = leases.iter().any(|lease| {
            lease.queue_job_id == record.queue_job_id
                && screen_evidence_queue_record::timestamp_is_after(&lease.lease_expires_at, now)
        });
        if !actively_leased
            && screen_evidence_queue_record::queue_record_expired(record.expires_at.as_deref(), now)
        {
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
    let outbox = screen_evidence_queue_outbox::read_outbox(queue)?;
    super::screen_evidence_queue_outbox_quarantine::quarantine_corrupt_outbox(queue, &outbox)?;
    let failures = screen_evidence_queue_outbox::outbox_failures(&outbox.corrupt_lines);
    let mut pending = outbox.entries;
    let known = pending
        .iter()
        .map(|entry| entry.queue_job_id.clone())
        .collect::<HashSet<_>>();
    if !newly_expired.is_empty() {
        pending.extend(
            newly_expired
                .into_iter()
                .filter(|entry| !known.contains(&entry.queue_job_id)),
        );
        screen_evidence_queue_outbox::write_outbox_with_corrupt_lines(
            queue,
            &pending,
            &outbox.corrupt_lines,
        )?;
        // The replacement directory entry is durable before any queue record
        // can disappear.
        super::sync_parent_directory(&screen_evidence_queue_outbox::outbox_path(queue))?;
        super::replace_queue_lines(queue, &retained)?;
    }
    Ok(ScreenEvidenceQueueSweep {
        expired_entries: pending,
        retained_count: retained.len() as u64,
        outbox_failures: failures,
    })
}

pub(crate) fn acknowledge_expired_entries(
    queue: &ScreenEvidenceQueue,
    queue_job_ids: &[String],
) -> Result<u64, JournalError> {
    super::with_exclusive_queue_lock(queue, || {
        let ids = queue_job_ids.iter().collect::<HashSet<_>>();
        let outbox = screen_evidence_queue_outbox::read_outbox(queue)?;
        super::screen_evidence_queue_outbox_quarantine::quarantine_corrupt_outbox(queue, &outbox)?;
        let pending = outbox.entries;
        let retained = pending
            .iter()
            .filter(|entry| !ids.contains(&entry.queue_job_id))
            .cloned()
            .collect::<Vec<_>>();
        let removed = pending.len().saturating_sub(retained.len()) as u64;
        if removed == 0 {
            return Ok(0);
        }
        screen_evidence_queue_outbox::write_outbox_with_corrupt_lines(
            queue,
            &retained,
            &outbox.corrupt_lines,
        )?;
        Ok(removed)
    })
}

pub(crate) fn acknowledge_outbox_failures(
    queue: &ScreenEvidenceQueue,
    failures: &[super::ScreenEvidenceOutboxFailure],
) -> Result<u64, JournalError> {
    super::with_exclusive_queue_lock(queue, || {
        let outbox = screen_evidence_queue_outbox::read_outbox(queue)?;
        super::screen_evidence_queue_outbox_quarantine::quarantine_corrupt_outbox(queue, &outbox)?;
        let failure_ids = failures
            .iter()
            .map(|failure| failure.queue_job_id.as_str())
            .collect::<HashSet<_>>();
        let retained_corrupt = outbox
            .corrupt_lines
            .iter()
            .filter(|line| {
                screen_evidence_queue_outbox::outbox_failures(std::slice::from_ref(line))
                    .first()
                    .is_none_or(|failure| !failure_ids.contains(failure.queue_job_id.as_str()))
            })
            .cloned()
            .collect::<Vec<_>>();
        let removed = outbox
            .corrupt_lines
            .len()
            .saturating_sub(retained_corrupt.len()) as u64;
        if removed > 0 {
            screen_evidence_queue_outbox::write_outbox_with_corrupt_lines(
                queue,
                &outbox.entries,
                &retained_corrupt,
            )?;
        }
        Ok(removed)
    })
}
