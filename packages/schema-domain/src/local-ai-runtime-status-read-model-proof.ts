import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  LocalAiRuntimeProviderProofReadModel,
  LocalAiRuntimeProviderProofReadModelSchema,
  type LocalAiRuntimeProviderProofEntry,
} from './local-ai-runtime-provider-proof';
import { LocalAiProviderSchedulerLifecycleSchema } from './local-ai-provider-scheduler';
import {
  LocalAiDegradedStateSchema,
  LocalAiModelIdSchema,
  LocalAiModelReferenceSchema,
  LocalAiProviderIdSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
  LocalAiUnavailableReasonSchema,
} from './ai-primitives';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
const RuntimeStatusRowCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiRuntimeStatusSurfaceReadModelIdSchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeStatusSurfaceReadModelId'
);
export const LocalAiRuntimeStatusSurfaceRowIdSchema = brandedNonEmptyStringSchema('LocalAiRuntimeStatusSurfaceRowId');
export const LocalAiRuntimeStatusSurfaceSourceRefSchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeStatusSurfaceSourceRef'
);
export const LocalAiRuntimeStatusSurfaceLabelSchema = brandedNonEmptyStringSchema('LocalAiRuntimeStatusSurfaceLabel');
export const LocalAiRuntimeStatusSurfaceNonClaimSchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeStatusSurfaceNonClaim'
);

export const LocalAiRuntimeStatusSurfaceStateSchema = withParser(
  Schema.Literal('ready-visible', 'queued-visible', 'degraded-visible', 'unavailable-visible', 'manual-setup-required')
);

const LocalAiRuntimeStatusSurfaceRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: LocalAiRuntimeStatusSurfaceRowIdSchema,
  sourceRuntimeProviderProofEntryId: LocalAiRuntimeStatusSurfaceSourceRefSchema,
  providerId: LocalAiProviderIdSchema,
  runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
  modelId: LocalAiModelIdSchema,
  modelReference: LocalAiModelReferenceSchema,
  schedulerLifecycle: LocalAiProviderSchedulerLifecycleSchema,
  surfaceState: LocalAiRuntimeStatusSurfaceStateSchema,
  parentVisible: Schema.Boolean,
  childSafetyPriorityVisible: Schema.Boolean,
  queueDepth: RuntimeStatusRowCountSchema,
  degradedState: LocalAiDegradedStateSchema,
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  statusLabel: LocalAiRuntimeStatusSurfaceLabelSchema,
  statusDetail: LocalAiRuntimeStatusSurfaceLabelSchema,
  lastCheckedAt: LocalAiTimestampSchema,
  portalRuntimeRenderingClaimed: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type LocalAiRuntimeStatusSurfaceRowCandidate = Infer<typeof LocalAiRuntimeStatusSurfaceRowBaseSchema>;

export const LocalAiRuntimeStatusSurfaceRowSchema = withParser(
  LocalAiRuntimeStatusSurfaceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        localAiRuntimeStatusSurfaceRowIsHonest(row) ||
        'Expected local AI runtime status surface rows to stay parent-visible without claiming portal runtime UI, remote AI, policy authority, or enforcement'
    )
  )
);

const LocalAiRuntimeStatusSurfaceReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelId: LocalAiRuntimeStatusSurfaceReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceReadModelIds: Schema.Array(LocalAiRuntimeStatusSurfaceSourceRefSchema),
  rows: Schema.Array(LocalAiRuntimeStatusSurfaceRowSchema),
  readyVisibleCount: RuntimeStatusRowCountSchema,
  queuedVisibleCount: RuntimeStatusRowCountSchema,
  degradedVisibleCount: RuntimeStatusRowCountSchema,
  unavailableVisibleCount: RuntimeStatusRowCountSchema,
  manualSetupRequiredCount: RuntimeStatusRowCountSchema,
  runtimeStatusNonClaims: Schema.Array(LocalAiRuntimeStatusSurfaceNonClaimSchema),
});

type LocalAiRuntimeStatusSurfaceReadModelCandidate = Infer<typeof LocalAiRuntimeStatusSurfaceReadModelBaseSchema>;

export const LocalAiRuntimeStatusSurfaceReadModelSchema = withParser(
  LocalAiRuntimeStatusSurfaceReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        localAiRuntimeStatusSurfaceReadModelCountsMatch(readModel) ||
        'Expected local AI runtime status surface read model counts to match visible runtime state rows'
    )
  )
);

export type LocalAiRuntimeStatusSurfaceState = Infer<typeof LocalAiRuntimeStatusSurfaceStateSchema>;
export type LocalAiRuntimeStatusSurfaceRow = Infer<typeof LocalAiRuntimeStatusSurfaceRowSchema>;
export type LocalAiRuntimeStatusSurfaceReadModel = Infer<typeof LocalAiRuntimeStatusSurfaceReadModelSchema>;

const decodeLocalAiRuntimeStatusSurfaceNonClaim = Schema.decodeUnknownSync(LocalAiRuntimeStatusSurfaceNonClaimSchema);

export const LocalAiRuntimeStatusSurfaceNonClaims = [
  decodeLocalAiRuntimeStatusSurfaceNonClaim(
    'This read model projects local runtime status into parent-facing rows without rendering the production portal.'
  ),
  decodeLocalAiRuntimeStatusSurfaceNonClaim(
    'This proof does not execute a model, prove model quality, use remote/API AI, grant policy authority, or dispatch enforcement.'
  ),
  decodeLocalAiRuntimeStatusSurfaceNonClaim(
    'Unavailable and manual rows are visible as setup/status states; they are not promoted to working local AI.'
  ),
] as const;

export const LocalAiRuntimeStatusSurfaceReadModel = buildLocalAiRuntimeStatusSurfaceReadModel(
  LocalAiRuntimeProviderProofReadModel
);

export function buildLocalAiRuntimeStatusSurfaceReadModel(
  sourceReadModel: typeof LocalAiRuntimeProviderProofReadModel
): LocalAiRuntimeStatusSurfaceReadModel {
  const parsedSource = LocalAiRuntimeProviderProofReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.entries.map(runtimeStatusSurfaceRowFromProviderEntry);

  return LocalAiRuntimeStatusSurfaceReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'local-ai-runtime-status-surface-read-model',
    generatedAt: parsedSource.generatedAt,
    sourceReadModelIds: [parsedSource.readModelId],
    rows,
    readyVisibleCount: countRows(rows, 'ready-visible'),
    queuedVisibleCount: countRows(rows, 'queued-visible'),
    degradedVisibleCount: countRows(rows, 'degraded-visible'),
    unavailableVisibleCount: countRows(rows, 'unavailable-visible'),
    manualSetupRequiredCount: countRows(rows, 'manual-setup-required'),
    runtimeStatusNonClaims: LocalAiRuntimeStatusSurfaceNonClaims,
  });
}

const runtimeStatusUnavailableSurfaceStates = {
  proved: 'manual-setup-required',
  unavailable: 'unavailable-visible',
  'not-claimed': 'manual-setup-required',
  implemented: 'manual-setup-required',
  degraded: 'manual-setup-required',
  'manual-required': 'manual-setup-required',
} as const;

function runtimeStatusSurfaceRowFromProviderEntry(
  entry: LocalAiRuntimeProviderProofEntry
): LocalAiRuntimeStatusSurfaceRow {
  return LocalAiRuntimeStatusSurfaceRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    rowId: `local-ai-runtime-status:${entry.proofEntryId}`,
    sourceRuntimeProviderProofEntryId: entry.proofEntryId,
    providerId: entry.providerId,
    runtimeReferenceId: entry.runtimeReferenceId,
    modelId: entry.modelId,
    modelReference: entry.modelReference,
    schedulerLifecycle: entry.schedulerLifecycle,
    surfaceState: runtimeStatusSurfaceStateFor(entry),
    parentVisible: true,
    childSafetyPriorityVisible: entry.childSafetyPriorityProved,
    queueDepth: entry.queue.childSafetyQueued + entry.queue.parentAssistantQueued + entry.queue.parentReportQueued,
    degradedState: entry.degradedState,
    unavailableReason: entry.unavailableReason,
    statusLabel: runtimeStatusLabelFor(entry),
    statusDetail: entry.fallbackBehavior,
    lastCheckedAt: entry.lastCheckedAt,
    portalRuntimeRenderingClaimed: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
  });
}

function runtimeStatusSurfaceStateFor(entry: LocalAiRuntimeProviderProofEntry): LocalAiRuntimeStatusSurfaceState {
  if (entry.schedulerLifecycle === 'unavailable') {
    return runtimeStatusUnavailableSurfaceStates[entry.proofStatus];
  }

  if (entry.proofStatus === 'unavailable') {
    return 'manual-setup-required';
  }

  if (entry.schedulerLifecycle === 'degraded') {
    return 'degraded-visible';
  }

  if (entry.schedulerLifecycle === 'queued') {
    return 'queued-visible';
  }

  return 'ready-visible';
}

const runtimeStatusLabels = {
  'ready-visible': 'Local AI runtime ready',
  'queued-visible': 'Local AI runtime queued',
  'degraded-visible': 'Local AI runtime degraded',
  'unavailable-visible': 'Local AI runtime unavailable',
  'manual-setup-required': 'Local AI runtime needs setup',
} as const satisfies Record<LocalAiRuntimeStatusSurfaceState, string>;

function runtimeStatusLabelFor(entry: LocalAiRuntimeProviderProofEntry): string {
  return runtimeStatusLabels[runtimeStatusSurfaceStateFor(entry)];
}

function localAiRuntimeStatusSurfaceRowIsHonest(row: LocalAiRuntimeStatusSurfaceRowCandidate): boolean {
  if (!runtimeStatusSurfaceRowKeepsClaimBoundary(row)) {
    return false;
  }

  return runtimeStatusSurfaceStateIsHonest(row);
}

function runtimeStatusSurfaceRowKeepsClaimBoundary(row: LocalAiRuntimeStatusSurfaceRowCandidate): boolean {
  return (
    row.parentVisible &&
    !row.portalRuntimeRenderingClaimed &&
    !row.remoteApiClaimed &&
    !row.policyAuthorityClaimed &&
    !row.enforcementClaimed
  );
}

const runtimeStatusSurfaceStateValidators = {
  'unavailable-visible': (row: LocalAiRuntimeStatusSurfaceRowCandidate) => row.unavailableReason !== null,
  'manual-setup-required': (row: LocalAiRuntimeStatusSurfaceRowCandidate) => row.unavailableReason !== null,
  'degraded-visible': (row: LocalAiRuntimeStatusSurfaceRowCandidate) =>
    row.degradedState !== 'none' && row.unavailableReason === null,
  'queued-visible': (row: LocalAiRuntimeStatusSurfaceRowCandidate) =>
    row.queueDepth > 0 && row.unavailableReason === null,
  'ready-visible': (row: LocalAiRuntimeStatusSurfaceRowCandidate) =>
    row.queueDepth === 0 && row.degradedState === 'none' && row.unavailableReason === null,
} satisfies Record<
  LocalAiRuntimeStatusSurfaceState,
  (row: LocalAiRuntimeStatusSurfaceRowCandidate) => boolean
>;

function runtimeStatusSurfaceStateIsHonest(row: LocalAiRuntimeStatusSurfaceRowCandidate): boolean {
  return runtimeStatusSurfaceStateValidators[row.surfaceState](row);
}

function localAiRuntimeStatusSurfaceReadModelCountsMatch(
  readModel: LocalAiRuntimeStatusSurfaceReadModelCandidate
): boolean {
  if (new Set(readModel.rows.map((row) => row.rowId)).size !== readModel.rows.length) {
    return false;
  }

  return (
    readModel.readyVisibleCount === countRows(readModel.rows, 'ready-visible') &&
    readModel.queuedVisibleCount === countRows(readModel.rows, 'queued-visible') &&
    readModel.degradedVisibleCount === countRows(readModel.rows, 'degraded-visible') &&
    readModel.unavailableVisibleCount === countRows(readModel.rows, 'unavailable-visible') &&
    readModel.manualSetupRequiredCount === countRows(readModel.rows, 'manual-setup-required') &&
    LocalAiRuntimeStatusSurfaceNonClaims.every((nonClaim) => readModel.runtimeStatusNonClaims.includes(nonClaim))
  );
}

function countRows(
  rows: readonly Pick<LocalAiRuntimeStatusSurfaceRow, 'surfaceState'>[],
  surfaceState: LocalAiRuntimeStatusSurfaceState
): number {
  return rows.filter((row) => row.surfaceState === surfaceState).length;
}

export const decodeLocalAiRuntimeStatusSurfaceRow = Schema.decodeUnknownSync(LocalAiRuntimeStatusSurfaceRowSchema);
export const decodeLocalAiRuntimeStatusSurfaceReadModel = Schema.decodeUnknownSync(
  LocalAiRuntimeStatusSurfaceReadModelSchema
);
