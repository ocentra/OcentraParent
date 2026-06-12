import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  parseAgentSocialAlertReportReadModelEvent,
  SocialAlertReportReadModelSnapshotSchema,
} from '../../src/contracts';
import {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
} from '@ocentra-parent/social-domain/social-alert-report-intent';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
} from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-07T01:39:00Z';

describe('social alert/report read model adapter', () => {
  it('parses ref-only service rows from the reported event', () => {
    const result = parseAgentSocialAlertReportReadModelEvent(eventWithSnapshot(snapshot()));

    expect(result.ok).toBe(true);
    if (!result.ok) {
      return;
    }
    expect(result.value.intents).toHaveLength(1);
    expect(result.value.providerStatusRows).toHaveLength(1);
    expect(result.value.intents[0]?.intentKind).toBe(SocialAlertReportIntentKind.HighRiskSignal);
    expect(result.value.providerStatusRows[0]?.providerStatus).toBe('manual-required');
    expect(result.value.claimBoundaries.providerDelivery).toBe('not-claimed');
  });

  it('rejects wrong event missing json invalid json and unsafe delivery claims', () => {
    expect(
      parseAgentSocialAlertReportReadModelEvent({ ...eventWithSnapshot(snapshot()), event: AgentEvent.HealthReported })
        .ok
    ).toBe(false);
    expect(parseAgentSocialAlertReportReadModelEvent(eventWithPayload({})).ok).toBe(false);
    expect(
      parseAgentSocialAlertReportReadModelEvent(
        eventWithPayload({ [AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel]: '{' })
      ).ok
    ).toBe(false);
    expect(
      SocialAlertReportReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        intents: [{ ...intent(), providerDeliveryAttempted: true }],
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        providerStatusRows: [{ ...providerStatusRow(), providerReceiptRefs: ['provider-receipt-claimed'] }],
      }).success
    ).toBe(false);
  });
});

function eventWithSnapshot(value: unknown): AgentEventEnvelope {
  return eventWithPayload({
    [AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel]: JSON.stringify(value),
  });
}

function eventWithPayload(payload: AgentEventEnvelope['payload']): AgentEventEnvelope {
  return {
    schemaVersion: 1,
    eventId: 'event-social-alert-report',
    correlationId: 'command-social-alert-report',
    sentAt: Timestamp,
    source: {
      peerId: 'agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal',
      role: 'portal',
    },
    event: AgentEvent.BrowserSocialAlertReportReadModelReported,
    severity: 'info',
    payload,
    snapshot: null,
  };
}

function snapshot() {
  return {
    schemaVersion: 'social-alert-report-read-model',
    familyId: 'family-social-alert-report-service',
    childProfileId: 'child-social-alert-report-service',
    generatedAt: Timestamp,
    intents: [intent()],
    providerStatusRows: [providerStatusRow()],
    claimBoundaries: {
      providerDelivery: 'not-claimed',
      reportDelivery: 'not-claimed',
      parentNotificationUi: 'not-claimed',
      finalPolicyDecision: 'not-claimed',
      enforcement: 'not-claimed',
    },
  };
}

function providerStatusRow() {
  return {
    statusEntryId: 'social-provider-status-social-alert-report-high-risk-service',
    sourceIntentRef: 'social-alert-report-high-risk-service',
    sourcePreflightStatus: 'provider-adapter-required',
    providerStatus: 'manual-required',
    statusProofState: 'manual-action-required',
    deliveryClaimState: 'not-observed',
    providerAttemptRef: 'social-provider-attempt-not-started-social-alert-report-high-risk-service',
    readinessRefs: [
      'provider-adapter-required-social-alert-report-high-risk-service',
      'provider-credentials-required-social-alert-report-high-risk-service',
      'provider-smoke-proof-required-social-alert-report-high-risk-service',
    ],
    providerReceiptRefs: [],
    manualProofRequirements: [
      'provider-adapter-required-social-alert-report-high-risk-service',
      'provider-credentials-required-social-alert-report-high-risk-service',
      'provider-smoke-proof-required-social-alert-report-high-risk-service',
    ],
    providerDeliveryImplemented: false,
    providerDeliveryObserved: false,
    deliveredNotificationClaimed: false,
    sensitiveProviderPayloadClaimed: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: Timestamp,
  };
}

function intent() {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    alertReportIntentId: 'social-alert-report-high-risk-service',
    intentKind: SocialAlertReportIntentKind.HighRiskSignal,
    intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-alert-report',
      childProfileId: 'child-social-alert-report-service',
      label: 'Study Phone',
      platform: 'android',
    },
    notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
    providerChannelPreference: 'in-app',
    parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
    parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-alert-report',
    explanationEventRefs: ['social-explanation-event-feed-video-gate'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-route-gate',
        kind: ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: Timestamp,
      },
    ],
    policyRefs: ['policy-ref-social-high-risk'],
    auditRefs: ['audit-ref-social-alert-report'],
    parentReportRef: null,
    parentActionRef: {
      actionReferenceId: 'parent-action-social-review',
      actor: {
        actorId: 'parent-local-account',
        role: ParentActorRole.Parent,
      },
      policyVersion: 'policy-social-alert-report-v1',
      createdAt: Timestamp,
    },
    localOutboxRecordRef: 'local-outbox-social-alert-report',
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: [],
    minimalPayloadFields: Object.values(SocialAlertReportPayloadField),
    deliveryClaimState: SocialAlertReportDeliveryClaimState.LocalOutboxOnly,
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
    adapterDispatchState: SocialAlertReportAdapterDispatchState.NotDispatched,
    adapterActionClaimed: false,
    createdAt: Timestamp,
  };
}
