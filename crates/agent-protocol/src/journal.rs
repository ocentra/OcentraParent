use serde::{Deserialize, Serialize};

pub const ACTIVITY_JOURNAL_SCHEMA_VERSION: u16 = crate::ACTIVITY_JOURNAL_SCHEMA_VERSION;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityJournalCipher {
    #[serde(rename = "xchacha20poly1305")]
    XChaCha20Poly1305,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityJournalLine {
    pub schema_version: u16,
    pub entry_id: String,
    pub segment_id: String,
    pub written_at: String,
    pub event_id: String,
    pub cipher: ActivityJournalCipher,
    pub nonce: String,
    pub ciphertext: String,
    pub activity_digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityJournalStatus {
    pub schema_version: u16,
    pub encrypted: bool,
    pub entries_written: u64,
    pub bytes_written: u64,
    pub active_segment_id: String,
    pub segment_count: u64,
    pub rotation_max_bytes: u64,
    pub last_entry_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityJournalRotationPolicy {
    pub max_segment_bytes: u64,
}
