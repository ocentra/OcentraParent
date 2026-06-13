import { describe, expect, it } from 'vitest';
import {
  type SocialAndroidNativeAppCapabilityMatrix,
  SocialAndroidNativeAppCapabilityMatrixSchema,
} from '../../src/social-android-native-app-capability-matrix';

describe('social Android native app capability matrix contracts', () => {
  it('accepts an honest Android social native-app capability matrix', acceptsHonestMatrix);
  it(
    'accepts package visibility as manual-required when no Android device proof exists',
    acceptsManualRequiredPackageVisibility
  );
  it('rejects missing required Android social surfaces', rejectsMissingSurface);
  it('rejects route, content, connector, UI, runtime adapter, and enforcement claims', rejectsRuntimeClaims);
  it('rejects unsupported Android capability upgrades', rejectsCapabilityUpgrades);
});

function acceptsHonestMatrix() {
  const parsed = SocialAndroidNativeAppCapabilityMatrixSchema.parse(validMatrix());

  expect(parsed.schemaVersion).toBe('social-android-native-app-capability-matrix');
  expect(parsed.rows).toHaveLength(6);
  expect(rowState(parsed, 'android-package-visibility')).toEqual({
    capabilityState: 'app-level-capable-with-proof',
    proofState: 'existing-parent-domain-proof-ref',
    policyScope: 'app-level-only',
  });
  expect(rowState(parsed, 'android-accessibility-route-hints')).toEqual({
    capabilityState: 'permission-required',
    proofState: 'permission-grant-required',
    policyScope: 'manual-review-only',
  });
}

function acceptsManualRequiredPackageVisibility() {
  const matrix = validMatrix();
  const parsed = SocialAndroidNativeAppCapabilityMatrixSchema.parse({
    ...matrix,
    rows: replaceRow(matrix, 'android-package-visibility', {
      capabilityState: 'manual-required',
      proofState: 'manual-device-proof-required',
      policyScope: 'manual-review-only',
    }),
  });

  expect(rowState(parsed, 'android-package-visibility')).toEqual({
    capabilityState: 'manual-required',
    proofState: 'manual-device-proof-required',
    policyScope: 'manual-review-only',
  });
}

function rejectsMissingSurface() {
  const matrix = validMatrix();

  expect(
    SocialAndroidNativeAppCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: matrix.rows.filter((row) => row.surface !== 'android-managed-profile-config'),
    }).success
  ).toBe(false);
}

function rejectsRuntimeClaims() {
  const matrix = validMatrix();
  const invalidRows = [
    { routeLevelProofClaimed: true },
    { perVideoOrReelBlockingClaimed: true },
    { messageContentClaimed: true },
    { accountIdentityClaimed: true },
    { accessibilityContentCaptureClaimed: true },
    { deviceOwnerEnrollmentClaimed: true },
    { vpnContentInspectionClaimed: true },
    { nativeRuntimeAdapterClaimed: true },
    { platformConnectorClaimed: true },
    { uiDeliveredClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(
      SocialAndroidNativeAppCapabilityMatrixSchema.safeParse({
        ...matrix,
        rows: matrix.rows.map((row) =>
          row.surface === 'android-accessibility-route-hints' ? { ...row, ...invalid } : row
        ),
      }).success
    ).toBe(false);
  }
}

function rejectsCapabilityUpgrades() {
  const matrix = validMatrix();

  expect(
    SocialAndroidNativeAppCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: replaceRow(matrix, 'android-accessibility-route-hints', {
        capabilityState: 'app-level-capable-with-proof',
        proofState: 'existing-parent-domain-proof-ref',
      }),
    }).success
  ).toBe(false);

  expect(
    SocialAndroidNativeAppCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: replaceRow(matrix, 'android-device-owner-app-control', {
        capabilityState: 'app-level-capable-with-proof',
        proofState: 'existing-parent-domain-proof-ref',
        policyScope: 'app-level-only',
      }),
    }).success
  ).toBe(false);
}

function rowState(matrix: SocialAndroidNativeAppCapabilityMatrix, surface: string) {
  const row = matrix.rows.find((entry) => entry.surface === surface);
  return {
    capabilityState: row?.capabilityState,
    proofState: row?.proofState,
    policyScope: row?.policyScope,
  };
}

function replaceRow(
  matrix: SocialAndroidNativeAppCapabilityMatrix,
  surface: string,
  overrides: Partial<SocialAndroidNativeAppCapabilityMatrix['rows'][number]>
) {
  return matrix.rows.map((row) => (row.surface === surface ? { ...row, ...overrides } : row));
}

function validMatrix(): SocialAndroidNativeAppCapabilityMatrix {
  return {
    schemaVersion: 'social-android-native-app-capability-matrix',
    generatedAt: '2026-06-03T07:34:00.000Z',
    proofRefs: ['parent-proof-social-android-native-matrix'],
    rows: [
      matrixRow('android-package-visibility', {
        targetKind: 'social-native-app-presence',
        parentCapability: 'package-lifecycle',
        parentCapabilityStatus: 'manual-required',
        capabilityState: 'app-level-capable-with-proof',
        proofState: 'existing-parent-domain-proof-ref',
        policyScope: 'app-level-only',
        reasons: ['package-visibility-limited', 'route-level-unavailable', 'content-proof-unavailable'],
      }),
      matrixRow('android-usage-stats-foreground', {
        targetKind: 'social-native-app-foreground',
        parentCapability: 'usage-stats',
        capabilityState: 'permission-required',
        proofState: 'permission-grant-required',
        policyScope: 'app-level-only',
        reasons: ['usage-access-required', 'route-level-unavailable', 'content-proof-unavailable'],
      }),
      matrixRow('android-accessibility-route-hints', {
        targetKind: 'social-native-app-route-hint',
        parentCapability: 'accessibility-service',
        capabilityState: 'permission-required',
        proofState: 'permission-grant-required',
        policyScope: 'manual-review-only',
        reasons: ['accessibility-explicit-approval-required', 'route-level-unavailable', 'content-proof-unavailable'],
      }),
      matrixRow('android-vpn-domain-hints', {
        targetKind: 'social-native-app-domain-hint',
        parentCapability: 'vpn-dns-filtering',
        capabilityState: 'manual-required',
        proofState: 'adapter-not-implemented',
        policyScope: 'domain-level-only',
        reasons: ['vpn-domain-only', 'route-level-unavailable', 'content-proof-unavailable'],
      }),
      matrixRow('android-device-owner-app-control', {
        targetKind: 'social-native-app-blocking',
        parentCapability: 'device-owner-policy',
        capabilityState: 'manual-required',
        proofState: 'manual-device-proof-required',
        policyScope: 'manual-review-only',
        reasons: ['device-owner-required', 'route-level-unavailable', 'content-proof-unavailable'],
      }),
      matrixRow('android-managed-profile-config', {
        targetKind: 'social-native-app-managed-config',
        parentCapability: 'managed-profile',
        capabilityState: 'manual-required',
        proofState: 'manual-device-proof-required',
        policyScope: 'manual-review-only',
        reasons: ['managed-profile-required', 'route-level-unavailable', 'content-proof-unavailable'],
      }),
    ],
    claimBoundaries: {
      nativeRouteProof: 'not-claimed',
      perVideoOrReelBlocking: 'not-claimed',
      messageContent: 'not-claimed',
      accountIdentity: 'not-claimed',
      accessibilityContentCapture: 'not-claimed',
      deviceOwnerEnrollment: 'not-claimed',
      runtimeAdapter: 'not-claimed',
      enforcement: 'not-claimed',
      reviewerSummary: 'Android native social app support remains app-level/manual-required until real device proof.',
    },
  };
}

function matrixRow(surface: SocialAndroidNativeAppCapabilityMatrix['rows'][number]['surface'], overrides: object) {
  return {
    surface,
    parentCapabilityStatus: 'manual-required',
    proofRefs: [`parent-proof-${surface}`],
    routeLevelProofClaimed: false,
    perVideoOrReelBlockingClaimed: false,
    messageContentClaimed: false,
    accountIdentityClaimed: false,
    accessibilityContentCaptureClaimed: false,
    deviceOwnerEnrollmentClaimed: false,
    vpnContentInspectionClaimed: false,
    nativeRuntimeAdapterClaimed: false,
    platformConnectorClaimed: false,
    uiDeliveredClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
