use ocentra_schema::export_import_backup_recovery as contracts;

const MIGRATION_REFERENCE_REQUIRED_NOTE: &str =
    "Migration is manual-required because the recovery bundle has no bound migration reference.";
const MIGRATION_PATH_REQUIRED_NOTE: &str =
    "Migration is manual-required until the bundle migration path is supported.";
const MIGRATION_DURABLE_OWNER_REQUIRED_NOTE: &str = "Migration is manual-required until a durable migration store and executor own apply, rollback, and idempotency receipts.";
const MIGRATION_PREVIEW_REQUIRED_NOTE: &str = "Migration is manual-required until import preview proves integrity, household, key, and tombstone safety.";

pub(super) fn migration_execution_readiness(
    bundle: &contracts::ExportImportRecoveryBundle,
    preflight: &contracts::ExportImportImportPreflight,
) -> contracts::ExportImportMigrationExecutionReadiness {
    let migration_ref = bundle.manifest.migration_ref.clone();
    if preflight.migration_state == contracts::ExportImportMigrationState::RequiredUnsupported {
        return manual_required_readiness(
            migration_ref,
            preflight,
            contracts::ExportImportMigrationExecutionDependency::SupportedMigrationPath,
            MIGRATION_PATH_REQUIRED_NOTE,
        );
    }
    if !preflight_is_safe_for_migration(preflight) {
        return manual_required_readiness(
            migration_ref,
            preflight,
            contracts::ExportImportMigrationExecutionDependency::SupportedMigrationPath,
            MIGRATION_PREVIEW_REQUIRED_NOTE,
        );
    }
    let (state, required_dependency, manual_required_note) = match preflight.migration_state {
        contracts::ExportImportMigrationState::NotRequired => (
            contracts::ExportImportMigrationExecutionState::NotRequired,
            None,
            None,
        ),
        contracts::ExportImportMigrationState::RequiredUnsupported => (
            contracts::ExportImportMigrationExecutionState::ManualRequired,
            Some(
                contracts::ExportImportMigrationExecutionDependency::SupportedMigrationPath,
            ),
            Some(MIGRATION_PATH_REQUIRED_NOTE.to_string()),
        ),
        _ if migration_ref.is_none() => (
            contracts::ExportImportMigrationExecutionState::ManualRequired,
            Some(
                contracts::ExportImportMigrationExecutionDependency::BundleMigrationReference,
            ),
            Some(MIGRATION_REFERENCE_REQUIRED_NOTE.to_string()),
        ),
        contracts::ExportImportMigrationState::RequiredSupported => (
            contracts::ExportImportMigrationExecutionState::ManualRequired,
            Some(
                contracts::ExportImportMigrationExecutionDependency::DurableMigrationStoreAndExecutor,
            ),
            Some(MIGRATION_DURABLE_OWNER_REQUIRED_NOTE.to_string()),
        ),
    };

    contracts::ExportImportMigrationExecutionReadiness {
        state,
        migration_ref,
        required_dependency,
        local_truth_mutated: false,
        tombstones_preserved: preflight.tombstones_preserved,
        no_default_support_decrypt: true,
        manual_required_note,
    }
}

fn preflight_is_safe_for_migration(preflight: &contracts::ExportImportImportPreflight) -> bool {
    matches!(
        preflight.state,
        contracts::ExportImportPreflightState::AcceptedPreview
            | contracts::ExportImportPreflightState::PartialPreview
    ) && preflight.schema_version_supported
        && preflight.household_binding_match
        && preflight.key_available
        && preflight.manifest_integrity_verified
        && preflight.payload_integrity_verified
        && !preflight.local_truth_mutated
        && preflight.tombstones_preserved
        && !preflight.duplicate_device_detected
}

fn manual_required_readiness(
    migration_ref: Option<contracts::ExportImportMigrationRef>,
    preflight: &contracts::ExportImportImportPreflight,
    dependency: contracts::ExportImportMigrationExecutionDependency,
    note: &str,
) -> contracts::ExportImportMigrationExecutionReadiness {
    contracts::ExportImportMigrationExecutionReadiness {
        state: contracts::ExportImportMigrationExecutionState::ManualRequired,
        migration_ref,
        required_dependency: Some(dependency),
        local_truth_mutated: false,
        tombstones_preserved: preflight.tombstones_preserved,
        no_default_support_decrypt: preflight.no_default_support_decrypt,
        manual_required_note: Some(note.to_string()),
    }
}
