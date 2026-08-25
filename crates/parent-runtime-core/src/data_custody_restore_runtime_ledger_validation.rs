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
    matches!(
        (from, to),
        (
            contracts::ExportImportRestoreApplyState::NotApplied,
            contracts::ExportImportRestoreApplyState::ApplyPending
                | contracts::ExportImportRestoreApplyState::Blocked
                | contracts::ExportImportRestoreApplyState::WrongHousehold
                | contracts::ExportImportRestoreApplyState::WrongKey
                | contracts::ExportImportRestoreApplyState::Corrupt
        ) | (
            contracts::ExportImportRestoreApplyState::ApplyPending,
            contracts::ExportImportRestoreApplyState::ApplyPending
                | contracts::ExportImportRestoreApplyState::Applied
                | contracts::ExportImportRestoreApplyState::Partial
                | contracts::ExportImportRestoreApplyState::Blocked
                | contracts::ExportImportRestoreApplyState::Corrupt
        ) | (
            contracts::ExportImportRestoreApplyState::Applied,
            contracts::ExportImportRestoreApplyState::Applied
                | contracts::ExportImportRestoreApplyState::Partial
        ) | (
            contracts::ExportImportRestoreApplyState::Partial,
            contracts::ExportImportRestoreApplyState::Partial
        ) | (
            contracts::ExportImportRestoreApplyState::WrongHousehold,
            contracts::ExportImportRestoreApplyState::WrongHousehold
        ) | (
            contracts::ExportImportRestoreApplyState::WrongKey,
            contracts::ExportImportRestoreApplyState::WrongKey
        ) | (
            contracts::ExportImportRestoreApplyState::Corrupt,
            contracts::ExportImportRestoreApplyState::Corrupt
        ) | (
            contracts::ExportImportRestoreApplyState::Blocked,
            contracts::ExportImportRestoreApplyState::Blocked
        )
    )
}

fn migration_transition_allowed(
    from: contracts::ExportImportMigrationOutcome,
    to: contracts::ExportImportMigrationOutcome,
) -> bool {
    matches!(
        (from, to),
        (
            contracts::ExportImportMigrationOutcome::Planned,
            contracts::ExportImportMigrationOutcome::Planned
                | contracts::ExportImportMigrationOutcome::Applied
                | contracts::ExportImportMigrationOutcome::Partial
                | contracts::ExportImportMigrationOutcome::RolledBack
                | contracts::ExportImportMigrationOutcome::Reconciled
                | contracts::ExportImportMigrationOutcome::Failed
                | contracts::ExportImportMigrationOutcome::ManualRequired
        ) | (
            contracts::ExportImportMigrationOutcome::Applied,
            contracts::ExportImportMigrationOutcome::Applied
                | contracts::ExportImportMigrationOutcome::Partial
                | contracts::ExportImportMigrationOutcome::RolledBack
                | contracts::ExportImportMigrationOutcome::Reconciled
                | contracts::ExportImportMigrationOutcome::Failed
                | contracts::ExportImportMigrationOutcome::ManualRequired
        ) | (
            contracts::ExportImportMigrationOutcome::Partial,
            contracts::ExportImportMigrationOutcome::Partial
                | contracts::ExportImportMigrationOutcome::RolledBack
                | contracts::ExportImportMigrationOutcome::Reconciled
                | contracts::ExportImportMigrationOutcome::Failed
                | contracts::ExportImportMigrationOutcome::ManualRequired
        ) | (
            contracts::ExportImportMigrationOutcome::Failed,
            contracts::ExportImportMigrationOutcome::Failed
                | contracts::ExportImportMigrationOutcome::Reconciled
                | contracts::ExportImportMigrationOutcome::ManualRequired
        ) | (
            contracts::ExportImportMigrationOutcome::ManualRequired,
            contracts::ExportImportMigrationOutcome::ManualRequired
                | contracts::ExportImportMigrationOutcome::Reconciled
        ) | (
            contracts::ExportImportMigrationOutcome::Reconciled,
            contracts::ExportImportMigrationOutcome::Reconciled
        ) | (
            contracts::ExportImportMigrationOutcome::RolledBack,
            contracts::ExportImportMigrationOutcome::RolledBack
        )
    )
}
