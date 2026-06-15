import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../../src/browser-url-intelligence';
import { buildBrowserSocialAccountFlowEvidenceFromRoute } from '../../src/browser-social-account-flow-schemas';
import {
  BrowserSocialAccountIdentityRegistryEntrySchema,
  BrowserSocialAccountIdentityRegistrySchemaVersion,
  buildUnverifiedSocialAccountIdentityContextFromFlow,
} from '../../src/browser-social-account-identity-registry';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '../../src/browser-social-url-patterns';

describe('browser social account identity registry contract', () => {
  it('builds unverified route-context identity entries from account-flow evidence', buildsRouteContextEntry);
  it('accepts parent-declared hashed identity refs without platform verification', acceptsParentDeclaredHashEntry);
  it('accepts manual-required identity state without account refs', acceptsManualRequiredEntry);
  it('rejects raw identity, credential, connector, AI, policy, native, and enforcement claims', rejectsClaims);
  it('rejects inconsistent route-context and parent-declared entries', rejectsInconsistentEntries);
});

function buildsRouteContextEntry() {
  const entry = buildUnverifiedSocialAccountIdentityContextFromFlow({
    registryEntryId: 'social-identity-entry-instagram-signup',
    accountIdentityRef: 'social-identity-ref-instagram-signup',
    observedAt: '2026-06-03T06:00:00.000Z',
    sourceEvidenceIds: ['browser-evidence-social-identity-route-context'],
    accountFlowEvidence: accountFlowEvidence('https://www.instagram.com/accounts/emailsignup/'),
  });

  expect(entry.schemaVersion).toBe(BrowserSocialAccountIdentityRegistrySchemaVersion);
  expect(entry.sourceKind).toBe('route-context-unverified');
  expect(entry.identityState).toBe('unverified-route-context');
  expect(entry.platform).toBe('instagram');
  expect(entry.identityVerifiedByPlatform).toBe(false);
  expect(entry.rawHandleCaptured).toBe(false);
}

function acceptsParentDeclaredHashEntry() {
  const parsed = BrowserSocialAccountIdentityRegistryEntrySchema.safeParse({
    ...parentDeclaredHashEntry(),
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.parentDeclaredIdentity).toBe(true);
    expect(parsed.data.identityVerifiedByPlatform).toBe(false);
    expect(parsed.data.handleHashRef).toBe('hash-ref-parent-declared-handle');
  }
}

function acceptsManualRequiredEntry() {
  const parsed = BrowserSocialAccountIdentityRegistryEntrySchema.safeParse({
    ...baseIdentityEntry(),
    registryEntryId: 'social-identity-entry-manual-required',
    accountIdentityRef: 'social-identity-ref-manual-required',
    sourceKind: 'manual-required',
    identityState: 'manual-required',
    platform: 'unknown-social',
  });

  expect(parsed.success).toBe(true);
}

function rejectsClaims() {
  const valid = parentDeclaredHashEntry();
  const invalidRows = [
    { ...valid, rawHandleCaptured: true },
    { ...valid, rawDisplayNameCaptured: true },
    { ...valid, rawPlatformAccountIdCaptured: true },
    { ...valid, credentialCaptured: true },
    { ...valid, identityVerifiedByPlatform: true },
    { ...valid, childDeclaredIdentity: true },
    { ...valid, accountCreationClaimed: true },
    { ...valid, loginSuccessClaimed: true },
    { ...valid, connectorAuthorizationClaimed: true },
    { ...valid, aiDecisionClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialAccountIdentityRegistryEntrySchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsInconsistentEntries() {
  expect(
    BrowserSocialAccountIdentityRegistryEntrySchema.safeParse({
      ...parentDeclaredHashEntry(),
      handleHashRef: null,
    }).success
  ).toBe(false);

  expect(
    BrowserSocialAccountIdentityRegistryEntrySchema.safeParse({
      ...baseIdentityEntry(),
      sourceKind: 'route-context-unverified',
      identityState: 'unverified-route-context',
      socialRouteEvidenceId: null,
      accountFlowEvidenceId: null,
    }).success
  ).toBe(false);
}

function parentDeclaredHashEntry() {
  return {
    ...baseIdentityEntry(),
    registryEntryId: 'social-identity-entry-parent-declared',
    accountIdentityRef: 'social-identity-ref-parent-declared',
    sourceEvidenceIds: ['parent-evidence-social-identity-hash'],
    sourceKind: 'parent-declared-hash',
    identityState: 'parent-declared',
    platform: 'instagram',
    parentAssertionRef: 'parent-assertion-ref-instagram',
    handleHashRef: 'hash-ref-parent-declared-handle',
    parentDeclaredIdentity: true,
  };
}

function baseIdentityEntry() {
  return {
    schemaVersion: BrowserSocialAccountIdentityRegistrySchemaVersion,
    registryEntryId: 'social-identity-entry',
    accountIdentityRef: 'social-identity-ref',
    observedAt: '2026-06-03T06:00:00.000Z',
    sourceEvidenceIds: ['browser-evidence-social-identity'],
    sourceKind: 'manual-required',
    identityState: 'manual-required',
    platform: 'unknown-social',
    socialRouteEvidenceId: null,
    accountFlowEvidenceId: null,
    parentAssertionRef: null,
    handleHashRef: null,
    displayNameHashRef: null,
    platformAccountIdHashRef: null,
    rawHandleCaptured: false,
    rawDisplayNameCaptured: false,
    rawPlatformAccountIdCaptured: false,
    credentialCaptured: false,
    identityVerifiedByPlatform: false,
    parentDeclaredIdentity: false,
    childDeclaredIdentity: false,
    accountCreationClaimed: false,
    loginSuccessClaimed: false,
    connectorAuthorizationClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
  };
}

function accountFlowEvidence(url: string) {
  return buildBrowserSocialAccountFlowEvidenceFromRoute({
    accountFlowEvidenceId: `social-account-flow-identity-${url.length}`,
    observedAt: '2026-06-03T06:00:00.000Z',
    sourceEvidenceIds: [`browser-evidence-identity-account-flow-${url.length}`],
    routeEvidence: buildBrowserSocialRouteEvidenceFromUrlPattern({
      socialRouteEvidenceId: `social-route-identity-${url.length}`,
      observedAt: '2026-06-03T06:00:00.000Z',
      sourceEvidenceIds: [`browser-evidence-identity-route-${url.length}`],
      classification: parseManagedUrl(url),
    }),
  });
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `social-identity-url-shape-${url.length}`,
    classifiedAt: '2026-06-03T06:00:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-identity-url-shape-${url.length}`],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Social account identity URL evidence',
  });
}
