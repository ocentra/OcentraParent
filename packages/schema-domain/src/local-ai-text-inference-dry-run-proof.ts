import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { PolicyAction, PolicyReasonCodeSchema } from './policy-contracts';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from './family-reference-primitives';
import { LocalAiEvaluationInputSchema, LocalAiSafetyResultSchema, type LocalAiSafetyResult } from './local-ai';
import {
  LocalAiDegradedState,
  LocalAiPromptVersionSchema,
  LocalAiUnknownState,
  type LocalAiDegradedStateSchema,
} from './ai-primitives';
import { LocalModelRuntimeStatusSchema, type LocalModelRuntimeStatus } from './ai-runtime';

export const LocalAiTextInferenceDryRunIdSchema = brandedNonEmptyStringSchema('LocalAiTextInferenceDryRunId');
export const LocalAiTextInferenceTraceRefSchema = brandedNonEmptyStringSchema('LocalAiTextInferenceTraceRef');
export const LocalAiTextInferenceNonClaimSchema = brandedNonEmptyStringSchema('LocalAiTextInferenceNonClaim');

export const LocalAiTextInferenceDryRunStateSchema = withParser(
  Schema.Literal('ready-dry-run', 'degraded-dry-run', 'unavailable-dry-run')
);

const LocalAiTextInferenceDryRunInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  dryRunId: LocalAiTextInferenceDryRunIdSchema,
  evaluationInput: LocalAiEvaluationInputSchema,
  modelRuntime: LocalModelRuntimeStatusSchema,
  rawPromptRetained: Schema.Boolean,
});

type LocalAiTextInferenceDryRunInputCandidate = Infer<typeof LocalAiTextInferenceDryRunInputBaseSchema>;

export const LocalAiTextInferenceDryRunInputSchema = withParser(
  LocalAiTextInferenceDryRunInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        localAiTextInferenceDryRunInputIsReady(input) ||
        'Expected local text inference dry-run input to use matching local runtime metadata without retaining raw prompts'
    )
  )
);

const LocalAiTextInferenceDryRunResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  dryRunId: LocalAiTextInferenceDryRunIdSchema,
  state: LocalAiTextInferenceDryRunStateSchema,
  result: LocalAiSafetyResultSchema,
  modelRuntime: LocalModelRuntimeStatusSchema,
  promptVersion: LocalAiPromptVersionSchema,
  evidenceReferenceCount: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  parentRuleReferenceCount: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  localOnly: Schema.Boolean,
  dryRunOnly: Schema.Boolean,
  modelExecuted: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  productionModelQualityClaimed: Schema.Boolean,
  rawPromptRetained: Schema.Boolean,
  inferenceTraceRefs: Schema.Array(LocalAiTextInferenceTraceRefSchema),
  nonClaims: Schema.Array(LocalAiTextInferenceNonClaimSchema),
});

type LocalAiTextInferenceDryRunResultCandidate = Infer<typeof LocalAiTextInferenceDryRunResultBaseSchema>;

export const LocalAiTextInferenceDryRunResultSchema = withParser(
  LocalAiTextInferenceDryRunResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        localAiTextInferenceDryRunResultIsHonest(result) ||
        'Expected local text inference dry-run result to stay local-only, dry-run-only, non-enforcing, and non-retaining'
    )
  )
);

export type LocalAiTextInferenceDryRunInput = Infer<typeof LocalAiTextInferenceDryRunInputSchema>;
export type LocalAiTextInferenceDryRunResult = Infer<typeof LocalAiTextInferenceDryRunResultSchema>;
export type LocalAiTextInferenceDryRunState = Infer<typeof LocalAiTextInferenceDryRunStateSchema>;

const decodeNonClaim = Schema.decodeUnknownSync(LocalAiTextInferenceNonClaimSchema);
const decodeTraceRef = Schema.decodeUnknownSync(LocalAiTextInferenceTraceRefSchema);
const decodeReasonCode = Schema.decodeUnknownSync(PolicyReasonCodeSchema);
const RuntimeUnavailableReasonCode = decodeReasonCode('local-ai-text-runtime-unavailable');
const MissingEvidenceReasonCode = decodeReasonCode('local-ai-text-missing-evidence');
const CandidateReasonCode = decodeReasonCode('local-ai-text-dry-run-candidate');

export const LocalAiTextInferenceDryRunNonClaims = [
  decodeNonClaim('This proof validates the local text inference dry-run boundary without executing a model.'),
  decodeNonClaim(
    'This proof does not claim production model quality, remote/API AI, policy authority, or enforcement.'
  ),
  decodeNonClaim('Raw prompt text is not retained; only typed evidence, rule, runtime, and trace refs are preserved.'),
] as const;

type DryRunOutcomeScenario = 'runtime-unavailable' | 'missing-evidence' | 'candidate';

const dryRunOutcomeDefinitions = {
  'runtime-unavailable': {
    action: PolicyAction.AskParent,
    confidence: 0.2,
    unknownState: LocalAiUnknownState.ModelUnavailable,
    reasonCodes: [RuntimeUnavailableReasonCode],
  },
  'missing-evidence': {
    action: PolicyAction.Unknown,
    confidence: 0.15,
    unknownState: LocalAiUnknownState.MissingEvidence,
    reasonCodes: [MissingEvidenceReasonCode],
  },
  candidate: {
    action: PolicyAction.Warn,
    confidence: 0.62,
    unknownState: LocalAiUnknownState.None,
    reasonCodes: [CandidateReasonCode],
  },
} as const satisfies Record<
  DryRunOutcomeScenario,
  Pick<LocalAiSafetyResult, 'action' | 'confidence' | 'unknownState' | 'reasonCodes'>
>;

const dryRunStateDefinitions = {
  'unavailable-dry-run': {
    action: 'ask-parent',
    unknownState: 'model-unavailable',
  },
  'degraded-dry-run': {
    degradedStateRequired: true,
    unknownStateAllowed: true,
  },
  'ready-dry-run': {
    action: 'warn',
    unknownState: 'none',
  },
} as const;

export function runLocalAiTextInferenceDryRun(input: unknown): LocalAiTextInferenceDryRunResult {
  const parsed = LocalAiTextInferenceDryRunInputSchema.parse(input);
  const result = resultFor(parsed);

  return LocalAiTextInferenceDryRunResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    dryRunId: parsed.dryRunId,
    state: stateFor(parsed.modelRuntime),
    result,
    modelRuntime: parsed.modelRuntime,
    promptVersion: parsed.evaluationInput.modelRequest.promptVersion,
    evidenceReferenceCount: parsed.evaluationInput.evidenceReferences.length,
    parentRuleReferenceCount: parsed.evaluationInput.parentRuleReferences.length,
    localOnly: true,
    dryRunOnly: true,
    modelExecuted: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    productionModelQualityClaimed: false,
    rawPromptRetained: false,
    inferenceTraceRefs: [decodeTraceRef(`local-ai-text-dry-run:${parsed.evaluationInput.requestId}`)],
    nonClaims: LocalAiTextInferenceDryRunNonClaims,
  });
}

function scenarioFor(unavailable: boolean, missingEvidence: boolean): DryRunOutcomeScenario {
  if (unavailable) {
    return 'runtime-unavailable';
  }
  if (missingEvidence) {
    return 'missing-evidence';
  }
  return 'candidate';
}

function resultFor(parsed: LocalAiTextInferenceDryRunInput): LocalAiSafetyResult {
  const unavailable =
    parsed.modelRuntime.executionState === 'disabled' || parsed.modelRuntime.loadState === 'unavailable';
  const missingEvidence = parsed.evaluationInput.evidenceReferences.length === 0;
  const scenario = scenarioFor(unavailable, missingEvidence);
  const outcome = dryRunOutcomeDefinitions[scenario];

  return LocalAiSafetyResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: `local-ai-text-dry-run-result:${parsed.evaluationInput.requestId}`,
    requestId: parsed.evaluationInput.requestId,
    action: outcome.action,
    confidence: outcome.confidence,
    unknownState: outcome.unknownState,
    degradedState: degradedStateFor(parsed.modelRuntime, unavailable),
    reasonCodes: outcome.reasonCodes,
    explanationReference: `local-ai-text-dry-run-explanation:${parsed.evaluationInput.requestId}`,
    evidenceReferences: parsed.evaluationInput.evidenceReferences,
    parentRuleReferences: parsed.evaluationInput.parentRuleReferences,
    memoryReferences: parsed.evaluationInput.memoryReferences,
    graphReferences: parsed.evaluationInput.graphReferences,
    modelRuntime: parsed.modelRuntime,
    promptVersion: parsed.evaluationInput.modelRequest.promptVersion,
    expiresAt: null,
  });
}

function degradedStateFor(
  runtime: LocalModelRuntimeStatus,
  unavailable: boolean
): typeof LocalAiDegradedStateSchema.Type {
  if (unavailable) {
    return LocalAiDegradedState.ProviderUnavailable;
  }
  return runtime.degradedState;
}

function stateFor(runtime: LocalModelRuntimeStatus): LocalAiTextInferenceDryRunState {
  if (runtime.executionState === 'disabled' || runtime.loadState === 'unavailable') {
    return 'unavailable-dry-run';
  }
  if (runtime.degradedState !== 'none' || runtime.executionState !== 'dry-run-ready') {
    return 'degraded-dry-run';
  }
  return 'ready-dry-run';
}

function localAiTextInferenceDryRunInputIsReady(candidate: LocalAiTextInferenceDryRunInputCandidate): boolean {
  return (
    !candidate.rawPromptRetained &&
    candidate.modelRuntime.privacyMode === 'local-only' &&
    candidate.modelRuntime.providerId === candidate.evaluationInput.modelRequest.providerId &&
    candidate.modelRuntime.modelId === candidate.evaluationInput.modelRequest.modelId
  );
}

function localAiTextInferenceDryRunResultIsHonest(result: LocalAiTextInferenceDryRunResultCandidate): boolean {
  if (!resultKeepsNonClaimBoundary(result) || !resultMatchesRuntimeBoundary(result)) {
    return false;
  }

  return resultStateMatchesOutcome(result);
}

function resultKeepsNonClaimBoundary(result: LocalAiTextInferenceDryRunResultCandidate): boolean {
  return (
    result.localOnly &&
    result.dryRunOnly &&
    !result.modelExecuted &&
    !result.remoteApiClaimed &&
    !result.policyAuthorityClaimed &&
    !result.enforcementClaimed &&
    !result.productionModelQualityClaimed &&
    !result.rawPromptRetained
  );
}

function resultMatchesRuntimeBoundary(result: LocalAiTextInferenceDryRunResultCandidate): boolean {
  return (
    result.modelRuntime.privacyMode === 'local-only' &&
    result.result.modelRuntime.runtimeReferenceId === result.modelRuntime.runtimeReferenceId &&
    result.result.modelRuntime.providerId === result.modelRuntime.providerId &&
    result.result.modelRuntime.modelId === result.modelRuntime.modelId &&
    result.result.promptVersion === result.promptVersion &&
    result.result.evidenceReferences.length === result.evidenceReferenceCount &&
    result.result.parentRuleReferences.length === result.parentRuleReferenceCount
  );
}

function resultStateMatchesOutcome(result: LocalAiTextInferenceDryRunResultCandidate): boolean {
  if (result.state === 'degraded-dry-run') {
    return result.result.degradedState !== 'none' || result.result.unknownState !== 'none';
  }

  if (result.evidenceReferenceCount === 0) {
    const missingEvidenceOutcome = dryRunOutcomeDefinitions['missing-evidence'];
    return (
      result.result.action === missingEvidenceOutcome.action &&
      result.result.unknownState === missingEvidenceOutcome.unknownState
    );
  }

  const stateDefinition = dryRunStateDefinitions[result.state];

  return (
    result.result.action === stateDefinition.action &&
    result.result.unknownState === stateDefinition.unknownState
  );
}
