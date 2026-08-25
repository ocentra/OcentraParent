use super::data_custody_restore_runtime_ledger::RestoreDispatchStage;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;

pub(crate) fn stage_for_kind(kind: &DataCustodyRuntimeEventKind) -> RestoreDispatchStage {
    use DataCustodyRuntimeEventKind::*;
    match kind {
        RestorePlanned | MigrationPlanned => RestoreDispatchStage::Planned,
        RestoreBeforeDispatch | MigrationBeforeDispatch => RestoreDispatchStage::BeforeDispatch,
        RestoreApplied | MigrationReceipt => RestoreDispatchStage::Terminal,
        RollbackBeforeDispatch => RestoreDispatchStage::RollbackBeforeDispatch,
        Rollback => RestoreDispatchStage::Rollback,
        Reconciliation => RestoreDispatchStage::Reconciled,
        BackupScheduled | BackupJobTransition => RestoreDispatchStage::Terminal,
    }
}
