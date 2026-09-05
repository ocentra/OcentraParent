use ocentra_schema::export_import_backup_recovery as contracts;

use ocentra_family_identity_core::household_authority::HouseholdAuthorityAction;
use ocentra_family_identity_core::household_authority_runtime_composer::HouseholdAuthorityRuntimeEffectAuthorization;

#[path = "export_import_backup_recovery_backup_job_state.rs"]
pub mod export_import_backup_recovery_backup_job_state;
#[path = "export_import_backup_recovery_backup_schedule.rs"]
pub mod export_import_backup_recovery_backup_schedule;
#[path = "export_import_backup_recovery_build.rs"]
mod export_import_backup_recovery_build;
#[path = "export_import_backup_recovery_bundle_preflight_binding.rs"]
pub mod export_import_backup_recovery_bundle_preflight_binding;
#[path = "export_import_backup_recovery_compensation.rs"]
pub mod export_import_backup_recovery_compensation;
#[path = "export_import_backup_recovery_import.rs"]
mod export_import_backup_recovery_import;
#[path = "export_import_backup_recovery_migration.rs"]
mod export_import_backup_recovery_migration;
#[path = "export_import_backup_recovery_migration_execution.rs"]
pub mod export_import_backup_recovery_migration_execution;
#[path = "export_import_backup_recovery_restore.rs"]
mod export_import_backup_recovery_restore;
#[path = "export_import_backup_recovery_restore_execution_plan.rs"]
pub mod export_import_backup_recovery_restore_execution_plan;

#[path = "../tests/unit/export_import_backup_recovery_private.rs"]
mod export_import_backup_recovery_tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupRequestInput {
    pub bundle_id: contracts::ExportImportBundleId,
    pub cadence: contracts::ExportImportBackupCadence,
    pub household_id: contracts::ExportImportHouseholdId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupRequestError {
    AuthorityActionRequired,
    HouseholdMismatch,
}

const BACKUP_SCHEDULED_MANUAL_REQUIRED_NOTE: &str =
    "Scheduled backup remains manual-required until a trusted scheduler and provider runtime exist.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportBundleBuildRequest {
    pub(crate) bundle_id: contracts::ExportImportBundleId,
    pub(crate) product_version: contracts::ExportImportProductVersion,
    pub(crate) created_at: contracts::ExportImportTimestamp,
    pub(crate) household: contracts::ExportImportHouseholdReference,
    pub(crate) source_device_id: Option<contracts::ExportImportDeviceId>,
    pub(crate) bundle_type: contracts::ExportImportBundleType,
    pub(crate) key_ref: contracts::ExportImportKeyRef,
    pub(crate) manifest_integrity_ref: Option<contracts::ExportImportIntegrityRef>,
    pub(crate) tombstone_cursor: Option<contracts::ExportImportTombstoneCursor>,
    pub(crate) retention_notes: Vec<String>,
    pub(crate) proof_tier: contracts::ExportImportProofTier,
    pub(crate) migration_ref: Option<contracts::ExportImportMigrationRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPayloadSectionInput {
    pub(crate) data_class: contracts::ExportImportDataClass,
    pub(crate) payload_ref: contracts::ExportImportPayloadRef,
    pub(crate) payload_integrity_ref: Option<contracts::ExportImportIntegrityRef>,
    pub(crate) retention_state: contracts::ExportImportSectionRetentionState,
    pub(crate) included_in_human_summary: bool,
    pub(crate) notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportHumanSummaryInput {
    pub(crate) headline: String,
    pub(crate) excluded_data_classes: Vec<contracts::ExportImportDataClass>,
    pub(crate) raw_payload_redacted: bool,
    pub(crate) support_safe: bool,
    pub(crate) notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportBundleBuildError {
    EncryptionCustodyUnavailable,
}

/// Import context is assembled by the storage owner, never from a wire or UI
/// payload. No public constructor exists until durable key, integrity, and
/// tombstone-cursor custody are available, so callers cannot mint an accepted
/// preview from booleans or advance revocation state with an imported cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportBundleContext {
    pub(crate) local_household_id: contracts::ExportImportHouseholdId,
    pub(crate) local_product_version: contracts::ExportImportProductVersion,
    pub(crate) available_key_refs: Vec<contracts::ExportImportKeyRef>,
    pub(crate) supported_schema_versions: Vec<String>,
    pub(crate) blocked_restore_data_classes: Vec<contracts::ExportImportDataClass>,
    pub(crate) known_device_ids: Vec<contracts::ExportImportDeviceId>,
    pub(crate) target_device_id: Option<contracts::ExportImportDeviceId>,
    pub(crate) migration_supported: bool,
    pub(crate) manifest_integrity_ok: bool,
    pub(crate) payload_integrity_failures: Vec<contracts::ExportImportDataClass>,
    pub(crate) current_tombstone_cursor: Option<contracts::ExportImportTombstoneCursor>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreApplyRequest {
    pub(crate) confirmed: bool,
}

pub fn derive_export_bundle(
    request: ExportBundleBuildRequest,
    sections: Vec<ExportPayloadSectionInput>,
    summary: ExportHumanSummaryInput,
) -> Result<contracts::ExportImportRecoveryBundle, ExportBundleBuildError> {
    export_import_backup_recovery_build::derive_export_bundle(request, sections, summary)
}

pub fn run_import_preflight(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> contracts::ExportImportImportPreflight {
    export_import_backup_recovery_import::run_import_preflight(bundle, context)
}

pub fn migration_execution_readiness(
    bound: &export_import_backup_recovery_bundle_preflight_binding::BoundImportPreflight,
) -> contracts::ExportImportMigrationExecutionReadiness {
    export_import_backup_recovery_migration::migration_execution_readiness(
        bound.execution_binding().migration_ref().cloned(),
        bound.preflight(),
    )
}

pub fn authorize_backup_request(
    input: BackupRequestInput,
    authority: HouseholdAuthorityRuntimeEffectAuthorization,
) -> Result<contracts::ExportImportBackupRequestState, BackupRequestError> {
    authority
        .consume_for_data_custody(
            HouseholdAuthorityAction::ExportDeleteData,
            input.household_id.as_str(),
            None,
            None,
        )
        .map_err(|_error| BackupRequestError::HouseholdMismatch)?;

    let scheduled = input.cadence == contracts::ExportImportBackupCadence::Scheduled;
    Ok(contracts::ExportImportBackupRequestState {
        bundle_id: input.bundle_id,
        cadence: input.cadence,
        state: if scheduled {
            contracts::ExportImportBackupState::ManualRequired
        } else {
            contracts::ExportImportBackupState::Authorized
        },
        explicit_confirmation_required: true,
        provider_runtime_claimed: false,
        manual_required_note: scheduled.then(|| BACKUP_SCHEDULED_MANUAL_REQUIRED_NOTE.to_string()),
    })
}

/// Restore application is unavailable until a storage owner can bind the
/// operation to a durable, reread-at-apply tombstone cursor. The serde-shaped
/// preflight and caller-held [`ImportBundleContext`] are deliberately ignored:
/// neither is currentness authority, and no restore side effect is permitted
/// through this dead seam.
pub fn apply_restore(
    _preflight: &contracts::ExportImportImportPreflight,
    _request: &RestoreApplyRequest,
) -> contracts::ExportImportRestoreApplyResult {
    export_import_backup_recovery_restore::blocked_restore()
}
