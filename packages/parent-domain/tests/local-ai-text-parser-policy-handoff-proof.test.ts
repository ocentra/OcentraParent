import { describe, expect, it } from 'vitest';
import { proveLocalAiTextLlmAdapterBoundary } from '../src/local-ai-text-llm-adapter-boundary-proof';
import { parseLocalAiTextOutput } from '../src/local-ai-text-output-parser-proof';
import { buildLocalAiTextParserReadModelSnapshot } from '../src/local-ai-text-parser-read-model-proof';
import {
  LocalAiTextParserPolicyHandoffProofSchema,
  LocalAiTextParserPolicyHandoffRowSchema,
  buildLocalAiTextParserPolicyHandoffProof,
} from '../src/local-ai-text-parser-policy-handoff-proof';

describe('local AI text parser policy handoff proof', () => {
  it('feeds ready local text parser rows into dry-run policy decisions', provesReadyPolicyDecision);
  it('keeps rejected and manual parser rows out of policy decisions', provesManualRowsStayManual);
  it('rejects policy handoff overclaims and mismatched counts', provesOverclaimRejections);
});

const observedAt = '2026-06-06T14:45:00.000Z';
const sourceProofRefs = ['proof:local-ai-text-parser-read-model'];

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

function provesReadyPolicyDecision() {
  const proof = buildPolicyHandoffProof([readModelRows().ready]);
  const row = proof.rows[0];

  expect(proof.policyReadyRowCount).toBe(1);
  expect(proof.manualRequiredRowCount).toBe(0);
  expect(row?.handoffState).toBe('policy-dry-run-ready');
  expect(row?.policyDecision?.action).toBe('warn');
  expect(row?.policyDecision?.dryRun).toBe(true);
  expect(row?.policyDecision?.enforcementHandoffState).toBe('disabled');
  expect(row?.policyDecision?.localAiResultId).toBe('local-ai-text-result:screen-summary-wiki-ocr');
}

function provesManualRowsStayManual() {
  const rows = readModelRows();
  const proof = buildPolicyHandoffProof([rows.malformed, rows.manual]);

  expect(proof.policyReadyRowCount).toBe(0);
  expect(proof.manualRequiredRowCount).toBe(2);
  expect(proof.rows.every((row) => row.policyDecision === null)).toBe(true);
  expect(proof.rows.every((row) => row.resultPolicyEligible === false)).toBe(true);
  expect(proof.rows.every((row) => row.policyDecisionHandoffState === 'not-requested')).toBe(true);
}

function provesOverclaimRejections() {
  const proof = buildPolicyHandoffProof([readModelRows().ready]);
  const row = proof.rows[0];

  expect(() => LocalAiTextParserPolicyHandoffRowSchema.parse({ ...row, modelExecuted: true })).toThrow();
  expect(() => LocalAiTextParserPolicyHandoffRowSchema.parse({ ...row, rawModelOutputRetained: true })).toThrow();
  expect(() => LocalAiTextParserPolicyHandoffRowSchema.parse({ ...row, policyAuthorityClaimed: true })).toThrow();
  expect(() => LocalAiTextParserPolicyHandoffRowSchema.parse({ ...row, enforcementClaimed: true })).toThrow();
  expect(() => LocalAiTextParserPolicyHandoffProofSchema.parse({ ...proof, policyReadyRowCount: 0 })).toThrow();
}

function buildPolicyHandoffProof(readModelRowsInput: readonly unknown[]) {
  return buildLocalAiTextParserPolicyHandoffProof({
    generatedAt: observedAt,
    proofId: 'local-ai-text-parser-policy-handoff:wiki-ocr',
    sourceProofRefs,
    readModelRows: readModelRowsInput,
  });
}

function readModelRows() {
  const snapshot = buildLocalAiTextParserReadModelSnapshot({
    generatedAt: observedAt,
    snapshotId: 'local-ai-text-parser-read-model:snapshot:policy',
    sourceProofRefs,
    parserProofs: [parsedProof(), malformedProof(), manualAdapterProof()],
  });

  return {
    ready: snapshot.rows[0],
    malformed: snapshot.rows[1],
    manual: snapshot.rows[2],
  };
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
  return parseLocalAiTextOutput(
    parserInput(
      validCandidateOutput(),
      proveLocalAiTextLlmAdapterBoundary({
        ...adapterInput(),
        localAdapterAvailable: false,
      })
    )
  );
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
