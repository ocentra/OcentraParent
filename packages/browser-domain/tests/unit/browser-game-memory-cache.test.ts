import { describe, expect, it } from 'vitest';
import {
  type BrowserGameMemoryCacheSnapshot,
  BrowserGameMemoryCacheSnapshotSchema,
} from '@ocentra-parent/schema-domain/browser-game-memory-cache';

describe('browser-game memory cache contracts', () => {
  it('accepts bounded URL, category, and parent-decision memory refs', acceptsHonestSnapshot);
  it('rejects missing subject keys and missing subject rows', rejectsMissingSubjectProof);
  it('rejects stale, miss, or manual rows that drive policy input', rejectsUnsafeReuse);
  it(
    'rejects raw URL/title/game payload, runtime cache, AI, UI, native, cloud, and enforcement claims',
    rejectsRuntimeClaims
  );
});

function acceptsHonestSnapshot() {
  const parsed = BrowserGameMemoryCacheSnapshotSchema.parse(validSnapshot());

  expect(parsed.schemaVersion).toBe('browser-game-memory-cache-contract');
  expect(parsed.entries).toHaveLength(3);
  expect(entryState(parsed, 'game-url-ref')).toEqual({
    memoryState: 'fresh-hit',
    canReuseForPolicyInput: true,
    actionCandidate: 'allow-candidate',
  });
}

function rejectsMissingSubjectProof() {
  const snapshot = validSnapshot();

  expect(
    BrowserGameMemoryCacheSnapshotSchema.safeParse({
      ...snapshot,
      entries: snapshot.entries.filter((entry) => entry.subjectKind !== 'category-ref'),
    }).success
  ).toBe(false);

  expect(
    BrowserGameMemoryCacheSnapshotSchema.safeParse({
      ...snapshot,
      entries: replaceEntry(snapshot, 'game-url-ref', {
        cacheKeys: baseKeys().filter((key) => key.keyKind !== 'canonical-url-ref'),
      }),
    }).success
  ).toBe(false);
}

function rejectsUnsafeReuse() {
  const snapshot = validSnapshot();
  const staleReuse = replaceEntry(snapshot, 'parent-decision-ref', { canReuseForPolicyInput: true });
  const missWithDecision = replaceEntry(snapshot, 'category-ref', {
    memoryState: 'miss',
    invalidationReasons: ['ttl-expired'],
    decisionRefs: ['browser-game-memory-decision-stale'],
  });

  expect(BrowserGameMemoryCacheSnapshotSchema.safeParse({ ...snapshot, entries: staleReuse }).success).toBe(false);
  expect(BrowserGameMemoryCacheSnapshotSchema.safeParse({ ...snapshot, entries: missWithDecision }).success).toBe(
    false
  );
}

function rejectsRuntimeClaims() {
  const snapshot = validSnapshot();
  const invalidRows = [
    { finalPolicyDecisionClaimed: true },
    { runtimeCacheStoreClaimed: true },
    { aiCacheClaimed: true },
    { rawCanonicalUrlStored: true },
    { rawPlatformGameIdStored: true },
    { rawCloudGameTitleStored: true },
    { rawGamePayloadStored: true },
    { rawModelTextStored: true },
    { uiDeliveredClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(
      BrowserGameMemoryCacheSnapshotSchema.safeParse({
        ...snapshot,
        entries: replaceEntry(snapshot, 'game-url-ref', invalid),
      }).success
    ).toBe(false);
  }
}

function validSnapshot(): BrowserGameMemoryCacheSnapshot {
  return {
    schemaVersion: 'browser-game-memory-cache-contract',
    snapshotId: 'browser-game-memory-cache-snapshot-1',
    familyId: 'family-browser-game-memory',
    childProfileId: 'child-browser-game-memory',
    capturedAt: '2026-06-03T10:37:00.000Z',
    entries: [gameUrlEntry(), categoryEntry(), parentDecisionEntry()],
    retentionBounded: true,
    rawGameContentStored: false,
    runtimeStoreClaimed: false,
  };
}

function gameUrlEntry(): BrowserGameMemoryCacheSnapshot['entries'][number] {
  return memoryEntry('game-url-ref', {
    entryId: 'browser-game-memory-url-ref',
    ttlClass: 'stable-approved-game',
    ttlMs: 86400000,
    actionCandidate: 'allow-candidate',
    reasonCodes: ['educational-benefit-present', 'parent-rule-match'],
    cacheKeys: [...baseKeys(), cacheKey('canonical-url-ref', 'canonical-url-ref-hash-1')],
    decisionRefs: ['browser-game-memory-parent-decision-ref'],
  });
}

function categoryEntry(): BrowserGameMemoryCacheSnapshot['entries'][number] {
  return memoryEntry('category-ref', {
    entryId: 'browser-game-memory-category-ref',
    memoryState: 'miss',
    ttlMs: 600000,
    decisionSource: 'unavailable',
    actionCandidate: 'unknown-candidate',
    reasonCodes: ['unknown-evidence'],
    confidence: 'unknown',
    cacheKeys: [...baseKeys(), cacheKey('game-category-ref', 'game-category-ref-educational')],
    decisionRefs: [],
    invalidationReasons: ['ttl-expired'],
    canReuseForPolicyInput: false,
  });
}

function parentDecisionEntry(): BrowserGameMemoryCacheSnapshot['entries'][number] {
  return memoryEntry('parent-decision-ref', {
    entryId: 'browser-game-memory-parent-decision-ref',
    memoryState: 'stale-hit',
    ttlClass: 'parent-approved-account-page',
    ttlMs: 43200000,
    actionCandidate: 'parent-review-candidate',
    reasonCodes: ['parent-rule-match'],
    cacheKeys: [...baseKeys(), cacheKey('parent-decision-ref', 'parent-decision-browser-game-ref')],
    decisionRefs: ['browser-game-memory-parent-decision-ref'],
    invalidationReasons: ['parent-rule-changed'],
    canReuseForPolicyInput: false,
  });
}

function memoryEntry(
  subjectKind: BrowserGameMemoryCacheSnapshot['entries'][number]['subjectKind'],
  overrides: Partial<BrowserGameMemoryCacheSnapshot['entries'][number]>
): BrowserGameMemoryCacheSnapshot['entries'][number] {
  return {
    schemaVersion: 'browser-game-memory-cache-contract',
    entryId: 'browser-game-memory-entry',
    familyId: 'family-browser-game-memory',
    childProfileId: 'child-browser-game-memory',
    policyVersionRef: 'policy-version-browser-game-memory',
    storedAt: '2026-06-03T10:30:00.000Z',
    expiresAt: '2026-06-03T22:30:00.000Z',
    ttlMs: 43200000,
    ttlClass: 'short-dynamic-game-page',
    subjectKind,
    memoryState: 'fresh-hit',
    decisionSource: 'parent-decision-candidate',
    actionCandidate: 'parent-review-candidate',
    reasonCodes: ['parent-rule-match'],
    confidence: 'medium',
    cacheKeys: baseKeys(),
    sourceEvidenceRefs: ['parent-evidence-browser-game-memory'],
    decisionRefs: ['browser-game-memory-decision-ref'],
    invalidationReasons: [],
    canReuseForPolicyInput: true,
    finalPolicyDecisionClaimed: false,
    runtimeCacheStoreClaimed: false,
    aiCacheClaimed: false,
    rawCanonicalUrlStored: false,
    rawPlatformGameIdStored: false,
    rawCloudGameTitleStored: false,
    rawGamePayloadStored: false,
    rawModelTextStored: false,
    uiDeliveredClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function baseKeys(): BrowserGameMemoryCacheSnapshot['entries'][number]['cacheKeys'] {
  return [
    cacheKey('policy-version', 'policy-version-browser-game-memory'),
    cacheKey('child-profile', 'child-browser-game-memory'),
    cacheKey('parent-rule-set', 'parent-rule-set-browser-game-memory'),
  ];
}

function cacheKey(
  keyKind: BrowserGameMemoryCacheSnapshot['entries'][number]['cacheKeys'][number]['keyKind'],
  keyValue: string
): BrowserGameMemoryCacheSnapshot['entries'][number]['cacheKeys'][number] {
  return { keyKind, keyValue };
}

function entryState(
  snapshot: BrowserGameMemoryCacheSnapshot,
  subjectKind: BrowserGameMemoryCacheSnapshot['entries'][number]['subjectKind']
) {
  const entry = snapshot.entries.find((candidate) => candidate.subjectKind === subjectKind);
  return {
    memoryState: entry?.memoryState,
    canReuseForPolicyInput: entry?.canReuseForPolicyInput,
    actionCandidate: entry?.actionCandidate,
  };
}

function replaceEntry(
  snapshot: BrowserGameMemoryCacheSnapshot,
  subjectKind: BrowserGameMemoryCacheSnapshot['entries'][number]['subjectKind'],
  overrides: Partial<BrowserGameMemoryCacheSnapshot['entries'][number]>
) {
  return snapshot.entries.map((entry) => (entry.subjectKind === subjectKind ? { ...entry, ...overrides } : entry));
}
