use ocentra_schema::export_import_backup_recovery as contracts;

use super::RestoreExecutorReceipt;

pub(super) fn apply_restore_after_execution(
    preflight: &contracts::ExportImportImportPreflight,
    receipt: RestoreExecutorReceipt,
) -> Option<contracts::ExportImportRestoreApplyResult> {
    if !receipt_is_coherent(preflight, &receipt) {
        return None;
    }
    Some(contracts::ExportImportRestoreApplyResult {
        explicit_confirmation_required: false,
        local_truth_authoritative: true,
        tombstones_preserved: receipt.tombstones_preserved,
        idempotent: receipt.idempotent,
        accepted_sections: receipt.applied_sections,
        rejected_sections: receipt.rejected_sections,
        duplicates_created: receipt.duplicates_created,
        no_default_support_decrypt: preflight.no_default_support_decrypt,
        state: receipt.state,
    })
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

fn receipt_is_coherent(
    preflight: &contracts::ExportImportImportPreflight,
    receipt: &RestoreExecutorReceipt,
) -> bool {
    if receipt.execution_ref.trim().is_empty()
        || !receipt.idempotent
        || !receipt.tombstones_preserved
        || receipt.duplicates_created
        || !preflight_is_applicable(preflight)
        || has_duplicate_data_classes(&receipt.applied_sections, &receipt.rejected_sections)
    {
        return false;
    }

    match receipt.state {
        contracts::ExportImportRestoreApplyState::Applied => {
            preflight.state == contracts::ExportImportPreflightState::AcceptedPreview
                && receipt.applied_sections == preflight.accepted_sections
                && receipt.rejected_sections == preflight.rejected_sections
        }
        contracts::ExportImportRestoreApplyState::Partial => {
            preflight.state == contracts::ExportImportPreflightState::PartialPreview
                && !receipt.applied_sections.is_empty()
                && !receipt.rejected_sections.is_empty()
                && receipt.applied_sections.iter().all(|decision| {
                    preflight
                        .accepted_sections
                        .iter()
                        .any(|allowed| allowed.data_class == decision.data_class)
                })
                && receipt.rejected_sections.iter().all(|decision| {
                    preflight
                        .accepted_sections
                        .iter()
                        .chain(preflight.rejected_sections.iter())
                        .any(|allowed| allowed.data_class == decision.data_class)
                })
                && all_preflight_sections_are_reported(preflight, receipt)
        }
        _ => false,
    }
}

fn has_duplicate_data_classes(
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> bool {
    let mut seen = Vec::new();
    for decision in applied_sections.iter().chain(rejected_sections.iter()) {
        if seen
            .iter()
            .any(|data_class| data_class == &decision.data_class)
        {
            return true;
        }
        seen.push(decision.data_class);
    }
    false
}

fn all_preflight_sections_are_reported(
    preflight: &contracts::ExportImportImportPreflight,
    receipt: &RestoreExecutorReceipt,
) -> bool {
    let receipt_sections = receipt
        .applied_sections
        .iter()
        .chain(receipt.rejected_sections.iter());
    preflight
        .accepted_sections
        .iter()
        .chain(preflight.rejected_sections.iter())
        .all(|expected| {
            receipt_sections
                .clone()
                .any(|actual| actual.data_class == expected.data_class)
        })
}
