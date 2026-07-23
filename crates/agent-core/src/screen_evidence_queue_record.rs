use chrono::{DateTime, Utc};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenAnalysisQueueJob, SCREEN_DELETION_REQUIRED, SCREEN_QUEUE_STATUS_QUEUED,
};
use serde_json::{json, Value};

use crate::JournalError;

#[derive(Clone, Debug)]
pub(crate) struct EncryptedScreenEvidenceQueueRecord {
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
    pub nonce: String,
    pub ciphertext: String,
}

pub(crate) fn encrypted_record_from_job(
    job: &ScreenAnalysisQueueJob,
    encrypted: impl Borrow<crate::journal_crypto::EncryptedPayload>,
) -> Value {
    let encrypted = encrypted.borrow();
    json!({
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
    })
}

pub(crate) fn decrypted_record_from_line(
    line: &str,
) -> Result<EncryptedScreenEvidenceQueueRecord, JournalError> {
    let value: Value = serde_json::from_str(line)?;
    Ok(EncryptedScreenEvidenceQueueRecord {
        schema_version: required_u16(&value, constants::field::SCHEMA_VERSION)?,
        queue_job_id: required_string(&value, constants::field::SCREEN_QUEUE_JOB_ID)?,
        created_at: optional_string(&value, constants::field::CREATED_AT)?,
        expires_at: optional_string(&value, constants::field::EXPIRES_AT)?,
        status: optional_string(&value, constants::field::STATUS)?
            .unwrap_or_else(|| SCREEN_QUEUE_STATUS_QUEUED.to_string()),
        deletion_required: optional_bool(&value, constants::field::SCREEN_DELETION_REQUIRED)?
            .unwrap_or(true),
        deletion_status: optional_string(&value, constants::field::SCREEN_DELETION_STATUS)?
            .unwrap_or_else(|| SCREEN_DELETION_REQUIRED.to_string()),
        deletion_proof_ref: optional_string(&value, constants::field::SCREEN_DELETION_PROOF_REF)?,
        custody_state: required_string(&value, constants::field::SCREEN_CUSTODY_STATE)?,
        image_digest: required_string(&value, constants::field::SCREEN_IMAGE_DIGEST)?,
        nonce: required_string(&value, constants::field::NONCE)?,
        ciphertext: required_string(&value, constants::field::CIPHERTEXT)?,
    })
}

pub(crate) fn queue_record_expired(expires_at: Option<&str>, now: &str) -> bool {
    let Some(expires_at) = expires_at else {
        return true;
    };
    match (parse_timestamp(expires_at), parse_timestamp(now)) {
        (Some(expires_at), Some(now)) => expires_at <= now,
        _ => true,
    }
}

pub(crate) fn prefixed_ref(prefix: &str, value: &str) -> String {
    let mut reference = String::from(prefix);
    reference.push_str(value);
    reference
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

pub(crate) fn timestamp_is_after(value: &str, reference: &str) -> bool {
    match (parse_timestamp(value), parse_timestamp(reference)) {
        (Some(value), Some(reference)) => value > reference,
        _ => false,
    }
}

fn parse_timestamp(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}
use std::borrow::Borrow;
