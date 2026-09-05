use super::super::{
    Operation, ProtocolValidationError, ProviderPayload, ProviderRelativePath, TransactionMutation,
    ValidatedOperation, MAXIMUM_APPEND_BYTES, MAXIMUM_REPLACE_BYTES as TRANSACTION_REPLACE_BYTES,
};

pub(super) fn validate(
    operation: &Operation,
) -> Option<Result<ValidatedOperation, ProtocolValidationError>> {
    if let Some(result) = validate_append(operation) {
        return Some(result);
    }
    if let Some(result) = validate_replace(operation) {
        return Some(result);
    }
    if let Some(result) = validate_remove(operation) {
        return Some(result);
    }
    if let Some(result) = validate_remove_tree(operation) {
        return Some(result);
    }
    if let Some(result) = validate_transaction(operation) {
        return Some(result);
    }
    if let Operation::Shutdown = operation {
        return Some(Ok(ValidatedOperation::Shutdown));
    }
    None
}

fn validate_append(
    operation: &Operation,
) -> Option<Result<ValidatedOperation, ProtocolValidationError>> {
    if let Operation::Append {
        relative_path,
        payload_base64,
    } = operation
    {
        return Some(
            ProviderRelativePath::parse(relative_path, false).and_then(|relative_path| {
                ProviderPayload::parse(payload_base64, MAXIMUM_APPEND_BYTES, false).map(
                    |payload_base64| ValidatedOperation::Append {
                        relative_path,
                        payload_base64,
                    },
                )
            }),
        );
    }
    None
}

fn validate_replace(
    operation: &Operation,
) -> Option<Result<ValidatedOperation, ProtocolValidationError>> {
    if let Operation::Replace {
        relative_path,
        payload_base64,
    } = operation
    {
        return Some(
            ProviderRelativePath::parse(relative_path, false).and_then(|relative_path| {
                ProviderPayload::parse(payload_base64, TRANSACTION_REPLACE_BYTES, true).map(
                    |payload_base64| ValidatedOperation::Replace {
                        relative_path,
                        payload_base64,
                    },
                )
            }),
        );
    }
    None
}

fn validate_remove(
    operation: &Operation,
) -> Option<Result<ValidatedOperation, ProtocolValidationError>> {
    if let Operation::Remove { relative_path } = operation {
        return Some(
            ProviderRelativePath::parse(relative_path, false)
                .map(|relative_path| ValidatedOperation::Remove { relative_path }),
        );
    }
    None
}

fn validate_remove_tree(
    operation: &Operation,
) -> Option<Result<ValidatedOperation, ProtocolValidationError>> {
    if let Operation::RemoveTree { relative_path } = operation {
        return Some(
            ProviderRelativePath::parse(relative_path, false)
                .map(|relative_path| ValidatedOperation::RemoveTree { relative_path }),
        );
    }
    None
}

fn validate_transaction(
    operation: &Operation,
) -> Option<Result<ValidatedOperation, ProtocolValidationError>> {
    if let Operation::ApplyTransaction { mutations } = operation {
        if mutations.is_empty() || mutations.len() > super::super::MAXIMUM_TRANSACTION_MUTATIONS {
            return Some(Err(ProtocolValidationError::MutationCount));
        }
        let validated = mutations
            .iter()
            .map(TransactionMutation::validate)
            .collect::<Result<Vec<_>, _>>();
        return Some(validated.map(|mutations| ValidatedOperation::ApplyTransaction { mutations }));
    }
    None
}
