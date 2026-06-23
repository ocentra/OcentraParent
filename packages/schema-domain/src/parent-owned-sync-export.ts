import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  syncExportCoversRequiredDataClasses,
  syncExportContractProofIsHonest,
  syncExportRecoveryBundleIsHonest,
  syncExportRecoveryHandoffIsHonest,
} from './parent-owned-sync-export-validation';
import { countProductionProofValues } from './production-proof-shape';

export const ParentOwnedSyncExportSchemaVersionSchema = withParser(
  Schema.Literal('parent-owned-sync-export-manifest-proof')
);

export const ParentOwnedSyncExportDataClassSchema = withParser(
  Schema.Literal(
    'encrypted-journal-segment',
    'sqlite-query-row',
    'parent-rule',
    'approval-decision',
    'device-registry-entry',
    'notification-history',
    'audit-event',
    'generated-summary'
  )
);

export const ParentOwnedSyncExportFormatSchema = withParser(
  Schema.Literal('encrypted-machine-readable', 'encrypted-support-bundle', 'human-readable-parent-report')
);

export const ParentOwnedSyncExportDestinationOwnershipSchema = withParser(
  Schema.Literal(
    'child-local',
    'parent-device-local',
    'parent-owned-external-storage',
    'ocentra-hosted-non-activity-metadata'
  )
);

export const ParentOwnedSyncExportEncryptionStateSchema = withParser(
  Schema.Literal('encrypted-at-rest', 'human-readable-parent-authorized', 'not-applicable')
);

export const ParentOwnedSyncExportRetentionStateSchema = withParser(
  Schema.Literal('delete-after-export', 'parent-retained', 'retention-window', 'delete-requested', 'delete-confirmed')
);

export const ParentOwnedSyncExportConnectorProviderSchema = withParser(
  Schema.Literal('google-drive', 'onedrive', 'icloud-drive', 'dropbox', 'nas', 'local-folder', 'disabled')
);

export const ParentOwnedSyncExportConnectorStatusSchema = withParser(
  Schema.Literal(
    'ready',
    'revoked',
    'wrong-account',
    'folder-unavailable',
    'partial-upload',
    'disabled',
    'not-configured'
  )
);

export const ParentOwnedSyncExportSyncCursorStateSchema = withParser(
  Schema.Literal('fresh', 'stale', 'missing', 'conflict', 'not-started')
);

export const ParentOwnedSyncExportConflictResolutionSchema = withParser(
  Schema.Literal('local-wins', 'parent-storage-wins', 'manual-review-required', 'not-applicable')
);

export const ParentOwnedSyncExportImportResultStateSchema = withParser(
  Schema.Literal('accepted-preview', 'rejected-schema-version', 'rejected-scope', 'not-applied')
);

export const ParentOwnedSyncExportDeleteResultStateSchema = withParser(
  Schema.Literal('pending', 'confirmed', 'failed', 'not-requested')
);

export const ParentOwnedSyncExportNonClaimSchema = withParser(
  Schema.Literal(
    'no-transfer-runtime',
    'no-connector-oauth',
    'no-portal-ui',
    'no-default-ocentra-custody',
    'no-raw-child-evidence-upload-by-default',
    'no-report-compiler-runtime',
    'no-account-subscription-backend'
  )
);

const ParentOwnedSyncExportManifestIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportManifestId');
const ParentOwnedSyncExportItemIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportItemId');
const ParentOwnedSyncExportVersionLabelSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportVersionLabel');
const ParentOwnedSyncExportConnectorIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportConnectorId');
const ParentOwnedSyncExportConnectorRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportConnectorRef');
const ParentOwnedSyncExportCursorRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportCursorRef');
const ParentOwnedSyncExportBatchRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportBatchRef');
const ParentOwnedSyncExportConflictRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportConflictRef');
const ParentOwnedSyncExportResultRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportResultRef');
const ParentOwnedSyncExportPolicyRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncExportPolicyRef');
const ParentOwnedSyncExportProofRequirementSchema = brandedNonEmptyStringSchema(
  'ParentOwnedSyncExportProofRequirement'
);

const RequiredDataClasses = [
  'encrypted-journal-segment',
  'sqlite-query-row',
  'parent-rule',
  'approval-decision',
  'device-registry-entry',
  'notification-history',
  'audit-event',
  'generated-summary',
] as const;

const RequiredNonClaims = [
  'no-transfer-runtime',
  'no-connector-oauth',
  'no-portal-ui',
  'no-default-ocentra-custody',
  'no-raw-child-evidence-upload-by-default',
  'no-report-compiler-runtime',
  'no-account-subscription-backend',
] as const;

const ParentOwnedSyncExportEncryptionMetadataBaseSchema = Schema.Struct({
  encryptionState: ParentOwnedSyncExportEncryptionStateSchema,
  keyOwner: ParentOwnedSyncExportDestinationOwnershipSchema,
  encryptionMetadataRef: ParentOwnedSyncExportPolicyRefSchema,
  proofRequirement: ParentOwnedSyncExportProofRequirementSchema,
});

export const ParentOwnedSyncExportEncryptionMetadataSchema = withParser(
  ParentOwnedSyncExportEncryptionMetadataBaseSchema.pipe(
    Schema.filter(
      (metadata) =>
        metadata.encryptionState !== 'encrypted-at-rest' ||
        metadata.keyOwner !== 'ocentra-hosted-non-activity-metadata' ||
        'Expected encrypted sync/export data keys to stay child-local, parent-device-local, or parent-owned'
    )
  )
);

const ParentOwnedSyncExportRetentionPolicyBaseSchema = Schema.Struct({
  retentionState: ParentOwnedSyncExportRetentionStateSchema,
  retentionPolicyRef: ParentOwnedSyncExportPolicyRefSchema,
  deleteResultRef: Schema.Union(ParentOwnedSyncExportResultRefSchema, Schema.Null),
  parentActionRequired: Schema.Boolean,
  auditRequired: Schema.Boolean,
});

export const ParentOwnedSyncExportRetentionPolicySchema = withParser(
  ParentOwnedSyncExportRetentionPolicyBaseSchema.pipe(
    Schema.filter(
      (policy) =>
        syncExportRetentionPolicyIsExplicit(policy) ||
        'Expected retention/delete states to require parent action, audit, and delete result refs when applicable'
    )
  )
);

const ParentOwnedSyncExportItemDescriptorBaseSchema = Schema.Struct({
  itemId: ParentOwnedSyncExportItemIdSchema,
  dataClass: ParentOwnedSyncExportDataClassSchema,
  exportFormat: ParentOwnedSyncExportFormatSchema,
  destinationOwnership: ParentOwnedSyncExportDestinationOwnershipSchema,
  schemaVersionLabel: ParentOwnedSyncExportVersionLabelSchema,
  encryption: ParentOwnedSyncExportEncryptionMetadataSchema,
  retention: ParentOwnedSyncExportRetentionPolicySchema,
  evidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  manifestRefs: Schema.Array(ParentOwnedSyncExportManifestIdSchema),
  parentActionRequired: Schema.Boolean,
  rawChildEvidenceUploadedByDefault: Schema.Boolean,
  ocentraHostedFamilyDataStored: Schema.Boolean,
  transferRuntimeClaimed: Schema.Boolean,
});

export const ParentOwnedSyncExportItemDescriptorSchema = withParser(
  ParentOwnedSyncExportItemDescriptorBaseSchema.pipe(
    Schema.filter(
      (item) =>
        syncExportItemDescriptorIsHonest(item) ||
        'Expected sync/export item descriptors to be parent-action-scoped, custody-safe, encrypted when needed, and contract-only'
    )
  )
);

const ParentOwnedSyncExportConnectorStatusRowBaseSchema = Schema.Struct({
  connectorId: ParentOwnedSyncExportConnectorIdSchema,
  provider: ParentOwnedSyncExportConnectorProviderSchema,
  status: ParentOwnedSyncExportConnectorStatusSchema,
  destinationOwnership: ParentOwnedSyncExportDestinationOwnershipSchema,
  accountRef: Schema.Union(ParentOwnedSyncExportConnectorRefSchema, Schema.Null),
  folderRef: Schema.Union(ParentOwnedSyncExportConnectorRefSchema, Schema.Null),
  revocationRef: Schema.Union(ParentOwnedSyncExportConnectorRefSchema, Schema.Null),
  statusRef: ParentOwnedSyncExportConnectorRefSchema,
  lastCheckedAt: ParentTimestampSchema,
  oauthRuntimeClaimed: Schema.Boolean,
  uploadRuntimeClaimed: Schema.Boolean,
  deleteRuntimeClaimed: Schema.Boolean,
});

export const ParentOwnedSyncExportConnectorStatusRowSchema = withParser(
  ParentOwnedSyncExportConnectorStatusRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        syncExportConnectorStatusRowIsHonest(row) ||
        'Expected connector rows to expose status without OAuth, upload, delete, or Ocentra custody overclaims'
    )
  )
);

const ParentOwnedSyncExportSyncCursorBaseSchema = Schema.Struct({
  cursorState: ParentOwnedSyncExportSyncCursorStateSchema,
  cursorRef: Schema.Union(ParentOwnedSyncExportCursorRefSchema, Schema.Null),
  batchRef: Schema.Union(ParentOwnedSyncExportBatchRefSchema, Schema.Null),
  lastSuccessfulSyncAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  conflictRefs: Schema.Array(ParentOwnedSyncExportConflictRefSchema),
  retryQueueState: Schema.Union(ParentOwnedSyncExportPolicyRefSchema, Schema.Null),
});

export const ParentOwnedSyncExportSyncCursorSchema = withParser(
  ParentOwnedSyncExportSyncCursorBaseSchema.pipe(
    Schema.filter(
      (cursor) =>
        syncExportCursorStateIsHonest(cursor) ||
        'Expected sync cursor states to keep stale/missing/conflict/not-started states explicit'
    )
  )
);

const ParentOwnedSyncExportConflictRecordBaseSchema = Schema.Struct({
  conflictRef: ParentOwnedSyncExportConflictRefSchema,
  dataClass: ParentOwnedSyncExportDataClassSchema,
  resolution: ParentOwnedSyncExportConflictResolutionSchema,
  parentAction: Schema.Union(ParentActionReferenceSchema, Schema.Null),
  localVersionRef: ParentOwnedSyncExportVersionLabelSchema,
  parentStorageVersionRef: Schema.Union(ParentOwnedSyncExportVersionLabelSchema, Schema.Null),
  auditRefs: Schema.Array(ParentEvidenceReferenceSchema),
});

export const ParentOwnedSyncExportConflictRecordSchema = withParser(
  ParentOwnedSyncExportConflictRecordBaseSchema.pipe(
    Schema.filter(
      (record) =>
        syncExportConflictRecordIsHonest(record) ||
        'Expected conflict records to keep manual review and parent-storage versions explicit before applying changes'
    )
  )
);

const ParentOwnedSyncExportImportResultBaseSchema = Schema.Struct({
  resultRef: ParentOwnedSyncExportResultRefSchema,
  resultState: ParentOwnedSyncExportImportResultStateSchema,
  acceptedSchemaVersion: Schema.Union(ParentOwnedSyncExportVersionLabelSchema, Schema.Null),
  rejectedReasonRef: Schema.Union(ParentOwnedSyncExportPolicyRefSchema, Schema.Null),
  appliedToLocalEvidence: Schema.Boolean,
  auditRefs: Schema.Array(ParentEvidenceReferenceSchema),
});

export const ParentOwnedSyncExportImportResultSchema = withParser(
  ParentOwnedSyncExportImportResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        syncExportImportResultIsHonest(result) ||
        'Expected import results to validate schema/scope and avoid applying untrusted data in this proof'
    )
  )
);

const ParentOwnedSyncExportDeleteResultBaseSchema = Schema.Struct({
  resultRef: ParentOwnedSyncExportResultRefSchema,
  resultState: ParentOwnedSyncExportDeleteResultStateSchema,
  dataClass: ParentOwnedSyncExportDataClassSchema,
  deleteRequestRef: Schema.Union(ParentOwnedSyncExportPolicyRefSchema, Schema.Null),
  connectorStatusRef: ParentOwnedSyncExportConnectorRefSchema,
  auditRefs: Schema.Array(ParentEvidenceReferenceSchema),
});

export const ParentOwnedSyncExportDeleteResultSchema = withParser(
  ParentOwnedSyncExportDeleteResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        syncExportDeleteResultIsHonest(result) ||
        'Expected delete results to carry request and audit refs for pending, confirmed, or failed deletion'
    )
  )
);

export const ParentOwnedSyncExportRecoveryBundleStateSchema = withParser(
  Schema.Literal(
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
    'bundleManualRequired'
  )
);

export const ParentOwnedSyncExportRecoveryBundleTypeSchema = withParser(
  Schema.Literal('export', 'backup', 'import-preview', 'restore', 'support')
);

export const ParentOwnedSyncExportRecoveryHandoffTargetSchema = withParser(
  Schema.Literal('setup-restore-preview', 'device-trust-recovery-persistence', 'parent-local-delete-runtime')
);

export const ParentOwnedSyncExportRecoveryHandoffStateSchema = withParser(
  Schema.Literal(
    'preview-only',
    'apply-pending',
    'applied',
    'partial-restore',
    'delete-pending',
    'delete-confirmed',
    'rejected',
    'manual-required'
  )
);

export const ParentOwnedSyncExportRecoveryBundleBindingStateSchema = withParser(
  Schema.Literal('matched', 'mismatched', 'absent')
);

export const ParentOwnedSyncExportRecoveryBundleKeyAvailabilityStateSchema = withParser(
  Schema.Literal('available', 'wrong-key', 'recovery-not-supported', 'manual-required')
);

const ParentOwnedSyncExportRecoveryBundleRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedSyncExportRecoveryBundleRef'
);
const ParentOwnedSyncExportRecoveryHandoffRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedSyncExportRecoveryHandoffRef'
);

const ParentOwnedSyncExportRecoveryHandoffBaseSchema = Schema.Struct({
  handoffRef: ParentOwnedSyncExportRecoveryHandoffRefSchema,
  handoffTarget: ParentOwnedSyncExportRecoveryHandoffTargetSchema,
  handoffState: ParentOwnedSyncExportRecoveryHandoffStateSchema,
  previewIsNonMutating: Schema.Boolean,
  explicitParentConfirmationRequired: Schema.Boolean,
  sourceOfTruthPreserved: Schema.Boolean,
  tombstonesPreserved: Schema.Boolean,
  deleteRequestRequired: Schema.Boolean,
});

export const ParentOwnedSyncExportRecoveryHandoffSchema = withParser(
  ParentOwnedSyncExportRecoveryHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        syncExportRecoveryHandoffIsHonest(handoff) ||
        'Expected recovery handoff records to keep preview non-mutating, preserve source-of-truth and tombstones, and keep delete handoff explicit'
    )
  )
);

const ParentOwnedSyncExportRecoveryBundleBaseSchema = Schema.Struct({
  bundleRef: ParentOwnedSyncExportRecoveryBundleRefSchema,
  manifestRef: ParentOwnedSyncExportManifestIdSchema,
  bundleType: ParentOwnedSyncExportRecoveryBundleTypeSchema,
  bundleState: ParentOwnedSyncExportRecoveryBundleStateSchema,
  handoff: ParentOwnedSyncExportRecoveryHandoffSchema,
  sourceHouseholdBindingState: ParentOwnedSyncExportRecoveryBundleBindingStateSchema,
  sourceDeviceBindingState: ParentOwnedSyncExportRecoveryBundleBindingStateSchema,
  keyAvailabilityState: ParentOwnedSyncExportRecoveryBundleKeyAvailabilityStateSchema,
  acceptedDataClasses: Schema.Array(ParentOwnedSyncExportDataClassSchema),
  rejectedDataClasses: Schema.Array(ParentOwnedSyncExportDataClassSchema),
  rejectionReasonRef: Schema.Union(ParentOwnedSyncExportPolicyRefSchema, Schema.Null),
  deleteRequestRef: Schema.Union(ParentOwnedSyncExportResultRefSchema, Schema.Null),
  previewMutatedLocalTruth: Schema.Boolean,
  applyConfirmedByParent: Schema.Boolean,
  auditRefs: Schema.Array(ParentEvidenceReferenceSchema),
});

export const ParentOwnedSyncExportRecoveryBundleSchema = withParser(
  ParentOwnedSyncExportRecoveryBundleBaseSchema.pipe(
    Schema.filter(
      (bundle) =>
        syncExportRecoveryBundleIsHonest(bundle) ||
        'Expected recovery bundle records to keep preview non-mutating, reject wrong household/key/corrupt bundles, and keep partial restore or delete handoff explicit'
    )
  )
);

const ParentOwnedSyncExportManifestBaseSchema = Schema.Struct({
  schemaVersion: ParentOwnedSyncExportSchemaVersionSchema,
  manifestId: ParentOwnedSyncExportManifestIdSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  productVersion: ParentOwnedSyncExportVersionLabelSchema,
  manifestVersion: ParentOwnedSyncExportVersionLabelSchema,
  generatedAt: ParentTimestampSchema,
  parentAction: ParentActionReferenceSchema,
  endpointContractRef: ParentOwnedSyncExportPolicyRefSchema,
  items: Schema.Array(ParentOwnedSyncExportItemDescriptorSchema),
});

export const ParentOwnedSyncExportManifestSchema = withParser(
  ParentOwnedSyncExportManifestBaseSchema.pipe(
    Schema.filter(
      (manifest) =>
        syncExportCoversRequiredDataClasses(manifest.items) ||
        'Expected export manifest to cover every required sync/export data class'
    )
  )
);

const ParentOwnedSyncExportContractProofBaseSchema = Schema.Struct({
  schemaVersion: ParentOwnedSyncExportSchemaVersionSchema,
  manifest: ParentOwnedSyncExportManifestSchema,
  connectorStatuses: Schema.Array(ParentOwnedSyncExportConnectorStatusRowSchema),
  syncCursors: Schema.Array(ParentOwnedSyncExportSyncCursorSchema),
  conflictRecords: Schema.Array(ParentOwnedSyncExportConflictRecordSchema),
  importResults: Schema.Array(ParentOwnedSyncExportImportResultSchema),
  deleteResults: Schema.Array(ParentOwnedSyncExportDeleteResultSchema),
  recoveryBundles: Schema.Array(ParentOwnedSyncExportRecoveryBundleSchema),
  nonClaims: Schema.Array(ParentOwnedSyncExportNonClaimSchema),
  transferRuntimeClaimed: Schema.Boolean,
  connectorOAuthClaimed: Schema.Boolean,
  portalUiClaimed: Schema.Boolean,
  reportCompilerRuntimeClaimed: Schema.Boolean,
  accountSubscriptionBackendClaimed: Schema.Boolean,
  ocentraHostedChildEvidenceStored: Schema.Boolean,
  updatedAt: ParentTimestampSchema,
});

export const ParentOwnedSyncExportContractProofSchema = withParser(
  ParentOwnedSyncExportContractProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        syncExportContractProofIsHonest(proof) ||
        'Expected parent-owned sync/export proof to cover manifest, connector status, cursor, conflict, import/delete, and custody boundaries without runtime overclaims'
    )
  )
);

type ParentOwnedSyncExportRetentionPolicyCandidate = Infer<typeof ParentOwnedSyncExportRetentionPolicyBaseSchema>;
type ParentOwnedSyncExportItemDescriptorCandidate = Infer<typeof ParentOwnedSyncExportItemDescriptorBaseSchema>;
type ParentOwnedSyncExportConnectorStatusRowCandidate = Infer<typeof ParentOwnedSyncExportConnectorStatusRowBaseSchema>;
type ParentOwnedSyncExportSyncCursorCandidate = Infer<typeof ParentOwnedSyncExportSyncCursorBaseSchema>;
type ParentOwnedSyncExportConflictRecordCandidate = Infer<typeof ParentOwnedSyncExportConflictRecordBaseSchema>;
type ParentOwnedSyncExportImportResultCandidate = Infer<typeof ParentOwnedSyncExportImportResultBaseSchema>;
type ParentOwnedSyncExportDeleteResultCandidate = Infer<typeof ParentOwnedSyncExportDeleteResultBaseSchema>;
export type ParentOwnedSyncExportRecoveryHandoffCandidate = Infer<
  typeof ParentOwnedSyncExportRecoveryHandoffBaseSchema
>;
export type ParentOwnedSyncExportRecoveryBundleCandidate = Infer<typeof ParentOwnedSyncExportRecoveryBundleBaseSchema>;
export type ParentOwnedSyncExportContractProofCandidate = Infer<typeof ParentOwnedSyncExportContractProofBaseSchema>;

export type ParentOwnedSyncExportDataClass = Infer<typeof ParentOwnedSyncExportDataClassSchema>;
export type ParentOwnedSyncExportFormat = Infer<typeof ParentOwnedSyncExportFormatSchema>;
export type ParentOwnedSyncExportDestinationOwnership = Infer<typeof ParentOwnedSyncExportDestinationOwnershipSchema>;
export type ParentOwnedSyncExportConnectorStatus = Infer<typeof ParentOwnedSyncExportConnectorStatusSchema>;
export type ParentOwnedSyncExportSyncCursorState = Infer<typeof ParentOwnedSyncExportSyncCursorStateSchema>;
export type ParentOwnedSyncExportConflictResolution = Infer<typeof ParentOwnedSyncExportConflictResolutionSchema>;
export type ParentOwnedSyncExportImportResultState = Infer<typeof ParentOwnedSyncExportImportResultStateSchema>;
export type ParentOwnedSyncExportDeleteResultState = Infer<typeof ParentOwnedSyncExportDeleteResultStateSchema>;
export type ParentOwnedSyncExportRecoveryBundleState = Infer<typeof ParentOwnedSyncExportRecoveryBundleStateSchema>;
export type ParentOwnedSyncExportRecoveryBundleType = Infer<typeof ParentOwnedSyncExportRecoveryBundleTypeSchema>;
export type ParentOwnedSyncExportRecoveryHandoffTarget = Infer<typeof ParentOwnedSyncExportRecoveryHandoffTargetSchema>;
export type ParentOwnedSyncExportRecoveryHandoffState = Infer<typeof ParentOwnedSyncExportRecoveryHandoffStateSchema>;
export type ParentOwnedSyncExportNonClaim = Infer<typeof ParentOwnedSyncExportNonClaimSchema>;
export type ParentOwnedSyncExportItemDescriptor = Infer<typeof ParentOwnedSyncExportItemDescriptorSchema>;
export type ParentOwnedSyncExportConnectorStatusRow = Infer<typeof ParentOwnedSyncExportConnectorStatusRowSchema>;
export type ParentOwnedSyncExportSyncCursor = Infer<typeof ParentOwnedSyncExportSyncCursorSchema>;
export type ParentOwnedSyncExportConflictRecord = Infer<typeof ParentOwnedSyncExportConflictRecordSchema>;
export type ParentOwnedSyncExportImportResult = Infer<typeof ParentOwnedSyncExportImportResultSchema>;
export type ParentOwnedSyncExportDeleteResult = Infer<typeof ParentOwnedSyncExportDeleteResultSchema>;
export type ParentOwnedSyncExportRecoveryHandoff = Infer<typeof ParentOwnedSyncExportRecoveryHandoffSchema>;
export type ParentOwnedSyncExportRecoveryBundle = Infer<typeof ParentOwnedSyncExportRecoveryBundleSchema>;
export type ParentOwnedSyncExportContractProof = Infer<typeof ParentOwnedSyncExportContractProofSchema>;

function syncExportRetentionPolicyIsExplicit(policy: ParentOwnedSyncExportRetentionPolicyCandidate): boolean {
  if (!policy.parentActionRequired || !policy.auditRequired) {
    return false;
  }
  if (policy.retentionState === 'delete-requested' || policy.retentionState === 'delete-confirmed') {
    return policy.deleteResultRef !== null;
  }
  return policy.deleteResultRef === null;
}

function syncExportItemDescriptorIsHonest(item: ParentOwnedSyncExportItemDescriptorCandidate): boolean {
  return (
    item.parentActionRequired &&
    item.evidenceRefs.length > 0 &&
    item.manifestRefs.length > 0 &&
    !item.rawChildEvidenceUploadedByDefault &&
    !item.ocentraHostedFamilyDataStored &&
    !item.transferRuntimeClaimed &&
    syncExportFormatMatchesDataClass(item) &&
    item.destinationOwnership !== 'ocentra-hosted-non-activity-metadata'
  );
}

function syncExportFormatMatchesDataClass(item: ParentOwnedSyncExportItemDescriptorCandidate): boolean {
  if (item.dataClass === 'generated-summary') {
    return item.exportFormat === 'human-readable-parent-report' || item.exportFormat === 'encrypted-support-bundle';
  }
  if (item.dataClass === 'encrypted-journal-segment' || item.dataClass === 'sqlite-query-row') {
    return item.exportFormat === 'encrypted-machine-readable';
  }
  return item.exportFormat === 'encrypted-machine-readable' || item.exportFormat === 'encrypted-support-bundle';
}

function syncExportConnectorStatusRowIsHonest(row: ParentOwnedSyncExportConnectorStatusRowCandidate): boolean {
  if (row.oauthRuntimeClaimed || row.uploadRuntimeClaimed || row.deleteRuntimeClaimed) {
    return false;
  }
  if (row.destinationOwnership === 'ocentra-hosted-non-activity-metadata') {
    return false;
  }
  if (row.status === 'ready') {
    return row.accountRef !== null && row.folderRef !== null && row.revocationRef === null;
  }
  if (row.status === 'revoked') {
    return row.revocationRef !== null;
  }
  if (row.status === 'disabled' || row.status === 'not-configured') {
    return row.accountRef === null && row.folderRef === null;
  }
  return row.statusRef.length > 0;
}

function syncExportCursorStateIsHonest(cursor: ParentOwnedSyncExportSyncCursorCandidate): boolean {
  if (cursor.cursorState === 'fresh') {
    return cursor.cursorRef !== null && cursor.batchRef !== null && cursor.lastSuccessfulSyncAt !== null;
  }
  if (cursor.cursorState === 'conflict') {
    return cursor.conflictRefs.length > 0 && cursor.retryQueueState !== null;
  }
  if (cursor.cursorState === 'not-started') {
    return cursor.cursorRef === null && cursor.batchRef === null && cursor.lastSuccessfulSyncAt === null;
  }
  return cursor.retryQueueState !== null;
}

function syncExportConflictRecordIsHonest(record: ParentOwnedSyncExportConflictRecordCandidate): boolean {
  if (record.auditRefs.length === 0) {
    return false;
  }
  if (record.resolution === 'manual-review-required') {
    return record.parentAction === null && record.parentStorageVersionRef !== null;
  }
  if (record.resolution === 'not-applicable') {
    return record.parentStorageVersionRef === null;
  }
  return record.parentAction !== null && record.parentStorageVersionRef !== null;
}

function syncExportImportResultIsHonest(result: ParentOwnedSyncExportImportResultCandidate): boolean {
  if (result.auditRefs.length === 0 || result.appliedToLocalEvidence) {
    return false;
  }
  if (result.resultState === 'accepted-preview') {
    return result.acceptedSchemaVersion !== null && result.rejectedReasonRef === null;
  }
  return result.acceptedSchemaVersion === null && result.rejectedReasonRef !== null;
}

function syncExportDeleteResultIsHonest(result: ParentOwnedSyncExportDeleteResultCandidate): boolean {
  if (result.auditRefs.length === 0) {
    return false;
  }
  if (result.resultState === 'not-requested') {
    return result.deleteRequestRef === null;
  }
  return result.deleteRequestRef !== null;
}

const Timestamp = '2026-06-03T09:03:46.841Z';
const Family = { familyId: 'family-sync-export-proof-1' } as const;
const Device = {
  deviceId: 'windows-child-device-sync-export-proof-1',
  childProfileId: 'child-sync-export-proof-1',
  label: 'Windows child device sync export proof',
  platform: 'windows',
} as const;
const ParentAction = {
  actionReferenceId: 'parent-action-sync-export-proof-1',
  actor: { actorId: 'parent-sync-export-proof-1', role: 'parent' },
  policyVersion: 'parent-owned-sync-export-policy-v1',
  createdAt: Timestamp,
} as const;
const EvidenceRef = {
  evidenceReferenceId: 'evidence-sync-export-proof-1',
  kind: 'journal-event',
  observedAt: Timestamp,
} as const;
const ManifestId = 'parent-owned-sync-export-manifest-proof-1';

export const ParentOwnedSyncExportContractProofReadModel = ParentOwnedSyncExportContractProofSchema.parse({
  schemaVersion: 'parent-owned-sync-export-manifest-proof',
  manifest: {
    schemaVersion: 'parent-owned-sync-export-manifest-proof',
    manifestId: ManifestId,
    family: Family,
    device: Device,
    productVersion: '0.1.1',
    manifestVersion: 'sync-export.manifest.v1',
    generatedAt: Timestamp,
    parentAction: ParentAction,
    endpointContractRef: 'sync-export-endpoint-contract-proof',
    items: [
      item('encrypted-journal-segment', 'encrypted-machine-readable', 'delete-after-export'),
      item('sqlite-query-row', 'encrypted-machine-readable', 'retention-window'),
      item('parent-rule', 'encrypted-support-bundle', 'parent-retained'),
      item('approval-decision', 'encrypted-support-bundle', 'parent-retained'),
      item('device-registry-entry', 'encrypted-machine-readable', 'retention-window'),
      item('notification-history', 'encrypted-support-bundle', 'retention-window'),
      item('audit-event', 'encrypted-machine-readable', 'delete-requested'),
      item('generated-summary', 'human-readable-parent-report', 'delete-confirmed'),
    ],
  },
  connectorStatuses: [
    connector('google-drive', 'ready', 'account-google-drive-proof', 'folder-google-drive-proof', null),
    connector('onedrive', 'revoked', 'account-onedrive-proof', 'folder-onedrive-proof', 'revocation-onedrive-proof'),
    connector('icloud-drive', 'wrong-account', 'account-icloud-proof', 'folder-icloud-proof', null),
    connector('dropbox', 'folder-unavailable', 'account-dropbox-proof', 'folder-dropbox-proof', null),
    connector('nas', 'partial-upload', 'account-nas-proof', 'folder-nas-proof', null),
    connector('local-folder', 'disabled', null, null, null),
    connector('disabled', 'not-configured', null, null, null),
  ],
  syncCursors: [
    cursor('fresh', 'cursor-fresh-proof', 'batch-fresh-proof', Timestamp, [], 'retry-queue-empty-ref'),
    cursor('stale', 'cursor-stale-proof', 'batch-stale-proof', null, [], 'retry-queue-stale-ref'),
    cursor('missing', null, null, null, [], 'retry-queue-missing-ref'),
    cursor(
      'conflict',
      'cursor-conflict-proof',
      'batch-conflict-proof',
      null,
      ['conflict-parent-rule-proof'],
      'retry-queue-conflict-ref'
    ),
    cursor('not-started', null, null, null, [], null),
  ],
  conflictRecords: [
    conflict('conflict-local-wins-proof', 'parent-rule', 'local-wins', ParentAction, 'local-rule-v2', 'parent-rule-v1'),
    conflict(
      'conflict-parent-storage-wins-proof',
      'approval-decision',
      'parent-storage-wins',
      ParentAction,
      'local-approval-v1',
      'parent-approval-v2'
    ),
    conflict(
      'conflict-manual-review-proof',
      'device-registry-entry',
      'manual-review-required',
      null,
      'local-device-v2',
      'parent-device-v2'
    ),
    conflict('conflict-none-proof', 'generated-summary', 'not-applicable', null, 'local-summary-v1', null),
  ],
  importResults: [
    importResult('import-accepted-preview-proof', 'accepted-preview', 'sync-export.manifest.v1', null),
    importResult('import-rejected-schema-proof', 'rejected-schema-version', null, 'unsupported-schema-version-ref'),
    importResult('import-rejected-scope-proof', 'rejected-scope', null, 'mismatched-family-or-device-scope-ref'),
    importResult('import-not-applied-proof', 'not-applied', null, 'preview-only-not-applied-ref'),
  ],
  deleteResults: [
    deleteResult('delete-pending-proof', 'pending', 'audit-event', 'delete-request-audit-event-proof'),
    deleteResult('delete-confirmed-proof', 'confirmed', 'generated-summary', 'delete-request-generated-summary-proof'),
    deleteResult('delete-failed-proof', 'failed', 'notification-history', 'delete-request-notification-history-proof'),
    deleteResult('delete-not-requested-proof', 'not-requested', 'parent-rule', null),
  ],
  recoveryBundles: [
    recoveryBundle(
      'bundle-preview-setup-proof',
      'import-preview',
      'bundlePreviewOnly',
      handoff('handoff-setup-preview-proof', 'setup-restore-preview', 'preview-only'),
      'matched',
      'matched',
      'available',
      ['encrypted-journal-segment', 'generated-summary'],
      [],
      null,
      null,
      false
    ),
    recoveryBundle(
      'bundle-apply-pending-device-trust-proof',
      'restore',
      'bundleApplyPending',
      handoff('handoff-device-trust-pending-proof', 'device-trust-recovery-persistence', 'apply-pending'),
      'matched',
      'matched',
      'available',
      ['encrypted-journal-segment', 'sqlite-query-row', 'parent-rule'],
      [],
      null,
      null,
      false
    ),
    recoveryBundle(
      'bundle-partial-restore-device-trust-proof',
      'restore',
      'bundleApplied',
      handoff('handoff-device-trust-partial-proof', 'device-trust-recovery-persistence', 'partial-restore'),
      'matched',
      'matched',
      'available',
      ['encrypted-journal-segment', 'sqlite-query-row', 'generated-summary'],
      ['device-registry-entry'],
      null,
      null,
      true
    ),
    recoveryBundle(
      'bundle-wrong-household-proof',
      'import-preview',
      'bundleWrongHousehold',
      handoff('handoff-wrong-household-proof', 'setup-restore-preview', 'rejected'),
      'mismatched',
      'matched',
      'available',
      [],
      ['encrypted-journal-segment', 'sqlite-query-row'],
      'wrong-household-binding-ref',
      null,
      false
    ),
    recoveryBundle(
      'bundle-wrong-key-proof',
      'restore',
      'bundleWrongKey',
      handoff('handoff-wrong-key-proof', 'device-trust-recovery-persistence', 'rejected'),
      'matched',
      'matched',
      'wrong-key',
      [],
      ['encrypted-journal-segment', 'generated-summary'],
      'wrong-key-ref',
      null,
      false
    ),
    recoveryBundle(
      'bundle-corrupt-proof',
      'backup',
      'bundleCorrupt',
      handoff('handoff-corrupt-proof', 'setup-restore-preview', 'rejected'),
      'matched',
      'absent',
      'available',
      [],
      ['encrypted-journal-segment', 'sqlite-query-row', 'generated-summary'],
      'integrity-check-failed-ref',
      null,
      false
    ),
    recoveryBundle(
      'bundle-manual-required-proof',
      'restore',
      'bundleManualRequired',
      handoff('handoff-manual-required-proof', 'device-trust-recovery-persistence', 'manual-required'),
      'matched',
      'matched',
      'recovery-not-supported',
      ['generated-summary'],
      ['encrypted-journal-segment'],
      'manual-recovery-approval-ref',
      null,
      false
    ),
    recoveryBundle(
      'bundle-delete-pending-proof',
      'export',
      'bundleWritten',
      handoff('handoff-delete-pending-proof', 'parent-local-delete-runtime', 'delete-pending'),
      'matched',
      'matched',
      'available',
      ['audit-event'],
      [],
      null,
      'delete-handoff-request-proof',
      false
    ),
    recoveryBundle(
      'bundle-delete-confirmed-proof',
      'export',
      'bundleWritten',
      handoff('handoff-delete-confirmed-proof', 'parent-local-delete-runtime', 'delete-confirmed'),
      'matched',
      'matched',
      'available',
      ['generated-summary'],
      [],
      null,
      'delete-handoff-confirmed-proof',
      false
    ),
  ],
  nonClaims: [...RequiredNonClaims],
  transferRuntimeClaimed: false,
  connectorOAuthClaimed: false,
  portalUiClaimed: false,
  reportCompilerRuntimeClaimed: false,
  accountSubscriptionBackendClaimed: false,
  ocentraHostedChildEvidenceStored: false,
  updatedAt: Timestamp,
});

export const ParentOwnedSyncExportKnownGaps = [
  'No export/import/upload/download runtime is implemented by this production-domain contract proof.',
  'No connector OAuth, token refresh, revocation runtime, or provider API calls are implemented.',
  'No portal UI or CLI control is claimed for export, sync, retention, or delete controls.',
  'No Ocentra-hosted storage of child evidence, generated reports, journal data, or query rows is claimed.',
  'Parent-owned local export/delete execution remains a separate parent-domain holdout; this proof only defines delete and recovery handoff contracts.',
  'Real encrypted journal export/import, SQLite rebuild, conflict replay, and delete execution remain future work.',
] as const;

export function summarizeParentOwnedSyncExportDataClasses(
  items: ReadonlyArray<ParentOwnedSyncExportItemDescriptor>
): Record<ParentOwnedSyncExportDataClass, number> {
  return countProductionProofValues(
    items.map((itemEntry) => itemEntry.dataClass),
    RequiredDataClasses
  );
}

export function summarizeParentOwnedSyncExportConnectorStatuses(
  rows: ReadonlyArray<ParentOwnedSyncExportConnectorStatusRow>
): Record<ParentOwnedSyncExportConnectorStatus, number> {
  return countProductionProofValues(
    rows.map((row) => row.status),
    ['ready', 'revoked', 'wrong-account', 'folder-unavailable', 'partial-upload', 'disabled', 'not-configured'] as const
  );
}

export function summarizeParentOwnedSyncExportRecoveryBundleStates(
  bundles: ReadonlyArray<ParentOwnedSyncExportRecoveryBundle>
): Record<ParentOwnedSyncExportRecoveryBundleState, number> {
  return countProductionProofValues(
    bundles.map((bundle) => bundle.bundleState),
    [
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
    ] as const
  );
}

export function summarizeParentOwnedSyncExportRecoveryHandoffStates(
  bundles: ReadonlyArray<ParentOwnedSyncExportRecoveryBundle>
): Record<ParentOwnedSyncExportRecoveryHandoffState, number> {
  return countProductionProofValues(
    bundles.map((bundle) => bundle.handoff.handoffState),
    [
      'preview-only',
      'apply-pending',
      'applied',
      'partial-restore',
      'delete-pending',
      'delete-confirmed',
      'rejected',
      'manual-required',
    ] as const
  );
}

function item(
  dataClass: ParentOwnedSyncExportDataClass,
  exportFormat: ParentOwnedSyncExportFormat,
  retentionState: Infer<typeof ParentOwnedSyncExportRetentionStateSchema>
): ParentOwnedSyncExportItemDescriptor {
  return ParentOwnedSyncExportItemDescriptorSchema.parse({
    itemId: `item-${dataClass}`,
    dataClass,
    exportFormat,
    destinationOwnership: 'parent-owned-external-storage',
    schemaVersionLabel: `${dataClass}.v1`,
    encryption: {
      encryptionState:
        exportFormat === 'human-readable-parent-report' ? 'human-readable-parent-authorized' : 'encrypted-at-rest',
      keyOwner: 'parent-owned-external-storage',
      encryptionMetadataRef: `encryption-${dataClass}-ref`,
      proofRequirement: 'parent-owned key material or explicit human-readable parent action before export',
    },
    retention: {
      retentionState,
      retentionPolicyRef: `retention-${dataClass}-policy-ref`,
      deleteResultRef:
        retentionState === 'delete-requested' || retentionState === 'delete-confirmed'
          ? `delete-result-${dataClass}-ref`
          : null,
      parentActionRequired: true,
      auditRequired: true,
    },
    evidenceRefs: [EvidenceRef],
    manifestRefs: [ManifestId],
    parentActionRequired: true,
    rawChildEvidenceUploadedByDefault: false,
    ocentraHostedFamilyDataStored: false,
    transferRuntimeClaimed: false,
  });
}

function connector(
  provider: Infer<typeof ParentOwnedSyncExportConnectorProviderSchema>,
  status: ParentOwnedSyncExportConnectorStatus,
  accountRef: string | null,
  folderRef: string | null,
  revocationRef: string | null
): ParentOwnedSyncExportConnectorStatusRow {
  return ParentOwnedSyncExportConnectorStatusRowSchema.parse({
    connectorId: `connector-${provider}`,
    provider,
    status,
    destinationOwnership: provider === 'local-folder' ? 'parent-device-local' : 'parent-owned-external-storage',
    accountRef,
    folderRef,
    revocationRef,
    statusRef: `connector-status-${provider}-${status}`,
    lastCheckedAt: Timestamp,
    oauthRuntimeClaimed: false,
    uploadRuntimeClaimed: false,
    deleteRuntimeClaimed: false,
  });
}

function cursor(
  cursorState: ParentOwnedSyncExportSyncCursorState,
  cursorRef: string | null,
  batchRef: string | null,
  lastSuccessfulSyncAt: string | null,
  conflictRefs: readonly string[],
  retryQueueState: string | null
): ParentOwnedSyncExportSyncCursor {
  return ParentOwnedSyncExportSyncCursorSchema.parse({
    cursorState,
    cursorRef,
    batchRef,
    lastSuccessfulSyncAt,
    conflictRefs,
    retryQueueState,
  });
}

function conflict(
  conflictRef: string,
  dataClass: ParentOwnedSyncExportDataClass,
  resolution: ParentOwnedSyncExportConflictResolution,
  parentAction: typeof ParentAction | null,
  localVersionRef: string,
  parentStorageVersionRef: string | null
): ParentOwnedSyncExportConflictRecord {
  return ParentOwnedSyncExportConflictRecordSchema.parse({
    conflictRef,
    dataClass,
    resolution,
    parentAction,
    localVersionRef,
    parentStorageVersionRef,
    auditRefs: [EvidenceRef],
  });
}

function importResult(
  resultRef: string,
  resultState: ParentOwnedSyncExportImportResultState,
  acceptedSchemaVersion: string | null,
  rejectedReasonRef: string | null
): ParentOwnedSyncExportImportResult {
  return ParentOwnedSyncExportImportResultSchema.parse({
    resultRef,
    resultState,
    acceptedSchemaVersion,
    rejectedReasonRef,
    appliedToLocalEvidence: false,
    auditRefs: [EvidenceRef],
  });
}

function deleteResult(
  resultRef: string,
  resultState: ParentOwnedSyncExportDeleteResultState,
  dataClass: ParentOwnedSyncExportDataClass,
  deleteRequestRef: string | null
): ParentOwnedSyncExportDeleteResult {
  return ParentOwnedSyncExportDeleteResultSchema.parse({
    resultRef,
    resultState,
    dataClass,
    deleteRequestRef,
    connectorStatusRef: 'connector-status-delete-proof',
    auditRefs: [EvidenceRef],
  });
}

function handoff(
  handoffRef: string,
  handoffTarget: ParentOwnedSyncExportRecoveryHandoffTarget,
  handoffState: ParentOwnedSyncExportRecoveryHandoffState
): ParentOwnedSyncExportRecoveryHandoff {
  return ParentOwnedSyncExportRecoveryHandoffSchema.parse({
    handoffRef,
    handoffTarget,
    handoffState,
    previewIsNonMutating: true,
    explicitParentConfirmationRequired:
      handoffState === 'preview-only' ||
      handoffState === 'apply-pending' ||
      handoffState === 'applied' ||
      handoffState === 'partial-restore',
    sourceOfTruthPreserved: true,
    tombstonesPreserved: true,
    deleteRequestRequired: handoffState === 'delete-pending' || handoffState === 'delete-confirmed',
  });
}

function recoveryBundle(
  bundleRef: string,
  bundleType: ParentOwnedSyncExportRecoveryBundleType,
  bundleState: ParentOwnedSyncExportRecoveryBundleState,
  handoffEntry: ParentOwnedSyncExportRecoveryHandoff,
  sourceHouseholdBindingState: Infer<typeof ParentOwnedSyncExportRecoveryBundleBindingStateSchema>,
  sourceDeviceBindingState: Infer<typeof ParentOwnedSyncExportRecoveryBundleBindingStateSchema>,
  keyAvailabilityState: Infer<typeof ParentOwnedSyncExportRecoveryBundleKeyAvailabilityStateSchema>,
  acceptedDataClasses: ReadonlyArray<ParentOwnedSyncExportDataClass>,
  rejectedDataClasses: ReadonlyArray<ParentOwnedSyncExportDataClass>,
  rejectionReasonRef: string | null,
  deleteRequestRef: string | null,
  applyConfirmedByParent: boolean
): ParentOwnedSyncExportRecoveryBundle {
  return ParentOwnedSyncExportRecoveryBundleSchema.parse({
    bundleRef,
    manifestRef: ManifestId,
    bundleType,
    bundleState,
    handoff: handoffEntry,
    sourceHouseholdBindingState,
    sourceDeviceBindingState,
    keyAvailabilityState,
    acceptedDataClasses,
    rejectedDataClasses,
    rejectionReasonRef,
    deleteRequestRef,
    previewMutatedLocalTruth: false,
    applyConfirmedByParent,
    auditRefs: [EvidenceRef],
  });
}
