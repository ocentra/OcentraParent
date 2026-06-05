import { describe, expect, it } from 'vitest';
import { LocalAiSafetyResultSchema } from '../src/local-ai';
import { PolicyDecisionSchema, selectStricterPolicyAction } from '../src/policy';

describe('screen AI policy integration proof', () => {
  it('keeps a stricter parent block rule when local screen AI suggests allow', () => {
    const aiResult = parseLocalAiResult({ action: 'allow', confidence: 0.91, unknownState: 'none' });
    const action = selectStricterPolicyAction('block', aiResult.action);
    const decision = parsePolicyDecision({
      action,
      ruleIds: ['parent-rule-screen-block'],
      localAiResultId: aiResult.resultId,
    });

    expect(action).toBe('block');
    expect(decision.action).toBe('block');
    expect(decision.localAiResultId).toBe('screen-ai-result-school-allow');
    expect(decision.ruleIds).toEqual(['parent-rule-screen-block']);
    expect(decision.evidenceReferences).toEqual(aiResult.evidenceReferences);
    expect(decision.enforcementHandoffState).toBe('disabled');
  });

  it('keeps a stricter parent time-limit rule when screen AI is low-confidence allow', () => {
    const aiResult = parseLocalAiResult({
      resultId: 'screen-ai-result-low-confidence-allow',
      action: 'allow',
      confidence: 0.42,
      unknownState: 'low-confidence',
      reasonCodes: ['screen-ai-low-confidence-school'],
    });
    const action = selectStricterPolicyAction('time-limit', aiResult.action);
    const decision = parsePolicyDecision({
      decisionId: 'policy-decision-screen-low-confidence-time-limit',
      action,
      reasonCodes: ['parent-rule-time-limit-wins', ...aiResult.reasonCodes],
      ruleIds: ['parent-rule-screen-time-limit'],
      localAiResultId: aiResult.resultId,
      expiresAt: '2026-06-05T23:00:00.000Z',
    });

    expect(action).toBe('time-limit');
    expect(decision.action).toBe('time-limit');
    expect(decision.expiresAt).toBe('2026-06-05T23:00:00.000Z');
    expect(decision.reasonCodes).toEqual(['parent-rule-time-limit-wins', 'screen-ai-low-confidence-school']);
  });

  it('rejects untyped policy decisions before policy can consume them', () => {
    expect(() =>
      PolicyDecisionSchema.parse({
        ...policyDecisionBase(),
        action: 'remote-ai-says-safe',
      })
    ).toThrow(/Expected/u);

    expect(() =>
      LocalAiSafetyResultSchema.parse({
        ...localAiResultBase(),
        confidence: 1.2,
      })
    ).toThrow(/Expected/u);
  });
});

function parseLocalAiResult(overrides: Record<string, unknown> = {}) {
  return LocalAiSafetyResultSchema.parse({ ...localAiResultBase(), ...overrides });
}

function parsePolicyDecision(overrides: Record<string, unknown> = {}) {
  return PolicyDecisionSchema.parse({ ...policyDecisionBase(), ...overrides });
}

function localAiResultBase() {
  return {
    schemaVersion: 'v0.6',
    resultId: 'screen-ai-result-school-allow',
    requestId: 'screen-ai-request-school-page',
    action: 'allow',
    confidence: 0.91,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: ['screen-ai-school-content'],
    explanationReference: 'screen-ai-explanation-school-page',
    evidenceReferences: [screenEvidenceReference()],
    parentRuleReferences: ['parent-rule-screen-block'],
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: {
      runtimeReferenceId: 'screen-ai-runtime-local',
      providerId: 'local-screen-provider',
      modelId: 'screen-safety-model',
      modelReference: 'local-model-cache-screen-safety',
      privacyMode: 'local-only',
      adapterBoundary: 'local-adapter-ready',
      executionState: 'dry-run-ready',
      providerSource: 'local-model-cache',
      loadState: 'loaded',
      capabilityFlags: ['safety-decision'],
      resourceClass: 'cpu',
      degradedState: 'none',
      lastCheckedAt: '2026-06-05T22:00:00.000Z',
      unavailableReason: null,
    },
    promptVersion: 'screen-ai-policy-integration-v1',
    expiresAt: null,
  };
}

function policyDecisionBase() {
  return {
    schemaVersion: 'v0.6',
    decisionId: 'policy-decision-screen-parent-rule-wins',
    action: 'block',
    reasonCodes: ['parent-rule-block-wins', 'screen-ai-school-content'],
    evidenceReferences: [screenEvidenceReference()],
    ruleIds: ['parent-rule-screen-block'],
    localAiResultId: 'screen-ai-result-school-allow',
    dryRun: true,
    enforcementHandoffState: 'disabled',
    expiresAt: null,
  };
}

function screenEvidenceReference() {
  return {
    evidenceReferenceId: 'screen-summary-evidence-wikipedia-school',
    kind: 'activity-event',
    observedAt: '2026-06-05T22:00:00.000Z',
  };
}
