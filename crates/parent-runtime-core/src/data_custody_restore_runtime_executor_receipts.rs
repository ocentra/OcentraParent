use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_bundle_preflight_binding::execution_binding::
        {RestoreDispatchReservation, RestoreExecutionBinding, RestoreExecutionStage};

use super::RestoreExecutionPlan;

/// Non-serializable provider operation identity issued only from a reserved
/// restore dispatch. The binding and execution reference are retained by
/// reference so rollback cannot be authorized from a persisted provider ref.
#[derive(Debug)]
pub(crate) struct RestoreRollbackBinding<'a> {
    binding: &'a RestoreExecutionBinding,
    execution_ref: &'a contracts::ExportImportExecutionRef,
    provider_operation_ref: contracts::ExportImportProviderOperationRef,
}

impl<'a> RestoreRollbackBinding<'a> {
    fn from_reservation(
        reservation: &RestoreDispatchReservation<'a>,
        provider_operation_ref: impl Into<String>,
    ) -> Option<Self> {
        if reservation.stage() != RestoreExecutionStage::Restore {
            return None;
        }
        Some(Self {
            binding: reservation.binding(),
            execution_ref: reservation.execution_ref(),
            provider_operation_ref: contracts::ExportImportProviderOperationRef::parse(
                provider_operation_ref,
            )?,
        })
    }

    pub(crate) fn is_bound_to(
        &self,
        binding: &RestoreExecutionBinding,
        execution_ref: &contracts::ExportImportExecutionRef,
    ) -> bool {
        std::ptr::eq(self.binding, binding) && self.execution_ref == execution_ref
    }

    pub(crate) fn provider_operation_ref(&self) -> &contracts::ExportImportProviderOperationRef {
        &self.provider_operation_ref
    }
}

#[derive(Debug)]
pub(crate) struct RestoreExecutorReceipt<'a> {
    execution_ref: contracts::ExportImportExecutionRef,
    state: contracts::ExportImportRestoreApplyState,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    rollback_binding: Option<RestoreRollbackBinding<'a>>,
}

impl<'a> RestoreExecutorReceipt<'a> {
    pub(super) fn new(
        plan: &RestoreExecutionPlan,
        reservation: &RestoreDispatchReservation<'a>,
        state: contracts::ExportImportRestoreApplyState,
        applied_sections: Vec<contracts::ExportImportSectionDecision>,
        rejected_sections: Vec<contracts::ExportImportSectionDecision>,
        provider_operation_ref: Option<String>,
    ) -> Option<Self> {
        if plan.execution_binding() != reservation.binding()
            || !plan
                .execution_binding()
                .is_same_capability(reservation.binding())
            || reservation.execution_ref() != plan.execution_ref()
            || reservation.stage() != RestoreExecutionStage::Restore
        {
            return None;
        }
        let rollback_binding = match provider_operation_ref {
            Some(value) => Some(RestoreRollbackBinding::from_reservation(
                reservation,
                value,
            )?),
            None => None,
        };
        if matches!(
            state,
            contracts::ExportImportRestoreApplyState::Applied
                | contracts::ExportImportRestoreApplyState::Partial
        ) && rollback_binding.is_none()
        {
            return None;
        }
        Some(Self {
            execution_ref: plan.execution_ref().clone(),
            state,
            applied_sections,
            rejected_sections,
            rollback_binding,
        })
    }

    pub(crate) fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

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

    pub(crate) fn into_observation_parts(
        self,
    ) -> (
        contracts::ExportImportRestoreApplyState,
        Vec<contracts::ExportImportSectionDecision>,
        Vec<contracts::ExportImportSectionDecision>,
        Option<RestoreRollbackBinding<'a>>,
    ) {
        (
            self.state,
            self.applied_sections,
            self.rejected_sections,
            self.rollback_binding,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreProviderOperationReceipt {
    execution_ref: contracts::ExportImportExecutionRef,
    provider_operation_ref: contracts::ExportImportProviderOperationRef,
}

impl RestoreProviderOperationReceipt {
    pub(crate) fn new(
        plan: &RestoreExecutionPlan,
        binding: &RestoreExecutionBinding,
        provider_operation_ref: impl Into<String>,
    ) -> Option<Self> {
        if plan.execution_binding() != binding
            || !plan.execution_binding().is_same_capability(binding)
        {
            return None;
        }
        Some(Self {
            execution_ref: plan.execution_ref().clone(),
            provider_operation_ref: contracts::ExportImportProviderOperationRef::parse(
                provider_operation_ref,
            )?,
        })
    }

    pub(crate) fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

    pub(crate) fn provider_operation_ref(&self) -> &contracts::ExportImportProviderOperationRef {
        &self.provider_operation_ref
    }
}
