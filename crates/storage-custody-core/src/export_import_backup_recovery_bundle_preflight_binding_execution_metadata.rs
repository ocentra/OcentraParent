use ocentra_schema::export_import_backup_recovery as contracts;

use super::{RestoreDispatchReservation, RestoreExecutionBinding, RestoreExecutionStage};

impl<'a> RestoreDispatchReservation<'a> {
    pub fn binding(&self) -> &'a RestoreExecutionBinding {
        self.binding
    }

    pub fn execution_ref(&self) -> &'a contracts::ExportImportExecutionRef {
        self.execution_ref
    }

    pub fn stage(&self) -> RestoreExecutionStage {
        self.stage
    }
}

impl RestoreExecutionStage {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Migration => "migration",
            Self::Restore => "restore",
            Self::Rollback => "rollback",
        }
    }
}

impl RestoreExecutionBinding {
    pub fn bundle_id(&self) -> &contracts::ExportImportBundleId {
        &self.bundle_id
    }

    pub fn key_ref(&self) -> &contracts::ExportImportKeyRef {
        &self.key_ref
    }

    pub fn manifest_integrity_ref(&self) -> &contracts::ExportImportIntegrityRef {
        &self.manifest_integrity_ref
    }

    pub fn target_device_id(&self) -> &contracts::ExportImportDeviceId {
        &self.target_device_id
    }

    pub fn payload_integrity_refs(
        &self,
    ) -> &[(
        contracts::ExportImportDataClass,
        contracts::ExportImportIntegrityRef,
    )] {
        &self.payload_integrity_refs
    }

    pub fn accepted_sections(&self) -> &[contracts::ExportImportSectionDecision] {
        &self.accepted_sections
    }

    pub fn rejected_sections(&self) -> &[contracts::ExportImportSectionDecision] {
        &self.rejected_sections
    }

    pub fn tombstones_preserved(&self) -> bool {
        self.tombstones_preserved
    }

    pub fn no_resurrection(&self) -> bool {
        self.no_resurrection
    }

    pub fn migration_state(&self) -> contracts::ExportImportMigrationState {
        self.migration_state
    }

    pub fn reserve_dispatch(
        &self,
        execution_ref: &contracts::ExportImportExecutionRef,
        stage: RestoreExecutionStage,
    ) -> Result<RestoreDispatchReservation<'_>, super::DispatchReservationError> {
        let key = format!("{}:{}", execution_ref.as_str(), stage.as_str());
        let mut reservations = self
            .dispatch_reservations
            .lock()
            .map_err(|_| super::DispatchReservationError::OwnerUnavailable)?;
        if !reservations.insert(key) {
            return Err(super::DispatchReservationError::AlreadyReserved);
        }
        Ok(RestoreDispatchReservation {
            binding: self,
            execution_ref,
            stage,
        })
    }
}
