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
    pub(crate) fn new(
        plan: &RestoreExecutionPlan,
        binding: &RestoreExecutionBinding,
        state: contracts::ExportImportRestoreApplyState,
        applied_sections: Vec<contracts::ExportImportSectionDecision>,
        rejected_sections: Vec<contracts::ExportImportSectionDecision>,
        provider_operation_ref: Option<String>,
    ) -> Option<Self> {
        if plan.execution_binding() != binding
            || !plan.execution_binding().is_same_capability(binding)
        {
            return None;
        }
        let provider_operation_ref = provider_operation_ref
            .map(|value| contracts::ExportImportProviderOperationRef::parse(value))
            .transpose()?;
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
        self.provider_operation_ref.as_ref()
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
