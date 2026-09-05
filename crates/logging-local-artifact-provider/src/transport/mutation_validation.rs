use super::{
    ProtocolValidationError, ProviderPayload, ProviderRelativePath, TransactionMutation,
    ValidatedMutation, MAXIMUM_REPLACE_BYTES,
};

impl TransactionMutation {
    pub(super) fn validate(&self) -> Result<ValidatedMutation, ProtocolValidationError> {
        match self {
            Self::Replace {
                relative_path,
                payload_base64,
            } => Ok(ValidatedMutation::Replace {
                relative_path: ProviderRelativePath::parse(relative_path, false)?,
                payload_base64: ProviderPayload::parse(
                    payload_base64,
                    MAXIMUM_REPLACE_BYTES,
                    true,
                )?,
            }),
            Self::Remove { relative_path } => Ok(ValidatedMutation::Remove {
                relative_path: ProviderRelativePath::parse(relative_path, false)?,
            }),
            Self::RemoveTree { relative_path } => Ok(ValidatedMutation::RemoveTree {
                relative_path: ProviderRelativePath::parse(relative_path, false)?,
            }),
        }
    }
}
