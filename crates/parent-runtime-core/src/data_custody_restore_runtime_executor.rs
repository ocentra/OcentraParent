use ocentra_family_identity_core::household_authority_runtime_composer::HouseholdAuthorityRuntimeEffectAuthorization;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_bundle_preflight_binding::custody_port::ImportCustodyCapabilityPort,
    export_import_backup_recovery_bundle_preflight_binding::execution_binding::{
        RestoreDispatchReservation, RestoreExecutionBinding, RestoreExecutionStage,
    },
    export_import_backup_recovery_compensation::{
        decide_partial_write_compensation, PartialWriteCompensation, PartialWriteObservation,
    },
    export_import_backup_recovery_migration_execution::MigrationExecutionError,
    export_import_backup_recovery_restore_execution_plan::{
        validate_restore_execution_observation, RestoreExecutionPlan, RestoreExecutionPlanError,
    },
};

#[path = "data_custody_restore_runtime_executor_receipts.rs"]
pub mod receipts;
use self::receipts::{
    RestoreExecutorReceipt, RestoreProviderOperationReceipt, RestoreRollbackBinding,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreAuthorityUnavailable {
    Unavailable,
}

pub(crate) trait RestoreAccountAuthorityPort: Send + Sync {
    fn current_restore_authority(
        &self,
        household_id: &contracts::ExportImportHouseholdId,
    ) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, RestoreAuthorityUnavailable>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreExecutorError {
    Unavailable,
    Failed,
}

#[derive(Debug)]
pub enum RestoreExecutorOperationError {
    Executor(RestoreExecutorError),
    Receipt(RestoreExecutionPlanError),
    Migration(MigrationExecutionError),
}

/// Opaque external executor boundary. The parent runtime mounts only these
/// provider-neutral operations; SDK, OAuth, filesystem, and key storage
/// implementations remain owned by their dependency runtimes.
mod restore_provider_sealed {
    pub trait Port {}
}

pub(crate) trait ProviderNeutralRestorePort:
    restore_provider_sealed::Port + Send + Sync
{
    fn execute_restore<'a>(
        &self,
        plan: &RestoreExecutionPlan,
        reservation: RestoreDispatchReservation<'a>,
    ) -> Result<RestoreExecutorReceipt<'a>, RestoreExecutorError>;
    fn execute_migration(
        &self,
        plan: &RestoreExecutionPlan,
        reservation: RestoreDispatchReservation<'_>,
    ) -> Result<RestoreProviderOperationReceipt, RestoreExecutorError>;
    fn rollback_restore<'a>(
        &self,
        plan: &RestoreExecutionPlan,
        reservation: RestoreDispatchReservation<'a>,
        rollback_binding: RestoreRollbackBinding<'a>,
    ) -> Result<RestoreProviderOperationReceipt, RestoreExecutorError>;
}

/// Explicit mount containing the account authority, key/decrypt capability,
/// and provider-neutral executor ports required by the parent restore owner.
/// No default implementation is supplied, so an absent dependency fails
/// closed instead of creating a synthetic successful receipt.
pub struct RestoreExecutorMount<'a> {
    account: &'a dyn RestoreAccountAuthorityPort,
    custody: &'a dyn ImportCustodyCapabilityPort,
    provider: Option<&'a dyn ProviderNeutralRestorePort>,
}

#[derive(Debug)]
pub(crate) struct RestoreExecutionObservation<'a> {
    state: contracts::ExportImportRestoreApplyState,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    rollback_binding: Option<RestoreRollbackBinding<'a>>,
    compensation: PartialWriteCompensation,
}

impl<'a> RestoreExecutionObservation<'a> {
    pub(crate) fn state(&self) -> contracts::ExportImportRestoreApplyState {
        self.state
    }

    pub(crate) fn applied_sections(&self) -> &[contracts::ExportImportSectionDecision] {
        &self.applied_sections
    }

    pub(crate) fn rejected_sections(&self) -> &[contracts::ExportImportSectionDecision] {
        &self.rejected_sections
    }

    pub(crate) fn provider_operation_ref(
        &self,
    ) -> Option<&contracts::ExportImportProviderOperationRef> {
        self.rollback_binding
            .as_ref()
            .map(RestoreRollbackBinding::provider_operation_ref)
    }

    pub(crate) fn compensation(&self) -> PartialWriteCompensation {
        self.compensation
    }

    pub(crate) fn into_rollback_observation(
        self,
    ) -> (
        Vec<contracts::ExportImportSectionDecision>,
        Vec<contracts::ExportImportSectionDecision>,
        Option<RestoreRollbackBinding<'a>>,
    ) {
        (
            self.applied_sections,
            self.rejected_sections,
            self.rollback_binding,
        )
    }
}

impl<'a> RestoreExecutorMount<'a> {
    pub(crate) fn new(
        account: &'a dyn RestoreAccountAuthorityPort,
        custody: &'a dyn ImportCustodyCapabilityPort,
        provider: Option<&'a dyn ProviderNeutralRestorePort>,
    ) -> Self {
        Self {
            account,
            custody,
            provider,
        }
    }

    pub(crate) fn account(&self) -> &'a dyn RestoreAccountAuthorityPort {
        self.account
    }

    pub(crate) fn custody(&self) -> &'a dyn ImportCustodyCapabilityPort {
        self.custody
    }

    pub(crate) fn provider(&self) -> Option<&'a dyn ProviderNeutralRestorePort> {
        self.provider
    }
}

pub(crate) fn execute_restore_operation<'a>(
    plan: &'a RestoreExecutionPlan,
    provider: &dyn ProviderNeutralRestorePort,
) -> Result<RestoreExecutionObservation<'a>, RestoreExecutorOperationError> {
    let reservation = plan
        .execution_binding()
        .reserve_dispatch(plan.execution_ref(), RestoreExecutionStage::Restore)
        .map_err(|_| RestoreExecutorOperationError::Executor(RestoreExecutorError::Failed))?;
    let receipt = provider
        .execute_restore(plan, reservation)
        .map_err(RestoreExecutorOperationError::Executor)?;
    if receipt.execution_ref() != plan.execution_ref() {
        return Err(RestoreExecutorOperationError::Executor(
            RestoreExecutorError::Failed,
        ));
    }
    validate_restore_execution_observation(
        plan,
        receipt.state(),
        receipt.applied_sections(),
        receipt.rejected_sections(),
    )
    .map_err(RestoreExecutorOperationError::Receipt)?;
    let (receipt_state, applied_sections, rejected_sections, rollback_binding) =
        receipt.into_observation_parts();
    let observation = PartialWriteObservation {
        applied_sections: applied_sections.clone(),
        rejected_sections: rejected_sections.clone(),
    };
    let compensation = decide_partial_write_compensation(plan, &observation);
    let state = match compensation {
        PartialWriteCompensation::Required => contracts::ExportImportRestoreApplyState::Partial,
        PartialWriteCompensation::ManualRequired => {
            contracts::ExportImportRestoreApplyState::Blocked
        }
        PartialWriteCompensation::NotRequired => receipt_state,
        PartialWriteCompensation::Applied => receipt_state,
    };
    Ok(RestoreExecutionObservation {
        state,
        applied_sections,
        rejected_sections,
        rollback_binding,
        compensation,
    })
}
