#![cfg(test)]

use super::{
    apply_restore, derive_export_bundle, run_import_preflight, ExportBundleBuildError,
    ExportBundleBuildRequest, ExportHumanSummaryInput, ExportPayloadSectionInput,
    ImportBundleContext, RestoreApplyRequest,
};
use ocentra_schema::export_import_backup_recovery as contracts;

macro_rules! parsed {
    ($type:ty, $value:expr) => {
        <$type>::parse($value).expect("typed export/import reference")
    };
}

#[test]
fn export_import_backup_recovery_bundle_stays_versioned_and_builder_fails_closed_without_custody() {
    let bundle = contracts::sample_export_import_backup_recovery_contract_proof().bundle;

    assert_eq!(
        bundle.manifest.schema_version,
        contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION
    );
    assert_eq!(
        bundle.manifest.encryption_mode,
        contracts::ExportImportEncryptionMode::PerClassEnvelopeEncrypted
    );
    assert_eq!(
        bundle.manifest.payload_integrity_mode,
        contracts::ExportImportIntegrityMode::ManifestAndPayloadHashes
    );
    assert!(bundle.sections.iter().all(|section| section.encrypted));
    assert!(bundle.human_summary.raw_payload_redacted);
    assert!(bundle.human_summary.support_safe);

    assert_eq!(
        derive_export_bundle(
            sample_build_request(),
            vec![section_input(
                contracts::ExportImportDataClass::EvidenceJournal,
                contracts::ExportImportSectionRetentionState::Active,
                true,
            )],
            summary_input(),
        ),
        Err(ExportBundleBuildError::EncryptionCustodyUnavailable)
    );
}

#[test]
fn export_import_backup_recovery_preview_is_non_mutating_and_partial_when_expired_and_tombstoned_sections_exist(
) {
    let bundle = bundle_with_sections(vec![
        section(
            contracts::ExportImportDataClass::EvidenceJournal,
            contracts::ExportImportSectionRetentionState::Active,
            true,
        ),
        section(
            contracts::ExportImportDataClass::Screenshots,
            contracts::ExportImportSectionRetentionState::Expired,
            false,
        ),
        section(
            contracts::ExportImportDataClass::Notifications,
            contracts::ExportImportSectionRetentionState::Active,
            false,
        ),
    ]);
    let mut context = matching_context(&bundle);
    context.local_product_version = parsed!(contracts::ExportImportProductVersion, "2026.07.01");
    context
        .blocked_restore_data_classes
        .push(contracts::ExportImportDataClass::Notifications);

    let preflight = run_import_preflight(&bundle, &context);

    assert_eq!(
        preflight.state,
        contracts::ExportImportPreflightState::PartialPreview
    );
    assert!(!preflight.local_truth_mutated);
    assert_eq!(
        preflight.migration_state,
        contracts::ExportImportMigrationState::RequiredSupported
    );
    assert_eq!(preflight.accepted_sections.len(), 1);
    assert_eq!(preflight.rejected_sections.len(), 2);
    assert!(preflight.tombstones_preserved);
}

#[test]
fn export_import_backup_recovery_rejects_wrong_household_wrong_key_and_corrupt_bundle_fail_closed()
{
    let bundle = bundle_with_sections(vec![section(
        contracts::ExportImportDataClass::EvidenceJournal,
        contracts::ExportImportSectionRetentionState::Active,
        true,
    )]);

    let mut wrong_household = matching_context(&bundle);
    wrong_household.local_household_id =
        parsed!(contracts::ExportImportHouseholdId, "other-family");
    assert_eq!(
        run_import_preflight(&bundle, &wrong_household).state,
        contracts::ExportImportPreflightState::HouseholdMismatch
    );

    let mut wrong_key = matching_context(&bundle);
    wrong_key.available_key_refs = vec![parsed!(contracts::ExportImportKeyRef, "some-other-key")];
    assert_eq!(
        run_import_preflight(&bundle, &wrong_key).state,
        contracts::ExportImportPreflightState::KeyUnavailable
    );

    let mut corrupt = matching_context(&bundle);
    corrupt.manifest_integrity_ok = false;
    assert_eq!(
        run_import_preflight(&bundle, &corrupt).state,
        contracts::ExportImportPreflightState::BundleCorrupt
    );
}

#[test]
fn export_import_backup_recovery_rejects_schema_expiry_duplicate_device_and_unsupported_migration()
{
    let expired_bundle = bundle_with_sections(vec![section(
        contracts::ExportImportDataClass::EvidenceJournal,
        contracts::ExportImportSectionRetentionState::Expired,
        true,
    )]);
    let mut unsupported_schema = matching_context(&expired_bundle);
    unsupported_schema.supported_schema_versions = vec!["other-schema".to_owned()];
    assert_eq!(
        run_import_preflight(&expired_bundle, &unsupported_schema).state,
        contracts::ExportImportPreflightState::SchemaVersionInvalid
    );

    assert_eq!(
        run_import_preflight(&expired_bundle, &matching_context(&expired_bundle)).state,
        contracts::ExportImportPreflightState::RetentionExpired
    );

    let duplicate_bundle = bundle_with_sections(vec![section(
        contracts::ExportImportDataClass::DeviceRegistry,
        contracts::ExportImportSectionRetentionState::Active,
        true,
    )]);
    let mut duplicate_device = matching_context(&duplicate_bundle);
    let source_device_id = parsed!(contracts::ExportImportDeviceId, "child-device-proof-1");
    assert_eq!(
        duplicate_bundle.manifest.source_device_id.as_ref(),
        Some(&source_device_id)
    );
    duplicate_device.known_device_ids = vec![source_device_id];
    duplicate_device.target_device_id =
        Some(parsed!(contracts::ExportImportDeviceId, "another-device"));
    assert_eq!(
        run_import_preflight(&duplicate_bundle, &duplicate_device).state,
        contracts::ExportImportPreflightState::DeviceDuplicate
    );
}

#[test]
fn export_import_backup_recovery_rejects_unsupported_migration() {
    let bundle = bundle_with_sections(vec![section(
        contracts::ExportImportDataClass::DeviceRegistry,
        contracts::ExportImportSectionRetentionState::Active,
        true,
    )]);
    let mut migration_blocked = matching_context(&bundle);
    migration_blocked.local_product_version =
        parsed!(contracts::ExportImportProductVersion, "2026.07.01");
    migration_blocked.migration_supported = false;

    let preflight = run_import_preflight(&bundle, &migration_blocked);
    assert_eq!(
        preflight.state,
        contracts::ExportImportPreflightState::MigrationUnsupported
    );
    assert_eq!(
        preflight.migration_state,
        contracts::ExportImportMigrationState::RequiredUnsupported
    );
    assert!(!preflight.local_truth_mutated);
}

#[test]
fn export_import_backup_recovery_apply_restore_blocks_without_real_executor() {
    let preflight = sample_restore_preflight();

    let blocked = apply_restore(&preflight, &RestoreApplyRequest { confirmed: false });
    assert_eq!(
        blocked.state,
        contracts::ExportImportRestoreApplyState::Blocked
    );
    assert!(blocked.explicit_confirmation_required);
    assert!(!blocked.local_truth_authoritative);
    assert!(!blocked.tombstones_preserved);
    assert!(!blocked.idempotent);
    assert!(!blocked.duplicates_created);
    assert!(blocked.accepted_sections.is_empty());
}

#[test]
fn export_import_backup_recovery_apply_restore_remains_blocked_until_executor_receipt() {
    let preflight = sample_restore_preflight();

    let blocked_once = apply_restore(&preflight, &RestoreApplyRequest { confirmed: true });
    let blocked_twice = apply_restore(&preflight, &RestoreApplyRequest { confirmed: true });

    assert_eq!(
        blocked_once.state,
        contracts::ExportImportRestoreApplyState::Blocked
    );
    assert_eq!(blocked_once, blocked_twice);
    assert!(blocked_once.explicit_confirmation_required);
    assert!(!blocked_once.local_truth_authoritative);
    assert!(!blocked_once.tombstones_preserved);
    assert!(!blocked_once.idempotent);
    assert!(!blocked_once.duplicates_created);
    assert!(blocked_once.accepted_sections.is_empty());
}

#[test]
fn export_import_backup_recovery_rejects_default_support_decrypt_path() {
    let proof = contracts::sample_export_import_backup_recovery_contract_proof();
    assert!(proof
        .bundle
        .sections
        .iter()
        .all(|section| !section.support_default_decryptable));

    assert_eq!(
        derive_export_bundle(
            sample_build_request(),
            vec![section_input(
                contracts::ExportImportDataClass::EvidenceJournal,
                contracts::ExportImportSectionRetentionState::Active,
                true,
            )],
            summary_input(),
        ),
        Err(ExportBundleBuildError::EncryptionCustodyUnavailable)
    );
}

fn sample_restore_preflight() -> contracts::ExportImportImportPreflight {
    let bundle = bundle_with_sections(vec![
        section(
            contracts::ExportImportDataClass::EvidenceJournal,
            contracts::ExportImportSectionRetentionState::Active,
            true,
        ),
        section(
            contracts::ExportImportDataClass::Notifications,
            contracts::ExportImportSectionRetentionState::Tombstoned,
            false,
        ),
    ]);
    run_import_preflight(&bundle, &matching_context(&bundle))
}

fn matching_context(bundle: &contracts::ExportImportRecoveryBundle) -> ImportBundleContext {
    ImportBundleContext {
        local_household_id: bundle.manifest.source_household_id.clone(),
        local_product_version: bundle.manifest.product_version.clone(),
        available_key_refs: vec![bundle.manifest.key_ref.clone()],
        supported_schema_versions: vec![bundle.manifest.schema_version.clone()],
        blocked_restore_data_classes: Vec::new(),
        known_device_ids: Vec::new(),
        target_device_id: bundle.manifest.source_device_id.clone(),
        migration_supported: true,
        manifest_integrity_ok: true,
        payload_integrity_failures: Vec::new(),
        current_tombstone_cursor: bundle.manifest.tombstone_cursor.clone(),
    }
}

fn bundle_with_sections(
    sections: Vec<contracts::ExportImportPayloadSection>,
) -> contracts::ExportImportRecoveryBundle {
    let mut bundle = contracts::sample_export_import_backup_recovery_contract_proof().bundle;
    bundle.manifest.data_classes = sections.iter().map(|section| section.data_class).collect();
    bundle.human_summary.included_data_classes = sections
        .iter()
        .filter(|section| section.included_in_human_summary)
        .map(|section| section.data_class)
        .collect();
    bundle.human_summary.excluded_data_classes = sections
        .iter()
        .filter(|section| !section.included_in_human_summary)
        .map(|section| section.data_class)
        .collect();
    bundle.sections = sections;
    bundle
}

fn section(
    data_class: contracts::ExportImportDataClass,
    retention_state: contracts::ExportImportSectionRetentionState,
    included_in_human_summary: bool,
) -> contracts::ExportImportPayloadSection {
    contracts::ExportImportPayloadSection {
        data_class,
        payload_ref: parsed!(
            contracts::ExportImportPayloadRef,
            format!("payload-{}", data_class.as_str())
        ),
        payload_integrity_ref: parsed!(
            contracts::ExportImportIntegrityRef,
            format!("payload-sha256-{}", data_class.as_str())
        ),
        encrypted: true,
        retention_state,
        support_default_decryptable: false,
        included_in_human_summary,
        notes: format!("Section for {}", data_class.as_str()),
    }
}

fn sample_build_request() -> ExportBundleBuildRequest {
    ExportBundleBuildRequest {
        bundle_id: parsed!(contracts::ExportImportBundleId, "bundle-wp05-proof-1"),
        product_version: parsed!(contracts::ExportImportProductVersion, "2026.06.28"),
        created_at: parsed!(contracts::ExportImportTimestamp, "2026-06-28T18:40:00.000Z"),
        household: contracts::ExportImportHouseholdReference {
            household_id: parsed!(contracts::ExportImportHouseholdId, "family-wp05-proof-1"),
        },
        source_device_id: Some(parsed!(
            contracts::ExportImportDeviceId,
            "child-device-wp05-proof-1"
        )),
        bundle_type: contracts::ExportImportBundleType::Backup,
        key_ref: parsed!(contracts::ExportImportKeyRef, "parent-key-wp05-proof-1"),
        manifest_integrity_ref: Some(parsed!(
            contracts::ExportImportIntegrityRef,
            "manifest-sha256-wp05-proof-1"
        )),
        tombstone_cursor: Some(parsed!(
            contracts::ExportImportTombstoneCursor,
            "tombstone-cursor-wp05-proof-4"
        )),
        retention_notes: vec![
            "Retention and tombstone ordering are preserved across restore preview.".to_owned(),
        ],
        proof_tier: contracts::ExportImportProofTier::RuntimeValidated,
        migration_ref: Some(parsed!(
            contracts::ExportImportMigrationRef,
            "migration-wp05-proof-1"
        )),
    }
}

fn section_input(
    data_class: contracts::ExportImportDataClass,
    retention_state: contracts::ExportImportSectionRetentionState,
    included_in_human_summary: bool,
) -> ExportPayloadSectionInput {
    ExportPayloadSectionInput {
        data_class,
        payload_ref: parsed!(
            contracts::ExportImportPayloadRef,
            format!("payload-{}", data_class.as_str())
        ),
        payload_integrity_ref: Some(parsed!(
            contracts::ExportImportIntegrityRef,
            format!("payload-sha256-{}", data_class.as_str())
        )),
        retention_state,
        included_in_human_summary,
        notes: format!("Section for {}", data_class.as_str()),
    }
}

fn summary_input() -> ExportHumanSummaryInput {
    ExportHumanSummaryInput {
        headline: "WP05 export bundle".to_owned(),
        excluded_data_classes: vec![contracts::ExportImportDataClass::Notifications],
        raw_payload_redacted: true,
        support_safe: true,
        notes: "Support-safe summary only.".to_owned(),
    }
}
