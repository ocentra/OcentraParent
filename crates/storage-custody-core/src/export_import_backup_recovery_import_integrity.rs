use ocentra_schema::export_import_backup_recovery as contracts;

pub(super) fn bundle_structure_is_honest(bundle: &contracts::ExportImportRecoveryBundle) -> bool {
    !bundle.sections.is_empty()
        && bundle.human_summary.raw_payload_redacted
        && bundle.human_summary.support_safe
        && manifest_identity_is_present(bundle)
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
            .manifest
            .data_classes
            .iter()
            .zip(bundle.sections.iter())
            .all(|(manifest_class, section)| manifest_class == &section.data_class)
        && unique_manifest_data_classes(&bundle.manifest.data_classes)
}

fn unique_manifest_data_classes(data_classes: &[contracts::ExportImportDataClass]) -> bool {
    data_classes.iter().enumerate().all(|(index, data_class)| {
        !data_classes[..index]
            .iter()
            .any(|previous| previous == data_class)
    })
}

/// Serde can construct the transparent identifier wrappers without going
/// through their `parse` constructors.  Import is therefore a trust boundary:
/// reject blank identity/integrity fields before any preflight state is
/// derived, rather than allowing a forged bundle to correlate with a local
/// household or payload by an empty reference.
fn manifest_identity_is_present(bundle: &contracts::ExportImportRecoveryBundle) -> bool {
    let manifest = &bundle.manifest;
    !manifest.bundle_id.as_str().trim().is_empty()
        && !manifest.schema_version.trim().is_empty()
        && !manifest.product_version.as_str().trim().is_empty()
        && !manifest.created_at.as_str().trim().is_empty()
        && !manifest.source_household_id.as_str().trim().is_empty()
        && !manifest.key_ref.as_str().trim().is_empty()
        && !manifest.manifest_integrity_ref.as_str().trim().is_empty()
        && manifest
            .source_device_id
            .as_ref()
            .map(|device_id| !device_id.as_str().trim().is_empty())
            .unwrap_or(true)
        && manifest
            .tombstone_cursor
            .as_ref()
            .map(|cursor| !cursor.as_str().trim().is_empty())
            .unwrap_or(true)
        && manifest
            .migration_ref
            .as_ref()
            .map(|migration_ref| !migration_ref.as_str().trim().is_empty())
            .unwrap_or(true)
        && bundle.sections.iter().all(|section| {
            !section.payload_ref.as_str().trim().is_empty()
                && !section.payload_integrity_ref.as_str().trim().is_empty()
        })
}
