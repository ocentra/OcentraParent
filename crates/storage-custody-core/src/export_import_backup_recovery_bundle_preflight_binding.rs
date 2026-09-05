use ocentra_family_identity_core::household_authority_runtime_composer::HouseholdAuthorityRuntimeEffectAuthorization;
use ocentra_schema::export_import_backup_recovery as contracts;
#[path = "export_import_backup_recovery_bundle_preflight_binding_custody_port.rs"]
pub mod custody_port;
#[path = "export_import_backup_recovery_bundle_preflight_binding_execution.rs"]
pub mod execution_binding;

use self::custody_port::{
    ImportBindingError, ImportCustodyCapabilityPort, VerifiedImportCustodyParts,
};
use self::execution_binding::{RestoreExecutionBinding, RestoreExecutionBindingParts};

#[derive(Debug)]
pub struct BoundImportPreflight {
    bundle_id: contracts::ExportImportBundleId,
    preflight: contracts::ExportImportImportPreflight,
    execution_binding: RestoreExecutionBinding,
}

impl BoundImportPreflight {
    pub(crate) fn bundle_id(&self) -> &contracts::ExportImportBundleId {
        &self.bundle_id
    }

    pub(crate) fn preflight(&self) -> &contracts::ExportImportImportPreflight {
        &self.preflight
    }

    pub(crate) fn execution_binding(&self) -> &RestoreExecutionBinding {
        &self.execution_binding
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        contracts::ExportImportBundleId,
        contracts::ExportImportImportPreflight,
        RestoreExecutionBinding,
    ) {
        (self.bundle_id, self.preflight, self.execution_binding)
    }
}

/// Builds an owner-bound preflight from opaque family authority and custody
/// capability results. The returned value intentionally does not expose a
/// public constructor, so a serde-shaped preflight cannot be promoted into a
/// restore plan by a caller.
pub fn bind_import_preflight(
    bundle: &contracts::ExportImportRecoveryBundle,
    authority: HouseholdAuthorityRuntimeEffectAuthorization,
    custody: &dyn ImportCustodyCapabilityPort,
) -> Result<BoundImportPreflight, ImportBindingError> {
    // Restore/import is a distinct state-changing household action. ExportDeleteData
    // and PairChildDevice must never be accepted as substitutes for it.
    let verified = custody.verify_import_bundle(bundle, &authority)?;
    verified.validate_for_binding(bundle, authority)?;
    let VerifiedImportCustodyParts {
        bundle_id: verified_bundle_id,
        key_ref,
        manifest_integrity_ref,
        payload_integrity_refs,
        household_id: _verified_household_id,
        target_device_id,
        migration_ref,
        preflight,
        capability,
    } = verified.into_parts();
    let execution_binding = RestoreExecutionBinding::from_parts(RestoreExecutionBindingParts {
        bundle_id: verified_bundle_id.clone(),
        key_ref,
        manifest_integrity_ref,
        payload_integrity_refs,
        target_device_id: target_device_id.ok_or(ImportBindingError::MissingLocalContext)?,
        accepted_sections: preflight.accepted_sections.clone(),
        rejected_sections: preflight.rejected_sections.clone(),
        tombstones_preserved: preflight.tombstones_preserved,
        no_resurrection: preflight.tombstones_preserved && !preflight.local_truth_mutated,
        migration_ref,
        migration_state: preflight.migration_state,
        capability,
    });

    Ok(BoundImportPreflight {
        bundle_id: verified_bundle_id,
        preflight,
        execution_binding,
    })
}
