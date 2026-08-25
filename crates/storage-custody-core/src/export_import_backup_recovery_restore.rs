use ocentra_schema::export_import_backup_recovery as contracts;

pub(super) fn blocked_restore() -> contracts::ExportImportRestoreApplyResult {
    contracts::ExportImportRestoreApplyResult {
        state: contracts::ExportImportRestoreApplyState::Blocked,
        explicit_confirmation_required: true,
        local_truth_authoritative: false,
        tombstones_preserved: false,
        idempotent: false,
        accepted_sections: Vec::new(),
        rejected_sections: Vec::new(),
        duplicates_created: false,
        no_default_support_decrypt: true,
    }
}
