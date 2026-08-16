use ocentra_schema::export_import_backup_recovery as contracts;

use ocentra_family_identity_core::household_authority::HouseholdAuthorityAction;
use ocentra_family_identity_core::household_authority_proof::VerifiedHouseholdAuthority;

#[path = "export_import_backup_recovery_build.rs"]
mod export_import_backup_recovery_build;
#[path = "export_import_backup_recovery_import.rs"]
mod export_import_backup_recovery_import;
#[path = "export_import_backup_recovery_restore.rs"]
mod export_import_backup_recovery_restore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportBundleBuildRequest {
    pub bundle_id: contracts::ExportImportBundleId,
    pub product_version: contracts::ExportImportProductVersion,
    pub created_at: contracts::ExportImportTimestamp,
    pub household: contracts::ExportImportHouseholdReference,
    pub source_device_id: Option<contracts::ExportImportDeviceId>,
    pub bundle_type: contracts::ExportImportBundleType,
    pub key_ref: contracts::ExportImportKeyRef,
    pub manifest_integrity_ref: Option<contracts::ExportImportIntegrityRef>,
    pub tombstone_cursor: Option<contracts::ExportImportTombstoneCursor>,
    pub retention_notes: Vec<String>,
    pub proof_tier: contracts::ExportImportProofTier,
    pub migration_ref: Option<contracts::ExportImportMigrationRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPayloadSectionInput {
    pub data_class: contracts::ExportImportDataClass,
    pub payload_ref: contracts::ExportImportPayloadRef,
    pub payload_integrity_ref: Option<contracts::ExportImportIntegrityRef>,
    pub encrypted: bool,
    pub retention_state: contracts::ExportImportSectionRetentionState,
    pub support_default_decryptable: bool,
    pub included_in_human_summary: bool,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportHumanSummaryInput {
    pub headline: String,
    pub excluded_data_classes: Vec<contracts::ExportImportDataClass>,
    pub raw_payload_redacted: bool,
    pub support_safe: bool,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportBundleBuildError {
    EmptySections,
    MissingManifestIntegrity,
    MissingPayloadIntegrity(contracts::ExportImportDataClass),
    SectionNotEncrypted(contracts::ExportImportDataClass),
    SupportDefaultDecryptForbidden(contracts::ExportImportDataClass),
    DuplicateDataClass(contracts::ExportImportDataClass),
    SummaryMustBeRedacted,
    SummaryMustBeSupportSafe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportBundleContext {
    pub local_household_id: contracts::ExportImportHouseholdId,
    pub local_product_version: contracts::ExportImportProductVersion,
    pub available_key_refs: Vec<contracts::ExportImportKeyRef>,
    pub supported_schema_versions: Vec<String>,
    pub blocked_restore_data_classes: Vec<contracts::ExportImportDataClass>,
    pub known_device_ids: Vec<contracts::ExportImportDeviceId>,
    pub target_device_id: Option<contracts::ExportImportDeviceId>,
    pub migration_supported: bool,
    pub manifest_integrity_ok: bool,
    pub payload_integrity_failures: Vec<contracts::ExportImportDataClass>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreApplyRequest {
    pub confirmed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreExecutorOutcome {
    Applied,
    Partial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreExecutorFailure {
    Unavailable,
}

pub trait RestoreExecutor {
    fn execute_restore(
        &mut self,
        preflight: &contracts::ExportImportImportPreflight,
        request: &RestoreApplyRequest,
    ) -> Result<RestoreExecutorOutcome, RestoreExecutorFailure>;
}

#[derive(Debug, Default)]
pub struct UnavailableRestoreExecutor;

impl RestoreExecutor for UnavailableRestoreExecutor {
    fn execute_restore(
        &mut self,
        _preflight: &contracts::ExportImportImportPreflight,
        _request: &RestoreApplyRequest,
    ) -> Result<RestoreExecutorOutcome, RestoreExecutorFailure> {
        Err(RestoreExecutorFailure::Unavailable)
    }
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

pub fn apply_restore(
    preflight: &contracts::ExportImportImportPreflight,
    request: &RestoreApplyRequest,
) -> contracts::ExportImportRestoreApplyResult {
    export_import_backup_recovery_restore::blocked_restore(preflight, request)
}

pub fn apply_restore_with_parent_authority(
    preflight: &contracts::ExportImportImportPreflight,
    context: &ImportBundleContext,
    request: &RestoreApplyRequest,
    authority: &VerifiedHouseholdAuthority,
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

pub fn apply_restore_with_parent_authority_and_executor(
    preflight: &contracts::ExportImportImportPreflight,
    context: &ImportBundleContext,
    request: &RestoreApplyRequest,
    authority: &VerifiedHouseholdAuthority,
    executor: &mut impl RestoreExecutor,
) -> contracts::ExportImportRestoreApplyResult {
    let Some(identity_binding) = authority.identity_binding() else {
        return export_import_backup_recovery_restore::blocked_restore(preflight, request);
    };
    let Some(target_device_id) = context.target_device_id.as_ref() else {
        return export_import_backup_recovery_restore::blocked_restore(preflight, request);
    };
    if !export_import_backup_recovery_restore::preflight_is_applicable(preflight)
        || !request.confirmed
        || authority.input().action != HouseholdAuthorityAction::PairChildDevice
        || identity_binding.household_id != context.local_household_id.as_str()
        || identity_binding.target_device_id != target_device_id.as_str()
    {
        return export_import_backup_recovery_restore::blocked_restore(preflight, request);
    }

    let Ok(outcome) = executor.execute_restore(preflight, request) else {
        return export_import_backup_recovery_restore::blocked_restore(preflight, request);
    };
    export_import_backup_recovery_restore::apply_restore_after_execution(preflight, outcome)
}
