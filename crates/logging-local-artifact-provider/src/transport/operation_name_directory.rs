use super::super::{Operation, OperationName};

pub(super) fn contains(operation: &Operation) -> bool {
    matches!(
        operation,
        Operation::EnsureDirectory { .. }
            | Operation::SyncDirectory { .. }
            | Operation::Stat { .. }
            | Operation::ReadSnapshot { .. }
            | Operation::List { .. }
    )
}

pub(super) fn name(operation: &Operation) -> OperationName {
    if matches!(operation, Operation::EnsureDirectory { .. }) {
        return OperationName::EnsureDirectory;
    }
    if matches!(operation, Operation::SyncDirectory { .. }) {
        return OperationName::SyncDirectory;
    }
    if matches!(operation, Operation::Stat { .. }) {
        return OperationName::Stat;
    }
    if matches!(operation, Operation::ReadSnapshot { .. }) {
        return OperationName::ReadSnapshot;
    }
    OperationName::List
}
