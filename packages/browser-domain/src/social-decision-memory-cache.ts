import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialParentPolicyActionCandidateSchema,
  SocialParentPolicyReasonCodesSchema,
} from './social-policy-compiler-values';
import {
  SocialDecisionMemoryCacheEntryIdSchema,
  SocialDecisionMemoryCacheKeyKindSchema,
  type SocialDecisionMemoryCacheKeyKind,
  SocialDecisionMemoryCacheKeyValueSchema,
  SocialDecisionMemoryCacheSchemaVersionSchema,
  SocialDecisionMemoryCacheSnapshotIdSchema,
  SocialDecisionMemoryInvalidationReasonSchema,
  SocialDecisionMemorySourceSchema,
  SocialDecisionMemoryStateSchema,
  type SocialDecisionMemorySubjectKind,
  SocialDecisionMemorySubjectKindSchema,
  type SocialDecisionMemoryTtlClass,
  SocialDecisionMemoryTtlClassSchema,
} from './social-decision-memory-cache-values';

const PositiveDecisionMemoryTtlMsSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value > 0 || 'Expected positive social decision memory TTL milliseconds')
);
const SocialDecisionMemoryCacheKeysSchema = Schema.Array(
  Schema.Struct({
    keyKind: SocialDecisionMemoryCacheKeyKindSchema,
    keyValue: SocialDecisionMemoryCacheKeyValueSchema,
  })
).pipe(Schema.filter((value) => value.length > 0 || 'Expected social decision memory cache keys'));
const SocialDecisionMemorySourceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social decision memory source evidence refs')
);
const SocialDecisionMemoryDecisionRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema);
const SocialDecisionMemoryInvalidationReasonsSchema = Schema.Array(SocialDecisionMemoryInvalidationReasonSchema);

const SocialDecisionMemoryCacheEntryBaseSchema = Schema.Struct({
  schemaVersion: SocialDecisionMemoryCacheSchemaVersionSchema,
  entryId: SocialDecisionMemoryCacheEntryIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  policyVersionRef: ParentPolicyVersionSchema,
  storedAt: ParentTimestampSchema,
  expiresAt: ParentTimestampSchema,
  ttlMs: PositiveDecisionMemoryTtlMsSchema,
  ttlClass: SocialDecisionMemoryTtlClassSchema,
  subjectKind: SocialDecisionMemorySubjectKindSchema,
  memoryState: SocialDecisionMemoryStateSchema,
  decisionSource: SocialDecisionMemorySourceSchema,
  actionCandidate: SocialParentPolicyActionCandidateSchema,
  reasonCodes: SocialParentPolicyReasonCodesSchema,
  confidence: Schema.Literal('high', 'medium', 'low', 'unknown'),
  cacheKeys: SocialDecisionMemoryCacheKeysSchema,
  sourceEvidenceRefs: SocialDecisionMemorySourceRefsSchema,
  decisionRefs: SocialDecisionMemoryDecisionRefsSchema,
  invalidationReasons: SocialDecisionMemoryInvalidationReasonsSchema,
  canReuseForPolicyInput: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeCacheStoreClaimed: Schema.Boolean,
  aiCacheClaimed: Schema.Boolean,
  rawAccountDataStored: Schema.Boolean,
  rawVideoContentStored: Schema.Boolean,
  rawMessageContentStored: Schema.Boolean,
  connectorDataStored: Schema.Boolean,
  uiDeliveredClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type SocialDecisionMemoryCacheEntryCandidate = Infer<typeof SocialDecisionMemoryCacheEntryBaseSchema>;

export const SocialDecisionMemoryCacheEntrySchema = withParser(
  SocialDecisionMemoryCacheEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        socialDecisionMemoryCacheEntryIsHonest(entry) ||
        'Expected social decision memory entry to stay bounded, ref-only, non-final, and non-enforcing'
    )
  )
);

const SocialDecisionMemoryCacheSnapshotBaseSchema = Schema.Struct({
  schemaVersion: SocialDecisionMemoryCacheSchemaVersionSchema,
  snapshotId: SocialDecisionMemoryCacheSnapshotIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  capturedAt: ParentTimestampSchema,
  entries: Schema.Array(SocialDecisionMemoryCacheEntrySchema),
  retentionBounded: Schema.Boolean,
  rawContentStored: Schema.Boolean,
  runtimeStoreClaimed: Schema.Boolean,
});

type SocialDecisionMemoryCacheSnapshotCandidate = Infer<typeof SocialDecisionMemoryCacheSnapshotBaseSchema>;

export const SocialDecisionMemoryCacheSnapshotSchema = withParser(
  SocialDecisionMemoryCacheSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        socialDecisionMemoryCacheSnapshotIsHonest(snapshot) ||
        'Expected social decision memory snapshot to cover account, video, and channel refs without runtime store or raw content claims'
    )
  )
);

export const decodeSocialDecisionMemoryCacheEntry = Schema.decodeUnknownSync(SocialDecisionMemoryCacheEntrySchema);
export const decodeSocialDecisionMemoryCacheSnapshot = Schema.decodeUnknownSync(
  SocialDecisionMemoryCacheSnapshotSchema
);

export type SocialDecisionMemoryCacheEntry = Infer<typeof SocialDecisionMemoryCacheEntrySchema>;
export type SocialDecisionMemoryCacheSnapshot = Infer<typeof SocialDecisionMemoryCacheSnapshotSchema>;

const RequiredSocialDecisionMemorySubjects = [
  'account-ref',
  'video-ref',
  'channel-ref',
] as const satisfies ReadonlyArray<SocialDecisionMemorySubjectKind>;

function socialDecisionMemoryCacheSnapshotIsHonest(snapshot: SocialDecisionMemoryCacheSnapshotCandidate): boolean {
  const subjects = new Set(snapshot.entries.map((entry) => entry.subjectKind));
  return (
    snapshot.retentionBounded &&
    !snapshot.rawContentStored &&
    !snapshot.runtimeStoreClaimed &&
    RequiredSocialDecisionMemorySubjects.every((subject) => subjects.has(subject))
  );
}

function socialDecisionMemoryCacheEntryIsHonest(entry: SocialDecisionMemoryCacheEntryCandidate): boolean {
  if (socialDecisionMemoryCacheEntryClaimsRuntime(entry)) {
    return false;
  }
  if (!socialDecisionMemoryEntryHasRequiredKeys(entry)) {
    return false;
  }
  if (!socialDecisionMemoryTtlIsBounded(entry.ttlClass, entry.ttlMs)) {
    return false;
  }
  if (entry.memoryState === 'fresh-hit') {
    return entry.invalidationReasons.length === 0 && entry.canReuseForPolicyInput && entry.decisionRefs.length > 0;
  }
  return (
    !entry.canReuseForPolicyInput &&
    entry.invalidationReasons.length > 0 &&
    (entry.memoryState !== 'miss' || entry.decisionRefs.length === 0)
  );
}

function socialDecisionMemoryEntryHasRequiredKeys(entry: SocialDecisionMemoryCacheEntryCandidate): boolean {
  return (
    socialDecisionMemoryHasKey(entry, 'policy-version') &&
    socialDecisionMemoryHasKey(entry, 'child-profile') &&
    socialDecisionMemoryHasKey(entry, 'parent-rule-set') &&
    socialDecisionMemoryHasSubjectKey(entry)
  );
}

function socialDecisionMemoryHasSubjectKey(entry: SocialDecisionMemoryCacheEntryCandidate): boolean {
  if (entry.subjectKind === 'account-ref') {
    return socialDecisionMemoryHasKey(entry, 'social-account-ref');
  }
  if (entry.subjectKind === 'video-ref') {
    return socialDecisionMemoryHasKey(entry, 'platform-video-ref');
  }
  return socialDecisionMemoryHasKey(entry, 'platform-channel-ref');
}

function socialDecisionMemoryHasKey(
  entry: SocialDecisionMemoryCacheEntryCandidate,
  keyKind: SocialDecisionMemoryCacheKeyKind
): boolean {
  return entry.cacheKeys.some((key) => key.keyKind === keyKind);
}

function socialDecisionMemoryTtlIsBounded(ttlClass: SocialDecisionMemoryTtlClass, ttlMs: number): boolean {
  if (ttlClass === 'stable-video-decision') {
    return ttlMs <= 86400000;
  }
  if (ttlClass === 'dynamic-feed-decision') {
    return ttlMs <= 600000;
  }
  return ttlMs <= 43200000;
}

function socialDecisionMemoryCacheEntryClaimsRuntime(entry: SocialDecisionMemoryCacheEntryCandidate): boolean {
  return (
    entry.finalPolicyDecisionClaimed ||
    entry.runtimeCacheStoreClaimed ||
    entry.aiCacheClaimed ||
    entry.rawAccountDataStored ||
    entry.rawVideoContentStored ||
    entry.rawMessageContentStored ||
    entry.connectorDataStored ||
    entry.uiDeliveredClaimed ||
    entry.nativeAppControlClaimed ||
    entry.enforcementClaimed
  );
}
