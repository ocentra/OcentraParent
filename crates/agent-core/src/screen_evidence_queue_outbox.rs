use std::fs::File;
use std::io::Write;

use atomicwrites::{AllowOverwrite, AtomicFile};
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_SERVICE_DELETION_OUTBOX_CORRUPT_ID_PREFIX, SCREEN_SERVICE_DELETION_OUTBOX_EXTENSION,
    SCREEN_SERVICE_DELETION_OUTBOX_QUARANTINE_PROOF_PREFIX,
};
use sha2::{Digest, Sha256};

use crate::screen_evidence_queue::{
    ScreenEvidenceExpiredQueueEntry, ScreenEvidenceOutboxFailure, ScreenEvidenceQueue,
};
use crate::JournalError;

pub(crate) struct OutboxRead {
    pub(crate) entries: Vec<ScreenEvidenceExpiredQueueEntry>,
    pub(crate) corrupt_lines: Vec<(usize, String)>,
}

pub(crate) fn outbox_path(queue: &ScreenEvidenceQueue) -> std::path::PathBuf {
    queue
        .path()
        .with_extension(SCREEN_SERVICE_DELETION_OUTBOX_EXTENSION)
}

pub(crate) fn read_outbox(queue: &ScreenEvidenceQueue) -> Result<OutboxRead, JournalError> {
    let path = outbox_path(queue);
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(parse_outbox_contents(&contents)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(OutboxRead {
            entries: Vec::new(),
            corrupt_lines: Vec::new(),
        }),
        Err(error) => Err(error.into()),
    }
}

fn parse_outbox_contents(contents: &str) -> OutboxRead {
    let mut entries = Vec::new();
    let mut corrupt_lines = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str(line) {
            Ok(entry) => entries.push(entry),
            Err(_) => corrupt_lines.push((index + 1, line.to_string())),
        }
    }
    OutboxRead {
        entries,
        corrupt_lines,
    }
}

pub(crate) fn write_outbox_with_corrupt_lines(
    queue: &ScreenEvidenceQueue,
    entries: &[ScreenEvidenceExpiredQueueEntry],
    corrupt_lines: &[(usize, String)],
) -> Result<(), JournalError> {
    let path = outbox_path(queue);
    let mut lines = entries
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()?;
    lines.extend(
        corrupt_lines
            .iter()
            .map(|(_, raw_record)| raw_record.clone()),
    );
    AtomicFile::new(&path, AllowOverwrite)
        .write(|file| write_outbox_contents(file, &lines.join("\n")))
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    crate::screen_evidence_queue::sync_parent_directory(&path)?;
    Ok(())
}

fn write_outbox_contents(file: &mut File, body: &str) -> std::io::Result<()> {
    file.write_all(body.as_bytes())?;
    if !body.is_empty() {
        file.write_all(b"\n")?;
    }
    file.sync_all()
}

pub(crate) fn outbox_failures(
    corrupt_lines: &[(usize, String)],
) -> Vec<ScreenEvidenceOutboxFailure> {
    corrupt_lines
        .iter()
        .map(|(_line_number, raw_record)| {
            let digest = format!("{:x}", Sha256::digest(raw_record.as_bytes()));
            ScreenEvidenceOutboxFailure {
                queue_job_id: format!("{SCREEN_SERVICE_DELETION_OUTBOX_CORRUPT_ID_PREFIX}{digest}"),
                malformed_record_digest: digest.clone(),
                deletion_proof_ref: format!(
                    "{SCREEN_SERVICE_DELETION_OUTBOX_QUARANTINE_PROOF_PREFIX}{digest}"
                ),
            }
        })
        .collect()
}
