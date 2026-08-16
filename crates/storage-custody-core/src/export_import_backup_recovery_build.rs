use std::collections::BTreeSet;

use ocentra_schema::export_import_backup_recovery as contracts;

use super::{
    ExportBundleBuildError, ExportBundleBuildRequest, ExportHumanSummaryInput,
    ExportPayloadSectionInput,
};

pub(super) fn derive_export_bundle(
    request: ExportBundleBuildRequest,
    sections: Vec<ExportPayloadSectionInput>,
    summary: ExportHumanSummaryInput,
) -> Result<contracts::ExportImportRecoveryBundle, ExportBundleBuildError> {
    if sections.is_empty() {
        return Err(ExportBundleBuildError::EmptySections);
    }
    let manifest_integrity_ref = request
        .manifest_integrity_ref
        .ok_or(ExportBundleBuildError::MissingManifestIntegrity)?;
    if !summary.raw_payload_redacted {
        return Err(ExportBundleBuildError::SummaryMustBeRedacted);
    }
    if !summary.support_safe {
        return Err(ExportBundleBuildError::SummaryMustBeSupportSafe);
    }

    let excluded = summary.excluded_data_classes;
    let (manifest_data_classes, section_contracts) =
        build_export_bundle_sections(sections, &excluded)?;

    let included_data_classes = section_contracts
        .iter()
        .filter(|section| section.included_in_human_summary)
        .map(|section| section.data_class)
        .collect::<Vec<_>>();

    Ok(contracts::ExportImportRecoveryBundle {
        manifest: contracts::ExportImportRecoveryBundleManifest {
            bundle_id: request.bundle_id,
            schema_version: contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            product_version: request.product_version,
            created_at: request.created_at,
            source_household_id: request.household.household_id,
            source_device_id: request.source_device_id,
            bundle_type: request.bundle_type,
            data_classes: manifest_data_classes,
            encryption_mode: contracts::ExportImportEncryptionMode::PerClassEnvelopeEncrypted,
            key_ref: request.key_ref,
            manifest_integrity_ref,
            payload_integrity_mode: contracts::ExportImportIntegrityMode::ManifestAndPayloadHashes,
            tombstone_cursor: request.tombstone_cursor,
            retention_notes: request.retention_notes,
            proof_tier: request.proof_tier,
            migration_ref: request.migration_ref,
        },
        sections: section_contracts,
        human_summary: contracts::ExportImportHumanSummary {
            headline: summary.headline,
            included_data_classes,
            excluded_data_classes: excluded,
            raw_payload_redacted: true,
            support_safe: true,
            notes: summary.notes,
        },
    })
}

fn build_export_bundle_sections(
    sections: Vec<ExportPayloadSectionInput>,
    excluded: &[contracts::ExportImportDataClass],
) -> Result<
    (
        Vec<contracts::ExportImportDataClass>,
        Vec<contracts::ExportImportPayloadSection>,
    ),
    ExportBundleBuildError,
> {
    let mut seen = BTreeSet::new();
    let mut manifest_data_classes = Vec::with_capacity(sections.len());
    let mut section_contracts = Vec::with_capacity(sections.len());

    for section in sections {
        if !seen.insert(section.data_class.as_str()) {
            return Err(ExportBundleBuildError::DuplicateDataClass(
                section.data_class,
            ));
        }
        if !section.encrypted {
            return Err(ExportBundleBuildError::SectionNotEncrypted(
                section.data_class,
            ));
        }
        if section.support_default_decryptable {
            return Err(ExportBundleBuildError::SupportDefaultDecryptForbidden(
                section.data_class,
            ));
        }
        let payload_integrity_ref = section.payload_integrity_ref.ok_or(
            ExportBundleBuildError::MissingPayloadIntegrity(section.data_class),
        )?;

        manifest_data_classes.push(section.data_class);
        section_contracts.push(contracts::ExportImportPayloadSection {
            data_class: section.data_class,
            payload_ref: section.payload_ref,
            payload_integrity_ref,
            encrypted: true,
            retention_state: section.retention_state,
            support_default_decryptable: false,
            included_in_human_summary: !excluded.contains(&section.data_class)
                && section.included_in_human_summary,
            notes: section.notes,
        });
    }

    Ok((manifest_data_classes, section_contracts))
}
