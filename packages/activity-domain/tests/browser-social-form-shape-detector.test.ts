import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../src/browser-url-intelligence';
import { buildBrowserSocialAccountFlowEvidenceFromRoute } from '../src/browser-social-account-flow-schemas';
import {
  type BrowserSocialFormControlKind,
  BrowserSocialFormShapeEvidenceSchema,
  detectBrowserSocialFormShape,
} from '../src/browser-social-form-shape-detector';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '../src/browser-social-url-patterns';

describe('browser social form-shape detector contract', () => {
  it('detects sanitized signup form shape from managed account-flow evidence', detectsSignupForm);
  it('detects sanitized login and account-switch form shapes', detectsLoginAndAccountSwitchForms);
  it('rejects captured field values or insufficient controls', rejectsCapturedValuesAndWeakForms);
  it(
    'rejects raw DOM, credential, submission, identity, AI, policy, connector, native, and enforcement claims',
    rejectsClaims
  );
});

function detectsSignupForm() {
  const evidence = detectFormShape('https://www.instagram.com/accounts/emailsignup/', [
    'email-input',
    'password-input',
    'display-name-input',
    'submit-button',
  ]);

  expect(evidence.platform).toBe('instagram');
  expect(evidence.accountFlowKind).toBe('signup-route');
  expect(evidence.formShapeKind).toBe('signup-form');
  expect(evidence.detectionState).toBe('detected');
  expect(evidence.missingControlKinds).toEqual([]);
  expect(evidence.fieldValuesCaptured).toBe(false);
}

function detectsLoginAndAccountSwitchForms() {
  const login = detectFormShape('https://www.pinterest.com/login/', ['email-input', 'password-input', 'submit-button']);
  const accountSwitch = detectFormShape('https://www.instagram.com/accounts/switch/', ['account-switch-link']);

  expect(login.formShapeKind).toBe('login-form');
  expect(accountSwitch.formShapeKind).toBe('account-switch-form');
  expect(accountSwitch.matchedControlKinds).toEqual(['account-switch-link']);
}

function rejectsCapturedValuesAndWeakForms() {
  expect(() =>
    detectBrowserSocialFormShape({
      ...detectorInput('https://www.instagram.com/accounts/emailsignup/', ['email-input', 'password-input']),
    })
  ).toThrow();

  expect(() =>
    detectBrowserSocialFormShape({
      ...detectorInput('https://www.instagram.com/accounts/emailsignup/', [
        'email-input',
        'password-input',
        'submit-button',
      ]),
      controls: [
        { controlKind: 'email-input', valueCaptured: true },
        { controlKind: 'password-input', valueCaptured: false },
        { controlKind: 'submit-button', valueCaptured: false },
      ],
    })
  ).toThrow();
}

function rejectsClaims() {
  const valid = detectFormShape('https://www.instagram.com/accounts/emailsignup/', [
    'email-input',
    'password-input',
    'submit-button',
  ]);
  const invalidRows = [
    { ...valid, rawDomCaptured: true },
    { ...valid, fieldValuesCaptured: true },
    { ...valid, credentialCaptured: true },
    { ...valid, formSubmissionClaimed: true },
    { ...valid, accountIdentityClaimed: true },
    { ...valid, parentApprovalDecisionClaimed: true },
    { ...valid, aiDecisionClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialFormShapeEvidenceSchema.safeParse(invalid).success).toBe(false);
  }
}

function detectFormShape(url: string, controls: readonly BrowserSocialFormControlKind[]) {
  return detectBrowserSocialFormShape(detectorInput(url, controls));
}

function detectorInput(url: string, controls: readonly BrowserSocialFormControlKind[]) {
  return {
    formShapeEvidenceId: `social-form-shape-${url.length}`,
    observedAt: '2026-06-03T05:55:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-form-${url.length}`],
    accountFlowEvidence: buildBrowserSocialAccountFlowEvidenceFromRoute({
      accountFlowEvidenceId: `social-account-flow-form-${url.length}`,
      observedAt: '2026-06-03T05:55:00.000Z',
      sourceEvidenceIds: [`browser-evidence-account-flow-form-${url.length}`],
      routeEvidence: buildBrowserSocialRouteEvidenceFromUrlPattern({
        socialRouteEvidenceId: `social-route-form-${url.length}`,
        observedAt: '2026-06-03T05:55:00.000Z',
        sourceEvidenceIds: [`browser-evidence-social-route-form-${url.length}`],
        classification: parseManagedUrl(url),
      }),
    }),
    controls: controls.map((controlKind) => ({ controlKind, valueCaptured: false })),
  };
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `social-form-url-shape-${url.length}`,
    classifiedAt: '2026-06-03T05:55:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-form-url-shape-${url.length}`],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Social form shape URL evidence',
  });
}
