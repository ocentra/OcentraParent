use serde::{Deserialize, Serialize};

use super::*;

export_import_string_enums!(
    ExportImportBundleType, "camelCase" {
        Export,
        Backup,
        ImportPreview,
        Restore,
        Support,
    },
    ExportImportDataClass, "kebab-case" {
        ConfigMetadata,
        AccountMetadata,
        PolicyHistory,
        DeviceRegistry,
        EvidenceJournal,
        Logs,
        Screenshots,
        NetworkArtifacts,
        AiOutputs,
        Reports,
        Notifications,
        BillingReferences,
    },
    ExportImportEncryptionMode, "kebab-case" {
        PerClassEnvelopeEncrypted,
    },
    ExportImportIntegrityMode, "kebab-case" {
        ManifestAndPayloadHashes,
    },
    ExportImportProofTier, "kebab-case" {
        ContractOnly,
        RuntimeValidated,
        ManualRequired,
    },
    ExportImportSectionRetentionState, "kebab-case" {
        Active,
        Expired,
        Tombstoned,
    },
    ExportImportMigrationState, "kebab-case" {
        NotRequired,
        RequiredSupported,
        RequiredUnsupported,
    },
    ExportImportBackupCadence, "kebab-case" {
        Manual,
        Scheduled,
    },
    ExportImportBackupState, "camelCase" {
        Authorized,
        ManualRequired,
    },
    ExportImportMigrationExecutionState, "camelCase" {
        NotRequired,
        Applied,
        RolledBack,
        ManualRequired,
        RollbackManualRequired,
    },
    ExportImportPreflightState, "camelCase" {
        AcceptedPreview,
        PartialPreview,
        #[serde(rename = "rejectedSchemaVersion")]
        SchemaVersionInvalid,
        #[serde(rename = "rejectedMigrationUnsupported")]
        MigrationUnsupported,
        #[serde(rename = "rejectedWrongHousehold")]
        HouseholdMismatch,
        #[serde(rename = "rejectedWrongKey")]
        KeyUnavailable,
        #[serde(rename = "rejectedCorruptBundle")]
        BundleCorrupt,
        #[serde(rename = "rejectedExpiredRetention")]
        RetentionExpired,
        #[serde(rename = "rejectedDuplicateDevice")]
        DeviceDuplicate,
        #[serde(rename = "rejectedTombstoneConflict")]
        TombstoneConflict,
    },
    ExportImportSectionDecisionState, "camelCase" {
        Accepted,
        #[serde(rename = "rejectedExpiredRetention")]
        RetentionExpired,
        #[serde(rename = "rejectedTombstonePreserved")]
        TombstonePreserved,
        #[serde(rename = "rejectedDuplicateDevice")]
        DuplicateDevice,
    },
    ExportImportRestoreApplyState, "camelCase" {
        NotApplied,
        ApplyPending,
        Applied,
        Partial,
        WrongHousehold,
        WrongKey,
        Corrupt,
        Blocked,
    },
    ExportImportNonClaim, "kebab-case" {
        #[serde(rename = "no-provider-runtime")]
        ProviderRuntime,
        #[serde(rename = "no-auto-apply")]
        AutoApply,
        #[serde(rename = "no-default-support-decrypt")]
        DefaultSupportDecrypt,
        #[serde(rename = "no-ts-business-owner")]
        TsBusinessOwner,
        #[serde(rename = "no-lan-ownership")]
        LanOwnership,
    },
);

export_import_string_enum_as_str_values!(
    ExportImportBundleType {
        variants: [Export, Backup, ImportPreview, Restore, Support],
        values: [
            EXPORT_IMPORT_BUNDLE_TYPE_EXPORT,
            EXPORT_IMPORT_BUNDLE_TYPE_BACKUP,
            EXPORT_IMPORT_BUNDLE_TYPE_IMPORT_PREVIEW,
            EXPORT_IMPORT_BUNDLE_TYPE_RESTORE,
            EXPORT_IMPORT_BUNDLE_TYPE_SUPPORT,
        ],
    },
    ExportImportDataClass {
        variants: [
            ConfigMetadata,
            AccountMetadata,
            PolicyHistory,
            DeviceRegistry,
            EvidenceJournal,
            Logs,
            Screenshots,
            NetworkArtifacts,
            AiOutputs,
            Reports,
            Notifications,
            BillingReferences,
        ],
        values: [
            EXPORT_IMPORT_DATA_CLASS_CONFIG_METADATA,
            EXPORT_IMPORT_DATA_CLASS_ACCOUNT_METADATA,
            EXPORT_IMPORT_DATA_CLASS_POLICY_HISTORY,
            EXPORT_IMPORT_DATA_CLASS_DEVICE_REGISTRY,
            EXPORT_IMPORT_DATA_CLASS_EVIDENCE_JOURNAL,
            EXPORT_IMPORT_DATA_CLASS_LOGS,
            EXPORT_IMPORT_DATA_CLASS_SCREENSHOTS,
            EXPORT_IMPORT_DATA_CLASS_NETWORK_ARTIFACTS,
            EXPORT_IMPORT_DATA_CLASS_AI_OUTPUTS,
            EXPORT_IMPORT_DATA_CLASS_REPORTS,
            EXPORT_IMPORT_DATA_CLASS_NOTIFICATIONS,
            EXPORT_IMPORT_DATA_CLASS_BILLING_REFERENCES,
        ],
    },
    ExportImportEncryptionMode {
        variants: [PerClassEnvelopeEncrypted],
        values: [EXPORT_IMPORT_ENCRYPTION_MODE_PER_CLASS_ENVELOPE_ENCRYPTED],
    },
    ExportImportIntegrityMode {
        variants: [ManifestAndPayloadHashes],
        values: [EXPORT_IMPORT_INTEGRITY_MODE_MANIFEST_AND_PAYLOAD_HASHES],
    },
    ExportImportProofTier {
        variants: [ContractOnly, RuntimeValidated, ManualRequired],
        values: [
            EXPORT_IMPORT_PROOF_TIER_CONTRACT_ONLY,
            EXPORT_IMPORT_PROOF_TIER_RUNTIME_VALIDATED,
            EXPORT_IMPORT_PROOF_TIER_MANUAL_REQUIRED,
        ],
    },
    ExportImportSectionRetentionState {
        variants: [Active, Expired, Tombstoned],
        values: [
            EXPORT_IMPORT_SECTION_RETENTION_STATE_ACTIVE,
            EXPORT_IMPORT_SECTION_RETENTION_STATE_EXPIRED,
            EXPORT_IMPORT_SECTION_RETENTION_STATE_TOMBSTONED,
        ],
    },
    ExportImportMigrationState {
        variants: [NotRequired, RequiredSupported, RequiredUnsupported],
        values: [
            EXPORT_IMPORT_MIGRATION_STATE_NOT_REQUIRED,
            EXPORT_IMPORT_MIGRATION_STATE_REQUIRED_SUPPORTED,
            EXPORT_IMPORT_MIGRATION_STATE_REQUIRED_UNSUPPORTED,
        ],
    },
    ExportImportBackupCadence {
        variants: [Manual, Scheduled],
        values: [
            EXPORT_IMPORT_BACKUP_CADENCE_MANUAL,
            EXPORT_IMPORT_BACKUP_CADENCE_SCHEDULED
        ],
    },
    ExportImportBackupState {
        variants: [Authorized, ManualRequired],
        values: [
            EXPORT_IMPORT_BACKUP_STATE_AUTHORIZED,
            EXPORT_IMPORT_BACKUP_STATE_MANUAL_REQUIRED,
        ],
    },
    ExportImportMigrationExecutionState {
        variants: [
            NotRequired,
            Applied,
            RolledBack,
            ManualRequired,
            RollbackManualRequired,
        ],
        values: [
            EXPORT_IMPORT_MIGRATION_EXECUTION_STATE_NOT_REQUIRED,
            EXPORT_IMPORT_MIGRATION_EXECUTION_STATE_APPLIED,
            EXPORT_IMPORT_MIGRATION_EXECUTION_STATE_ROLLED_BACK,
            EXPORT_IMPORT_MIGRATION_EXECUTION_STATE_MANUAL_REQUIRED,
            EXPORT_IMPORT_MIGRATION_EXECUTION_STATE_ROLLBACK_MANUAL_REQUIRED,
        ],
    },
    ExportImportPreflightState {
        variants: [
            AcceptedPreview,
            PartialPreview,
            SchemaVersionInvalid,
            MigrationUnsupported,
            HouseholdMismatch,
            KeyUnavailable,
            BundleCorrupt,
            RetentionExpired,
            DeviceDuplicate,
            TombstoneConflict,
        ],
        values: [
            EXPORT_IMPORT_PREFLIGHT_STATE_ACCEPTED_PREVIEW,
            EXPORT_IMPORT_PREFLIGHT_STATE_PARTIAL_PREVIEW,
            EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_SCHEMA_VERSION,
            EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_MIGRATION_UNSUPPORTED,
            EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_WRONG_HOUSEHOLD,
            EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_WRONG_KEY,
            EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_CORRUPT_BUNDLE,
            EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_EXPIRED_RETENTION,
            EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_DUPLICATE_DEVICE,
            EXPORT_IMPORT_PREFLIGHT_STATE_REJECTED_TOMBSTONE_CONFLICT,
        ],
    },
    ExportImportSectionDecisionState {
        variants: [
            Accepted,
            RetentionExpired,
            TombstonePreserved,
            DuplicateDevice,
        ],
        values: [
            EXPORT_IMPORT_SECTION_DECISION_STATE_ACCEPTED,
            EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_EXPIRED_RETENTION,
            EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_TOMBSTONE_PRESERVED,
            EXPORT_IMPORT_SECTION_DECISION_STATE_REJECTED_DUPLICATE_DEVICE,
        ],
    },
    ExportImportRestoreApplyState {
        variants: [
            NotApplied,
            ApplyPending,
            Applied,
            Partial,
            WrongHousehold,
            WrongKey,
            Corrupt,
            Blocked,
        ],
        values: [
            EXPORT_IMPORT_RESTORE_APPLY_STATE_NOT_APPLIED,
            EXPORT_IMPORT_RESTORE_APPLY_STATE_APPLY_PENDING,
            EXPORT_IMPORT_RESTORE_APPLY_STATE_APPLIED,
            EXPORT_IMPORT_RESTORE_APPLY_STATE_PARTIAL,
            EXPORT_IMPORT_RESTORE_APPLY_STATE_WRONG_HOUSEHOLD,
            EXPORT_IMPORT_RESTORE_APPLY_STATE_WRONG_KEY,
            EXPORT_IMPORT_RESTORE_APPLY_STATE_CORRUPT,
            EXPORT_IMPORT_RESTORE_APPLY_STATE_BLOCKED,
        ],
    },
    ExportImportNonClaim {
        variants: [
            ProviderRuntime,
            AutoApply,
            DefaultSupportDecrypt,
            TsBusinessOwner,
            LanOwnership,
        ],
        values: [
            EXPORT_IMPORT_NON_CLAIM_NO_PROVIDER_RUNTIME,
            EXPORT_IMPORT_NON_CLAIM_NO_AUTO_APPLY,
            EXPORT_IMPORT_NON_CLAIM_NO_DEFAULT_SUPPORT_DECRYPT,
            EXPORT_IMPORT_NON_CLAIM_NO_TS_BUSINESS_OWNER,
            EXPORT_IMPORT_NON_CLAIM_NO_LAN_OWNERSHIP,
        ],
    },
);
