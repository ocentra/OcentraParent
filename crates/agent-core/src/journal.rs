use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityJournalCipher, ActivityJournalLine, ActivityJournalStatus,
    ACTIVITY_JOURNAL_SCHEMA_VERSION,
};

use crate::{
    journal_crypto::{decrypt_payload, encrypt_payload, JournalKey},
    journal_error::JournalError,
};

pub struct ActivityJournal {
    path: PathBuf,
    key: JournalKey,
    status: ActivityJournalStatus,
}

impl ActivityJournal {
    pub fn open(path: PathBuf, key: JournalKey) -> Result<Self, JournalError> {
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        OpenOptions::new().create(true).append(true).open(&path)?;
        let status = status_from_path(&path)?;
        Ok(Self { path, key, status })
    }

    pub fn append(&mut self, event: &ActivityEvent) -> Result<ActivityJournalLine, JournalError> {
        let plaintext = serde_json::to_vec(event)?;
        let encrypted = encrypt_payload(&self.key, &plaintext)?;
        let line = ActivityJournalLine {
            schema_version: ACTIVITY_JOURNAL_SCHEMA_VERSION,
            entry_id: entry_id_from_nonce(&encrypted.nonce),
            written_at: journal_timestamp(),
            event_id: event.event_id.clone(),
            cipher: ActivityJournalCipher::XChaCha20Poly1305,
            nonce: encrypted.nonce,
            ciphertext: encrypted.ciphertext,
            activity_digest: encrypted.digest,
        };

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        serde_json::to_writer(&mut file, &line)?;
        file.write_all(&[constants::byte::NEWLINE])?;
        file.sync_data()?;

        self.status.entries_written += 1;
        self.status.bytes_written = file.metadata()?.len();
        self.status.last_entry_id = Some(line.entry_id.clone());
        Ok(line)
    }

    pub fn lines(&self) -> Result<Vec<ActivityJournalLine>, JournalError> {
        lines_from_path(&self.path)
    }

    pub fn decrypt_line(&self, line: &ActivityJournalLine) -> Result<ActivityEvent, JournalError> {
        let plaintext = decrypt_payload(&self.key, &line.nonce, &line.ciphertext)?;
        Ok(serde_json::from_slice(&plaintext)?)
    }

    pub fn status(&self) -> ActivityJournalStatus {
        self.status.clone()
    }
}

fn status_from_path(path: &PathBuf) -> Result<ActivityJournalStatus, JournalError> {
    let lines = lines_from_path(path)?;
    let last_entry_id = lines.last().map(|line| line.entry_id.clone());
    Ok(ActivityJournalStatus {
        schema_version: ACTIVITY_JOURNAL_SCHEMA_VERSION,
        encrypted: true,
        entries_written: lines.len() as u64,
        bytes_written: File::open(path)?.metadata()?.len(),
        last_entry_id,
    })
}

fn lines_from_path(path: &PathBuf) -> Result<Vec<ActivityJournalLine>, JournalError> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            lines.push(serde_json::from_str(&line)?);
        }
    }
    Ok(lines)
}

fn entry_id_from_nonce(nonce: &str) -> String {
    let mut entry_id = String::from(constants::journal::ENTRY_ID_PREFIX);
    entry_id.push_str(nonce);
    entry_id
}

fn journal_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
        .to_string()
}
