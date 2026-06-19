import { describe, expect, it } from 'vitest';
import { SocialParentApprovalDecisionSchema, SocialParentApprovalRequestSchema } from '../../src/social-parent-approval';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

describe('social parent approval request and decision contracts', () => {
  it('accepts pending social account signup requests with evidence refs', acceptsPendingSignupRequest);
  it('accepts recorded parent approval decisions without delivery or enforcement claims', acceptsRecordedDecision);
  it('accepts manual-required request and decision states', acceptsManualRequiredStates);
  it(
    'rejects request rows that claim raw data, notification delivery, UI, policy, connector, native, or enforcement',
    rejectsRequestClaims
  );
  it(
    'rejects decision rows that claim delivery, child notification, policy action, connector, native, or enforcement',
    rejectsDecisionClaims
  );
});

function acceptsPendingSignupRequest() {
  const parsed = SocialParentApprovalRequestSchema.safeParse(pendingSignupRequest());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.subjectKind).toBe('social-account-signup');
    expect(parsed.data.requestState).toBe('pending');
    expect(parsed.data.deliveryState).toBe('contract-only');
    expect(parsed.data.policyDecisionClaimed).toBe(false);
  }
}

function acceptsRecordedDecision() {
  const parsed = SocialParentApprovalDecisionSchema.safeParse(recordedDecision('allow-once'));

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.decisionKind).toBe('allow-once');
    expect(parsed.data.decisionState).toBe('recorded');
    expect(parsed.data.actionRef).toBeNull();
    expect(parsed.data.enforcementClaimed).toBe(false);
  }
}

function acceptsManualRequiredStates() {
  expect(SocialParentApprovalRequestSchema.safeParse(manualRequiredRequest()).success).toBe(true);
  expect(SocialParentApprovalDecisionSchema.safeParse(manualRequiredDecision()).success).toBe(true);
}

function rejectsRequestClaims() {
  const valid = pendingSignupRequest();
  const invalidRows = [
    { ...valid, rawMessageCaptured: true },
    { ...valid, rawAccountIdentityCaptured: true },
    { ...valid, credentialCaptured: true },
    { ...valid, notificationDeliveredClaimed: true },
    { ...valid, uiRenderedClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, connectorAuthorizationClaimed: true },
    { ...valid, accountFlowEvidenceRef: null },
  ];

  for (const invalid of invalidRows) {
    expect(SocialParentApprovalRequestSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsDecisionClaims() {
  const valid = recordedDecision('deny');
  const invalidRows = [
    { ...valid, notificationDeliveredClaimed: true },
    { ...valid, uiRenderedClaimed: true },
    { ...valid, childNotifiedClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, connectorAuthorizationClaimed: true },
    { ...valid, actionRef: 'action-ref-not-executed' },
    { ...valid, decidedByActorId: null },
  ];

  for (const invalid of invalidRows) {
    expect(SocialParentApprovalDecisionSchema.safeParse(invalid).success).toBe(false);
  }
}

function pendingSignupRequest() {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    approvalRequestId: 'social-approval-request-instagram-signup',
    familyId: 'family-social-approval',
    childProfileId: 'child-social-approval',
    requestedByDeviceId: 'parent-device-managed-browser',
    createdAt: '2026-06-03T06:06:00.000Z',
    expiresAt: '2026-06-03T06:16:00.000Z',
    subjectKind: 'social-account-signup',
    requestState: 'pending',
    sourceEvidenceRefs: ['parent-evidence-social-route', 'parent-evidence-account-flow'],
    socialRouteEvidenceRef: 'social-route-evidence-instagram-signup',
    accountFlowEvidenceRef: 'social-account-flow-instagram-signup',
    formShapeEvidenceRef: 'social-form-shape-instagram-signup',
    accountIdentityRef: 'social-identity-ref-instagram-signup',
    deliveryState: 'contract-only',
    rawMessageCaptured: false,
    rawAccountIdentityCaptured: false,
    credentialCaptured: false,
    notificationDeliveredClaimed: false,
    uiRenderedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    connectorAuthorizationClaimed: false,
  };
}

function manualRequiredRequest() {
  return {
    ...pendingSignupRequest(),
    approvalRequestId: 'social-approval-request-manual-required',
    subjectKind: 'social-route-manual-required',
    requestState: 'manual-required',
    socialRouteEvidenceRef: null,
    accountFlowEvidenceRef: null,
    formShapeEvidenceRef: null,
    accountIdentityRef: null,
  };
}

function recordedDecision(decisionKind: 'allow-once' | 'allow-account' | 'deny') {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    approvalDecisionId: `social-approval-decision-${decisionKind}`,
    approvalRequestId: 'social-approval-request-instagram-signup',
    familyId: 'family-social-approval',
    childProfileId: 'child-social-approval',
    decidedAt: '2026-06-03T06:07:00.000Z',
    decidedByActorId: 'parent-actor-social-approval',
    decisionKind,
    decisionState: 'recorded',
    sourceEvidenceRefs: ['parent-evidence-social-approval-request'],
    policyVersionRef: 'policy-version-social-approval',
    actionRef: null,
    deliveryState: 'contract-only',
    notificationDeliveredClaimed: false,
    uiRenderedClaimed: false,
    childNotifiedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    connectorAuthorizationClaimed: false,
  };
}

function manualRequiredDecision() {
  return {
    ...recordedDecision('deny'),
    approvalDecisionId: 'social-approval-decision-manual-required',
    decisionKind: 'manual-required',
    decisionState: 'manual-required',
    decidedByActorId: null,
    policyVersionRef: null,
  };
}
