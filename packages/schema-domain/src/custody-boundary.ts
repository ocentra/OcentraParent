/* thin custody boundary helpers over Rust-generated data custody source-of-truth literals plus local workpack adapters */

import { FamilyReferenceSchema, ParentEvidenceReferenceSchema } from './family-references';
import { ParentContractSchemaVersionSchema } from './family-reference-primitives';
import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  GeneratedDataCustodyAuthorities,
  GeneratedDataCustodyClassIds,
  GeneratedDataCustodyDefaultLocations,
  GeneratedDataCustodyExposures,
  GeneratedDataCustodyOcentraHostingModes,
  GeneratedDataCustodySourceOfTruthKinds,
  type GeneratedDataCustodyAuthority,
  type GeneratedDataCustodyClassId,
  type GeneratedDataCustodyDefaultLocation,
  type GeneratedDataCustodyExposure,
  type GeneratedDataCustodyOcentraHostingMode,
} from './generated-data-custody-source-of-truth-contracts';

export const DataCustodyRecordIdSchema = brandedNonEmptyStringSchema('DataCustodyRecordId');
export const DataCustodyStoreRefSchema = brandedNonEmptyStringSchema('DataCustodyStoreRef');
export const DataCustodyRetentionPolicyIdSchema = brandedNonEmptyStringSchema('DataCustodyRetentionPolicyId');

export const DataCustodyStateLiteral = {
  LocalOnly: 'local-only',
  FamilyShared: 'family-shared',
  ExportReady: 'export-ready',
} as const;

export const DataCustodyRawPayloadStateLiteral = {
  Excluded: 'excluded',
  RedactedOnly: 'redacted-only',
} as const;

export const DataCustodyRetentionDispositionLiteral = {
  Retain: 'retain',
  DeleteEligible: 'delete-eligible',
  DeleteRequested: 'delete-requested',
} as const;

export const DataCustodyStateSchema = withParser(
  Schema.Literal(
    DataCustodyStateLiteral.LocalOnly,
    DataCustodyStateLiteral.FamilyShared,
    DataCustodyStateLiteral.ExportReady
  )
);

export const DataCustodyRawPayloadStateSchema = withParser(
  Schema.Literal(DataCustodyRawPayloadStateLiteral.Excluded, DataCustodyRawPayloadStateLiteral.RedactedOnly)
);

export const DataCustodyRetentionDispositionSchema = withParser(
  Schema.Literal(
    DataCustodyRetentionDispositionLiteral.Retain,
    DataCustodyRetentionDispositionLiteral.DeleteEligible,
    DataCustodyRetentionDispositionLiteral.DeleteRequested
  )
);

export const DataCustodyBoundarySchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    family: FamilyReferenceSchema,
    evidence: ParentEvidenceReferenceSchema,
    recordId: DataCustodyRecordIdSchema,
    storeRef: DataCustodyStoreRefSchema,
    custodyState: DataCustodyStateSchema,
    rawPayloadState: DataCustodyRawPayloadStateSchema,
    retentionPolicyId: DataCustodyRetentionPolicyIdSchema,
    retentionDisposition: DataCustodyRetentionDispositionSchema,
  })
);

export type DataCustodyState = Infer<typeof DataCustodyStateSchema>;
export type DataCustodyRawPayloadState = Infer<typeof DataCustodyRawPayloadStateSchema>;
export type DataCustodyRetentionDisposition = Infer<typeof DataCustodyRetentionDispositionSchema>;
export type DataCustodyBoundary = Infer<typeof DataCustodyBoundarySchema>;

export const DataCustodyState = {
  LocalOnly: DataCustodyStateSchema.parse(DataCustodyStateLiteral.LocalOnly),
  FamilyShared: DataCustodyStateSchema.parse(DataCustodyStateLiteral.FamilyShared),
  ExportReady: DataCustodyStateSchema.parse(DataCustodyStateLiteral.ExportReady),
} as const;

export const DataCustodyRawPayloadState = {
  Excluded: DataCustodyRawPayloadStateSchema.parse(DataCustodyRawPayloadStateLiteral.Excluded),
  RedactedOnly: DataCustodyRawPayloadStateSchema.parse(DataCustodyRawPayloadStateLiteral.RedactedOnly),
} as const;

export const DataCustodyRetentionDisposition = {
  Retain: DataCustodyRetentionDispositionSchema.parse(DataCustodyRetentionDispositionLiteral.Retain),
  DeleteEligible: DataCustodyRetentionDispositionSchema.parse(DataCustodyRetentionDispositionLiteral.DeleteEligible),
  DeleteRequested: DataCustodyRetentionDispositionSchema.parse(DataCustodyRetentionDispositionLiteral.DeleteRequested),
} as const;

export function parseDataCustodyBoundary(input: unknown): DataCustodyBoundary {
  return DataCustodyBoundarySchema.parse(input);
}

const DataCustodyBundleTypes = ['export', 'backup', 'import-preview', 'restore', 'support'] as const;

export const DataCustodyBundleTypeSchema = withParser(Schema.Literal(...DataCustodyBundleTypes));
export type DataCustodyBundleType = Infer<typeof DataCustodyBundleTypeSchema>;

export const DataCustodyBundleType = {
  Export: DataCustodyBundleTypeSchema.parse('export'),
  Backup: DataCustodyBundleTypeSchema.parse('backup'),
  ImportPreview: DataCustodyBundleTypeSchema.parse('import-preview'),
  Restore: DataCustodyBundleTypeSchema.parse('restore'),
  Support: DataCustodyBundleTypeSchema.parse('support'),
} as const;

const DataCustodyBundleStates = [
  'bundleQueued',
  'bundleWritten',
  'bundleVerified',
  'bundlePreviewOnly',
  'bundleApplyPending',
  'bundleApplied',
  'bundleRejected',
  'bundleCorrupt',
  'bundleWrongHousehold',
  'bundleWrongKey',
  'bundleManualRequired',
] as const;

export const DataCustodyBundleStateSchema = withParser(Schema.Literal(...DataCustodyBundleStates));
export type DataCustodyBundleState = Infer<typeof DataCustodyBundleStateSchema>;

export const DataCustodyBundleState = {
  BundleQueued: DataCustodyBundleStateSchema.parse('bundleQueued'),
  BundleWritten: DataCustodyBundleStateSchema.parse('bundleWritten'),
  BundleVerified: DataCustodyBundleStateSchema.parse('bundleVerified'),
  BundlePreviewOnly: DataCustodyBundleStateSchema.parse('bundlePreviewOnly'),
  BundleApplyPending: DataCustodyBundleStateSchema.parse('bundleApplyPending'),
  BundleApplied: DataCustodyBundleStateSchema.parse('bundleApplied'),
  BundleRejected: DataCustodyBundleStateSchema.parse('bundleRejected'),
  BundleCorrupt: DataCustodyBundleStateSchema.parse('bundleCorrupt'),
  BundleWrongHousehold: DataCustodyBundleStateSchema.parse('bundleWrongHousehold'),
  BundleWrongKey: DataCustodyBundleStateSchema.parse('bundleWrongKey'),
  BundleManualRequired: DataCustodyBundleStateSchema.parse('bundleManualRequired'),
} as const;

const DataCustodyRecoveryHandoffTargets = [
  'setup-restore-preview',
  'device-trust-recovery-persistence',
  'parent-local-delete-runtime',
] as const;

export const DataCustodyRecoveryHandoffTargetSchema = withParser(Schema.Literal(...DataCustodyRecoveryHandoffTargets));
export type DataCustodyRecoveryHandoffTarget = Infer<typeof DataCustodyRecoveryHandoffTargetSchema>;

export const DataCustodyRecoveryHandoffTarget = {
  SetupRestorePreview: DataCustodyRecoveryHandoffTargetSchema.parse('setup-restore-preview'),
  DeviceTrustRecoveryPersistence: DataCustodyRecoveryHandoffTargetSchema.parse('device-trust-recovery-persistence'),
  ParentLocalDeleteRuntime: DataCustodyRecoveryHandoffTargetSchema.parse('parent-local-delete-runtime'),
} as const;

const DataCustodyRecoveryHandoffStates = [
  'preview-only',
  'apply-pending',
  'applied',
  'partial-restore',
  'delete-pending',
  'delete-confirmed',
  'rejected',
  'manual-required',
] as const;

export const DataCustodyRecoveryHandoffStateSchema = withParser(Schema.Literal(...DataCustodyRecoveryHandoffStates));
export type DataCustodyRecoveryHandoffState = Infer<typeof DataCustodyRecoveryHandoffStateSchema>;

export const DataCustodyRecoveryHandoffState = {
  PreviewOnly: DataCustodyRecoveryHandoffStateSchema.parse('preview-only'),
  ApplyPending: DataCustodyRecoveryHandoffStateSchema.parse('apply-pending'),
  Applied: DataCustodyRecoveryHandoffStateSchema.parse('applied'),
  PartialRestore: DataCustodyRecoveryHandoffStateSchema.parse('partial-restore'),
  DeletePending: DataCustodyRecoveryHandoffStateSchema.parse('delete-pending'),
  DeleteConfirmed: DataCustodyRecoveryHandoffStateSchema.parse('delete-confirmed'),
  Rejected: DataCustodyRecoveryHandoffStateSchema.parse('rejected'),
  ManualRequired: DataCustodyRecoveryHandoffStateSchema.parse('manual-required'),
} as const;

const DataCustodyRecoveryHandoffBaseSchema = Schema.Struct({
  bundleType: DataCustodyBundleTypeSchema,
  bundleState: DataCustodyBundleStateSchema,
  handoffTarget: DataCustodyRecoveryHandoffTargetSchema,
  handoffState: DataCustodyRecoveryHandoffStateSchema,
  previewIsNonMutating: Schema.Boolean,
  explicitParentConfirmationRequired: Schema.Boolean,
  sourceOfTruthPreserved: Schema.Boolean,
  tombstonesPreserved: Schema.Boolean,
  deleteRequestRequired: Schema.Boolean,
});

type DataCustodyRecoveryHandoffCandidate = Infer<typeof DataCustodyRecoveryHandoffBaseSchema>;

export const DataCustodyRecoveryHandoffSchema = withParser(
  DataCustodyRecoveryHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        dataCustodyRecoveryHandoffIsSafe(handoff) ||
        'Expected recovery/delete handoff contracts to keep preview non-mutating, preserve tombstones, and require explicit confirmation or delete refs when applicable'
    )
  )
);

export type DataCustodyRecoveryHandoff = Infer<typeof DataCustodyRecoveryHandoffSchema>;

export const SeededDataCustodyClassIds = [...GeneratedDataCustodyClassIds] as const;
export const DataCustodyClassIdSchema = withParser(Schema.Literal(...GeneratedDataCustodyClassIds));
export type DataCustodyClassId = Infer<typeof DataCustodyClassIdSchema> & GeneratedDataCustodyClassId;

export const DataCustodyClassId = {
  AccountIdentityMetadata: DataCustodyClassIdSchema.parse('account-identity-metadata'),
  SubscriptionEntitlementMetadata: DataCustodyClassIdSchema.parse('subscription-entitlement-metadata'),
  BillingProviderIdentityReference: DataCustodyClassIdSchema.parse('billing-provider-identity-reference'),
  LicenseDownloadUpdateMetadata: DataCustodyClassIdSchema.parse('license-download-update-metadata'),
  HouseholdDeviceRegistry: DataCustodyClassIdSchema.parse('household-device-registry'),
  DeviceRegistrationPairingRouteMetadata: DataCustodyClassIdSchema.parse('device-registration-pairing-route-metadata'),
  SetupStateAndPairingDraft: DataCustodyClassIdSchema.parse('setup-state-and-pairing-draft'),
  MinimalNotificationRoutingMetadata: DataCustodyClassIdSchema.parse('minimal-notification-routing-metadata'),
  ShortLivedReportCompilerStatus: DataCustodyClassIdSchema.parse('short-lived-report-compiler-status'),
  SupportCaseMetadata: DataCustodyClassIdSchema.parse('support-case-metadata'),
  PublicWebsiteReleaseStatus: DataCustodyClassIdSchema.parse('public-website-release-status'),
  ChildProfile: DataCustodyClassIdSchema.parse('child-profile'),
  ParentRulesAndApprovalHistory: DataCustodyClassIdSchema.parse('parent-rules-and-approval-history'),
  AuditLog: DataCustodyClassIdSchema.parse('audit-log'),
  EvidenceJournalSegments: DataCustodyClassIdSchema.parse('evidence-journal-segments'),
  SqliteEvidenceReadModelDatabase: DataCustodyClassIdSchema.parse('sqlite-evidence-read-model-database'),
  ScreenshotsAndScreenAnalysisImages: DataCustodyClassIdSchema.parse('screenshots-and-screen-analysis-images'),
  BrowserUrlHistory: DataCustodyClassIdSchema.parse('browser-url-history'),
  NetworkAppGameEvidence: DataCustodyClassIdSchema.parse('network-app-game-evidence'),
  LocationTrackingEvidence: DataCustodyClassIdSchema.parse('location-tracking-evidence'),
  LocalAiAndPolicyDecisions: DataCustodyClassIdSchema.parse('local-ai-and-policy-decisions'),
  GeneratedLongTermReports: DataCustodyClassIdSchema.parse('generated-long-term-reports'),
  ParentNotificationHistoryCache: DataCustodyClassIdSchema.parse('parent-notification-history-cache'),
  AssistantChildEvidenceContext: DataCustodyClassIdSchema.parse('assistant-child-evidence-context'),
  ParentOwnedStorageContents: DataCustodyClassIdSchema.parse('parent-owned-storage-contents'),
  ProviderSyncPayloads: DataCustodyClassIdSchema.parse('provider-sync-payloads'),
  SupportBundlesContainingRawChildActivity: DataCustodyClassIdSchema.parse(
    'support-bundles-containing-raw-child-activity'
  ),
  UniversalDecryptKeys: DataCustodyClassIdSchema.parse('universal-decrypt-keys'),
} as const;

export const DataCustodySourceOfTruthKindSchema = withParser(Schema.Literal(...GeneratedDataCustodySourceOfTruthKinds));
export type DataCustodySourceOfTruthKind = Infer<typeof DataCustodySourceOfTruthKindSchema>;

const DataCustodySourceOfTruthBaseSchema = Schema.Struct({
  kind: DataCustodySourceOfTruthKindSchema,
  sourceClassIds: Schema.Array(DataCustodyClassIdSchema),
});

type DataCustodySourceOfTruthCandidate = Infer<typeof DataCustodySourceOfTruthBaseSchema>;

export const DataCustodySourceOfTruthSchema = withParser(
  DataCustodySourceOfTruthBaseSchema.pipe(
    Schema.filter(
      (source) =>
        dataCustodySourceOfTruthIsUnambiguous(source) ||
        'Expected source-of-truth to be self or derived from one or more explicit data classes'
    )
  )
);

export type DataCustodySourceOfTruth = Infer<typeof DataCustodySourceOfTruthSchema>;

export const DataCustodySourceOfTruth = {
  self(): DataCustodySourceOfTruth {
    return DataCustodySourceOfTruthSchema.parse({
      kind: 'self',
      sourceClassIds: [],
    });
  },
  derivedFromDataClasses(
    ...sourceClassIds: readonly DataCustodyClassId[]
  ): DataCustodySourceOfTruth {
    return DataCustodySourceOfTruthSchema.parse({
      kind: 'derived-from-data-classes',
      sourceClassIds,
    });
  },
} as const;

export const DataCustodyDefaultLocationSchema = withParser(Schema.Literal(...GeneratedDataCustodyDefaultLocations));
export type DataCustodyDefaultLocation = Infer<typeof DataCustodyDefaultLocationSchema> & GeneratedDataCustodyDefaultLocation;

export const DataCustodyDefaultLocation = {
  OcentraAccountMetadataStore: DataCustodyDefaultLocationSchema.parse('ocentra-account-metadata-store'),
  OcentraBillingMetadataStore: DataCustodyDefaultLocationSchema.parse('ocentra-billing-metadata-store'),
  BillingProviderCustomerRecord: DataCustodyDefaultLocationSchema.parse('billing-provider-customer-record'),
  OcentraLicenseUpdateStore: DataCustodyDefaultLocationSchema.parse('ocentra-license-update-store'),
  HouseholdDeviceRegistry: DataCustodyDefaultLocationSchema.parse('household-device-registry'),
  OcentraHouseholdRouteStore: DataCustodyDefaultLocationSchema.parse('ocentra-household-route-store'),
  HouseholdSetupDraftStore: DataCustodyDefaultLocationSchema.parse('household-setup-draft-store'),
  OcentraNotificationRouteStore: DataCustodyDefaultLocationSchema.parse('ocentra-notification-route-store'),
  OcentraShortLivedReportStatusStore: DataCustodyDefaultLocationSchema.parse('ocentra-short-lived-report-status-store'),
  OcentraSupportCaseStore: DataCustodyDefaultLocationSchema.parse('ocentra-support-case-store'),
  PublicReleaseSurface: DataCustodyDefaultLocationSchema.parse('public-release-surface'),
  HouseholdProfileStore: DataCustodyDefaultLocationSchema.parse('household-profile-store'),
  HouseholdRuleStore: DataCustodyDefaultLocationSchema.parse('household-rule-store'),
  HouseholdAuditStore: DataCustodyDefaultLocationSchema.parse('household-audit-store'),
  ChildDeviceEncryptedJournal: DataCustodyDefaultLocationSchema.parse('child-device-encrypted-journal'),
  ChildDeviceLocalQueryStore: DataCustodyDefaultLocationSchema.parse('child-device-local-query-store'),
  ChildDeviceSensitiveEvidenceStore: DataCustodyDefaultLocationSchema.parse('child-device-sensitive-evidence-store'),
  ChildDeviceLocationStore: DataCustodyDefaultLocationSchema.parse('child-device-location-store'),
  ChildDeviceLocalAiStore: DataCustodyDefaultLocationSchema.parse('child-device-local-ai-store'),
  ParentDeviceReportCache: DataCustodyDefaultLocationSchema.parse('parent-device-report-cache'),
  ParentDeviceNotificationHistoryCache: DataCustodyDefaultLocationSchema.parse(
    'parent-device-notification-history-cache'
  ),
  ParentAssistantEphemeralContext: DataCustodyDefaultLocationSchema.parse('parent-assistant-ephemeral-context'),
  ParentOwnedEncryptedStorage: DataCustodyDefaultLocationSchema.parse('parent-owned-encrypted-storage'),
  ProviderEnvelopeMetadata: DataCustodyDefaultLocationSchema.parse('provider-envelope-metadata'),
  SupportExportArtifact: DataCustodyDefaultLocationSchema.parse('support-export-artifact'),
  HouseholdKeyStore: DataCustodyDefaultLocationSchema.parse('household-key-store'),
} as const;

export const DataCustodyAuthoritySchema = withParser(Schema.Literal(...GeneratedDataCustodyAuthorities));
export type DataCustodyAuthority = Infer<typeof DataCustodyAuthoritySchema> & GeneratedDataCustodyAuthority;

export const DataCustodyAuthority = {
  OcentraAccountControlPlane: DataCustodyAuthoritySchema.parse('ocentra-account-control-plane'),
  PaymentControlPlane: DataCustodyAuthoritySchema.parse('payment-control-plane'),
  BillingProvider: DataCustodyAuthoritySchema.parse('billing-provider'),
  HouseholdControlPlane: DataCustodyAuthoritySchema.parse('household-control-plane'),
  OcentraRoutingService: DataCustodyAuthoritySchema.parse('ocentra-routing-service'),
  OcentraReportStatusRuntime: DataCustodyAuthoritySchema.parse('ocentra-report-status-runtime'),
  SupportSystem: DataCustodyAuthoritySchema.parse('support-system'),
  PublicReleasePipeline: DataCustodyAuthoritySchema.parse('public-release-pipeline'),
  ChildDevice: DataCustodyAuthoritySchema.parse('child-device'),
  ParentDevice: DataCustodyAuthoritySchema.parse('parent-device'),
  ParentOwnedStorage: DataCustodyAuthoritySchema.parse('parent-owned-storage'),
  SupportExportBoundary: DataCustodyAuthoritySchema.parse('support-export-boundary'),
} as const;

export const DataCustodyExposureSchema = withParser(Schema.Literal(...GeneratedDataCustodyExposures));
export type DataCustodyExposure = Infer<typeof DataCustodyExposureSchema> & GeneratedDataCustodyExposure;

export const DataCustodyExposure = {
  None: DataCustodyExposureSchema.parse('none'),
  AllowedReferencesOnly: DataCustodyExposureSchema.parse('allowed-references-only'),
  RedactedMetadataOnly: DataCustodyExposureSchema.parse('redacted-metadata-only'),
  Minimal: DataCustodyExposureSchema.parse('minimal'),
  DerivedOutputOnly: DataCustodyExposureSchema.parse('derived-output-only'),
  Public: DataCustodyExposureSchema.parse('public'),
} as const;

export const DataCustodyOcentraHostingModeSchema = withParser(
  Schema.Literal(...GeneratedDataCustodyOcentraHostingModes)
);
export type DataCustodyOcentraHostingMode =
  Infer<typeof DataCustodyOcentraHostingModeSchema> & GeneratedDataCustodyOcentraHostingMode;

export const DataCustodyOcentraHostingMode = {
  Forbidden: DataCustodyOcentraHostingModeSchema.parse('forbidden'),
  AllowedMetadataOnly: DataCustodyOcentraHostingModeSchema.parse('allowed-metadata-only'),
  ShortLivedStatusOnly: DataCustodyOcentraHostingModeSchema.parse('short-lived-status-only'),
  PublicReleaseOnly: DataCustodyOcentraHostingModeSchema.parse('public-release-only'),
} as const;

const DataCustodyHostingPolicyBaseSchema = Schema.Struct({
  ocentraHostingMode: DataCustodyOcentraHostingModeSchema,
  parentOwnedStorageAllowed: Schema.Boolean,
  providerMetadataAllowed: Schema.Boolean,
  supportExportParentInitiatedOnly: Schema.Boolean,
});

type DataCustodyHostingPolicyCandidate = Infer<typeof DataCustodyHostingPolicyBaseSchema>;

export const DataCustodyHostingPolicySchema = withParser(
  DataCustodyHostingPolicyBaseSchema.pipe(
    Schema.filter(
      (policy) =>
        dataCustodyHostingPolicyIsCoherent(policy) ||
        'Expected hosting policy to allow parent-owned storage or declare limited hosted metadata/status behavior'
    )
  )
);

export type DataCustodyHostingPolicy = Infer<typeof DataCustodyHostingPolicySchema>;

function dataCustodySourceOfTruthIsUnambiguous(source: DataCustodySourceOfTruthCandidate): boolean {
  if (source.kind === 'self') {
    return source.sourceClassIds.length === 0;
  }

  return source.sourceClassIds.length > 0;
}

function dataCustodyHostingPolicyIsCoherent(_policy: DataCustodyHostingPolicyCandidate): boolean {
  return true;
}

function dataCustodyRecoveryHandoffIsSafe(handoff: DataCustodyRecoveryHandoffCandidate): boolean {
  return (
    dataCustodyRecoveryHandoffPreservesTruth(handoff) &&
    dataCustodyRecoveryHandoffConfirmationIsSafe(handoff) &&
    dataCustodyRecoveryHandoffDeleteFlowIsSafe(handoff) &&
    dataCustodyRecoveryHandoffTargetIsSafe(handoff) &&
    dataCustodyRecoveryHandoffBundleStateIsSafe(handoff)
  );
}

function dataCustodyRecoveryHandoffPreservesTruth(handoff: DataCustodyRecoveryHandoffCandidate): boolean {
  return handoff.previewIsNonMutating && handoff.sourceOfTruthPreserved && handoff.tombstonesPreserved;
}

function dataCustodyRecoveryHandoffConfirmationIsSafe(handoff: DataCustodyRecoveryHandoffCandidate): boolean {
  const confirmationStates: readonly DataCustodyRecoveryHandoffCandidate['handoffState'][] = [
    'preview-only',
    'apply-pending',
    'applied',
    'partial-restore',
  ];
  return !confirmationStates.includes(handoff.handoffState) || handoff.explicitParentConfirmationRequired;
}

function dataCustodyRecoveryHandoffDeleteFlowIsSafe(handoff: DataCustodyRecoveryHandoffCandidate): boolean {
  const deleteStates: readonly DataCustodyRecoveryHandoffCandidate['handoffState'][] = [
    'delete-pending',
    'delete-confirmed',
  ];
  if (deleteStates.includes(handoff.handoffState)) {
    return handoff.deleteRequestRequired && handoff.handoffTarget === 'parent-local-delete-runtime';
  }

  return !handoff.deleteRequestRequired;
}

function dataCustodyRecoveryHandoffTargetIsSafe(handoff: DataCustodyRecoveryHandoffCandidate): boolean {
  const deleteRuntimeStates: readonly DataCustodyRecoveryHandoffCandidate['handoffState'][] = [
    'delete-pending',
    'delete-confirmed',
    'manual-required',
  ];
  const deleteStates: readonly DataCustodyRecoveryHandoffCandidate['handoffState'][] = [
    'delete-pending',
    'delete-confirmed',
  ];

  if (handoff.handoffTarget === 'parent-local-delete-runtime') {
    return deleteRuntimeStates.includes(handoff.handoffState);
  }

  return !deleteStates.includes(handoff.handoffState);
}

function dataCustodyRecoveryHandoffBundleStateIsSafe(handoff: DataCustodyRecoveryHandoffCandidate): boolean {
  const handoffStatesByBundleState: Record<
    DataCustodyRecoveryHandoffCandidate['bundleState'],
    readonly DataCustodyRecoveryHandoffState[]
  > = {
    bundleQueued: ['preview-only', 'apply-pending', 'applied', 'partial-restore', 'rejected', 'manual-required'],
    bundleWritten: ['preview-only', 'apply-pending', 'applied', 'partial-restore', 'rejected', 'manual-required'],
    bundleVerified: ['preview-only', 'apply-pending', 'applied', 'partial-restore', 'rejected', 'manual-required'],
    bundlePreviewOnly: ['preview-only'],
    bundleApplyPending: ['apply-pending'],
    bundleApplied: ['applied', 'partial-restore'],
    bundleRejected: ['rejected'],
    bundleCorrupt: ['rejected', 'manual-required'],
    bundleWrongHousehold: ['rejected'],
    bundleWrongKey: ['rejected', 'manual-required'],
    bundleManualRequired: ['manual-required'],
  };

  return handoffStatesByBundleState[handoff.bundleState].includes(handoff.handoffState);
}
