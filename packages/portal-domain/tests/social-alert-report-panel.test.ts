import { describe, expect, it } from 'vitest';
import { createSocialAlertReportPanelIntent } from '../src/social-alert-report-panel';

describe('social alert/report panel intent', () => {
  it('renders empty unavailable intent for missing service snapshot', () => {
    const intent = createSocialAlertReportPanelIntent(null);

    expect(intent.rows).toEqual([]);
    expect(intent.summary).toBe('0 social alert/report rows');
    expect(intent.productClaim).toContain('provider delivery');
  });

  it('renders ref-only rows from a parsed service snapshot', () => {
    const intent = createSocialAlertReportPanelIntent(serviceSnapshot());

    expect(intent.rows).toHaveLength(1);
    expect(intent.rows[0]?.title).toBe('Manual alert/report proof required');
    expect(intent.rows[0]?.details.some((detail) => detail.value === 'manual-required')).toBe(true);
  });
});

function serviceSnapshot(): unknown {
  return {
    schemaVersion: 'social-alert-report-read-model',
    familyId: 'family-social-alert-report-service',
    childProfileId: 'child-social-alert-report-service',
    generatedAt: '2026-06-07T01:39:00Z',
    intents: [manualRequiredIntent()],
    claimBoundaries: {
      providerDelivery: 'not-claimed',
      reportDelivery: 'not-claimed',
      parentNotificationUi: 'not-claimed',
      finalPolicyDecision: 'not-claimed',
      enforcement: 'not-claimed',
    },
  };
}

function manualRequiredIntent(): unknown {
  return {
    schemaVersion: 'v0.6',
    alertReportIntentId: 'social-alert-report-manual-required',
    intentKind: 'manual-required',
    intentStatus: 'manual-required',
    priority: 'attention',
    severity: 'warning',
    device: {
      deviceId: 'device-social-alert-report',
      childProfileId: 'child-social-alert-report-service',
      label: 'Study Phone',
      platform: 'android',
    },
    notificationReasonCode: 'social-manual-review-required',
    providerChannelPreference: 'in-app',
    parentTitleToken: 'social.alert.manualRequired.title',
    parentBodyToken: 'social.alert.manualRequired.body',
    parentActionToken: 'social.alert.action.reviewManually',
    dashboardPanelRefs: ['panel-manual-required-gaps'],
    explanationSnapshotRef: 'social-explanation-snapshot-alert-report',
    explanationEventRefs: ['social-explanation-event-manual-required'],
    evidenceReferences: [manualEvidenceRef()],
    policyRefs: ['policy-ref-social-manual-required'],
    auditRefs: ['audit-ref-social-alert-report'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: null,
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: ['provider-delivery-runtime-proof-required'],
    minimalPayloadFields: minimalPayloadFields(),
    deliveryClaimState: 'manual-required',
    rawAccountDataIncluded: false,
    rawVideoContentIncluded: false,
    rawMessageContentIncluded: false,
    screenshotIncluded: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    reportDeliveryClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    adapterDispatchState: 'not-dispatched',
    adapterActionClaimed: false,
    createdAt: '2026-06-07T01:39:00Z',
  };
}

function manualEvidenceRef(): unknown {
  return {
    evidenceReferenceId: 'evidence-social-manual-gap',
    kind: 'policy-decision',
    observedAt: '2026-06-07T01:39:00Z',
  };
}

function minimalPayloadFields(): readonly string[] {
  return [
    'alert-id',
    'family-device-scope',
    'severity',
    'reason-code',
    'evidence-ref',
    'policy-ref',
    'explanation-ref',
    'parent-action-link-ref',
  ];
}
