import { describe, expect, it } from 'vitest';
import {
  type SocialPlatformConnectorAuthorizationBoundary,
  SocialPlatformConnectorAuthorizationBoundarySchema,
} from '../src/social-platform-connector-authorization';

describe('social platform connector authorization boundary contracts', () => {
  it('accepts an honest optional connector authorization boundary', acceptsHonestBoundary);
  it('rejects missing required connector provider rows', rejectsMissingProvider);
  it('rejects token, API, content, UI, policy, AI, native, and enforcement claims', rejectsRuntimeClaims);
  it('rejects unsupported connector authorization upgrades', rejectsAuthorizationUpgrades);
});

function acceptsHonestBoundary() {
  const parsed = SocialPlatformConnectorAuthorizationBoundarySchema.parse(validBoundary());

  expect(parsed.schemaVersion).toBe('social-platform-connector-authorization-boundary');
  expect(parsed.rows).toHaveLength(5);
  expect(rowState(parsed, 'google-youtube-supervision')).toEqual({
    authorizationState: 'not-implemented',
    proofState: 'provider-artifact-required',
    custodyState: 'parent-owned-token-required',
  });
  expect(rowState(parsed, 'parent-provided-account-ref')).toEqual({
    authorizationState: 'parent-authorized',
    proofState: 'parent-consent-record-only',
    custodyState: 'redacted-parent-input-only',
  });
}

function rejectsMissingProvider() {
  const boundary = validBoundary();

  expect(
    SocialPlatformConnectorAuthorizationBoundarySchema.safeParse({
      ...boundary,
      rows: boundary.rows.filter((row) => row.provider !== 'meta-family-center'),
    }).success
  ).toBe(false);
}

function rejectsRuntimeClaims() {
  const boundary = validBoundary();
  const invalidRows = [
    { rawTokenStoredClaimed: true },
    { oauthClientImplementedClaimed: true },
    { providerApiCallClaimed: true },
    { rawAccountDataCaptured: true },
    { messageContentCaptured: true },
    { feedContentCaptured: true },
    { accountIdentityVerifiedClaimed: true },
    { nativeAppControlClaimed: true },
    { policyDecisionClaimed: true },
    { aiRuntimeClaimed: true },
    { uiDeliveredClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(
      SocialPlatformConnectorAuthorizationBoundarySchema.safeParse({
        ...boundary,
        rows: boundary.rows.map((row) =>
          row.provider === 'google-youtube-supervision' ? { ...row, ...invalid } : row
        ),
      }).success
    ).toBe(false);
  }
}

function rejectsAuthorizationUpgrades() {
  const boundary = validBoundary();

  expect(
    SocialPlatformConnectorAuthorizationBoundarySchema.safeParse({
      ...boundary,
      rows: replaceRow(boundary, 'google-youtube-supervision', {
        authorizationState: 'parent-authorized',
        proofState: 'parent-consent-record-only',
        authorizedByActorId: 'parent-actor-social-connector',
        authorizedAt: '2026-06-03T08:00:00.000Z',
        visibleParentSettingRef: 'parent-visible-setting-social-connector',
      }),
    }).success
  ).toBe(false);

  expect(
    SocialPlatformConnectorAuthorizationBoundarySchema.safeParse({
      ...boundary,
      rows: replaceRow(boundary, 'parent-provided-account-ref', {
        visibleParentSettingRef: null,
      }),
    }).success
  ).toBe(false);
}

function validBoundary(): SocialPlatformConnectorAuthorizationBoundary {
  return {
    schemaVersion: 'social-platform-connector-authorization-boundary',
    authorizationBoundaryId: 'social-connector-boundary-family-1',
    familyId: 'family-social-connector',
    childProfileId: 'child-social-connector',
    generatedAt: '2026-06-03T08:00:00.000Z',
    rows: [...providerRows(), manualRows(), parentProvidedRow()],
    claimBoundaries: claimBoundaries(),
  };
}

function providerRows(): SocialPlatformConnectorAuthorizationBoundary['rows'] {
  return [
    providerRow('google-youtube-supervision', ['account-supervision-state', 'video-channel-metadata']),
    providerRow('meta-family-center', ['family-center-state']),
    providerRow('tiktok-family-pairing', ['family-pairing-state']),
  ];
}

function manualRows(): SocialPlatformConnectorAuthorizationBoundary['rows'][number] {
  return connectorRow('platform-export-import', {
    authorizationState: 'manual-required',
    proofState: 'manual-export-required',
    custodyState: 'manual-export-required',
    scopes: ['manual-export-file'],
    reasons: ['manual-export-required', 'core-gating-independent'],
  });
}

function providerRow(
  provider: SocialPlatformConnectorAuthorizationBoundary['rows'][number]['provider'],
  scopes: SocialPlatformConnectorAuthorizationBoundary['rows'][number]['scopes']
): SocialPlatformConnectorAuthorizationBoundary['rows'][number] {
  return connectorRow(provider, {
    authorizationState: 'not-implemented',
    proofState: 'provider-artifact-required',
    custodyState: 'parent-owned-token-required',
    scopes,
    reasons: [
      'optional-adjacent-source',
      'parent-authorization-required',
      'provider-api-not-implemented',
      'token-storage-not-implemented',
      'core-gating-independent',
      'message-content-unavailable',
      'feed-content-unavailable',
    ],
  });
}

function parentProvidedRow(): SocialPlatformConnectorAuthorizationBoundary['rows'][number] {
  return connectorRow('parent-provided-account-ref', {
    authorizationState: 'parent-authorized',
    proofState: 'parent-consent-record-only',
    custodyState: 'redacted-parent-input-only',
    scopes: ['parent-declared-account-ref'],
    reasons: [
      'optional-adjacent-source',
      'visible-setting-required',
      'redacted-input-required',
      'core-gating-independent',
      'message-content-unavailable',
      'feed-content-unavailable',
    ],
    authorizedByActorId: 'parent-actor-social-connector',
    authorizedAt: '2026-06-03T08:00:00.000Z',
    expiresAt: '2026-07-03T08:00:00.000Z',
    visibleParentSettingRef: 'parent-visible-setting-social-connector',
  });
}

function connectorRow(
  provider: SocialPlatformConnectorAuthorizationBoundary['rows'][number]['provider'],
  overrides: Partial<SocialPlatformConnectorAuthorizationBoundary['rows'][number]>
): SocialPlatformConnectorAuthorizationBoundary['rows'][number] {
  return {
    provider,
    authorizationState: 'manual-required',
    proofState: 'provider-artifact-required',
    custodyState: 'not-applicable',
    scopes: ['account-supervision-state'],
    reasons: ['optional-adjacent-source'],
    proofRefs: [`parent-proof-${provider}`],
    authorizedByActorId: null,
    authorizedAt: null,
    expiresAt: null,
    revokedAt: null,
    visibleParentSettingRef: null,
    coreGatingDependency: 'not-required',
    rawTokenStoredClaimed: false,
    oauthClientImplementedClaimed: false,
    providerApiCallClaimed: false,
    rawAccountDataCaptured: false,
    messageContentCaptured: false,
    feedContentCaptured: false,
    accountIdentityVerifiedClaimed: false,
    nativeAppControlClaimed: false,
    policyDecisionClaimed: false,
    aiRuntimeClaimed: false,
    uiDeliveredClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function claimBoundaries(): SocialPlatformConnectorAuthorizationBoundary['claimBoundaries'] {
  return {
    tokenStorage: 'not-claimed',
    oauthClient: 'not-claimed',
    providerApiCalls: 'not-claimed',
    rawAccountData: 'not-claimed',
    messageContent: 'not-claimed',
    feedContent: 'not-claimed',
    accountIdentityVerification: 'not-claimed',
    coreGatingDependency: 'not-claimed',
    policyDecision: 'not-claimed',
    aiRuntime: 'not-claimed',
    uiDelivery: 'not-claimed',
    nativeAppControl: 'not-claimed',
    enforcement: 'not-claimed',
    reviewerSummary: 'Social connectors are optional parent-authorized sources and not core gating dependencies.',
  };
}

function rowState(
  boundary: SocialPlatformConnectorAuthorizationBoundary,
  provider: SocialPlatformConnectorAuthorizationBoundary['rows'][number]['provider']
) {
  const row = boundary.rows.find((entry) => entry.provider === provider);
  return {
    authorizationState: row?.authorizationState,
    proofState: row?.proofState,
    custodyState: row?.custodyState,
  };
}

function replaceRow(
  boundary: SocialPlatformConnectorAuthorizationBoundary,
  provider: SocialPlatformConnectorAuthorizationBoundary['rows'][number]['provider'],
  overrides: Partial<SocialPlatformConnectorAuthorizationBoundary['rows'][number]>
) {
  return boundary.rows.map((row) => (row.provider === provider ? { ...row, ...overrides } : row));
}
