use ocentra_schema::export_import_backup_recovery as contracts;

pub(super) fn bundle_structure_is_honest(bundle: &contracts::ExportImportRecoveryBundle) -> bool {
    !bundle.sections.is_empty()
        && bundle.human_summary.raw_payload_redacted
        && bundle.human_summary.support_safe
        && bundle
            .sections
            .iter()
            .all(|section| section.encrypted && !section.support_default_decryptable)
        && unique_data_classes(&bundle.sections)
        && manifest_matches_sections(bundle)
}

fn unique_data_classes(sections: &[contracts::ExportImportPayloadSection]) -> bool {
    sections.iter().enumerate().all(|(index, section)| {
        !sections[..index]
            .iter()
            .any(|previous| previous.data_class == section.data_class)
    })
}

fn manifest_matches_sections(bundle: &contracts::ExportImportRecoveryBundle) -> bool {
    bundle.manifest.data_classes.len() == bundle.sections.len()
        && bundle
            .sections
            .iter()
            .all(|section| bundle.manifest.data_classes.contains(&section.data_class))
}
