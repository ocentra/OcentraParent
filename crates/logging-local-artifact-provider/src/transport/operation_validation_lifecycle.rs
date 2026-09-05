use super::super::{
    Operation, ProtocolValidationError, ProviderIdentifier, ValidatedOperation,
    MAXIMUM_LEASE_ID_BYTES,
};

pub(super) fn validate(
    operation: &Operation,
) -> Option<Result<ValidatedOperation, ProtocolValidationError>> {
    if let Operation::BeginLease = operation {
        return Some(Ok(ValidatedOperation::BeginLease));
    }
    if let Operation::EndLease { lease_id } = operation {
        return Some(
            ProviderIdentifier::parse(lease_id, MAXIMUM_LEASE_ID_BYTES)
                .map(|lease_id| ValidatedOperation::EndLease { lease_id }),
        );
    }
    if let Operation::Recover = operation {
        return Some(Ok(ValidatedOperation::Recover));
    }
    None
}
