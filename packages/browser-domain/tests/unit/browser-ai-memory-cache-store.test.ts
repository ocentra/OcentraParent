import { describe, expect, it } from 'vitest';
import {
  BrowserAiMemoryCacheEntrySchema,
  BrowserAiMemoryCacheStoreSchemaVersion,
  BrowserAiMemoryCacheStoreSnapshotSchema,
} from '../../src/browser-ai-memory-cache-store-schemas';
import { BrowserUrlIntelligenceMemorySchemaVersion } from '../../src/browser-url-intelligence-schemas';

describe('browser AI memory cache store contract', () => {
  it('accepts fresh video cache entries with complete keys', acceptsFreshVideoCacheEntry);
  it('rejects fresh entries missing policy model or child keys', rejectsIncompleteFreshKeys);
  it('accepts stale prompt-changed entries that cannot drive policy input', acceptsStaleInvalidatedEntry);
  it('rejects stale entries that still drive policy input', rejectsStalePolicyInput);
  it('rejects dynamic feed entries with long TTL', rejectsLongDynamicFeedTtl);
  it('accepts bounded store snapshots without raw content', acceptsBoundedStoreSnapshot);
  it('rejects snapshots that store raw content or claim unbounded retention', rejectsRawContentSnapshot);
});

function acceptsFreshVideoCacheEntry() {
  const parsed = BrowserAiMemoryCacheEntrySchema.safeParse(freshEntry());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.canDrivePolicyInput).toBe(true);
    expect(parsed.data.cacheKeys.map((key) => key.keyKind)).toContain('model-prompt-version');
    expect(parsed.data.directEnforcementClaimed).toBe(false);
  }
}

function rejectsIncompleteFreshKeys() {
  const parsed = BrowserAiMemoryCacheEntrySchema.safeParse({
    ...freshEntry(),
    cacheKeys: [{ keyKind: 'platform-video-id', keyValue: 'abc123' }],
  });

  expect(parsed.success).toBe(false);
}

function acceptsStaleInvalidatedEntry() {
  const parsed = BrowserAiMemoryCacheEntrySchema.safeParse(staleEntry());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.invalidationReasons).toEqual(['prompt-changed']);
    expect(parsed.data.canDrivePolicyInput).toBe(false);
  }
}

function rejectsStalePolicyInput() {
  const parsed = BrowserAiMemoryCacheEntrySchema.safeParse({
    ...staleEntry(),
    canDrivePolicyInput: true,
  });

  expect(parsed.success).toBe(false);
}

function rejectsLongDynamicFeedTtl() {
  const parsed = BrowserAiMemoryCacheEntrySchema.safeParse({
    ...freshEntry(),
    ttlClass: 'dynamic-feed',
    ttlMs: 86400000,
  });

  expect(parsed.success).toBe(false);
}

function acceptsBoundedStoreSnapshot() {
  const parsed = BrowserAiMemoryCacheStoreSnapshotSchema.safeParse(storeSnapshot());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.retentionBounded).toBe(true);
    expect(parsed.data.rawContentStored).toBe(false);
  }
}

function rejectsRawContentSnapshot() {
  const parsed = BrowserAiMemoryCacheStoreSnapshotSchema.safeParse({
    ...storeSnapshot(),
    retentionBounded: false,
    rawContentStored: true,
  });

  expect(parsed.success).toBe(false);
}

function storeSnapshot() {
  return {
    schemaVersion: BrowserAiMemoryCacheStoreSchemaVersion,
    storeId: 'browser-ai-memory-cache-store-local',
    capturedAt: '2026-06-03T03:55:00.000Z',
    entries: [freshEntry()],
    retentionBounded: true,
    rawContentStored: false,
  };
}

function freshEntry() {
  return {
    schemaVersion: BrowserAiMemoryCacheStoreSchemaVersion,
    entryId: 'browser-ai-memory-cache-entry-video',
    storedAt: '2026-06-03T03:54:00.000Z',
    expiresAt: '2026-06-04T03:54:00.000Z',
    ttlMs: 86400000,
    ttlClass: 'stable-video',
    cacheKeys: cacheKeys(),
    memoryHit: memoryHit(),
    invalidationReasons: [],
    canDrivePolicyInput: true,
    directEnforcementClaimed: false,
  };
}

function staleEntry() {
  return {
    ...freshEntry(),
    memoryHit: {
      ...memoryHit(),
      hitState: 'stale',
      staleReason: 'prompt-changed',
      canDrivePolicyInput: false,
    },
    invalidationReasons: ['prompt-changed'],
    canDrivePolicyInput: false,
  };
}

function cacheKeys() {
  return [
    { keyKind: 'platform-video-id', keyValue: 'abc123' },
    { keyKind: 'metadata-hash', keyValue: 'metadata-hash-abc123' },
    { keyKind: 'model-prompt-version', keyValue: 'local-model-ref:prompt-v1' },
    { keyKind: 'policy-version', keyValue: 'browser-policy-version-2026-06-03' },
    { keyKind: 'child-profile', keyValue: 'child-profile-middle-school' },
  ];
}

function memoryHit() {
  return {
    schemaVersion: BrowserUrlIntelligenceMemorySchemaVersion,
    memoryHitId: 'browser-url-memory-hit-video',
    lookedUpAt: '2026-06-03T03:54:00.000Z',
    key: { keyKind: 'platform-video-id', keyValue: 'abc123' },
    hitState: 'hit',
    decisionKind: 'known-allowed',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    analysisRef: 'browser-ai-analysis-result-youtube-video',
    parentActionRef: null,
    policyVersionRef: 'browser-policy-version-2026-06-03',
    expiresAt: '2026-06-04T03:54:00.000Z',
    staleReason: null,
    canDrivePolicyInput: true,
    canDirectlyEnforce: false,
  };
}
