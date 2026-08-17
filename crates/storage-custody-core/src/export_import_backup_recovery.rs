use ocentra_schema::export_import_backup_recovery as contracts;

use ocentra_family_identity_core::household_authority::HouseholdAuthorityAction;
use ocentra_family_identity_core::household_authority_proof::CurrentVerifiedHouseholdAuthority;

#[path = "export_import_backup_recovery_build.rs"]
mod export_import_backup_recovery_build;
#[path = "export_import_backup_recovery_import.rs"]
mod export_import_backup_recovery_import;
#[path = "export_import_backup_recovery_restore.rs"]
mod export_import_backup_recovery_restore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExportBundleBuildRequest {
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
pub(crate) struct ExportPayloadSectionInput {
    pub(crate) data_class: contracts::ExportImportDataClass,
    pub(crate) payload_ref: contracts::ExportImportPayloadRef,
    pub(crate) payload_integrity_ref: Option<contracts::ExportImportIntegrityRef>,
    pub(crate) encrypted: bool,
    pub(crate) retention_state: contracts::ExportImportSectionRetentionState,
    pub(crate) support_default_decryptable: bool,
    pub(crate) included_in_human_summary: bool,
    pub(crate) notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExportHumanSummaryInput {
    pub(crate) headline: String,
    pub(crate) excluded_data_classes: Vec<contracts::ExportImportDataClass>,
    pub(crate) raw_payload_redacted: bool,
    pub(crate) support_safe: bool,
    pub(crate) notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExportBundleBuildError {
    EmptySections,
    MissingManifestIntegrity,
    MissingPayloadIntegrity(contracts::ExportImportDataClass),
    SectionNotEncrypted(contracts::ExportImportDataClass),
    SupportDefaultDecryptForbidden(contracts::ExportImportDataClass),
    DuplicateDataClass(contracts::ExportImportDataClass),
    SummaryMustBeRedacted,
    SummaryMustBeSupportSafe,
}

/// Import context is assembled by the storage owner, never from a wire or UI
/// payload. No public constructor exists until durable key and integrity
/// custody is available, so callers cannot mint an accepted preview from
/// booleans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImportBundleContext {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreApplyRequest {
    pub confirmed: bool,
}

/// An executor receipt is an internal post-side-effect result. Keeping it
/// non-cloneable and crate-private prevents callers from manufacturing or
/// replaying a restore result outside the owner boundary.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RestoreExecutorReceipt {
    pub(crate) execution_ref: String,
    pub(crate) state: contracts::ExportImportRestoreApplyState,
    pub(crate) applied_sections: Vec<contracts::ExportImportSectionDecision>,
    pub(crate) rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    pub(crate) idempotent: bool,
    pub(crate) tombstones_preserved: bool,
    pub(crate) duplicates_created: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RestoreExecutorFailure {
    Unavailable,
}

pub(crate) trait RestoreExecutor {
    fn execute_restore(
        &mut self,
        preflight: &contracts::ExportImportImportPreflight,
        request: &RestoreApplyRequest,
    ) -> Result<RestoreExecutorReceipt, RestoreExecutorFailure>;
}

#[derive(Debug, Default)]
pub(crate) struct UnavailableRestoreExecutor;

impl RestoreExecutor for UnavailableRestoreExecutor {
    fn execute_restore(
        &mut self,
        _preflight: &contracts::ExportImportImportPreflight,
        _request: &RestoreApplyRequest,
    ) -> Result<RestoreExecutorReceipt, RestoreExecutorFailure> {
        Err(RestoreExecutorFailure::Unavailable)
    }
}

pub(crate) fn derive_export_bundle(
    request: ExportBundleBuildRequest,
    sections: Vec<ExportPayloadSectionInput>,
    summary: ExportHumanSummaryInput,
) -> Result<contracts::ExportImportRecoveryBundle, ExportBundleBuildError> {
    export_import_backup_recovery_build::derive_export_bundle(request, sections, summary)
}

pub(crate) fn run_import_preflight(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> contracts::ExportImportImportPreflight {
    export_import_backup_recovery_import::run_import_preflight(bundle, context)
}

pub fn apply_restore(
    preflight: &contracts::ExportImportImportPreflight,
    request: &RestoreApplyRequest,
) -> contracts::ExportImportRestoreApplyResult {
    export_import_backup_recovery_restore::blocked_restore(preflight, request)
}

pub(crate) fn apply_restore_with_parent_authority(
    preflight: &contracts::ExportImportImportPreflight,
    context: &ImportBundleContext,
    request: &RestoreApplyRequest,
    authority: CurrentVerifiedHouseholdAuthority,
) -> contracts::ExportImportRestoreApplyResult {
    let mut executor = UnavailableRestoreExecutor;
    apply_restore_with_parent_authority_and_executor(
        preflight,
        context,
        request,
        authority,
        &mut executor,
    )
}

pub(crate) fn apply_restore_with_parent_authority_and_executor(
    preflight: &contracts::ExportImportImportPreflight,
    context: &ImportBundleContext,
    request: &RestoreApplyRequest,
    authority: CurrentVerifiedHouseholdAuthority,
    executor: &mut impl RestoreExecutor,
) -> contracts::ExportImportRestoreApplyResult {
    let identity_binding = authority.identity_binding();
    let Some(target_device_id) = context.target_device_id.as_ref() else {
        return export_import_backup_recovery_restore::blocked_restore(preflight, request);
    };
    if !export_import_backup_recovery_restore::preflight_is_applicable(preflight)
        || !request.confirmed
        || authority.input().action != HouseholdAuthorityAction::PairChildDevice
        || identity_binding.household_id() != context.local_household_id.as_str()
        || identity_binding.target_device_id() != target_device_id.as_str()
    {
        return export_import_backup_recovery_restore::blocked_restore(preflight, request);
    }

    let Ok(receipt) = executor.execute_restore(preflight, request) else {
        return export_import_backup_recovery_restore::blocked_restore(preflight, request);
    };
    let Some(result) =
        export_import_backup_recovery_restore::apply_restore_after_execution(preflight, receipt)
    else {
        return export_import_backup_recovery_restore::blocked_restore(preflight, request);
    };
    result
}
