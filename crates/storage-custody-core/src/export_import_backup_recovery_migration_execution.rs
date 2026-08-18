use chrono::Utc;
use ocentra_schema::export_import_backup_recovery as contracts;

use super::export_import_backup_recovery_compensation::PartialWriteCompensation;
use super::export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationExecutionError {
    MigrationNotRequired,
    MigrationReferenceMissing,
    InvalidProviderOperationRef,
    AppliedSectionsMissing,
    UnsafeSectionDecision,
    ProviderOperationRequired,
    CompensationNotResolved,
    InvalidTimestamp,
}

/// Produces a plan-only receipt. No local truth is changed and no provider
/// operation is claimed; the parent runtime must persist this receipt before
/// mounting an external executor.
pub fn plan_migration(
    plan: &RestoreExecutionPlan,
) -> Result<contracts::ExportImportMigrationReceipt, MigrationExecutionError> {
    if plan.migration_state() == contracts::ExportImportMigrationState::NotRequired {
        return Err(MigrationExecutionError::MigrationNotRequired);
    }
    let migration_ref = plan
        .migration_ref()
        .cloned()
        .ok_or(MigrationExecutionError::MigrationReferenceMissing)?;
    if plan.preflight_state() == contracts::ExportImportPreflightState::AcceptedPreview
        && plan.accepted_sections().is_empty()
    {
        return Err(MigrationExecutionError::AppliedSectionsMissing);
    }
    Ok(contracts::ExportImportMigrationReceipt {
        bundle_id: plan.bundle_id().clone(),
        migration_plan_ref: plan.plan_ref().clone(),
        migration_ref,
        operation_ref: plan.operation_ref().clone(),
        execution_ref: plan.execution_ref().clone(),
        recorded_at: contracts::ExportImportTimestamp::parse(Utc::now().to_rfc3339())
            .ok_or(MigrationExecutionError::InvalidTimestamp)?,
        outcome: contracts::ExportImportMigrationOutcome::Planned,
        applied_sections: plan.accepted_sections().to_vec(),
        rejected_sections: plan.rejected_sections().to_vec(),
        tombstones_preserved: plan.tombstones_preserved(),
        no_resurrection: plan.no_resurrection(),
        compensation_applied: false,
        provider_operation_ref: None,
        rollback_provider_operation_ref: None,
        note: Some("Migration plan is bound; apply remains parent-runtime-owned.".to_string()),
    })
}

/// Completes a migration receipt from the parent runtime's provider-neutral
/// operation result. Bundle, plan, operation, and execution references all
/// come from the owner-bound plan rather than caller-supplied identity fields.
pub fn complete_migration(
    plan: &RestoreExecutionPlan,
    outcome: contracts::ExportImportMigrationOutcome,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    compensation: PartialWriteCompensation,
    provider_operation_ref: Option<String>,
    rollback_provider_operation_ref: Option<String>,
    note: Option<String>,
) -> Result<contracts::ExportImportMigrationReceipt, MigrationExecutionError> {
    let migration_ref = plan
        .migration_ref()
        .cloned()
        .ok_or(MigrationExecutionError::MigrationReferenceMissing)?;
    if !sections_match_plan(plan, &applied_sections, &rejected_sections) {
        return Err(MigrationExecutionError::UnsafeSectionDecision);
    }
    if matches!(
        outcome,
        contracts::ExportImportMigrationOutcome::Applied
            | contracts::ExportImportMigrationOutcome::Partial
    ) && applied_sections.is_empty()
    {
        return Err(MigrationExecutionError::AppliedSectionsMissing);
    }
    if outcome == contracts::ExportImportMigrationOutcome::Applied
        && provider_operation_ref.is_none()
    {
        return Err(MigrationExecutionError::ProviderOperationRequired);
    }
    if outcome == contracts::ExportImportMigrationOutcome::RolledBack
        && compensation != PartialWriteCompensation::Applied
    {
        return Err(MigrationExecutionError::CompensationNotResolved);
    }
    if compensation == PartialWriteCompensation::Applied
        && rollback_provider_operation_ref.is_none()
    {
        return Err(MigrationExecutionError::ProviderOperationRequired);
    }
    let provider_operation_ref = provider_operation_ref
        .map(|value| {
            contracts::ExportImportProviderOperationRef::parse(value)
                .ok_or(MigrationExecutionError::InvalidProviderOperationRef)
        })
        .transpose()?;
    let rollback_provider_operation_ref = rollback_provider_operation_ref
        .map(|value| {
            contracts::ExportImportProviderOperationRef::parse(value)
                .ok_or(MigrationExecutionError::InvalidProviderOperationRef)
        })
        .transpose()?;

    Ok(contracts::ExportImportMigrationReceipt {
        bundle_id: plan.bundle_id().clone(),
        migration_plan_ref: plan.plan_ref().clone(),
        migration_ref,
        operation_ref: plan.operation_ref().clone(),
        execution_ref: plan.execution_ref().clone(),
        recorded_at: contracts::ExportImportTimestamp::parse(Utc::now().to_rfc3339())
            .ok_or(MigrationExecutionError::InvalidTimestamp)?,
        outcome,
        applied_sections,
        rejected_sections,
        tombstones_preserved: plan.tombstones_preserved(),
        no_resurrection: plan.no_resurrection(),
        compensation_applied: compensation == PartialWriteCompensation::Applied,
        provider_operation_ref,
        rollback_provider_operation_ref,
        note,
    })
}

fn sections_are_safe(
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> bool {
    let mut seen = Vec::new();
    for section in applied_sections.iter().chain(rejected_sections.iter()) {
        if section.state != contracts::ExportImportSectionDecisionState::Accepted
            && applied_sections.iter().any(|candidate| {
                candidate.data_class == section.data_class
                    && candidate.state == contracts::ExportImportSectionDecisionState::Accepted
            })
        {
            return false;
        }
        if seen.contains(&section.data_class) {
            return false;
        }
        seen.push(section.data_class);
    }
    true
}

fn sections_match_plan(
    plan: &RestoreExecutionPlan,
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> bool {
    sections_are_safe(applied_sections, rejected_sections)
        && applied_sections.iter().all(|section| {
            section.state == contracts::ExportImportSectionDecisionState::Accepted
                && plan.accepted_sections().contains(section)
        })
        && rejected_sections
            .iter()
            .all(|section| plan.rejected_sections().contains(section))
        && plan
            .rejected_sections()
            .iter()
            .all(|section| rejected_sections.contains(section))
}
