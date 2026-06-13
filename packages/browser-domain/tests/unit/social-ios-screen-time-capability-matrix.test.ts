import { describe, expect, it } from 'vitest';
import {
  type SocialIosScreenTimeCapabilityMatrix,
  SocialIosScreenTimeCapabilityMatrixSchema,
} from '../../src/social-ios-screen-time-capability-matrix';

describe('social iOS Screen Time capability matrix contracts', () => {
  it('accepts an honest iOS Screen Time and ManagedSettings capability matrix', acceptsHonestMatrix);
  it('rejects missing required iOS token and shield surfaces', rejectsMissingSurface);
  it('rejects entitlement, identity, content, UI, connector, adapter, and enforcement claims', rejectsRuntimeClaims);
  it('rejects unsupported iOS capability upgrades', rejectsCapabilityUpgrades);
});

function acceptsHonestMatrix() {
  const parsed = SocialIosScreenTimeCapabilityMatrixSchema.parse(validMatrix());

  expect(parsed.schemaVersion).toBe('social-ios-screen-time-capability-matrix');
  expect(parsed.rows).toHaveLength(6);
  expect(rowState(parsed, 'ios-family-controls-authorization')).toEqual({
    capabilityState: 'entitlement-required',
    proofState: 'apple-entitlement-required',
    policyScope: 'manual-review-only',
  });
  expect(rowState(parsed, 'ios-application-token-selection')).toEqual({
    capabilityState: 'token-selection-required',
    proofState: 'family-authorization-required',
    policyScope: 'app-token-level',
  });
}

function rejectsMissingSurface() {
  const matrix = validMatrix();

  expect(
    SocialIosScreenTimeCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: matrix.rows.filter((row) => row.surface !== 'ios-managed-settings-web-domain-shield'),
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
    { rawApplicationIdentityClaimed: true },
    { screenContentCaptureClaimed: true },
    { deviceActivityRuntimeClaimed: true },
    { managedSettingsRuntimeClaimed: true },
    { entitlementApprovalClaimed: true },
    { platformConnectorClaimed: true },
    { uiDeliveredClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(
      SocialIosScreenTimeCapabilityMatrixSchema.safeParse({
        ...matrix,
        rows: matrix.rows.map((row) =>
          row.surface === 'ios-managed-settings-application-shield' ? { ...row, ...invalid } : row
        ),
      }).success
    ).toBe(false);
  }
}

function rejectsCapabilityUpgrades() {
  const matrix = validMatrix();

  expect(
    SocialIosScreenTimeCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: replaceRow(matrix, 'ios-family-controls-authorization', {
        capabilityState: 'manual-device-proof-required',
        proofState: 'existing-ios-entitlement-proof-ref',
      }),
    }).success
  ).toBe(false);

  expect(
    SocialIosScreenTimeCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: replaceRow(matrix, 'ios-application-token-selection', {
        reasons: ['opaque-token-required', 'route-level-unavailable', 'content-proof-unavailable'],
      }),
    }).success
  ).toBe(false);

  expect(
    SocialIosScreenTimeCapabilityMatrixSchema.safeParse({
      ...matrix,
      rows: replaceRow(matrix, 'ios-managed-settings-application-shield', {
        proofState: 'existing-ios-entitlement-proof-ref',
      }),
    }).success
  ).toBe(false);
}

function rowState(
  matrix: SocialIosScreenTimeCapabilityMatrix,
  surface: SocialIosScreenTimeCapabilityMatrix['rows'][number]['surface']
) {
  const row = matrix.rows.find((entry) => entry.surface === surface);
  return {
    capabilityState: row?.capabilityState,
    proofState: row?.proofState,
    policyScope: row?.policyScope,
  };
}

function replaceRow(
  matrix: SocialIosScreenTimeCapabilityMatrix,
  surface: SocialIosScreenTimeCapabilityMatrix['rows'][number]['surface'],
  overrides: Partial<SocialIosScreenTimeCapabilityMatrix['rows'][number]>
) {
  return matrix.rows.map((row) => (row.surface === surface ? { ...row, ...overrides } : row));
}

function validMatrix(): SocialIosScreenTimeCapabilityMatrix {
  return {
    schemaVersion: 'social-ios-screen-time-capability-matrix',
    generatedAt: '2026-06-03T07:47:00.000Z',
    proofRefs: ['parent-proof-social-ios-screentime-matrix'],
    rows: socialIosScreenTimeRows(),
    claimBoundaries: claimBoundaries(),
  };
}

function socialIosScreenTimeRows(): SocialIosScreenTimeCapabilityMatrix['rows'] {
  return [...familyControlsRows(), ...managedSettingsRows()];
}

function familyControlsRows(): SocialIosScreenTimeCapabilityMatrix['rows'] {
  return [
    matrixRow('ios-family-controls-authorization', {
      targetKind: 'social-ios-family-authorization',
      parentCapability: 'family-controls-entitlement',
      capabilityState: 'entitlement-required',
      proofState: 'apple-entitlement-required',
      policyScope: 'manual-review-only',
      reasons: ['family-controls-entitlement-required', 'family-authorization-required'],
    }),
    matrixRow('ios-application-token-selection', {
      targetKind: 'social-ios-app-token',
      parentCapability: 'family-controls-entitlement',
      capabilityState: 'token-selection-required',
      proofState: 'family-authorization-required',
      policyScope: 'app-token-level',
      reasons: [
        'family-authorization-required',
        'opaque-token-required',
        'raw-app-identity-unavailable',
        'route-level-unavailable',
        'content-proof-unavailable',
      ],
    }),
    matrixRow('ios-web-domain-token-selection', {
      targetKind: 'social-ios-web-domain-token',
      parentCapability: 'family-controls-entitlement',
      capabilityState: 'token-selection-required',
      proofState: 'family-authorization-required',
      policyScope: 'web-domain-token-level',
      reasons: [
        'family-authorization-required',
        'opaque-token-required',
        'web-domain-token-limited',
        'route-level-unavailable',
        'content-proof-unavailable',
      ],
    }),
  ];
}

function managedSettingsRows(): SocialIosScreenTimeCapabilityMatrix['rows'] {
  return [
    matrixRow('ios-device-activity-monitor', {
      targetKind: 'social-ios-device-activity',
      parentCapability: 'device-activity',
      capabilityState: 'manual-device-proof-required',
      proofState: 'apple-entitlement-required',
      policyScope: 'category-token-level',
      reasons: ['device-activity-entitlement-required', 'route-level-unavailable', 'content-proof-unavailable'],
    }),
    matrixRow('ios-managed-settings-application-shield', {
      targetKind: 'social-ios-application-shield',
      parentCapability: 'screen-time-api',
      capabilityState: 'manual-device-proof-required',
      proofState: 'apple-entitlement-required',
      policyScope: 'app-token-level',
      reasons: [
        'managed-settings-entitlement-required',
        'shield-state-device-proof-required',
        'route-level-unavailable',
        'content-proof-unavailable',
      ],
    }),
    matrixRow('ios-managed-settings-web-domain-shield', {
      targetKind: 'social-ios-web-domain-shield',
      parentCapability: 'screen-time-api',
      capabilityState: 'manual-device-proof-required',
      proofState: 'apple-entitlement-required',
      policyScope: 'web-domain-token-level',
      reasons: [
        'managed-settings-entitlement-required',
        'shield-state-device-proof-required',
        'web-domain-token-limited',
        'route-level-unavailable',
        'content-proof-unavailable',
      ],
    }),
  ];
}

function claimBoundaries(): SocialIosScreenTimeCapabilityMatrix['claimBoundaries'] {
  return {
    familyControlsAuthorization: 'not-claimed',
    rawApplicationIdentity: 'not-claimed',
    nativeRouteProof: 'not-claimed',
    perVideoOrReelBlocking: 'not-claimed',
    messageContent: 'not-claimed',
    accountIdentity: 'not-claimed',
    screenContentCapture: 'not-claimed',
    runtimeAdapter: 'not-claimed',
    connectorAuthorization: 'not-claimed',
    uiDelivery: 'not-claimed',
    enforcement: 'not-claimed',
    reviewerSummary:
      'iOS social support remains token and shield capability mapping only until Apple entitlement and device proof.',
  };
}

function matrixRow(surface: SocialIosScreenTimeCapabilityMatrix['rows'][number]['surface'], overrides: object) {
  return {
    surface,
    parentCapabilityStatus: 'manual-required',
    proofRefs: [`parent-proof-${surface}`],
    routeLevelProofClaimed: false,
    perVideoOrReelBlockingClaimed: false,
    messageContentClaimed: false,
    accountIdentityClaimed: false,
    rawApplicationIdentityClaimed: false,
    screenContentCaptureClaimed: false,
    deviceActivityRuntimeClaimed: false,
    managedSettingsRuntimeClaimed: false,
    entitlementApprovalClaimed: false,
    platformConnectorClaimed: false,
    uiDeliveredClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
