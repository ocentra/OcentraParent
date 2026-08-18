use ocentra_family_identity_core::household_authority_runtime_composer::HouseholdAuthorityRuntimeEffectAuthorization;
use ocentra_schema::export_import_backup_recovery as contracts;

use super::execution_binding::RestoreExecutionCapability;

mod sealed {
    pub trait Port {}
}

/// Capability port owned by the account/key custody runtime. Implementations
/// are external to storage-custody-core; this crate consumes only successful
/// capability operations and never accepts caller-supplied integrity or
/// authority booleans.
pub trait ImportCustodyCapabilityPort: sealed::Port + Send + Sync {
    /// Atomically verifies the current authority binding, local target, key,
    /// manifest/payload integrity, migration path, and section decisions. The
    /// returned snapshot carries one opaque custody capability; callers cannot
    /// assemble a preflight by mixing observations from separate calls.
    fn verify_import_bundle(
        &self,
        bundle: &contracts::ExportImportRecoveryBundle,
        authority: &HouseholdAuthorityRuntimeEffectAuthorization,
    ) -> Result<VerifiedImportCustody, ImportBindingError>;
}

/// Non-serde result issued only by the atomic account/key custody port. It
/// retains the complete owner decision and the authority generation expected by
/// the Account-owned opaque handoff until the parent runtime creates its
/// non-serializable execution binding.
pub struct VerifiedImportCustody {
    bundle_id: contracts::ExportImportBundleId,
    key_ref: contracts::ExportImportKeyRef,
    manifest_integrity_ref: contracts::ExportImportIntegrityRef,
    payload_integrity_refs: Vec<(
        contracts::ExportImportDataClass,
        contracts::ExportImportIntegrityRef,
    )>,
    household_id: contracts::ExportImportHouseholdId,
    target_device_id: Option<contracts::ExportImportDeviceId>,
    authority_generation: u64,
    migration_ref: Option<contracts::ExportImportMigrationRef>,
    preflight: contracts::ExportImportImportPreflight,
    capability: Box<dyn RestoreExecutionCapability>,
}

impl std::fmt::Debug for VerifiedImportCustody {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VerifiedImportCustody")
            .field("bundle_id", &self.bundle_id)
            .field("key_ref", &self.key_ref)
            .field("manifest_integrity_ref", &self.manifest_integrity_ref)
            .field("payload_integrity_refs", &self.payload_integrity_refs)
            .field("household_id", &self.household_id)
            .field("target_device_id", &self.target_device_id)
            .field("migration_ref", &self.migration_ref)
            .field("preflight", &self.preflight)
            .finish_non_exhaustive()
    }
}

impl VerifiedImportCustody {
    pub(crate) fn validate_for_binding(
        &self,
        bundle: &contracts::ExportImportRecoveryBundle,
        authority: HouseholdAuthorityRuntimeEffectAuthorization,
    ) -> Result<(), ImportBindingError> {
        if self.bundle_id != bundle.manifest.bundle_id
            || self.household_id != bundle.manifest.source_household_id
        {
            return Err(ImportBindingError::HouseholdMismatch);
        }
        let target_device_id = self
            .target_device_id
            .as_ref()
            .ok_or(ImportBindingError::MissingLocalContext)?;
        authority
            .consume_for_data_custody(
                ocentra_family_identity_core::household_authority::HouseholdAuthorityAction::ImportRestoreData,
                self.household_id.as_str(),
                Some(target_device_id.as_str()),
                Some(self.authority_generation),
            )
            .map_err(|_| ImportBindingError::AuthorityProofMismatch)?;
        if self.key_ref != bundle.manifest.key_ref
            || self.manifest_integrity_ref != bundle.manifest.manifest_integrity_ref
        {
            return Err(ImportBindingError::IntegrityBindingMismatch);
        }
        if self.migration_ref != bundle.manifest.migration_ref {
            return Err(ImportBindingError::MigrationMismatch);
        }
        if self.payload_integrity_refs.len() != bundle.sections.len()
            || bundle.sections.iter().any(|section| {
                !self
                    .payload_integrity_refs
                    .iter()
                    .any(|(data_class, integrity_ref)| {
                        data_class == &section.data_class
                            && integrity_ref == &section.payload_integrity_ref
                    })
            })
        {
            return Err(ImportBindingError::IntegrityBindingMismatch);
        }
        if self.preflight.accepted_sections.is_empty()
            && self.preflight.rejected_sections.is_empty()
        {
            return Err(ImportBindingError::SectionBindingMismatch);
        }
        Ok(())
    }

    pub(crate) fn from_verified_parts(
        bundle_id: contracts::ExportImportBundleId,
        key_ref: contracts::ExportImportKeyRef,
        manifest_integrity_ref: contracts::ExportImportIntegrityRef,
        payload_integrity_refs: Vec<(
            contracts::ExportImportDataClass,
            contracts::ExportImportIntegrityRef,
        )>,
        household_id: contracts::ExportImportHouseholdId,
        target_device_id: Option<contracts::ExportImportDeviceId>,
        authority_generation: u64,
        migration_ref: Option<contracts::ExportImportMigrationRef>,
        preflight: contracts::ExportImportImportPreflight,
        capability: Box<dyn RestoreExecutionCapability>,
    ) -> Self {
        Self {
            bundle_id,
            key_ref,
            manifest_integrity_ref,
            payload_integrity_refs,
            household_id,
            target_device_id,
            authority_generation,
            migration_ref,
            preflight,
            capability,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        contracts::ExportImportBundleId,
        contracts::ExportImportKeyRef,
        contracts::ExportImportIntegrityRef,
        Vec<(
            contracts::ExportImportDataClass,
            contracts::ExportImportIntegrityRef,
        )>,
        contracts::ExportImportHouseholdId,
        Option<contracts::ExportImportDeviceId>,
        Option<contracts::ExportImportMigrationRef>,
        contracts::ExportImportImportPreflight,
        Box<dyn RestoreExecutionCapability>,
    ) {
        (
            self.bundle_id,
            self.key_ref,
            self.manifest_integrity_ref,
            self.payload_integrity_refs,
            self.household_id,
            self.target_device_id,
            self.migration_ref,
            self.preflight,
            self.capability,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportBindingError {
    AuthorityActionRequired,
    HouseholdMismatch,
    AuthorityProofMismatch,
    MigrationMismatch,
    IntegrityBindingMismatch,
    SectionBindingMismatch,
    MissingLocalContext,
    CapabilityUnavailable,
}
