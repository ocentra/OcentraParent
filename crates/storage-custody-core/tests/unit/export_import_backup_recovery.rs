use crate::support::{StorageCustodyTestErrorExt, StorageCustodyTestValueExt};

use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    apply_restore, derive_export_bundle, run_import_preflight, ExportBundleBuildError,
    ExportBundleBuildRequest, ExportHumanSummaryInput, ExportPayloadSectionInput,
    ImportBundleContext, RestoreApplyRequest,
};

#[test]
fn export_import_backup_recovery_builds_versioned_bundle_with_encrypted_sections_and_redacted_summary(
) {
    let bundle = derive_export_bundle(
        sample_build_request(),
        vec![
            section_input(
                contracts::ExportImportDataClass::EvidenceJournal,
                contracts::ExportImportSectionRetentionState::Active,
                true,
                false,
            ),
            section_input(
                contracts::ExportImportDataClass::Reports,
                contracts::ExportImportSectionRetentionState::Active,
                true,
                false,
            ),
        ],
        summary_input(),
    )
    .value_or_unreachable("bundle builds");

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
}

#[test]
fn export_import_backup_recovery_preview_is_non_mutating_and_partial_when_expired_and_tombstoned_sections_exist(
) {
    let bundle = derive_export_bundle(
        sample_build_request(),
        vec![
            section_input(
                contracts::ExportImportDataClass::EvidenceJournal,
                contracts::ExportImportSectionRetentionState::Active,
                true,
                false,
            ),
            section_input(
                contracts::ExportImportDataClass::Screenshots,
                contracts::ExportImportSectionRetentionState::Expired,
                false,
                false,
            ),
            section_input(
                contracts::ExportImportDataClass::Notifications,
                contracts::ExportImportSectionRetentionState::Active,
                false,
                false,
            ),
        ],
        summary_input(),
    )
    .value_or_unreachable("bundle builds");

    let preflight = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id("family-wp05-proof-1"),
            local_product_version: product_version("2026.07.01"),
            available_key_refs: vec![key_ref("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![contracts::ExportImportDataClass::Notifications],
            known_device_ids: vec![],
            target_device_id: Some(device_id("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );

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
    let bundle = derive_export_bundle(
        sample_build_request(),
        vec![section_input(
            contracts::ExportImportDataClass::EvidenceJournal,
            contracts::ExportImportSectionRetentionState::Active,
            true,
            false,
        )],
        summary_input(),
    )
    .value_or_unreachable("bundle builds");

    let wrong_household = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id("other-family"),
            local_product_version: product_version("2026.06.28"),
            available_key_refs: vec![key_ref("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        wrong_household.state,
        contracts::ExportImportPreflightState::RejectedWrongHousehold
    );

    let wrong_key = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id("family-wp05-proof-1"),
            local_product_version: product_version("2026.06.28"),
            available_key_refs: vec![key_ref("some-other-key")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        wrong_key.state,
        contracts::ExportImportPreflightState::RejectedWrongKey
    );

    let corrupt = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id("family-wp05-proof-1"),
            local_product_version: product_version("2026.06.28"),
            available_key_refs: vec![key_ref("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: false,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        corrupt.state,
        contracts::ExportImportPreflightState::RejectedCorruptBundle
    );
}

#[test]
fn export_import_backup_recovery_rejects_schema_expiry_duplicate_device_and_unsupported_migration()
{
    let bundle = derive_export_bundle(
        sample_build_request(),
        vec![section_input(
            contracts::ExportImportDataClass::EvidenceJournal,
            contracts::ExportImportSectionRetentionState::Expired,
            true,
            false,
        )],
        summary_input(),
    )
    .value_or_unreachable("bundle builds");

    let unsupported_schema = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id("family-wp05-proof-1"),
            local_product_version: product_version("2026.06.28"),
            available_key_refs: vec![key_ref("parent-key-wp05-proof-1")],
            supported_schema_versions: vec!["other-schema".to_string()],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        unsupported_schema.state,
        contracts::ExportImportPreflightState::RejectedSchemaVersion
    );

    let expired = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id("family-wp05-proof-1"),
            local_product_version: product_version("2026.06.28"),
            available_key_refs: vec![key_ref("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        expired.state,
        contracts::ExportImportPreflightState::RejectedExpiredRetention
    );

    let duplicate_device_bundle = derive_export_bundle(
        sample_build_request(),
        vec![section_input(
            contracts::ExportImportDataClass::DeviceRegistry,
            contracts::ExportImportSectionRetentionState::Active,
            true,
            false,
        )],
        summary_input(),
    )
    .value_or_unreachable("bundle builds");
    let duplicate_device = run_import_preflight(
        &duplicate_device_bundle,
        &ImportBundleContext {
            local_household_id: household_id("family-wp05-proof-1"),
            local_product_version: product_version("2026.06.28"),
            available_key_refs: vec![key_ref("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![device_id("child-device-wp05-proof-1")],
            target_device_id: Some(device_id("another-device")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        duplicate_device.state,
        contracts::ExportImportPreflightState::RejectedDuplicateDevice
    );
}

#[test]
fn export_import_backup_recovery_rejects_unsupported_migration() {
    let bundle = derive_export_bundle(
        sample_build_request(),
        vec![section_input(
            contracts::ExportImportDataClass::DeviceRegistry,
            contracts::ExportImportSectionRetentionState::Active,
            true,
            false,
        )],
        summary_input(),
    )
    .value_or_unreachable("bundle builds");

    let migration_blocked = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id("family-wp05-proof-1"),
            local_product_version: product_version("2026.07.01"),
            available_key_refs: vec![key_ref("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id("child-device-wp05-proof-1")),
            migration_supported: false,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        migration_blocked.state,
        contracts::ExportImportPreflightState::RejectedMigrationUnsupported
    );
}

#[test]
fn export_import_backup_recovery_apply_restore_reports_pending_before_confirmation() {
    let preflight = sample_restore_preflight();

    let pending = apply_restore(&preflight, &RestoreApplyRequest { confirmed: false });
    assert_eq!(
        pending.state,
        contracts::ExportImportRestoreApplyState::ApplyPending
    );
}

#[test]
fn export_import_backup_recovery_apply_restore_is_idempotent_and_preserves_tombstones() {
    let preflight = sample_restore_preflight();

    let applied_once = apply_restore(&preflight, &RestoreApplyRequest { confirmed: true });
    let applied_twice = apply_restore(&preflight, &RestoreApplyRequest { confirmed: true });

    assert_applied_restore_is_idempotent(&applied_once, &applied_twice);
}

#[test]
fn export_import_backup_recovery_rejects_default_support_decrypt_path() {
    let error = derive_export_bundle(
        sample_build_request(),
        vec![section_input(
            contracts::ExportImportDataClass::EvidenceJournal,
            contracts::ExportImportSectionRetentionState::Active,
            true,
            true,
        )],
        summary_input(),
    )
    .error_or_unreachable("support default decrypt must be rejected");

    assert_eq!(
        error,
        ExportBundleBuildError::SupportDefaultDecryptForbidden(
            contracts::ExportImportDataClass::EvidenceJournal
        )
    );
}

fn sample_restore_preflight() -> contracts::ExportImportImportPreflight {
    let bundle = derive_export_bundle(
        sample_build_request(),
        vec![
            section_input(
                contracts::ExportImportDataClass::EvidenceJournal,
                contracts::ExportImportSectionRetentionState::Active,
                true,
                false,
            ),
            section_input(
                contracts::ExportImportDataClass::Notifications,
                contracts::ExportImportSectionRetentionState::Tombstoned,
                false,
                false,
            ),
        ],
        summary_input(),
    )
    .value_or_unreachable("bundle builds");

    run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id("family-wp05-proof-1"),
            local_product_version: product_version("2026.06.28"),
            available_key_refs: vec![key_ref("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    )
}

fn assert_applied_restore_is_idempotent(
    applied_once: &contracts::ExportImportRestoreApplyResult,
    applied_twice: &contracts::ExportImportRestoreApplyResult,
) {
    assert_eq!(
        applied_once.state,
        contracts::ExportImportRestoreApplyState::Partial
    );
    assert_eq!(applied_once, applied_twice);
    assert!(applied_once.tombstones_preserved);
    assert!(applied_once.idempotent);
    assert!(!applied_once.duplicates_created);
}

fn sample_build_request() -> ExportBundleBuildRequest {
    ExportBundleBuildRequest {
        bundle_id: bundle_id("bundle-wp05-proof-1"),
        product_version: product_version("2026.06.28"),
        created_at: timestamp("2026-06-28T18:40:00.000Z"),
        household: contracts::ExportImportHouseholdReference {
            household_id: household_id("family-wp05-proof-1"),
        },
        source_device_id: Some(device_id("child-device-wp05-proof-1")),
        bundle_type: contracts::ExportImportBundleType::Backup,
        key_ref: key_ref("parent-key-wp05-proof-1"),
        manifest_integrity_ref: Some(integrity_ref("manifest-sha256-wp05-proof-1")),
        tombstone_cursor: Some(tombstone_cursor("tombstone-cursor-wp05-proof-4")),
        retention_notes: vec![
            "Retention and tombstone ordering are preserved across restore preview.".to_string(),
        ],
        proof_tier: contracts::ExportImportProofTier::RuntimeValidated,
        migration_ref: Some(migration_ref("migration-wp05-proof-1")),
    }
}

fn section_input(
    data_class: contracts::ExportImportDataClass,
    retention_state: contracts::ExportImportSectionRetentionState,
    included_in_summary: bool,
    support_default_decryptable: bool,
) -> ExportPayloadSectionInput {
    ExportPayloadSectionInput {
        data_class,
        payload_ref: payload_ref(format!("payload-{}", data_class.as_str())),
        payload_integrity_ref: Some(integrity_ref(format!(
            "payload-sha256-{}",
            data_class.as_str()
        ))),
        encrypted: true,
        retention_state,
        support_default_decryptable,
        included_in_human_summary: included_in_summary,
        notes: format!("Section for {}", data_class.as_str()),
    }
}

fn summary_input() -> ExportHumanSummaryInput {
    ExportHumanSummaryInput {
        headline: "WP05 export bundle".to_string(),
        excluded_data_classes: vec![contracts::ExportImportDataClass::Notifications],
        raw_payload_redacted: true,
        support_safe: true,
        notes: "Support-safe summary only.".to_string(),
    }
}

fn bundle_id(value: &str) -> contracts::ExportImportBundleId {
    contracts::ExportImportBundleId::parse(value).value_or_unreachable("bundle id")
}

fn household_id(value: &str) -> contracts::ExportImportHouseholdId {
    contracts::ExportImportHouseholdId::parse(value).value_or_unreachable("household id")
}

fn device_id(value: &str) -> contracts::ExportImportDeviceId {
    contracts::ExportImportDeviceId::parse(value).value_or_unreachable("device id")
}

fn key_ref(value: &str) -> contracts::ExportImportKeyRef {
    contracts::ExportImportKeyRef::parse(value).value_or_unreachable("key ref")
}

fn payload_ref(value: impl Into<String>) -> contracts::ExportImportPayloadRef {
    contracts::ExportImportPayloadRef::parse(value).value_or_unreachable("payload ref")
}

fn integrity_ref(value: impl Into<String>) -> contracts::ExportImportIntegrityRef {
    contracts::ExportImportIntegrityRef::parse(value).value_or_unreachable("integrity ref")
}

fn tombstone_cursor(value: &str) -> contracts::ExportImportTombstoneCursor {
    contracts::ExportImportTombstoneCursor::parse(value).value_or_unreachable("tombstone cursor")
}

fn timestamp(value: &str) -> contracts::ExportImportTimestamp {
    contracts::ExportImportTimestamp::parse(value).value_or_unreachable("timestamp")
}

fn product_version(value: &str) -> contracts::ExportImportProductVersion {
    contracts::ExportImportProductVersion::parse(value).value_or_unreachable("product version")
}

fn migration_ref(value: &str) -> contracts::ExportImportMigrationRef {
    contracts::ExportImportMigrationRef::parse(value).value_or_unreachable("migration ref")
}
