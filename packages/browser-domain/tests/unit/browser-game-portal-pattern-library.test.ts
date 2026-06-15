import { describe, expect, it } from 'vitest';
import {
  BrowserGamePortalPatternEntrySchema,
  BrowserGamePortalPatternLibrarySchema,
  type BrowserGamePortalPatternLibrary,
} from '../../src/browser-game-portal-pattern-library';

describe('browser-game portal pattern library contracts', () => {
  it('accepts reviewed portal pattern libraries backed by refs and fingerprints', acceptsReviewedPatternLibrary);
  it('accepts manual-required unknown portal pattern entries', acceptsManualRequiredPatternEntry);
  it('rejects raw domain, URL, title, body, runtime, AI, policy, and enforcement claims', rejectsAuthorityClaims);
  it('rejects inconsistent reviewed and cloud-gaming pattern states', rejectsInconsistentPatternStates);
});

function acceptsReviewedPatternLibrary() {
  const parsed = BrowserGamePortalPatternLibrarySchema.parse(library());

  expect(parsed.schemaVersion).toBe('browser-game-portal-pattern-library-contract');
  expect(parsed.patterns[0]?.portalFamily).toBe('known-game-portal');
  expect(parsed.patterns[0]?.cloudGamingCandidate).toBe(false);
  expect(parsed.runtimeDetectionClaimed).toBe(false);
}

function acceptsManualRequiredPatternEntry() {
  const parsed = BrowserGamePortalPatternEntrySchema.parse(
    patternEntry({
      patternId: 'browser-game-portal-pattern-manual',
      portalFamily: 'unknown',
      routeKinds: ['unknown-route'],
      signalKinds: ['unknown-signal'],
      confidence: 'low',
      reviewState: 'manual-required',
    })
  );

  expect(parsed.reviewState).toBe('manual-required');
}

function rejectsAuthorityClaims() {
  const invalidRows = [
    { rawDomainStored: true },
    { rawUrlStored: true },
    { rawPageTitleStored: true },
    { rawPageBodyStored: true },
    { runtimeDetectionClaimed: true },
    { aiClassificationClaimed: true },
    { policyDecisionClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGamePortalPatternEntrySchema.safeParse(patternEntry(invalid)).success).toBe(false);
    expect(BrowserGamePortalPatternLibrarySchema.safeParse(library(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentPatternStates() {
  const invalidEntries = [
    { portalFamily: 'unknown' },
    { routeKinds: ['unknown-route'] },
    { signalKinds: ['unknown-signal'] },
    { confidence: 'unknown' },
    { cloudGamingCandidate: true },
  ];
  const invalidLibraries = [
    { patterns: [] },
    { confidence: 'unknown' },
    {
      patterns: [
        patternEntry({ reviewState: 'manual-required', portalFamily: 'unknown', routeKinds: ['unknown-route'] }),
      ],
    },
  ];

  for (const invalid of invalidEntries) {
    expect(BrowserGamePortalPatternEntrySchema.safeParse(patternEntry(invalid)).success).toBe(false);
  }
  for (const invalid of invalidLibraries) {
    expect(BrowserGamePortalPatternLibrarySchema.safeParse(library(invalid)).success).toBe(false);
  }
}

function library(overrides = {}): BrowserGamePortalPatternLibrary {
  return {
    schemaVersion: 'browser-game-portal-pattern-library-contract',
    libraryId: 'browser-game-portal-pattern-library-v1',
    generatedAt: '2026-06-03T11:10:00.000Z',
    sourceEvidenceRefs: ['browser-game-portal-library-source-ref'],
    patterns: [patternEntry()],
    confidence: 'high',
    reviewState: 'reviewed',
    rawDomainStored: false,
    rawUrlStored: false,
    rawPageTitleStored: false,
    rawPageBodyStored: false,
    runtimeDetectionClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function patternEntry(overrides = {}) {
  return {
    patternId: 'browser-game-portal-pattern-known',
    portalFamily: 'known-game-portal',
    routeKinds: ['catalog-route', 'game-detail-route', 'play-route'],
    signalKinds: ['domain-ref', 'path-pattern-ref', 'game-id-segment'],
    patternFingerprint: 'browser-game-portal-pattern-fingerprint',
    sourceEvidenceRefs: ['browser-game-portal-pattern-source-ref'],
    confidence: 'high',
    reviewState: 'reviewed',
    educationalCandidate: false,
    ugcCandidate: false,
    purchaseFlowCandidate: false,
    cloudGamingCandidate: false,
    rawDomainStored: false,
    rawUrlStored: false,
    rawPageTitleStored: false,
    rawPageBodyStored: false,
    runtimeDetectionClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
