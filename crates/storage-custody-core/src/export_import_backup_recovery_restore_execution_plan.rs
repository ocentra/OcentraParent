use chrono::Utc;
use ocentra_schema::export_import_backup_recovery as contracts;

use super::export_import_backup_recovery_bundle_preflight_binding::execution_binding::RestoreExecutionBinding;
use super::export_import_backup_recovery_bundle_preflight_binding::BoundImportPreflight;
use super::export_import_backup_recovery_compensation::PartialWriteCompensation;
#[path = "export_import_backup_recovery_restore_execution_plan_validation.rs"]
mod validation;
use validation::{preflight_is_safe, sections_match_plan};

#[derive(Debug, PartialEq, Eq)]
pub struct RestoreExecutionPlan {
    bundle_id: contracts::ExportImportBundleId,
    household_id: contracts::ExportImportHouseholdId,
    plan_ref: contracts::ExportImportMigrationPlanRef,
    operation_ref: contracts::ExportImportOperationRef,
    execution_ref: contracts::ExportImportExecutionRef,
    migration_ref: Option<contracts::ExportImportMigrationRef>,
    preflight: contracts::ExportImportImportPreflight,
    execution_binding: RestoreExecutionBinding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreExecutionPlanError {
    BundleBindingMismatch,
    InvalidPlanRef,
    InvalidOperationRef,
    InvalidExecutionRef,
    PreflightNotApplicable,
    TombstoneSafetyRequired,
    UnsafeSectionDecision,
    ProviderOperationRefInvalid,
    ProviderOperationRefRequired,
    StateRequiresSections,
    InvalidTimestamp,
}

/// Binds a restore plan to the storage-produced preflight and bundle identity.
/// Raw `ExportImportImportPreflight` values are deliberately not accepted.
pub fn build_restore_execution_plan(
    bundle: &contracts::ExportImportRecoveryBundle,
    bound: BoundImportPreflight,
    plan_ref: impl Into<String>,
    operation_ref: impl Into<String>,
    execution_ref: impl Into<String>,
) -> Result<RestoreExecutionPlan, RestoreExecutionPlanError> {
    if bound.bundle_id() != &bundle.manifest.bundle_id {
        return Err(RestoreExecutionPlanError::BundleBindingMismatch);
    }
    let plan_ref = contracts::ExportImportMigrationPlanRef::parse(plan_ref)
        .ok_or(RestoreExecutionPlanError::InvalidPlanRef)?;
    let operation_ref = contracts::ExportImportOperationRef::parse(operation_ref)
        .ok_or(RestoreExecutionPlanError::InvalidOperationRef)?;
    let execution_ref = contracts::ExportImportExecutionRef::parse(execution_ref)
        .ok_or(RestoreExecutionPlanError::InvalidExecutionRef)?;
    let (bound_bundle_id, preflight, execution_binding) = bound.into_parts();
    if bound_bundle_id != bundle.manifest.bundle_id {
        return Err(RestoreExecutionPlanError::BundleBindingMismatch);
    }
    if !preflight_is_safe(&preflight) {
        return Err(RestoreExecutionPlanError::PreflightNotApplicable);
    }
    if !preflight.tombstones_preserved {
        return Err(RestoreExecutionPlanError::TombstoneSafetyRequired);
    }

    Ok(RestoreExecutionPlan {
        bundle_id: bundle.manifest.bundle_id.clone(),
        household_id: bundle.manifest.source_household_id.clone(),
        plan_ref,
        operation_ref,
        execution_ref,
        migration_ref: bundle.manifest.migration_ref.clone(),
        preflight,
        execution_binding,
    })
}

impl RestoreExecutionPlan {
    pub fn bundle_id(&self) -> &contracts::ExportImportBundleId {
        &self.bundle_id
    }

    pub fn household_id(&self) -> &contracts::ExportImportHouseholdId {
        &self.household_id
    }

    pub fn plan_ref(&self) -> &contracts::ExportImportMigrationPlanRef {
        &self.plan_ref
    }

    pub fn operation_ref(&self) -> &contracts::ExportImportOperationRef {
        &self.operation_ref
    }

    pub fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

    pub fn migration_ref(&self) -> Option<&contracts::ExportImportMigrationRef> {
        self.migration_ref.as_ref()
    }

    pub fn migration_state(&self) -> contracts::ExportImportMigrationState {
        self.preflight.migration_state
    }

    pub fn preflight_state(&self) -> contracts::ExportImportPreflightState {
        self.preflight.state
    }

    pub fn accepted_sections(&self) -> &[contracts::ExportImportSectionDecision] {
        &self.preflight.accepted_sections
    }

    pub fn rejected_sections(&self) -> &[contracts::ExportImportSectionDecision] {
        &self.preflight.rejected_sections
    }

    pub fn tombstones_preserved(&self) -> bool {
        self.preflight.tombstones_preserved
    }

    pub fn no_resurrection(&self) -> bool {
        self.preflight.tombstones_preserved && !self.preflight.local_truth_mutated
    }

    pub fn is_partial(&self) -> bool {
        self.preflight.state == contracts::ExportImportPreflightState::PartialPreview
    }

    pub fn execution_binding(&self) -> &RestoreExecutionBinding {
        &self.execution_binding
    }
}

/// Converts a provider-neutral executor result into a durable restore receipt
/// while retaining all identity and safety fields from the owner-bound plan.
pub fn complete_restore_receipt(
    plan: &RestoreExecutionPlan,
    state: contracts::ExportImportRestoreApplyState,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    compensation: PartialWriteCompensation,
    provider_operation_ref: Option<String>,
    rollback_provider_operation_ref: Option<String>,
    note: Option<String>,
) -> Result<contracts::ExportImportRestoreReceipt, RestoreExecutionPlanError> {
    if !plan.no_resurrection()
        || !sections_match_plan(plan, state, &applied_sections, &rejected_sections)
    {
        return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
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
    let provider_operation_ref = provider_operation_ref
        .map(|value| {
            contracts::ExportImportProviderOperationRef::parse(value)
                .ok_or(RestoreExecutionPlanError::ProviderOperationRefInvalid)
        })
        .transpose()?;
    let rollback_provider_operation_ref = rollback_provider_operation_ref
        .map(|value| {
            contracts::ExportImportProviderOperationRef::parse(value)
                .ok_or(RestoreExecutionPlanError::ProviderOperationRefInvalid)
        })
        .transpose()?;
    if matches!(
        state,
        contracts::ExportImportRestoreApplyState::Applied
            | contracts::ExportImportRestoreApplyState::Partial
    ) && provider_operation_ref.is_none()
    {
        return Err(RestoreExecutionPlanError::ProviderOperationRefRequired);
    }
    if compensation == PartialWriteCompensation::Applied
        && rollback_provider_operation_ref.is_none()
    {
        return Err(RestoreExecutionPlanError::ProviderOperationRefRequired);
    }
    let recorded_at = contracts::ExportImportTimestamp::parse(Utc::now().to_rfc3339())
        .ok_or(RestoreExecutionPlanError::InvalidTimestamp)?;

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
        provider_operation_ref,
        rollback_provider_operation_ref,
        note,
    })
}

/// Validates an executor observation before the parent runtime can persist an
/// outcome or begin compensation. The provider may only report the exact
/// owner-bound section partition; dispatch-pending state and applied data in a
/// blocked/error outcome are never accepted as a terminal observation.
pub fn validate_restore_execution_observation(
    plan: &RestoreExecutionPlan,
    state: contracts::ExportImportRestoreApplyState,
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> Result<(), RestoreExecutionPlanError> {
    validation::validate_restore_execution_observation(
        plan,
        state,
        applied_sections,
        rejected_sections,
    )
}
