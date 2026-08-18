use ocentra_family_identity_core::household_authority_runtime_composer::HouseholdAuthorityRuntimeEffectAuthorization;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_bundle_preflight_binding::custody_port::ImportCustodyCapabilityPort,
    export_import_backup_recovery_bundle_preflight_binding::execution_binding::{
        RestoreDispatchReservation, RestoreExecutionBinding, RestoreExecutionStage,
    },
    export_import_backup_recovery_compensation::{
        PartialWriteCompensation, PartialWriteObservation, decide_partial_write_compensation,
    },
    export_import_backup_recovery_migration_execution::MigrationExecutionError,
    export_import_backup_recovery_restore_execution_plan::{
        RestoreExecutionPlan, RestoreExecutionPlanError, validate_restore_execution_observation,
    },
};

#[path = "data_custody_restore_runtime_executor_receipts.rs"]
pub mod receipts;
use self::receipts::{RestoreExecutorReceipt, RestoreProviderOperationReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreAuthorityUnavailable {
    Unavailable,
}

/// Account-owned currentness boundary for restore authorization. Implementors
/// return only the opaque runtime authorization issued by Account; no
/// household, role, generation, or session fields are accepted from callers.
pub trait RestoreAccountAuthorityPort: Send + Sync {
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
pub trait ProviderNeutralRestorePort: Send + Sync {
    fn execute_restore(
        &self,
        plan: &RestoreExecutionPlan,
        reservation: RestoreDispatchReservation<'_>,
    ) -> Result<RestoreExecutorReceipt, RestoreExecutorError>;
    fn execute_migration(
        &self,
        plan: &RestoreExecutionPlan,
        reservation: RestoreDispatchReservation<'_>,
    ) -> Result<RestoreProviderOperationReceipt, RestoreExecutorError>;
    fn rollback_restore(
        &self,
        plan: &RestoreExecutionPlan,
        reservation: RestoreDispatchReservation<'_>,
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
pub(crate) struct RestoreExecutionObservation {
    state: contracts::ExportImportRestoreApplyState,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    provider_operation_ref: Option<contracts::ExportImportProviderOperationRef>,
    compensation: PartialWriteCompensation,
}

impl RestoreExecutionObservation {
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
        self.provider_operation_ref.as_ref()
    }

    pub(crate) fn compensation(&self) -> PartialWriteCompensation {
        self.compensation
    }
}

impl<'a> RestoreExecutorMount<'a> {
    pub fn new(
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

    pub fn account(&self) -> &'a dyn RestoreAccountAuthorityPort {
        self.account
    }

    pub fn custody(&self) -> &'a dyn ImportCustodyCapabilityPort {
        self.custody
    }

    pub fn provider(&self) -> Option<&'a dyn ProviderNeutralRestorePort> {
        self.provider
    }
}

pub(crate) fn execute_restore_operation(
    plan: &RestoreExecutionPlan,
    provider: &dyn ProviderNeutralRestorePort,
) -> Result<RestoreExecutionObservation, RestoreExecutorOperationError> {
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
    let observation = PartialWriteObservation {
        applied_sections: receipt.applied_sections.clone(),
        rejected_sections: receipt.rejected_sections.clone(),
    };
    let compensation = decide_partial_write_compensation(plan, &observation);
    let state = match compensation {
        PartialWriteCompensation::Required => contracts::ExportImportRestoreApplyState::Partial,
        PartialWriteCompensation::ManualRequired => {
            contracts::ExportImportRestoreApplyState::Blocked
        }
        PartialWriteCompensation::NotRequired => receipt.state,
        PartialWriteCompensation::Applied => receipt.state,
    };
    Ok(RestoreExecutionObservation {
        state,
        applied_sections: receipt.applied_sections,
        rejected_sections: receipt.rejected_sections,
        provider_operation_ref: receipt.provider_operation_ref,
        compensation,
    })
}
