use std::collections::HashSet;

use crate::JournalError;

use super::{screen_evidence_queue_record, ScreenEvidenceQueue};

pub(crate) fn remove_entries(
    queue: &ScreenEvidenceQueue,
    queue_job_ids: &[String],
) -> Result<u64, JournalError> {
    super::with_exclusive_queue_lock(queue, || {
        let Some(contents) = super::read_queue_contents(queue)? else {
            return Ok(0);
        };
        let ids = queue_job_ids.iter().collect::<HashSet<_>>();
        let mut retained = Vec::new();
        let mut removed_count = 0;
        for line in contents.lines().filter(|line| !line.trim().is_empty()) {
            let record = screen_evidence_queue_record::decrypted_record_from_line(line)?;
            if ids.contains(&record.queue_job_id) {
                removed_count += 1;
            } else {
                retained.push(line);
            }
        }
        super::replace_queue_lines(queue, &retained)?;
        Ok(removed_count)
    })
}

pub(crate) fn complete_claimed_entry(
    queue: &ScreenEvidenceQueue,
    queue_job_id: &str,
) -> Result<(), JournalError> {
    super::with_exclusive_queue_lock(queue, || {
        let contents = super::read_queue_contents(queue)?.unwrap_or_default();
        let mut retained = Vec::new();
        let mut removed_count = 0_u64;
        for line in contents.lines().filter(|line| !line.trim().is_empty()) {
            let record = screen_evidence_queue_record::decrypted_record_from_line(line)?;
            if record.queue_job_id == queue_job_id {
                removed_count += 1;
            } else {
                retained.push(line);
            }
        }
        if removed_count == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "claimed screen queue job was not removed",
            )
            .into());
        }
        super::replace_queue_lines(queue, &retained)?;
        let leases = super::screen_evidence_queue_leases::read_leases(queue)?
            .into_iter()
            .filter(|lease| lease.queue_job_id != queue_job_id)
            .collect::<Vec<_>>();
        super::screen_evidence_queue_leases::write_leases(queue, &leases)
    })
}

pub(crate) fn release_claimed_entry(
    queue: &ScreenEvidenceQueue,
    queue_job_id: &str,
) -> Result<(), JournalError> {
    super::with_exclusive_queue_lock(queue, || {
        let leases = super::screen_evidence_queue_leases::read_leases(queue)?
            .into_iter()
            .filter(|lease| lease.queue_job_id != queue_job_id)
            .collect::<Vec<_>>();
        super::screen_evidence_queue_leases::write_leases(queue, &leases)
    })
}

impl ScreenEvidenceQueue {
    pub fn release_claimed_entry(&self, queue_job_id: &str) -> Result<(), JournalError> {
        release_claimed_entry(self, queue_job_id)
    }
}
