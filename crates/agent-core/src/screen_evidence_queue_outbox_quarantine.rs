use std::{collections::HashSet, io::Write};

use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_DELETION_OUTBOX_QUARANTINE_EXTENSION;

use crate::JournalError;

use super::{
    screen_evidence_queue_outbox::{outbox_path, OutboxRead},
    ScreenEvidenceQueue,
};

pub(crate) fn quarantine_corrupt_outbox(
    queue: &ScreenEvidenceQueue,
    outbox: &OutboxRead,
) -> Result<(), JournalError> {
    if outbox.corrupt_lines.is_empty() {
        return Ok(());
    }
    let quarantine_path =
        outbox_path(queue).with_extension(SCREEN_SERVICE_DELETION_OUTBOX_QUARANTINE_EXTENSION);
    let existing_records = existing_quarantine_records(&quarantine_path);
    let mut quarantine = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&quarantine_path)?;
    for (line_number, raw_record) in &outbox.corrupt_lines {
        if existing_records.contains(raw_record) {
            continue;
        }
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
    super::sync_parent_directory(&quarantine_path)?;
    Ok(())
}

fn existing_quarantine_records(path: &std::path::Path) -> HashSet<String> {
    std::fs::read_to_string(path)
        .ok()
        .into_iter()
        .flat_map(|contents| {
            contents
                .lines()
                .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
                .filter_map(|value| {
                    value
                        .get("rawRecord")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
