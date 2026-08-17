use ocentra_schema::export_import_backup_recovery as contracts;

const MIGRATION_REQUEST_INVALID_NOTE: &str =
    "Migration apply requires explicit confirmation and a durable idempotency key.";
const MIGRATION_PREFLIGHT_REQUIRED_NOTE: &str =
    "Migration is manual-required until import preflight is accepted.";
const MIGRATION_EXECUTOR_UNAVAILABLE_NOTE: &str =
    "Migration executor is unavailable; local truth was not mutated.";
const MIGRATION_APPLY_FAILED_NOTE: &str =
    "Migration apply failed before a verified completion receipt.";
const MIGRATION_ROLLBACK_FAILED_NOTE: &str =
    "Migration rollback failed after state mutation; manual recovery is required.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationExecutionRequest {
    pub migration_ref: contracts::ExportImportMigrationRef,
    pub idempotency_key: String,
    pub confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationApplyReceipt {
    pub execution_ref: String,
    pub local_truth_mutated: bool,
    pub tombstones_preserved: bool,
    pub idempotent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationRollbackReceipt {
    pub rollback_ref: String,
    pub tombstones_preserved: bool,
    pub idempotent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationExecutionFailure {
    Unavailable,
    ApplyFailed { receipt: MigrationApplyReceipt },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationRollbackFailure {
    Unavailable,
}

pub trait MigrationExecutor {
    fn apply_migration(
        &mut self,
        request: &MigrationExecutionRequest,
    ) -> Result<MigrationApplyReceipt, MigrationExecutionFailure>;

    fn rollback_migration(
        &mut self,
        request: &MigrationExecutionRequest,
        apply: &MigrationApplyReceipt,
    ) -> Result<MigrationRollbackReceipt, MigrationRollbackFailure>;
}

pub fn execute_import_migration(
    preflight: &contracts::ExportImportImportPreflight,
    request: &MigrationExecutionRequest,
    executor: &mut impl MigrationExecutor,
) -> contracts::ExportImportMigrationExecutionResult {
    let migration_ref = request.migration_ref.clone();
    if preflight.migration_state == contracts::ExportImportMigrationState::NotRequired {
        return result_not_required(migration_ref, preflight.tombstones_preserved);
    }
    if !super::export_import_backup_recovery_restore::preflight_is_applicable(preflight) {
        return result_manual_required(
            migration_ref,
            preflight.tombstones_preserved,
            MIGRATION_PREFLIGHT_REQUIRED_NOTE,
        );
    }
    if !request.confirmed || request.idempotency_key.trim().is_empty() {
        return result_manual_required(
            migration_ref,
            preflight.tombstones_preserved,
            MIGRATION_REQUEST_INVALID_NOTE,
        );
    }

    let apply = match executor.apply_migration(request) {
        Ok(receipt) => receipt,
        Err(MigrationExecutionFailure::ApplyFailed { receipt }) if receipt.local_truth_mutated => {
            return rollback_failed_apply(executor, request, migration_ref, receipt);
        }
        Err(MigrationExecutionFailure::ApplyFailed { .. })
        | Err(MigrationExecutionFailure::Unavailable) => {
            return result_manual_required(
                migration_ref,
                preflight.tombstones_preserved,
                MIGRATION_EXECUTOR_UNAVAILABLE_NOTE,
            );
        }
    };

    if apply_is_not_verified(&apply) {
        return result_manual_required(
            migration_ref,
            preflight.tombstones_preserved,
            MIGRATION_APPLY_FAILED_NOTE,
        );
    }

    contracts::ExportImportMigrationExecutionResult {
        state: contracts::ExportImportMigrationExecutionState::Applied,
        migration_ref,
        execution_ref: Some(apply.execution_ref),
        rollback_ref: None,
        local_truth_mutated: true,
        tombstones_preserved: true,
        idempotent: true,
        rollback_available: true,
        manual_required_note: None,
    }
}

fn rollback_failed_apply(
    executor: &mut impl MigrationExecutor,
    request: &MigrationExecutionRequest,
    migration_ref: contracts::ExportImportMigrationRef,
    receipt: MigrationApplyReceipt,
) -> contracts::ExportImportMigrationExecutionResult {
    match executor.rollback_migration(request, &receipt) {
        Ok(rollback)
            if !rollback.rollback_ref.trim().is_empty()
                && rollback.tombstones_preserved
                && rollback.idempotent =>
        {
            contracts::ExportImportMigrationExecutionResult {
                state: contracts::ExportImportMigrationExecutionState::RolledBack,
                migration_ref,
                execution_ref: None,
                rollback_ref: Some(rollback.rollback_ref),
                local_truth_mutated: false,
                tombstones_preserved: true,
                idempotent: true,
                rollback_available: false,
                manual_required_note: None,
            }
        }
        _ => result_rollback_manual_required(
            migration_ref,
            receipt.tombstones_preserved,
            MIGRATION_ROLLBACK_FAILED_NOTE,
        ),
    }
}

fn apply_is_not_verified(receipt: &MigrationApplyReceipt) -> bool {
    receipt.execution_ref.trim().is_empty()
        || !receipt.local_truth_mutated
        || !receipt.tombstones_preserved
        || !receipt.idempotent
}

fn result_not_required(
    migration_ref: contracts::ExportImportMigrationRef,
    tombstones_preserved: bool,
) -> contracts::ExportImportMigrationExecutionResult {
    contracts::ExportImportMigrationExecutionResult {
        state: contracts::ExportImportMigrationExecutionState::NotRequired,
        migration_ref,
        execution_ref: None,
        rollback_ref: None,
        local_truth_mutated: false,
        tombstones_preserved,
        idempotent: true,
        rollback_available: false,
        manual_required_note: None,
    }
}

fn result_manual_required(
    migration_ref: contracts::ExportImportMigrationRef,
    tombstones_preserved: bool,
    note: &str,
) -> contracts::ExportImportMigrationExecutionResult {
    contracts::ExportImportMigrationExecutionResult {
        state: contracts::ExportImportMigrationExecutionState::ManualRequired,
        migration_ref,
        execution_ref: None,
        rollback_ref: None,
        local_truth_mutated: false,
        tombstones_preserved,
        idempotent: false,
        rollback_available: false,
        manual_required_note: Some(note.to_string()),
    }
}

fn result_rollback_manual_required(
    migration_ref: contracts::ExportImportMigrationRef,
    tombstones_preserved: bool,
    note: &str,
) -> contracts::ExportImportMigrationExecutionResult {
    contracts::ExportImportMigrationExecutionResult {
        state: contracts::ExportImportMigrationExecutionState::RollbackManualRequired,
        migration_ref,
        execution_ref: None,
        rollback_ref: None,
        local_truth_mutated: true,
        tombstones_preserved,
        idempotent: false,
        rollback_available: false,
        manual_required_note: Some(note.to_string()),
    }
}
