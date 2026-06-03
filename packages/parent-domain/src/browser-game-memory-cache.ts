import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  BrowserGamePolicyActionCandidateSchema,
  BrowserGamePolicyReasonCodesSchema,
} from './browser-game-policy-compiler-values';
import {
  BrowserGameMemoryCacheEntryIdSchema,
  BrowserGameMemoryCacheKeyKindSchema,
  type BrowserGameMemoryCacheKeyKind,
  BrowserGameMemoryCacheKeyValueSchema,
  BrowserGameMemoryCacheSchemaVersionSchema,
  BrowserGameMemoryCacheSnapshotIdSchema,
  BrowserGameMemoryInvalidationReasonSchema,
  BrowserGameMemorySourceSchema,
  BrowserGameMemoryStateSchema,
  type BrowserGameMemorySubjectKind,
  BrowserGameMemorySubjectKindSchema,
  type BrowserGameMemoryTtlClass,
  BrowserGameMemoryTtlClassSchema,
} from './browser-game-memory-cache-values';

const PositiveBrowserGameMemoryTtlMsSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value > 0 || 'Expected positive browser-game memory TTL milliseconds')
);
const BrowserGameMemoryCacheKeysSchema = Schema.Array(
  Schema.Struct({
    keyKind: BrowserGameMemoryCacheKeyKindSchema,
    keyValue: BrowserGameMemoryCacheKeyValueSchema,
  })
).pipe(Schema.filter((value) => value.length > 0 || 'Expected browser-game memory cache keys'));
const BrowserGameMemorySourceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game memory source evidence refs')
);
const BrowserGameMemoryDecisionRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema);
const BrowserGameMemoryInvalidationReasonsSchema = Schema.Array(BrowserGameMemoryInvalidationReasonSchema);

const BrowserGameMemoryCacheEntryBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameMemoryCacheSchemaVersionSchema,
  entryId: BrowserGameMemoryCacheEntryIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  policyVersionRef: ParentPolicyVersionSchema,
  storedAt: ParentTimestampSchema,
  expiresAt: ParentTimestampSchema,
  ttlMs: PositiveBrowserGameMemoryTtlMsSchema,
  ttlClass: BrowserGameMemoryTtlClassSchema,
  subjectKind: BrowserGameMemorySubjectKindSchema,
  memoryState: BrowserGameMemoryStateSchema,
  decisionSource: BrowserGameMemorySourceSchema,
  actionCandidate: BrowserGamePolicyActionCandidateSchema,
  reasonCodes: BrowserGamePolicyReasonCodesSchema,
  confidence: Schema.Literal('high', 'medium', 'low', 'unknown'),
  cacheKeys: BrowserGameMemoryCacheKeysSchema,
  sourceEvidenceRefs: BrowserGameMemorySourceRefsSchema,
  decisionRefs: BrowserGameMemoryDecisionRefsSchema,
  invalidationReasons: BrowserGameMemoryInvalidationReasonsSchema,
  canReuseForPolicyInput: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeCacheStoreClaimed: Schema.Boolean,
  aiCacheClaimed: Schema.Boolean,
  rawCanonicalUrlStored: Schema.Boolean,
  rawPlatformGameIdStored: Schema.Boolean,
  rawCloudGameTitleStored: Schema.Boolean,
  rawGamePayloadStored: Schema.Boolean,
  rawModelTextStored: Schema.Boolean,
  uiDeliveredClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameMemoryCacheEntryCandidate = Infer<typeof BrowserGameMemoryCacheEntryBaseSchema>;

export const BrowserGameMemoryCacheEntrySchema = withParser(
  BrowserGameMemoryCacheEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        browserGameMemoryCacheEntryIsHonest(entry) ||
        'Expected browser-game memory entry to stay bounded, ref-only, non-final, and non-enforcing'
    )
  )
);

const BrowserGameMemoryCacheSnapshotBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameMemoryCacheSchemaVersionSchema,
  snapshotId: BrowserGameMemoryCacheSnapshotIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  capturedAt: ParentTimestampSchema,
  entries: Schema.Array(BrowserGameMemoryCacheEntrySchema),
  retentionBounded: Schema.Boolean,
  rawGameContentStored: Schema.Boolean,
  runtimeStoreClaimed: Schema.Boolean,
});

type BrowserGameMemoryCacheSnapshotCandidate = Infer<typeof BrowserGameMemoryCacheSnapshotBaseSchema>;

export const BrowserGameMemoryCacheSnapshotSchema = withParser(
  BrowserGameMemoryCacheSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        browserGameMemoryCacheSnapshotIsHonest(snapshot) ||
        'Expected browser-game memory snapshot to cover URL, category, and parent-decision refs without runtime store or raw content claims'
    )
  )
);

export const decodeBrowserGameMemoryCacheEntry = Schema.decodeUnknownSync(BrowserGameMemoryCacheEntrySchema);
export const decodeBrowserGameMemoryCacheSnapshot = Schema.decodeUnknownSync(BrowserGameMemoryCacheSnapshotSchema);

export type BrowserGameMemoryCacheEntry = Infer<typeof BrowserGameMemoryCacheEntrySchema>;
export type BrowserGameMemoryCacheSnapshot = Infer<typeof BrowserGameMemoryCacheSnapshotSchema>;

const RequiredBrowserGameMemorySubjects = [
  'game-url-ref',
  'category-ref',
  'parent-decision-ref',
] as const satisfies ReadonlyArray<BrowserGameMemorySubjectKind>;

function browserGameMemoryCacheSnapshotIsHonest(snapshot: BrowserGameMemoryCacheSnapshotCandidate): boolean {
  const subjects = new Set(snapshot.entries.map((entry) => entry.subjectKind));
  return (
    snapshot.retentionBounded &&
    !snapshot.rawGameContentStored &&
    !snapshot.runtimeStoreClaimed &&
    RequiredBrowserGameMemorySubjects.every((subject) => subjects.has(subject))
  );
}

function browserGameMemoryCacheEntryIsHonest(entry: BrowserGameMemoryCacheEntryCandidate): boolean {
  if (browserGameMemoryCacheEntryClaimsRuntime(entry)) {
    return false;
  }
  if (!browserGameMemoryEntryHasRequiredKeys(entry)) {
    return false;
  }
  if (!browserGameMemoryTtlIsBounded(entry.ttlClass, entry.ttlMs)) {
    return false;
  }
  if (entry.memoryState === 'fresh-hit') {
    return (
      entry.invalidationReasons.length === 0 &&
      entry.canReuseForPolicyInput &&
      entry.decisionRefs.length > 0 &&
      entry.confidence !== 'unknown'
    );
  }
  return (
    !entry.canReuseForPolicyInput &&
    entry.invalidationReasons.length > 0 &&
    (entry.memoryState !== 'miss' || entry.decisionRefs.length === 0)
  );
}

function browserGameMemoryEntryHasRequiredKeys(entry: BrowserGameMemoryCacheEntryCandidate): boolean {
  return (
    browserGameMemoryHasKey(entry, 'policy-version') &&
    browserGameMemoryHasKey(entry, 'child-profile') &&
    browserGameMemoryHasKey(entry, 'parent-rule-set') &&
    browserGameMemoryHasSubjectKey(entry)
  );
}

function browserGameMemoryHasSubjectKey(entry: BrowserGameMemoryCacheEntryCandidate): boolean {
  if (entry.subjectKind === 'game-url-ref') {
    return browserGameMemoryHasKey(entry, 'canonical-url-ref') || browserGameMemoryHasKey(entry, 'domain-path-hash');
  }
  if (entry.subjectKind === 'platform-game-ref') {
    return browserGameMemoryHasKey(entry, 'platform-game-ref');
  }
  if (entry.subjectKind === 'cloud-game-ref') {
    return browserGameMemoryHasKey(entry, 'cloud-game-title-ref');
  }
  if (entry.subjectKind === 'category-ref') {
    return browserGameMemoryHasKey(entry, 'game-category-ref');
  }
  return browserGameMemoryHasKey(entry, 'parent-decision-ref');
}

function browserGameMemoryHasKey(
  entry: BrowserGameMemoryCacheEntryCandidate,
  keyKind: BrowserGameMemoryCacheKeyKind
): boolean {
  return entry.cacheKeys.some((key) => key.keyKind === keyKind);
}

function browserGameMemoryTtlIsBounded(ttlClass: BrowserGameMemoryTtlClass, ttlMs: number): boolean {
  if (ttlClass === 'short-dynamic-game-page' || ttlClass === 'cloud-launcher-page' || ttlClass === 'ugc-game-page') {
    return ttlMs <= 600000;
  }
  if (ttlClass === 'stable-approved-game') {
    return ttlMs <= 86400000;
  }
  return ttlMs <= 43200000;
}

function browserGameMemoryCacheEntryClaimsRuntime(entry: BrowserGameMemoryCacheEntryCandidate): boolean {
  return (
    entry.finalPolicyDecisionClaimed ||
    entry.runtimeCacheStoreClaimed ||
    entry.aiCacheClaimed ||
    entry.rawCanonicalUrlStored ||
    entry.rawPlatformGameIdStored ||
    entry.rawCloudGameTitleStored ||
    entry.rawGamePayloadStored ||
    entry.rawModelTextStored ||
    entry.uiDeliveredClaimed ||
    entry.nativeGameControlClaimed ||
    entry.cloudFrameAnalysisClaimed ||
    entry.enforcementClaimed
  );
}
