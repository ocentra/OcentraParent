use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_bundle_preflight_binding::execution_binding::
        RestoreExecutionBinding;

use super::RestoreExecutionPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreExecutorReceipt {
    execution_ref: contracts::ExportImportExecutionRef,
    state: contracts::ExportImportRestoreApplyState,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    provider_operation_ref: Option<contracts::ExportImportProviderOperationRef>,
}

impl RestoreExecutorReceipt {
    /// Creates a provider receipt only when the plan and owner-issued binding
    /// are the same non-serializable capability instance.
    pub fn new(
        plan: &RestoreExecutionPlan,
        binding: &RestoreExecutionBinding,
        state: contracts::ExportImportRestoreApplyState,
        applied_sections: Vec<contracts::ExportImportSectionDecision>,
        rejected_sections: Vec<contracts::ExportImportSectionDecision>,
        provider_operation_ref: Option<contracts::ExportImportProviderOperationRef>,
    ) -> Option<Self> {
        if plan.execution_binding() != binding
            || !plan.execution_binding().is_same_capability(binding)
        {
            return None;
        }
        if matches!(
            state,
            contracts::ExportImportRestoreApplyState::Applied
                | contracts::ExportImportRestoreApplyState::Partial
        ) && provider_operation_ref.is_none()
        {
            return None;
        }
        Some(Self {
            execution_ref: plan.execution_ref().clone(),
            state,
            applied_sections,
            rejected_sections,
            provider_operation_ref,
        })
    }

    pub fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

    pub fn state(&self) -> contracts::ExportImportRestoreApplyState {
        self.state
    }

    pub fn applied_sections(&self) -> &[contracts::ExportImportSectionDecision] {
        &self.applied_sections
    }

    pub fn rejected_sections(&self) -> &[contracts::ExportImportSectionDecision] {
        &self.rejected_sections
    }

    pub fn provider_operation_ref(&self) -> Option<&contracts::ExportImportProviderOperationRef> {
        self.provider_operation_ref.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreProviderOperationReceipt {
    execution_ref: contracts::ExportImportExecutionRef,
    provider_operation_ref: contracts::ExportImportProviderOperationRef,
}

impl RestoreProviderOperationReceipt {
    /// Creates a migration or rollback receipt only from the matching
    /// owner-issued plan/binding pair.
    pub fn new(
        plan: &RestoreExecutionPlan,
        binding: &RestoreExecutionBinding,
        provider_operation_ref: contracts::ExportImportProviderOperationRef,
    ) -> Option<Self> {
        if plan.execution_binding() != binding
            || !plan.execution_binding().is_same_capability(binding)
        {
            return None;
        }
        Some(Self {
            execution_ref: plan.execution_ref().clone(),
            provider_operation_ref,
        })
    }

    pub fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

    pub fn provider_operation_ref(&self) -> &contracts::ExportImportProviderOperationRef {
        &self.provider_operation_ref
    }
}
