use std::{
    collections::HashSet,
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
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

pub struct DecryptedScreenEvidenceQueueEntry {
    pub schema_version: u16,
    pub queue_job_id: String,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
    pub status: String,
    pub deletion_required: bool,
    pub deletion_status: String,
    pub deletion_proof_ref: Option<String>,
    pub custody_state: String,
    pub image_digest: String,
    pub image_bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenEvidenceExpiredQueueEntry {
    pub queue_job_id: String,
    pub image_digest: String,
    pub expires_at: String,
    pub deletion_proof_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenEvidenceQueueSweep {
    pub expired_entries: Vec<ScreenEvidenceExpiredQueueEntry>,
    pub retained_count: u64,
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
            constants::field::CREATED_AT: job.created_at,
            constants::field::EXPIRES_AT: job.expires_at,
            constants::field::STATUS: job.status,
            constants::field::SCREEN_DELETION_REQUIRED: job.deletion_required,
            constants::field::SCREEN_DELETION_STATUS: job.deletion_status,
            constants::field::SCREEN_DELETION_PROOF_REF: job.deletion_proof_ref,
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
    ) -> Result<Vec<DecryptedScreenEvidenceQueueEntry>, JournalError> {
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
            entries.push(DecryptedScreenEvidenceQueueEntry {
                schema_version: record.schema_version,
                queue_job_id: record.queue_job_id,
                created_at: record.created_at,
                expires_at: record.expires_at,
                status: record.status,
                deletion_required: record.deletion_required,
                deletion_status: record.deletion_status,
                deletion_proof_ref: record.deletion_proof_ref,
                custody_state: record.custody_state,
                image_digest: record.image_digest,
                image_bytes,
            });
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

    pub fn remove_expired_entries(
        &self,
        now: &str,
        deletion_proof_prefix: &str,
    ) -> Result<ScreenEvidenceQueueSweep, JournalError> {
        let contents = match read_to_string(&self.path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(ScreenEvidenceQueueSweep {
                    expired_entries: Vec::new(),
                    retained_count: 0,
                });
            }
            Err(error) => return Err(error.into()),
        };
        let mut retained = Vec::new();
        let mut expired_entries = Vec::new();
        for line in contents.lines().filter(|line| !line.trim().is_empty()) {
            let record = EncryptedScreenEvidenceQueueRecord::from_line(line)?;
            if queue_record_expired(record.expires_at.as_deref(), now) {
                expired_entries.push(ScreenEvidenceExpiredQueueEntry {
                    queue_job_id: record.queue_job_id.clone(),
                    image_digest: record.image_digest,
                    expires_at: record.expires_at.unwrap_or_default(),
                    deletion_proof_ref: prefixed_ref(deletion_proof_prefix, &record.queue_job_id),
                });
            } else {
                retained.push(line);
            }
        }
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        for line in &retained {
            file.write_all(line.as_bytes())?;
            file.write_all(&[constants::byte::NEWLINE])?;
        }
        file.sync_data()?;
        Ok(ScreenEvidenceQueueSweep {
            expired_entries,
            retained_count: retained.len() as u64,
        })
    }
}

struct EncryptedScreenEvidenceQueueRecord {
    schema_version: u16,
    queue_job_id: String,
    created_at: Option<String>,
    expires_at: Option<String>,
    status: String,
    deletion_required: bool,
    deletion_status: String,
    deletion_proof_ref: Option<String>,
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
            created_at: optional_string(&value, constants::field::CREATED_AT)?,
            expires_at: optional_string(&value, constants::field::EXPIRES_AT)?,
            status: optional_string(&value, constants::field::STATUS)?.unwrap_or_else(|| {
                ocentra_parent_agent_protocol::SCREEN_QUEUE_STATUS_QUEUED.to_string()
            }),
            deletion_required: optional_bool(&value, constants::field::SCREEN_DELETION_REQUIRED)?
                .unwrap_or(true),
            deletion_status: optional_string(&value, constants::field::SCREEN_DELETION_STATUS)?
                .unwrap_or_else(|| {
                    ocentra_parent_agent_protocol::SCREEN_DELETION_REQUIRED.to_string()
                }),
            deletion_proof_ref: optional_string(
                &value,
                constants::field::SCREEN_DELETION_PROOF_REF,
            )?,
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

fn optional_string(value: &Value, key: &str) -> Result<Option<String>, JournalError> {
    Ok(serde_json::from_value(
        value.get(key).cloned().unwrap_or(Value::Null),
    )?)
}

fn optional_bool(value: &Value, key: &str) -> Result<Option<bool>, JournalError> {
    Ok(serde_json::from_value(
        value.get(key).cloned().unwrap_or(Value::Null),
    )?)
}

fn required_u16(value: &Value, key: &str) -> Result<u16, JournalError> {
    Ok(serde_json::from_value(
        value.get(key).cloned().unwrap_or(Value::Null),
    )?)
}

fn queue_record_expired(expires_at: Option<&str>, now: &str) -> bool {
    let Some(expires_at) = expires_at else {
        return false;
    };
    match (parse_timestamp(expires_at), parse_timestamp(now)) {
        (Some(expires_at), Some(now)) => expires_at <= now,
        _ => false,
    }
}

fn parse_timestamp(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}

fn prefixed_ref(prefix: &str, value: &str) -> String {
    let mut reference = String::from(prefix);
    reference.push_str(value);
    reference
}
