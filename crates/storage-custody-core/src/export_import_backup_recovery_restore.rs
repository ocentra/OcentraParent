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
        (contracts::ExportImportPreflightState::RejectedWrongHousehold, _) => {
            contracts::ExportImportRestoreApplyState::WrongHousehold
        }
        (contracts::ExportImportPreflightState::RejectedWrongKey, _) => {
            contracts::ExportImportRestoreApplyState::WrongKey
        }
        (contracts::ExportImportPreflightState::RejectedCorruptBundle, _) => {
            contracts::ExportImportRestoreApplyState::Corrupt
        }
        _ => contracts::ExportImportRestoreApplyState::Blocked,
    }
}
