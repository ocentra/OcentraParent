use ocentra_schema::export_import_backup_recovery as contracts;

use super::export_import_backup_recovery_import_rejection::RejectedPreflightInput;

pub(super) fn accepted_preflight(
    state: contracts::ExportImportPreflightState,
    migration_state: contracts::ExportImportMigrationState,
    accepted_sections: Vec<contracts::ExportImportSectionDecision>,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
    tombstones_preserved: bool,
) -> contracts::ExportImportImportPreflight {
    contracts::ExportImportImportPreflight {
        state,
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        manifest_integrity_verified: true,
        payload_integrity_verified: true,
        local_truth_mutated: false,
        tombstones_preserved,
        duplicate_device_detected: false,
        migration_state,
        accepted_sections,
        rejected_sections,
        no_default_support_decrypt: true,
    }
}

pub(super) fn rejected_preflight(
    input: RejectedPreflightInput,
) -> contracts::ExportImportImportPreflight {
    contracts::ExportImportImportPreflight {
        state: input.state,
        schema_version_supported: input.schema_version_supported,
        household_binding_match: input.household_binding_match,
        key_available: input.key_available,
        manifest_integrity_verified: input.integrity_ok,
        payload_integrity_verified: input.integrity_ok,
        local_truth_mutated: false,
        tombstones_preserved: input.tombstones_preserved,
        duplicate_device_detected: input.duplicate_device_detected,
        migration_state: input.migration_state,
        accepted_sections: Vec::new(),
        rejected_sections: input.rejected_sections,
        no_default_support_decrypt: true,
    }
}
