import { describe, expect, it } from 'vitest';
import { LocalAiSafetyResultSchema } from '@ocentra-parent/ai-domain/local-ai';

const observedAt = '2026-06-05T22:32:00.000Z';

const evidenceReference = {
  evidenceReferenceId: 'screen-evidence:invalid-output-degrade',
  kind: 'activity-event',
  observedAt,
};

const readyRuntime = {
  runtimeReferenceId: 'runtime:screen-invalid-output-degrade',
  providerId: 'screen-local-provider',
  modelId: 'screen-child-safety-v1',
  modelReference: 'artifact:screen-child-safety-v1',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: observedAt,
  unavailableReason: null,
};

const baseResult = {
  schemaVersion: 'v0.6',
  resultId: 'screen-ai-result:invalid-output-degrade',
  requestId: 'screen-ai-request:invalid-output-degrade',
  action: 'warn',
  confidence: 0.7,
  unknownState: 'none',
  degradedState: 'none',
  reasonCodes: ['screen-ai:valid-output'],
  explanationReference: 'explanation:screen-ai-valid-output',
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule:screen-safety'],
  memoryReferences: [],
  graphReferences: [],
  modelRuntime: readyRuntime,
  promptVersion: 'screen-safety-template-v1',
  expiresAt: null,
};

const invalidOutputDegradedResult = {
  ...baseResult,
  resultId: 'screen-ai-result:invalid-output',
  action: 'unknown',
  confidence: 0,
  unknownState: 'model-unavailable',
  degradedState: 'invalid-output',
  reasonCodes: ['screen-ai:invalid-ai-output', 'screen-ai:model-output-unparseable'],
  explanationReference: null,
  modelRuntime: {
    ...readyRuntime,
    executionState: 'failed',
    loadState: 'degraded',
    degradedState: 'invalid-output',
    unavailableReason: 'screen-ai-model-output-unparseable',
  },
};

const timeoutDegradedResult = {
  ...baseResult,
  resultId: 'screen-ai-result:timeout',
  action: 'ask-parent',
  confidence: 0,
  unknownState: 'model-unavailable',
  degradedState: 'overloaded',
  reasonCodes: ['screen-ai:model-timeout', 'screen-ai:manual-parent-review-required'],
  explanationReference: null,
  modelRuntime: {
    ...readyRuntime,
    executionState: 'failed',
    loadState: 'failed',
    degradedState: 'overloaded',
    unavailableReason: 'screen-ai-local-model-timeout',
  },
};

describe('screen AI invalid output degrade proof', () => {
  it('rejects malformed model output before it can become a safety result', () => {
    const malformedOutput = {
      ...baseResult,
      action: 'silently-allow',
      confidence: 1.4,
      evidenceReferences: 'screen-evidence:invalid-output-degrade',
      modelRuntime: {
        ...readyRuntime,
        privacyMode: 'remote-api',
      },
    };

    const result = LocalAiSafetyResultSchema.safeParse(malformedOutput);

    expect(result.success).toBe(false);
  });

  it('parses unparseable output fallback as non-enforcing unknown with invalid-output degradation', () => {
    const parsed = LocalAiSafetyResultSchema.parse(invalidOutputDegradedResult);

    expect(parsed.action).toBe('unknown');
    expect(parsed.confidence).toBe(0);
    expect(parsed.unknownState).toBe('model-unavailable');
    expect(parsed.degradedState).toBe('invalid-output');
    expect(parsed.modelRuntime.degradedState).toBe('invalid-output');
    expect(parsed.modelRuntime.unavailableReason).toBe('screen-ai-model-output-unparseable');
    expect(parsed.explanationReference).toBeNull();
  });

  it('parses timeout fallback as parent-review required with overloaded runtime metadata', () => {
    const parsed = LocalAiSafetyResultSchema.parse(timeoutDegradedResult);

    expect(parsed.action).toBe('ask-parent');
    expect(parsed.confidence).toBe(0);
    expect(parsed.unknownState).toBe('model-unavailable');
    expect(parsed.degradedState).toBe('overloaded');
    expect(parsed.modelRuntime.executionState).toBe('failed');
    expect(parsed.modelRuntime.loadState).toBe('failed');
    expect(parsed.modelRuntime.unavailableReason).toBe('screen-ai-local-model-timeout');
  });

  it('keeps degraded output tied to evidence and parent rules without creating enforcement authority', () => {
    for (const degradedResult of [invalidOutputDegradedResult, timeoutDegradedResult]) {
      const parsed = LocalAiSafetyResultSchema.parse(degradedResult);

      expect(parsed.evidenceReferences).toEqual([evidenceReference]);
      expect(parsed.parentRuleReferences).toEqual(['policy-rule:screen-safety']);
      expect(['unknown', 'ask-parent']).toContain(parsed.action);
      expect(parsed.action).not.toBe('allow');
      expect(parsed.action).not.toBe('block');
    }
  });
});
