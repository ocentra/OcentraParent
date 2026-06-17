import { NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
const LOCAL_MODEL_ARTIFACT_REF_PREFIX = 'artifact:';
const LOCAL_MODEL_MANIFEST_REF_PREFIX = 'manifest:';
const LOCAL_MODEL_OPAQUE_REF_BODY = /^[a-z0-9][a-z0-9_-]{2,127}$/u;
const LOCAL_MODEL_UNSAFE_REF_PATTERNS = [/^[a-z][a-z0-9+.-]*:\/\//iu, /^[a-z]:[\\/]/iu, /^\\\\/u, /^\//u, /[\\/]/u];
const LocalAiModelArtifactText = NonEmptyStringSchema.pipe(Schema.filter((candidate) => isOpaqueLocalModelRef(candidate, LOCAL_MODEL_ARTIFACT_REF_PREFIX) ||
    'Expected local model artifact refs to be opaque ids, not filesystem paths or URLs'));
const LocalAiModelManifestText = NonEmptyStringSchema.pipe(Schema.filter((candidate) => isOpaqueLocalModelRef(candidate, LOCAL_MODEL_MANIFEST_REF_PREFIX) ||
    'Expected local model manifest refs to be opaque ids, not filesystem paths or URLs'));
export const LocalAiModelArtifactRefSchema = LocalAiModelArtifactText.pipe(Schema.brand('LocalAiModelArtifactRef'));
export const LocalAiModelManifestRefSchema = LocalAiModelManifestText.pipe(Schema.brand('LocalAiModelManifestRef'));
export const LocalAiModelSourcePolicySchema = withParser(Schema.Literal('bundled', 'parent-installed', 'local-cache', 'unavailable'));
export const LocalAiModelCacheStateSchema = withParser(Schema.Literal('unavailable', 'not-cached', 'cache-ready', 'cache-degraded', 'cache-corrupted', 'storage-error'));
export const LocalAiModelCacheHealthSchema = withParser(Schema.Literal('healthy', 'degraded', 'unavailable', 'download-disabled', 'corrupted', 'storage-error'));
export const LocalAiModelManifestIntegrityStateSchema = withParser(Schema.Literal('unavailable', 'unchecked', 'verified', 'manifest-missing', 'checksum-mismatch', 'signature-invalid', 'corrupted'));
export const LocalAiModelDownloadStatusSchema = withParser(Schema.Literal('download-disabled', 'download-not-requested', 'download-in-progress', 'download-complete', 'download-failed'));
export const LocalAiModelCacheUnavailableReasonSchema = withParser(Schema.Literal('model-source-unconfigured', 'artifact-not-installed', 'manifest-unavailable', 'download-disabled', 'cache-storage-unavailable', 'integrity-unverified', 'corruption-detected'));
export const LocalAiModelCacheStorageErrorCodeSchema = withParser(Schema.Literal('cache-root-unavailable', 'manifest-read-failed', 'artifact-read-failed', 'metadata-write-disabled', 'storage-permission-denied', 'quota-unavailable'));
export const LocalAiModelCacheCorruptionReasonCodeSchema = withParser(Schema.Literal('manifest-missing', 'checksum-mismatch', 'signature-invalid', 'artifact-missing', 'manifest-artifact-mismatch', 'unknown-integrity'));
function isOpaqueLocalModelRef(candidate, prefix) {
    if (typeof candidate !== 'string' || LOCAL_MODEL_UNSAFE_REF_PATTERNS.some((pattern) => pattern.test(candidate))) {
        return false;
    }
    if (!candidate.startsWith(prefix)) {
        return false;
    }
    return LOCAL_MODEL_OPAQUE_REF_BODY.test(candidate.slice(prefix.length));
}
//# sourceMappingURL=local-ai-model-artifact-primitives.js.map