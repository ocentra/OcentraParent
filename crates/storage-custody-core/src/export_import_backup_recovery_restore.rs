use ocentra_schema::export_import_backup_recovery as contracts;

use super::RestoreApplyRequest;

pub(super) fn apply_restore(
    preflight: &contracts::ExportImportImportPreflight,
    request: &RestoreApplyRequest,
) -> contracts::ExportImportRestoreApplyResult {
    let state = restore_state(preflight.state, request.confirmed);

    contracts::ExportImportRestoreApplyResult {
        explicit_confirmation_required: state
            == contracts::ExportImportRestoreApplyState::ApplyPending,
        local_truth_authoritative: true,
        tombstones_preserved: preflight.tombstones_preserved,
        idempotent: matches!(
            state,
            contracts::ExportImportRestoreApplyState::Applied
                | contracts::ExportImportRestoreApplyState::Partial
                | contracts::ExportImportRestoreApplyState::ApplyPending
        ),
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

fn restore_state(
    preflight_state: contracts::ExportImportPreflightState,
    confirmed: bool,
) -> contracts::ExportImportRestoreApplyState {
    match (preflight_state, confirmed) {
        (contracts::ExportImportPreflightState::AcceptedPreview, true) => {
            contracts::ExportImportRestoreApplyState::Applied
        }
        (contracts::ExportImportPreflightState::PartialPreview, true) => {
            contracts::ExportImportRestoreApplyState::Partial
        }
        (contracts::ExportImportPreflightState::AcceptedPreview, false)
        | (contracts::ExportImportPreflightState::PartialPreview, false) => {
            contracts::ExportImportRestoreApplyState::ApplyPending
        }
        (contracts::ExportImportPreflightState::HouseholdMismatch, _) => {
            contracts::ExportImportRestoreApplyState::WrongHousehold
        }
        (contracts::ExportImportPreflightState::KeyUnavailable, _) => {
            contracts::ExportImportRestoreApplyState::WrongKey
        }
        (contracts::ExportImportPreflightState::BundleCorrupt, _) => {
            contracts::ExportImportRestoreApplyState::Corrupt
        }
        _ => contracts::ExportImportRestoreApplyState::Blocked,
    }
}
