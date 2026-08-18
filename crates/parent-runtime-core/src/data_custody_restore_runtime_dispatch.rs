use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    export_import_backup_recovery_compensation::PartialWriteCompensation,
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan,
};

use super::data_custody_restore_runtime::{
    ParentRestoreRuntime, RestoreRuntimeError, RestoreRuntimeReceipts,
};
use super::data_custody_restore_runtime_dispatch_preflight::RestorePreparation;
use super::data_custody_restore_runtime_executor::{
    execute_restore_operation, RestoreExecutorMount,
};
use super::data_custody_restore_runtime_receipts::restore_receipt_from_dispatch;
use super::data_custody_runtime_eventing::DataCustodyRuntimeEventKind;

pub(crate) const RESTORE_MIGRATION_BEFORE_DISPATCH_NOTE: &str =
    "Migration execution dispatch is durable; provider must honor the execution reference.";

impl ParentRestoreRuntime {
    pub(crate) async fn execute_restore(
        &mut self,
        plan: &RestoreExecutionPlan,
        mount: &RestoreExecutorMount<'_>,
    ) -> Result<RestoreRuntimeReceipts, RestoreRuntimeError> {
        if !self.recovered {
            return Err(RestoreRuntimeError::RuntimeNotRecovered);
        }
        let preparation = self.prepare_restore_dispatch(plan, mount).await?;
        let (existing_restore, provider, migration) = match preparation {
            RestorePreparation::Ready {
                existing_restore,
                provider,
                migration,
            } => (existing_restore, provider, migration),
            RestorePreparation::Complete(receipts) => return Ok(receipts),
        };
        self.revalidate_authority(plan, mount)?;
        let mut before_dispatch = existing_restore.clone();
        before_dispatch.recorded_at = self.next_recorded_at()?;
        before_dispatch.note = Some(
            "Restore execution dispatch is durable; provider must honor the execution reference."
                .to_owned(),
        );
        self.persist_restore_phase(
            &before_dispatch,
            DataCustodyRuntimeEventKind::RestoreBeforeDispatch,
            before_dispatch.note.clone(),
            JournalDispatchPhase::BeforeDispatch,
        )
        .await?;
        self.dispatch_started_restore
            .insert(plan.operation_ref().as_str().to_owned());
        self.revalidate_authority(plan, mount)?;

        let observation = execute_restore_operation(plan, provider)?;
        if observation.compensation() == PartialWriteCompensation::Required {
            let (applied_sections, rejected_sections, rollback_binding) =
                observation.into_rollback_observation();
            return self
                .rollback_after_observation(
                    plan,
                    mount,
                    provider,
                    applied_sections,
                    rejected_sections,
                    rollback_binding,
                )
                .await;
        }
        let note = (observation.compensation() == PartialWriteCompensation::ManualRequired)
            .then(|| "Restore result failed the no-resurrection compensation gate.".to_owned());
        let restore = restore_receipt_from_dispatch(
            plan,
            observation.state(),
            observation.applied_sections().to_vec(),
            observation.rejected_sections().to_vec(),
            observation.compensation(),
            observation.provider_operation_ref().as_ref(),
            None,
            self.next_recorded_at()?,
            note,
        )?;
        self.persist_restore(
            &restore,
            restore_event_kind(observation.state()),
            restore.note.clone(),
        )
        .await?;
        self.dispatch_started_restore
            .remove(plan.operation_ref().as_str());
        Ok(RestoreRuntimeReceipts { restore, migration })
    }
}

fn restore_event_kind(
    state: ocentra_schema::export_import_backup_recovery::ExportImportRestoreApplyState,
) -> DataCustodyRuntimeEventKind {
    match state {
        ocentra_schema::export_import_backup_recovery::ExportImportRestoreApplyState::Applied
        | ocentra_schema::export_import_backup_recovery::ExportImportRestoreApplyState::Partial => {
            DataCustodyRuntimeEventKind::RestoreApplied
        }
        _ => DataCustodyRuntimeEventKind::Reconciliation,
    }
}
