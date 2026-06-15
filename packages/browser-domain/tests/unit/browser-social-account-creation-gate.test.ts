import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../../src/browser-url-intelligence';
import { buildBrowserSocialAccountFlowEvidenceFromRoute } from '../../src/browser-social-account-flow-schemas';
import {
  type BrowserSocialFormControlKind,
  detectBrowserSocialFormShape,
} from '../../src/browser-social-form-shape-detector';
import {
  BrowserSocialAccountCreationGatePlanSchema,
  planBrowserSocialAccountCreationGate,
} from '../../src/browser-social-account-creation-gate';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '../../src/browser-social-url-patterns';

describe('browser social account creation gate contract', () => {
  it('plans a parent-approval hold from matching signup flow and form evidence', plansParentApprovalHold);
  it('plans block and allow candidates without executing browser actions', plansBlockAndAllowCandidates);
  it('rejects mismatched evidence and missing action refs', rejectsMismatchedEvidenceAndRefs);
  it('rejects runtime, UI, policy finality, native, connector, and account-created claims', rejectsRuntimeClaims);
});

function plansParentApprovalHold() {
  const plan = planBrowserSocialAccountCreationGate(
    gateInput('https://www.instagram.com/accounts/emailsignup/', ['email-input', 'password-input', 'submit-button'], {
      gateAction: 'hold-for-parent-approval',
      parentApprovalRequired: true,
      parentApprovalRequestRef: 'social-parent-approval-request-signup',
      policyDecisionCandidateRef: null,
      reasons: ['signup-flow', 'form-shape-detected', 'parent-policy-requires-approval'],
    })
  );

  expect(plan.platform).toBe('instagram');
  expect(plan.accountFlowKind).toBe('signup-route');
  expect(plan.gateState).toBe('planned');
  expect(plan.navigationPausedClaimed).toBe(false);
  expect(plan.formSubmitBlockedClaimed).toBe(false);
  expect(plan.enforcementClaimed).toBe(false);
}

function plansBlockAndAllowCandidates() {
  const block = planBrowserSocialAccountCreationGate(
    gateInput('https://www.instagram.com/accounts/emailsignup/', ['email-input', 'password-input', 'submit-button'], {
      gateAction: 'block-submit-candidate',
      reasons: ['signup-flow', 'form-shape-detected', 'policy-block-candidate'],
    })
  );
  const allow = planBrowserSocialAccountCreationGate(
    gateInput('https://www.pinterest.com/login/', ['email-input', 'password-input', 'submit-button'], {
      gateAction: 'allow-navigation-candidate',
      reasons: ['login-flow', 'form-shape-detected'],
    })
  );

  expect(block.gateAction).toBe('block-submit-candidate');
  expect(block.policyDecisionCandidateRef).toBe('social-policy-decision-candidate-account');
  expect(allow.gateAction).toBe('allow-navigation-candidate');
  expect(allow.accountFlowKind).toBe('login-route');
}

function rejectsMismatchedEvidenceAndRefs() {
  const input = gateInput('https://www.instagram.com/accounts/emailsignup/', [
    'email-input',
    'password-input',
    'submit-button',
  ]);
  const mismatched = {
    ...input,
    formShapeEvidence: formShape('https://www.pinterest.com/login/', [
      'email-input',
      'password-input',
      'submit-button',
    ]),
  };
  const missingApprovalRef = gateInput(
    'https://www.instagram.com/accounts/emailsignup/',
    ['email-input', 'password-input', 'submit-button'],
    { gateAction: 'hold-for-parent-approval', parentApprovalRequired: true, parentApprovalRequestRef: null }
  );
  const missingPolicyRef = gateInput(
    'https://www.instagram.com/accounts/emailsignup/',
    ['email-input', 'password-input', 'submit-button'],
    { gateAction: 'block-submit-candidate', policyDecisionCandidateRef: null }
  );

  expect(() => planBrowserSocialAccountCreationGate(mismatched)).toThrow();
  expect(() => planBrowserSocialAccountCreationGate(missingApprovalRef)).toThrow();
  expect(() => planBrowserSocialAccountCreationGate(missingPolicyRef)).toThrow();
}

function rejectsRuntimeClaims() {
  const valid = planBrowserSocialAccountCreationGate(
    gateInput('https://www.instagram.com/accounts/emailsignup/', ['email-input', 'password-input', 'submit-button'])
  );
  const invalidRows = [
    { ...valid, navigationPausedClaimed: true },
    { ...valid, formSubmitBlockedClaimed: true },
    { ...valid, childUiRenderedClaimed: true },
    { ...valid, parentUiNotifiedClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
    { ...valid, credentialCaptured: true },
    { ...valid, formSubmittedClaimed: true },
    { ...valid, accountCreatedClaimed: true },
    { ...valid, gateAction: 'manual-review-required', gateState: 'planned' },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialAccountCreationGatePlanSchema.safeParse(invalid).success).toBe(false);
  }
}

function gateInput(url: string, controls: readonly BrowserSocialFormControlKind[], overrides = {}) {
  const shape = formShape(url, controls);
  return {
    gatePlanId: `social-account-gate-${url.length}`,
    plannedAt: '2026-06-03T06:58:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-account-gate-${url.length}`],
    accountFlowEvidence: accountFlow(url),
    formShapeEvidence: shape,
    policyDecisionCandidateRef: 'social-policy-decision-candidate-account',
    parentApprovalRequestRef: null,
    gateAction: 'allow-navigation-candidate',
    parentApprovalRequired: false,
    reasons: ['form-shape-detected'],
    ...overrides,
  };
}

function formShape(url: string, controls: readonly BrowserSocialFormControlKind[]) {
  return detectBrowserSocialFormShape({
    formShapeEvidenceId: `social-form-shape-gate-${url.length}`,
    observedAt: '2026-06-03T06:57:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-form-gate-${url.length}`],
    accountFlowEvidence: accountFlow(url),
    controls: controls.map((controlKind) => ({ controlKind, valueCaptured: false })),
  });
}

function accountFlow(url: string) {
  return buildBrowserSocialAccountFlowEvidenceFromRoute({
    accountFlowEvidenceId: `social-account-flow-gate-${url.length}`,
    observedAt: '2026-06-03T06:57:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-account-flow-gate-${url.length}`],
    routeEvidence: buildBrowserSocialRouteEvidenceFromUrlPattern({
      socialRouteEvidenceId: `social-route-gate-${url.length}`,
      observedAt: '2026-06-03T06:57:00.000Z',
      sourceEvidenceIds: [`browser-evidence-social-route-gate-${url.length}`],
      classification: parseManagedUrl(url),
    }),
  });
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `social-account-gate-url-shape-${url.length}`,
    classifiedAt: '2026-06-03T06:57:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-account-gate-url-${url.length}`],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Social account gate URL evidence',
  });
}
