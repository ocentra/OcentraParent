use super::super::{Operation, OperationName};

pub(super) fn contains(operation: &Operation) -> bool {
    matches!(
        operation,
        Operation::BeginLease | Operation::EndLease { .. } | Operation::Recover
    )
}

pub(super) fn name(operation: &Operation) -> OperationName {
    if matches!(operation, Operation::BeginLease) {
        return OperationName::BeginLease;
    }
    if matches!(operation, Operation::EndLease { .. }) {
        return OperationName::EndLease;
    }
    OperationName::Recover
}
