use ocentra_schema::export_import_backup_recovery as contracts;
use std::{collections::BTreeSet, fmt::Debug, sync::Mutex};

#[path = "export_import_backup_recovery_bundle_preflight_binding_execution_metadata.rs"]
mod metadata;
#[path = "export_import_backup_recovery_bundle_preflight_binding_execution_metadata_identity.rs"]
mod metadata_identity;

/// The family/key owner supplies the implementation. The value is deliberately
/// opaque, non-Clone, and non-serde; it is accepted only inside an owner-issued
/// custody result and is never exposed as a raw key or decrypt handle.
pub trait RestoreExecutionCapability: Send + Sync {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreExecutionStage {
    Migration,
    Restore,
    Rollback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchReservationError {
    AlreadyReserved,
    OwnerUnavailable,
}

/// One provider-dispatch reservation. It is intentionally non-clone and is
/// consumed by the provider port, while the binding tracks stage identity so
/// an ambiguous provider return cannot be dispatched again in this process.
pub struct RestoreDispatchReservation<'a> {
    binding: &'a RestoreExecutionBinding,
    execution_ref: &'a contracts::ExportImportExecutionRef,
    stage: RestoreExecutionStage,
}

/// The provider-facing custody binding is intentionally non-serde and cannot
/// be constructed from a wire preflight. Its public metadata is limited to
/// typed references; the capability token itself remains opaque.
pub struct RestoreExecutionBinding {
    bundle_id: contracts::ExportImportBundleId,
    key_ref: contracts::ExportImportKeyRef,
    manifest_integrity_ref: contracts::ExportImportIntegrityRef,
    payload_integrity_refs: Vec<(
        contracts::ExportImportDataClass,
        contracts::ExportImportIntegrityRef,
    )>,
    target_device_id: contracts::ExportImportDeviceId,
    accepted_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    tombstones_preserved: bool,
    no_resurrection: bool,
    migration_ref: Option<contracts::ExportImportMigrationRef>,
    migration_state: contracts::ExportImportMigrationState,
    capability: Box<dyn RestoreExecutionCapability>,
    dispatch_reservations: Mutex<BTreeSet<String>>,
}

impl Debug for RestoreExecutionBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RestoreExecutionBinding")
            .field("bundle_id", &self.bundle_id)
            .field("key_ref", &self.key_ref)
            .field("manifest_integrity_ref", &self.manifest_integrity_ref)
            .field("target_device_id", &self.target_device_id)
            .field("payload_integrity_refs", &self.payload_integrity_refs)
            .field("accepted_sections", &self.accepted_sections)
            .field("rejected_sections", &self.rejected_sections)
            .field("tombstones_preserved", &self.tombstones_preserved)
            .field("no_resurrection", &self.no_resurrection)
            .field("migration_ref", &self.migration_ref)
            .field("migration_state", &self.migration_state)
            .finish_non_exhaustive()
    }
}

impl PartialEq for RestoreExecutionBinding {
    fn eq(&self, other: &Self) -> bool {
        self.bundle_id == other.bundle_id
            && self.key_ref == other.key_ref
            && self.manifest_integrity_ref == other.manifest_integrity_ref
            && self.payload_integrity_refs == other.payload_integrity_refs
            && self.target_device_id == other.target_device_id
            && self.accepted_sections == other.accepted_sections
            && self.rejected_sections == other.rejected_sections
            && self.tombstones_preserved == other.tombstones_preserved
            && self.no_resurrection == other.no_resurrection
            && self.migration_ref == other.migration_ref
            && self.migration_state == other.migration_state
    }
}

impl Eq for RestoreExecutionBinding {}

impl RestoreExecutionBinding {
    pub(crate) fn from_parts(
        bundle_id: contracts::ExportImportBundleId,
        key_ref: contracts::ExportImportKeyRef,
        manifest_integrity_ref: contracts::ExportImportIntegrityRef,
        payload_integrity_refs: Vec<(
            contracts::ExportImportDataClass,
            contracts::ExportImportIntegrityRef,
        )>,
        target_device_id: contracts::ExportImportDeviceId,
        accepted_sections: Vec<contracts::ExportImportSectionDecision>,
        rejected_sections: Vec<contracts::ExportImportSectionDecision>,
        tombstones_preserved: bool,
        no_resurrection: bool,
        migration_ref: Option<contracts::ExportImportMigrationRef>,
        migration_state: contracts::ExportImportMigrationState,
        capability: Box<dyn RestoreExecutionCapability>,
    ) -> Self {
        Self {
            bundle_id,
            key_ref,
            manifest_integrity_ref,
            payload_integrity_refs,
            target_device_id,
            accepted_sections,
            rejected_sections,
            tombstones_preserved,
            no_resurrection,
            migration_ref,
            migration_state,
            capability,
            dispatch_reservations: Mutex::new(BTreeSet::new()),
        }
    }
}
