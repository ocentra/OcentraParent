import { describe, expect, it } from 'vitest';
import {
  BrowserSocialUnmanagedBypassEvidenceSchema,
  detectBrowserSocialUnmanagedBypass,
} from '@ocentra-parent/schema-domain/browser-social-unmanaged-bypass-detector';

describe('browser social unmanaged bypass detector contract', () => {
  it('detects possible social bypass process evidence without exact URL claims', detectsSocialBypassProcess);
  it('detects supported browser outside the managed session as bypass-only', detectsSupportedBrowserBypass);
  it('rejects path leaks, exact URL proof, and runtime fallback actions', rejectsOverclaims);
  it('rejects social proof, UI, connector, native, and enforcement claims', rejectsRuntimeClaims);
});

function detectsSocialBypassProcess() {
  const evidence = detectBrowserSocialUnmanagedBypass(bypassInput());

  expect(evidence.targetState).toBe('bypass-detected');
  expect(evidence.managedBrowserRequired).toBe(true);
  expect(evidence.bypassOnly).toBe(true);
  expect(evidence.exactUrlClaimed).toBe(false);
  expect(evidence.feedVideoRouteClaimed).toBe(false);
}

function detectsSupportedBrowserBypass() {
  const evidence = detectBrowserSocialUnmanagedBypass(
    bypassInput({
      bypassEvidenceId: 'social-bypass-supported-browser',
      processKind: 'supported-browser',
      processName: 'chrome-personal-profile',
      confidence: 'medium',
      reasons: ['supported-browser-outside-managed-session', 'managed-browser-required', 'exact-url-unavailable'],
      suspectedPlatformRef: null,
      browserBoundaryState: 'unmanaged-browser-process',
      unmanagedFallbackAction: 'parent-review',
    })
  );

  expect(evidence.processKind).toBe('supported-browser');
  expect(evidence.exactUrlClaimState).toBe('not-claimed');
  expect(evidence.childUiRenderedClaimed).toBe(false);
}

function rejectsOverclaims() {
  expect(() =>
    detectBrowserSocialUnmanagedBypass(
      bypassInput({
        executablePathRef: 'C:/Users/child/AppData/browser.exe',
      })
    )
  ).toThrow();
  expect(() => detectBrowserSocialUnmanagedBypass(bypassInput({ exactUrlClaimState: 'exact-url-proven' }))).toThrow();
  expect(() => detectBrowserSocialUnmanagedBypass(bypassInput({ browserBoundaryState: 'managed-session' }))).toThrow();
  expect(() =>
    detectBrowserSocialUnmanagedBypass(bypassInput({ unmanagedFallbackAction: 'terminate-process' }))
  ).toThrow();
}

function rejectsRuntimeClaims() {
  const valid = detectBrowserSocialUnmanagedBypass(bypassInput());
  const invalidRows = [
    { ...valid, exactUrlClaimed: true },
    { ...valid, routeEvidenceClaimed: true },
    { ...valid, socialAccountProofClaimed: true },
    { ...valid, feedVideoRouteClaimed: true },
    { ...valid, messageContentClaimed: true },
    { ...valid, accountIdentityClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
    { ...valid, childUiRenderedClaimed: true },
    { ...valid, parentUiNotifiedClaimed: true },
    { ...valid, processTerminatedClaimed: true },
    { ...valid, managedBrowserRelaunchedClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, exactUrlClaimState: 'exact-url-proven' },
    { ...valid, managedBrowserRequired: false },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialUnmanagedBypassEvidenceSchema.safeParse(invalid).success).toBe(false);
  }
}

function bypassInput(overrides = {}) {
  return {
    bypassEvidenceId: 'social-unmanaged-bypass-evidence',
    observedAt: '2026-06-03T07:22:00.000Z',
    sourceEvidenceIds: ['browser-evidence-social-unmanaged-bypass'],
    processKind: 'possible-social-bypass',
    processName: 'social-browser-like-process',
    executablePathRef: 'redacted-executable-ref-social-browser',
    processHashRef: 'redacted-process-hash-ref-social-browser',
    signatureRef: 'redacted-signature-ref-social-browser',
    confidence: 'high',
    reasons: ['possible-social-bypass-process', 'managed-browser-required', 'exact-url-unavailable'],
    suspectedPlatformRef: 'suspected-social-platform-ref',
    browserBoundaryState: 'browser-like-process',
    exactUrlClaimState: 'not-claimed',
    unmanagedDetectionState: 'detected',
    unmanagedFallbackAction: 'report-only',
    ...overrides,
  };
}
