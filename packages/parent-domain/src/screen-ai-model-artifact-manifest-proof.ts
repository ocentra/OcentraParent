import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiModelArtifactRefSchema,
  LocalAiModelCacheStatusSchema,
  LocalAiModelManifestRefSchema,
} from './local-ai-model-artifacts';
import {
  LocalAiCapabilityFlagSchema,
  LocalAiModelIdSchema,
  LocalAiProviderIdSchema,
  LocalAiTimestampSchema,
} from './local-ai-primitives';
import {
  LocalAiProviderPrivacyModeSchema,
  LocalModelRuntimeStatusSchema,
  LocalProviderCapabilitySchema,
} from './local-ai-runtime';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyScreenAiModelManifestTextSchema = Schema.String.pipe(Schema.minLength(1));
const ScreenAiModelManifestProofIdSchema = NonEmptyScreenAiModelManifestTextSchema.pipe(
  Schema.brand('ScreenAiModelManifestProofId')
);

export const ScreenAiModelManifestClaimBoundarySchema = withParser(
  Schema.Struct({
    remoteProviderUsed: Schema.Literal(false),
    apiProviderUsed: Schema.Literal(false),
    ocentraHostedProcessingUsed: Schema.Literal(false),
    modelQualityClaimed: Schema.Literal(false),
    rawEvidenceEmbedded: Schema.Literal(false),
    executionClaimed: Schema.Literal(false),
  })
);

export const ScreenAiModelManifestPurposeSchema = withParser(Schema.Literal('screen-child-safety-local-analysis'));

const ScreenAiModelArtifactManifestBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: ScreenAiModelManifestProofIdSchema,
  generatedAt: ParentTimestampSchema,
  purpose: ScreenAiModelManifestPurposeSchema,
  providerId: LocalAiProviderIdSchema,
  modelId: LocalAiModelIdSchema,
  artifactRef: LocalAiModelArtifactRefSchema,
  manifestRef: LocalAiModelManifestRefSchema,
  requiredCapability: LocalAiCapabilityFlagSchema,
  privacyMode: LocalAiProviderPrivacyModeSchema,
  cacheStatus: LocalAiModelCacheStatusSchema,
  runtimeStatus: LocalModelRuntimeStatusSchema,
  providerCapability: LocalProviderCapabilitySchema,
  manifestCheckedAt: LocalAiTimestampSchema,
  claimBoundaries: ScreenAiModelManifestClaimBoundarySchema,
});

type ScreenAiModelArtifactManifestCandidate = Infer<typeof ScreenAiModelArtifactManifestBaseSchema>;

export const ScreenAiModelArtifactManifestSchema = withParser(
  ScreenAiModelArtifactManifestBaseSchema.pipe(
    Schema.filter(
      (manifest) =>
        screenAiModelArtifactManifestIsReady(manifest) ||
        'Expected screen AI model manifest to reference a verified local-only artifact without remote, API, quality, or execution claims'
    )
  )
);

export type ScreenAiModelArtifactManifest = Infer<typeof ScreenAiModelArtifactManifestSchema>;

export function buildScreenAiModelArtifactManifestProof(input: unknown): ScreenAiModelArtifactManifest {
  return ScreenAiModelArtifactManifestSchema.parse(input);
}

function screenAiModelArtifactManifestIsReady(manifest: ScreenAiModelArtifactManifestCandidate): boolean {
  return (
    artifactRefsMatch(manifest) &&
    providerRefsMatch(manifest) &&
    cacheIsVerifiedLocalArtifact(manifest) &&
    runtimeIsLocalOnlyAndLoaded(manifest) &&
    capabilityIsLocalAndSupported(manifest) &&
    boundariesStayNonClaiming(manifest)
  );
}

function artifactRefsMatch(manifest: ScreenAiModelArtifactManifestCandidate): boolean {
  return (
    manifest.cacheStatus.artifactRef === manifest.artifactRef &&
    manifest.cacheStatus.manifestRef === manifest.manifestRef &&
    String(manifest.runtimeStatus.modelReference) === String(manifest.artifactRef)
  );
}

function providerRefsMatch(manifest: ScreenAiModelArtifactManifestCandidate): boolean {
  return (
    manifest.runtimeStatus.providerId === manifest.providerId &&
    manifest.runtimeStatus.modelId === manifest.modelId &&
    manifest.providerCapability.providerId === manifest.providerId
  );
}

function cacheIsVerifiedLocalArtifact(manifest: ScreenAiModelArtifactManifestCandidate): boolean {
  return (
    manifest.cacheStatus.sourcePolicy === 'local-cache' &&
    manifest.cacheStatus.cacheState === 'cache-ready' &&
    manifest.cacheStatus.cacheHealth === 'healthy' &&
    manifest.cacheStatus.manifestIntegrity === 'verified'
  );
}

function runtimeIsLocalOnlyAndLoaded(manifest: ScreenAiModelArtifactManifestCandidate): boolean {
  return (
    manifest.runtimeStatus.privacyMode === 'local-only' &&
    manifest.runtimeStatus.providerSource === 'local-model-cache' &&
    manifest.runtimeStatus.loadState === 'loaded' &&
    manifest.runtimeStatus.degradedState === 'none' &&
    manifest.runtimeStatus.unavailableReason === null
  );
}

function capabilityIsLocalAndSupported(manifest: ScreenAiModelArtifactManifestCandidate): boolean {
  return (
    manifest.providerCapability.privacyMode === 'local-only' &&
    manifest.providerCapability.supportedTasks.includes(manifest.requiredCapability)
  );
}

function boundariesStayNonClaiming(manifest: ScreenAiModelArtifactManifestCandidate): boolean {
  return (
    manifest.claimBoundaries.remoteProviderUsed === false &&
    manifest.claimBoundaries.apiProviderUsed === false &&
    manifest.claimBoundaries.ocentraHostedProcessingUsed === false &&
    manifest.claimBoundaries.modelQualityClaimed === false &&
    manifest.claimBoundaries.rawEvidenceEmbedded === false &&
    manifest.claimBoundaries.executionClaimed === false
  );
}
