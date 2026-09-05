use crate::error::ArtifactError;
use crate::owner_types::MAX_REQUEST_ID_BYTES;

use super::super::JournalText;

pub(crate) trait RequestIdInput {
    fn is_valid_request_id(&self) -> bool;
}

impl RequestIdInput for str {
    fn is_valid_request_id(&self) -> bool {
        !self.is_empty()
            && self.len() <= MAX_REQUEST_ID_BYTES
            && self
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    }
}

impl RequestIdInput for JournalText<'_> {
    fn is_valid_request_id(&self) -> bool {
        self.as_str().is_valid_request_id()
    }
}

impl RequestIdInput for String {
    fn is_valid_request_id(&self) -> bool {
        self.as_str().is_valid_request_id()
    }
}

pub(in crate::owner_journal) fn validate_request_id<R>(request_id: &R) -> Result<(), ArtifactError>
where
    R: RequestIdInput + ?Sized,
{
    if request_id.is_valid_request_id() {
        Ok(())
    } else {
        Err(ArtifactError::InvalidRequestId)
    }
}
