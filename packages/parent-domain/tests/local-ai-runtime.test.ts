import { describe, expect, it } from 'vitest';
import { LocalAiDegradedState } from '../src/local-ai-primitives';
import {
  LocalModelRuntimeStatusSchema,
  LocalProviderAdapterProbeSchema,
  LocalProviderCapabilitySchema,
} from '../src/local-ai-runtime';

const readyRuntimeStatus = {
  runtimeReferenceId: 'runtime-1',
  providerId: 'local-provider',
  modelId: 'safety-model',
  modelReference: 'local-model-cache/safety-model',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: LocalAiDegradedState.None,
  lastCheckedAt: '2026-05-20T20:44:00.000Z',
  unavailableReason: null,
} as const;

const unconfiguredRuntimeStatus = {
  runtimeReferenceId: 'runtime-unconfigured',
  providerId: 'local-provider-unconfigured',
  modelId: 'safety-model-unconfigured',
  modelReference: 'local-model-cache-unconfigured',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-unavailable',
  executionState: 'disabled',
  providerSource: 'unavailable',
  loadState: 'unavailable',
  capabilityFlags: [],
  resourceClass: 'cpu',
  degradedState: LocalAiDegradedState.ProviderUnavailable,
  lastCheckedAt: '2026-05-20T20:44:00.000Z',
  unavailableReason: 'local-ai-provider-unconfigured',
} as const;

const providerCapability = {
  providerId: 'local-provider',
  supportedTasks: ['safety-decision'],
  resourceClass: 'cpu',
  privacyMode: 'local-only',
  fallbackOrder: 1,
} as const;

const unconfiguredAdapterProbe = {
  providerId: 'local-provider-unconfigured',
  privacyMode: 'local-only',
  adapterBoundary: 'status-only',
  executionState: 'disabled',
  providerSource: 'unavailable',
  probeState: 'probe-unavailable',
  configurationState: 'local-provider-unconfigured',
  readinessState: 'adapter-not-ready',
  executionAllowed: false,
  lastCheckedAt: '2026-05-21T14:45:00.000Z',
  unavailableReason: 'local-ai-provider-unconfigured',
} as const;

describe('local AI runtime contracts', () => {
  it('runtime and provider schemas: distinguish local-only provider capability from runtime load state', () => {
    expect(LocalModelRuntimeStatusSchema.parse(readyRuntimeStatus).loadState).toBe('loaded');
    expect(LocalModelRuntimeStatusSchema.parse(unconfiguredRuntimeStatus).executionState).toBe('disabled');
    expect(LocalProviderCapabilitySchema.parse(providerCapability)).toEqual(providerCapability);
  });

  it('adapter probe schema: reports configuration state without permitting execution', () => {
    expect(LocalProviderAdapterProbeSchema.parse(unconfiguredAdapterProbe)).toEqual(unconfiguredAdapterProbe);
  });

  it('adapter probe schema: rejects non-boolean execution flags', () => {
    expect(
      LocalProviderAdapterProbeSchema.safeParse({
        ...unconfiguredAdapterProbe,
        executionAllowed: 'false',
      }).success
    ).toBe(false);
  });

  it('adapter probe schema: rejects execution when readiness is not ready', () => {
    expect(
      LocalProviderAdapterProbeSchema.safeParse({
        ...unconfiguredAdapterProbe,
        executionAllowed: true,
      }).success
    ).toBe(false);
  });

  it('adapter probe schema: rejects ready state without configured local provider boundary', () => {
    expect(
      LocalProviderAdapterProbeSchema.safeParse({
        ...unconfiguredAdapterProbe,
        readinessState: 'adapter-ready',
      }).success
    ).toBe(false);
  });
});
