/* thin adapter over Rust-generated parent-owned sync export contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { countProductionProofValues } from './proof-shape';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import type {
  ChildProfileId as ChildProfileIdType,
  FamilyId as FamilyIdType,
  ParentActionReferenceId as ParentActionReferenceIdType,
  ParentActorId as ParentActorIdType,
  ParentDeviceId as ParentDeviceIdType,
  ParentDeviceLabel as ParentDeviceLabelType,
  ParentEvidenceReferenceId as ParentEvidenceReferenceIdType,
  ParentPolicyVersion as ParentPolicyVersionType,
  ParentTimestamp as ParentTimestampType,
} from './family-reference-primitives';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  GeneratedParentOwnedSyncDeleteVisibilityStates,
  GeneratedParentOwnedSyncDisconnectVisibilityStates,
  GeneratedParentOwnedSyncExportContractProof,
  GeneratedParentOwnedSyncExportDataClasses,
  GeneratedParentOwnedSyncExportDestinationOwnerships,
  GeneratedParentOwnedSyncExportEncryptionStates,
  GeneratedParentOwnedSyncExportFormats,
  GeneratedParentOwnedSyncExportKnownGaps,
  GeneratedParentOwnedSyncExportNonClaims,
  type GeneratedParentOwnedSyncExportManifestItem,
  GeneratedParentOwnedSyncManifestIntegrityStates,
  GeneratedParentOwnedSyncProviderModes,
  GeneratedParentOwnedSyncProviderStatuses,
  GeneratedParentOwnedSyncStates,
  GeneratedParentOwnedSyncTombstonePropagationStates,
  ParentOwnedSyncExportContractRuntime,
  type GeneratedParentOwnedSyncExportContractProof as GeneratedParentOwnedSyncExportContractProofShape,
  type GeneratedParentOwnedSyncProviderStatusRow,
  type GeneratedParentOwnedSyncStateRow,
  type GeneratedParentOwnedSyncTombstoneRow,
} from './generated/parent-owned-sync-export-contracts';
import {
  syncExportContractProofIsHonestGenerated,
  syncExportCoversRequiredDataClassesGenerated,
  syncExportManifestItemIsHonestGenerated,
  syncExportProviderStatusRowIsHonestGenerated,
  syncExportSyncStateRowIsHonestGenerated,
  syncExportTombstoneRowIsHonestGenerated,
} from './generated/parent-owned-sync-export-contract-rules';

export const ParentOwnedSyncExportSchemaVersionSchema = withParser(
  Schema.Literal(ParentOwnedSyncExportContractRuntime.SchemaVersion)
);

export const ParentOwnedSyncExportDataClassSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncExportDataClasses)
);
export const ParentOwnedSyncExportFormatSchema = withParser(Schema.Literal(...GeneratedParentOwnedSyncExportFormats));
export const ParentOwnedSyncExportDestinationOwnershipSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncExportDestinationOwnerships)
);
export const ParentOwnedSyncExportEncryptionStateSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncExportEncryptionStates)
);
export const ParentOwnedSyncExportProviderModeSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncProviderModes)
);
export const ParentOwnedSyncExportProviderStatusSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncProviderStatuses)
);
export const ParentOwnedSyncExportManifestIntegrityStateSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncManifestIntegrityStates)
);
export const ParentOwnedSyncExportSyncStateSchema = withParser(Schema.Literal(...GeneratedParentOwnedSyncStates));
export const ParentOwnedSyncExportTombstonePropagationStateSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncTombstonePropagationStates)
);
export const ParentOwnedSyncExportDisconnectVisibilityStateSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncDisconnectVisibilityStates)
);
export const ParentOwnedSyncExportDeleteVisibilityStateSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncDeleteVisibilityStates)
);
export const ParentOwnedSyncExportNonClaimSchema = withParser(
  Schema.Literal(...GeneratedParentOwnedSyncExportNonClaims)
);

const ParentOwnedSyncManifestIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncManifestId');
const ParentOwnedSyncItemIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncItemId');
const ParentOwnedSyncVersionLabelSchema = brandedNonEmptyStringSchema('ParentOwnedSyncVersionLabel');
const ParentOwnedSyncPolicyRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncPolicyRef');
const ParentOwnedSyncProviderIdSchema = brandedNonEmptyStringSchema('ParentOwnedSyncProviderId');
const ParentOwnedSyncProviderRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncProviderRef');
const ParentOwnedSyncStatusRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncStatusRef');
const ParentOwnedSyncCursorRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncCursorRef');
const ParentOwnedSyncBatchRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncBatchRef');
const ParentOwnedSyncConflictRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncConflictRef');
const ParentOwnedSyncChecksumRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncChecksumRef');
const ParentOwnedSyncSignatureRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncSignatureRef');
const ParentOwnedSyncTombstoneRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncTombstoneRef');
const ParentOwnedSyncDeleteRequestRefSchema = brandedNonEmptyStringSchema('ParentOwnedSyncDeleteRequestRef');

export type FamilyId = FamilyIdType;
export type ChildProfileId = ChildProfileIdType;
export type ParentDeviceId = ParentDeviceIdType;
export type ParentDeviceLabel = ParentDeviceLabelType;
export type ParentActorId = ParentActorIdType;
export type ParentPolicyVersion = ParentPolicyVersionType;
export type ParentEvidenceReferenceId = ParentEvidenceReferenceIdType;
export type ParentActionReferenceId = ParentActionReferenceIdType;
export type ParentTimestamp = ParentTimestampType;
export type ParentOwnedSyncManifestId = typeof ParentOwnedSyncManifestIdSchema.Type;
export type ParentOwnedSyncItemId = typeof ParentOwnedSyncItemIdSchema.Type;
export type ParentOwnedSyncVersionLabel = typeof ParentOwnedSyncVersionLabelSchema.Type;
export type ParentOwnedSyncPolicyRef = typeof ParentOwnedSyncPolicyRefSchema.Type;
export type ParentOwnedSyncProviderId = typeof ParentOwnedSyncProviderIdSchema.Type;
export type ParentOwnedSyncProviderRef = typeof ParentOwnedSyncProviderRefSchema.Type;
export type ParentOwnedSyncStatusRef = typeof ParentOwnedSyncStatusRefSchema.Type;
export type ParentOwnedSyncCursorRef = typeof ParentOwnedSyncCursorRefSchema.Type;
export type ParentOwnedSyncBatchRef = typeof ParentOwnedSyncBatchRefSchema.Type;
export type ParentOwnedSyncConflictRef = typeof ParentOwnedSyncConflictRefSchema.Type;
export type ParentOwnedSyncChecksumRef = typeof ParentOwnedSyncChecksumRefSchema.Type;
export type ParentOwnedSyncSignatureRef = typeof ParentOwnedSyncSignatureRefSchema.Type;
export type ParentOwnedSyncTombstoneRef = typeof ParentOwnedSyncTombstoneRefSchema.Type;
export type ParentOwnedSyncDeleteRequestRef = typeof ParentOwnedSyncDeleteRequestRefSchema.Type;

const ParentOwnedSyncExportEncryptionMetadataBaseSchema = Schema.Struct({
  encryptionState: ParentOwnedSyncExportEncryptionStateSchema,
  encryptedBeforeUpload: Schema.Boolean,
  keyOwner: ParentOwnedSyncExportDestinationOwnershipSchema,
  proofRequirementRef: ParentOwnedSyncPolicyRefSchema,
});

export const ParentOwnedSyncExportEncryptionMetadataSchema = withParser(
  ParentOwnedSyncExportEncryptionMetadataBaseSchema
);

const ParentOwnedSyncExportManifestItemBaseSchema = Schema.Struct({
  itemId: ParentOwnedSyncItemIdSchema,
  dataClass: ParentOwnedSyncExportDataClassSchema,
  exportFormat: ParentOwnedSyncExportFormatSchema,
  destinationOwnership: ParentOwnedSyncExportDestinationOwnershipSchema,
  schemaVersionLabel: ParentOwnedSyncVersionLabelSchema,
  encryption: ParentOwnedSyncExportEncryptionMetadataSchema,
  parentActionRequired: Schema.Boolean,
  rawChildEvidenceUploadedByDefault: Schema.Boolean,
  ocentraHostedFamilyDataStored: Schema.Boolean,
  claimSafe: Schema.Boolean,
});

export const ParentOwnedSyncExportManifestItemSchema = withParser(
  ParentOwnedSyncExportManifestItemBaseSchema.pipe(
    Schema.filter(
      (item) =>
        syncExportManifestItemIsHonestGenerated(item as GeneratedParentOwnedSyncExportManifestItem) ||
        'Expected manifest items to stay encrypted-before-upload or explicitly human-readable, claim-safe, and free of hosted evidence storage'
    )
  )
);

const ParentOwnedSyncExportManifestBaseSchema = Schema.Struct({
  schemaVersion: ParentOwnedSyncExportSchemaVersionSchema,
  manifestId: ParentOwnedSyncManifestIdSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  parentAction: ParentActionReferenceSchema,
  productVersion: ParentOwnedSyncVersionLabelSchema,
  manifestVersion: ParentOwnedSyncVersionLabelSchema,
  generatedAt: ParentTimestampSchema,
  items: Schema.Array(ParentOwnedSyncExportManifestItemSchema),
});

export const ParentOwnedSyncExportManifestSchema = withParser(
  ParentOwnedSyncExportManifestBaseSchema.pipe(
    Schema.filter(
      (manifest) =>
        syncExportCoversRequiredDataClassesGenerated(
          manifest.items as ReadonlyArray<GeneratedParentOwnedSyncExportManifestItem>
        ) ||
        'Expected the parent-owned sync manifest to cover every required data class'
    )
  )
);

const ParentOwnedSyncExportProviderStatusRowBaseSchema = Schema.Struct({
  providerId: ParentOwnedSyncProviderIdSchema,
  providerMode: ParentOwnedSyncExportProviderModeSchema,
  providerStatus: ParentOwnedSyncExportProviderStatusSchema,
  destinationOwnership: ParentOwnedSyncExportDestinationOwnershipSchema,
  accountRef: Schema.Union(ParentOwnedSyncProviderRefSchema, Schema.Null),
  folderRef: Schema.Union(ParentOwnedSyncProviderRefSchema, Schema.Null),
  statusRef: ParentOwnedSyncStatusRefSchema,
  revocationRef: Schema.Union(ParentOwnedSyncProviderRefSchema, Schema.Null),
  disconnectVisibilityState: ParentOwnedSyncExportDisconnectVisibilityStateSchema,
  deleteVisibilityState: ParentOwnedSyncExportDeleteVisibilityStateSchema,
  lastCheckedAt: ParentTimestampSchema,
  oauthRuntimeClaimed: Schema.Boolean,
  uploadRuntimeClaimed: Schema.Boolean,
  deleteRuntimeClaimed: Schema.Boolean,
  ocentraHostedFamilyDataStored: Schema.Boolean,
  claimSafe: Schema.Boolean,
});

export const ParentOwnedSyncExportProviderStatusRowSchema = withParser(
  ParentOwnedSyncExportProviderStatusRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        syncExportProviderStatusRowIsHonestGenerated(row as GeneratedParentOwnedSyncProviderStatusRow) ||
        'Expected provider status rows to keep mode split, claim-safe status, and delete or disconnect visibility explicit without runtime overclaim'
    )
  )
);

const ParentOwnedSyncExportSyncStateRowBaseSchema = Schema.Struct({
  syncState: ParentOwnedSyncExportSyncStateSchema,
  providerStatusRef: ParentOwnedSyncStatusRefSchema,
  cursorRef: Schema.Union(ParentOwnedSyncCursorRefSchema, Schema.Null),
  batchRef: Schema.Union(ParentOwnedSyncBatchRefSchema, Schema.Null),
  manifestIntegrityState: ParentOwnedSyncExportManifestIntegrityStateSchema,
  manifestChecksumRef: Schema.Union(ParentOwnedSyncChecksumRefSchema, Schema.Null),
  manifestSignatureRef: Schema.Union(ParentOwnedSyncSignatureRefSchema, Schema.Null),
  lastSuccessfulSyncAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  conflictRef: Schema.Union(ParentOwnedSyncConflictRefSchema, Schema.Null),
  retryQueueRef: Schema.Union(ParentOwnedSyncPolicyRefSchema, Schema.Null),
  parentActionRequired: Schema.Boolean,
  claimSafe: Schema.Boolean,
});

export const ParentOwnedSyncExportSyncStateRowSchema = withParser(
  ParentOwnedSyncExportSyncStateRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        syncExportSyncStateRowIsHonestGenerated(row as GeneratedParentOwnedSyncStateRow) ||
        'Expected sync states to keep checksum, signature, conflict, retry, corruption, and manual-required boundaries explicit'
    )
  )
);

const ParentOwnedSyncExportTombstoneRowBaseSchema = Schema.Struct({
  tombstoneRef: ParentOwnedSyncTombstoneRefSchema,
  dataClass: ParentOwnedSyncExportDataClassSchema,
  propagationState: ParentOwnedSyncExportTombstonePropagationStateSchema,
  deleteRequestRef: Schema.Union(ParentOwnedSyncDeleteRequestRefSchema, Schema.Null),
  providerStatusRef: ParentOwnedSyncStatusRefSchema,
  lastPropagatedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  blockedReasonRef: Schema.Union(ParentOwnedSyncPolicyRefSchema, Schema.Null),
  claimSafe: Schema.Boolean,
});

export const ParentOwnedSyncExportTombstoneRowSchema = withParser(
  ParentOwnedSyncExportTombstoneRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        syncExportTombstoneRowIsHonestGenerated(row as GeneratedParentOwnedSyncTombstoneRow) ||
        'Expected tombstone propagation to stay explicit and separate from sync success or provider status'
    )
  )
);

const ParentOwnedSyncExportContractProofBaseSchema = Schema.Struct({
  schemaVersion: ParentOwnedSyncExportSchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  manifest: ParentOwnedSyncExportManifestSchema,
  providerStatuses: Schema.Array(ParentOwnedSyncExportProviderStatusRowSchema),
  syncStates: Schema.Array(ParentOwnedSyncExportSyncStateRowSchema),
  tombstones: Schema.Array(ParentOwnedSyncExportTombstoneRowSchema),
  nonClaims: Schema.Array(ParentOwnedSyncExportNonClaimSchema),
  transferRuntimeClaimed: Schema.Boolean,
  connectorOAuthClaimed: Schema.Boolean,
  uploadRuntimeClaimed: Schema.Boolean,
  deleteRuntimeClaimed: Schema.Boolean,
  ocentraHostedChildEvidenceStored: Schema.Boolean,
  updatedAt: ParentTimestampSchema,
});

export const ParentOwnedSyncExportContractProofSchema = withParser(
  ParentOwnedSyncExportContractProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        syncExportContractProofIsHonestGenerated(proof as GeneratedParentOwnedSyncExportContractProofShape) ||
        'Expected parent-owned sync proof to keep provider, sync, manifest integrity, tombstone, and non-claim boundaries honest'
    )
  )
);

export type ParentOwnedSyncExportDataClass = Infer<typeof ParentOwnedSyncExportDataClassSchema>;
export type ParentOwnedSyncExportFormat = Infer<typeof ParentOwnedSyncExportFormatSchema>;
export type ParentOwnedSyncExportDestinationOwnership = Infer<
  typeof ParentOwnedSyncExportDestinationOwnershipSchema
>;
export type ParentOwnedSyncExportProviderMode = Infer<typeof ParentOwnedSyncExportProviderModeSchema>;
export type ParentOwnedSyncExportProviderStatus = Infer<typeof ParentOwnedSyncExportProviderStatusSchema>;
export type ParentOwnedSyncExportSyncState = Infer<typeof ParentOwnedSyncExportSyncStateSchema>;
export type ParentOwnedSyncExportTombstonePropagationState = Infer<
  typeof ParentOwnedSyncExportTombstonePropagationStateSchema
>;
export type ParentOwnedSyncExportNonClaim = Infer<typeof ParentOwnedSyncExportNonClaimSchema>;
export type ParentOwnedSyncExportManifestItem = GeneratedParentOwnedSyncExportManifestItem;
export type ParentOwnedSyncExportProviderStatusRow = GeneratedParentOwnedSyncProviderStatusRow;
export type ParentOwnedSyncExportSyncStateRow = GeneratedParentOwnedSyncStateRow;
export type ParentOwnedSyncExportTombstoneRow = GeneratedParentOwnedSyncTombstoneRow;
export type ParentOwnedSyncExportContractProofCandidate = GeneratedParentOwnedSyncExportContractProofShape;
export type ParentOwnedSyncExportContractProof = GeneratedParentOwnedSyncExportContractProofShape;

export const RequiredParentOwnedSyncExportProviderModes = [...GeneratedParentOwnedSyncProviderModes] as const;
export const RequiredParentOwnedSyncExportProviderStatuses = [...GeneratedParentOwnedSyncProviderStatuses] as const;
export const RequiredParentOwnedSyncExportSyncStates = [...GeneratedParentOwnedSyncStates] as const;
export const RequiredParentOwnedSyncExportTombstoneStates = [
  ...GeneratedParentOwnedSyncTombstonePropagationStates,
] as const;
export const ParentOwnedSyncExportKnownGaps = [...GeneratedParentOwnedSyncExportKnownGaps] as const;

export const ParentOwnedSyncExportContractProofReadModel = ParentOwnedSyncExportContractProofSchema.parse(
  GeneratedParentOwnedSyncExportContractProof
);

export function summarizeParentOwnedSyncExportDataClasses(
  items: ReadonlyArray<ParentOwnedSyncExportManifestItem>
): Record<ParentOwnedSyncExportDataClass, number> {
  return countProductionProofValues(items.map((item) => item.dataClass), GeneratedParentOwnedSyncExportDataClasses);
}

export function summarizeParentOwnedSyncExportProviderModes(
  rows: ReadonlyArray<ParentOwnedSyncExportProviderStatusRow>
): Record<ParentOwnedSyncExportProviderMode, number> {
  return countProductionProofValues(rows.map((row) => row.providerMode), GeneratedParentOwnedSyncProviderModes);
}

export function summarizeParentOwnedSyncExportProviderStatuses(
  rows: ReadonlyArray<ParentOwnedSyncExportProviderStatusRow>
): Record<ParentOwnedSyncExportProviderStatus, number> {
  return countProductionProofValues(rows.map((row) => row.providerStatus), GeneratedParentOwnedSyncProviderStatuses);
}

export function summarizeParentOwnedSyncExportSyncStates(
  rows: ReadonlyArray<ParentOwnedSyncExportSyncStateRow>
): Record<ParentOwnedSyncExportSyncState, number> {
  return countProductionProofValues(rows.map((row) => row.syncState), GeneratedParentOwnedSyncStates);
}

export function summarizeParentOwnedSyncExportTombstoneStates(
  rows: ReadonlyArray<ParentOwnedSyncExportTombstoneRow>
): Record<ParentOwnedSyncExportTombstonePropagationState, number> {
  return countProductionProofValues(
    rows.map((row) => row.propagationState),
    GeneratedParentOwnedSyncTombstonePropagationStates
  );
}
