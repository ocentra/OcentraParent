import { describe, expect, it } from 'vitest';
import { LocalAiModelCacheStatusSchema } from '../../src/local-ai-model-artifacts';

const checkedAt = '2026-05-21T21:35:00.000Z';

const readyCacheStatus = {
  statusKind: 'local-model-cache-status',
  artifactRef: 'artifact:child-safety-v1',
  manifestRef: 'manifest:child-safety-v1',
  sourcePolicy: 'local-cache',
  cacheState: 'cache-ready',
  cacheHealth: 'healthy',
  manifestIntegrity: 'verified',
  downloadEnabled: false,
  downloadStatus: 'download-disabled',
  cacheByteSize: 4096,
  checkedAt,
  unavailableReason: null,
  storageError: null,
  corruptionReason: null,
} as const;

const unavailableCacheStatus = {
  statusKind: 'local-model-cache-status',
  artifactRef: 'artifact:child-safety-unavailable',
  manifestRef: null,
  sourcePolicy: 'unavailable',
  cacheState: 'unavailable',
  cacheHealth: 'unavailable',
  manifestIntegrity: 'unavailable',
  downloadEnabled: false,
  downloadStatus: 'download-disabled',
  cacheByteSize: 0,
  checkedAt,
  unavailableReason: 'model-source-unconfigured',
  storageError: null,
  corruptionReason: null,
} as const;

const storageErrorCacheStatus = {
  ...unavailableCacheStatus,
  artifactRef: 'artifact:child-safety-storage-error',
  sourcePolicy: 'local-cache',
  cacheState: 'storage-error',
  cacheHealth: 'storage-error',
  unavailableReason: 'cache-storage-unavailable',
  storageError: 'cache-root-unavailable',
} as const;

const corruptedCacheStatus = {
  ...unavailableCacheStatus,
  artifactRef: 'artifact:child-safety-corrupted',
  manifestRef: 'manifest:child-safety-corrupted',
  sourcePolicy: 'local-cache',
  cacheState: 'cache-corrupted',
  cacheHealth: 'corrupted',
  manifestIntegrity: 'checksum-mismatch',
  unavailableReason: 'corruption-detected',
  corruptionReason: 'checksum-mismatch',
} as const;

const unsafeArtifactRefs = [
  'C:\\Users\\sujan\\models\\child-safety.gguf',
  '/var/cache/models/child-safety.gguf',
  '..\\cache\\child-safety.gguf',
  'file:///var/cache/models/child-safety.gguf',
  'https://models.example.invalid/child-safety.gguf',
] as const;

const unsafeManifestRefs = [
  'C:\\Users\\sujan\\models\\manifest.json',
  '/var/cache/models/manifest.json',
  '..\\cache\\manifest.json',
  'file:///var/cache/models/manifest.json',
  'https://models.example.invalid/manifest.json',
] as const;

function parseReadyStatusWith(overrides: Record<string, unknown>): boolean {
  return LocalAiModelCacheStatusSchema.safeParse({
    ...readyCacheStatus,
    ...overrides,
  }).success;
}

function parseUnavailableStatusWith(overrides: Record<string, unknown>): boolean {
  return LocalAiModelCacheStatusSchema.safeParse({
    ...unavailableCacheStatus,
    ...overrides,
  }).success;
}

describe('local AI model artifact and cache contracts', () => {
  it('accepts ready local-cache status with opaque artifact and manifest references', () => {
    expect(LocalAiModelCacheStatusSchema.parse(readyCacheStatus)).toEqual(readyCacheStatus);
  });

  it('accepts unavailable no-download status without claiming a manifest or cache bytes', () => {
    expect(LocalAiModelCacheStatusSchema.parse(unavailableCacheStatus)).toEqual(unavailableCacheStatus);
  });

  it('accepts storage-error and corrupted cache states only with exact reason codes', () => {
    expect(LocalAiModelCacheStatusSchema.parse(storageErrorCacheStatus).storageError).toBe('cache-root-unavailable');
    expect(LocalAiModelCacheStatusSchema.parse(corruptedCacheStatus).corruptionReason).toBe('checksum-mismatch');
  });

  it('rejects filesystem paths and URLs as artifact references', () => {
    expect(unsafeArtifactRefs.map((artifactRef) => parseReadyStatusWith({ artifactRef }))).toEqual([
      false,
      false,
      false,
      false,
      false,
    ]);
  });

  it('rejects filesystem paths and URLs as manifest references', () => {
    expect(unsafeManifestRefs.map((manifestRef) => parseReadyStatusWith({ manifestRef }))).toEqual([
      false,
      false,
      false,
      false,
      false,
    ]);
  });

  it('rejects unavailable source policy with unsupported ready cache fields', () => {
    expect(
      parseReadyStatusWith({
        sourcePolicy: 'unavailable',
      })
    ).toBe(false);
  });

  it('rejects ready cache status without verified manifest integrity', () => {
    expect(
      parseReadyStatusWith({
        manifestIntegrity: 'unchecked',
      })
    ).toBe(false);
  });

  it('rejects disabled download flags when download status claims progress', () => {
    expect(
      parseUnavailableStatusWith({
        downloadStatus: 'download-in-progress',
      })
    ).toBe(false);
  });
});
