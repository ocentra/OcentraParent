use std::fmt::{Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod identifiers;
mod sample;

pub const EXPORT_IMPORT_BACKUP_RECOVERY_SCHEMA_VERSION: &str =
    "export-import-backup-recovery-proof";

const EXPORT_IMPORT_BUNDLE_TYPE_EXPORT: &str = "export";
const EXPORT_IMPORT_BUNDLE_TYPE_BACKUP: &str = "backup";
const EXPORT_IMPORT_BUNDLE_TYPE_IMPORT_PREVIEW: &str = "importPreview";
const EXPORT_IMPORT_BUNDLE_TYPE_RESTORE: &str = "restore";
const EXPORT_IMPORT_BUNDLE_TYPE_SUPPORT: &str = "support";
const EXPORT_IMPORT_DATA_CLASS_CONFIG_METADATA: &str = "config-metadata";
const EXPORT_IMPORT_DATA_CLASS_ACCOUNT_METADATA: &str = "account-metadata";
const EXPORT_IMPORT_DATA_CLASS_POLICY_HISTORY: &str = "policy-history";
const EXPORT_IMPORT_DATA_CLASS_DEVICE_REGISTRY: &str = "device-registry";
const EXPORT_IMPORT_DATA_CLASS_EVIDENCE_JOURNAL: &str = "evidence-journal";
const EXPORT_IMPORT_DATA_CLASS_LOGS: &str = "logs";
const EXPORT_IMPORT_DATA_CLASS_SCREENSHOTS: &str = "screenshots";
const EXPORT_IMPORT_DATA_CLASS_NETWORK_ARTIFACTS: &str = "network-artifacts";
const EXPORT_IMPORT_DATA_CLASS_AI_OUTPUTS: &str = "ai-outputs";
const EXPORT_IMPORT_DATA_CLASS_REPORTS: &str = "reports";
const EXPORT_IMPORT_DATA_CLASS_NOTIFICATIONS: &str = "notifications";
const EXPORT_IMPORT_DATA_CLASS_BILLING_REFERENCES: &str = "billing-references";
const EXPORT_IMPORT_ENCRYPTION_MODE_PER_CLASS_ENVELOPE_ENCRYPTED: &str =
    "per-class-envelope-encrypted";
const EXPORT_IMPORT_INTEGRITY_MODE_MANIFEST_AND_PAYLOAD_HASHES: &str =
    "manifest-and-payload-hashes";
const EXPORT_IMPORT_PROOF_TIER_CONTRACT_ONLY: &str = "contract-only";
const EXPORT_IMPORT_PROOF_TIER_RUNTIME_VALIDATED: &str = "runtime-validated";
const EXPORT_IMPORT_PROOF_TIER_MANUAL_REQUIRED: &str = "manual-required";
const EXPORT_IMPORT_SECTION_RETENTION_STATE_ACTIVE: &str = "active";
const EXPORT_IMPORT_SECTION_RETENTION_STATE_EXPIRED: &str = "expired";
const EXPORT_IMPORT_SECTION_RETENTION_STATE_TOMBSTONED: &str = "tombstoned";
const EXPORT_IMPORT_MIGRATION_STATE_NOT_REQUIRED: &str = "not-required";
const EXPORT_IMPORT_MIGRATION_STATE_REQUIRED_SUPPORTED: &str = "required-supported";
const EXPORT_IMPORT_MIGRATION_STATE_REQUIRED_UNSUPPORTED: &str = "required-unsupported";
const EXPORT_IMPORT_PREFLIGHT_STATE_ACCEPTED_PREVIEW: &str = "acceptedPreview";
const EXPORT_IMPORT_PREFLIGHT_STATE_PARTIAL_PREVIEW: &str = "partialPreview";
const EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_SCHEMA_VERSION: &str = "rejectedSchemaVersion";
const EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_MIGRATION_UNSUPPORTED: &str =
    "rejectedMigrationUnsupported";
const EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_WRONG_HOUSEHOLD: &str = "rejectedWrongHousehold";
const EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_WRONG_KEY: &str = "rejectedWrongKey";
const EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_CORRUPT_BUNDLE: &str = "rejectedCorruptBundle";
const EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_EXPIRED_RETENTION: &str = "rejectedExpiredRetention";
const EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_DUPLICATE_DEVICE: &str = "rejectedDuplicateDevice";
const EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_TOMBSTONE_CONFLICT: &str = "rejectedTombstoneConflict";
const EXPORT_IMPORT_SECTION_DECISION_STATE_ACCEPTED: &str = "accepted";
const EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_EXPIRED_RETENTION: &str =
    "rejectedExpiredRetention";
const EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_TOMBSTONE_PRESERVED: &str =
    "rejectedTombstonePreserved";
const EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_DUPLICATE_DEVICE: &str =
    "rejectedDuplicateDevice";
const EXPORT_IMPORT_RESTORE_APPLY_STATE_NOT_APPLIED: &str = "notApplied";
const EXPORT_IMPORT_RESTORE_APPLY_STATE_APPLY_PENDING: &str = "applyPending";
const EXPORT_IMPORT_RESTORE_APPLY_STATE_APPLIED: &str = "applied";
const EXPORT_IMPORT_RESTORE_APPLY_STATE_PARTIAL: &str = "partial";
const EXPORT_IMPORT_RESTORE_APPLY_STATE_WRONG_HOUSEHOLD: &str = "wrongHousehold";
const EXPORT_IMPORT_RESTORE_APPLY_STATE_WRONG_KEY: &str = "wrongKey";
const EXPORT_IMPORT_RESTORE_APPLY_STATE_CORRUPT: &str = "corrupt";
const EXPORT_IMPORT_RESTORE_APPLY_STATE_BLOCKED: &str = "blocked";
const EXPORT_IMPORT_NON_CLAIM_NO_PROVIDER_RUNTIME: &str = "no-provider-runtime";
const EXPORT_IMPORT_NON_CLAIM_NO_AUTO_APPLY: &str = "no-auto-apply";
const EXPORT_IMPORT_NON_CLAIM_NO_DEFAULT_SUPPORT_DECRYPT: &str = "no-default-support-decrypt";
const EXPORT_IMPORT_NON_CLAIM_NO_TS_BUSINESS_OWNER: &str = "no-ts-business-owner";
const EXPORT_IMPORT_NON_CLAIM_NO_LAN_OWNERSHIP: &str = "no-lan-ownership";
const EXPORT_IMPORT_CONTRACT_VERSION_V0_5: &str = "v0.5";
const EXPORT_IMPORT_BUNDLE_ID_PROOF_1: &str = "bundle-proof-1";
const EXPORT_IMPORT_PRODUCT_VERSION_2026_06_28: &str = "2026.06.28";
const EXPORT_IMPORT_CREATED_AT: &str = "2026-06-28T18:30:00.000Z";
const EXPORT_IMPORT_SOURCE_HOUSEHOLD_ID_PROOF_1: &str = "family-export-proof-1";
const EXPORT_IMPORT_SOURCE_DEVICE_ID_PROOF_1: &str = "child-device-proof-1";
const EXPORT_IMPORT_PARENT_KEY_PROOF_1: &str = "parent-key-proof-1";
const EXPORT_IMPORT_MANIFEST_SHA256_PROOF_1: &str = "manifest-sha256-proof-1";
const EXPORT_IMPORT_TOMBSTONE_CURSOR_PROOF_7: &str = "tombstone-cursor-proof-7";
const EXPORT_IMPORT_MIGRATION_PROOF_1: &str = "migration-proof-1";
const EXPORT_IMPORT_PAYLOAD_EVIDENCE_PROOF_1: &str = "payload-evidence-proof-1";
const EXPORT_IMPORT_PAYLOAD_SHA256_EVIDENCE_PROOF_1: &str = "payload-sha256-evidence-proof-1";
const EXPORT_IMPORT_PAYLOAD_REPORT_PROOF_1: &str = "payload-report-proof-1";
const EXPORT_IMPORT_PAYLOAD_SHA256_REPORT_PROOF_1: &str = "payload-sha256-report-proof-1";
const EXPORT_IMPORT_PAYLOAD_SCREENSHOT_PROOF_1: &str = "payload-screenshot-proof-1";
const EXPORT_IMPORT_PAYLOAD_SHA256_SCREENSHOT_PROOF_1: &str = "payload-sha256-screenshot-proof-1";
const EXPORT_IMPORT_PAYLOAD_NOTIFICATION_PROOF_1: &str = "payload-notification-proof-1";
const EXPORT_IMPORT_PAYLOAD_SHA256_NOTIFICATION_PROOF_1: &str =
    "payload-sha256-notification-proof-1";
const EXPORT_IMPORT_UPDATED_AT: &str = "2026-06-28T18:35:00.000Z";
const EXPORT_IMPORT_EXPECT_CONTRACT_VERSION: &str = "contract version";
const EXPORT_IMPORT_EXPECT_BUNDLE_ID: &str = "bundle id";
const EXPORT_IMPORT_EXPECT_HOUSEHOLD_ID: &str = "household id";
const EXPORT_IMPORT_EXPECT_DEVICE_ID: &str = "device id";
const EXPORT_IMPORT_EXPECT_KEY_REF: &str = "key ref";
const EXPORT_IMPORT_EXPECT_PAYLOAD_REF: &str = "payload ref";
const EXPORT_IMPORT_EXPECT_INTEGRITY_REF: &str = "integrity ref";
const EXPORT_IMPORT_EXPECT_TOMBSTONE_CURSOR: &str = "tombstone cursor";
const EXPORT_IMPORT_EXPECT_TIMESTAMP: &str = "timestamp";
const EXPORT_IMPORT_EXPECT_PRODUCT_VERSION: &str = "product version";
const EXPORT_IMPORT_EXPECT_MIGRATION_REF: &str = "migration ref";
const EXPORT_IMPORT_RETENTION_NOTE_TOMBSTONE_ORDERING: &str =
    "Evidence bundles preserve tombstone ordering and reject resurrection.";
const EXPORT_IMPORT_RETENTION_NOTE_EXPIRED_PREVIEW: &str =
    "Expired sections are preview-visible but fail closed during restore.";
const EXPORT_IMPORT_SECTION_NOTE_EVIDENCE_JOURNAL: &str =
    "Encrypted journal segment remains portable but not support-readable.";
const EXPORT_IMPORT_SECTION_NOTE_REPORTS: &str =
    "Redacted report summary may be previewed without exposing raw child payload.";
const EXPORT_IMPORT_SECTION_NOTE_SCREENSHOTS: &str =
    "Expired screenshot section remains encrypted and is rejected during restore preview.";
const EXPORT_IMPORT_SECTION_NOTE_NOTIFICATIONS: &str =
    "Notification payloads remain redacted and cannot revive tombstoned source evidence.";
const EXPORT_IMPORT_HUMAN_SUMMARY_HEADLINE: &str = "Parent-authored backup bundle";
const EXPORT_IMPORT_HUMAN_SUMMARY_NOTES: &str =
    "Support sees redacted class names, counts, and proof refs only.";
const EXPORT_IMPORT_REJECTION_REASON_EXPIRED_RETENTION: &str =
    "Section expired under retention policy and is preview-only.";
const EXPORT_IMPORT_REJECTION_REASON_TOMBSTONE_PRESERVED: &str =
    "Section is blocked by tombstone preservation and cannot revive deleted truth.";
const EXPORT_IMPORT_ACCEPTANCE_REASON_EVIDENCE_JOURNAL: &str =
    "Encrypted journal segment is household-bound and restore-eligible.";
const EXPORT_IMPORT_ACCEPTANCE_REASON_REPORTS: &str =
    "Derived report bundle stays redacted and can be restored safely.";
const EXPORT_IMPORT_NEGATIVE_REASON_ALL_EXPIRED: &str =
    "All previewed sections were expired at import time.";
const EXPORT_IMPORT_NEGATIVE_REASON_DUPLICATE_DEVICE: &str =
    "Source device would duplicate an existing local device record.";

macro_rules! export_import_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! export_import_string_enum {
    ($name:ident { $($variant:ident => $value:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $(
                $variant,
            )+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                match value.as_str() {
                    $(candidate if candidate == $value => Ok(Self::$variant),)+
                    _ => Err(serde::de::Error::custom(format!("invalid {}: {}", stringify!($name), value))),
                }
            }
        }
    };
}

export_import_string_enum!(ExportImportBundleType {
    Export => EXPORT_IMPORT_BUNDLE_TYPE_EXPORT,
    Backup => EXPORT_IMPORT_BUNDLE_TYPE_BACKUP,
    ImportPreview => EXPORT_IMPORT_BUNDLE_TYPE_IMPORT_PREVIEW,
    Restore => EXPORT_IMPORT_BUNDLE_TYPE_RESTORE,
    Support => EXPORT_IMPORT_BUNDLE_TYPE_SUPPORT,
});

export_import_string_enum!(ExportImportDataClass {
    ConfigMetadata => EXPORT_IMPORT_DATA_CLASS_CONFIG_METADATA,
    AccountMetadata => EXPORT_IMPORT_DATA_CLASS_ACCOUNT_METADATA,
    PolicyHistory => EXPORT_IMPORT_DATA_CLASS_POLICY_HISTORY,
    DeviceRegistry => EXPORT_IMPORT_DATA_CLASS_DEVICE_REGISTRY,
    EvidenceJournal => EXPORT_IMPORT_DATA_CLASS_EVIDENCE_JOURNAL,
    Logs => EXPORT_IMPORT_DATA_CLASS_LOGS,
    Screenshots => EXPORT_IMPORT_DATA_CLASS_SCREENSHOTS,
    NetworkArtifacts => EXPORT_IMPORT_DATA_CLASS_NETWORK_ARTIFACTS,
    AiOutputs => EXPORT_IMPORT_DATA_CLASS_AI_OUTPUTS,
    Reports => EXPORT_IMPORT_DATA_CLASS_REPORTS,
    Notifications => EXPORT_IMPORT_DATA_CLASS_NOTIFICATIONS,
    BillingReferences => EXPORT_IMPORT_DATA_CLASS_BILLING_REFERENCES,
});

export_import_string_enum!(ExportImportEncryptionMode {
    PerClassEnvelopeEncrypted => EXPORT_IMPORT_ENCRYPTION_MODE_PER_CLASS_ENVELOPE_ENCRYPTED,
});

export_import_string_enum!(ExportImportIntegrityMode {
    ManifestAndPayloadHashes => EXPORT_IMPORT_INTEGRITY_MODE_MANIFEST_AND_PAYLOAD_HASHES,
});

export_import_string_enum!(ExportImportProofTier {
    ContractOnly => EXPORT_IMPORT_PROOF_TIER_CONTRACT_ONLY,
    RuntimeValidated => EXPORT_IMPORT_PROOF_TIER_RUNTIME_VALIDATED,
    ManualRequired => EXPORT_IMPORT_PROOF_TIER_MANUAL_REQUIRED,
});

export_import_string_enum!(ExportImportSectionRetentionState {
    Active => EXPORT_IMPORT_SECTION_RETENTION_STATE_ACTIVE,
    Expired => EXPORT_IMPORT_SECTION_RETENTION_STATE_EXPIRED,
    Tombstoned => EXPORT_IMPORT_SECTION_RETENTION_STATE_TOMBSTONED,
});

export_import_string_enum!(ExportImportMigrationState {
    NotRequired => EXPORT_IMPORT_MIGRATION_STATE_NOT_REQUIRED,
    RequiredSupported => EXPORT_IMPORT_MIGRATION_STATE_REQUIRED_SUPPORTED,
    RequiredUnsupported => EXPORT_IMPORT_MIGRATION_STATE_REQUIRED_UNSUPPORTED,
});

export_import_string_enum!(ExportImportPreflightState {
    AcceptedPreview => EXPORT_IMPORT_PREFLIGHT_STATE_ACCEPTED_PREVIEW,
    PartialPreview => EXPORT_IMPORT_PREFLIGHT_STATE_PARTIAL_PREVIEW,
    RejectedSchemaVersion => EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_SCHEMA_VERSION,
    RejectedMigrationUnsupported => EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_MIGRATION_UNSUPPORTED,
    RejectedWrongHousehold => EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_WRONG_HOUSEHOLD,
    RejectedWrongKey => EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_WRONG_KEY,
    RejectedCorruptBundle => EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_CORRUPT_BUNDLE,
    RejectedExpiredRetention => EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_EXPIRED_RETENTION,
    RejectedDuplicateDevice => EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_DUPLICATE_DEVICE,
    RejectedTombstoneConflict => EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_TOMBSTONE_CONFLICT,
});

export_import_string_enum!(ExportImportSectionDecisionState {
    Accepted => EXPORT_IMPORT_SECTION_DECISION_STATE_ACCEPTED,
    RejectedExpiredRetention => EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_EXPIRED_RETENTION,
    RejectedTombstonePreserved => EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_TOMBSTONE_PRESERVED,
    RejectedDuplicateDevice => EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_DUPLICATE_DEVICE,
});

export_import_string_enum!(ExportImportRestoreApplyState {
    NotApplied => EXPORT_IMPORT_RESTORE_APPLY_STATE_NOT_APPLIED,
    ApplyPending => EXPORT_IMPORT_RESTORE_APPLY_STATE_APPLY_PENDING,
    Applied => EXPORT_IMPORT_RESTORE_APPLY_STATE_APPLIED,
    Partial => EXPORT_IMPORT_RESTORE_APPLY_STATE_PARTIAL,
    WrongHousehold => EXPORT_IMPORT_RESTORE_APPLY_STATE_WRONG_HOUSEHOLD,
    WrongKey => EXPORT_IMPORT_RESTORE_APPLY_STATE_WRONG_KEY,
    Corrupt => EXPORT_IMPORT_RESTORE_APPLY_STATE_CORRUPT,
    Blocked => EXPORT_IMPORT_RESTORE_APPLY_STATE_BLOCKED,
});

export_import_string_enum!(ExportImportNonClaim {
    NoProviderRuntime => EXPORT_IMPORT_NON_CLAIM_NO_PROVIDER_RUNTIME,
    NoAutoApply => EXPORT_IMPORT_NON_CLAIM_NO_AUTO_APPLY,
    NoDefaultSupportDecrypt => EXPORT_IMPORT_NON_CLAIM_NO_DEFAULT_SUPPORT_DECRYPT,
    NoTsBusinessOwner => EXPORT_IMPORT_NON_CLAIM_NO_TS_BUSINESS_OWNER,
    NoLanOwnership => EXPORT_IMPORT_NON_CLAIM_NO_LAN_OWNERSHIP,
});

export_import_text_identifier!(ExportImportContractVersion);
export_import_text_identifier!(ExportImportBundleId);
export_import_text_identifier!(ExportImportHouseholdId);
export_import_text_identifier!(ExportImportDeviceId);
export_import_text_identifier!(ExportImportKeyRef);
export_import_text_identifier!(ExportImportPayloadRef);
export_import_text_identifier!(ExportImportIntegrityRef);
export_import_text_identifier!(ExportImportTombstoneCursor);
export_import_text_identifier!(ExportImportTimestamp);
export_import_text_identifier!(ExportImportProductVersion);
export_import_text_identifier!(ExportImportMigrationRef);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportHouseholdReference {
    pub household_id: ExportImportHouseholdId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportRecoveryBundleManifest {
    pub bundle_id: ExportImportBundleId,
    pub schema_version: String,
    pub product_version: ExportImportProductVersion,
    pub created_at: ExportImportTimestamp,
    pub source_household_id: ExportImportHouseholdId,
    pub source_device_id: Option<ExportImportDeviceId>,
    pub bundle_type: ExportImportBundleType,
    pub data_classes: Vec<ExportImportDataClass>,
    pub encryption_mode: ExportImportEncryptionMode,
    pub key_ref: ExportImportKeyRef,
    pub manifest_integrity_ref: ExportImportIntegrityRef,
    pub payload_integrity_mode: ExportImportIntegrityMode,
    pub tombstone_cursor: Option<ExportImportTombstoneCursor>,
    pub retention_notes: Vec<String>,
    pub proof_tier: ExportImportProofTier,
    pub migration_ref: Option<ExportImportMigrationRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportPayloadSection {
    pub data_class: ExportImportDataClass,
    pub payload_ref: ExportImportPayloadRef,
    pub payload_integrity_ref: ExportImportIntegrityRef,
    pub encrypted: bool,
    pub retention_state: ExportImportSectionRetentionState,
    pub support_default_decryptable: bool,
    pub included_in_human_summary: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportHumanSummary {
    pub headline: String,
    pub included_data_classes: Vec<ExportImportDataClass>,
    pub excluded_data_classes: Vec<ExportImportDataClass>,
    pub raw_payload_redacted: bool,
    pub support_safe: bool,
    pub notes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportRecoveryBundle {
    pub manifest: ExportImportRecoveryBundleManifest,
    pub sections: Vec<ExportImportPayloadSection>,
    pub human_summary: ExportImportHumanSummary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportSectionDecision {
    pub data_class: ExportImportDataClass,
    pub state: ExportImportSectionDecisionState,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportImportPreflight {
    pub state: ExportImportPreflightState,
    pub schema_version_supported: bool,
    pub household_binding_match: bool,
    pub key_available: bool,
    pub manifest_integrity_verified: bool,
    pub payload_integrity_verified: bool,
    pub local_truth_mutated: bool,
    pub tombstones_preserved: bool,
    pub duplicate_device_detected: bool,
    pub migration_state: ExportImportMigrationState,
    pub accepted_sections: Vec<ExportImportSectionDecision>,
    pub rejected_sections: Vec<ExportImportSectionDecision>,
    pub no_default_support_decrypt: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportRestoreApplyResult {
    pub state: ExportImportRestoreApplyState,
    pub explicit_confirmation_required: bool,
    pub local_truth_authoritative: bool,
    pub tombstones_preserved: bool,
    pub idempotent: bool,
    pub accepted_sections: Vec<ExportImportSectionDecision>,
    pub rejected_sections: Vec<ExportImportSectionDecision>,
    pub duplicates_created: bool,
    pub no_default_support_decrypt: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportImportBackupRecoveryContractProof {
    pub schema_version: String,
    pub contract_version: ExportImportContractVersion,
    pub bundle: ExportImportRecoveryBundle,
    pub import_preflight: ExportImportImportPreflight,
    pub negative_preflights: Vec<ExportImportImportPreflight>,
    pub restore_apply: ExportImportRestoreApplyResult,
    pub non_claims: Vec<ExportImportNonClaim>,
    pub provider_runtime_claimed: bool,
    pub support_default_child_evidence_decryption: bool,
    pub ts_business_owner_claimed: bool,
    pub updated_at: ExportImportTimestamp,
}

#[derive(Clone, Copy)]
struct SectionSeed {
    data_class: ExportImportDataClass,
    payload_ref: &'static str,
    payload_integrity_ref: &'static str,
    retention_state: ExportImportSectionRetentionState,
    included_in_human_summary: bool,
    notes: &'static str,
}

struct NegativePreflightInput {
    state: ExportImportPreflightState,
    migration_state: ExportImportMigrationState,
    schema_version_supported: bool,
    household_binding_match: bool,
    key_available: bool,
    integrity_ok: bool,
    duplicate_device_detected: bool,
    rejected_sections: Vec<ExportImportSectionDecision>,
}

const EXPORT_IMPORT_BUNDLE_DATA_CLASSES: &[ExportImportDataClass] = &[
    ExportImportDataClass::EvidenceJournal,
    ExportImportDataClass::Reports,
    ExportImportDataClass::Screenshots,
    ExportImportDataClass::Notifications,
];

const EXPORT_IMPORT_RETENTION_NOTES: &[&str] = &[
    EXPORT_IMPORT_RETENTION_NOTE_TOMBSTONE_ORDERING,
    EXPORT_IMPORT_RETENTION_NOTE_EXPIRED_PREVIEW,
];

const EXPORT_IMPORT_INCLUDED_DATA_CLASSES: &[ExportImportDataClass] = &[
    ExportImportDataClass::EvidenceJournal,
    ExportImportDataClass::Reports,
];

const EXPORT_IMPORT_EXCLUDED_DATA_CLASSES: &[ExportImportDataClass] = &[
    ExportImportDataClass::Screenshots,
    ExportImportDataClass::Notifications,
];

const EXPORT_IMPORT_SECTION_SEEDS: &[SectionSeed] = &[
    SectionSeed {
        data_class: ExportImportDataClass::EvidenceJournal,
        payload_ref: EXPORT_IMPORT_PAYLOAD_EVIDENCE_PROOF_1,
        payload_integrity_ref: EXPORT_IMPORT_PAYLOAD_SHA256_EVIDENCE_PROOF_1,
        retention_state: ExportImportSectionRetentionState::Active,
        included_in_human_summary: true,
        notes: EXPORT_IMPORT_SECTION_NOTE_EVIDENCE_JOURNAL,
    },
    SectionSeed {
        data_class: ExportImportDataClass::Reports,
        payload_ref: EXPORT_IMPORT_PAYLOAD_REPORT_PROOF_1,
        payload_integrity_ref: EXPORT_IMPORT_PAYLOAD_SHA256_REPORT_PROOF_1,
        retention_state: ExportImportSectionRetentionState::Active,
        included_in_human_summary: true,
        notes: EXPORT_IMPORT_SECTION_NOTE_REPORTS,
    },
    SectionSeed {
        data_class: ExportImportDataClass::Screenshots,
        payload_ref: EXPORT_IMPORT_PAYLOAD_SCREENSHOT_PROOF_1,
        payload_integrity_ref: EXPORT_IMPORT_PAYLOAD_SHA256_SCREENSHOT_PROOF_1,
        retention_state: ExportImportSectionRetentionState::Expired,
        included_in_human_summary: false,
        notes: EXPORT_IMPORT_SECTION_NOTE_SCREENSHOTS,
    },
    SectionSeed {
        data_class: ExportImportDataClass::Notifications,
        payload_ref: EXPORT_IMPORT_PAYLOAD_NOTIFICATION_PROOF_1,
        payload_integrity_ref: EXPORT_IMPORT_PAYLOAD_SHA256_NOTIFICATION_PROOF_1,
        retention_state: ExportImportSectionRetentionState::Tombstoned,
        included_in_human_summary: false,
        notes: EXPORT_IMPORT_SECTION_NOTE_NOTIFICATIONS,
    },
];

pub fn required_export_import_negative_preflight_states() -> Vec<ExportImportPreflightState> {
    vec![
        ExportImportPreflightState::RejectedSchemaVersion,
        ExportImportPreflightState::RejectedMigrationUnsupported,
        ExportImportPreflightState::RejectedWrongHousehold,
        ExportImportPreflightState::RejectedWrongKey,
        ExportImportPreflightState::RejectedCorruptBundle,
        ExportImportPreflightState::RejectedExpiredRetention,
        ExportImportPreflightState::RejectedDuplicateDevice,
    ]
}

pub fn required_export_import_non_claims() -> Vec<ExportImportNonClaim> {
    vec![
        ExportImportNonClaim::NoProviderRuntime,
        ExportImportNonClaim::NoAutoApply,
        ExportImportNonClaim::NoDefaultSupportDecrypt,
        ExportImportNonClaim::NoTsBusinessOwner,
        ExportImportNonClaim::NoLanOwnership,
    ]
}

pub fn sample_export_import_backup_recovery_contract_proof(
) -> ExportImportBackupRecoveryContractProof {
    sample::sample_export_import_backup_recovery_contract_proof()
}
