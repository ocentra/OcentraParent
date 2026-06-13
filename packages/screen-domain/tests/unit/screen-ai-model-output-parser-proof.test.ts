import { describe, expect, it } from 'vitest';
import { LocalAiEvaluationInputSchema, LocalAiSafetyResultSchema } from '@ocentra-parent/ai-domain/local-ai';

const observedAt = '2026-06-05T22:10:00.000Z';

const evidenceReference = {
  evidenceReferenceId: 'screen-evidence:winrt-ocr-row',
  kind: 'activity-event',
  observedAt,
};

const childProfile = {
  childProfileId: 'child:screen-ai-parser',
  displayName: 'Sam',
};

const device = {
  deviceId: 'device:screen-ai-parser',
  childProfileId: childProfile.childProfileId,
  label: 'Sam Windows PC',
  platform: 'windows',
};

const modelRuntime = {
  runtimeReferenceId: 'runtime:screen-child-safety-parser',
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

const modelRequest = {
  providerId: modelRuntime.providerId,
  modelId: modelRuntime.modelId,
  promptVersion: 'screen-safety-template-v1',
};

const validScreenModelInput = {
  schemaVersion: 'v0.6',
  requestId: 'screen-ai-request:parser-proof',
  childProfile,
  device,
  currentObservation: {
    contextKind: 'video',
    evidence: evidenceReference,
  },
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule:school-night'],
  recentActivityWindow: [evidenceReference],
  memoryReferences: [],
  graphReferences: [],
  modelRequest,
};

const validScreenModelOutput = {
  schemaVersion: 'v0.6',
  resultId: 'screen-ai-result:parser-proof',
  requestId: validScreenModelInput.requestId,
  action: 'warn',
  confidence: 0.74,
  unknownState: 'none',
  degradedState: 'none',
  reasonCodes: ['screen-ai:video-detected', 'policy:school-night'],
  explanationReference: 'explanation:screen-ai-parser-proof',
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule:school-night'],
  memoryReferences: [],
  graphReferences: [],
  modelRuntime,
  promptVersion: modelRequest.promptVersion,
  expiresAt: '2026-06-05T22:15:00.000Z',
};

function screenModelOutputWith(overrides: Record<string, unknown>) {
  return LocalAiSafetyResultSchema.safeParse({
    ...validScreenModelOutput,
    ...overrides,
  });
}

describe('screen AI model output parser proof', () => {
  it('parses schema-valid screen model input and output without remote/API fields', () => {
    const parsedInput = LocalAiEvaluationInputSchema.parse(validScreenModelInput);
    const parsedOutput = LocalAiSafetyResultSchema.parse(validScreenModelOutput);

    expect(parsedInput.currentObservation.contextKind).toBe('video');
    expect(parsedOutput.action).toBe('warn');
    expect(parsedOutput.evidenceReferences).toEqual([evidenceReference]);
    expect(parsedOutput.modelRuntime.privacyMode).toBe('local-only');
    expect(parsedOutput.promptVersion).toBe(modelRequest.promptVersion);
  });

  it('rejects malformed model output action, confidence, and state values', () => {
    const invalidCases = [
      screenModelOutputWith({ action: 'redirect' }),
      screenModelOutputWith({ confidence: 1.01 }),
      screenModelOutputWith({ confidence: -0.01 }),
      screenModelOutputWith({ unknownState: 'guessing' }),
      screenModelOutputWith({ degradedState: 'remote-fallback' }),
    ];

    for (const result of invalidCases) {
      expect(result.success).toBe(false);
    }
  });

  it('rejects missing evidence, rules, or local-only runtime metadata', () => {
    expect(screenModelOutputWith({ evidenceReferences: 'screen-evidence:winrt-ocr-row' }).success).toBe(false);
    expect(screenModelOutputWith({ reasonCodes: 'screen-ai:video-detected' }).success).toBe(false);
    expect(screenModelOutputWith({ parentRuleReferences: 'policy-rule:school-night' }).success).toBe(false);
    expect(
      screenModelOutputWith({
        modelRuntime: {
          ...modelRuntime,
          privacyMode: 'remote-api',
        },
      }).success
    ).toBe(false);
  });

  it('rejects parser input that drops the current screen observation evidence', () => {
    const result = LocalAiEvaluationInputSchema.safeParse({
      ...validScreenModelInput,
      currentObservation: {
        contextKind: 'video',
      },
    });

    expect(result.success).toBe(false);
  });
});
