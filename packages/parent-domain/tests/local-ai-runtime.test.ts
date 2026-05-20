import { describe, expect, it } from 'vitest';
import { LocalAiDegradedState } from '../src/local-ai-primitives';
import { LocalModelRuntimeStatusSchema, LocalProviderCapabilitySchema } from '../src/local-ai-runtime';

describe('local AI runtime contracts', () => {
  it('runtime and provider schemas: distinguish local-only provider capability from runtime load state', () => {
    expect(
      LocalModelRuntimeStatusSchema.parse({
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
      }).loadState
    ).toBe('loaded');
    expect(
      LocalProviderCapabilitySchema.parse({
        providerId: 'local-provider',
        supportedTasks: ['safety-decision'],
        resourceClass: 'cpu',
        privacyMode: 'local-only',
        fallbackOrder: 1,
      })
    ).toEqual({
      providerId: 'local-provider',
      supportedTasks: ['safety-decision'],
      resourceClass: 'cpu',
      privacyMode: 'local-only',
      fallbackOrder: 1,
    });
  });
});
