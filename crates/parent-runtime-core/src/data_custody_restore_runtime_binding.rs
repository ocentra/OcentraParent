use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_bundle_preflight_binding::bind_import_preflight,
    export_import_backup_recovery_bundle_preflight_binding::custody_port::ImportBindingError,
    export_import_backup_recovery_restore_execution_plan::{
        build_restore_execution_plan, RestoreExecutionPlan, RestoreExecutionPlanError,
    },
};

use super::data_custody_restore_runtime::{ParentRestoreRuntime, RestoreRuntimeError};
use super::data_custody_restore_runtime_executor::RestoreExecutorMount;

impl ParentRestoreRuntime {
    pub(crate) fn next_recorded_at(
        &self,
    ) -> Result<contracts::ExportImportTimestamp, RestoreRuntimeError> {
        contracts::ExportImportTimestamp::parse(self.journal.next_recorded_at()?).ok_or(
            RestoreRuntimeError::Plan(RestoreExecutionPlanError::InvalidTimestamp),
        )
    }

    pub(crate) fn bind_plan(
        &self,
        bundle: &contracts::ExportImportRecoveryBundle,
        mount: &RestoreExecutorMount<'_>,
        plan_ref: impl Into<String>,
        operation_ref: impl Into<String>,
        execution_ref: impl Into<String>,
    ) -> Result<RestoreExecutionPlan, RestoreRuntimeError> {
        if !self.recovered {
            return Err(RestoreRuntimeError::RuntimeNotRecovered);
        }
        let authority = mount
            .account()
            .current_restore_authority(&bundle.manifest.source_household_id)
            .map_err(RestoreRuntimeError::Authority)?;
        let bound = bind_import_preflight(bundle, authority, mount.custody())?;
        build_restore_execution_plan(bundle, bound, plan_ref, operation_ref, execution_ref)
            .map_err(RestoreRuntimeError::Plan)
    }

    pub(crate) fn revalidate_authority(
        &self,
        plan: &RestoreExecutionPlan,
        mount: &RestoreExecutorMount<'_>,
    ) -> Result<(), RestoreRuntimeError> {
        let authority = mount
            .account()
            .current_restore_authority(plan.household_id())
            .map_err(RestoreRuntimeError::Authority)?;
        if plan.matches_current_authority(&authority) {
            Ok(())
        } else {
            Err(RestoreRuntimeError::Binding(
                ImportBindingError::AuthorityProofMismatch,
            ))
        }
    }
}
