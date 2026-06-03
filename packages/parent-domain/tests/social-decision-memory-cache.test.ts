import { describe, expect, it } from 'vitest';
import {
  type SocialDecisionMemoryCacheSnapshot,
  SocialDecisionMemoryCacheSnapshotSchema,
} from '../src/social-decision-memory-cache';

describe('social decision memory cache contracts', () => {
  it('accepts bounded account, video, and channel decision memory refs', acceptsHonestSnapshot);
  it('rejects missing subject keys and missing subject rows', rejectsMissingSubjectProof);
  it('rejects stale, miss, or manual rows that drive policy input', rejectsUnsafeReuse);
  it('rejects raw content, runtime cache, UI, native, connector, and enforcement claims', rejectsRuntimeClaims);
});

function acceptsHonestSnapshot() {
  const parsed = SocialDecisionMemoryCacheSnapshotSchema.parse(validSnapshot());

  expect(parsed.schemaVersion).toBe('social-decision-memory-cache');
  expect(parsed.entries).toHaveLength(3);
  expect(entryState(parsed, 'video-ref')).toEqual({
    memoryState: 'fresh-hit',
    canReuseForPolicyInput: true,
    actionCandidate: 'warn-candidate',
  });
}

function rejectsMissingSubjectProof() {
  const snapshot = validSnapshot();

  expect(
    SocialDecisionMemoryCacheSnapshotSchema.safeParse({
      ...snapshot,
      entries: snapshot.entries.filter((entry) => entry.subjectKind !== 'channel-ref'),
    }).success
  ).toBe(false);

  expect(
    SocialDecisionMemoryCacheSnapshotSchema.safeParse({
      ...snapshot,
      entries: replaceEntry(snapshot, 'account-ref', {
        cacheKeys: baseKeys().filter((key) => key.keyKind !== 'social-account-ref'),
      }),
    }).success
  ).toBe(false);
}

function rejectsUnsafeReuse() {
  const snapshot = validSnapshot();
  const staleReuse = replaceEntry(snapshot, 'channel-ref', { canReuseForPolicyInput: true });
  const missWithDecision = replaceEntry(snapshot, 'account-ref', {
    memoryState: 'miss',
    invalidationReasons: ['ttl-expired'],
    decisionRefs: ['parent-decision-ref-stale'],
  });

  expect(SocialDecisionMemoryCacheSnapshotSchema.safeParse({ ...snapshot, entries: staleReuse }).success).toBe(false);
  expect(SocialDecisionMemoryCacheSnapshotSchema.safeParse({ ...snapshot, entries: missWithDecision }).success).toBe(
    false
  );
}

function rejectsRuntimeClaims() {
  const snapshot = validSnapshot();
  const invalidRows = [
    { finalPolicyDecisionClaimed: true },
    { runtimeCacheStoreClaimed: true },
    { aiCacheClaimed: true },
    { rawAccountDataStored: true },
    { rawVideoContentStored: true },
    { rawMessageContentStored: true },
    { connectorDataStored: true },
    { uiDeliveredClaimed: true },
    { nativeAppControlClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(
      SocialDecisionMemoryCacheSnapshotSchema.safeParse({
        ...snapshot,
        entries: replaceEntry(snapshot, 'video-ref', invalid),
      }).success
    ).toBe(false);
  }
}

function validSnapshot(): SocialDecisionMemoryCacheSnapshot {
  return {
    schemaVersion: 'social-decision-memory-cache',
    snapshotId: 'social-decision-memory-cache-snapshot-1',
    familyId: 'family-social-memory',
    childProfileId: 'child-social-memory',
    capturedAt: '2026-06-03T08:08:00.000Z',
    entries: [accountEntry(), videoEntry(), channelEntry()],
    retentionBounded: true,
    rawContentStored: false,
    runtimeStoreClaimed: false,
  };
}

function accountEntry(): SocialDecisionMemoryCacheSnapshot['entries'][number] {
  return memoryEntry('account-ref', {
    entryId: 'social-memory-account-ref',
    memoryState: 'miss',
    decisionSource: 'unavailable',
    actionCandidate: 'unknown-candidate',
    reasonCodes: ['unknown-evidence'],
    confidence: 'unknown',
    cacheKeys: [...baseKeys(), cacheKey('social-account-ref', 'account-ref-hash-1')],
    decisionRefs: [],
    invalidationReasons: ['ttl-expired'],
    canReuseForPolicyInput: false,
  });
}

function videoEntry(): SocialDecisionMemoryCacheSnapshot['entries'][number] {
  return memoryEntry('video-ref', {
    entryId: 'social-memory-video-ref',
    ttlClass: 'stable-video-decision',
    ttlMs: 86400000,
    actionCandidate: 'warn-candidate',
    reasonCodes: ['social-risk-high', 'video-safety-risk'],
    cacheKeys: [...baseKeys(), cacheKey('platform-video-ref', 'video-ref-123')],
    decisionRefs: ['parent-decision-ref-video'],
  });
}

function channelEntry(): SocialDecisionMemoryCacheSnapshot['entries'][number] {
  return memoryEntry('channel-ref', {
    entryId: 'social-memory-channel-ref',
    memoryState: 'stale-hit',
    ttlClass: 'channel-decision',
    ttlMs: 43200000,
    actionCandidate: 'ask-parent-candidate',
    reasonCodes: ['parent-rule-match'],
    cacheKeys: [...baseKeys(), cacheKey('platform-channel-ref', 'channel-ref-456')],
    decisionRefs: ['parent-decision-ref-channel'],
    invalidationReasons: ['parent-rule-changed'],
    canReuseForPolicyInput: false,
  });
}

function memoryEntry(
  subjectKind: SocialDecisionMemoryCacheSnapshot['entries'][number]['subjectKind'],
  overrides: Partial<SocialDecisionMemoryCacheSnapshot['entries'][number]>
): SocialDecisionMemoryCacheSnapshot['entries'][number] {
  return {
    schemaVersion: 'social-decision-memory-cache',
    entryId: 'social-memory-entry',
    familyId: 'family-social-memory',
    childProfileId: 'child-social-memory',
    policyVersionRef: 'policy-version-social-memory',
    storedAt: '2026-06-03T08:00:00.000Z',
    expiresAt: '2026-06-03T20:00:00.000Z',
    ttlMs: 43200000,
    ttlClass: 'account-decision',
    subjectKind,
    memoryState: 'fresh-hit',
    decisionSource: 'parent-decision-candidate',
    actionCandidate: 'allow-candidate',
    reasonCodes: ['parent-rule-match'],
    confidence: 'medium',
    cacheKeys: baseKeys(),
    sourceEvidenceRefs: ['parent-evidence-social-memory'],
    decisionRefs: ['parent-decision-ref-social-memory'],
    invalidationReasons: [],
    canReuseForPolicyInput: true,
    finalPolicyDecisionClaimed: false,
    runtimeCacheStoreClaimed: false,
    aiCacheClaimed: false,
    rawAccountDataStored: false,
    rawVideoContentStored: false,
    rawMessageContentStored: false,
    connectorDataStored: false,
    uiDeliveredClaimed: false,
    nativeAppControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function baseKeys(): SocialDecisionMemoryCacheSnapshot['entries'][number]['cacheKeys'] {
  return [
    cacheKey('policy-version', 'policy-version-social-memory'),
    cacheKey('child-profile', 'child-social-memory'),
    cacheKey('parent-rule-set', 'parent-rule-set-social-memory'),
  ];
}

function cacheKey(
  keyKind: SocialDecisionMemoryCacheSnapshot['entries'][number]['cacheKeys'][number]['keyKind'],
  keyValue: string
): SocialDecisionMemoryCacheSnapshot['entries'][number]['cacheKeys'][number] {
  return { keyKind, keyValue };
}

function entryState(
  snapshot: SocialDecisionMemoryCacheSnapshot,
  subjectKind: SocialDecisionMemoryCacheSnapshot['entries'][number]['subjectKind']
) {
  const entry = snapshot.entries.find((candidate) => candidate.subjectKind === subjectKind);
  return {
    memoryState: entry?.memoryState,
    canReuseForPolicyInput: entry?.canReuseForPolicyInput,
    actionCandidate: entry?.actionCandidate,
  };
}

function replaceEntry(
  snapshot: SocialDecisionMemoryCacheSnapshot,
  subjectKind: SocialDecisionMemoryCacheSnapshot['entries'][number]['subjectKind'],
  overrides: Partial<SocialDecisionMemoryCacheSnapshot['entries'][number]>
) {
  return snapshot.entries.map((entry) => (entry.subjectKind === subjectKind ? { ...entry, ...overrides } : entry));
}
