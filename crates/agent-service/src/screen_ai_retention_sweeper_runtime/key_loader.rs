use std::{fs, path::Path};

use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};

use crate::activity_capture::ActivityCaptureError;

pub(super) fn load_existing_screen_key(
    path: &Path,
) -> Result<Option<JournalKey>, ActivityCaptureError> {
    match fs::read(path) {
        Ok(bytes) => journal_key_from_bytes(&bytes).map(Some),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err(ActivityCaptureError::Io),
    }
}

fn journal_key_from_bytes(bytes: &[u8]) -> Result<JournalKey, ActivityCaptureError> {
    if bytes.len() != JOURNAL_KEY_BYTES {
        return Err(ActivityCaptureError::InvalidKeyLength);
    }
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(bytes);
    Ok(JournalKey::from_bytes(key))
}
