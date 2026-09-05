use super::*;

pub(super) fn receipt(record: ReceiptRecord) -> MutationReceipt {
    let outcome = match record.outcome {
        ReceiptRecordOutcome::Appended { offset, length } => {
            ReceiptOutcome::Appended { offset, length }
        }
        ReceiptRecordOutcome::Replaced => ReceiptOutcome::Replaced,
        ReceiptRecordOutcome::Removed { existed } => ReceiptOutcome::Removed { existed },
        ReceiptRecordOutcome::TransactionCommitted { count } => {
            ReceiptOutcome::TransactionCommitted { count }
        }
        ReceiptRecordOutcome::Unsupported { operation } => {
            ReceiptOutcome::Unsupported { operation }
        }
    };
    MutationReceipt::new(
        record.request_id,
        record.operation,
        record.relative_path,
        outcome,
        true,
    )
}
