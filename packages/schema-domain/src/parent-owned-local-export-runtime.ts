import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  ParentOwnedSyncExportDataClassSchema,
  type ParentOwnedSyncExportDataClass,
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

const RequiredStates = RequiredParentOwnedLocalExportRuntimeStates;
const RuntimeClaimFlags = [
  'cloudTransferRuntimeClaimed',
  'connectorOAuthClaimed',
  'providerApiClaimed',
  'portalUiClaimed',
  'ocentraHostedFamilyDataCustodyClaimed',
  'remoteReportCompilerClaimed',
  'childDeviceMutationClaimed',
  'rawEvidenceUploadClaimed',
] as const;

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

type RuntimeScopeCandidate = Infer<typeof ParentOwnedLocalExportRuntimeScopeBaseSchema>;
type RuntimeOutputCandidate = Infer<typeof ParentOwnedLocalExportRuntimeOutputBaseSchema>;
type RuntimeDeleteReceiptCandidate = Infer<typeof ParentOwnedLocalExportRuntimeDeleteReceiptBaseSchema>;
type RuntimeJobCandidate = Infer<typeof ParentOwnedLocalExportRuntimeJobBaseSchema>;
type RuntimeProofCandidate = Infer<typeof ParentOwnedLocalExportRuntimeProofBaseSchema>;

export type ParentOwnedLocalExportRuntimeState = Infer<typeof ParentOwnedLocalExportRuntimeStateSchema>;
export type ParentOwnedLocalExportRuntimeNonClaim = Infer<typeof ParentOwnedLocalExportRuntimeNonClaimSchema>;
export type ParentOwnedLocalExportRuntimeScope = Infer<typeof ParentOwnedLocalExportRuntimeScopeSchema>;
export type ParentOwnedLocalExportRuntimeOutput = Infer<typeof ParentOwnedLocalExportRuntimeOutputSchema>;
export type ParentOwnedLocalExportRuntimeDeleteReceipt = Infer<typeof ParentOwnedLocalExportRuntimeDeleteReceiptSchema>;
export type ParentOwnedLocalExportRuntimeJob = Infer<typeof ParentOwnedLocalExportRuntimeJobSchema>;
export type ParentOwnedLocalExportRuntimeProof = Infer<typeof ParentOwnedLocalExportRuntimeProofSchema>;

function localExportRuntimeScopeIsSafe(scope: RuntimeScopeCandidate): boolean {
  return (
    scope.parentAuthorized &&
    !scope.rawEvidenceUploaded &&
    !scope.ocentraHostedFamilyDataStored &&
    scope.requestedDataClasses.length > 0 &&
    scope.destinationOwnership !== 'ocentra-hosted-non-activity-metadata'
  );
}

function localExportRuntimeOutputIsSafe(output: RuntimeOutputCandidate): boolean {
  if (output.destinationOwnership === 'ocentra-hosted-non-activity-metadata') {
    return false;
  }
  if (output.rawEvidenceIncludedByDefault || output.ocentraHostedCopyRetained) {
    return false;
  }
  if (output.outputFormat !== 'human-readable-parent-report' && !output.encryptedAtRest) {
    return false;
  }
  return output.sourceEvidenceRefs.length > 0 && output.childDetailMinimized;
}

function localExportRuntimeDeleteReceiptIsSafe(receipt: RuntimeDeleteReceiptCandidate): boolean {
  if (!receipt.localSafetyStatePreserved || receipt.sourceEvidenceRetained) {
    return false;
  }
  if (receipt.deleteConfirmed) {
    return receipt.deletedAt !== null && receipt.exportedOutputDeleted && receipt.auditState === 'audit-recorded';
  }
  return receipt.failureReasonRef !== null && !receipt.exportedOutputDeleted;
}

function localExportRuntimeJobIsSafe(job: RuntimeJobCandidate): boolean {
  if (job.localEvidenceMutated || job.parentOwnedOutputMutatedByFailure || !job.localSafetyStatePreserved) {
    return false;
  }
  if (job.operation === 'export') {
    return exportJobStateIsSafe(job);
  }
  return deleteJobStateIsSafe(job);
}

function exportJobStateIsSafe(job: RuntimeJobCandidate): boolean {
  if (job.state === 'export-written') {
    return job.output !== null && job.deleteReceipt === null && !job.manualActionRequired;
  }
  if (job.state === 'offline-queued' || job.state === 'manual-required') {
    return job.output === null && job.manualActionRequired;
  }
  if (job.state === 'export-queued' || job.state === 'export-running') {
    return job.output === null && job.deleteReceipt === null;
  }
  return false;
}

function deleteJobStateIsSafe(job: RuntimeJobCandidate): boolean {
  if (job.state === 'delete-requested') {
    return job.output !== null && job.deleteReceipt !== null && !job.deleteReceipt.deleteConfirmed;
  }
  if (job.state === 'delete-confirmed') {
    return job.output !== null && job.deleteReceipt !== null && job.deleteReceipt.deleteConfirmed;
  }
  if (job.state === 'delete-failed') {
    return job.output !== null && job.deleteReceipt !== null && !job.deleteReceipt.deleteConfirmed;
  }
  return false;
}

function localExportRuntimeProofIsSafe(proof: RuntimeProofCandidate): boolean {
  return (
    requiredStatesAreCovered(proof.jobs) &&
    RequiredParentOwnedLocalExportRuntimeNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    RuntimeClaimFlags.every((flag) => proof[flag] === false)
  );
}

function requiredStatesAreCovered(jobs: ReadonlyArray<RuntimeJobCandidate>): boolean {
  return RequiredStates.every((state) => jobs.some((job) => job.state === state));
}

const Timestamp = '2026-06-03T14:26:23.877Z';
const Family = { familyId: 'family-local-export-runtime-proof-1' } as const;
const Device = {
  deviceId: 'windows-child-device-local-export-proof-1',
  childProfileId: 'child-local-export-proof-1',
  label: 'Windows child device local export proof',
  platform: 'windows',
} as const;
const ParentAction = {
  actionReferenceId: 'parent-action-local-export-proof-1',
  actor: { actorId: 'parent-local-export-proof-1', role: 'parent' },
  policyVersion: 'parent-owned-local-export-runtime-v1',
  createdAt: Timestamp,
} as const;
const EvidenceRef = {
  evidenceReferenceId: 'evidence-local-export-runtime-proof-1',
  kind: 'journal-event',
  observedAt: Timestamp,
} as const;

const BaseScope = {
  family: Family,
  device: Device,
  parentAction: ParentAction,
  requestedDataClasses: ['encrypted-journal-segment', 'sqlite-query-row', 'generated-summary'],
  outputFormat: 'encrypted-machine-readable',
  destinationOwnership: 'parent-device-local',
  destinationRef: 'parent-local-export-folder-proof-1',
  parentAuthorized: true,
  rawEvidenceUploaded: false,
  ocentraHostedFamilyDataStored: false,
} as const;

const BaseOutput = {
  bundleRef: 'local-export-bundle-proof-1',
  outputRef: 'local-export-output-proof-1',
  outputFormat: 'encrypted-machine-readable',
  destinationOwnership: 'parent-device-local',
  encryptedAtRest: true,
  schemaVersionLabel: 'parent-owned-local-export-runtime-schema-v1',
  byteCountRange: 'support-safe-size-range-1',
  checksumRef: 'local-export-checksum-proof-1',
  createdAt: Timestamp,
  sourceEvidenceRefs: [EvidenceRef],
  childDetailMinimized: true,
  rawEvidenceIncludedByDefault: false,
  ocentraHostedCopyRetained: false,
} as const;

function jobFor(state: ParentOwnedLocalExportRuntimeState, overrides: Record<string, unknown> = {}) {
  return {
    jobId: `local-export-runtime-job-${state}`,
    operation: state.startsWith('delete') ? 'delete' : 'export',
    state,
    queueRef: `local-export-runtime-queue-${state}`,
    storageState: state === 'offline-queued' ? 'offline' : 'local-folder-ready',
    scope: BaseScope,
    output: state === 'export-written' || state.startsWith('delete') ? BaseOutput : null,
    deleteReceipt: deleteReceiptFor(state),
    queuedAt: Timestamp,
    updatedAt: Timestamp,
    auditRefs: [EvidenceRef],
    localEvidenceMutated: false,
    parentOwnedOutputMutatedByFailure: false,
    localSafetyStatePreserved: true,
    manualActionRequired: state === 'manual-required' || state === 'offline-queued',
    ...overrides,
  };
}

function deleteReceiptFor(state: ParentOwnedLocalExportRuntimeState) {
  if (!state.startsWith('delete')) {
    return null;
  }
  const deleteConfirmed = state === 'delete-confirmed';
  return {
    deleteRequestRef: `local-export-delete-request-${state}`,
    targetBundleRef: BaseOutput.bundleRef,
    requestedAt: Timestamp,
    deletedAt: deleteConfirmed ? Timestamp : null,
    deleteConfirmed,
    auditState: deleteConfirmed
      ? 'audit-recorded'
      : state === 'delete-failed'
        ? 'manual-audit-required'
        : 'audit-pending',
    sourceEvidenceRetained: false,
    exportedOutputDeleted: deleteConfirmed,
    localSafetyStatePreserved: true,
    failureReasonRef: deleteConfirmed ? null : `local-export-delete-failure-${state}`,
  };
}

export const ParentOwnedLocalExportRuntimeProofReadModel = ParentOwnedLocalExportRuntimeProofSchema.parse({
  schemaVersion: 'parent-owned-local-export-runtime-proof',
  jobs: RequiredStates.map((state) => jobFor(state)),
  nonClaims: RequiredParentOwnedLocalExportRuntimeNonClaims,
  cloudTransferRuntimeClaimed: false,
  connectorOAuthClaimed: false,
  providerApiClaimed: false,
  portalUiClaimed: false,
  ocentraHostedFamilyDataCustodyClaimed: false,
  remoteReportCompilerClaimed: false,
  childDeviceMutationClaimed: false,
  rawEvidenceUploadClaimed: false,
  updatedAt: Timestamp,
});

export function summarizeParentOwnedLocalExportRuntimeStates(
  jobs: ReadonlyArray<ParentOwnedLocalExportRuntimeJob>
): Record<ParentOwnedLocalExportRuntimeState, number> {
  return countBy(
    jobs.map((job) => job.state),
    RequiredStates
  );
}

export function summarizeParentOwnedLocalExportRuntimeDataClasses(
  jobs: ReadonlyArray<ParentOwnedLocalExportRuntimeJob>
): Record<ParentOwnedSyncExportDataClass, number> {
  const dataClasses = jobs.flatMap((job) => job.scope.requestedDataClasses);
  return countBy(dataClasses, [
    'encrypted-journal-segment',
    'sqlite-query-row',
    'parent-rule',
    'approval-decision',
    'device-registry-entry',
    'notification-history',
    'audit-event',
    'generated-summary',
  ] as const);
}

function countBy<const T extends string>(values: ReadonlyArray<T>, keys: readonly T[]): Record<T, number> {
  return Object.fromEntries(keys.map((key) => [key, values.filter((value) => value === key).length])) as Record<
    T,
    number
  >;
}
