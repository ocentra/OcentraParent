import { type Infer, parseUnknown, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  type LocalAiContextReasonCode,
  type LocalAiEvidenceContext,
  type LocalAiEvidenceContextSourceRef,
  LocalAiContextReasonCodeSchema,
  LocalAiEvidenceContextIdSchema,
  LocalAiEvidenceContextRefIdSchema,
  LocalAiEvidenceContextSummarySchema,
  LocalAiStoredEvidenceContextBuildInputSchema,
} from '@ocentra-parent/ai-domain/local-ai-context';
import { LocalAiContextBuildStateSchema, LocalAiContextNonNegativeCountSchema } from './local-ai-context-primitives';
import { buildLocalAiEvidenceContext } from '@ocentra-parent/ai-domain/local-ai-context-builder';
import { LocalAiMemoryReferenceSchema, type LocalAiMemoryReference } from './local-ai-references';
import { LocalAiEvaluationRequestIdSchema, LocalAiTimestampSchema } from './local-ai-primitives';
import { ChildProfileReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';

export const LocalAiRecentMemoryWindowSchema = withParser(
  Schema.Struct({
    observedFrom: LocalAiTimestampSchema,
    observedUntil: LocalAiTimestampSchema,
    asOf: LocalAiTimestampSchema,
  }).pipe(
    Schema.filter(
      (window) =>
        Date.parse(window.observedUntil) >= Date.parse(window.observedFrom) ||
        'Expected local AI recent activity window to be ordered'
    ),
    Schema.filter(
      (window) =>
        Date.parse(window.asOf) >= Date.parse(window.observedUntil) ||
        'Expected local AI recent activity window to close before read time'
    )
  )
);

export const LocalAiRecentMemoryWindowReadInputSchema = withParser(
  Schema.Struct({
    contextInput: LocalAiStoredEvidenceContextBuildInputSchema,
    window: LocalAiRecentMemoryWindowSchema,
    limit: LocalAiContextNonNegativeCountSchema,
  })
);

export const LocalAiRecentMemoryWindowReadModelSchema = withParser(
  Schema.Struct({
    state: LocalAiContextBuildStateSchema,
    contextId: Schema.Union(LocalAiEvidenceContextIdSchema, Schema.Null),
    requestId: LocalAiEvaluationRequestIdSchema,
    readAt: LocalAiTimestampSchema,
    childProfile: ChildProfileReferenceSchema,
    device: ParentDeviceReferenceSchema,
    window: LocalAiRecentMemoryWindowSchema,
    recentActivityEvidenceRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
    recentActivitySourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    recentMemoryReferences: Schema.Array(LocalAiMemoryReferenceSchema),
    returnedRecentActivityCount: LocalAiContextNonNegativeCountSchema,
    returnedRecentMemoryCount: LocalAiContextNonNegativeCountSchema,
    omittedRecentActivityCount: LocalAiContextNonNegativeCountSchema,
    omittedRecentMemoryCount: LocalAiContextNonNegativeCountSchema,
    degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
    custodyBoundarySummary: LocalAiEvidenceContextSummarySchema,
    rawEvidenceRetained: Schema.Literal(false),
    remoteAiUsed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (model) =>
        model.returnedRecentActivityCount === model.recentActivityEvidenceRefs.length ||
        'Expected returned recent activity count to match refs'
    ),
    Schema.filter(
      (model) =>
        model.returnedRecentMemoryCount === model.recentMemoryReferences.length ||
        'Expected returned recent memory count to match refs'
    )
  )
);

export type LocalAiRecentMemoryWindow = Infer<typeof LocalAiRecentMemoryWindowSchema>;
export type LocalAiRecentMemoryWindowReadInput = Infer<typeof LocalAiRecentMemoryWindowReadInputSchema>;
export type LocalAiRecentMemoryWindowReadModel = Infer<typeof LocalAiRecentMemoryWindowReadModelSchema>;

export function buildLocalAiRecentMemoryWindowReadModel(input: unknown): LocalAiRecentMemoryWindowReadModel {
  const parsed = LocalAiRecentMemoryWindowReadInputSchema.parse(input);
  const contextResult = buildLocalAiEvidenceContext(parsed.contextInput);
  const baseDegradedReasons = contextResult.context?.degradedReasons ?? ['missing-evidence'];

  if (contextResult.context === null) {
    return LocalAiRecentMemoryWindowReadModelSchema.parse({
      state: 'insufficient',
      contextId: null,
      requestId: parsed.contextInput.request.requestId,
      readAt: parsed.window.asOf,
      childProfile: parsed.contextInput.request.childProfile,
      device: parsed.contextInput.request.device,
      window: parsed.window,
      recentActivityEvidenceRefs: [],
      recentActivitySourceEvidenceReferences: [],
      recentMemoryReferences: [],
      returnedRecentActivityCount: 0,
      returnedRecentMemoryCount: 0,
      omittedRecentActivityCount: parsed.contextInput.evidenceReferences.length,
      omittedRecentMemoryCount: parsed.contextInput.memoryReferences.length,
      degradedReasons: uniqueReasonCodes(baseDegradedReasons),
      custodyBoundarySummary: contextResult.custodyBoundarySummary,
      rawEvidenceRetained: false,
      remoteAiUsed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
    });
  }

  return LocalAiRecentMemoryWindowReadModelSchema.parse(
    readModelForContext(contextResult.context, parsed, baseDegradedReasons)
  );
}

function readModelForContext(
  context: LocalAiEvidenceContext,
  input: LocalAiRecentMemoryWindowReadInput,
  baseDegradedReasons: readonly LocalAiContextReasonCode[]
): LocalAiRecentMemoryWindowReadModel {
  const recentActivityCandidates = context.evidenceReferences.filter(
    (reference) => reference.evidenceKind === 'recent-activity'
  );
  const recentActivityReferences = recentActivityCandidates
    .filter((reference) => referenceIsInsideWindow(reference, input.window))
    .slice(0, input.limit);
  const selectedRecentEvidenceIds = selectedEvidenceIds(recentActivityReferences);
  const recentMemoryCandidates = context.memoryReferences.filter((reference) => reference.kind === 'recent-activity');
  const recentMemoryReferences = recentMemoryCandidates
    .filter((reference) => memoryIsGroundedInWindow(reference, selectedRecentEvidenceIds))
    .slice(0, input.limit);
  const omittedRecentActivityCount = recentActivityCandidates.length - recentActivityReferences.length;
  const omittedRecentMemoryCount = recentMemoryCandidates.length - recentMemoryReferences.length;
  const degradedReasons = uniqueReasonCodes([
    ...baseDegradedReasons,
    ...(recentActivityReferences.length === 0 ? ['missing-evidence' as const] : []),
    ...(omittedRecentActivityCount > 0 ? ['stale-evidence' as const] : []),
    ...(omittedRecentMemoryCount > 0 ? ['memory-ungrounded' as const] : []),
  ]);

  return {
    state: stateFor(recentActivityReferences.length, degradedReasons),
    contextId: context.contextId,
    requestId: context.requestId,
    readAt: input.window.asOf,
    childProfile: context.childProfile,
    device: context.device,
    window: input.window,
    recentActivityEvidenceRefs: recentActivityReferences.map((reference) => reference.evidenceRefId),
    recentActivitySourceEvidenceReferences: uniqueEvidenceReferences(
      recentActivityReferences.flatMap((reference) => reference.sourceEvidenceReferences)
    ),
    recentMemoryReferences,
    returnedRecentActivityCount: recentActivityReferences.length,
    returnedRecentMemoryCount: recentMemoryReferences.length,
    omittedRecentActivityCount,
    omittedRecentMemoryCount,
    degradedReasons,
    custodyBoundarySummary: parseUnknown(LocalAiEvidenceContextSummarySchema, context.custodyLabels.join(',')),
    rawEvidenceRetained: false,
    remoteAiUsed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
  };
}

function referenceIsInsideWindow(
  reference: LocalAiEvidenceContextSourceRef,
  window: LocalAiRecentMemoryWindow
): boolean {
  const observedAt = Date.parse(reference.observedAt);
  const freshUntil = reference.freshUntil === null ? Number.POSITIVE_INFINITY : Date.parse(reference.freshUntil);
  return (
    observedAt >= Date.parse(window.observedFrom) &&
    observedAt <= Date.parse(window.observedUntil) &&
    freshUntil > Date.parse(window.asOf)
  );
}

function selectedEvidenceIds(references: readonly LocalAiEvidenceContextSourceRef[]): Set<string> {
  const ids = new Set<string>();
  for (const reference of references) {
    ids.add(reference.evidenceRefId);
    ids.add(reference.evidence.evidenceReferenceId);
    for (const sourceReference of reference.sourceEvidenceReferences) {
      ids.add(sourceReference.evidenceReferenceId);
    }
  }
  return ids;
}

function memoryIsGroundedInWindow(
  reference: LocalAiMemoryReference,
  selectedRecentEvidenceIds: ReadonlySet<string>
): boolean {
  return reference.sourceEvidenceReferences.every((sourceReference) =>
    selectedRecentEvidenceIds.has(sourceReference.evidenceReferenceId)
  );
}

function stateFor(
  recentActivityCount: number,
  degradedReasons: readonly LocalAiContextReasonCode[]
): LocalAiRecentMemoryWindowReadModel['state'] {
  if (recentActivityCount === 0) {
    return 'insufficient';
  }
  return degradedReasons.length > 0 ? 'partial' : 'ready';
}

function uniqueReasonCodes(reasonCodes: readonly LocalAiContextReasonCode[]): LocalAiContextReasonCode[] {
  return [...new Set(reasonCodes)];
}

function uniqueEvidenceReferences(
  references: readonly LocalAiRecentMemoryWindowReadModel['recentActivitySourceEvidenceReferences'][number][]
): LocalAiRecentMemoryWindowReadModel['recentActivitySourceEvidenceReferences'] {
  const referencesById = new Map(references.map((reference) => [reference.evidenceReferenceId, reference]));
  return [...referencesById.values()];
}
