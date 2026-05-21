use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::{constants, ScreenAnalysisQueueJob};
use serde_json::json;

use crate::{
    journal_crypto::{encrypt_payload, JournalKey},
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
}
