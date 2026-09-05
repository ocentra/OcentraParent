use super::super::{Operation, OperationName};

pub(super) fn name(operation: &Operation) -> OperationName {
    if matches!(operation, Operation::Append { .. }) {
        return OperationName::Append;
    }
    if matches!(operation, Operation::Replace { .. }) {
        return OperationName::Replace;
    }
    if matches!(operation, Operation::Remove { .. }) {
        return OperationName::Remove;
    }
    if matches!(operation, Operation::RemoveTree { .. }) {
        return OperationName::RemoveTree;
    }
    if matches!(operation, Operation::ApplyTransaction { .. }) {
        return OperationName::ApplyTransaction;
    }
    OperationName::Shutdown
}
