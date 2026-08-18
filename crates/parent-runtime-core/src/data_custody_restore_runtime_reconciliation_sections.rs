use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan;

#[path = "data_custody_restore_runtime_reconciliation_section_partition.rs"]
mod partition;

pub(crate) fn restore_sections_match_plan(
    plan: &RestoreExecutionPlan,
    receipt: &contracts::ExportImportRestoreReceipt,
) -> bool {
    if !partition::matches_plan(plan, &receipt.applied_sections, &receipt.rejected_sections) {
        return false;
    }
    match receipt.state {
        contracts::ExportImportRestoreApplyState::Applied => {
            receipt.applied_sections == plan.accepted_sections()
                && receipt.rejected_sections == plan.rejected_sections()
        }
        contracts::ExportImportRestoreApplyState::Partial => !receipt.applied_sections.is_empty(),
        _ => true,
    }
}

pub(crate) fn migration_sections_match_plan(
    plan: &RestoreExecutionPlan,
    receipt: &contracts::ExportImportMigrationReceipt,
) -> bool {
    if !partition::matches_plan(plan, &receipt.applied_sections, &receipt.rejected_sections) {
        return false;
    }
    match receipt.outcome {
        contracts::ExportImportMigrationOutcome::Applied => {
            receipt.applied_sections == plan.accepted_sections()
                && receipt.rejected_sections == plan.rejected_sections()
        }
        contracts::ExportImportMigrationOutcome::Partial => !receipt.applied_sections.is_empty(),
        _ => true,
    }
}
