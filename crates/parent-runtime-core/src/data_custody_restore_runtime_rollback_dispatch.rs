use super::data_custody_restore_runtime::{
    ParentRestoreRuntime, RestoreRuntimeError, RestoreRuntimeReceipts,
};
use super::data_custody_restore_runtime_executor::{
    receipts::RestoreProviderOperationReceipt, ProviderNeutralRestorePort, RestoreExecutorError,
    RestoreExecutorMount,
};
use super::data_custody_restore_runtime_receipts::restore_receipt_from_dispatch;
use super::data_custody_restore_runtime_reconciliation_validation::restore_receipt_matches_plan;
use super::data_custody_restore_runtime_rollback::record_rollback_migration;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;
use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_bundle_preflight_binding::execution_binding::RestoreExecutionStage,
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_restore_execution_plan::{
        validate_restore_execution_observation, RestoreExecutionPlan,
    },
};

impl ParentRestoreRuntime {
    pub(crate) async fn rollback_after_observation(
        &mut self,
        plan: &RestoreExecutionPlan,
        mount: &RestoreExecutorMount<'_>,
        provider: &dyn ProviderNeutralRestorePort,
        applied_sections: Vec<contracts::ExportImportSectionDecision>,
        rejected_sections: Vec<contracts::ExportImportSectionDecision>,
        observed_provider_operation_ref: Option<contracts::ExportImportProviderOperationRef>,
    ) -> Result<RestoreRuntimeReceipts, RestoreRuntimeError> {
        if !self.recovered {
            return Err(RestoreRuntimeError::RuntimeNotRecovered);
        }
        if self
            .restart_pending_rollback
            .contains(plan.operation_ref().as_str())
            || self
                .dispatch_started_rollback
                .contains(plan.operation_ref().as_str())
        {
            return Err(RestoreRuntimeError::RestartReconciliationRequired);
        }
        let Some(existing_restore) = self.ledger.restore_receipt(plan.operation_ref()).cloned()
        else {
            return Err(RestoreRuntimeError::PlanNotDurablyPending);
        };
        ensure_restore_identity(plan, &existing_restore)?;
        validate_restore_execution_observation(
            plan,
            contracts::ExportImportRestoreApplyState::Partial,
            &applied_sections,
            &rejected_sections,
        )
        .map_err(RestoreRuntimeError::Plan)?;
        self.revalidate_authority(plan, mount)?;
        if existing_restore.provider_operation_ref.is_none() {
            return Err(RestoreRuntimeError::Executor(RestoreExecutorError::Failed));
        }
        if existing_restore.compensation_applied
            && existing_restore.rollback_provider_operation_ref.is_some()
        {
            return Ok(RestoreRuntimeReceipts {
                restore: existing_restore,
                migration: self.ledger.migration_receipt(plan.operation_ref()).cloned(),
            });
        }
        let rollback_intent = rollback_intent(
            existing_restore,
            observed_provider_operation_ref,
            &applied_sections,
            &rejected_sections,
        );
        persist_rollback_intents(self, plan, &rollback_intent).await?;
        // Keep the in-memory intent fenced after the durable before-dispatch
        // records exist. Any provider error or terminal-journal failure leaves
        // this marker set so a same-process retry requires reconciliation.
        self.dispatch_started_rollback
            .insert(plan.operation_ref().as_str().to_owned());
        let provider_receipt = dispatch_rollback(plan, provider)?;
        let (restore, migration) = persist_rollback_results(
            self,
            plan,
            provider_receipt,
            applied_sections,
            rejected_sections,
        )
        .await?;
        self.dispatch_started_rollback
            .remove(plan.operation_ref().as_str());
        Ok(RestoreRuntimeReceipts { restore, migration })
    }
}

fn ensure_restore_identity(
    plan: &RestoreExecutionPlan,
    receipt: &contracts::ExportImportRestoreReceipt,
) -> Result<(), RestoreRuntimeError> {
    if restore_receipt_matches_plan(plan, receipt) {
        Ok(())
    } else {
        Err(RestoreRuntimeError::Ledger(
            super::data_custody_restore_runtime_ledger::RestoreLedgerError::IdentityMismatch,
        ))
    }
}

fn rollback_intent(
    mut existing_restore: contracts::ExportImportRestoreReceipt,
    observed_provider_operation_ref: Option<contracts::ExportImportProviderOperationRef>,
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> contracts::ExportImportRestoreReceipt {
    if let Some(provider_operation_ref) = observed_provider_operation_ref {
        existing_restore.state = contracts::ExportImportRestoreApplyState::Partial;
        existing_restore.applied_sections = applied_sections.to_vec();
        existing_restore.rejected_sections = rejected_sections.to_vec();
        existing_restore.provider_operation_ref = Some(provider_operation_ref);
        existing_restore.compensation_applied = false;
    }
    existing_restore.note = Some(
        "Rollback dispatch is durable; provider must honor the execution reference.".to_owned(),
    );
    existing_restore
}

async fn persist_rollback_intents(
    runtime: &mut ParentRestoreRuntime,
    plan: &RestoreExecutionPlan,
    restore_intent: &contracts::ExportImportRestoreReceipt,
) -> Result<(), RestoreRuntimeError> {
    let mut restore_intent = restore_intent.clone();
    restore_intent.recorded_at = runtime.next_recorded_at()?;
    runtime
        .persist_restore_phase(
            &restore_intent,
            DataCustodyRuntimeEventKind::RollbackBeforeDispatch,
            restore_intent.note.clone(),
            JournalDispatchPhase::BeforeDispatch,
        )
        .await?;
    if let Some(mut migration_intent) = runtime
        .ledger
        .migration_receipt(plan.operation_ref())
        .cloned()
    {
        if migration_intent.provider_operation_ref.is_none() {
            return Ok(());
        }
        migration_intent.recorded_at = runtime.next_recorded_at()?;
        migration_intent.note = Some(
            "Migration rollback dispatch is durable; provider must honor the execution reference."
                .to_owned(),
        );
        runtime
            .persist_migration_phase(
                &migration_intent,
                DataCustodyRuntimeEventKind::RollbackBeforeDispatch,
                migration_intent.note.clone(),
                JournalDispatchPhase::BeforeDispatch,
            )
            .await?;
    }
    Ok(())
}

fn dispatch_rollback(
    plan: &RestoreExecutionPlan,
    provider: &dyn ProviderNeutralRestorePort,
) -> Result<RestoreProviderOperationReceipt, RestoreRuntimeError> {
    let reservation = plan
        .execution_binding()
        .reserve_dispatch(plan.execution_ref(), RestoreExecutionStage::Rollback)
        .map_err(|_| RestoreRuntimeError::Executor(RestoreExecutorError::Failed))?;
    let provider_receipt = provider.rollback_restore(plan, reservation)?;
    if provider_receipt.execution_ref() != plan.execution_ref() {
        return Err(RestoreRuntimeError::Executor(RestoreExecutorError::Failed));
    }
    Ok(provider_receipt)
}

async fn persist_rollback_results(
    runtime: &mut ParentRestoreRuntime,
    plan: &RestoreExecutionPlan,
    provider_receipt: RestoreProviderOperationReceipt,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
) -> Result<
    (
        contracts::ExportImportRestoreReceipt,
        Option<contracts::ExportImportMigrationReceipt>,
    ),
    RestoreRuntimeError,
> {
    let original_restore_provider_operation_ref = runtime
        .ledger
        .restore_receipt(plan.operation_ref())
        .and_then(|receipt| receipt.provider_operation_ref.as_ref())
        .cloned();
    let restore = restore_receipt_from_dispatch(
        plan,
        contracts::ExportImportRestoreApplyState::Partial,
        applied_sections.clone(),
        rejected_sections.clone(),
        PartialWriteCompensation::Applied,
        original_restore_provider_operation_ref.as_ref(),
        Some(provider_receipt.provider_operation_ref()),
        runtime.next_recorded_at()?,
        Some("Restore rollback completed through the mounted provider port.".to_owned()),
    )?;
    runtime
        .persist_restore(
            &restore,
            DataCustodyRuntimeEventKind::Rollback,
            restore.note.clone(),
        )
        .await?;
    let migration_recorded_at = runtime.next_recorded_at()?;
    let migration = rollback_migration_if_required(
        runtime,
        plan,
        provider_receipt.provider_operation_ref().clone(),
        applied_sections,
        rejected_sections,
        migration_recorded_at,
    )
    .await?;
    Ok((restore, migration))
}

async fn rollback_migration_if_required(
    runtime: &mut ParentRestoreRuntime,
    plan: &RestoreExecutionPlan,
    rollback_provider_operation_ref: contracts::ExportImportProviderOperationRef,
    applied_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    recorded_at: contracts::ExportImportTimestamp,
) -> Result<Option<contracts::ExportImportMigrationReceipt>, RestoreRuntimeError> {
    let Some(existing_migration) = runtime.ledger.migration_receipt(plan.operation_ref()) else {
        return Ok(None);
    };
    if existing_migration.provider_operation_ref.is_none() {
        return Ok(None);
    }
    let original_provider_operation_ref =
        existing_migration.provider_operation_ref.as_ref().cloned();
    let migration = record_rollback_migration(
        plan,
        original_provider_operation_ref,
        rollback_provider_operation_ref,
        applied_sections,
        rejected_sections,
        recorded_at,
        None,
    )
    .map_err(RestoreRuntimeError::Rollback)?;
    runtime
        .persist_migration(
            &migration,
            DataCustodyRuntimeEventKind::Rollback,
            migration.note.clone(),
        )
        .await?;
    Ok(Some(migration))
}
