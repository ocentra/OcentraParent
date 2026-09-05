use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_migration_execution::{
        validate_migration_execution_observation, MigrationExecutionError,
    },
    export_import_backup_recovery_restore_execution_plan::{
        validate_restore_execution_observation, RestoreExecutionPlan, RestoreExecutionPlanError,
    },
};

pub(crate) struct RestoreReceiptDispatch<'a> {
    pub(crate) state: contracts::ExportImportRestoreApplyState,
    pub(crate) applied_sections: Vec<contracts::ExportImportSectionDecision>,
    pub(crate) rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    pub(crate) compensation: PartialWriteCompensation,
    pub(crate) provider_operation: Option<&'a contracts::ExportImportProviderOperationRef>,
    pub(crate) rollback_provider_operation: Option<&'a contracts::ExportImportProviderOperationRef>,
    pub(crate) recorded_at: contracts::ExportImportTimestamp,
    pub(crate) note: Option<String>,
}

pub(crate) struct MigrationReceiptDispatch<'a> {
    pub(crate) outcome: contracts::ExportImportMigrationOutcome,
    pub(crate) applied_sections: Vec<contracts::ExportImportSectionDecision>,
    pub(crate) rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    pub(crate) compensation: PartialWriteCompensation,
    pub(crate) provider_operation: Option<&'a contracts::ExportImportProviderOperationRef>,
    pub(crate) rollback_provider_operation: Option<&'a contracts::ExportImportProviderOperationRef>,
    pub(crate) recorded_at: contracts::ExportImportTimestamp,
    pub(crate) note: Option<String>,
}

pub(crate) fn restore_receipt_from_dispatch(
    plan: &RestoreExecutionPlan,
    dispatch: RestoreReceiptDispatch<'_>,
) -> Result<contracts::ExportImportRestoreReceipt, RestoreExecutionPlanError> {
    let RestoreReceiptDispatch {
        state,
        applied_sections,
        rejected_sections,
        compensation,
        provider_operation,
        rollback_provider_operation,
        recorded_at,
        note,
    } = dispatch;
    if !plan.no_resurrection() {
        return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
    }
    if matches!(
        state,
        contracts::ExportImportRestoreApplyState::ApplyPending
            | contracts::ExportImportRestoreApplyState::Blocked
    ) {
        if !applied_sections.is_empty() || rejected_sections != plan.rejected_sections() {
            return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
        }
    } else {
        validate_restore_execution_observation(plan, state, &applied_sections, &rejected_sections)?;
    }
    if state == contracts::ExportImportRestoreApplyState::Applied
        && (plan.is_partial()
            || applied_sections.as_slice() != plan.accepted_sections()
            || rejected_sections.as_slice() != plan.rejected_sections())
    {
        return Err(RestoreExecutionPlanError::StateRequiresSections);
    }
    if state == contracts::ExportImportRestoreApplyState::Partial && applied_sections.is_empty() {
        return Err(RestoreExecutionPlanError::StateRequiresSections);
    }
    if matches!(
        state,
        contracts::ExportImportRestoreApplyState::Applied
            | contracts::ExportImportRestoreApplyState::Partial
    ) && provider_operation.is_none()
    {
        return Err(RestoreExecutionPlanError::ProviderOperationRefRequired);
    }
    if compensation == PartialWriteCompensation::Applied
        && (rollback_provider_operation.is_none()
            || provider_operation
                .zip(rollback_provider_operation)
                .is_some_and(|(original, rollback)| original == rollback))
    {
        return Err(RestoreExecutionPlanError::ProviderOperationRefRequired);
    }
    Ok(contracts::ExportImportRestoreReceipt {
        bundle_id: plan.bundle_id().clone(),
        restore_plan_ref: plan.plan_ref().clone(),
        operation_ref: plan.operation_ref().clone(),
        execution_ref: plan.execution_ref().clone(),
        recorded_at,
        state,
        applied_sections,
        rejected_sections,
        tombstones_preserved: plan.tombstones_preserved(),
        no_resurrection: plan.no_resurrection(),
        compensation_applied: compensation == PartialWriteCompensation::Applied,
        provider_operation_ref: provider_operation.cloned(),
        rollback_provider_operation_ref: rollback_provider_operation.cloned(),
        note,
    })
}

pub(crate) fn migration_receipt_from_dispatch(
    plan: &RestoreExecutionPlan,
    dispatch: MigrationReceiptDispatch<'_>,
) -> Result<contracts::ExportImportMigrationReceipt, MigrationExecutionError> {
    let MigrationReceiptDispatch {
        outcome,
        applied_sections,
        rejected_sections,
        compensation,
        provider_operation,
        rollback_provider_operation,
        recorded_at,
        note,
    } = dispatch;
    let migration_ref = plan
        .migration_ref()
        .cloned()
        .ok_or(MigrationExecutionError::MigrationReferenceMissing)?;
    validate_migration_execution_observation(plan, &applied_sections, &rejected_sections)?;
    if matches!(
        outcome,
        contracts::ExportImportMigrationOutcome::Applied
            | contracts::ExportImportMigrationOutcome::Partial
    ) && (applied_sections.is_empty() || provider_operation.is_none())
    {
        return Err(MigrationExecutionError::ProviderOperationRequired);
    }
    if outcome == contracts::ExportImportMigrationOutcome::RolledBack
        && compensation != PartialWriteCompensation::Applied
    {
        return Err(MigrationExecutionError::CompensationNotResolved);
    }
    if compensation == PartialWriteCompensation::Applied
        && (rollback_provider_operation.is_none()
            || provider_operation
                .zip(rollback_provider_operation)
                .is_some_and(|(original, rollback)| original == rollback))
    {
        return Err(MigrationExecutionError::ProviderOperationRequired);
    }
    Ok(contracts::ExportImportMigrationReceipt {
        bundle_id: plan.bundle_id().clone(),
        migration_plan_ref: plan.plan_ref().clone(),
        migration_ref,
        operation_ref: plan.operation_ref().clone(),
        execution_ref: plan.execution_ref().clone(),
        recorded_at,
        outcome,
        applied_sections,
        rejected_sections,
        tombstones_preserved: plan.tombstones_preserved(),
        no_resurrection: plan.no_resurrection(),
        compensation_applied: compensation == PartialWriteCompensation::Applied,
        provider_operation_ref: provider_operation.cloned(),
        rollback_provider_operation_ref: rollback_provider_operation.cloned(),
        note,
    })
}
