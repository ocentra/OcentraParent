import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { LocalAiTimestampSchema } from './ai-primitives';

const LOCAL_MODEL_ARTIFACT_REF_PREFIX = 'artifact:';
const LOCAL_MODEL_MANIFEST_REF_PREFIX = 'manifest:';
const LOCAL_MODEL_OPAQUE_REF_BODY = /^[a-z0-9][a-z0-9_-]{2,127}$/u;
const LOCAL_MODEL_UNSAFE_REF_PATTERNS = [/^[a-z][a-z0-9+.-]*:\/\//iu, /^[a-z]:[\\/]/iu, /^\\\\/u, /^\//u, /[\\/]/u];

const LocalAiModelArtifactText = NonEmptyStringSchema.pipe(
  Schema.filter(
    (candidate) =>
      isOpaqueLocalModelRef(candidate, LOCAL_MODEL_ARTIFACT_REF_PREFIX) || 'Expected local model artifact ref'
  )
);
const LocalAiModelManifestText = NonEmptyStringSchema.pipe(
  Schema.filter(
    (candidate) =>
      isOpaqueLocalModelRef(candidate, LOCAL_MODEL_MANIFEST_REF_PREFIX) || 'Expected local model manifest ref'
  )
);

export const LocalAiModelArtifactRefSchema = LocalAiModelArtifactText.pipe(Schema.brand('LocalAiModelArtifactRef'));
export const LocalAiModelManifestRefSchema = LocalAiModelManifestText.pipe(Schema.brand('LocalAiModelManifestRef'));
export const LocalAiModelSourcePolicySchema = withParser(
  Schema.Literal('bundled', 'parent-installed', 'local-cache', 'unavailable')
);
export const LocalAiModelCacheStateSchema = withParser(
  Schema.Literal('unavailable', 'not-cached', 'cache-ready', 'cache-degraded', 'cache-corrupted', 'storage-error')
);
export const LocalAiModelCacheHealthSchema = withParser(
  Schema.Literal('healthy', 'degraded', 'unavailable', 'download-disabled', 'corrupted', 'storage-error')
);
export const LocalAiModelManifestIntegrityStateSchema = withParser(
  Schema.Literal(
    'unavailable',
    'unchecked',
    'verified',
    'manifest-missing',
    'checksum-mismatch',
    'signature-invalid',
    'corrupted'
  )
);
export const LocalAiModelDownloadStatusSchema = withParser(
  Schema.Literal(
    'download-disabled',
    'download-not-requested',
    'download-in-progress',
    'download-complete',
    'download-failed'
  )
);
export const LocalAiModelCacheUnavailableReasonSchema = withParser(
  Schema.Literal(
    'model-source-unconfigured',
    'artifact-not-installed',
    'manifest-unavailable',
    'download-disabled',
    'cache-storage-unavailable',
    'integrity-unverified',
    'corruption-detected'
  )
);
export const LocalAiModelCacheStorageErrorCodeSchema = withParser(
  Schema.Literal(
    'cache-root-unavailable',
    'manifest-read-failed',
    'artifact-read-failed',
    'metadata-write-disabled',
    'storage-permission-denied',
    'quota-unavailable'
  )
);
export const LocalAiModelCacheCorruptionReasonCodeSchema = withParser(
  Schema.Literal(
    'manifest-missing',
    'checksum-mismatch',
    'signature-invalid',
    'artifact-missing',
    'manifest-artifact-mismatch',
    'unknown-integrity'
  )
);

function isOpaqueLocalModelRef(
  candidate: unknown,
  prefix: typeof LOCAL_MODEL_ARTIFACT_REF_PREFIX | typeof LOCAL_MODEL_MANIFEST_REF_PREFIX
): boolean {
  if (typeof candidate !== 'string' || LOCAL_MODEL_UNSAFE_REF_PATTERNS.some((pattern) => pattern.test(candidate)))
    return false;
  if (!candidate.startsWith(prefix)) return false;
  return LOCAL_MODEL_OPAQUE_REF_BODY.test(candidate.slice(prefix.length));
}

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
      (status) => localAiModelCacheStatusIsConsistent(status) || 'Expected consistent local model cache status'
    )
  )
);

function localAiModelCacheStatusIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  return (
    localAiModelDownloadStatusMatchesAvailability(status) &&
    localAiModelUnavailableSourcePolicyIsConsistent(status) &&
    localAiModelCacheReadyStateIsConsistent(status)
  );
}

function localAiModelDownloadStatusMatchesAvailability(status: LocalAiModelCacheStatusCandidate): boolean {
  return status.downloadEnabled === (status.downloadStatus !== 'download-disabled');
}

function localAiModelUnavailableSourcePolicyIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  if (status.sourcePolicy !== 'unavailable') {
    return true;
  }

  return (
    status.cacheState === 'unavailable' &&
    status.cacheHealth === 'unavailable' &&
    status.manifestIntegrity === 'unavailable' &&
    status.manifestRef === null &&
    status.cacheByteSize === 0 &&
    status.unavailableReason !== null
  );
}

function localAiModelCacheReadyStateIsConsistent(status: LocalAiModelCacheStatusCandidate): boolean {
  if (status.cacheState !== 'cache-ready') {
    return true;
  }

  return (
    status.cacheHealth === 'healthy' &&
    status.manifestIntegrity === 'verified' &&
    status.manifestRef !== null &&
    status.cacheByteSize > 0 &&
    status.unavailableReason === null &&
    status.storageError === null &&
    status.corruptionReason === null
  );
}

export type LocalAiModelArtifactRef = typeof LocalAiModelArtifactRefSchema.Type;
export type LocalAiModelManifestRef = typeof LocalAiModelManifestRefSchema.Type;
export type LocalAiModelSourcePolicy = Infer<typeof LocalAiModelSourcePolicySchema>;
export type LocalAiModelCacheState = Infer<typeof LocalAiModelCacheStateSchema>;
export type LocalAiModelCacheHealth = Infer<typeof LocalAiModelCacheHealthSchema>;
export type LocalAiModelManifestIntegrityState = Infer<typeof LocalAiModelManifestIntegrityStateSchema>;
export type LocalAiModelDownloadStatus = Infer<typeof LocalAiModelDownloadStatusSchema>;
export type LocalAiModelCacheUnavailableReason = Infer<typeof LocalAiModelCacheUnavailableReasonSchema>;
export type LocalAiModelCacheStorageErrorCode = Infer<typeof LocalAiModelCacheStorageErrorCodeSchema>;
export type LocalAiModelCacheCorruptionReasonCode = Infer<typeof LocalAiModelCacheCorruptionReasonCodeSchema>;
export type LocalAiModelCacheStatusKind = Infer<typeof LocalAiModelCacheStatusKindSchema>;
export type LocalAiModelCacheStatus = Infer<typeof LocalAiModelCacheStatusSchema>;
