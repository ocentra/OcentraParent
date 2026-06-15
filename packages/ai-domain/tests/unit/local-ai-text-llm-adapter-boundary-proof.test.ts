import { describe, expect, it } from 'vitest';
import {
  LocalAiTextLlmAdapterBoundaryInputSchema,
  LocalAiTextLlmAdapterBoundaryProofSchema,
  proveLocalAiTextLlmAdapterBoundary,
} from '../../src/local-ai-text-llm-adapter-boundary-proof';

describe('local AI text LLM adapter boundary proof', () => {
  it('marks schema-valid local runtime input ready for the local adapter without executing a model', provesReady);
  it('keeps unavailable runtimes and manual adapter gaps out of ready state', provesUnavailableAndManual);
  it('rejects retained raw material and runtime metadata mismatches', provesInputRejections);
  it('rejects remote, policy, enforcement, model-quality, and raw-output overclaims', provesProofRejections);
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
  lastCheckedAt: '2026-06-06T12:24:00.000Z',
  unavailableReason: null,
};

function readyInput() {
  return {
    schemaVersion: 'v0.6',
    adapterRequestId: 'local-ai-text-adapter:screen-summary-wiki-ocr',
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    localAdapterAvailable: true,
    manualProofRequired: false,
    modelRuntime: readyRuntime,
    promptVersion: 'prompt:screen-safety:v1',
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
          observedAt: '2026-06-06T12:24:00.000Z',
        },
      },
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T12:24:00.000Z',
        },
      ],
      parentRuleReferences: ['policy-rule:video-warn'],
      recentActivityWindow: [
        {
          evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
          kind: 'query-store-summary',
          observedAt: '2026-06-06T12:24:00.000Z',
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

function provesReady() {
  const proof = proveLocalAiTextLlmAdapterBoundary(readyInput());

  expect(proof.state).toBe('ready-for-local-adapter');
  expect(proof).toMatchObject({
    localOnly: true,
    adapterBoundaryOnly: true,
    parserRequiredBeforeResult: true,
    modelExecuted: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    productionModelQualityClaimed: false,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    evidenceReferenceCount: 1,
    parentRuleReferenceCount: 1,
    promptVersion: 'prompt:screen-safety:v1',
  });
  expect(proof.adapterTraceRefs).toEqual(['local-ai-text-adapter:local-ai-eval:screen-summary-wiki-ocr']);
  expect(proof.parserRefs).toEqual(['local-ai-text-parser:prompt:screen-safety:v1']);
}

function provesUnavailableAndManual() {
  const unavailable = proveLocalAiTextLlmAdapterBoundary({
    ...readyInput(),
    modelRuntime: {
      ...readyRuntime,
      executionState: 'disabled',
      loadState: 'unavailable',
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-provider-unconfigured',
    },
  });
  const manual = proveLocalAiTextLlmAdapterBoundary({ ...readyInput(), localAdapterAvailable: false });

  expect(unavailable.state).toBe('unavailable');
  expect(manual.state).toBe('manual-required');
  expect(unavailable.modelExecuted).toBe(false);
  expect(manual.policyAuthorityClaimed).toBe(false);
}

function provesInputRejections() {
  const input = readyInput();

  expect(() => LocalAiTextLlmAdapterBoundaryInputSchema.parse({ ...input, rawPromptRetained: true })).toThrow();
  expect(() => LocalAiTextLlmAdapterBoundaryInputSchema.parse({ ...input, rawModelOutputRetained: true })).toThrow();
  expect(() =>
    LocalAiTextLlmAdapterBoundaryInputSchema.parse({ ...input, promptVersion: 'prompt:other:v1' })
  ).toThrow();
  expect(() =>
    LocalAiTextLlmAdapterBoundaryInputSchema.parse({
      ...input,
      modelRuntime: {
        ...readyRuntime,
        providerId: 'remote-provider-openai',
      },
    })
  ).toThrow();
}

function provesProofRejections() {
  const proof = proveLocalAiTextLlmAdapterBoundary(readyInput());

  expect(() => LocalAiTextLlmAdapterBoundaryProofSchema.parse({ ...proof, modelExecuted: true })).toThrow();
  expect(() => LocalAiTextLlmAdapterBoundaryProofSchema.parse({ ...proof, remoteApiClaimed: true })).toThrow();
  expect(() => LocalAiTextLlmAdapterBoundaryProofSchema.parse({ ...proof, policyAuthorityClaimed: true })).toThrow();
  expect(() => LocalAiTextLlmAdapterBoundaryProofSchema.parse({ ...proof, enforcementClaimed: true })).toThrow();
  expect(() => LocalAiTextLlmAdapterBoundaryProofSchema.parse({ ...proof, rawModelOutputRetained: true })).toThrow();
}
