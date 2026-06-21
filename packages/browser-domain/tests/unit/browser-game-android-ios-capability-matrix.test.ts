import { describe, expect, it } from 'vitest';
import {
  type BrowserGameMobileCapabilityMatrix,
  BrowserGameMobileCapabilityMatrixSchema,
} from '@ocentra-parent/schema-domain/browser-game-android-ios-capability-matrix';

describe('browser game Android/iOS capability matrix contracts', () => {
  it('accepts an honest mobile browser-game capability matrix', acceptsHonestMatrix);
  it('rejects missing or duplicate mobile surfaces', rejectsMissingOrDuplicateSurface);
  it('rejects content, runtime, native-game, UI, and enforcement claims', rejectsRuntimeClaims);
  it('rejects unsupported mobile capability upgrades', rejectsUnsupportedCapabilityUpgrades);
});

function acceptsHonestMatrix() {
  const parsed = BrowserGameMobileCapabilityMatrixSchema.parse(validMatrix());

  expect(parsed.schemaVersion).toBe('browser-game-android-ios-capability-matrix');
  expect(parsed.rows).toHaveLength(12);
  expect(rowState(parsed, 'android-owned-browser-shell')).toEqual({
    capabilityState: 'manual-device-proof-required',
    proofState: 'manual-device-proof-required',
    policyScope: 'owned-browser-shell-only',
  });
  expect(rowState(parsed, 'ios-safari-web-domain-token')).toEqual({
    capabilityState: 'domain-token-limited',
    proofState: 'platform-entitlement-required',
    policyScope: 'web-domain-token-level',
  });
}

function rejectsMissingOrDuplicateSurface() {
  const matrix = validMatrix();

  expect(
    BrowserGameMobileCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: matrix.rows.filter((row) => row.surface !== 'ios-webclip-pwa'),
    }).success
  ).toBe(false);

  expect(
    BrowserGameMobileCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: matrix.rows.map((row) =>
        row.surface === 'ios-webclip-pwa' ? { ...row, surface: 'ios-cloud-gaming-web-session' } : row
      ),
    }).success
  ).toBe(false);
}

function rejectsRuntimeClaims() {
  const matrix = validMatrix();
  const invalidRows = [
    { exactGameContentClaimed: true },
    { cloudStreamFrameAnalysisClaimed: true },
    { nativeGameControlClaimed: true },
    { nativeLauncherControlClaimed: true },
    { gameChatContentClaimed: true },
    { perGameCloudTitleClaimed: true },
    { runtimeSignalClaimed: true },
    { appStoreOrPurchaseControlClaimed: true },
    { uiDeliveredClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(
      BrowserGameMobileCapabilityMatrixSchema.safeParse({
        ...matrix,
        rows: matrix.rows.map((row) =>
          row.surface === 'android-cloud-gaming-browser-session' ? { ...row, ...invalid } : row
        ),
      }).success
    ).toBe(false);
  }
}

function rejectsUnsupportedCapabilityUpgrades() {
  const matrix = validMatrix();

  expect(
    BrowserGameMobileCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: replaceRow(matrix, 'android-owned-browser-shell', {
        capabilityState: 'owned-browser-shell-capable-with-proof',
        proofState: 'existing-parent-domain-proof-ref',
      }),
    }).success
  ).toBe(false);

  expect(
    BrowserGameMobileCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: replaceRow(matrix, 'ios-safari-web-domain-token', {
        policyScope: 'owned-browser-shell-only',
      }),
    }).success
  ).toBe(false);

  expect(
    BrowserGameMobileCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: replaceRow(matrix, 'ios-managed-browser-shell', {
        platform: 'android',
      }),
    }).success
  ).toBe(false);
}

function rowState(matrix: BrowserGameMobileCapabilityMatrix, surface: string) {
  const row = matrix.rows.find((entry) => entry.surface === surface);
  return {
    capabilityState: row?.capabilityState,
    proofState: row?.proofState,
    policyScope: row?.policyScope,
  };
}

function replaceRow(
  matrix: BrowserGameMobileCapabilityMatrix,
  surface: string,
  overrides: Partial<BrowserGameMobileCapabilityMatrix['rows'][number]>
) {
  return matrix.rows.map((row) => (row.surface === surface ? { ...row, ...overrides } : row));
}

function validMatrix(): BrowserGameMobileCapabilityMatrix {
  return {
    schemaVersion: 'browser-game-android-ios-capability-matrix',
    generatedAt: '2026-06-03T09:02:00.000Z',
    proofRefs: ['parent-proof-browser-game-mobile-matrix'],
    rows: [...androidRows(), ...iosRows()],
    claimBoundaries: claimBoundaries(),
  };
}

function androidRows(): BrowserGameMobileCapabilityMatrix['rows'] {
  return [
    matrixRow('android', 'android-owned-browser-shell', androidOwnedBrowserShell()),
    matrixRow('android', 'android-managed-webview-shell', androidOwnedBrowserShell()),
    matrixRow('android', 'android-custom-tabs', {
      targetKind: 'browser-game-web-surface',
      parentCapability: 'managed-browser-control',
      capabilityState: 'manual-required',
      proofState: 'adapter-not-implemented',
      policyScope: 'manual-review-only',
      reasons: ['android-custom-tabs-not-owned', 'unmanaged-exact-url-unavailable'],
    }),
    matrixRow('android', 'android-installed-browser-app', {
      targetKind: 'installed-browser-app',
      parentCapability: 'package-lifecycle',
      capabilityState: 'manual-required',
      proofState: 'manual-device-proof-required',
      policyScope: 'app-level-only',
      reasons: ['unmanaged-exact-url-unavailable', 'manual-device-proof-required'],
    }),
    matrixRow('android', 'android-cloud-gaming-browser-session', cloudGamingBrowserSession('managed-browser-control')),
    matrixRow('android', 'android-device-owner-browser-policy', {
      targetKind: 'device-owner-browser-policy',
      parentCapability: 'device-owner-policy',
      capabilityState: 'manual-required',
      proofState: 'manual-device-proof-required',
      policyScope: 'manual-review-only',
      reasons: ['device-owner-required', 'app-store-control-unavailable'],
    }),
  ];
}

function iosRows(): BrowserGameMobileCapabilityMatrix['rows'] {
  return [
    matrixRow('ios', 'ios-family-controls-authorization', {
      targetKind: 'application-token',
      parentCapability: 'family-controls-entitlement',
      capabilityState: 'entitlement-required',
      proofState: 'platform-entitlement-required',
      policyScope: 'manual-review-only',
      reasons: ['family-controls-entitlement-required', 'native-game-boundary'],
    }),
    matrixRow('ios', 'ios-safari-web-domain-token', {
      targetKind: 'web-domain-token',
      parentCapability: 'screen-time-api',
      capabilityState: 'domain-token-limited',
      proofState: 'platform-entitlement-required',
      policyScope: 'web-domain-token-level',
      reasons: ['web-domain-token-limited', 'cloud-title-unavailable'],
    }),
    matrixRow('ios', 'ios-application-token', iosApplicationToken()),
    matrixRow('ios', 'ios-managed-browser-shell', androidOwnedBrowserShell()),
    matrixRow('ios', 'ios-cloud-gaming-web-session', cloudGamingBrowserSession('screen-time-api')),
    matrixRow('ios', 'ios-webclip-pwa', {
      ...iosApplicationToken(),
      targetKind: 'webclip-or-pwa',
      reasons: ['opaque-application-token-required', 'native-game-boundary', 'cloud-title-unavailable'],
    }),
  ];
}

function androidOwnedBrowserShell() {
  return {
    targetKind: 'owned-browser-shell',
    parentCapability: 'managed-browser-control',
    capabilityState: 'manual-device-proof-required',
    proofState: 'manual-device-proof-required',
    policyScope: 'owned-browser-shell-only',
    reasons: ['owned-shell-required', 'managed-browser-required', 'manual-device-proof-required'],
  };
}

function cloudGamingBrowserSession(parentCapability: 'managed-browser-control' | 'screen-time-api') {
  return {
    targetKind: 'cloud-gaming-web-session',
    parentCapability,
    capabilityState: 'manual-required',
    proofState:
      parentCapability === 'screen-time-api' ? 'platform-entitlement-required' : 'manual-device-proof-required',
    policyScope: 'manual-review-only',
    reasons: ['cloud-title-unavailable', 'content-frame-analysis-unavailable', 'runtime-signal-unavailable'],
  };
}

function iosApplicationToken() {
  return {
    targetKind: 'application-token',
    parentCapability: 'family-controls-entitlement',
    capabilityState: 'app-token-limited',
    proofState: 'platform-entitlement-required',
    policyScope: 'app-token-level',
    reasons: ['opaque-application-token-required', 'native-game-boundary'],
  };
}

function claimBoundaries(): BrowserGameMobileCapabilityMatrix['claimBoundaries'] {
  return {
    exactGameContent: 'not-claimed',
    cloudStreamFrameAnalysis: 'not-claimed',
    nativeGameControl: 'not-claimed',
    nativeLauncherControl: 'not-claimed',
    gameChatContent: 'not-claimed',
    perGameCloudTitle: 'not-claimed',
    runtimeSignals: 'not-claimed',
    appStoreOrPurchaseControl: 'not-claimed',
    uiDelivery: 'not-claimed',
    enforcement: 'not-claimed',
    reviewerSummary:
      'Mobile browser-game support remains manual-required or token-limited until real Android/iOS browser, entitlement, and device proof exists.',
  };
}

function matrixRow(
  platform: BrowserGameMobileCapabilityMatrix['rows'][number]['platform'],
  surface: BrowserGameMobileCapabilityMatrix['rows'][number]['surface'],
  overrides: object
) {
  return {
    platform,
    surface,
    parentCapabilityStatus: 'manual-required',
    proofRefs: [`parent-proof-${surface}`],
    exactGameContentClaimed: false,
    cloudStreamFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    nativeLauncherControlClaimed: false,
    gameChatContentClaimed: false,
    perGameCloudTitleClaimed: false,
    runtimeSignalClaimed: false,
    appStoreOrPurchaseControlClaimed: false,
    uiDeliveredClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
