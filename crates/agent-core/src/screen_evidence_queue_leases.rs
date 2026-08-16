use std::{fs::OpenOptions, io::Write, path::PathBuf};

use atomicwrites::{AllowOverwrite, AtomicFile};

use crate::JournalError;

use super::{ScreenEvidenceQueue, ScreenEvidenceQueueLease};

struct LeaseRead {
    leases: Vec<ScreenEvidenceQueueLease>,
    corrupt_lines: Vec<(usize, String)>,
}

pub(crate) fn read_leases(
    queue: &ScreenEvidenceQueue,
) -> Result<Vec<ScreenEvidenceQueueLease>, JournalError> {
    match std::fs::read_to_string(lease_path(queue)) {
        Ok(contents) => repair_lease_read(queue, parse_lease_contents(&contents)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error.into()),
    }
}

fn parse_lease_contents(contents: &str) -> LeaseRead {
    let mut read = LeaseRead {
        leases: Vec::new(),
        corrupt_lines: Vec::new(),
    };
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str(line) {
            Ok(lease) => read.leases.push(lease),
            Err(_) => read.corrupt_lines.push((index + 1, line.to_string())),
        }
    }
    read
}

fn repair_lease_read(
    queue: &ScreenEvidenceQueue,
    read: LeaseRead,
) -> Result<Vec<ScreenEvidenceQueueLease>, JournalError> {
    if read.corrupt_lines.is_empty() {
        return Ok(read.leases);
    }
    quarantine_corrupt_leases(queue, &read.corrupt_lines)?;
    write_leases(queue, &read.leases)?;
    Ok(read.leases)
}

pub(crate) fn renew_claimed_entry(
    queue: &ScreenEvidenceQueue,
    queue_job_id: &str,
    lease_expires_at: &str,
) -> Result<bool, JournalError> {
    super::with_exclusive_queue_lock(queue, || {
        let mut leases = read_leases(queue)?;
        let Some(lease) = leases
            .iter_mut()
            .find(|lease| lease.queue_job_id == queue_job_id)
        else {
            return Ok(false);
        };
        lease.lease_expires_at = lease_expires_at.to_string();
        write_leases(queue, &leases)?;
        Ok(true)
    })
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

fn lease_path(queue: &ScreenEvidenceQueue) -> PathBuf {
    queue.path().with_extension("analysis-leases")
}

fn quarantine_corrupt_leases(
    queue: &ScreenEvidenceQueue,
    corrupt_lines: &[(usize, String)],
) -> Result<(), JournalError> {
    let path = lease_path(queue).with_extension("analysis-leases.quarantine");
    let mut quarantine = OpenOptions::new().create(true).append(true).open(&path)?;
    for (line_number, raw_record) in corrupt_lines {
        serde_json::to_writer(
            &mut quarantine,
            &serde_json::json!({
                "lineNumber": line_number,
                "rawRecord": raw_record,
            }),
        )?;
        quarantine.write_all(b"\n")?;
    }
    quarantine.sync_all()?;
    super::sync_parent_directory(&path)?;
    Ok(())
}
