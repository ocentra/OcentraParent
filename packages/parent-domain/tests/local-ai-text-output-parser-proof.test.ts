import { describe, expect, it } from 'vitest';
import {
  LocalAiTextOutputParserInputSchema,
  LocalAiTextOutputParserProofSchema,
  parseLocalAiTextOutput,
} from '../src/local-ai-text-output-parser-proof';
import { proveLocalAiTextLlmAdapterBoundary } from '../src/local-ai-text-llm-adapter-boundary-proof';

describe('local AI text output parser proof', () => {
  it('parses schema-valid local text output into a policy-eligible local AI result', provesParsedOutput);
  it('rejects malformed and remote output before policy eligibility', provesInvalidOutputRejection);
  it('keeps manual adapter rows out of policy eligibility', provesManualAdapterBoundary);
  it('rejects raw output retention and parser overclaims', provesOverclaimRejections);
});

const observedAt = '2026-06-06T12:40:00.000Z';

const evidenceReference = {
  evidenceReferenceId: 'evidence:screen-summary:wiki-ocr',
  kind: 'query-store-summary',
  observedAt,
};

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
  lastCheckedAt: observedAt,
  unavailableReason: null,
};

function adapterInput() {
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
        evidence: evidenceReference,
      },
      evidenceReferences: [evidenceReference],
      parentRuleReferences: ['policy-rule:video-warn'],
      recentActivityWindow: [evidenceReference],
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: readyRuntime.providerId,
        modelId: readyRuntime.modelId,
        promptVersion: 'prompt:screen-safety:v1',
      },
    },
  };
}

function validCandidateOutput() {
  return {
    schemaVersion: 'v0.6',
    resultId: 'local-ai-text-result:screen-summary-wiki-ocr',
    requestId: 'local-ai-eval:screen-summary-wiki-ocr',
    action: 'warn',
    confidence: 0.68,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: ['local-ai-text:screen-video-risk'],
    explanationReference: 'local-ai-text-explanation:screen-summary-wiki-ocr',
    evidenceReferences: [evidenceReference],
    parentRuleReferences: ['policy-rule:video-warn'],
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: readyRuntime,
    promptVersion: 'prompt:screen-safety:v1',
    expiresAt: null,
  };
}

function parserInput(candidateOutput: unknown, adapterProof = proveLocalAiTextLlmAdapterBoundary(adapterInput())) {
  return {
    schemaVersion: 'v0.6',
    parserRunId: 'local-ai-text-parser:screen-summary-wiki-ocr',
    adapterProof,
    candidateOutput,
    rawModelOutputRetained: false,
  };
}

function provesParsedOutput() {
  const proof = parseLocalAiTextOutput(parserInput(validCandidateOutput()));

  expect(proof.state).toBe('parsed-local-result');
  expect(proof.parserRejectedOutput).toBe(false);
  expect(proof.resultPolicyEligible).toBe(true);
  expect(proof.result?.action).toBe('warn');
  expect(proof.result?.modelRuntime.privacyMode).toBe('local-only');
  expect(proof.rawModelOutputRetained).toBe(false);
}

function provesInvalidOutputRejection() {
  const malformed = parseLocalAiTextOutput(parserInput({ ...validCandidateOutput(), action: 'silently-allow' }));
  const remote = parseLocalAiTextOutput(
    parserInput({
      ...validCandidateOutput(),
      modelRuntime: {
        ...readyRuntime,
        privacyMode: 'remote-api',
      },
    })
  );
  const mismatchedPrompt = parseLocalAiTextOutput(
    parserInput({ ...validCandidateOutput(), promptVersion: 'prompt:other:v1' })
  );

  for (const proof of [malformed, remote, mismatchedPrompt]) {
    expect(proof.state).toBe('rejected-invalid-output');
    expect(proof.result).toBeNull();
    expect(proof.resultPolicyEligible).toBe(false);
  }
}

function provesManualAdapterBoundary() {
  const manualAdapter = proveLocalAiTextLlmAdapterBoundary({
    ...adapterInput(),
    localAdapterAvailable: false,
  });
  const proof = parseLocalAiTextOutput(parserInput(validCandidateOutput(), manualAdapter));

  expect(proof.state).toBe('manual-required');
  expect(proof.result).toBeNull();
  expect(proof.resultPolicyEligible).toBe(false);
}

function provesOverclaimRejections() {
  const input = parserInput(validCandidateOutput());
  const proof = parseLocalAiTextOutput(input);

  expect(() => LocalAiTextOutputParserInputSchema.parse({ ...input, rawModelOutputRetained: true })).toThrow();
  expect(() => LocalAiTextOutputParserProofSchema.parse({ ...proof, modelExecuted: true })).toThrow();
  expect(() => LocalAiTextOutputParserProofSchema.parse({ ...proof, remoteApiClaimed: true })).toThrow();
  expect(() => LocalAiTextOutputParserProofSchema.parse({ ...proof, policyAuthorityClaimed: true })).toThrow();
  expect(() => LocalAiTextOutputParserProofSchema.parse({ ...proof, rawModelOutputRetained: true })).toThrow();
}
