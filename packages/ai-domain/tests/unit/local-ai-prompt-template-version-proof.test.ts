import { describe, expect, it } from 'vitest';
import { buildLocalAiPromptTemplateVersionProof } from '@ocentra-parent/schema-domain/local-ai-prompt-template-version-proof';
import { LocalAiDegradedState, LocalAiUnknownState } from '@ocentra-parent/schema-domain/ai-primitives';

const observedAt = '2026-06-06T09:18:00.000Z';
const promptVersion = 'local-ai-safety-template-v1';
const childProfile = { childProfileId: 'child-prompt-template', displayName: 'Sam' };
const device = {
  deviceId: 'device-prompt-template',
  childProfileId: 'child-prompt-template',
  label: 'Sam Windows PC',
  platform: 'windows',
};
const evidenceReference = {
  evidenceReferenceId: 'prompt-template-source-evidence',
  kind: 'journal-event',
  observedAt,
};
const runtimeStatus = {
  runtimeReferenceId: 'runtime-prompt-template',
  providerId: 'local-provider-prompt-template',
  modelId: 'local-model-prompt-template',
  modelReference: 'artifact:local_prompt_template_model',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: LocalAiDegradedState.None,
  lastCheckedAt: observedAt,
  unavailableReason: null,
};
const contextRequest = {
  schemaVersion: 'v0.6',
  requestId: 'prompt-template-request',
  requestedAt: observedAt,
  childProfile,
  device,
  requestedEvaluationKind: 'mixed-context',
  requiredEvidenceKinds: ['screen-summary'],
  parentRuleContextReferences: [],
  modelTaskRequirements: ['safety-decision'],
  allowedCustody: ['child-device-query-store'],
  promptVersion,
};
const evaluationInput = {
  schemaVersion: 'v0.6',
  requestId: 'prompt-template-request',
  childProfile,
  device,
  currentObservation: { contextKind: 'recent-activity', evidence: evidenceReference },
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['prompt-template-policy-rule'],
  recentActivityWindow: [evidenceReference],
  memoryReferences: [],
  graphReferences: [],
  modelRequest: {
    providerId: runtimeStatus.providerId,
    modelId: runtimeStatus.modelId,
    promptVersion,
  },
};
const safetyResult = {
  schemaVersion: 'v0.6',
  resultId: 'prompt-template-result',
  requestId: evaluationInput.requestId,
  action: 'warn',
  confidence: 0.66,
  unknownState: LocalAiUnknownState.None,
  degradedState: LocalAiDegradedState.None,
  reasonCodes: ['local-ai-text-dry-run-candidate'],
  explanationReference: 'prompt-template-explanation',
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['prompt-template-policy-rule'],
  memoryReferences: [],
  graphReferences: [],
  modelRuntime: runtimeStatus,
  promptVersion,
  expiresAt: null,
};
const claimBoundaries = {
  modelExecutionClaimed: false,
  modelQualityClaimed: false,
  policyAuthorityClaimed: false,
  enforcementClaimed: false,
  portalUiClaimed: false,
  remoteApiAiUsed: false,
  rawPromptRetained: false,
  rawModelOutputRetained: false,
};

function templateRow(inputBinding: string, rowPromptVersion = promptVersion) {
  return {
    templateRef: `prompt-template:${inputBinding}`,
    promptVersion: rowPromptVersion,
    providerId: runtimeStatus.providerId,
    modelId: runtimeStatus.modelId,
    task: 'safety-decision',
    inputBinding,
    outputSchemaRef: `schema:${inputBinding}`,
    active: true,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
  };
}

function proofInput(overrides: Record<string, unknown> = {}) {
  return {
    schemaVersion: 'v0.6',
    contextRequest,
    evaluationInput,
    safetyResult,
    templateRows: [templateRow('context-builder'), templateRow('evaluation-input'), templateRow('safety-result')],
    claimBoundaries,
    ...overrides,
  };
}

describe('local AI prompt template version proof', () => {
  it('proves prompt/template version refs align across context, input, result, provider, and model metadata', () => {
    const proof = buildLocalAiPromptTemplateVersionProof(proofInput());

    expect(proof.contextRequest.promptVersion).toBe(promptVersion);
    expect(proof.evaluationInput.modelRequest.promptVersion).toBe(promptVersion);
    expect(proof.safetyResult.promptVersion).toBe(promptVersion);
    expect(proof.selectedTemplateRows.map((row) => row.inputBinding).sort()).toEqual([
      'context-builder',
      'evaluation-input',
      'safety-result',
    ]);
    expect(proof.summary).toEqual({
      templateRowCount: 3,
      activeTemplateRowCount: 3,
      inputBindingCount: 3,
      promptVersionMatchCount: 3,
      nonRetainingTemplateRowCount: 3,
    });
  });

  it('rejects prompt rows that do not cover every required input binding', () => {
    expect(() =>
      buildLocalAiPromptTemplateVersionProof({
        ...proofInput(),
        templateRows: [templateRow('context-builder'), templateRow('evaluation-input')],
      })
    ).toThrow('Expected local AI prompt/template proof');
  });

  it('rejects mismatched context or result prompt versions', () => {
    expect(() =>
      buildLocalAiPromptTemplateVersionProof({
        ...proofInput(),
        safetyResult: { ...safetyResult, promptVersion: 'other-prompt-version' },
      })
    ).toThrow('Expected local AI prompt/template proof');
  });

  it('rejects raw prompt or model output retention overclaims', () => {
    expect(() =>
      buildLocalAiPromptTemplateVersionProof({
        ...proofInput(),
        claimBoundaries: { ...claimBoundaries, rawPromptRetained: true },
      })
    ).toThrow();
  });
});
