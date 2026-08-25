use super::data_custody_runtime_eventing::{DataCustodyRuntimeEventKind, DataCustodyRuntimeRecord};

pub(crate) fn kind_matches_record(
    kind: &DataCustodyRuntimeEventKind,
    record: &DataCustodyRuntimeRecord,
) -> bool {
    matches!(
        (kind, record),
        (
            DataCustodyRuntimeEventKind::BackupScheduled,
            DataCustodyRuntimeRecord::Schedule(_) | DataCustodyRuntimeRecord::ScheduleAndJob { .. }
        ) | (
            DataCustodyRuntimeEventKind::BackupJobTransition
                | DataCustodyRuntimeEventKind::Reconciliation,
            DataCustodyRuntimeRecord::BackupJob(_)
        ) | (
            DataCustodyRuntimeEventKind::RestorePlanned,
            DataCustodyRuntimeRecord::RestoreReceipt(_)
        ) | (
            DataCustodyRuntimeEventKind::RestoreBeforeDispatch,
            DataCustodyRuntimeRecord::RestoreReceipt(_)
        ) | (
            DataCustodyRuntimeEventKind::RestoreApplied,
            DataCustodyRuntimeRecord::RestoreReceipt(_)
        ) | (
            DataCustodyRuntimeEventKind::MigrationPlanned
                | DataCustodyRuntimeEventKind::MigrationBeforeDispatch
                | DataCustodyRuntimeEventKind::MigrationReceipt,
            DataCustodyRuntimeRecord::MigrationReceipt(_)
        ) | (
            DataCustodyRuntimeEventKind::Rollback,
            DataCustodyRuntimeRecord::RestoreReceipt(_)
                | DataCustodyRuntimeRecord::MigrationReceipt(_)
        ) | (
            DataCustodyRuntimeEventKind::RollbackBeforeDispatch,
            DataCustodyRuntimeRecord::RestoreReceipt(_)
                | DataCustodyRuntimeRecord::MigrationReceipt(_)
        ) | (
            DataCustodyRuntimeEventKind::Reconciliation,
            DataCustodyRuntimeRecord::RestoreReceipt(_)
                | DataCustodyRuntimeRecord::MigrationReceipt(_)
        )
    )
}
