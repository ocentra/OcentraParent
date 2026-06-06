import { describe, expect, it } from 'vitest';
import { buildLocalAiPromptTemplateVersionProof } from '../src/local-ai-prompt-template-version-proof';

const childProfile = { childProfileId: 'child-prompt-template-proof', displayName: 'Sam' };
const device = {
  deviceId: 'device-prompt-template-proof',
  childProfileId: 'child-prompt-template-proof',
  label: 'Sam Windows PC',
  platform: 'windows',
};
const evidence = {
  evidenceReferenceId: 'evidence-prompt-template-proof',
  kind: 'query-store-summary',
  observedAt: '2026-06-06T08:00:00.000Z',
};
const parentRule = 'rule-prompt-template-proof';
const modelRequest = {
  providerId: 'provider-local-prompt-template-proof',
  modelId: 'model-local-prompt-template-proof',
  promptVersion: 'prompt-template-screen-safety-v1',
};
const runtimeStatus = {
  runtimeReferenceId: 'runtime-local-prompt-template-proof',
  providerId: modelRequest.providerId,
  modelId: modelRequest.modelId,
  modelReference: 'model-ref-local-prompt-template-proof',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['classification', 'safety-decision'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: '2026-06-06T08:01:00.000Z',
  unavailableReason: null,
};
const evaluationInput = {
  schemaVersion: 'v0.6',
  requestId: 'request-prompt-template-proof',
  childProfile,
  device,
  currentObservation: {
    contextKind: 'page',
    evidence,
  },
  evidenceReferences: [evidence],
  parentRuleReferences: [parentRule],
  recentActivityWindow: [evidence],
  memoryReferences: [],
  graphReferences: [],
  modelRequest,
};
const safetyResult = {
  schemaVersion: 'v0.6',
  resultId: 'result-prompt-template-proof',
  requestId: evaluationInput.requestId,
  action: 'warn',
  confidence: 0.82,
  unknownState: 'none',
  degradedState: 'none',
  reasonCodes: ['screen-category-video'],
  explanationReference: 'explanation-prompt-template-proof',
  evidenceReferences: [evidence],
  parentRuleReferences: [parentRule],
  memoryReferences: [],
  graphReferences: [],
  modelRuntime: runtimeStatus,
  promptVersion: modelRequest.promptVersion,
  expiresAt: null,
};
const activePromptRecord = {
  schemaVersion: 'v0.6',
  promptVersion: modelRequest.promptVersion,
  lifecycleState: 'active',
  compatibleModelIds: [modelRequest.modelId],
  compatibleRuntimeRefs: [runtimeStatus.runtimeReferenceId],
  taskRequirements: ['classification', 'safety-decision'],
  evidenceReferences: [evidence],
  parentRuleReferences: [parentRule],
  generatedAt: '2026-06-06T07:55:00.000Z',
  validFrom: '2026-06-06T07:55:00.000Z',
  validUntil: null,
  supersededByPromptVersion: null,
  rawPromptRetained: false,
  rawTemplateTextRetained: false,
  remoteAiRequired: false,
};
const claimBoundaries = {
  remoteAiUsed: false,
  apiAiUsed: false,
  rawPromptRetained: false,
  rawTemplateTextRetained: false,
  modelQualityClaimed: false,
  policyAuthorityClaimed: false,
  enforcementClaimed: false,
  uiClaimed: false,
};

function proofInput(promptRecord = activePromptRecord): unknown {
  return {
    schemaVersion: 'v0.6',
    evaluationInput,
    safetyResult,
    runtimeStatus,
    promptRecords: [promptRecord],
    claimBoundaries,
  };
}

describe('local AI prompt template version proof', () => {
  it('selects one active local-only prompt template version for a matching request and result', () => {
    const proof = buildLocalAiPromptTemplateVersionProof(proofInput());

    expect(proof.promptRecord.promptVersion).toBe(modelRequest.promptVersion);
    expect(proof.summary).toEqual({
      inputPromptRecordCount: 1,
      selectedPromptRecordCount: 1,
      evidenceReferenceCount: 1,
      parentRuleReferenceCount: 1,
      compatibleModelCount: 1,
      compatibleRuntimeCount: 1,
    });
    expect(proof.claimBoundaries).toEqual(claimBoundaries);
  });

  it('rejects superseded prompt versions before a local AI request can use them', () => {
    expect(() =>
      buildLocalAiPromptTemplateVersionProof(
        proofInput({
          ...activePromptRecord,
          lifecycleState: 'superseded',
          validUntil: '2026-06-06T07:59:00.000Z',
          supersededByPromptVersion: 'prompt-template-screen-safety-v2',
        })
      )
    ).toThrow('Expected one active local AI prompt template version record for the evaluation request');
  });

  it('rejects prompt versions that are not compatible with the selected model/runtime', () => {
    expect(() =>
      buildLocalAiPromptTemplateVersionProof(
        proofInput({
          ...activePromptRecord,
          compatibleModelIds: ['model-other-prompt-template-proof'],
        })
      )
    ).toThrow('Expected one active local AI prompt template version record for the evaluation request');
  });

  it('rejects raw prompt retention, remote AI, model-quality, policy, UI, or enforcement overclaims', () => {
    expect(() =>
      buildLocalAiPromptTemplateVersionProof({
        ...proofInput(),
        claimBoundaries: { ...claimBoundaries, rawPromptRetained: true },
      })
    ).toThrow();
  });
});
