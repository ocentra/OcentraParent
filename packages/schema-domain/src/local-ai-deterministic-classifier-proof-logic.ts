import { Schema, brandedNonEmptyStringSchema } from './effect';
import { LocalAiDegradedState, LocalAiUnknownState, type LocalAiContextKind } from './ai-primitives';
import { PolicyAction, PolicyReasonCodeSchema } from './policy-contracts';
import { type LocalAiSafetyResult } from './local-ai';
import { type LocalModelRuntimeStatus } from './ai-runtime';
import { ParentContractSchemaVersion } from './family-reference-primitives';

export const LocalAiDeterministicClassifierTraceRefSchema = brandedNonEmptyStringSchema(
  'LocalAiDeterministicClassifierTraceRef'
);
export const LocalAiDeterministicClassifierNonClaimSchema = brandedNonEmptyStringSchema(
  'LocalAiDeterministicClassifierNonClaim'
);

export const decodeTraceRef = Schema.decodeUnknownSync(LocalAiDeterministicClassifierTraceRefSchema);
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

interface LocalAiDeterministicClassifierInputCandidate {
  readonly rawEvidenceRetained: boolean;
  readonly modelRuntime: LocalModelRuntimeStatus;
  readonly evaluationInput: {
    readonly currentObservation: {
      readonly contextKind: LocalAiContextKind;
    };
    readonly evidenceReferences: readonly unknown[];
    readonly graphReferences: readonly unknown[];
    readonly memoryReferences: readonly unknown[];
    readonly modelRequest: {
      readonly modelId: string;
      readonly promptVersion: string;
      readonly providerId: string;
    };
    readonly parentRuleReferences: readonly unknown[];
    readonly requestId: string;
  };
}

interface LocalAiDeterministicClassifierResultCandidate {
  readonly classifierRunId: string;
  readonly deterministicOnly: boolean;
  readonly dryRun: boolean;
  readonly enforcementClaimed: boolean;
  readonly evidenceReferenceCount: number;
  readonly localOnly: boolean;
  readonly modelExecuted: boolean;
  readonly modelRuntime: LocalModelRuntimeStatus;
  readonly parentRuleReferenceCount: number;
  readonly productionModelQualityClaimed: boolean;
  readonly promptVersion: string;
  readonly rawEvidenceRetained: boolean;
  readonly remoteApiClaimed: boolean;
  readonly result: {
    readonly evidenceReferences: readonly unknown[];
    readonly modelRuntime: LocalModelRuntimeStatus;
    readonly parentRuleReferences: readonly unknown[];
    readonly promptVersion: string;
  };
  readonly state: string;
  readonly policyAuthorityClaimed: boolean;
}

export function resultFor(parsed: LocalAiDeterministicClassifierInputCandidate): LocalAiSafetyResult {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: `local-ai-deterministic-classifier-result:${parsed.evaluationInput.requestId}`,
    requestId: parsed.evaluationInput.requestId,
    action: actionFor(parsed),
    confidence: confidenceFor(parsed),
    unknownState: unknownStateFor(parsed),
    degradedState: degradedStateFor(parsed),
    reasonCodes: reasonCodesFor(parsed),
    explanationReference: `local-ai-deterministic-classifier-explanation:${parsed.evaluationInput.requestId}`,
    evidenceReferences: parsed.evaluationInput.evidenceReferences,
    parentRuleReferences: parsed.evaluationInput.parentRuleReferences,
    memoryReferences: parsed.evaluationInput.memoryReferences,
    graphReferences: parsed.evaluationInput.graphReferences,
    modelRuntime: parsed.modelRuntime,
    promptVersion: parsed.evaluationInput.modelRequest.promptVersion,
    expiresAt: null,
  };
}

export function stateFor(parsed: LocalAiDeterministicClassifierInputCandidate, result: LocalAiSafetyResult) {
  const stateEntries = [
    [runtimeUnavailable(parsed.modelRuntime), 'runtime-unavailable'],
    [parsed.evaluationInput.evidenceReferences.length === 0, 'missing-evidence'],
    [result.confidence < 0.6, 'low-confidence'],
  ] as const;

  return stateEntries.find(([matches]) => matches)?.[1] ?? 'classified';
}

export function deterministicClassifierInputIsReady(candidate: LocalAiDeterministicClassifierInputCandidate): boolean {
  return [
    !candidate.rawEvidenceRetained,
    candidate.modelRuntime.privacyMode === 'local-only',
    candidate.modelRuntime.providerId === candidate.evaluationInput.modelRequest.providerId,
    candidate.modelRuntime.modelId === candidate.evaluationInput.modelRequest.modelId,
  ].every(Boolean);
}

export function deterministicClassifierResultIsHonest(candidate: LocalAiDeterministicClassifierResultCandidate): boolean {
  return [
    deterministicClassifierKeepsNonClaimBoundary(candidate),
    candidate.dryRun,
    deterministicClassifierMatchesRuntime(candidate),
    deterministicClassifierMatchesReferenceCounts(candidate),
  ].every(Boolean);
}

function actionFor(candidate: LocalAiDeterministicClassifierInputCandidate): LocalAiSafetyResult['action'] {
  const contextKind = candidate.evaluationInput.currentObservation.contextKind;
  const actionEntries = [
    [runtimeUnavailable(candidate.modelRuntime), PolicyAction.AskParent],
    [candidate.evaluationInput.evidenceReferences.length === 0, PolicyAction.Unknown],
    [contextKind === 'network', PolicyAction.AskParent],
    [contextKind === 'app', PolicyAction.TimeLimit],
    [contextKind === 'process', PolicyAction.Block],
    [['video', 'url', 'domain', 'page'].includes(contextKind), PolicyAction.Warn],
  ] as const;

  return actionEntries.find(([matches]) => matches)?.[1] ?? PolicyAction.Allow;
}

function confidenceFor(candidate: LocalAiDeterministicClassifierInputCandidate): number {
  const contextKind = candidate.evaluationInput.currentObservation.contextKind;
  const confidenceEntries = [
    [runtimeUnavailable(candidate.modelRuntime), 0.2],
    [candidate.evaluationInput.evidenceReferences.length === 0, 0.1],
    [contextKind === 'network', 0.55],
    [['app', 'process'].includes(contextKind), 0.78],
    [['page', 'domain'].includes(contextKind), 0.58],
  ] as const;

  return confidenceEntries.find(([matches]) => matches)?.[1] ?? 0.82;
}

function unknownStateFor(
  candidate: LocalAiDeterministicClassifierInputCandidate
): (typeof LocalAiUnknownState)[keyof typeof LocalAiUnknownState] {
  const unknownEntries = [
    [runtimeUnavailable(candidate.modelRuntime), LocalAiUnknownState.ModelUnavailable],
    [candidate.evaluationInput.evidenceReferences.length === 0, LocalAiUnknownState.MissingEvidence],
  ] as const;

  return unknownEntries.find(([matches]) => matches)?.[1] ?? LocalAiUnknownState.None;
}

function degradedStateFor(
  candidate: LocalAiDeterministicClassifierInputCandidate
): (typeof LocalAiDegradedState)[keyof typeof LocalAiDegradedState] {
  const contextKind = candidate.evaluationInput.currentObservation.contextKind;
  const degradedEntries = [
    [runtimeUnavailable(candidate.modelRuntime), LocalAiDegradedState.ProviderUnavailable],
    [['network', 'page', 'domain'].includes(contextKind), LocalAiDegradedState.InvalidOutput],
  ] as const;

  return degradedEntries.find(([matches]) => matches)?.[1] ?? candidate.modelRuntime.degradedState;
}

function reasonCodesFor(
  candidate: LocalAiDeterministicClassifierInputCandidate
): LocalAiSafetyResult['reasonCodes'] {
  const contextKind = candidate.evaluationInput.currentObservation.contextKind;
  const reasonEntries = [
    [runtimeUnavailable(candidate.modelRuntime), [RuntimeUnavailableReasonCode]],
    [candidate.evaluationInput.evidenceReferences.length === 0, [MissingEvidenceReasonCode]],
    [contextKind === 'network', [NetworkReviewReasonCode, LowConfidenceReasonCode]],
    [['page', 'domain'].includes(contextKind), [VideoReasonCode, LowConfidenceReasonCode]],
    [contextKind === 'app', [AppTimeLimitReasonCode]],
    [contextKind === 'process', [ProcessBlockReasonCode]],
    [['video', 'url'].includes(contextKind), [VideoReasonCode]],
  ] as const;

  return reasonEntries.find(([matches]) => matches)?.[1] ?? [SafeProductivityReasonCode];
}

function runtimeUnavailable(runtime: LocalModelRuntimeStatus): boolean {
  return [
    runtime.executionState === 'disabled',
    runtime.loadState === 'unavailable',
    !runtime.capabilityFlags.includes('classification'),
  ].some(Boolean);
}

function deterministicClassifierKeepsNonClaimBoundary(candidate: LocalAiDeterministicClassifierResultCandidate): boolean {
  return [
    candidate.deterministicOnly,
    candidate.localOnly,
    !candidate.modelExecuted,
    !candidate.remoteApiClaimed,
    !candidate.policyAuthorityClaimed,
    !candidate.enforcementClaimed,
    !candidate.productionModelQualityClaimed,
    !candidate.rawEvidenceRetained,
  ].every(Boolean);
}

function deterministicClassifierMatchesRuntime(candidate: LocalAiDeterministicClassifierResultCandidate): boolean {
  return [
    candidate.result.modelRuntime.providerId === candidate.modelRuntime.providerId,
    candidate.result.modelRuntime.modelId === candidate.modelRuntime.modelId,
    candidate.result.promptVersion === candidate.promptVersion,
  ].every(Boolean);
}

function deterministicClassifierMatchesReferenceCounts(candidate: LocalAiDeterministicClassifierResultCandidate): boolean {
  return [
    candidate.result.evidenceReferences.length === candidate.evidenceReferenceCount,
    candidate.result.parentRuleReferences.length === candidate.parentRuleReferenceCount,
  ].every(Boolean);
}
