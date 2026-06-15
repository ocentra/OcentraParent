import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../../src/browser-url-intelligence';
import { buildBrowserSocialAccountFlowEvidenceFromRoute } from '../../src/browser-social-account-flow-schemas';
import {
  buildBrowserSocialRouteEvidenceFromUrlPattern,
  matchBrowserSocialUrlPattern,
} from '../../src/browser-social-url-patterns';
import {
  BrowserSocialAccountFlowEvidenceSchema,
  BrowserSocialAccountFlowSchemaVersion,
} from '../../src/browser-social-account-flow-schemas';

describe('browser social account flow evidence contract', () => {
  it('builds signup route-only account flow evidence from managed social route evidence', buildsSignupEvidence);
  it('builds login and account-switch route-only evidence', buildsLoginAndSwitchEvidence);
  it('rejects non-account social route evidence', rejectsFeedRouteEvidence);
  it('accepts manual-required account flow state without exact route proof', acceptsManualRequiredState);
  it(
    'rejects identity, credential, form, approval, AI, policy, connector, native, and enforcement claims',
    rejectsClaims
  );
});

function buildsSignupEvidence() {
  const evidence = buildAccountFlowEvidence('https://www.instagram.com/accounts/emailsignup/');

  expect(evidence.schemaVersion).toBe(BrowserSocialAccountFlowSchemaVersion);
  expect(evidence.platform).toBe('instagram');
  expect(evidence.routeKind).toBe('account-signup');
  expect(evidence.accountFlowKind).toBe('signup-route');
  expect(evidence.evidenceState).toBe('route-only');
  expect(evidence.exactManagedBrowserRouteEvidence).toBe(true);
  expect(evidence.accountCreationCompletedClaimed).toBe(false);
}

function buildsLoginAndSwitchEvidence() {
  const login = buildAccountFlowEvidence('https://www.pinterest.com/login/');
  const accountSwitch = buildAccountFlowEvidence('https://www.instagram.com/accounts/switch/');

  expect(login.platform).toBe('pinterest');
  expect(login.accountFlowKind).toBe('login-route');
  expect(accountSwitch.platform).toBe('instagram');
  expect(accountSwitch.accountFlowKind).toBe('account-switch-route');
}

function rejectsFeedRouteEvidence() {
  expect(matchBrowserSocialUrlPattern(parseManagedUrl('https://x.com/home'))?.routeKind).toBe('feed');
  expect(() => buildAccountFlowEvidence('https://x.com/home')).toThrow();
}

function acceptsManualRequiredState() {
  const parsed = BrowserSocialAccountFlowEvidenceSchema.safeParse({
    ...baseAccountFlowEvidence(),
    accountFlowEvidenceId: 'social-account-flow-manual-required',
    sourceEvidenceIds: ['browser-evidence-unmanaged-social-account-flow'],
    routeKind: 'unknown-social-route',
    accountFlowKind: 'manual-required',
    evidenceState: 'manual-required',
    exactManagedBrowserRouteEvidence: false,
    manualRequired: true,
  });

  expect(parsed.success).toBe(true);
}

function rejectsClaims() {
  const valid = baseAccountFlowEvidence();
  const invalidRows = [
    { ...valid, accountIdentityRef: 'account-ref-not-proved' },
    { ...valid, parentApprovalRequestRef: 'parent-approval-not-created' },
    { ...valid, accountIdentityClaimed: true },
    { ...valid, credentialCaptured: true },
    { ...valid, formFieldValuesCaptured: true },
    { ...valid, formSubmissionClaimed: true },
    { ...valid, accountCreationCompletedClaimed: true },
    { ...valid, loginSuccessClaimed: true },
    { ...valid, accountSwitchCompletedClaimed: true },
    { ...valid, parentApprovalDecisionClaimed: true },
    { ...valid, aiDecisionClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialAccountFlowEvidenceSchema.safeParse(invalid).success).toBe(false);
  }
}

function buildAccountFlowEvidence(url: string) {
  return buildBrowserSocialAccountFlowEvidenceFromRoute({
    accountFlowEvidenceId: `social-account-flow-${url.length}`,
    observedAt: '2026-06-03T05:49:00.000Z',
    sourceEvidenceIds: [`browser-evidence-account-flow-${url.length}`],
    routeEvidence: buildBrowserSocialRouteEvidenceFromUrlPattern({
      socialRouteEvidenceId: `social-route-account-flow-${url.length}`,
      observedAt: '2026-06-03T05:49:00.000Z',
      sourceEvidenceIds: [`browser-evidence-social-route-${url.length}`],
      classification: parseManagedUrl(url),
    }),
  });
}

function baseAccountFlowEvidence() {
  return buildAccountFlowEvidence('https://www.instagram.com/accounts/emailsignup/');
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `social-account-url-shape-${url.length}`,
    classifiedAt: '2026-06-03T05:49:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-url-shape-${url.length}`],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Social account flow URL shape evidence',
  });
}
