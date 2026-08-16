use crate::support::{StorageCustodyTestErrorExt, StorageCustodyTestValueExt};

use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    apply_restore, derive_export_bundle, run_import_preflight, ExportBundleBuildError,
    ExportBundleBuildRequest, ExportHumanSummaryInput, ExportPayloadSectionInput,
    ImportBundleContext, RestoreApplyRequest,
};

macro_rules! bundle_id {
    ($value:expr $(,)?) => {
        contracts::ExportImportBundleId::parse($value).assume_ok()
    };
}

macro_rules! household_id {
    ($value:expr $(,)?) => {
        contracts::ExportImportHouseholdId::parse($value).assume_ok()
    };
}

macro_rules! device_id {
    ($value:expr $(,)?) => {
        contracts::ExportImportDeviceId::parse($value).assume_ok()
    };
}

macro_rules! key_ref {
    ($value:expr $(,)?) => {
        contracts::ExportImportKeyRef::parse($value).assume_ok()
    };
}

macro_rules! tombstone_cursor {
    ($value:expr $(,)?) => {
        contracts::ExportImportTombstoneCursor::parse($value).assume_ok()
    };
}

macro_rules! timestamp {
    ($value:expr $(,)?) => {
        contracts::ExportImportTimestamp::parse($value).assume_ok()
    };
}

macro_rules! product_version {
    ($value:expr $(,)?) => {
        contracts::ExportImportProductVersion::parse($value).assume_ok()
    };
}

macro_rules! migration_ref {
    ($value:expr $(,)?) => {
        contracts::ExportImportMigrationRef::parse($value).assume_ok()
    };
}

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
    .assume_ok();

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
    .assume_ok();

    let preflight = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id!("family-wp05-proof-1"),
            local_product_version: product_version!("2026.07.01"),
            available_key_refs: vec![key_ref!("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![contracts::ExportImportDataClass::Notifications],
            known_device_ids: vec![],
            target_device_id: Some(device_id!("child-device-wp05-proof-1")),
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
    .assume_ok();

    let wrong_household = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id!("other-family"),
            local_product_version: product_version!("2026.06.28"),
            available_key_refs: vec![key_ref!("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id!("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        wrong_household.state,
        contracts::ExportImportPreflightState::HouseholdMismatch
    );

    let wrong_key = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id!("family-wp05-proof-1"),
            local_product_version: product_version!("2026.06.28"),
            available_key_refs: vec![key_ref!("some-other-key")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id!("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        wrong_key.state,
        contracts::ExportImportPreflightState::KeyUnavailable
    );

    let corrupt = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id!("family-wp05-proof-1"),
            local_product_version: product_version!("2026.06.28"),
            available_key_refs: vec![key_ref!("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id!("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: false,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        corrupt.state,
        contracts::ExportImportPreflightState::BundleCorrupt
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
    .assume_ok();

    let unsupported_schema = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id!("family-wp05-proof-1"),
            local_product_version: product_version!("2026.06.28"),
            available_key_refs: vec![key_ref!("parent-key-wp05-proof-1")],
            supported_schema_versions: vec!["other-schema".to_string()],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id!("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        unsupported_schema.state,
        contracts::ExportImportPreflightState::SchemaVersionInvalid
    );

    let expired = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id!("family-wp05-proof-1"),
            local_product_version: product_version!("2026.06.28"),
            available_key_refs: vec![key_ref!("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id!("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        expired.state,
        contracts::ExportImportPreflightState::RetentionExpired
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
    .assume_ok();
    let duplicate_device = run_import_preflight(
        &duplicate_device_bundle,
        &ImportBundleContext {
            local_household_id: household_id!("family-wp05-proof-1"),
            local_product_version: product_version!("2026.06.28"),
            available_key_refs: vec![key_ref!("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![device_id!("child-device-wp05-proof-1")],
            target_device_id: Some(device_id!("another-device")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        duplicate_device.state,
        contracts::ExportImportPreflightState::DeviceDuplicate
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
    .assume_ok();

    let migration_blocked = run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id!("family-wp05-proof-1"),
            local_product_version: product_version!("2026.07.01"),
            available_key_refs: vec![key_ref!("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id!("child-device-wp05-proof-1")),
            migration_supported: false,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    );
    assert_eq!(
        migration_blocked.state,
        contracts::ExportImportPreflightState::MigrationUnsupported
    );
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
    assert!(blocked.local_truth_authoritative);
    assert!(blocked.tombstones_preserved);
    assert!(!blocked.idempotent);
    assert!(!blocked.duplicates_created);
    assert!(blocked.accepted_sections.is_empty());
}

#[test]
fn export_import_backup_recovery_apply_restore_remains_blocked_until_executor_receipt() {
    let preflight = sample_restore_preflight();

    let blocked_once = apply_restore(&preflight, &RestoreApplyRequest { confirmed: true });
    let blocked_twice = apply_restore(&preflight, &RestoreApplyRequest { confirmed: true });

    assert_blocked_restore_is_stable(&blocked_once, &blocked_twice);
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
    .assume_err();

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
    .assume_ok();

    run_import_preflight(
        &bundle,
        &ImportBundleContext {
            local_household_id: household_id!("family-wp05-proof-1"),
            local_product_version: product_version!("2026.06.28"),
            available_key_refs: vec![key_ref!("parent-key-wp05-proof-1")],
            supported_schema_versions: vec![
                contracts::EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            ],
            blocked_restore_data_classes: vec![],
            known_device_ids: vec![],
            target_device_id: Some(device_id!("child-device-wp05-proof-1")),
            migration_supported: true,
            manifest_integrity_ok: true,
            payload_integrity_failures: vec![],
        },
    )
}

fn assert_blocked_restore_is_stable(
    blocked_once: &contracts::ExportImportRestoreApplyResult,
    blocked_twice: &contracts::ExportImportRestoreApplyResult,
) {
    assert_eq!(
        blocked_once.state,
        contracts::ExportImportRestoreApplyState::Blocked
    );
    assert_eq!(blocked_once, blocked_twice);
    assert!(blocked_once.explicit_confirmation_required);
    assert!(blocked_once.local_truth_authoritative);
    assert!(blocked_once.tombstones_preserved);
    assert!(!blocked_once.idempotent);
    assert!(!blocked_once.duplicates_created);
    assert!(blocked_once.accepted_sections.is_empty());
}

fn sample_build_request() -> ExportBundleBuildRequest {
    ExportBundleBuildRequest {
        bundle_id: bundle_id!("bundle-wp05-proof-1"),
        product_version: product_version!("2026.06.28"),
        created_at: timestamp!("2026-06-28T18:40:00.000Z"),
        household: contracts::ExportImportHouseholdReference {
            household_id: household_id!("family-wp05-proof-1"),
        },
        source_device_id: Some(device_id!("child-device-wp05-proof-1")),
        bundle_type: contracts::ExportImportBundleType::Backup,
        key_ref: key_ref!("parent-key-wp05-proof-1"),
        manifest_integrity_ref: Some(
            contracts::ExportImportIntegrityRef::parse("manifest-sha256-wp05-proof-1").assume_ok(),
        ),
        tombstone_cursor: Some(tombstone_cursor!("tombstone-cursor-wp05-proof-4")),
        retention_notes: vec![
            "Retention and tombstone ordering are preserved across restore preview.".to_string(),
        ],
        proof_tier: contracts::ExportImportProofTier::RuntimeValidated,
        migration_ref: Some(migration_ref!("migration-wp05-proof-1")),
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
        payload_ref: contracts::ExportImportPayloadRef::parse(format!(
            "payload-{}",
            data_class.as_str()
        ))
        .assume_ok(),
        payload_integrity_ref: Some(
            contracts::ExportImportIntegrityRef::parse(format!(
                "payload-sha256-{}",
                data_class.as_str()
            ))
            .assume_ok(),
        ),
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
