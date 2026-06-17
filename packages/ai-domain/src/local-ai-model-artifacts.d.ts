import { type Infer, Schema } from '@ocentra-parent/schema-domain/effect';
export * from './local-ai-model-artifact-primitives';
export declare const LocalAiModelCacheStatusKindSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["local-model-cache-status"]>>;
export declare const LocalAiModelCacheStatusSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    statusKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["local-model-cache-status"]>>;
    artifactRef: Schema.brand<Schema.filter<Schema.filter<typeof Schema.String>>, "LocalAiModelArtifactRef">;
    manifestRef: Schema.Union<[Schema.brand<Schema.filter<Schema.filter<typeof Schema.String>>, "LocalAiModelManifestRef">, typeof Schema.Null]>;
    sourcePolicy: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["bundled", "parent-installed", "local-cache", "unavailable"]>>;
    cacheState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["unavailable", "not-cached", "cache-ready", "cache-degraded", "cache-corrupted", "storage-error"]>>;
    cacheHealth: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["healthy", "degraded", "unavailable", "download-disabled", "corrupted", "storage-error"]>>;
    manifestIntegrity: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["unavailable", "unchecked", "verified", "manifest-missing", "checksum-mismatch", "signature-invalid", "corrupted"]>>;
    downloadEnabled: typeof Schema.Boolean;
    downloadStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["download-disabled", "download-not-requested", "download-in-progress", "download-complete", "download-failed"]>>;
    cacheByteSize: Schema.filter<Schema.filter<typeof Schema.Number>>;
    checkedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    unavailableReason: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["model-source-unconfigured", "artifact-not-installed", "manifest-unavailable", "download-disabled", "cache-storage-unavailable", "integrity-unverified", "corruption-detected"]>>, typeof Schema.Null]>;
    storageError: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["cache-root-unavailable", "manifest-read-failed", "artifact-read-failed", "metadata-write-disabled", "storage-permission-denied", "quota-unavailable"]>>, typeof Schema.Null]>;
    corruptionReason: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["manifest-missing", "checksum-mismatch", "signature-invalid", "artifact-missing", "manifest-artifact-mismatch", "unknown-integrity"]>>, typeof Schema.Null]>;
}>>>;
export type LocalAiModelCacheStatusKind = Infer<typeof LocalAiModelCacheStatusKindSchema>;
export type LocalAiModelCacheStatus = Infer<typeof LocalAiModelCacheStatusSchema>;
//# sourceMappingURL=local-ai-model-artifacts.d.ts.map