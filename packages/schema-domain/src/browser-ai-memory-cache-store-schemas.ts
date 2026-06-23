import { type Infer, Schema, withParser, NonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserUrlIntelligenceMemoryHitSchema } from './browser-url-intelligence-schemas';
import {
  BrowserAiMemoryCacheEntryIdSchema,
  BrowserAiMemoryCacheInvalidationReasonSchema,
  type BrowserAiMemoryCacheKeyKind,
  BrowserAiMemoryCacheKeyKindSchema,
  BrowserAiMemoryCacheStoreIdSchema,
  type BrowserAiMemoryCacheTtlClass,
  BrowserAiMemoryCacheTtlClassSchema,
} from './browser-ai-memory-cache-store-values';

const PositiveCacheTtlMsSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value > 0 || 'Expected positive cache TTL milliseconds')
);
const MemoryCacheKeysSchema = Schema.Array(
  Schema.Struct({
    keyKind: BrowserAiMemoryCacheKeyKindSchema,
    keyValue: NonEmptyStringSchema,
  })
).pipe(Schema.filter((value) => value.length > 0 || 'Expected at least one memory cache key'));
const InvalidationReasonsSchema = Schema.Array(BrowserAiMemoryCacheInvalidationReasonSchema);

export const BrowserAiMemoryCacheStoreSchemaVersion = 1;

const BrowserAiMemoryCacheEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiMemoryCacheStoreSchemaVersion),
  entryId: BrowserAiMemoryCacheEntryIdSchema,
  storedAt: ActivityTimestampSchema,
  expiresAt: ActivityTimestampSchema,
  ttlMs: PositiveCacheTtlMsSchema,
  ttlClass: BrowserAiMemoryCacheTtlClassSchema,
  cacheKeys: MemoryCacheKeysSchema,
  memoryHit: BrowserUrlIntelligenceMemoryHitSchema,
  invalidationReasons: InvalidationReasonsSchema,
  canDrivePolicyInput: Schema.Boolean,
  directEnforcementClaimed: Schema.Boolean,
});
export const BrowserAiMemoryCacheEntrySchema = withParser(
  BrowserAiMemoryCacheEntryBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiMemoryCacheEntryIsConsistent(value) ||
        'Expected memory cache entry to be complete, bounded, and non-enforcing'
    )
  )
);

const BrowserAiMemoryCacheStoreSnapshotBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiMemoryCacheStoreSchemaVersion),
  storeId: BrowserAiMemoryCacheStoreIdSchema,
  capturedAt: ActivityTimestampSchema,
  entries: Schema.Array(BrowserAiMemoryCacheEntrySchema),
  retentionBounded: Schema.Boolean,
  rawContentStored: Schema.Boolean,
});
export const BrowserAiMemoryCacheStoreSnapshotSchema = withParser(
  BrowserAiMemoryCacheStoreSnapshotBaseSchema.pipe(
    Schema.filter(
      (value) =>
        (value.retentionBounded && !value.rawContentStored) ||
        'Expected cache store snapshot to avoid raw content storage'
    )
  )
);

export const decodeBrowserAiMemoryCacheEntry = Schema.decodeUnknownSync(BrowserAiMemoryCacheEntrySchema);
export const decodeBrowserAiMemoryCacheStoreSnapshot = Schema.decodeUnknownSync(
  BrowserAiMemoryCacheStoreSnapshotSchema
);

export type BrowserAiMemoryCacheEntry = Infer<typeof BrowserAiMemoryCacheEntrySchema>;
export type BrowserAiMemoryCacheStoreSnapshot = Infer<typeof BrowserAiMemoryCacheStoreSnapshotSchema>;

function browserAiMemoryCacheEntryIsConsistent(value: Infer<typeof BrowserAiMemoryCacheEntryBaseSchema>) {
  if (value.directEnforcementClaimed || value.canDrivePolicyInput !== value.memoryHit.canDrivePolicyInput) {
    return false;
  }
  if (!cacheEntryHasRequiredKeys(value.cacheKeys)) {
    return false;
  }
  if (!ttlClassIsBounded(value.ttlClass, value.ttlMs)) {
    return false;
  }
  if (value.memoryHit.hitState === 'hit') {
    return value.invalidationReasons.length === 0 && value.canDrivePolicyInput;
  }
  return value.invalidationReasons.length > 0 && !value.canDrivePolicyInput;
}

function cacheEntryHasRequiredKeys(keys: Infer<typeof MemoryCacheKeysSchema>) {
  return (
    hasKey(keys, 'model-prompt-version') &&
    hasKey(keys, 'policy-version') &&
    hasKey(keys, 'child-profile') &&
    (hasKey(keys, 'canonical-url') || hasKey(keys, 'platform-video-id') || hasKey(keys, 'normalized-origin-path'))
  );
}

function hasKey(keys: Infer<typeof MemoryCacheKeysSchema>, keyKind: BrowserAiMemoryCacheKeyKind) {
  return keys.some((key) => key.keyKind === keyKind);
}

function ttlClassIsBounded(ttlClass: BrowserAiMemoryCacheTtlClass, ttlMs: number) {
  if (ttlClass === 'stable-video' || ttlClass === 'parent-approved-educational') {
    return ttlMs <= 86400000;
  }
  return ttlMs <= 600000;
}
