use super::super::data_custody_restore_runtime_ledger_event_stage::stage_for_kind;
use super::super::data_custody_runtime_eventing::{
    DataCustodyRuntimeEvent, DataCustodyRuntimeRecord,
};
use super::{RestoreLedger, RestoreLedgerError};

pub(super) fn apply_event(
    ledger: &mut RestoreLedger,
    event: &DataCustodyRuntimeEvent,
) -> Result<(), RestoreLedgerError> {
    match &event.record {
        DataCustodyRuntimeRecord::MigrationReceipt(receipt) => {
            ledger.insert_migration_receipt(receipt.clone())?;
            if let Some(stage) = stage_for_kind(&event.kind) {
                ledger
                    .migration_stages
                    .insert(receipt.operation_ref.as_str().to_owned(), stage);
            }
        }
        DataCustodyRuntimeRecord::RestoreReceipt(receipt) => {
            ledger.insert_restore_receipt(receipt.clone())?;
            if let Some(stage) = stage_for_kind(&event.kind) {
                ledger
                    .restore_stages
                    .insert(receipt.operation_ref.as_str().to_owned(), stage);
            }
        }
        DataCustodyRuntimeRecord::Schedule(_)
        | DataCustodyRuntimeRecord::ScheduleAndJob { .. }
        | DataCustodyRuntimeRecord::BackupJob(_) => {}
    }
    Ok(())
}
