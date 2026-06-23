import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { LocalAiProviderSchedulerLifecycleSchema } from './local-ai-provider-scheduler';
import {
  LocalAiRuntimeProviderProofReadModel,
  LocalAiRuntimeProviderProofRequirementSchema,
  type LocalAiRuntimeProviderProofEntry,
  type LocalAiRuntimeProviderProofRequirement,
} from './local-ai-runtime-provider-proof';
import {
  ParentAssistantActionConfirmResultSchema,
  ParentAssistantActionPreviewResultSchema,
  ParentAssistantAnswerSchema,
  ParentAssistantProviderStatusSchema,
} from './parent-assistant';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const LocalAiParentAssistantRuntimeProofReadModelIdSchema = brandedNonEmptyStringSchema(
  'LocalAiParentAssistantRuntimeProofReadModelId'
);
export const LocalAiParentAssistantRuntimeProofEntryIdSchema = brandedNonEmptyStringSchema(
  'LocalAiParentAssistantRuntimeProofEntryId'
);
export const LocalAiParentAssistantRuntimeProofEvidenceLabelSchema = brandedNonEmptyStringSchema(
  'LocalAiParentAssistantRuntimeProofEvidenceLabel'
);
export const LocalAiParentAssistantRuntimeProofRuntimeCommandSchema = brandedNonEmptyStringSchema(
  'LocalAiParentAssistantRuntimeProofRuntimeCommand'
);
export const LocalAiParentAssistantRuntimeProofClaimBoundarySchema = brandedNonEmptyStringSchema(
  'LocalAiParentAssistantRuntimeProofClaimBoundary'
);
export const LocalAiParentAssistantRuntimeProofFallbackSchema = brandedNonEmptyStringSchema(
  'LocalAiParentAssistantRuntimeProofFallback'
);

export const LocalAiParentAssistantRuntimeProofRequirementSchema = withParser(
  Schema.Literal(
    'local-provider-answer-uses-shared-runtime',
    'busy-provider-degrades-without-extra-runtime',
    'provider-unavailable-is-explicit-and-cited',
    'child-safety-priority-keeps-assistant-queued',
    'api-provider-remains-optional-parent-authorized-boundary',
    'action-preview-confirm-requires-child-contract'
  )
);

export const LocalAiParentAssistantRuntimeProofStatusSchema = withParser(
  Schema.Literal('proved', 'degraded', 'unavailable', 'not-claimed')
);

const LocalAiParentAssistantRuntimeProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: LocalAiParentAssistantRuntimeProofEntryIdSchema,
  requirement: LocalAiParentAssistantRuntimeProofRequirementSchema,
  proofStatus: LocalAiParentAssistantRuntimeProofStatusSchema,
  sourceProviderProofRequirement: LocalAiRuntimeProviderProofRequirementSchema,
  sourceProviderProofEntryId: NonEmptyStringSchema,
  schedulerLifecycle: LocalAiProviderSchedulerLifecycleSchema,
  parentAssistantAnswer: Schema.Union(ParentAssistantAnswerSchema, Schema.Null),
  providerStatus: Schema.Union(ParentAssistantProviderStatusSchema, Schema.Null),
  actionPreviewResult: Schema.Union(ParentAssistantActionPreviewResultSchema, Schema.Null),
  actionConfirmResult: Schema.Union(ParentAssistantActionConfirmResultSchema, Schema.Null),
  localProviderSelected: Schema.Boolean,
  apiProviderSelected: Schema.Boolean,
  remoteAiOptional: Schema.Literal(true),
  evidenceCitationRequired: Schema.Literal(true),
  childSafetyOrEnforcementUseAllowed: Schema.Literal(false),
  runtimeProofCommand: LocalAiParentAssistantRuntimeProofRuntimeCommandSchema,
  evidenceLabel: LocalAiParentAssistantRuntimeProofEvidenceLabelSchema,
  claimBoundary: LocalAiParentAssistantRuntimeProofClaimBoundarySchema,
  fallbackBehavior: LocalAiParentAssistantRuntimeProofFallbackSchema,
  lastCheckedAt: ParentTimestampSchema,
});

type LocalAiParentAssistantRuntimeProofEntryCandidate = Infer<typeof LocalAiParentAssistantRuntimeProofEntryBaseSchema>;

export const LocalAiParentAssistantRuntimeProofEntrySchema = withParser(
  LocalAiParentAssistantRuntimeProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        localAiParentAssistantRuntimeProofEntryIsConsistent(entry) ||
        'Expected local AI Parent Assistant runtime proof entries to preserve provider scheduler, citation, optional API, and no-direct-enforcement boundaries'
    )
  )
);

const LocalAiParentAssistantRuntimeProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelId: LocalAiParentAssistantRuntimeProofReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceReadModelIds: Schema.Array(NonEmptyStringSchema),
  entries: Schema.Array(LocalAiParentAssistantRuntimeProofEntrySchema),
});

type LocalAiParentAssistantRuntimeProofReadModelCandidate = Infer<
  typeof LocalAiParentAssistantRuntimeProofReadModelBaseSchema
>;

export const LocalAiParentAssistantRuntimeProofReadModelSchema = withParser(
  LocalAiParentAssistantRuntimeProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        localAiParentAssistantRuntimeProofReadModelIsComplete(readModel) ||
        'Expected local AI Parent Assistant runtime proof read model to contain one unique entry for every proof requirement'
    )
  )
);

export type LocalAiParentAssistantRuntimeProofReadModelId =
  typeof LocalAiParentAssistantRuntimeProofReadModelIdSchema.Type;
export type LocalAiParentAssistantRuntimeProofEntryId = typeof LocalAiParentAssistantRuntimeProofEntryIdSchema.Type;
export type LocalAiParentAssistantRuntimeProofRequirement = Infer<
  typeof LocalAiParentAssistantRuntimeProofRequirementSchema
>;
export type LocalAiParentAssistantRuntimeProofStatus = Infer<typeof LocalAiParentAssistantRuntimeProofStatusSchema>;
export type LocalAiParentAssistantRuntimeProofEntry = Infer<typeof LocalAiParentAssistantRuntimeProofEntrySchema>;
export type LocalAiParentAssistantRuntimeProofReadModel = Infer<
  typeof LocalAiParentAssistantRuntimeProofReadModelSchema
>;

export const LocalAiParentAssistantRuntimeProofRequirementValues = [
  'local-provider-answer-uses-shared-runtime',
  'busy-provider-degrades-without-extra-runtime',
  'provider-unavailable-is-explicit-and-cited',
  'child-safety-priority-keeps-assistant-queued',
  'api-provider-remains-optional-parent-authorized-boundary',
  'action-preview-confirm-requires-child-contract',
] as const satisfies ReadonlyArray<LocalAiParentAssistantRuntimeProofRequirement>;

const localAiParentAssistantRuntimeProofRequirementValidators: Record<
  LocalAiParentAssistantRuntimeProofRequirement,
  (entry: LocalAiParentAssistantRuntimeProofEntryCandidate, source: LocalAiRuntimeProviderProofEntry) => boolean
> = {
  'local-provider-answer-uses-shared-runtime': localProviderAnswerUsesSharedRuntime,
  'busy-provider-degrades-without-extra-runtime': busyProviderDegradesWithoutExtraRuntime,
  'provider-unavailable-is-explicit-and-cited': providerUnavailableIsExplicitAndCited,
  'child-safety-priority-keeps-assistant-queued': childSafetyPriorityKeepsAssistantQueued,
  'api-provider-remains-optional-parent-authorized-boundary': apiProviderRemainsOptionalBoundary,
  'action-preview-confirm-requires-child-contract': actionPreviewConfirmRequiresChildContract,
};

function localAiParentAssistantRuntimeProofEntryIsConsistent(
  entry: LocalAiParentAssistantRuntimeProofEntryCandidate
): boolean {
  const source = sourceProofEntryFor(entry.sourceProviderProofEntryId, entry.sourceProviderProofRequirement);
  if (source === undefined || source.schedulerLifecycle !== entry.schedulerLifecycle) {
    return false;
  }

  if (!entryHasProofSubject(entry) || !entryRoutesStaySafe(entry) || !entryHasCitationProof(entry)) {
    return false;
  }

  return localAiParentAssistantRuntimeProofRequirementValidators[entry.requirement](entry, source);
}

function localAiParentAssistantRuntimeProofReadModelIsComplete(
  readModel: LocalAiParentAssistantRuntimeProofReadModelCandidate
): boolean {
  if (new Set(readModel.entries.map((entry) => entry.proofEntryId)).size !== readModel.entries.length) {
    return false;
  }

  const requirements = new Set(readModel.entries.map((entry) => entry.requirement));
  return LocalAiParentAssistantRuntimeProofRequirementValues.every((requirement) => requirements.has(requirement));
}

function entryHasProofSubject(entry: LocalAiParentAssistantRuntimeProofEntryCandidate): boolean {
  return (
    entry.parentAssistantAnswer !== null ||
    entry.providerStatus !== null ||
    entry.actionPreviewResult !== null ||
    entry.actionConfirmResult !== null
  );
}

function entryRoutesStaySafe(entry: LocalAiParentAssistantRuntimeProofEntryCandidate): boolean {
  const routes = [entry.parentAssistantAnswer?.providerRoute, entry.providerStatus?.providerRoute].filter(
    (route): route is NonNullable<typeof route> => route !== undefined
  );
  return routes.every(
    (route) =>
      route.remoteAiOptional === entry.remoteAiOptional &&
      route.evidenceCitationRequired === entry.evidenceCitationRequired &&
      route.childSafetyOrEnforcementUseAllowed === entry.childSafetyOrEnforcementUseAllowed &&
      route.selectedProvider === (entry.localProviderSelected ? 'local' : 'none')
  );
}

function entryHasCitationProof(entry: LocalAiParentAssistantRuntimeProofEntryCandidate): boolean {
  return (
    (entry.parentAssistantAnswer?.citations.length ?? 0) > 0 ||
    (entry.providerStatus?.apiProviderBoundary.citations.length ?? 0) > 0 ||
    (entry.actionPreviewResult?.evidenceContext.length ?? 0) > 0
  );
}

function localProviderAnswerUsesSharedRuntime(
  entry: LocalAiParentAssistantRuntimeProofEntryCandidate,
  source: LocalAiRuntimeProviderProofEntry
): boolean {
  return (
    entry.proofStatus === 'proved' &&
    entry.localProviderSelected &&
    !entry.apiProviderSelected &&
    source.parentAssistantSubmissionAllowed &&
    source.runtimeLoadCount === 1 &&
    entry.parentAssistantAnswer?.answerState === 'answered' &&
    entry.parentAssistantAnswer.providerRoute.routingState === 'local-provider-ready'
  );
}

function busyProviderDegradesWithoutExtraRuntime(
  entry: LocalAiParentAssistantRuntimeProofEntryCandidate,
  source: LocalAiRuntimeProviderProofEntry
): boolean {
  return (
    entry.proofStatus === 'degraded' &&
    entry.localProviderSelected &&
    source.runtimeLoadCount === 1 &&
    entry.parentAssistantAnswer?.answerState === 'degraded' &&
    entry.providerStatus?.busy === true
  );
}

function providerUnavailableIsExplicitAndCited(
  entry: LocalAiParentAssistantRuntimeProofEntryCandidate,
  source: LocalAiRuntimeProviderProofEntry
): boolean {
  return (
    entry.proofStatus === 'unavailable' &&
    !entry.localProviderSelected &&
    source.unavailableReason !== null &&
    entry.parentAssistantAnswer?.unavailableReason === source.unavailableReason &&
    entry.providerStatus?.providerRoute.routingState === 'no-provider-available'
  );
}

function childSafetyPriorityKeepsAssistantQueued(
  entry: LocalAiParentAssistantRuntimeProofEntryCandidate,
  source: LocalAiRuntimeProviderProofEntry
): boolean {
  return (
    entry.proofStatus === 'proved' &&
    source.childSafetyPriorityProved &&
    source.queue.childSafetyQueued > 0 &&
    source.queue.parentAssistantQueued > 0 &&
    entry.parentAssistantAnswer?.answerState === 'queued'
  );
}

function apiProviderRemainsOptionalBoundary(entry: LocalAiParentAssistantRuntimeProofEntryCandidate): boolean {
  return (
    entry.proofStatus === 'not-claimed' &&
    !entry.apiProviderSelected &&
    entry.providerStatus?.apiProviderBoundary.authorizationState === 'authorized' &&
    entry.providerStatus.apiProviderBoundary.accessState === 'authorized-degraded' &&
    entry.providerStatus.providerRoute.routingState === 'api-provider-authorized-degraded' &&
    entry.providerStatus.providerRoute.childSafetyOrEnforcementUseAllowed === false
  );
}

function actionPreviewConfirmRequiresChildContract(entry: LocalAiParentAssistantRuntimeProofEntryCandidate): boolean {
  return (
    entry.proofStatus === 'proved' &&
    entry.actionPreviewResult?.policyWritten === false &&
    entry.actionPreviewResult.enforcementApplied === false &&
    entry.actionConfirmResult?.policyWritten === false &&
    entry.actionConfirmResult.enforcementApplied === false &&
    entry.actionConfirmResult.childAgentContractRequired
  );
}

function sourceProofEntryFor(
  proofEntryId: string,
  requirement: LocalAiRuntimeProviderProofRequirement
): LocalAiRuntimeProviderProofEntry | undefined {
  return LocalAiRuntimeProviderProofReadModel.entries.find(
    (entry) => entry.proofEntryId === proofEntryId && entry.requirement === requirement
  );
}

export const decodeLocalAiParentAssistantRuntimeProofEntry = Schema.decodeUnknownSync(
  LocalAiParentAssistantRuntimeProofEntrySchema
);
export const decodeLocalAiParentAssistantRuntimeProofReadModel = Schema.decodeUnknownSync(
  LocalAiParentAssistantRuntimeProofReadModelSchema
);
