import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiModelArtifactRefSchema,
  LocalAiModelCacheCorruptionReasonCodeSchema,
  LocalAiModelCacheHealthSchema,
  LocalAiModelCacheStateSchema,
  LocalAiModelCacheStorageErrorCodeSchema,
  LocalAiModelCacheUnavailableReasonSchema,
  LocalAiModelDownloadStatusSchema,
  LocalAiModelManifestIntegrityStateSchema,
  LocalAiModelManifestRefSchema,
  LocalAiModelSourcePolicySchema,
} from './local-ai-model-artifact-primitives';
import { LocalAiTimestampSchema } from './local-ai-primitives';

export * from './local-ai-model-artifact-primitives';

export const LocalAiModelCacheStatusKindSchema = withParser(Schema.Literal('local-model-cache-status'));

const LocalAiModelCacheByteSizeSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

const LocalAiModelCacheStatusBaseSchema = Schema.Struct({
  statusKind: LocalAiModelCacheStatusKindSchema,
  artifactRef: LocalAiModelArtifactRefSchema,
  manifestRef: Schema.Union(LocalAiModelManifestRefSchema, Schema.Null),
  sourcePolicy: LocalAiModelSourcePolicySchema,
  cacheState: LocalAiModelCacheStateSchema,
  cacheHealth: LocalAiModelCacheHealthSchema,
  manifestIntegrity: LocalAiModelManifestIntegrityStateSchema,
  downloadEnabled: Schema.Boolean,
  downloadStatus: LocalAiModelDownloadStatusSchema,
  cacheByteSize: LocalAiModelCacheByteSizeSchema,
  checkedAt: LocalAiTimestampSchema,
  unavailableReason: Schema.Union(LocalAiModelCacheUnavailableReasonSchema, Schema.Null),
  storageError: Schema.Union(LocalAiModelCacheStorageErrorCodeSchema, Schema.Null),
  corruptionReason: Schema.Union(LocalAiModelCacheCorruptionReasonCodeSchema, Schema.Null),
});

type LocalAiModelCacheStatusCandidate = Infer<typeof LocalAiModelCacheStatusBaseSchema>;

export const LocalAiModelCacheStatusSchema = withParser(
  LocalAiModelCacheStatusBaseSchema.pipe(
    Schema.filter(
      (status) =>
        localAiModelCacheStatusIsConsistent(status) ||
        'Expected local model cache status to keep readiness, source policy, download, and integrity states consistent'
    )
  )
);

function localAiModelCacheStatusIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  return (
    downloadStateIsConsistent(status) &&
    unavailableSourceIsConsistent(status) &&
    readyCacheStateIsConsistent(status) &&
    manifestRefIsConsistent(status) &&
    storageErrorStateIsConsistent(status) &&
    corruptionStateIsConsistent(status) &&
    unavailableReasonIsConsistent(status)
  );
}

function downloadStateIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  if (status.downloadEnabled) {
    return status.downloadStatus !== 'download-disabled';
  }

  return status.downloadStatus === 'download-disabled';
}

function unavailableSourceIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  if (status.sourcePolicy !== 'unavailable') {
    return true;
  }

  return (
    status.cacheState === 'unavailable' &&
    status.cacheHealth === 'unavailable' &&
    status.manifestIntegrity === 'unavailable' &&
    status.manifestRef === null &&
    status.cacheByteSize === 0 &&
    status.unavailableReason !== null &&
    status.downloadEnabled === false &&
    status.downloadStatus === 'download-disabled'
  );
}

function readyCacheStateIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  if (status.cacheState !== 'cache-ready') {
    return status.cacheHealth !== 'healthy';
  }

  return (
    status.sourcePolicy !== 'unavailable' &&
    status.cacheHealth === 'healthy' &&
    status.manifestIntegrity === 'verified' &&
    status.manifestRef !== null &&
    status.cacheByteSize > 0 &&
    status.unavailableReason === null &&
    status.storageError === null &&
    status.corruptionReason === null
  );
}

function manifestRefIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  if (status.manifestIntegrity === 'verified') {
    return status.manifestRef !== null;
  }

  return true;
}

function storageErrorStateIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  const reportsStorageError = status.cacheState === 'storage-error' || status.cacheHealth === 'storage-error';
  if (reportsStorageError) {
    return status.storageError !== null && status.unavailableReason !== null;
  }

  return status.storageError === null;
}

function corruptionStateIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  const reportsCorruption =
    status.cacheState === 'cache-corrupted' ||
    status.cacheHealth === 'corrupted' ||
    status.manifestIntegrity === 'checksum-mismatch' ||
    status.manifestIntegrity === 'signature-invalid' ||
    status.manifestIntegrity === 'corrupted';

  if (reportsCorruption) {
    return status.corruptionReason !== null && status.unavailableReason !== null;
  }

  return status.corruptionReason === null;
}

function unavailableReasonIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  const isFullyReady =
    status.sourcePolicy !== 'unavailable' &&
    status.cacheState === 'cache-ready' &&
    status.cacheHealth === 'healthy' &&
    status.manifestIntegrity === 'verified';

  if (isFullyReady) {
    return status.unavailableReason === null;
  }

  return status.unavailableReason !== null;
}

export type LocalAiModelCacheStatusKind = Infer<typeof LocalAiModelCacheStatusKindSchema>;
export type LocalAiModelCacheStatus = Infer<typeof LocalAiModelCacheStatusSchema>;
