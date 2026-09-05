use super::super::{
    Operation, ProtocolValidationError, ProviderRelativePath, ReadMaximum, ValidatedOperation,
};

pub(super) fn validate(
    operation: &Operation,
) -> Option<Result<ValidatedOperation, ProtocolValidationError>> {
    if let Operation::EnsureDirectory { relative_path } = operation {
        return Some(
            ProviderRelativePath::parse(relative_path, true)
                .map(|relative_path| ValidatedOperation::EnsureDirectory { relative_path }),
        );
    }
    if let Operation::SyncDirectory { relative_path } = operation {
        return Some(
            ProviderRelativePath::parse(relative_path, true)
                .map(|relative_path| ValidatedOperation::SyncDirectory { relative_path }),
        );
    }
    if let Operation::Stat { relative_path } = operation {
        return Some(
            ProviderRelativePath::parse(relative_path, true)
                .map(|relative_path| ValidatedOperation::Stat { relative_path }),
        );
    }
    if let Operation::ReadSnapshot {
        relative_path,
        maximum_bytes,
    } = operation
    {
        return Some(validate_snapshot(relative_path, *maximum_bytes));
    }
    if let Operation::List { relative_path } = operation {
        return Some(
            ProviderRelativePath::parse(relative_path, true)
                .map(|relative_path| ValidatedOperation::List { relative_path }),
        );
    }
    None
}

fn validate_snapshot(
    relative_path: &str,
    maximum_bytes: u64,
) -> Result<ValidatedOperation, ProtocolValidationError> {
    let relative_path = ProviderRelativePath::parse(relative_path, false)?;
    let maximum_bytes = ReadMaximum::parse(maximum_bytes)?;
    Ok(ValidatedOperation::ReadSnapshot {
        relative_path,
        maximum_bytes,
    })
}
