use ocentra_schema::export_import_backup_recovery as contracts;

use super::{RestoreApplyRequest, RestoreExecutorOutcome};

pub(super) fn apply_restore_after_execution(
    preflight: &contracts::ExportImportImportPreflight,
    outcome: RestoreExecutorOutcome,
) -> contracts::ExportImportRestoreApplyResult {
    let state = match outcome {
        RestoreExecutorOutcome::Applied => contracts::ExportImportRestoreApplyState::Applied,
        RestoreExecutorOutcome::Partial => contracts::ExportImportRestoreApplyState::Partial,
    };
    contracts::ExportImportRestoreApplyResult {
        explicit_confirmation_required: false,
        local_truth_authoritative: true,
        tombstones_preserved: preflight.tombstones_preserved,
        idempotent: true,
        accepted_sections: preflight.accepted_sections.clone(),
        rejected_sections: preflight.rejected_sections.clone(),
        duplicates_created: false,
        no_default_support_decrypt: preflight.no_default_support_decrypt,
        state,
    }
}

pub(super) fn blocked_restore(
    preflight: &contracts::ExportImportImportPreflight,
    _request: &RestoreApplyRequest,
) -> contracts::ExportImportRestoreApplyResult {
    contracts::ExportImportRestoreApplyResult {
        state: contracts::ExportImportRestoreApplyState::Blocked,
        explicit_confirmation_required: true,
        local_truth_authoritative: true,
        tombstones_preserved: preflight.tombstones_preserved,
        idempotent: false,
        accepted_sections: Vec::new(),
        rejected_sections: preflight.rejected_sections.clone(),
        duplicates_created: false,
        no_default_support_decrypt: preflight.no_default_support_decrypt,
    }
}

pub(super) fn preflight_is_applicable(preflight: &contracts::ExportImportImportPreflight) -> bool {
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
