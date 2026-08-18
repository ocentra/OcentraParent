use super::data_custody_restore_runtime_receipts::migration_receipt_from_dispatch;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan,
};

use super::data_custody_restore_runtime::RestoreRuntimeError;
use super::data_custody_restore_runtime_reconciliation_sections::{
    migration_sections_match_plan, restore_sections_match_plan,
};

pub(crate) fn restore_receipt_matches_plan(
    plan: &RestoreExecutionPlan,
    receipt: &contracts::ExportImportRestoreReceipt,
) -> bool {
    receipt.bundle_id == *plan.bundle_id()
        && receipt.restore_plan_ref == *plan.plan_ref()
        && receipt.operation_ref == *plan.operation_ref()
        && receipt.execution_ref == *plan.execution_ref()
        && restore_sections_match_plan(plan, receipt)
        && receipt.tombstones_preserved == plan.tombstones_preserved()
        && receipt.no_resurrection == plan.no_resurrection()
}

pub(crate) fn migration_receipt_matches_plan(
    plan: &RestoreExecutionPlan,
    receipt: &contracts::ExportImportMigrationReceipt,
) -> bool {
    receipt.bundle_id == *plan.bundle_id()
        && receipt.migration_plan_ref == *plan.plan_ref()
        && receipt.migration_ref.as_ref() == plan.migration_ref()
        && receipt.operation_ref == *plan.operation_ref()
        && receipt.execution_ref == *plan.execution_ref()
        && migration_sections_match_plan(plan, receipt)
        && receipt.tombstones_preserved == plan.tombstones_preserved()
        && receipt.no_resurrection == plan.no_resurrection()
}

pub(crate) fn plan_migration_manual_required(
    plan: &RestoreExecutionPlan,
    planned: &contracts::ExportImportMigrationReceipt,
    recorded_at: contracts::ExportImportTimestamp,
) -> Result<contracts::ExportImportMigrationReceipt, RestoreRuntimeError> {
    migration_receipt_from_dispatch(
        plan,
        contracts::ExportImportMigrationOutcome::ManualRequired,
        planned.applied_sections.clone(),
        planned.rejected_sections.clone(),
        PartialWriteCompensation::NotRequired,
        planned.provider_operation_ref.as_ref(),
        planned.rollback_provider_operation_ref.as_ref(),
        recorded_at,
        Some(
            "Migration was pending before restart; provider status reconciliation is required."
                .to_owned(),
        ),
    )
    .map_err(RestoreRuntimeError::Migration)
}
