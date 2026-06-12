import { describe, expect, it } from 'vitest';
import {
  LocalAiDeterministicClassifierInputSchema,
  LocalAiDeterministicClassifierResultSchema,
  runLocalAiDeterministicClassifier,
} from '../../src/local-ai-deterministic-classifier-proof';

describe('local AI deterministic classifier proof', () => {
  it('classifies video evidence into a local-only warning result without model execution', provesVideoWarning);
  it('classifies productivity evidence into a local-only allow dry-run result', provesProductivityAllow);
  it('classifies app and process evidence into time-limit and block dry-run rows', provesTimeLimitAndBlock);
  it('routes lower-confidence network evidence to ask-parent review', provesNetworkReview);
  it('keeps missing evidence as a typed unknown result', provesMissingEvidence);
  it('degrades unavailable runtime without policy authority or enforcement', provesRuntimeUnavailable);
  it('rejects raw evidence retention, runtime mismatch, and overclaimed output', provesOverclaimRejection);
});

const ObservedAt = '2026-06-06T07:30:00.000Z';
const ChildProfile = {
  childProfileId: 'child:maya',
  displayName: 'Maya',
};
const Device = {
  deviceId: 'device:maya-windows',
  childProfileId: 'child:maya',
  label: 'Maya Windows laptop',
  platform: 'windows',
};
const Runtime = {
  runtimeReferenceId: 'local-ai-runtime-deterministic-classifier',
  providerId: 'local-provider-deterministic-classifier',
  modelId: 'deterministic-classifier-v1',
  modelReference: 'artifact:deterministic_classifier_v1',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['classification', 'safety-decision'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: ObservedAt,
  unavailableReason: null,
};

function provesVideoWarning() {
  const proof = runLocalAiDeterministicClassifier(classifierInput('video'));

  expect(proof).toMatchObject({
    state: 'classified',
    contextKind: 'video',
    dryRun: true,
    deterministicOnly: true,
    localOnly: true,
    modelExecuted: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    productionModelQualityClaimed: false,
    rawEvidenceRetained: false,
  });
  expect(proof.result).toMatchObject({
    action: 'warn',
    confidence: 0.82,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: ['local-ai-deterministic-video-warning'],
  });
  expect(proof.result.evidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual(['evidence:video']);
  expect(proof.result.parentRuleReferences).toEqual(['policy-rule:screen-video-warn']);
}

function provesProductivityAllow() {
  const proof = runLocalAiDeterministicClassifier(classifierInput('window'));

  expect(proof).toMatchObject({
    state: 'classified',
    contextKind: 'window',
    dryRun: true,
    enforcementClaimed: false,
  });
  expect(proof.result).toMatchObject({
    action: 'allow',
    confidence: 0.82,
    reasonCodes: ['local-ai-deterministic-productivity-allow'],
  });
}

function provesTimeLimitAndBlock() {
  const timeLimit = runLocalAiDeterministicClassifier(classifierInput('app'));
  const block = runLocalAiDeterministicClassifier(classifierInput('process'));

  expect(timeLimit).toMatchObject({
    state: 'classified',
    contextKind: 'app',
    dryRun: true,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
  });
  expect(timeLimit.result).toMatchObject({
    action: 'time-limit',
    reasonCodes: ['local-ai-deterministic-app-time-limit'],
  });
  expect(block).toMatchObject({
    state: 'classified',
    contextKind: 'process',
    dryRun: true,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
  });
  expect(block.result).toMatchObject({
    action: 'block',
    reasonCodes: ['local-ai-deterministic-process-block'],
  });
}

function provesNetworkReview() {
  const proof = runLocalAiDeterministicClassifier(classifierInput('network'));

  expect(proof).toMatchObject({
    state: 'low-confidence',
    contextKind: 'network',
  });
  expect(proof.result).toMatchObject({
    action: 'ask-parent',
    confidence: 0.55,
    degradedState: 'invalid-output',
    reasonCodes: ['local-ai-deterministic-network-review', 'local-ai-deterministic-low-confidence'],
  });
  expect(proof.policyAuthorityClaimed).toBe(false);
}

function provesMissingEvidence() {
  const input = classifierInput('window');
  const proof = runLocalAiDeterministicClassifier({
    ...input,
    evaluationInput: {
      ...input.evaluationInput,
      evidenceReferences: [],
    },
  });

  expect(proof).toMatchObject({
    state: 'missing-evidence',
    evidenceReferenceCount: 0,
  });
  expect(proof.result).toMatchObject({
    action: 'unknown',
    confidence: 0.1,
    unknownState: 'missing-evidence',
    reasonCodes: ['local-ai-deterministic-missing-evidence'],
  });
}

function provesRuntimeUnavailable() {
  const proof = runLocalAiDeterministicClassifier({
    ...classifierInput('video'),
    modelRuntime: {
      ...Runtime,
      executionState: 'disabled',
      loadState: 'unavailable',
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-deterministic-classifier-unavailable',
    },
  });

  expect(proof).toMatchObject({
    state: 'runtime-unavailable',
    modelExecuted: false,
    enforcementClaimed: false,
  });
  expect(proof.result).toMatchObject({
    action: 'ask-parent',
    unknownState: 'model-unavailable',
    degradedState: 'provider-unavailable',
    reasonCodes: ['local-ai-deterministic-runtime-unavailable'],
  });
}

function provesOverclaimRejection() {
  const input = classifierInput('video');
  const proof = runLocalAiDeterministicClassifier(input);

  expect(() => LocalAiDeterministicClassifierInputSchema.parse({ ...input, rawEvidenceRetained: true })).toThrow();
  expect(() =>
    LocalAiDeterministicClassifierInputSchema.parse({
      ...input,
      modelRuntime: {
        ...Runtime,
        modelId: 'other-deterministic-classifier',
      },
    })
  ).toThrow();
  expect(() => LocalAiDeterministicClassifierResultSchema.parse({ ...proof, modelExecuted: true })).toThrow();
  expect(() => LocalAiDeterministicClassifierResultSchema.parse({ ...proof, enforcementClaimed: true })).toThrow();
  expect(() => LocalAiDeterministicClassifierResultSchema.parse({ ...proof, dryRun: false })).toThrow();
}

function classifierInput(contextKind: string) {
  const evidence = evidenceReference(contextKind);
  return {
    schemaVersion: 'v0.6',
    classifierRunId: `local-ai-deterministic-classifier:${contextKind}`,
    rawEvidenceRetained: false,
    modelRuntime: Runtime,
    evaluationInput: {
      schemaVersion: 'v0.6',
      requestId: `local-ai-eval:deterministic-classifier:${contextKind}`,
      childProfile: ChildProfile,
      device: Device,
      currentObservation: {
        contextKind,
        evidence,
      },
      evidenceReferences: [evidence],
      parentRuleReferences: ['policy-rule:screen-video-warn'],
      recentActivityWindow: [evidence],
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: Runtime.providerId,
        modelId: Runtime.modelId,
        promptVersion: 'prompt:deterministic-classifier:v1',
      },
    },
  };
}

function evidenceReference(contextKind: string) {
  return {
    evidenceReferenceId: `evidence:${contextKind}`,
    kind: 'query-store-summary',
    observedAt: ObservedAt,
  };
}
