import { describe, expect, it } from 'vitest';
import { proveLocalAiTextLlmAdapterBoundary } from '../src/local-ai-text-llm-adapter-boundary-proof';
import { parseLocalAiTextOutput } from '../src/local-ai-text-output-parser-proof';
import {
  LocalAiTextParserReadModelRowSchema,
  LocalAiTextParserReadModelSnapshotSchema,
  buildLocalAiTextParserReadModelSnapshot,
} from '../src/local-ai-text-parser-read-model-proof';

describe('local AI text parser read-model proof', () => {
  it('projects parsed local text output into a ready parent-visible read-model row', provesReadyProjection);
  it('keeps rejected and manual parser rows manual-required without policy eligibility', provesManualRows);
  it('rejects raw output retention and read-model authority overclaims', provesOverclaimRejections);
});

const observedAt = '2026-06-06T14:25:00.000Z';
const sourceProofRefs = ['proof:local-ai-text-output-parser'];

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

function provesReadyProjection() {
  const snapshot = buildLocalAiTextParserReadModelSnapshot({
    generatedAt: observedAt,
    snapshotId: 'local-ai-text-parser-read-model:snapshot:wiki-ocr',
    sourceProofRefs,
    parserProofs: [parsedProof()],
  });

  expect(snapshot.readyRowCount).toBe(1);
  expect(snapshot.manualRequiredRowCount).toBe(0);
  expect(snapshot.rejectedParserRowCount).toBe(0);
  expect(snapshot.rows[0]?.readModelState).toBe('ready');
  expect(snapshot.rows[0]?.action).toBe('warn');
  expect(snapshot.rows[0]?.runtimeReferenceId).toBe(readyRuntime.runtimeReferenceId);
  expect(snapshot.rows[0]?.sourceProofRefs).toEqual(sourceProofRefs);
}

function provesManualRows() {
  const snapshot = buildLocalAiTextParserReadModelSnapshot({
    generatedAt: observedAt,
    snapshotId: 'local-ai-text-parser-read-model:snapshot:manual',
    sourceProofRefs,
    parserProofs: [malformedProof(), manualAdapterProof()],
  });

  expect(snapshot.readyRowCount).toBe(0);
  expect(snapshot.manualRequiredRowCount).toBe(2);
  expect(snapshot.rejectedParserRowCount).toBe(2);
  expect(snapshot.rows.map((row) => row.readModelState)).toEqual(['manual-required', 'manual-required']);
  expect(snapshot.rows.every((row) => row.resultPolicyEligible === false)).toBe(true);
  expect(snapshot.rows.every((row) => row.manualRequiredReasons.length > 0)).toBe(true);
}

function provesOverclaimRejections() {
  const snapshot = buildLocalAiTextParserReadModelSnapshot({
    generatedAt: observedAt,
    snapshotId: 'local-ai-text-parser-read-model:snapshot:overclaim',
    sourceProofRefs,
    parserProofs: [parsedProof()],
  });
  const row = snapshot.rows[0];

  expect(() => LocalAiTextParserReadModelRowSchema.parse({ ...row, rawModelOutputRetained: true })).toThrow();
  expect(() => LocalAiTextParserReadModelRowSchema.parse({ ...row, modelExecuted: true })).toThrow();
  expect(() => LocalAiTextParserReadModelRowSchema.parse({ ...row, policyAuthorityClaimed: true })).toThrow();
  expect(() => LocalAiTextParserReadModelRowSchema.parse({ ...row, enforcementClaimed: true })).toThrow();
  expect(() => LocalAiTextParserReadModelSnapshotSchema.parse({ ...snapshot, readyRowCount: 0 })).toThrow();
}

function parsedProof() {
  return parseLocalAiTextOutput(parserInput(validCandidateOutput(), readyAdapterProof()));
}

function malformedProof() {
  return parseLocalAiTextOutput(
    parserInput({ ...validCandidateOutput(), action: 'silently-allow' }, readyAdapterProof())
  );
}

function manualAdapterProof() {
  const manualAdapter = proveLocalAiTextLlmAdapterBoundary({
    ...adapterInput(),
    localAdapterAvailable: false,
  });
  return parseLocalAiTextOutput(parserInput(validCandidateOutput(), manualAdapter));
}

function readyAdapterProof() {
  return proveLocalAiTextLlmAdapterBoundary(adapterInput());
}

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

function parserInput(candidateOutput: unknown, adapterProof = readyAdapterProof()) {
  return {
    schemaVersion: 'v0.6',
    parserRunId: 'local-ai-text-parser:screen-summary-wiki-ocr',
    adapterProof,
    candidateOutput,
    rawModelOutputRetained: false,
  };
}
