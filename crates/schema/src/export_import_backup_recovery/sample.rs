use super::identifiers::{
    bundle_id, contract_version, device_id, household_id, integrity_ref, key_ref, migration_ref,
    payload_ref, product_version, timestamp, tombstone_cursor,
};
use super::*;

pub(super) fn sample_export_import_backup_recovery_contract_proof(
) -> ExportImportBackupRecoveryContractProof {
    let partial_rejections = partial_rejections();
    let partial_acceptances = partial_acceptances();

    ExportImportBackupRecoveryContractProof {
        schema_version: EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
        contract_version: contract_version(EXPORT_IMPORT_CONTRACT_VERSION_V0_5),
        bundle: sample_bundle(),
        import_preflight: import_preflight(&partial_acceptances, &partial_rejections),
        negative_preflights: negative_preflights(),
        restore_apply: restore_apply(partial_acceptances, partial_rejections),
        non_claims: required_export_import_non_claims(),
        provider_runtime_claimed: false,
        support_default_child_evidence_decryption: false,
        ts_business_owner_claimed: false,
        updated_at: timestamp(EXPORT_IMPORT_UPDATED_AT),
    }
}

fn sample_bundle() -> ExportImportRecoveryBundle {
    ExportImportRecoveryBundle {
        manifest: ExportImportRecoveryBundleManifest {
            bundle_id: bundle_id(EXPORT_IMPORT_BUNDLE_ID_PROOF_1),
            schema_version: EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION.to_string(),
            product_version: product_version(EXPORT_IMPORT_PRODUCT_VERSION_2026_06_28),
            created_at: timestamp(EXPORT_IMPORT_CREATED_AT),
            source_household_id: household_id(EXPORT_IMPORT_SOURCE_HOUSEHOLD_ID_PROOF_1),
            source_device_id: Some(device_id(EXPORT_IMPORT_SOURCE_DEVICE_ID_PROOF_1)),
            bundle_type: ExportImportBundleType::Backup,
            data_classes: EXPORT_IMPORT_BUNDLE_DATA_CLASSES.to_vec(),
            encryption_mode: ExportImportEncryptionMode::PerClassEnvelopeEncrypted,
            key_ref: key_ref(EXPORT_IMPORT_PARENT_KEY_PROOF_1),
            manifest_integrity_ref: integrity_ref(EXPORT_IMPORT_MANIFEST_SHA256_PROOF_1),
            payload_integrity_mode: ExportImportIntegrityMode::ManifestAndPayloadHashes,
            tombstone_cursor: Some(tombstone_cursor(EXPORT_IMPORT_TOMBSTONE_CURSOR_PROOF_7)),
            retention_notes: EXPORT_IMPORT_RETENTION_NOTES
                .iter()
                .map(|note| (*note).to_string())
                .collect(),
            proof_tier: ExportImportProofTier::RuntimeValidated,
            migration_ref: Some(migration_ref(EXPORT_IMPORT_MIGRATION_PROOF_1)),
        },
        sections: EXPORT_IMPORT_SECTION_SEEDS
            .iter()
            .copied()
            .map(section)
            .collect(),
        human_summary: ExportImportHumanSummary {
            headline: EXPORT_IMPORT_HUMAN_SUMMARY_HEADLINE.to_string(),
            included_data_classes: EXPORT_IMPORT_INCLUDED_DATA_CLASSES.to_vec(),
            excluded_data_classes: EXPORT_IMPORT_EXCLUDED_DATA_CLASSES.to_vec(),
            raw_payload_redacted: true,
            support_safe: true,
            notes: EXPORT_IMPORT_HUMAN_SUMMARY_NOTES.to_string(),
        },
    }
}

fn partial_rejections() -> Vec<ExportImportSectionDecision> {
    vec![
        rejection(
            ExportImportDataClass::Screenshots,
            ExportImportSectionDecisionState::RetentionExpired,
            EXPORT_IMPORT_REJECTION_REASON_EXPIRED_RETENTION,
        ),
        rejection(
            ExportImportDataClass::Notifications,
            ExportImportSectionDecisionState::TombstonePreserved,
            EXPORT_IMPORT_REJECTION_REASON_TOMBSTONE_PRESERVED,
        ),
    ]
}

fn partial_acceptances() -> Vec<ExportImportSectionDecision> {
    vec![
        acceptance(
            ExportImportDataClass::EvidenceJournal,
            EXPORT_IMPORT_ACCEPTANCE_REASON_EVIDENCE_JOURNAL,
        ),
        acceptance(
            ExportImportDataClass::Reports,
            EXPORT_IMPORT_ACCEPTANCE_REASON_REPORTS,
        ),
    ]
}

fn import_preflight(
    partial_acceptances: &[ExportImportSectionDecision],
    partial_rejections: &[ExportImportSectionDecision],
) -> ExportImportImportPreflight {
    ExportImportImportPreflight {
        state: ExportImportPreflightState::PartialPreview,
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        manifest_integrity_verified: true,
        payload_integrity_verified: true,
        local_truth_mutated: false,
        tombstones_preserved: true,
        duplicate_device_detected: false,
        migration_state: ExportImportMigrationState::RequiredSupported,
        accepted_sections: partial_acceptances.to_vec(),
        rejected_sections: partial_rejections.to_vec(),
        no_default_support_decrypt: true,
    }
}

fn negative_preflights() -> Vec<ExportImportImportPreflight> {
    let mut preflights = negative_preflight_policy_rejections();
    preflights.extend(negative_preflight_retention_rejections());
    preflights
}

fn negative_preflight_policy_rejections() -> Vec<ExportImportImportPreflight> {
    vec![
        negative_preflight(NegativePreflightInput {
            state: ExportImportPreflightState::SchemaVersionInvalid,
            migration_state: ExportImportMigrationState::NotRequired,
            schema_version_supported: false,
            household_binding_match: true,
            key_available: true,
            integrity_ok: true,
            duplicate_device_detected: false,
            rejected_sections: Vec::new(),
        }),
        negative_preflight(NegativePreflightInput {
            state: ExportImportPreflightState::MigrationUnsupported,
            migration_state: ExportImportMigrationState::RequiredUnsupported,
            schema_version_supported: true,
            household_binding_match: true,
            key_available: true,
            integrity_ok: true,
            duplicate_device_detected: false,
            rejected_sections: Vec::new(),
        }),
        negative_preflight(NegativePreflightInput {
            state: ExportImportPreflightState::HouseholdMismatch,
            migration_state: ExportImportMigrationState::NotRequired,
            schema_version_supported: true,
            household_binding_match: false,
            key_available: true,
            integrity_ok: true,
            duplicate_device_detected: false,
            rejected_sections: Vec::new(),
        }),
        negative_preflight(NegativePreflightInput {
            state: ExportImportPreflightState::KeyUnavailable,
            migration_state: ExportImportMigrationState::NotRequired,
            schema_version_supported: true,
            household_binding_match: true,
            key_available: false,
            integrity_ok: true,
            duplicate_device_detected: false,
            rejected_sections: Vec::new(),
        }),
        negative_preflight(NegativePreflightInput {
            state: ExportImportPreflightState::BundleCorrupt,
            migration_state: ExportImportMigrationState::NotRequired,
            schema_version_supported: true,
            household_binding_match: true,
            key_available: true,
            integrity_ok: false,
            duplicate_device_detected: false,
            rejected_sections: Vec::new(),
        }),
    ]
}

fn negative_preflight_retention_rejections() -> Vec<ExportImportImportPreflight> {
    vec![
        negative_preflight(NegativePreflightInput {
            state: ExportImportPreflightState::RetentionExpired,
            migration_state: ExportImportMigrationState::NotRequired,
            schema_version_supported: true,
            household_binding_match: true,
            key_available: true,
            integrity_ok: true,
            duplicate_device_detected: false,
            rejected_sections: vec![rejection(
                ExportImportDataClass::Screenshots,
                ExportImportSectionDecisionState::RetentionExpired,
                EXPORT_IMPORT_NEGATIVE_REASON_ALL_EXPIRED,
            )],
        }),
        negative_preflight(NegativePreflightInput {
            state: ExportImportPreflightState::DeviceDuplicate,
            migration_state: ExportImportMigrationState::NotRequired,
            schema_version_supported: true,
            household_binding_match: true,
            key_available: true,
            integrity_ok: true,
            duplicate_device_detected: true,
            rejected_sections: vec![rejection(
                ExportImportDataClass::DeviceRegistry,
                ExportImportSectionDecisionState::DuplicateDevice,
                EXPORT_IMPORT_NEGATIVE_REASON_DUPLICATE_DEVICE,
            )],
        }),
    ]
}

fn restore_apply(
    partial_acceptances: Vec<ExportImportSectionDecision>,
    partial_rejections: Vec<ExportImportSectionDecision>,
) -> ExportImportRestoreApplyResult {
    ExportImportRestoreApplyResult {
        state: ExportImportRestoreApplyState::Partial,
        explicit_confirmation_required: false,
        local_truth_authoritative: true,
        tombstones_preserved: true,
        idempotent: true,
        accepted_sections: partial_acceptances,
        rejected_sections: partial_rejections,
        duplicates_created: false,
        no_default_support_decrypt: true,
    }
}

fn section(seed: SectionSeed) -> ExportImportPayloadSection {
    ExportImportPayloadSection {
        data_class: seed.data_class,
        payload_ref: payload_ref(seed.payload_ref),
        payload_integrity_ref: integrity_ref(seed.payload_integrity_ref),
        encrypted: true,
        retention_state: seed.retention_state,
        support_default_decryptable: false,
        included_in_human_summary: seed.included_in_human_summary,
        notes: seed.notes.to_string(),
    }
}

fn acceptance(data_class: ExportImportDataClass, reason: &str) -> ExportImportSectionDecision {
    ExportImportSectionDecision {
        data_class,
        state: ExportImportSectionDecisionState::Accepted,
        reason: reason.to_string(),
    }
}

fn rejection(
    data_class: ExportImportDataClass,
    state: ExportImportSectionDecisionState,
    reason: &str,
) -> ExportImportSectionDecision {
    ExportImportSectionDecision {
        data_class,
        state,
        reason: reason.to_string(),
    }
}

fn negative_preflight(input: NegativePreflightInput) -> ExportImportImportPreflight {
    let NegativePreflightInput {
        state,
        migration_state,
        schema_version_supported,
        household_binding_match,
        key_available,
        integrity_ok,
        duplicate_device_detected,
        rejected_sections,
    } = input;

    ExportImportImportPreflight {
        state,
        schema_version_supported,
        household_binding_match,
        key_available,
        manifest_integrity_verified: integrity_ok,
        payload_integrity_verified: integrity_ok,
        local_truth_mutated: false,
        tombstones_preserved: true,
        duplicate_device_detected,
        migration_state,
        accepted_sections: Vec::new(),
        rejected_sections,
        no_default_support_decrypt: true,
    }
}
