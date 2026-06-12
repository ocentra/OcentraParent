import { describe, expect, it } from 'vitest';
import {
  BrowserGameCloudPatternEntrySchema,
  BrowserGameCloudPatternLibrarySchema,
  type BrowserGameCloudPatternEntry,
  type BrowserGameCloudPatternLibrary,
} from '../../src/browser-game-cloud-pattern-library';

describe('browser-game cloud pattern library contracts', () => {
  it('accepts reviewed cloud-gaming platform patterns as refs and fingerprints', acceptsReviewedPatterns);
  it('accepts manual-required and unavailable cloud pattern rows', acceptsManualRequiredRows);
  it('accepts reviewed and manual-required libraries without runtime claims', acceptsLibraries);
  it('rejects raw cloud data, stream-frame, native-control, policy, and enforcement claims', rejectsAuthorityClaims);
  it('rejects inconsistent reviewed rows, signal mismatches, and library upgrades', rejectsInconsistentRows);
});

function acceptsReviewedPatterns() {
  expect(BrowserGameCloudPatternEntrySchema.safeParse(cloudPattern()).success).toBe(true);
  expect(
    BrowserGameCloudPatternEntrySchema.safeParse(
      cloudPattern({
        patternId: 'cloud-pattern-native-launcher',
        platform: 'shadow-cloud-pc',
        cloudFamily: 'cloud-pc-platform',
        routeKinds: ['cloud-launch-route', 'cloud-session-route'],
        signalKinds: ['domain-ref', 'streaming-session-route', 'native-launcher-prompt'],
        titleMetadataCandidate: false,
        nativeLauncherPromptCandidate: true,
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameCloudPatternEntrySchema.safeParse(
      cloudPattern({
        patternId: 'cloud-pattern-subscription',
        routeKinds: ['cloud-account-route', 'cloud-subscription-route'],
        signalKinds: ['domain-ref', 'path-pattern-ref', 'subscription-prompt'],
        sessionCandidate: false,
        titleMetadataCandidate: false,
        subscriptionOrAccountCandidate: true,
      })
    ).success
  ).toBe(true);
}

function acceptsManualRequiredRows() {
  expect(BrowserGameCloudPatternEntrySchema.safeParse(manualPattern()).success).toBe(true);
  expect(
    BrowserGameCloudPatternEntrySchema.safeParse(
      manualPattern({
        patternId: 'cloud-pattern-unavailable',
        reviewState: 'unavailable',
        confidence: 'unknown',
      })
    ).success
  ).toBe(true);
}

function acceptsLibraries() {
  expect(BrowserGameCloudPatternLibrarySchema.safeParse(cloudLibrary()).success).toBe(true);
  expect(
    BrowserGameCloudPatternLibrarySchema.safeParse(
      cloudLibrary({
        reviewState: 'manual-required',
        confidence: 'low',
        patterns: [manualPattern()],
      })
    ).success
  ).toBe(true);
}

function rejectsAuthorityClaims() {
  const invalidEntryClaims = [
    { rawCloudDomainStored: true },
    { rawCloudUrlStored: true },
    { rawCloudTitleStored: true },
    { rawStreamFrameStored: true },
    { runtimeDetectionClaimed: true },
    { cloudStreamFrameAnalysisClaimed: true },
    { perGameCloudTitleCertaintyClaimed: true },
    { nativeLauncherControlClaimed: true },
    { nativeGameControlClaimed: true },
    { policyDecisionClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidEntryClaims) {
    expect(BrowserGameCloudPatternEntrySchema.safeParse(cloudPattern(invalid)).success).toBe(false);
    expect(BrowserGameCloudPatternLibrarySchema.safeParse(cloudLibrary(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentRows() {
  const invalidRows = [
    { platform: 'unknown-cloud-gaming' },
    { cloudFamily: 'unknown' },
    { routeKinds: ['unknown-route'] },
    { signalKinds: ['unknown-signal'] },
    { reviewState: 'manual-required', confidence: 'high', platform: 'unknown-cloud-gaming' },
    { sessionCandidate: true, routeKinds: ['cloud-title-route'] },
    { titleMetadataCandidate: true, signalKinds: ['domain-ref', 'streaming-session-route'] },
    { ratingMetadataCandidate: true, signalKinds: ['domain-ref', 'streaming-session-route'] },
    { subscriptionOrAccountCandidate: true, routeKinds: ['cloud-title-route'], signalKinds: ['domain-ref'] },
    { nativeLauncherPromptCandidate: true, signalKinds: ['domain-ref', 'streaming-session-route'] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameCloudPatternEntrySchema.safeParse(cloudPattern(invalid)).success).toBe(false);
  }

  expect(
    BrowserGameCloudPatternLibrarySchema.safeParse(
      cloudLibrary({
        patterns: [manualPattern()],
      })
    ).success
  ).toBe(false);
  expect(
    BrowserGameCloudPatternLibrarySchema.safeParse(
      cloudLibrary({
        reviewState: 'manual-required',
        confidence: 'high',
        patterns: [manualPattern()],
      })
    ).success
  ).toBe(false);
}

function cloudPattern(overrides = {}): BrowserGameCloudPatternEntry {
  return {
    patternId: 'cloud-pattern-xbox-session',
    platform: 'xbox-cloud-gaming',
    cloudFamily: 'cloud-gaming-platform',
    routeKinds: ['cloud-session-route', 'cloud-title-route'],
    signalKinds: ['domain-ref', 'streaming-session-route', 'gamepad-api', 'platform-title-metadata-ref'],
    patternFingerprint: 'cloud-pattern-fingerprint-xbox-session',
    sourceEvidenceRefs: ['cloud-pattern-evidence-route', 'cloud-pattern-evidence-signal'],
    confidence: 'high',
    reviewState: 'reviewed',
    sessionCandidate: true,
    titleMetadataCandidate: true,
    ratingMetadataCandidate: false,
    subscriptionOrAccountCandidate: false,
    nativeLauncherPromptCandidate: false,
    rawCloudDomainStored: false,
    rawCloudUrlStored: false,
    rawCloudTitleStored: false,
    rawStreamFrameStored: false,
    runtimeDetectionClaimed: false,
    cloudStreamFrameAnalysisClaimed: false,
    perGameCloudTitleCertaintyClaimed: false,
    nativeLauncherControlClaimed: false,
    nativeGameControlClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function manualPattern(overrides = {}): BrowserGameCloudPatternEntry {
  return cloudPattern({
    patternId: 'cloud-pattern-manual',
    platform: 'unknown-cloud-gaming',
    cloudFamily: 'unknown',
    routeKinds: ['unknown-route'],
    signalKinds: ['unknown-signal'],
    confidence: 'low',
    reviewState: 'manual-required',
    sessionCandidate: false,
    titleMetadataCandidate: false,
    ...overrides,
  });
}

function cloudLibrary(overrides = {}): BrowserGameCloudPatternLibrary {
  return {
    schemaVersion: 'browser-game-cloud-pattern-library-contract',
    libraryId: 'cloud-pattern-library-reviewed',
    generatedAt: '2026-06-03T11:20:00.000Z',
    sourceEvidenceRefs: ['cloud-pattern-library-evidence'],
    patterns: [cloudPattern()],
    confidence: 'high',
    reviewState: 'reviewed',
    rawCloudDomainStored: false,
    rawCloudUrlStored: false,
    rawCloudTitleStored: false,
    rawStreamFrameStored: false,
    runtimeDetectionClaimed: false,
    cloudStreamFrameAnalysisClaimed: false,
    perGameCloudTitleCertaintyClaimed: false,
    nativeLauncherControlClaimed: false,
    nativeGameControlClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
