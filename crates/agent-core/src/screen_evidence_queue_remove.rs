use std::{collections::HashSet, fs::OpenOptions, io::Write};

use ocentra_parent_agent_protocol::constants;

use crate::JournalError;

use super::{screen_evidence_queue_record, ScreenEvidenceQueue};

pub(crate) fn remove_entries(
    queue: &ScreenEvidenceQueue,
    queue_job_ids: &[String],
) -> Result<u64, JournalError> {
    let contents = match std::fs::read_to_string(&queue.path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(error) => return Err(error.into()),
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
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&queue.path)?;
    for line in retained {
        file.write_all(line.as_bytes())?;
        file.write_all(&[constants::byte::NEWLINE])?;
    }
    file.sync_data()?;
    Ok(removed_count)
}
