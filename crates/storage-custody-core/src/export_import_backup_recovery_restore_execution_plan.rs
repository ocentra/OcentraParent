use ocentra_family_identity_core::household_authority_runtime_composer::HouseholdAuthorityRuntimeEffectAuthorization;
use ocentra_schema::export_import_backup_recovery as contracts;

use super::export_import_backup_recovery_bundle_preflight_binding::execution_binding::RestoreExecutionBinding;
use super::export_import_backup_recovery_bundle_preflight_binding::BoundImportPreflight;
#[path = "export_import_backup_recovery_restore_execution_plan_validation.rs"]
mod validation;
use validation::{preflight_is_safe, sections_match_plan};

#[derive(Debug, PartialEq, Eq)]
pub struct RestoreExecutionPlan {
    bundle_id: contracts::ExportImportBundleId,
    household_id: contracts::ExportImportHouseholdId,
    created_at: contracts::ExportImportTimestamp,
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
        created_at: bundle.manifest.created_at.clone(),
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

    pub fn created_at(&self) -> &contracts::ExportImportTimestamp {
        &self.created_at
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

    pub fn matches_current_runtime_authority(
        &self,
        authority: HouseholdAuthorityRuntimeEffectAuthorization,
    ) -> bool {
        authority
            .consume_for_data_custody(
                ocentra_family_identity_core::household_authority::HouseholdAuthorityAction::ImportRestoreData,
                self.household_id.as_str(),
                Some(self.execution_binding.target_device_id().as_str()),
                None,
            )
            .is_ok()
    }
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
