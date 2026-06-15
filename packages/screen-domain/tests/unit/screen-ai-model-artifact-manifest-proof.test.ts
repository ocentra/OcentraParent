import { describe, expect, it } from 'vitest';
import {
  ScreenAiModelArtifactManifestSchema,
  buildScreenAiModelArtifactManifestProof,
} from '../../src/screen-ai-model-artifact-manifest-proof';

const generatedAt = '2026-06-05T20:01:00.000Z';

const readyManifest = {
  schemaVersion: 'v0.6',
  proofId: 'screen-ai-model-artifact-manifest-proof',
  generatedAt,
  purpose: 'screen-child-safety-local-analysis',
  providerId: 'screen-local-provider',
  modelId: 'screen-child-safety-v1',
  artifactRef: 'artifact:screen-child-safety-v1',
  manifestRef: 'manifest:screen-child-safety-v1',
  requiredCapability: 'safety-decision',
  privacyMode: 'local-only',
  cacheStatus: {
    statusKind: 'local-model-cache-status',
    artifactRef: 'artifact:screen-child-safety-v1',
    manifestRef: 'manifest:screen-child-safety-v1',
    sourcePolicy: 'local-cache',
    cacheState: 'cache-ready',
    cacheHealth: 'healthy',
    manifestIntegrity: 'verified',
    downloadEnabled: false,
    downloadStatus: 'download-disabled',
    cacheByteSize: 524288,
    checkedAt: generatedAt,
    unavailableReason: null,
    storageError: null,
    corruptionReason: null,
  },
  runtimeStatus: {
    runtimeReferenceId: 'runtime:screen-child-safety-v1',
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
    lastCheckedAt: generatedAt,
    unavailableReason: null,
  },
  providerCapability: {
    providerId: 'screen-local-provider',
    supportedTasks: ['safety-decision', 'classification'],
    resourceClass: 'cpu',
    privacyMode: 'local-only',
    fallbackOrder: 1,
  },
  manifestCheckedAt: generatedAt,
  claimBoundaries: {
    remoteProviderUsed: false,
    apiProviderUsed: false,
    ocentraHostedProcessingUsed: false,
    modelQualityClaimed: false,
    rawEvidenceEmbedded: false,
    executionClaimed: false,
  },
} as const;

function parsesWith(overrides: Record<string, unknown>): boolean {
  return ScreenAiModelArtifactManifestSchema.safeParse({
    ...readyManifest,
    ...overrides,
  }).success;
}

describe('screen AI model artifact manifest proof', () => {
  it('accepts a verified local-only screen model artifact manifest boundary', () => {
    expect(buildScreenAiModelArtifactManifestProof(readyManifest)).toEqual(readyManifest);
  });

  it('rejects mismatched cache, runtime, or provider refs', () => {
    expect(parsesWith({ artifactRef: 'artifact:other-screen-model' })).toBe(false);
    expect(
      parsesWith({
        runtimeStatus: {
          ...readyManifest.runtimeStatus,
          providerId: 'other-provider',
        },
      })
    ).toBe(false);
    expect(
      parsesWith({
        providerCapability: {
          ...readyManifest.providerCapability,
          providerId: 'other-provider',
        },
      })
    ).toBe(false);
  });

  it('rejects unverified, unavailable, or remote-like manifest states', () => {
    expect(
      parsesWith({
        cacheStatus: {
          ...readyManifest.cacheStatus,
          manifestIntegrity: 'checksum-mismatch',
          corruptionReason: 'checksum-mismatch',
          unavailableReason: 'corruption-detected',
        },
      })
    ).toBe(false);
    expect(
      parsesWith({
        runtimeStatus: {
          ...readyManifest.runtimeStatus,
          providerSource: 'unavailable',
        },
      })
    ).toBe(false);
  });

  it('rejects quality, execution, API, remote, hosted, or raw evidence overclaims', () => {
    for (const claim of Object.keys(readyManifest.claimBoundaries)) {
      expect(
        parsesWith({
          claimBoundaries: {
            ...readyManifest.claimBoundaries,
            [claim]: true,
          },
        })
      ).toBe(false);
    }
  });

  it('rejects missing required screen safety capability', () => {
    expect(
      parsesWith({
        providerCapability: {
          ...readyManifest.providerCapability,
          supportedTasks: ['classification'],
        },
      })
    ).toBe(false);
  });
});
