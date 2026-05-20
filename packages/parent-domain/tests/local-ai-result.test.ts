import { describe, expect, it } from 'vitest';
import { LocalAiDegradedState } from '../src/local-ai-primitives';
import { LocalAiSafetyResultSchema } from '../src/local-ai';

const evidenceReference = {
  evidenceReferenceId: 'evidence-1',
  kind: 'journal-event',
  observedAt: '2026-05-20T20:45:00.000Z',
};

const runtimeStatus = {
  runtimeReferenceId: 'runtime-1',
  providerId: 'local-provider',
  modelId: 'safety-model',
  modelReference: 'local-model-cache/safety-model',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: LocalAiDegradedState.None,
  lastCheckedAt: '2026-05-20T20:44:00.000Z',
  unavailableReason: null,
};

describe('local AI safety result contracts', () => {
  const validSafetyResult = {
    schemaVersion: 'v0.6',
    resultId: 'ai-result-1',
    requestId: 'request-1',
    action: 'ask-parent',
    confidence: 0.61,
    unknownState: 'low-confidence',
    degradedState: 'none',
    reasonCodes: ['low-confidence-video-domain'],
    explanationReference: 'explanation-1',
    evidenceReferences: [evidenceReference],
    parentRuleReferences: ['rule-1'],
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: runtimeStatus,
    promptVersion: 'prompt-v1',
    expiresAt: null,
  };

  it('LocalAiSafetyResultSchema: parses safe degraded output without enabling enforcement', () => {
    const parsed = LocalAiSafetyResultSchema.parse(validSafetyResult);

    expect(parsed.action).toBe('ask-parent');
    expect(parsed.modelRuntime.loadState).toBe('loaded');
    expect(parsed.evidenceReferences).toEqual([evidenceReference]);
  });

  it('LocalAiSafetyResultSchema: rejects model output actions outside the typed decision set', () => {
    const result = LocalAiSafetyResultSchema.safeParse({
      ...validSafetyResult,
      action: 'silently-monitor',
      reasonCodes: ['bad-action'],
    });

    expect(result.success).toBe(false);
    if (!result.success) {
      expect([...new Set(result.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['action']);
    }
  });

  it('LocalAiSafetyResultSchema: rejects confidence below zero and above one', () => {
    for (const confidence of [-0.01, 1.01]) {
      const result = LocalAiSafetyResultSchema.safeParse({
        ...validSafetyResult,
        confidence,
      });

      expect(result.success).toBe(false);
      if (!result.success) {
        expect([...new Set(result.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['confidence']);
      }
    }
  });
});
