use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ACTIVITY_JOURNAL_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::journal::{
    ActivityJournalCipher, ActivityJournalLine, ActivityJournalRotationPolicy,
    ActivityJournalStatus,
};
use sha2::{Digest, Sha256};

use crate::{
    journal_crypto::{decrypt_payload, encrypt_payload, JournalKey},
    journal_error::JournalError,
    journal_rotation::{
        active_segment_id, default_rotation_policy, rotate_if_needed, segment_paths,
    },
};

pub struct ActivityJournal {
    path: PathBuf,
    key: JournalKey,
    rotation_policy: ActivityJournalRotationPolicy,
    status: ActivityJournalStatus,
}

impl ActivityJournal {
    pub fn open(path: PathBuf, key: JournalKey) -> Result<Self, JournalError> {
        Self::open_with_policy(path, key, default_rotation_policy())
    }

    pub fn open_with_policy(
        path: PathBuf,
        key: JournalKey,
        rotation_policy: ActivityJournalRotationPolicy,
    ) -> Result<Self, JournalError> {
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        OpenOptions::new().create(true).append(true).open(&path)?;
        let status = status_from_path(&path, &rotation_policy)?;
        Ok(Self {
            path,
            key,
            rotation_policy,
            status,
        })
    }

    pub fn append(&mut self, event: &ActivityEvent) -> Result<ActivityJournalLine, JournalError> {
        rotate_if_needed(&self.path, &self.rotation_policy)?;
        let plaintext = serde_json::to_vec(event)?;
        let encrypted = encrypt_payload(&self.key, &plaintext)?;
        let line = ActivityJournalLine {
            schema_version: ACTIVITY_JOURNAL_SCHEMA_VERSION,
            entry_id: entry_id_from_nonce(&encrypted.nonce),
            segment_id: active_segment_id(&self.path)?,
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

        self.status = status_from_path(&self.path, &self.rotation_policy)?;
        Ok(line)
    }

    pub fn lines(&self) -> Result<Vec<ActivityJournalLine>, JournalError> {
        lines_from_paths(&segment_paths(&self.path)?)
    }

    pub fn decrypt_line(&self, line: &ActivityJournalLine) -> Result<ActivityEvent, JournalError> {
        if line.schema_version != ACTIVITY_JOURNAL_SCHEMA_VERSION
            || line.entry_id != entry_id_from_nonce(&line.nonce)
        {
            return Err(JournalError::Crypto);
        }
        let plaintext = decrypt_payload(&self.key, &line.nonce, &line.ciphertext)?;
        let activity_digest = BASE64_URL_SAFE_NO_PAD.encode(Sha256::digest(&plaintext));
        if activity_digest != line.activity_digest {
            return Err(JournalError::Crypto);
        }
        let event = serde_json::from_slice::<ActivityEvent>(&plaintext)?;
        if event.event_id != line.event_id {
            return Err(JournalError::Crypto);
        }
        Ok(event)
    }

    pub fn status(&self) -> ActivityJournalStatus {
        self.status.clone()
    }
}

fn status_from_path(
    path: &Path,
    rotation_policy: &ActivityJournalRotationPolicy,
) -> Result<ActivityJournalStatus, JournalError> {
    let paths = segment_paths(path)?;
    let lines = lines_from_paths(&paths)?;
    let last_entry_id = lines.last().map(|line| line.entry_id.clone());
    crate::journal_rotation::status_from_path(
        path,
        rotation_policy,
        lines.len() as u64,
        last_entry_id,
    )
}

fn lines_from_paths(paths: &[PathBuf]) -> Result<Vec<ActivityJournalLine>, JournalError> {
    let mut lines = Vec::new();
    for path in paths {
        let reader = BufReader::new(File::open(path)?);
        for line in reader.lines() {
            let line = line?;
            if !line.is_empty() {
                lines.push(serde_json::from_str(&line)?);
            }
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
