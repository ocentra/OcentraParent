import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LocalAiEvaluationInputSchema, LocalAiSafetyResultSchema, type LocalAiSafetyResult } from './local-ai';
import {
  LocalAiDegradedState,
  LocalAiContextKindSchema,
  LocalAiPromptVersionSchema,
  LocalAiUnknownState,
  type LocalAiContextKind,
} from './local-ai-primitives';
import { LocalModelRuntimeStatusSchema, type LocalModelRuntimeStatus } from './local-ai-runtime';
import { PolicyAction, PolicyReasonCodeSchema } from './policy';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from './reference-primitives';

const NonEmptyClassifierText = Schema.String.pipe(Schema.minLength(1));

export const LocalAiDeterministicClassifierRunIdSchema = NonEmptyClassifierText.pipe(
  Schema.brand('LocalAiDeterministicClassifierRunId')
);
export const LocalAiDeterministicClassifierTraceRefSchema = NonEmptyClassifierText.pipe(
  Schema.brand('LocalAiDeterministicClassifierTraceRef')
);
export const LocalAiDeterministicClassifierNonClaimSchema = NonEmptyClassifierText.pipe(
  Schema.brand('LocalAiDeterministicClassifierNonClaim')
);

export const LocalAiDeterministicClassifierStateSchema = withParser(
  Schema.Literal('classified', 'low-confidence', 'missing-evidence', 'runtime-unavailable')
);

const LocalAiDeterministicClassifierInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  classifierRunId: LocalAiDeterministicClassifierRunIdSchema,
  evaluationInput: LocalAiEvaluationInputSchema,
  modelRuntime: LocalModelRuntimeStatusSchema,
  rawEvidenceRetained: Schema.Boolean,
});

type LocalAiDeterministicClassifierInputCandidate = Infer<typeof LocalAiDeterministicClassifierInputBaseSchema>;

export const LocalAiDeterministicClassifierInputSchema = withParser(
  LocalAiDeterministicClassifierInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        deterministicClassifierInputIsReady(input) ||
        'Expected deterministic classifier input to use matching local runtime metadata without raw evidence retention'
    )
  )
);

const LocalAiDeterministicClassifierResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  classifierRunId: LocalAiDeterministicClassifierRunIdSchema,
  state: LocalAiDeterministicClassifierStateSchema,
  result: LocalAiSafetyResultSchema,
  contextKind: LocalAiContextKindSchema,
  modelRuntime: LocalModelRuntimeStatusSchema,
  promptVersion: LocalAiPromptVersionSchema,
  evidenceReferenceCount: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  parentRuleReferenceCount: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  dryRun: Schema.Boolean,
  deterministicOnly: Schema.Boolean,
  localOnly: Schema.Boolean,
  modelExecuted: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  productionModelQualityClaimed: Schema.Boolean,
  rawEvidenceRetained: Schema.Boolean,
  classifierTraceRefs: Schema.Array(LocalAiDeterministicClassifierTraceRefSchema),
  nonClaims: Schema.Array(LocalAiDeterministicClassifierNonClaimSchema),
});

type LocalAiDeterministicClassifierResultCandidate = Infer<typeof LocalAiDeterministicClassifierResultBaseSchema>;

export const LocalAiDeterministicClassifierResultSchema = withParser(
  LocalAiDeterministicClassifierResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        deterministicClassifierResultIsHonest(result) ||
        'Expected deterministic classifier result to stay local-only, deterministic-only, non-enforcing, and non-retaining'
    )
  )
);

export type LocalAiDeterministicClassifierInput = Infer<typeof LocalAiDeterministicClassifierInputSchema>;
export type LocalAiDeterministicClassifierResult = Infer<typeof LocalAiDeterministicClassifierResultSchema>;
export type LocalAiDeterministicClassifierState = Infer<typeof LocalAiDeterministicClassifierStateSchema>;

const decodeTraceRef = Schema.decodeUnknownSync(LocalAiDeterministicClassifierTraceRefSchema);
const decodeNonClaim = Schema.decodeUnknownSync(LocalAiDeterministicClassifierNonClaimSchema);
const decodeReasonCode = Schema.decodeUnknownSync(PolicyReasonCodeSchema);

const VideoReasonCode = decodeReasonCode('local-ai-deterministic-video-warning');
const SafeProductivityReasonCode = decodeReasonCode('local-ai-deterministic-productivity-allow');
const AppTimeLimitReasonCode = decodeReasonCode('local-ai-deterministic-app-time-limit');
const ProcessBlockReasonCode = decodeReasonCode('local-ai-deterministic-process-block');
const NetworkReviewReasonCode = decodeReasonCode('local-ai-deterministic-network-review');
const MissingEvidenceReasonCode = decodeReasonCode('local-ai-deterministic-missing-evidence');
const RuntimeUnavailableReasonCode = decodeReasonCode('local-ai-deterministic-runtime-unavailable');
const LowConfidenceReasonCode = decodeReasonCode('local-ai-deterministic-low-confidence');

export const LocalAiDeterministicClassifierNonClaims = [
  decodeNonClaim('This proof is a deterministic local classifier lane over typed evidence refs, not model execution.'),
  decodeNonClaim(
    'This proof does not claim production model quality, remote/API AI, policy authority, or enforcement.'
  ),
  decodeNonClaim(
    'Raw evidence is not retained; only typed evidence, parent-rule, runtime, and trace refs are preserved.'
  ),
] as const;

export function runLocalAiDeterministicClassifier(input: unknown): LocalAiDeterministicClassifierResult {
  const parsed = LocalAiDeterministicClassifierInputSchema.parse(input);
  const result = resultFor(parsed);

  return LocalAiDeterministicClassifierResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    classifierRunId: parsed.classifierRunId,
    state: stateFor(parsed, result),
    result,
    contextKind: parsed.evaluationInput.currentObservation.contextKind,
    modelRuntime: parsed.modelRuntime,
    promptVersion: parsed.evaluationInput.modelRequest.promptVersion,
    evidenceReferenceCount: parsed.evaluationInput.evidenceReferences.length,
    parentRuleReferenceCount: parsed.evaluationInput.parentRuleReferences.length,
    dryRun: true,
    deterministicOnly: true,
    localOnly: true,
    modelExecuted: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    productionModelQualityClaimed: false,
    rawEvidenceRetained: false,
    classifierTraceRefs: [decodeTraceRef(`local-ai-deterministic-classifier:${parsed.evaluationInput.requestId}`)],
    nonClaims: LocalAiDeterministicClassifierNonClaims,
  });
}

function resultFor(parsed: LocalAiDeterministicClassifierInput): LocalAiSafetyResult {
  const unavailable = runtimeUnavailable(parsed.modelRuntime);
  const missingEvidence = parsed.evaluationInput.evidenceReferences.length === 0;
  const contextKind = parsed.evaluationInput.currentObservation.contextKind;

  return LocalAiSafetyResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: `local-ai-deterministic-classifier-result:${parsed.evaluationInput.requestId}`,
    requestId: parsed.evaluationInput.requestId,
    action: actionFor(contextKind, unavailable, missingEvidence),
    confidence: confidenceFor(contextKind, unavailable, missingEvidence),
    unknownState: unknownStateFor(unavailable, missingEvidence),
    degradedState: degradedStateFor(contextKind, parsed.modelRuntime, unavailable),
    reasonCodes: reasonCodesFor(contextKind, unavailable, missingEvidence),
    explanationReference: `local-ai-deterministic-classifier-explanation:${parsed.evaluationInput.requestId}`,
    evidenceReferences: parsed.evaluationInput.evidenceReferences,
    parentRuleReferences: parsed.evaluationInput.parentRuleReferences,
    memoryReferences: parsed.evaluationInput.memoryReferences,
    graphReferences: parsed.evaluationInput.graphReferences,
    modelRuntime: parsed.modelRuntime,
    promptVersion: parsed.evaluationInput.modelRequest.promptVersion,
    expiresAt: null,
  });
}

function actionFor(
  contextKind: LocalAiContextKind,
  unavailable: boolean,
  missingEvidence: boolean
): LocalAiSafetyResult['action'] {
  if (unavailable) {
    return PolicyAction.AskParent;
  }
  if (missingEvidence) {
    return PolicyAction.Unknown;
  }
  if (contextKind === 'network') {
    return PolicyAction.AskParent;
  }
  if (contextKind === 'app') {
    return PolicyAction.TimeLimit;
  }
  if (contextKind === 'process') {
    return PolicyAction.Block;
  }
  if (contextKind === 'video' || contextKind === 'url' || contextKind === 'domain' || contextKind === 'page') {
    return PolicyAction.Warn;
  }
  return PolicyAction.Allow;
}

function confidenceFor(contextKind: LocalAiContextKind, unavailable: boolean, missingEvidence: boolean): number {
  if (unavailable) {
    return 0.2;
  }
  if (missingEvidence) {
    return 0.1;
  }
  if (contextKind === 'network') {
    return 0.55;
  }
  if (contextKind === 'app' || contextKind === 'process') {
    return 0.78;
  }
  if (contextKind === 'page' || contextKind === 'domain') {
    return 0.58;
  }
  return 0.82;
}

function unknownStateFor(
  unavailable: boolean,
  missingEvidence: boolean
): (typeof LocalAiUnknownState)[keyof typeof LocalAiUnknownState] {
  if (unavailable) {
    return LocalAiUnknownState.ModelUnavailable;
  }
  if (missingEvidence) {
    return LocalAiUnknownState.MissingEvidence;
  }
  return LocalAiUnknownState.None;
}

function degradedStateFor(
  contextKind: LocalAiContextKind,
  runtime: LocalModelRuntimeStatus,
  unavailable: boolean
): (typeof LocalAiDegradedState)[keyof typeof LocalAiDegradedState] {
  if (unavailable) {
    return LocalAiDegradedState.ProviderUnavailable;
  }
  if (contextKind === 'network' || contextKind === 'page' || contextKind === 'domain') {
    return LocalAiDegradedState.InvalidOutput;
  }
  return runtime.degradedState;
}

function reasonCodesFor(
  contextKind: LocalAiContextKind,
  unavailable: boolean,
  missingEvidence: boolean
): LocalAiSafetyResult['reasonCodes'] {
  if (unavailable) {
    return [RuntimeUnavailableReasonCode];
  }
  if (missingEvidence) {
    return [MissingEvidenceReasonCode];
  }
  if (contextKind === 'network') {
    return [NetworkReviewReasonCode, LowConfidenceReasonCode];
  }
  if (contextKind === 'page' || contextKind === 'domain') {
    return [VideoReasonCode, LowConfidenceReasonCode];
  }
  if (contextKind === 'app') {
    return [AppTimeLimitReasonCode];
  }
  if (contextKind === 'process') {
    return [ProcessBlockReasonCode];
  }
  if (contextKind === 'video' || contextKind === 'url') {
    return [VideoReasonCode];
  }
  return [SafeProductivityReasonCode];
}

function stateFor(
  parsed: LocalAiDeterministicClassifierInput,
  result: LocalAiSafetyResult
): LocalAiDeterministicClassifierState {
  if (runtimeUnavailable(parsed.modelRuntime)) {
    return 'runtime-unavailable';
  }
  if (parsed.evaluationInput.evidenceReferences.length === 0) {
    return 'missing-evidence';
  }
  if (result.confidence < 0.6) {
    return 'low-confidence';
  }
  return 'classified';
}

function runtimeUnavailable(runtime: LocalModelRuntimeStatus): boolean {
  return (
    runtime.executionState === 'disabled' ||
    runtime.loadState === 'unavailable' ||
    !runtime.capabilityFlags.includes('classification')
  );
}

function deterministicClassifierInputIsReady(candidate: LocalAiDeterministicClassifierInputCandidate): boolean {
  return (
    !candidate.rawEvidenceRetained &&
    candidate.modelRuntime.privacyMode === 'local-only' &&
    candidate.modelRuntime.providerId === candidate.evaluationInput.modelRequest.providerId &&
    candidate.modelRuntime.modelId === candidate.evaluationInput.modelRequest.modelId
  );
}

function deterministicClassifierResultIsHonest(candidate: LocalAiDeterministicClassifierResultCandidate): boolean {
  return (
    deterministicClassifierKeepsNonClaimBoundary(candidate) &&
    candidate.dryRun &&
    deterministicClassifierMatchesRuntime(candidate) &&
    deterministicClassifierMatchesReferenceCounts(candidate)
  );
}

function deterministicClassifierKeepsNonClaimBoundary(
  candidate: LocalAiDeterministicClassifierResultCandidate
): boolean {
  return (
    candidate.deterministicOnly &&
    candidate.localOnly &&
    !candidate.modelExecuted &&
    !candidate.remoteApiClaimed &&
    !candidate.policyAuthorityClaimed &&
    !candidate.enforcementClaimed &&
    !candidate.productionModelQualityClaimed &&
    !candidate.rawEvidenceRetained
  );
}

function deterministicClassifierMatchesRuntime(candidate: LocalAiDeterministicClassifierResultCandidate): boolean {
  return (
    candidate.result.modelRuntime.providerId === candidate.modelRuntime.providerId &&
    candidate.result.modelRuntime.modelId === candidate.modelRuntime.modelId &&
    candidate.result.promptVersion === candidate.promptVersion
  );
}

function deterministicClassifierMatchesReferenceCounts(
  candidate: LocalAiDeterministicClassifierResultCandidate
): boolean {
  return (
    candidate.result.evidenceReferences.length === candidate.evidenceReferenceCount &&
    candidate.result.parentRuleReferences.length === candidate.parentRuleReferenceCount
  );
}
