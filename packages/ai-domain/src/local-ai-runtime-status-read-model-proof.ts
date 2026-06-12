import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
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
} from './local-ai-primitives';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyRuntimeStatusText = Schema.String.pipe(Schema.minLength(1));
const RuntimeStatusRowCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiRuntimeStatusSurfaceReadModelIdSchema = NonEmptyRuntimeStatusText.pipe(
  Schema.brand('LocalAiRuntimeStatusSurfaceReadModelId')
);
export const LocalAiRuntimeStatusSurfaceRowIdSchema = NonEmptyRuntimeStatusText.pipe(
  Schema.brand('LocalAiRuntimeStatusSurfaceRowId')
);
export const LocalAiRuntimeStatusSurfaceSourceRefSchema = NonEmptyRuntimeStatusText.pipe(
  Schema.brand('LocalAiRuntimeStatusSurfaceSourceRef')
);
export const LocalAiRuntimeStatusSurfaceLabelSchema = NonEmptyRuntimeStatusText.pipe(
  Schema.brand('LocalAiRuntimeStatusSurfaceLabel')
);
export const LocalAiRuntimeStatusSurfaceNonClaimSchema = NonEmptyRuntimeStatusText.pipe(
  Schema.brand('LocalAiRuntimeStatusSurfaceNonClaim')
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
    return entry.proofStatus === 'unavailable' ? 'unavailable-visible' : 'manual-setup-required';
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

function runtimeStatusLabelFor(entry: LocalAiRuntimeProviderProofEntry): string {
  switch (runtimeStatusSurfaceStateFor(entry)) {
    case 'ready-visible':
      return 'Local AI runtime ready';
    case 'queued-visible':
      return 'Local AI runtime queued';
    case 'degraded-visible':
      return 'Local AI runtime degraded';
    case 'unavailable-visible':
      return 'Local AI runtime unavailable';
    case 'manual-setup-required':
      return 'Local AI runtime needs setup';
  }
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

function runtimeStatusSurfaceStateIsHonest(row: LocalAiRuntimeStatusSurfaceRowCandidate): boolean {
  if (row.surfaceState === 'unavailable-visible' || row.surfaceState === 'manual-setup-required') {
    return row.unavailableReason !== null;
  }

  if (row.surfaceState === 'degraded-visible') {
    return row.degradedState !== 'none' && row.unavailableReason === null;
  }

  if (row.surfaceState === 'queued-visible') {
    return row.queueDepth > 0 && row.unavailableReason === null;
  }

  return row.queueDepth === 0 && row.degradedState === 'none' && row.unavailableReason === null;
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
