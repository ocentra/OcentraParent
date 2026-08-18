use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_restore_runtime_ledger::RestoreLedgerError;

pub(crate) fn validate_restore_receipt(
    previous: &contracts::ExportImportRestoreReceipt,
    incoming: &contracts::ExportImportRestoreReceipt,
) -> Result<(), RestoreLedgerError> {
    if previous == incoming {
        return Ok(());
    }
    if restore_identity_changed(previous, incoming)
        || !restore_transition_allowed(previous.state, incoming.state)
    {
        return Err(RestoreLedgerError::IdentityMismatch);
    }
    if matches!(
        previous.state,
        contracts::ExportImportRestoreApplyState::Applied
            | contracts::ExportImportRestoreApplyState::Partial
    ) && !incoming
        .applied_sections
        .iter()
        .all(|section| previous.applied_sections.contains(section))
    {
        return Err(RestoreLedgerError::InvalidRestoreTransition);
    }
    Ok(())
}

pub(crate) fn validate_migration_receipt(
    previous: &contracts::ExportImportMigrationReceipt,
    incoming: &contracts::ExportImportMigrationReceipt,
) -> Result<(), RestoreLedgerError> {
    if previous == incoming {
        return Ok(());
    }
    if migration_identity_changed(previous, incoming)
        || !migration_transition_allowed(previous.outcome, incoming.outcome)
    {
        return Err(RestoreLedgerError::IdentityMismatch);
    }
    if matches!(
        previous.outcome,
        contracts::ExportImportMigrationOutcome::Applied
            | contracts::ExportImportMigrationOutcome::Partial
    ) && !incoming
        .applied_sections
        .iter()
        .all(|section| previous.applied_sections.contains(section))
    {
        return Err(RestoreLedgerError::InvalidMigrationTransition);
    }
    Ok(())
}

fn restore_identity_changed(
    previous: &contracts::ExportImportRestoreReceipt,
    incoming: &contracts::ExportImportRestoreReceipt,
) -> bool {
    previous.bundle_id != incoming.bundle_id
        || previous.restore_plan_ref != incoming.restore_plan_ref
        || previous.operation_ref != incoming.operation_ref
        || previous.execution_ref != incoming.execution_ref
        || previous.tombstones_preserved != incoming.tombstones_preserved
        || previous.no_resurrection != incoming.no_resurrection
        || (previous.provider_operation_ref.is_some()
            && previous.provider_operation_ref != incoming.provider_operation_ref)
        || (previous.rollback_provider_operation_ref.is_some()
            && previous.rollback_provider_operation_ref != incoming.rollback_provider_operation_ref)
}

fn migration_identity_changed(
    previous: &contracts::ExportImportMigrationReceipt,
    incoming: &contracts::ExportImportMigrationReceipt,
) -> bool {
    previous.bundle_id != incoming.bundle_id
        || previous.migration_plan_ref != incoming.migration_plan_ref
        || previous.migration_ref != incoming.migration_ref
        || previous.operation_ref != incoming.operation_ref
        || previous.execution_ref != incoming.execution_ref
        || previous.tombstones_preserved != incoming.tombstones_preserved
        || previous.no_resurrection != incoming.no_resurrection
        || (previous.provider_operation_ref.is_some()
            && previous.provider_operation_ref != incoming.provider_operation_ref)
        || (previous.rollback_provider_operation_ref.is_some()
            && previous.rollback_provider_operation_ref != incoming.rollback_provider_operation_ref)
}

fn restore_transition_allowed(
    from: contracts::ExportImportRestoreApplyState,
    to: contracts::ExportImportRestoreApplyState,
) -> bool {
    use contracts::ExportImportRestoreApplyState::*;
    matches!(
        (from, to),
        (
            NotApplied,
            ApplyPending | Blocked | WrongHousehold | WrongKey | Corrupt
        ) | (
            ApplyPending,
            ApplyPending | Applied | Partial | Blocked | Corrupt
        ) | (Applied, Applied | Partial)
            | (Partial, Partial)
            | (WrongHousehold, WrongHousehold)
            | (WrongKey, WrongKey)
            | (Corrupt, Corrupt)
            | (Blocked, Blocked)
    )
}

fn migration_transition_allowed(
    from: contracts::ExportImportMigrationOutcome,
    to: contracts::ExportImportMigrationOutcome,
) -> bool {
    use contracts::ExportImportMigrationOutcome::*;
    matches!(
        (from, to),
        (
            Planned,
            Planned | Applied | Partial | RolledBack | Reconciled | Failed | ManualRequired
        ) | (
            Applied,
            Applied | Partial | RolledBack | Reconciled | Failed | ManualRequired
        ) | (
            Partial,
            Partial | RolledBack | Reconciled | Failed | ManualRequired
        ) | (Failed, Failed | Reconciled | ManualRequired)
            | (ManualRequired, ManualRequired | Reconciled)
            | (Reconciled, Reconciled)
            | (RolledBack, RolledBack)
    )
}
