use ocentra_schema::export_import_backup_recovery as contracts;

use super::ImportBundleContext;

pub(super) struct RejectedPreflightInput {
    pub state: contracts::ExportImportPreflightState,
    pub migration_state: contracts::ExportImportMigrationState,
    pub schema_version_supported: bool,
    pub household_binding_match: bool,
    pub key_available: bool,
    pub integrity_ok: bool,
    pub duplicate_device_detected: bool,
    pub rejected_sections: Vec<contracts::ExportImportSectionDecision>,
}

pub(super) fn import_preflight_rejection(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    reject_schema_version(bundle, context)
        .or_else(|| reject_wrong_household(bundle, context))
        .or_else(|| reject_wrong_key(bundle, context))
        .or_else(|| reject_corrupt_bundle(bundle, context))
        .or_else(|| reject_migration_unsupported(bundle, context))
        .or_else(|| reject_duplicate_device(bundle, context))
}

fn reject_schema_version(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let schema_version_supported = context
        .supported_schema_versions
        .iter()
        .any(|version| version == &bundle.manifest.schema_version);
    if schema_version_supported {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::SchemaVersionInvalid,
        migration_state: contracts::ExportImportMigrationState::NotRequired,
        schema_version_supported,
        household_binding_match: true,
        key_available: true,
        integrity_ok: true,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_wrong_household(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    if bundle.manifest.source_household_id == context.local_household_id {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::HouseholdMismatch,
        migration_state: contracts::ExportImportMigrationState::NotRequired,
        schema_version_supported: true,
        household_binding_match: false,
        key_available: true,
        integrity_ok: true,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_wrong_key(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let key_available = context
        .available_key_refs
        .iter()
        .any(|key_ref| key_ref == &bundle.manifest.key_ref);
    if key_available {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::KeyUnavailable,
        migration_state: contracts::ExportImportMigrationState::NotRequired,
        schema_version_supported: true,
        household_binding_match: true,
        key_available,
        integrity_ok: true,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_corrupt_bundle(
    _bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let payload_integrity_verified = context.payload_integrity_failures.is_empty();
    if context.manifest_integrity_ok && payload_integrity_verified {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::BundleCorrupt,
        migration_state: contracts::ExportImportMigrationState::NotRequired,
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        integrity_ok: false,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_migration_unsupported(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let migration_state = import_preflight_migration_state(bundle, context);
    if migration_state != contracts::ExportImportMigrationState::RequiredUnsupported {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::MigrationUnsupported,
        migration_state,
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        integrity_ok: true,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_duplicate_device(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let duplicate_device_detected = import_preflight_duplicate_device_detected(bundle, context);
    if !duplicate_device_detected {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::DeviceDuplicate,
        migration_state: import_preflight_migration_state(bundle, context),
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        integrity_ok: true,
        duplicate_device_detected: true,
        rejected_sections: vec![contracts::ExportImportSectionDecision {
            data_class: contracts::ExportImportDataClass::DeviceRegistry,
            state: contracts::ExportImportSectionDecisionState::DuplicateDevice,
            reason: "Existing local device identity would be duplicated by restore.".to_string(),
        }],
    })
}

pub(super) fn import_preflight_migration_state(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> contracts::ExportImportMigrationState {
    if bundle.manifest.product_version == context.local_product_version {
        contracts::ExportImportMigrationState::NotRequired
    } else if context.migration_supported {
        contracts::ExportImportMigrationState::RequiredSupported
    } else {
        contracts::ExportImportMigrationState::RequiredUnsupported
    }
}

fn import_preflight_duplicate_device_detected(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> bool {
    bundle
        .manifest
        .source_device_id
        .as_ref()
        .map(|source_device_id| {
            context
                .known_device_ids
                .iter()
                .any(|known_device_id| known_device_id == source_device_id)
                && context.target_device_id.as_ref() != Some(source_device_id)
        })
        .unwrap_or(false)
}
