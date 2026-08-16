/* generated from crates/schema/src/data_custody_source_of_truth.rs */

export const DataCustodySourceOfTruthContractRuntime = {
  SchemaVersion: 'data-custody-source-of-truth-proof',
} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export type GeneratedDataCustodySourceOfTruthKind = 'self' | 'derived-from-data-classes';
export type GeneratedDataCustodyAuthority =
  | 'ocentra-account-control-plane'
  | 'payment-control-plane'
  | 'billing-provider'
  | 'household-control-plane'
  | 'ocentra-routing-service'
  | 'ocentra-report-status-runtime'
  | 'support-system'
  | 'public-release-pipeline'
  | 'child-device'
  | 'parent-device'
  | 'parent-owned-storage'
  | 'support-export-boundary';
export type GeneratedDataCustodyDefaultLocation =
  | 'ocentra-account-metadata-store'
  | 'ocentra-billing-metadata-store'
  | 'billing-provider-customer-record'
  | 'ocentra-license-update-store'
  | 'household-device-registry'
  | 'ocentra-household-route-store'
  | 'household-setup-draft-store'
  | 'ocentra-notification-route-store'
  | 'ocentra-short-lived-report-status-store'
  | 'ocentra-support-case-store'
  | 'public-release-surface'
  | 'household-profile-store'
  | 'household-rule-store'
  | 'household-audit-store'
  | 'child-device-encrypted-journal'
  | 'child-device-local-query-store'
  | 'child-device-sensitive-evidence-store'
  | 'child-device-location-store'
  | 'child-device-local-ai-store'
  | 'parent-device-report-cache'
  | 'parent-device-notification-history-cache'
  | 'parent-assistant-ephemeral-context'
  | 'parent-owned-encrypted-storage'
  | 'provider-envelope-metadata'
  | 'support-export-artifact'
  | 'household-key-store';
export type GeneratedDataCustodyExposure =
  | 'none'
  | 'allowed-references-only'
  | 'redacted-metadata-only'
  | 'minimal'
  | 'derived-output-only'
  | 'public';
export type GeneratedDataCustodyOcentraHostingMode =
  | 'forbidden'
  | 'allowed-metadata-only'
  | 'short-lived-status-only'
  | 'public-release-only';
export type GeneratedDataCustodyNonClaim =
  | 'no-default-ocentra-child-activity-store'
  | 'no-sqlite-truth-layer'
  | 'no-provider-auto-apply'
  | 'no-support-decrypt-default'
  | 'no-ocentra-owned-parent-rules'
  | 'no-raw-child-evidence-in-notifications'
  | 'no-long-lived-hosted-reports';
export type GeneratedDataCustodyClassId =
  | 'account-identity-metadata'
  | 'subscription-entitlement-metadata'
  | 'billing-provider-identity-reference'
  | 'license-download-update-metadata'
  | 'household-device-registry'
  | 'device-registration-pairing-route-metadata'
  | 'setup-state-and-pairing-draft'
  | 'minimal-notification-routing-metadata'
  | 'short-lived-report-compiler-status'
  | 'support-case-metadata'
  | 'public-website-release-status'
  | 'child-profile'
  | 'parent-rules-and-approval-history'
  | 'audit-log'
  | 'evidence-journal-segments'
  | 'sqlite-evidence-read-model-database'
  | 'screenshots-and-screen-analysis-images'
  | 'browser-url-history'
  | 'network-app-game-evidence'
  | 'location-tracking-evidence'
  | 'local-ai-and-policy-decisions'
  | 'generated-long-term-reports'
  | 'parent-notification-history-cache'
  | 'assistant-child-evidence-context'
  | 'parent-owned-storage-contents'
  | 'provider-sync-payloads'
  | 'support-bundles-containing-raw-child-activity'
  | 'universal-decrypt-keys';

export const GeneratedDataCustodySourceOfTruthKinds = [
  'self',
  'derived-from-data-classes',
] as const satisfies readonly GeneratedDataCustodySourceOfTruthKind[];
export const GeneratedDataCustodyAuthorities = [
  'ocentra-account-control-plane',
  'payment-control-plane',
  'billing-provider',
  'household-control-plane',
  'ocentra-routing-service',
  'ocentra-report-status-runtime',
  'support-system',
  'public-release-pipeline',
  'child-device',
  'parent-device',
  'parent-owned-storage',
  'support-export-boundary',
] as const satisfies readonly GeneratedDataCustodyAuthority[];
export const GeneratedDataCustodyDefaultLocations = [
  'ocentra-account-metadata-store',
  'ocentra-billing-metadata-store',
  'billing-provider-customer-record',
  'ocentra-license-update-store',
  'household-device-registry',
  'ocentra-household-route-store',
  'household-setup-draft-store',
  'ocentra-notification-route-store',
  'ocentra-short-lived-report-status-store',
  'ocentra-support-case-store',
  'public-release-surface',
  'household-profile-store',
  'household-rule-store',
  'household-audit-store',
  'child-device-encrypted-journal',
  'child-device-local-query-store',
  'child-device-sensitive-evidence-store',
  'child-device-location-store',
  'child-device-local-ai-store',
  'parent-device-report-cache',
  'parent-device-notification-history-cache',
  'parent-assistant-ephemeral-context',
  'parent-owned-encrypted-storage',
  'provider-envelope-metadata',
  'support-export-artifact',
  'household-key-store',
] as const satisfies readonly GeneratedDataCustodyDefaultLocation[];
export const GeneratedDataCustodyExposures = [
  'none',
  'allowed-references-only',
  'redacted-metadata-only',
  'minimal',
  'derived-output-only',
  'public',
] as const satisfies readonly GeneratedDataCustodyExposure[];
export const GeneratedDataCustodyOcentraHostingModes = [
  'forbidden',
  'allowed-metadata-only',
  'short-lived-status-only',
  'public-release-only',
] as const satisfies readonly GeneratedDataCustodyOcentraHostingMode[];
export const GeneratedDataCustodyNonClaims = [
  'no-default-ocentra-child-activity-store',
  'no-sqlite-truth-layer',
  'no-provider-auto-apply',
  'no-support-decrypt-default',
  'no-ocentra-owned-parent-rules',
  'no-raw-child-evidence-in-notifications',
  'no-long-lived-hosted-reports',
] as const satisfies readonly GeneratedDataCustodyNonClaim[];
export const GeneratedDataCustodyClassIds = [
  'account-identity-metadata',
  'subscription-entitlement-metadata',
  'billing-provider-identity-reference',
  'license-download-update-metadata',
  'household-device-registry',
  'device-registration-pairing-route-metadata',
  'setup-state-and-pairing-draft',
  'minimal-notification-routing-metadata',
  'short-lived-report-compiler-status',
  'support-case-metadata',
  'public-website-release-status',
  'child-profile',
  'parent-rules-and-approval-history',
  'audit-log',
  'evidence-journal-segments',
  'sqlite-evidence-read-model-database',
  'screenshots-and-screen-analysis-images',
  'browser-url-history',
  'network-app-game-evidence',
  'location-tracking-evidence',
  'local-ai-and-policy-decisions',
  'generated-long-term-reports',
  'parent-notification-history-cache',
  'assistant-child-evidence-context',
  'parent-owned-storage-contents',
  'provider-sync-payloads',
  'support-bundles-containing-raw-child-activity',
  'universal-decrypt-keys',
] as const satisfies readonly GeneratedDataCustodyClassId[];

export interface GeneratedDataCustodySourceOfTruth {
  kind: GeneratedDataCustodySourceOfTruthKind;
  sourceClassIds: readonly GeneratedDataCustodyClassId[];
}

export interface GeneratedDataCustodyHostingPolicy {
  ocentraHostingMode: GeneratedDataCustodyOcentraHostingMode;
  parentOwnedStorageAllowed: boolean;
  providerMetadataAllowed: boolean;
  supportExportParentInitiatedOnly: boolean;
}

export interface GeneratedDataCustodySourceOfTruthRow {
  rowId: string;
  classId: GeneratedDataCustodyClassId;
  classLabel: string;
  sourceOwner: string;
  sourceOfTruth: GeneratedDataCustodySourceOfTruth;
  custodyAuthority: GeneratedDataCustodyAuthority;
  defaultLocation: GeneratedDataCustodyDefaultLocation;
  ocentraHostedByDefault: boolean;
  mustNeverBeHostedByDefault: boolean;
  encryptedBeforeUpload: boolean;
  mayAppearInReports: boolean;
  mayAppearInNotifications: boolean;
  reportExposure: GeneratedDataCustodyExposure;
  notificationExposure: GeneratedDataCustodyExposure;
  rawChildEvidenceAllowed: boolean;
  derivedUseOnly: boolean;
  sensitive: boolean;
  hostingPolicy: GeneratedDataCustodyHostingPolicy;
  notes: string;
}

export interface GeneratedDataCustodySourceOfTruthContractProof {
  schemaVersion: typeof DataCustodySourceOfTruthContractRuntime.SchemaVersion;
  contractVersion: GeneratedParentContractSchemaVersion;
  matrixId: string;
  rows: readonly GeneratedDataCustodySourceOfTruthRow[];
  allowedOcentraHostedMetadata: readonly GeneratedDataCustodyClassId[];
  mustNeverBeHostedByDefault: readonly GeneratedDataCustodyClassId[];
  claimSafeLanguage: readonly string[];
  nonClaims: readonly GeneratedDataCustodyNonClaim[];
  accountControlPlaneSeparated: boolean;
  providerOwnedBillingIdentitySeparated: boolean;
  ocentraIsDefaultChildDataStore: boolean;
  providerAutoApplyClaimed: boolean;
  supportDecryptByDefaultClaimed: boolean;
  sqliteAsTruthLayerClaimed: boolean;
  rawChildActivityHostedByDefaultClaimed: boolean;
  updatedAt: string;
}

export const GeneratedDataCustodyKnownGaps = [
  'Support decrypt-by-default remains false until product and key-custody decisions exist.',
  'Provider mode defaults stay explicit and are not implied by the custody matrix.',
  'Mobile restore and key-custody proof stays manual-required outside this workpack.',
  'Delete ergonomics and tombstone propagation are owned by later data-custody workpacks.',
  'The matrix records source truth and no-hosting boundaries; it does not claim transfer runtime.',
] as const;

export const GeneratedDataCustodySourceOfTruthContractProof = {
  schemaVersion: 'data-custody-source-of-truth-proof',
  contractVersion: 'v0.6',
  matrixId: 'data-custody-source-of-truth-wp01',
  rows: [
    {
      rowId: 'custody-row-account-identity-metadata',
      classId: 'account-identity-metadata',
      classLabel: 'Account identity metadata',
      sourceOwner: 'Control plane / account plan',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'ocentra-account-control-plane',
      defaultLocation: 'ocentra-account-metadata-store',
      ocentraHostedByDefault: true,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'redacted-metadata-only',
      notificationExposure: 'redacted-metadata-only',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'allowed-metadata-only',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Identity, household, and entitlement metadata only.',
    },
    {
      rowId: 'custody-row-subscription-entitlement-metadata',
      classId: 'subscription-entitlement-metadata',
      classLabel: 'Subscription, billing, and entitlement metadata',
      sourceOwner: 'Billing / payment plan',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'payment-control-plane',
      defaultLocation: 'ocentra-billing-metadata-store',
      ocentraHostedByDefault: true,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: false,
      reportExposure: 'redacted-metadata-only',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'allowed-metadata-only',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: true,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Billing and entitlement state may be mirrored, but child evidence stays separate.',
    },
    {
      rowId: 'custody-row-billing-provider-identity-reference',
      classId: 'billing-provider-identity-reference',
      classLabel: 'Billing provider identity reference',
      sourceOwner: 'Payment provider',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'billing-provider',
      defaultLocation: 'billing-provider-customer-record',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: false,
      reportExposure: 'redacted-metadata-only',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: true,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Provider-owned billing identity remains outside the parent evidence truth layer.',
    },
    {
      rowId: 'custody-row-license-download-update-metadata',
      classId: 'license-download-update-metadata',
      classLabel: 'License, download, and update metadata',
      sourceOwner: 'Release process',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'public-release-pipeline',
      defaultLocation: 'ocentra-license-update-store',
      ocentraHostedByDefault: true,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: false,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'public',
      notificationExposure: 'minimal',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: false,
      hostingPolicy: {
        ocentraHostingMode: 'allowed-metadata-only',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Public product and installer metadata only.',
    },
    {
      rowId: 'custody-row-household-device-registry',
      classId: 'household-device-registry',
      classLabel: 'Household device registry',
      sourceOwner: 'Household control plane',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'household-control-plane',
      defaultLocation: 'household-device-registry',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: false,
      reportExposure: 'allowed-references-only',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Household-owned device list and role binding state.',
    },
    {
      rowId: 'custody-row-device-registration-pairing-route-metadata',
      classId: 'device-registration-pairing-route-metadata',
      classLabel: 'Device registration and pairing route metadata',
      sourceOwner: 'Household control plane',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'ocentra-routing-service',
      defaultLocation: 'ocentra-household-route-store',
      ocentraHostedByDefault: true,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'minimal',
      notificationExposure: 'minimal',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'allowed-metadata-only',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Route and pairing metadata only; no child activity payloads.',
    },
    {
      rowId: 'custody-row-setup-state-and-pairing-draft',
      classId: 'setup-state-and-pairing-draft',
      classLabel: 'Setup state and pairing draft',
      sourceOwner: 'Household setup flow',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'household-control-plane',
      defaultLocation: 'household-setup-draft-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: false,
      reportExposure: 'allowed-references-only',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Setup drafts stay household-owned until another workpack proves remote storage.',
    },
    {
      rowId: 'custody-row-minimal-notification-routing-metadata',
      classId: 'minimal-notification-routing-metadata',
      classLabel: 'Minimal notification routing metadata',
      sourceOwner: 'Notification service',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'ocentra-routing-service',
      defaultLocation: 'ocentra-notification-route-store',
      ocentraHostedByDefault: true,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'minimal',
      notificationExposure: 'minimal',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'allowed-metadata-only',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: true,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Payload detail must stay redacted and drill-in must remain authenticated.',
    },
    {
      rowId: 'custody-row-short-lived-report-compiler-status',
      classId: 'short-lived-report-compiler-status',
      classLabel: 'Short-lived report compiler status',
      sourceOwner: 'Report runtime / control plane',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'ocentra-report-status-runtime',
      defaultLocation: 'ocentra-short-lived-report-status-store',
      ocentraHostedByDefault: true,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: false,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'minimal',
      notificationExposure: 'minimal',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'short-lived-status-only',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Status only, not report content or source child evidence.',
    },
    {
      rowId: 'custody-row-support-case-metadata',
      classId: 'support-case-metadata',
      classLabel: 'Support case metadata',
      sourceOwner: 'Support system',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'support-system',
      defaultLocation: 'ocentra-support-case-store',
      ocentraHostedByDefault: true,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'redacted-metadata-only',
      notificationExposure: 'redacted-metadata-only',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'allowed-metadata-only',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: true,
      },
      notes: 'Support-safe metadata only; no raw child activity by default.',
    },
    {
      rowId: 'custody-row-public-website-release-status',
      classId: 'public-website-release-status',
      classLabel: 'Public website and release status',
      sourceOwner: 'Public site / release process',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'public-release-pipeline',
      defaultLocation: 'public-release-surface',
      ocentraHostedByDefault: true,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: false,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'public',
      notificationExposure: 'public',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: false,
      hostingPolicy: {
        ocentraHostingMode: 'public-release-only',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Public product metadata only.',
    },
    {
      rowId: 'custody-row-child-profile',
      classId: 'child-profile',
      classLabel: 'Child profile',
      sourceOwner: 'Child device / household model',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'household-control-plane',
      defaultLocation: 'household-profile-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'allowed-references-only',
      notificationExposure: 'allowed-references-only',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Household-owned, role-bound profile state.',
    },
    {
      rowId: 'custody-row-parent-rules-and-approval-history',
      classId: 'parent-rules-and-approval-history',
      classLabel: 'Parent rules and approval history',
      sourceOwner: 'Household control plane',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'household-control-plane',
      defaultLocation: 'household-rule-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'allowed-references-only',
      notificationExposure: 'allowed-references-only',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Parent rule and approval state must not become an Ocentra-owned truth layer.',
    },
    {
      rowId: 'custody-row-audit-log',
      classId: 'audit-log',
      classLabel: 'Audit log',
      sourceOwner: 'Household control plane',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'household-control-plane',
      defaultLocation: 'household-audit-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: false,
      reportExposure: 'allowed-references-only',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Audit history may prove action lineage but should not carry raw child payloads.',
    },
    {
      rowId: 'custody-row-evidence-journal-segments',
      classId: 'evidence-journal-segments',
      classLabel: 'Evidence journal segments',
      sourceOwner: 'Child device local journal',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'child-device',
      defaultLocation: 'child-device-encrypted-journal',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: false,
      reportExposure: 'allowed-references-only',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: true,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Encrypted append-only journal is the evidence source of truth.',
    },
    {
      rowId: 'custody-row-sqlite-evidence-read-model-database',
      classId: 'sqlite-evidence-read-model-database',
      classLabel: 'SQLite evidence/read-model database',
      sourceOwner: 'Child device local cache',
      sourceOfTruth: {
        kind: 'derived-from-data-classes',
        sourceClassIds: ['evidence-journal-segments'],
      },
      custodyAuthority: 'child-device',
      defaultLocation: 'child-device-local-query-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: true,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'SQLite is rebuildable local cache, not the truth layer.',
    },
    {
      rowId: 'custody-row-screenshots-and-screen-analysis-images',
      classId: 'screenshots-and-screen-analysis-images',
      classLabel: 'Screenshots and screen-analysis images',
      sourceOwner: 'Child device local evidence',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'child-device',
      defaultLocation: 'child-device-sensitive-evidence-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: true,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Sensitive visual evidence stays local unless explicitly exported.',
    },
    {
      rowId: 'custody-row-browser-url-history',
      classId: 'browser-url-history',
      classLabel: 'Browser URL history',
      sourceOwner: 'Child device local evidence',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'child-device',
      defaultLocation: 'child-device-sensitive-evidence-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: true,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'URL history never becomes generic telemetry.',
    },
    {
      rowId: 'custody-row-network-app-game-evidence',
      classId: 'network-app-game-evidence',
      classLabel: 'Network, app, and game evidence',
      sourceOwner: 'Child device local evidence',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'child-device',
      defaultLocation: 'child-device-sensitive-evidence-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: true,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Network, app, and game evidence stays under the same local-first custody rule.',
    },
    {
      rowId: 'custody-row-location-tracking-evidence',
      classId: 'location-tracking-evidence',
      classLabel: 'Location and tracking evidence',
      sourceOwner: 'Child device local evidence',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'child-device',
      defaultLocation: 'child-device-location-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: true,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Tracking evidence remains child-device local unless a later plan proves otherwise.',
    },
    {
      rowId: 'custody-row-local-ai-and-policy-decisions',
      classId: 'local-ai-and-policy-decisions',
      classLabel: 'Local AI and policy decisions',
      sourceOwner: 'Child device local runtime',
      sourceOfTruth: {
        kind: 'derived-from-data-classes',
        sourceClassIds: ['sqlite-evidence-read-model-database', 'parent-rules-and-approval-history'],
      },
      custodyAuthority: 'child-device',
      defaultLocation: 'child-device-local-ai-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'derived-output-only',
      notificationExposure: 'minimal',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: true,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Derived decisions may inform reports or alerts without replacing cited evidence.',
    },
    {
      rowId: 'custody-row-generated-long-term-reports',
      classId: 'generated-long-term-reports',
      classLabel: 'Generated long-term reports',
      sourceOwner: 'Parent-owned output',
      sourceOfTruth: {
        kind: 'derived-from-data-classes',
        sourceClassIds: ['sqlite-evidence-read-model-database', 'local-ai-and-policy-decisions', 'child-profile'],
      },
      custodyAuthority: 'parent-device',
      defaultLocation: 'parent-device-report-cache',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'derived-output-only',
      notificationExposure: 'minimal',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: true,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Reports are derived output, not an Ocentra-owned truth layer.',
    },
    {
      rowId: 'custody-row-parent-notification-history-cache',
      classId: 'parent-notification-history-cache',
      classLabel: 'Parent notification history cache',
      sourceOwner: 'Parent device notification history/cache',
      sourceOfTruth: {
        kind: 'derived-from-data-classes',
        sourceClassIds: ['minimal-notification-routing-metadata'],
      },
      custodyAuthority: 'parent-device',
      defaultLocation: 'parent-device-notification-history-cache',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: false,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: true,
      reportExposure: 'redacted-metadata-only',
      notificationExposure: 'minimal',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: true,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Parent device caches notification status without making cloud routing the family-data store.',
    },
    {
      rowId: 'custody-row-assistant-child-evidence-context',
      classId: 'assistant-child-evidence-context',
      classLabel: 'Assistant child-evidence context',
      sourceOwner: 'Parent assistant runtime',
      sourceOfTruth: {
        kind: 'derived-from-data-classes',
        sourceClassIds: ['generated-long-term-reports', 'audit-log'],
      },
      custodyAuthority: 'parent-device',
      defaultLocation: 'parent-assistant-ephemeral-context',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: true,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Assistant context stays citation-only and excludes raw child content.',
    },
    {
      rowId: 'custody-row-parent-owned-storage-contents',
      classId: 'parent-owned-storage-contents',
      classLabel: 'Parent-owned storage contents',
      sourceOwner: 'Parent-selected provider',
      sourceOfTruth: {
        kind: 'derived-from-data-classes',
        sourceClassIds: [
          'child-profile',
          'parent-rules-and-approval-history',
          'evidence-journal-segments',
          'generated-long-term-reports',
          'local-ai-and-policy-decisions',
        ],
      },
      custodyAuthority: 'parent-owned-storage',
      defaultLocation: 'parent-owned-encrypted-storage',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: true,
      mayAppearInNotifications: false,
      reportExposure: 'redacted-metadata-only',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: true,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Parent-owned storage is an encrypted destination, not an Ocentra default.',
    },
    {
      rowId: 'custody-row-provider-sync-payloads',
      classId: 'provider-sync-payloads',
      classLabel: 'Provider sync payloads',
      sourceOwner: 'Provider bundle',
      sourceOfTruth: {
        kind: 'derived-from-data-classes',
        sourceClassIds: ['parent-owned-storage-contents'],
      },
      custodyAuthority: 'parent-owned-storage',
      defaultLocation: 'provider-envelope-metadata',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: true,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: true,
        supportExportParentInitiatedOnly: false,
      },
      notes: 'Provider-visible envelope metadata is limited to unavoidable connector behavior.',
    },
    {
      rowId: 'custody-row-support-bundles-containing-raw-child-activity',
      classId: 'support-bundles-containing-raw-child-activity',
      classLabel: 'Support bundles containing raw child activity',
      sourceOwner: 'Support flow',
      sourceOfTruth: {
        kind: 'derived-from-data-classes',
        sourceClassIds: [
          'screenshots-and-screen-analysis-images',
          'browser-url-history',
          'network-app-game-evidence',
          'location-tracking-evidence',
          'generated-long-term-reports',
        ],
      },
      custodyAuthority: 'support-export-boundary',
      defaultLocation: 'support-export-artifact',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: true,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: true,
      derivedUseOnly: true,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: true,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: true,
      },
      notes: 'Support raw-activity export requires explicit parent initiation and redaction review.',
    },
    {
      rowId: 'custody-row-universal-decrypt-keys',
      classId: 'universal-decrypt-keys',
      classLabel: 'Universal decrypt keys',
      sourceOwner: 'Household key model',
      sourceOfTruth: {
        kind: 'self',
        sourceClassIds: [],
      },
      custodyAuthority: 'household-control-plane',
      defaultLocation: 'household-key-store',
      ocentraHostedByDefault: false,
      mustNeverBeHostedByDefault: true,
      encryptedBeforeUpload: false,
      mayAppearInReports: false,
      mayAppearInNotifications: false,
      reportExposure: 'none',
      notificationExposure: 'none',
      rawChildEvidenceAllowed: false,
      derivedUseOnly: false,
      sensitive: true,
      hostingPolicy: {
        ocentraHostingMode: 'forbidden',
        parentOwnedStorageAllowed: false,
        providerMetadataAllowed: false,
        supportExportParentInitiatedOnly: false,
      },
      notes: "Universal decrypt keys never host by default and remain outside this workpack's runtime claims.",
    },
  ],
  allowedOcentraHostedMetadata: [
    'account-identity-metadata',
    'subscription-entitlement-metadata',
    'license-download-update-metadata',
    'device-registration-pairing-route-metadata',
    'minimal-notification-routing-metadata',
    'short-lived-report-compiler-status',
    'support-case-metadata',
    'public-website-release-status',
  ],
  mustNeverBeHostedByDefault: [
    'parent-rules-and-approval-history',
    'evidence-journal-segments',
    'sqlite-evidence-read-model-database',
    'screenshots-and-screen-analysis-images',
    'browser-url-history',
    'network-app-game-evidence',
    'location-tracking-evidence',
    'generated-long-term-reports',
    'assistant-child-evidence-context',
    'parent-owned-storage-contents',
    'provider-sync-payloads',
    'support-bundles-containing-raw-child-activity',
    'universal-decrypt-keys',
  ],
  claimSafeLanguage: [
    'Ocentra-hosted infrastructure is not the default child-data store.',
    'SQLite/read-model databases are rebuildable caches, not the evidence truth layer.',
    'Reports, notifications, and assistant context may reference allowed source data only.',
    'Provider payloads and support bundles require encryption and parent-initiated export before leaving the household boundary.',
    'Billing provider identity remains provider-owned even when entitlement metadata is mirrored in the control plane.',
  ],
  nonClaims: [
    'no-default-ocentra-child-activity-store',
    'no-sqlite-truth-layer',
    'no-provider-auto-apply',
    'no-support-decrypt-default',
    'no-ocentra-owned-parent-rules',
    'no-raw-child-evidence-in-notifications',
    'no-long-lived-hosted-reports',
  ],
  accountControlPlaneSeparated: true,
  providerOwnedBillingIdentitySeparated: true,
  ocentraIsDefaultChildDataStore: false,
  providerAutoApplyClaimed: false,
  supportDecryptByDefaultClaimed: false,
  sqliteAsTruthLayerClaimed: false,
  rawChildActivityHostedByDefaultClaimed: false,
  updatedAt: '2026-06-28T18:44:00.000Z',
} as const satisfies GeneratedDataCustodySourceOfTruthContractProof;
