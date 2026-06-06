import { describe, expect, it } from 'vitest';
import {
  LocalAiTextInferenceDryRunInputSchema,
  LocalAiTextInferenceDryRunResultSchema,
  runLocalAiTextInferenceDryRun,
} from '../src/local-ai-text-inference-dry-run-proof';

describe('local AI text inference dry-run proof', () => {
  it('emits a local-only schema-valid dry-run safety result without executing a model', provesReadyDryRun);
  it(
    'degrades unavailable local runtime to ask-parent without policy authority or enforcement',
    provesUnavailableDryRun
  );
  it('keeps missing evidence as a typed unknown dry-run result', provesMissingEvidenceDryRun);
  it('rejects mismatched runtime metadata, retained raw prompts, and overclaimed results', provesOverclaimsReject);
});

const readyRuntime = {
  runtimeReferenceId: 'local-ai-runtime-local-llama-cli',
  providerId: 'local-provider-llama-cli',
  modelId: 'gemma-4-e2b-it-q4-k-m',
  modelReference: 'artifact:gemma_4_e2b_it_q4_k_m',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['classification', 'safety-decision'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: '2026-06-06T04:00:00.000Z',
  unavailableReason: null,
};

function readyDryRunInput() {
  return {
    schemaVersion: 'v0.6',
    dryRunId: 'local-ai-text-dry-run:screen-summary-wiki-ocr',
    rawPromptRetained: false,
    modelRuntime: readyRuntime,
    evaluationInput: {
      schemaVersion: 'v0.6',
      requestId: 'local-ai-eval:screen-summary-wiki-ocr',
      childProfile: {
        childProfileId: 'child:maya',
        displayName: 'Maya',
      },
      device: {
        deviceId: 'device:maya-windows',
        childProfileId: 'child:maya',
        label: 'Maya Windows laptop',
        platform: 'windows',
      },
      currentObservation: {
        observationReferenceId: 'observation:screen-summary-wiki-ocr',
        contextKind: 'page',
        evidence: {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      },
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      ],
      parentRuleReferences: ['policy-rule:video-warn'],
      recentActivityWindow: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T04:00:00.000Z',
        },
      ],
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: 'local-provider-llama-cli',
        modelId: 'gemma-4-e2b-it-q4-k-m',
        promptVersion: 'prompt:screen-safety:v1',
      },
    },
  };
}

function provesReadyDryRun() {
  const proof = runLocalAiTextInferenceDryRun(readyDryRunInput());

  expect(proof).toMatchObject({
    state: 'ready-dry-run',
    localOnly: true,
    dryRunOnly: true,
    modelExecuted: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    productionModelQualityClaimed: false,
    rawPromptRetained: false,
  });
  expect(proof.result).toMatchObject({
    action: 'warn',
    confidence: 0.62,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: ['local-ai-text-dry-run-candidate'],
  });
  expect(proof.result.evidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
    'evidence:screen-summary:wiki-ocr',
  ]);
  expect(proof.result.parentRuleReferences).toEqual(['policy-rule:video-warn']);
}

function provesUnavailableDryRun() {
  const proof = runLocalAiTextInferenceDryRun({
    ...readyDryRunInput(),
    modelRuntime: {
      ...readyRuntime,
      executionState: 'disabled',
      loadState: 'unavailable',
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-provider-unconfigured',
    },
  });

  expect(proof.state).toBe('unavailable-dry-run');
  expect(proof.result).toMatchObject({
    action: 'ask-parent',
    unknownState: 'model-unavailable',
    degradedState: 'provider-unavailable',
    reasonCodes: ['local-ai-text-runtime-unavailable'],
  });
  expect(proof.enforcementClaimed).toBe(false);
  expect(proof.policyAuthorityClaimed).toBe(false);
}

function provesMissingEvidenceDryRun() {
  const input = readyDryRunInput();
  const proof = runLocalAiTextInferenceDryRun({
    ...input,
    evaluationInput: {
      ...input.evaluationInput,
      evidenceReferences: [],
    },
  });

  expect(proof.state).toBe('ready-dry-run');
  expect(proof.result).toMatchObject({
    action: 'unknown',
    unknownState: 'missing-evidence',
    reasonCodes: ['local-ai-text-missing-evidence'],
  });
  expect(proof.evidenceReferenceCount).toBe(0);
}

function provesOverclaimsReject() {
  const input = readyDryRunInput();
  const proof = runLocalAiTextInferenceDryRun(input);

  expect(() => LocalAiTextInferenceDryRunInputSchema.parse({ ...input, rawPromptRetained: true })).toThrow();
  expect(() =>
    LocalAiTextInferenceDryRunInputSchema.parse({
      ...input,
      modelRuntime: {
        ...readyRuntime,
        providerId: 'local-provider-other',
      },
    })
  ).toThrow();
  expect(() => LocalAiTextInferenceDryRunResultSchema.parse({ ...proof, modelExecuted: true })).toThrow();
  expect(() => LocalAiTextInferenceDryRunResultSchema.parse({ ...proof, remoteApiClaimed: true })).toThrow();
}
