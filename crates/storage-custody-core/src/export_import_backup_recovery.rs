use std::collections::BTreeSet;

use ocentra_schema::export_import_backup_recovery as contracts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportBundleBuildRequest {
    pub bundle_id: contracts::ExportImportBundleId,
    pub product_version: contracts::ExportImportProductVersion,
    pub created_at: contracts::ExportImportTimestamp,
    pub household: contracts::ExportImportHouseholdReference,
    pub source_device_id: Option<contracts::ExportImportDeviceId>,
    pub bundle_type: contracts::ExportImportBundleType,
    pub key_ref: contracts::ExportImportKeyRef,
    pub manifest_integrity_ref: Option<contracts::ExportImportIntegrityRef>,
    pub tombstone_cursor: Option<contracts::ExportImportTombstoneCursor>,
    pub retention_notes: Vec<String>,
    pub proof_tier: contracts::ExportImportProofTier,
    pub migration_ref: Option<contracts::ExportImportMigrationRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPayloadSectionInput {
    pub data_class: contracts::ExportImportDataClass,
    pub payload_ref: contracts::ExportImportPayloadRef,
    pub payload_integrity_ref: Option<contracts::ExportImportIntegrityRef>,
    pub encrypted: bool,
    pub retention_state: contracts::ExportImportSectionRetentionState,
    pub support_default_decryptable: bool,
    pub included_in_human_summary: bool,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportHumanSummaryInput {
    pub headline: String,
    pub excluded_data_classes: Vec<contracts::ExportImportDataClass>,
    pub raw_payload_redacted: bool,
    pub support_safe: bool,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportBundleBuildError {
    EmptySections,
    MissingManifestIntegrity,
    MissingPayloadIntegrity(contracts::ExportImportDataClass),
    SectionNotEncrypted(contracts::ExportImportDataClass),
    SupportDefaultDecryptForbidden(contracts::ExportImportDataClass),
    DuplicateDataClass(contracts::ExportImportDataClass),
    SummaryMustBeRedacted,
    SummaryMustBeSupportSafe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportBundleContext {
    pub local_household_id: contracts::ExportImportHouseholdId,
    pub local_product_version: contracts::ExportImportProductVersion,
    pub available_key_refs: Vec<contracts::ExportImportKeyRef>,
    pub supported_schema_versions: Vec<String>,
    pub blocked_restore_data_classes: Vec<contracts::ExportImportDataClass>,
    pub known_device_ids: Vec<contracts::ExportImportDeviceId>,
    pub target_device_id: Option<contracts::ExportImportDeviceId>,
    pub migration_supported: bool,
    pub manifest_integrity_ok: bool,
    pub payload_integrity_failures: Vec<contracts::ExportImportDataClass>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreApplyRequest {
    pub confirmed: bool,
}

struct RejectedPreflightInput {
    state: contracts::ExportImportPreflightState,
    migration_state: contracts::ExportImportMigrationState,
    schema_version_supported: bool,
    household_binding_match: bool,
    key_available: bool,
    integrity_ok: bool,
    duplicate_device_detected: bool,
    rejected_sections: Vec<contracts::ExportImportSectionDecision>,
}

pub fn derive_export_bundle(
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

pub fn run_import_preflight(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> contracts::ExportImportImportPreflight {
    if let Some(rejected) = import_preflight_rejection(bundle, context) {
        return rejected_preflight(rejected);
    }

    let migration_state = import_preflight_migration_state(bundle, context);
    let (state, accepted_sections, rejected_sections) =
        import_preflight_section_decisions(bundle, context);

    contracts::ExportImportImportPreflight {
        state,
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        manifest_integrity_verified: true,
        payload_integrity_verified: true,
        local_truth_mutated: false,
        tombstones_preserved: true,
        duplicate_device_detected: false,
        migration_state,
        accepted_sections,
        rejected_sections,
        no_default_support_decrypt: true,
    }
}

fn import_preflight_rejection(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    reject_schema_version(bundle, context)
        .or_else(|| reject_wrong_household(bundle, context))
        .or_else(|| reject_wrong_key(bundle, context))
        .or_else(|| reject_corrupt_bundle(bundle, context))
        .or_else(|| reject_migration_unsupported(bundle, context))
        .or_else(|| reject_duplicate_device(bundle, context))
}

fn reject_schema_version(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let schema_version_supported = context
        .supported_schema_versions
        .iter()
        .any(|version| version == &bundle.manifest.schema_version);
    if schema_version_supported {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::RejectedSchemaVersion,
        migration_state: contracts::ExportImportMigrationState::NotRequired,
        schema_version_supported,
        household_binding_match: true,
        key_available: true,
        integrity_ok: true,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_wrong_household(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let household_binding_match = bundle.manifest.source_household_id == context.local_household_id;
    if household_binding_match {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::RejectedWrongHousehold,
        migration_state: contracts::ExportImportMigrationState::NotRequired,
        schema_version_supported: true,
        household_binding_match: false,
        key_available: true,
        integrity_ok: true,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_wrong_key(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let key_available = context
        .available_key_refs
        .iter()
        .any(|key_ref| key_ref == &bundle.manifest.key_ref);
    if key_available {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::RejectedWrongKey,
        migration_state: contracts::ExportImportMigrationState::NotRequired,
        schema_version_supported: true,
        household_binding_match: true,
        key_available: false,
        integrity_ok: true,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_corrupt_bundle(
    _bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let payload_integrity_verified = context.payload_integrity_failures.is_empty();
    if context.manifest_integrity_ok && payload_integrity_verified {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::RejectedCorruptBundle,
        migration_state: contracts::ExportImportMigrationState::NotRequired,
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        integrity_ok: false,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_migration_unsupported(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let migration_state = import_preflight_migration_state(bundle, context);
    if migration_state != contracts::ExportImportMigrationState::RequiredUnsupported {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::RejectedMigrationUnsupported,
        migration_state,
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        integrity_ok: true,
        duplicate_device_detected: false,
        rejected_sections: Vec::new(),
    })
}

fn reject_duplicate_device(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> Option<RejectedPreflightInput> {
    let duplicate_device_detected = import_preflight_duplicate_device_detected(bundle, context);
    if !duplicate_device_detected {
        return None;
    }
    Some(RejectedPreflightInput {
        state: contracts::ExportImportPreflightState::RejectedDuplicateDevice,
        migration_state: import_preflight_migration_state(bundle, context),
        schema_version_supported: true,
        household_binding_match: true,
        key_available: true,
        integrity_ok: true,
        duplicate_device_detected: true,
        rejected_sections: vec![contracts::ExportImportSectionDecision {
            data_class: contracts::ExportImportDataClass::DeviceRegistry,
            state: contracts::ExportImportSectionDecisionState::RejectedDuplicateDevice,
            reason: "Existing local device identity would be duplicated by restore.".to_string(),
        }],
    })
}

fn import_preflight_migration_state(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> contracts::ExportImportMigrationState {
    if bundle.manifest.product_version == context.local_product_version {
        contracts::ExportImportMigrationState::NotRequired
    } else if context.migration_supported {
        contracts::ExportImportMigrationState::RequiredSupported
    } else {
        contracts::ExportImportMigrationState::RequiredUnsupported
    }
}

fn import_preflight_duplicate_device_detected(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> bool {
    bundle
        .manifest
        .source_device_id
        .as_ref()
        .map(|source_device_id| {
            context
                .known_device_ids
                .iter()
                .any(|known_device_id| known_device_id == source_device_id)
                && context.target_device_id.as_ref() != Some(source_device_id)
        })
        .unwrap_or(false)
}

fn import_preflight_section_decisions(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> (
    contracts::ExportImportPreflightState,
    Vec<contracts::ExportImportSectionDecision>,
    Vec<contracts::ExportImportSectionDecision>,
) {
    let mut accepted_sections = Vec::new();
    let mut rejected_sections = Vec::new();

    for section in &bundle.sections {
        let blocked_by_tombstone = context
            .blocked_restore_data_classes
            .iter()
            .any(|data_class| data_class == &section.data_class)
            || section.retention_state == contracts::ExportImportSectionRetentionState::Tombstoned;

        if section.retention_state == contracts::ExportImportSectionRetentionState::Expired {
            rejected_sections.push(contracts::ExportImportSectionDecision {
                data_class: section.data_class,
                state: contracts::ExportImportSectionDecisionState::RejectedExpiredRetention,
                reason: "Retention expired before restore preview.".to_string(),
            });
            continue;
        }

        if blocked_by_tombstone {
            rejected_sections.push(contracts::ExportImportSectionDecision {
                data_class: section.data_class,
                state: contracts::ExportImportSectionDecisionState::RejectedTombstonePreserved,
                reason: "Local tombstone ordering blocks section resurrection.".to_string(),
            });
            continue;
        }

        accepted_sections.push(contracts::ExportImportSectionDecision {
            data_class: section.data_class,
            state: contracts::ExportImportSectionDecisionState::Accepted,
            reason: "Section passed household, key, integrity, and retention preflight."
                .to_string(),
        });
    }

    let state = if accepted_sections.is_empty() {
        if rejected_sections.iter().all(|decision| {
            decision.state == contracts::ExportImportSectionDecisionState::RejectedExpiredRetention
        }) {
            contracts::ExportImportPreflightState::RejectedExpiredRetention
        } else {
            contracts::ExportImportPreflightState::RejectedTombstoneConflict
        }
    } else if rejected_sections.is_empty() {
        contracts::ExportImportPreflightState::AcceptedPreview
    } else {
        contracts::ExportImportPreflightState::PartialPreview
    };

    (state, accepted_sections, rejected_sections)
}

pub fn apply_restore(
    preflight: &contracts::ExportImportImportPreflight,
    request: &RestoreApplyRequest,
) -> contracts::ExportImportRestoreApplyResult {
    let state = match preflight.state {
        contracts::ExportImportPreflightState::AcceptedPreview
        | contracts::ExportImportPreflightState::PartialPreview => {
            if request.confirmed {
                if preflight.state == contracts::ExportImportPreflightState::PartialPreview {
                    contracts::ExportImportRestoreApplyState::Partial
                } else {
                    contracts::ExportImportRestoreApplyState::Applied
                }
            } else {
                contracts::ExportImportRestoreApplyState::ApplyPending
            }
        }
        contracts::ExportImportPreflightState::RejectedWrongHousehold => {
            contracts::ExportImportRestoreApplyState::WrongHousehold
        }
        contracts::ExportImportPreflightState::RejectedWrongKey => {
            contracts::ExportImportRestoreApplyState::WrongKey
        }
        contracts::ExportImportPreflightState::RejectedCorruptBundle => {
            contracts::ExportImportRestoreApplyState::Corrupt
        }
        _ => contracts::ExportImportRestoreApplyState::Blocked,
    };

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

fn rejected_preflight(input: RejectedPreflightInput) -> contracts::ExportImportImportPreflight {
    contracts::ExportImportImportPreflight {
        state: input.state,
        schema_version_supported: input.schema_version_supported,
        household_binding_match: input.household_binding_match,
        key_available: input.key_available,
        manifest_integrity_verified: input.integrity_ok,
        payload_integrity_verified: input.integrity_ok,
        local_truth_mutated: false,
        tombstones_preserved: true,
        duplicate_device_detected: input.duplicate_device_detected,
        migration_state: input.migration_state,
        accepted_sections: Vec::new(),
        rejected_sections: input.rejected_sections,
        no_default_support_decrypt: true,
    }
}
