#[path = "operation_validation_directory.rs"]
mod directory;
#[path = "operation_validation_lifecycle.rs"]
mod lifecycle;
#[path = "operation_validation_mutation.rs"]
mod mutation;

use super::{Operation, ProtocolValidationError, ValidatedOperation};

impl Operation {
    pub(super) fn validate(&self) -> Result<ValidatedOperation, ProtocolValidationError> {
        lifecycle::validate(self)
            .or_else(|| directory::validate(self))
            .or_else(|| mutation::validate(self))
            .ok_or(ProtocolValidationError::RelativePath)?
    }
}
