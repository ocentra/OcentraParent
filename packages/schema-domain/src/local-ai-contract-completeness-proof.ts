import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { LocalAiEvaluationInputSchema, LocalAiSafetyResultSchema } from './local-ai';
import { LocalAiDegradedState, LocalAiUnknownState } from './ai-primitives';
import {
  LocalAiProviderSchedulerDecisionSchema,
  LocalAiProviderSchedulerStatusSchema,
} from './local-ai-provider-scheduler';
import { LocalProviderCapabilitySchema, LocalModelRuntimeStatusSchema } from './ai-runtime';
import { ParentContractSchemaVersion } from './family-reference-primitives';
const LocalAiContractProofCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiContractCompletenessProofIdSchema = brandedNonEmptyStringSchema(
  'LocalAiContractCompletenessProofId'
);

export const LocalAiContractCompletenessContractKindSchema = withParser(
  Schema.Literal('input', 'result', 'provider-capability', 'job-queue', 'provider-route')
);

export const LocalAiContractCompletenessClaimBoundariesSchema = withParser(
  Schema.Struct({
    modelExecutionClaimed: Schema.Boolean,
    modelQualityClaimed: Schema.Boolean,
    policyAuthorityClaimed: Schema.Boolean,
    enforcementClaimed: Schema.Boolean,
    portalUiClaimed: Schema.Boolean,
    remoteApiAiUsed: Schema.Boolean,
    rawPromptRetained: Schema.Boolean,
    rawEvidenceRetained: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (boundaries) =>
        Object.values(boundaries).every((claim) => claim === false) ||
        'Expected local AI contract completeness proof to keep every runtime/UI/enforcement claim false'
    )
  )
);

const LocalAiContractCompletenessProofBaseSchema = Schema.Struct({
  proofId: LocalAiContractCompletenessProofIdSchema,
  generatedAt: NonEmptyStringSchema,
  evaluationInput: LocalAiEvaluationInputSchema,
  safetyResult: LocalAiSafetyResultSchema,
  providerCapability: LocalProviderCapabilitySchema,
  runtimeStatus: LocalModelRuntimeStatusSchema,
  queueStatus: LocalAiProviderSchedulerStatusSchema,
  routeDecision: LocalAiProviderSchedulerDecisionSchema,
  provedContractKinds: Schema.Array(LocalAiContractCompletenessContractKindSchema),
  validationSummary: Schema.Struct({
    evidenceReferenceCount: LocalAiContractProofCountSchema,
    parentRuleReferenceCount: LocalAiContractProofCountSchema,
    memoryReferenceCount: LocalAiContractProofCountSchema,
    graphReferenceCount: LocalAiContractProofCountSchema,
    providerCapabilityCount: LocalAiContractProofCountSchema,
    selectedRuntimeCount: LocalAiContractProofCountSchema,
    queuedJobCount: LocalAiContractProofCountSchema,
  }),
  claimBoundaries: LocalAiContractCompletenessClaimBoundariesSchema,
});

type LocalAiContractCompletenessProofCandidate = Infer<typeof LocalAiContractCompletenessProofBaseSchema>;

export const LocalAiContractCompletenessProofSchema = withParser(
  LocalAiContractCompletenessProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localAiContractCompletenessProofIsReady(proof) ||
        'Expected local AI contract completeness proof to cite evidence, rules, local provider route, and matching runtime metadata'
    )
  )
);

export type LocalAiContractCompletenessClaimBoundaries = Infer<typeof LocalAiContractCompletenessClaimBoundariesSchema>;
export type LocalAiContractCompletenessProof = Infer<typeof LocalAiContractCompletenessProofSchema>;

const generatedAt = '2026-06-06T07:08:00.000Z';

const evidenceReference = {
  evidenceReferenceId: 'local-ai-contract-proof-screen-summary',
  kind: 'journal-event',
  observedAt: '2026-06-06T07:07:00.000Z',
} as const;

const childProfile = { childProfileId: 'child-local-ai-contract', displayName: 'Sam' } as const;
const device = {
  deviceId: 'device-local-ai-contract',
  childProfileId: 'child-local-ai-contract',
  label: 'Sam Windows PC',
  platform: 'windows',
} as const;
const modelRequest = {
  providerId: 'local-provider-llama-cli',
  modelId: 'screen-safety-local-model',
  promptVersion: 'screen-safety-template-v1',
} as const;

const memoryReference = {
  memoryReferenceId: 'recent-memory-contract-proof',
  kind: 'recent-activity',
  sourceEvidenceReferences: [evidenceReference],
  sourcePolicyVersion: 'policy-v1',
  generatedAt,
  confidence: 0.84,
  derivedIndexVersion: 'recent-memory-index-v1',
} as const;

const graphReference = {
  graphReferenceId: 'graph-edge-contract-proof',
  kind: 'graph-edge',
  sourceEvidenceReferences: [evidenceReference],
  sourcePolicyVersion: 'policy-v1',
  generatedAt,
  confidence: 0.81,
  derivedIndexVersion: 'activity-graph-index-v1',
} as const;

const runtimeStatus = LocalModelRuntimeStatusSchema.parse({
  runtimeReferenceId: 'runtime-local-ai-contract',
  providerId: modelRequest.providerId,
  modelId: modelRequest.modelId,
  modelReference: 'artifact:screen_safety_local_model',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: LocalAiDegradedState.None,
  lastCheckedAt: generatedAt,
  unavailableReason: null,
});

const queueStatus = LocalAiProviderSchedulerStatusSchema.parse({
  physicalDeviceId: 'physical-device-local-ai-contract',
  singletonScope: 'physical-device',
  providerId: modelRequest.providerId,
  runtimeReferenceId: runtimeStatus.runtimeReferenceId,
  modelId: modelRequest.modelId,
  modelReference: runtimeStatus.modelReference,
  resourceClass: runtimeStatus.resourceClass,
  lifecycleState: 'queued',
  currentJobClass: 'parent-assistant',
  queue: { childSafetyQueued: 1, parentAssistantQueued: 1, parentReportQueued: 0 },
  duplicateRuntimeBlocked: true,
  degradedState: LocalAiDegradedState.Overloaded,
  unavailableReason: null,
  lastCheckedAt: generatedAt,
});

const routeDecision = LocalAiProviderSchedulerDecisionSchema.parse({
  physicalDeviceId: queueStatus.physicalDeviceId,
  jobClass: 'child-safety',
  jobStatus: 'queued',
  selectedRuntimeReferenceId: runtimeStatus.runtimeReferenceId,
  queuePosition: 1,
  unavailableReason: null,
  duplicateRuntimeBlocked: true,
});

const evaluationInput = LocalAiEvaluationInputSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  requestId: 'local-ai-contract-proof-request',
  childProfile,
  device,
  currentObservation: { contextKind: 'recent-activity', evidence: evidenceReference },
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule-screen-safety'],
  recentActivityWindow: [evidenceReference],
  memoryReferences: [memoryReference],
  graphReferences: [graphReference],
  modelRequest,
});

const safetyResult = LocalAiSafetyResultSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  resultId: 'local-ai-contract-proof-result',
  requestId: evaluationInput.requestId,
  action: 'ask-parent',
  confidence: 0.63,
  unknownState: LocalAiUnknownState.LowConfidence,
  degradedState: LocalAiDegradedState.None,
  reasonCodes: ['low-confidence-video-domain'],
  explanationReference: 'local-ai-contract-proof-explanation',
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule-screen-safety'],
  memoryReferences: [memoryReference],
  graphReferences: [graphReference],
  modelRuntime: runtimeStatus,
  promptVersion: modelRequest.promptVersion,
  expiresAt: null,
});

export const LocalAiContractCompletenessProof = LocalAiContractCompletenessProofSchema.parse({
  proofId: 'local-ai-contract-completeness-proof',
  generatedAt,
  evaluationInput,
  safetyResult,
  providerCapability: {
    providerId: modelRequest.providerId,
    supportedTasks: ['safety-decision', 'classification'],
    resourceClass: runtimeStatus.resourceClass,
    privacyMode: 'local-only',
    fallbackOrder: 1,
  },
  runtimeStatus,
  queueStatus,
  routeDecision,
  provedContractKinds: ['input', 'result', 'provider-capability', 'job-queue', 'provider-route'],
  validationSummary: {
    evidenceReferenceCount: 1,
    parentRuleReferenceCount: 1,
    memoryReferenceCount: 1,
    graphReferenceCount: 1,
    providerCapabilityCount: 1,
    selectedRuntimeCount: 1,
    queuedJobCount: 2,
  },
  claimBoundaries: {
    modelExecutionClaimed: false,
    modelQualityClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    portalUiClaimed: false,
    remoteApiAiUsed: false,
    rawPromptRetained: false,
    rawEvidenceRetained: false,
  },
});

function localAiContractCompletenessProofIsReady(proof: LocalAiContractCompletenessProofCandidate): boolean {
  return (
    proof.provedContractKinds.length === 5 &&
    localAiRequestAndResultMatch(proof) &&
    localAiRuntimeRouteMatches(proof) &&
    localAiValidationSummaryMatchesInput(proof) &&
    localAiResultIsCited(proof) &&
    proof.safetyResult.modelRuntime.privacyMode === 'local-only'
  );
}

function localAiRequestAndResultMatch(proof: LocalAiContractCompletenessProofCandidate): boolean {
  return (
    proof.evaluationInput.requestId === proof.safetyResult.requestId &&
    proof.evaluationInput.modelRequest.promptVersion === proof.safetyResult.promptVersion
  );
}

function localAiRuntimeRouteMatches(proof: LocalAiContractCompletenessProofCandidate): boolean {
  return (
    proof.routeDecision.selectedRuntimeReferenceId === proof.runtimeStatus.runtimeReferenceId &&
    proof.queueStatus.runtimeReferenceId === proof.runtimeStatus.runtimeReferenceId &&
    proof.providerCapability.providerId === proof.runtimeStatus.providerId
  );
}

function localAiValidationSummaryMatchesInput(proof: LocalAiContractCompletenessProofCandidate): boolean {
  return (
    proof.validationSummary.evidenceReferenceCount === proof.evaluationInput.evidenceReferences.length &&
    proof.validationSummary.parentRuleReferenceCount === proof.evaluationInput.parentRuleReferences.length &&
    proof.validationSummary.memoryReferenceCount === proof.evaluationInput.memoryReferences.length &&
    proof.validationSummary.graphReferenceCount === proof.evaluationInput.graphReferences.length
  );
}

function localAiResultIsCited(proof: LocalAiContractCompletenessProofCandidate): boolean {
  return (
    proof.safetyResult.evidenceReferences.length > 0 &&
    proof.safetyResult.parentRuleReferences.length > 0 &&
    proof.safetyResult.memoryReferences.length > 0 &&
    proof.safetyResult.graphReferences.length > 0
  );
}

export const decodeLocalAiContractCompletenessProof = Schema.decodeUnknownSync(LocalAiContractCompletenessProofSchema);
