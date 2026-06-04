use std::{
    collections::HashSet,
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::{constants, ScreenAnalysisQueueJob};
use serde_json::{json, Value};

use crate::{
    journal_crypto::{decrypt_payload, encrypt_payload, JournalKey},
    JournalError,
};

pub struct ScreenEvidenceQueue {
    path: PathBuf,
    key: JournalKey,
}

impl ScreenEvidenceQueue {
    pub fn open(directory: impl AsRef<Path>, key: JournalKey) -> Result<Self, JournalError> {
        create_dir_all(directory.as_ref())?;
        let path = directory
            .as_ref()
            .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
        OpenOptions::new().create(true).append(true).open(&path)?;
        Ok(Self { path, key })
    }

    pub fn append_encrypted_image(
        &self,
        job: &ScreenAnalysisQueueJob,
        image_bytes: &[u8],
    ) -> Result<(), JournalError> {
        let encrypted = encrypt_payload(&self.key, image_bytes)?;
        let record = json!({
            constants::field::SCHEMA_VERSION: job.schema_version,
            constants::field::SCREEN_QUEUE_JOB_ID: job.queue_job_id,
            constants::field::SCREEN_CUSTODY_STATE: job.custody_state,
            constants::field::SCREEN_IMAGE_DIGEST: encrypted.digest,
            constants::field::NONCE: encrypted.nonce,
            constants::field::CIPHERTEXT: encrypted.ciphertext,
        });
        let mut file = OpenOptions::new().append(true).open(&self.path)?;
        serde_json::to_writer(&mut file, &record)?;
        file.write_all(&[constants::byte::NEWLINE])?;
        file.sync_data()?;
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read_decrypted_entries(
        &self,
        max_entries: usize,
    ) -> Result<Vec<(u16, String, String, String, Vec<u8>)>, JournalError> {
        let contents = match read_to_string(&self.path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(error.into()),
        };

        let mut entries = Vec::new();
        for line in contents.lines().filter(|line| !line.trim().is_empty()) {
            if entries.len() >= max_entries {
                break;
            }
            let record = EncryptedScreenEvidenceQueueRecord::from_line(line)?;
            let image_bytes = decrypt_payload(&self.key, &record.nonce, &record.ciphertext)?;
            entries.push((
                record.schema_version,
                record.queue_job_id,
                record.custody_state,
                record.image_digest,
                image_bytes,
            ));
        }
        Ok(entries)
    }

    pub fn remove_entries(&self, queue_job_ids: &[String]) -> Result<u64, JournalError> {
        let contents = match read_to_string(&self.path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
            Err(error) => return Err(error.into()),
        };
        let ids = queue_job_ids.iter().collect::<HashSet<_>>();
        let mut retained = Vec::new();
        let mut removed_count = 0;
        for line in contents.lines().filter(|line| !line.trim().is_empty()) {
            let record = EncryptedScreenEvidenceQueueRecord::from_line(line)?;
            if ids.contains(&record.queue_job_id) {
                removed_count += 1;
            } else {
                retained.push(line);
            }
        }
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        for line in retained {
            file.write_all(line.as_bytes())?;
            file.write_all(&[constants::byte::NEWLINE])?;
        }
        file.sync_data()?;
        Ok(removed_count)
    }
}

struct EncryptedScreenEvidenceQueueRecord {
    schema_version: u16,
    queue_job_id: String,
    custody_state: String,
    image_digest: String,
    nonce: String,
    ciphertext: String,
}

impl EncryptedScreenEvidenceQueueRecord {
    fn from_line(line: &str) -> Result<Self, JournalError> {
        let value: Value = serde_json::from_str(line)?;
        Ok(Self {
            schema_version: required_u16(&value, constants::field::SCHEMA_VERSION)?,
            queue_job_id: required_string(&value, constants::field::SCREEN_QUEUE_JOB_ID)?,
            custody_state: required_string(&value, constants::field::SCREEN_CUSTODY_STATE)?,
            image_digest: required_string(&value, constants::field::SCREEN_IMAGE_DIGEST)?,
            nonce: required_string(&value, constants::field::NONCE)?,
            ciphertext: required_string(&value, constants::field::CIPHERTEXT)?,
        })
    }
}

fn required_string(value: &Value, key: &str) -> Result<String, JournalError> {
    Ok(serde_json::from_value(
        value.get(key).cloned().unwrap_or(Value::Null),
    )?)
}

fn required_u16(value: &Value, key: &str) -> Result<u16, JournalError> {
    Ok(serde_json::from_value(
        value.get(key).cloned().unwrap_or(Value::Null),
    )?)
}
