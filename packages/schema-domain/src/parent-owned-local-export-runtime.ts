import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  ParentOwnedSyncExportDataClassSchema,
  ParentOwnedSyncExportDestinationOwnershipSchema,
  ParentOwnedSyncExportFormatSchema,
} from './parent-owned-sync-export';
import {
  RequiredParentOwnedLocalExportRuntimeNonClaims,
  RequiredParentOwnedLocalExportRuntimeStates,
} from './parent-owned-local-export-runtime-values';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  BaseOutput as ParentOwnedLocalExportRuntimeBaseOutput,
  BaseScope as ParentOwnedLocalExportRuntimeBaseScope,
  EvidenceRef as ParentOwnedLocalExportRuntimeEvidenceRef,
  ParentAction as ParentOwnedLocalExportRuntimeParentAction,
  ParentOwnedLocalExportRuntimeProofReadModelFixture,
  summarizeParentOwnedLocalExportRuntimeDataClasses as summarizeParentOwnedLocalExportRuntimeDataClassesHelper,
  summarizeParentOwnedLocalExportRuntimeStates as summarizeParentOwnedLocalExportRuntimeStatesHelper,
} from './parent-owned-local-export-runtime-fixtures';
import {
  localExportRuntimeDeleteReceiptIsSafe,
  localExportRuntimeOutputIsSafe,
  localExportRuntimeScopeIsSafe,
} from './parent-owned-local-export-runtime-scope-guards';
import { localExportRuntimeJobIsSafe } from './parent-owned-local-export-runtime-job-guards';
import { localExportRuntimeProofIsSafe } from './parent-owned-local-export-runtime-proof-guards';

export const ParentOwnedLocalExportRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('parent-owned-local-export-runtime-proof')
);
export const ParentOwnedLocalExportRuntimeStateSchema = withParser(
  Schema.Literal(...RequiredParentOwnedLocalExportRuntimeStates)
);
export const ParentOwnedLocalExportRuntimeNonClaimSchema = withParser(
  Schema.Literal(...RequiredParentOwnedLocalExportRuntimeNonClaims)
);
export const ParentOwnedLocalExportRuntimeOperationSchema = withParser(Schema.Literal('export', 'delete'));
export const ParentOwnedLocalExportRuntimeStorageStateSchema = withParser(
  Schema.Literal('local-folder-ready', 'local-folder-unavailable', 'offline', 'delete-target-missing')
);
export const ParentOwnedLocalExportRuntimeAuditStateSchema = withParser(
  Schema.Literal('audit-recorded', 'audit-pending', 'manual-audit-required')
);

const ParentOwnedLocalExportRuntimeJobIdSchema = brandedNonEmptyStringSchema('ParentOwnedLocalExportRuntimeJobId');
const ParentOwnedLocalExportRuntimeBundleRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedLocalExportRuntimeBundleRef'
);
const ParentOwnedLocalExportRuntimeOutputRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedLocalExportRuntimeOutputRef'
);
const ParentOwnedLocalExportRuntimeDeleteRequestRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedLocalExportRuntimeDeleteRequestRef'
);
const ParentOwnedLocalExportRuntimePolicyRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedLocalExportRuntimePolicyRef'
);
const ParentOwnedLocalExportRuntimeQueueRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedLocalExportRuntimeQueueRef'
);
const ParentOwnedLocalExportRuntimeStorageRefSchema = brandedNonEmptyStringSchema(
  'ParentOwnedLocalExportRuntimeStorageRef'
);

const ParentOwnedLocalExportRuntimeScopeBaseSchema = Schema.Struct({
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  parentAction: ParentActionReferenceSchema,
  requestedDataClasses: Schema.Array(ParentOwnedSyncExportDataClassSchema),
  outputFormat: ParentOwnedSyncExportFormatSchema,
  destinationOwnership: ParentOwnedSyncExportDestinationOwnershipSchema,
  destinationRef: ParentOwnedLocalExportRuntimeStorageRefSchema,
  parentAuthorized: Schema.Boolean,
  rawEvidenceUploaded: Schema.Boolean,
  ocentraHostedFamilyDataStored: Schema.Boolean,
});

export const ParentOwnedLocalExportRuntimeScopeSchema = withParser(
  ParentOwnedLocalExportRuntimeScopeBaseSchema.pipe(
    Schema.filter(
      (scope) =>
        localExportRuntimeScopeIsSafe(scope) ||
        'Expected local export runtime scope to be parent-authorized, local/parent-owned, and custody-safe'
    )
  )
);

const ParentOwnedLocalExportRuntimeOutputBaseSchema = Schema.Struct({
  bundleRef: ParentOwnedLocalExportRuntimeBundleRefSchema,
  outputRef: ParentOwnedLocalExportRuntimeOutputRefSchema,
  outputFormat: ParentOwnedSyncExportFormatSchema,
  destinationOwnership: ParentOwnedSyncExportDestinationOwnershipSchema,
  encryptedAtRest: Schema.Boolean,
  schemaVersionLabel: ParentOwnedLocalExportRuntimePolicyRefSchema,
  byteCountRange: ParentOwnedLocalExportRuntimePolicyRefSchema,
  checksumRef: ParentOwnedLocalExportRuntimePolicyRefSchema,
  createdAt: ParentTimestampSchema,
  sourceEvidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  childDetailMinimized: Schema.Boolean,
  rawEvidenceIncludedByDefault: Schema.Boolean,
  ocentraHostedCopyRetained: Schema.Boolean,
});

export const ParentOwnedLocalExportRuntimeOutputSchema = withParser(
  ParentOwnedLocalExportRuntimeOutputBaseSchema.pipe(
    Schema.filter(
      (output) =>
        localExportRuntimeOutputIsSafe(output) ||
        'Expected local export outputs to be encrypted/minimized, locally owned, and not retained by Ocentra'
    )
  )
);

const ParentOwnedLocalExportRuntimeDeleteReceiptBaseSchema = Schema.Struct({
  deleteRequestRef: ParentOwnedLocalExportRuntimeDeleteRequestRefSchema,
  targetBundleRef: ParentOwnedLocalExportRuntimeBundleRefSchema,
  requestedAt: ParentTimestampSchema,
  deletedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  deleteConfirmed: Schema.Boolean,
  auditState: ParentOwnedLocalExportRuntimeAuditStateSchema,
  sourceEvidenceRetained: Schema.Boolean,
  exportedOutputDeleted: Schema.Boolean,
  localSafetyStatePreserved: Schema.Boolean,
  failureReasonRef: Schema.Union(ParentOwnedLocalExportRuntimePolicyRefSchema, Schema.Null),
});

export const ParentOwnedLocalExportRuntimeDeleteReceiptSchema = withParser(
  ParentOwnedLocalExportRuntimeDeleteReceiptBaseSchema.pipe(
    Schema.filter(
      (receipt) =>
        localExportRuntimeDeleteReceiptIsSafe(receipt) ||
        'Expected local export delete receipts to preserve source safety data and expose delete confirmation or failure refs'
    )
  )
);

const ParentOwnedLocalExportRuntimeJobBaseSchema = Schema.Struct({
  jobId: ParentOwnedLocalExportRuntimeJobIdSchema,
  operation: ParentOwnedLocalExportRuntimeOperationSchema,
  state: ParentOwnedLocalExportRuntimeStateSchema,
  queueRef: ParentOwnedLocalExportRuntimeQueueRefSchema,
  storageState: ParentOwnedLocalExportRuntimeStorageStateSchema,
  scope: ParentOwnedLocalExportRuntimeScopeSchema,
  output: Schema.Union(ParentOwnedLocalExportRuntimeOutputSchema, Schema.Null),
  deleteReceipt: Schema.Union(ParentOwnedLocalExportRuntimeDeleteReceiptSchema, Schema.Null),
  queuedAt: ParentTimestampSchema,
  updatedAt: ParentTimestampSchema,
  auditRefs: Schema.Array(ParentEvidenceReferenceSchema),
  localEvidenceMutated: Schema.Boolean,
  parentOwnedOutputMutatedByFailure: Schema.Boolean,
  localSafetyStatePreserved: Schema.Boolean,
  manualActionRequired: Schema.Boolean,
});

export const ParentOwnedLocalExportRuntimeJobSchema = withParser(
  ParentOwnedLocalExportRuntimeJobBaseSchema.pipe(
    Schema.filter(
      (job) =>
        localExportRuntimeJobIsSafe(job) ||
        'Expected local export/delete runtime jobs to expose state, output/delete refs, and non-mutating failure behavior'
    )
  )
);

const ParentOwnedLocalExportRuntimeProofBaseSchema = Schema.Struct({
  schemaVersion: ParentOwnedLocalExportRuntimeSchemaVersionSchema,
  jobs: Schema.Array(ParentOwnedLocalExportRuntimeJobSchema),
  nonClaims: Schema.Array(ParentOwnedLocalExportRuntimeNonClaimSchema),
  cloudTransferRuntimeClaimed: Schema.Boolean,
  connectorOAuthClaimed: Schema.Boolean,
  providerApiClaimed: Schema.Boolean,
  portalUiClaimed: Schema.Boolean,
  ocentraHostedFamilyDataCustodyClaimed: Schema.Boolean,
  remoteReportCompilerClaimed: Schema.Boolean,
  childDeviceMutationClaimed: Schema.Boolean,
  rawEvidenceUploadClaimed: Schema.Boolean,
  updatedAt: ParentTimestampSchema,
});

export const ParentOwnedLocalExportRuntimeProofSchema = withParser(
  ParentOwnedLocalExportRuntimeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localExportRuntimeProofIsSafe(proof) ||
        'Expected local export/delete runtime proof to cover all states and keep cloud/provider/UI/custody non-claims explicit'
    )
  )
);

export type ParentOwnedLocalExportRuntimeState = Infer<typeof ParentOwnedLocalExportRuntimeStateSchema>;
export type ParentOwnedLocalExportRuntimeNonClaim = Infer<typeof ParentOwnedLocalExportRuntimeNonClaimSchema>;
export type ParentOwnedLocalExportRuntimeScope = Infer<typeof ParentOwnedLocalExportRuntimeScopeSchema>;
export type ParentOwnedLocalExportRuntimeOutput = Infer<typeof ParentOwnedLocalExportRuntimeOutputSchema>;
export type ParentOwnedLocalExportRuntimeDeleteReceipt = Infer<typeof ParentOwnedLocalExportRuntimeDeleteReceiptSchema>;
export type ParentOwnedLocalExportRuntimeJob = Infer<typeof ParentOwnedLocalExportRuntimeJobSchema>;
export type ParentOwnedLocalExportRuntimeProof = Infer<typeof ParentOwnedLocalExportRuntimeProofSchema>;

export const BaseOutput = ParentOwnedLocalExportRuntimeBaseOutput;
export const BaseScope = ParentOwnedLocalExportRuntimeBaseScope;
export const EvidenceRef = ParentOwnedLocalExportRuntimeEvidenceRef;
export const ParentAction = ParentOwnedLocalExportRuntimeParentAction;
export const ParentOwnedLocalExportRuntimeProofReadModel = ParentOwnedLocalExportRuntimeProofSchema.parse(
  ParentOwnedLocalExportRuntimeProofReadModelFixture
);
export const summarizeParentOwnedLocalExportRuntimeStates = summarizeParentOwnedLocalExportRuntimeStatesHelper;
export const summarizeParentOwnedLocalExportRuntimeDataClasses = summarizeParentOwnedLocalExportRuntimeDataClassesHelper;
