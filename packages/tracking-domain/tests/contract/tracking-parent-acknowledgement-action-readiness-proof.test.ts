import { describe, expect, it } from 'vitest';
import {
  TrackingParentAcknowledgementActionReadModelSchema,
  TrackingParentAcknowledgementActionRowSchema,
  buildTrackingParentAcknowledgementActionReadModel,
} from '../../src/tracking-parent-acknowledgement-action-readiness-proof';
import { TrackingPolicySchemaVersion } from '../../src/tracking-location-policy';

const EvidenceTrace = {
  evidenceReferenceId: 'tracking-parent-action-evidence-1',
  kind: 'journal-event',
  observedAt: '2026-06-06T17:40:00.000Z',
} as const;

describe('tracking parent acknowledgement action readiness proof', () => {
  it('derives parent action readiness rows from alert acknowledgement state', () => {
    const readModel = buildTrackingParentAcknowledgementActionReadModel(
      {
        generatedAt: '2026-06-06T17:45:00.000Z',
        readinessId: 'tracking-parent-acknowledgement-action-readiness-proof',
        sourceContractRefs: ['tracking-location-policy', 'tracking-wp17-parent-acknowledgement'],
      },
      trackingPolicyReadModelFixture()
    );

    expect(readModel.rows).toHaveLength(5);
    expect(readModel.actionReadyCount).toBe(2);
    expect(readModel.recordedCount).toBe(3);
    expect(readModel.manualRequiredCount).toBe(0);
    expect(readModel.productClaimReady).toBe(false);
    expect(readModel.renderedPortalAcknowledgementUiClaimed).toBe(false);
    expect(readModel.liveServiceMutationClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);

    expect(rowState(readModel, 'tracking-alert-safe')).toBe('acknowledgement-recorded');
    expect(rowAction(readModel, 'tracking-alert-expected')).toBe('mark-expected');
    expect(rowState(readModel, 'tracking-alert-false-alarm')).toBe('false-alarm-recorded');
    expect(rowAction(readModel, 'tracking-alert-check-in')).toBe('request-child-check-in');
    expect(rowState(readModel, 'tracking-alert-critical-review')).toBe('escalation-review-ready');
  });

  it('keeps critical acknowledgement and action rows visible with audit and manual proof refs', () => {
    const readModel = buildTrackingParentAcknowledgementActionReadModel(
      {
        generatedAt: '2026-06-06T17:45:00.000Z',
        readinessId: 'tracking-parent-acknowledgement-action-readiness-proof',
        sourceContractRefs: ['tracking-location-policy', 'tracking-wp17-parent-acknowledgement'],
      },
      trackingPolicyReadModelFixture()
    );
    const critical = readModel.rows.find((row) => row.alertId === 'tracking-alert-critical-review');

    expect(critical?.stillAlertForCritical).toBe(true);
    expect(critical?.allowedActions).toContain('escalate-manual-review');
    expect(critical?.manualProofRequirements).toContain('provider-delivery-proof-required');
    expect(critical?.auditRefs).toContain('tracking-parent-action-readiness-escalation-review-ready');
  });

  it('rejects rows and read models that claim rendered UI or live service mutation', () => {
    const readModel = buildTrackingParentAcknowledgementActionReadModel(
      {
        generatedAt: '2026-06-06T17:45:00.000Z',
        readinessId: 'tracking-parent-acknowledgement-action-readiness-proof',
        sourceContractRefs: ['tracking-location-policy', 'tracking-wp17-parent-acknowledgement'],
      },
      trackingPolicyReadModelFixture()
    );
    const unsafeRow = TrackingParentAcknowledgementActionRowSchema.safeParse({
      ...readModel.rows[0],
      renderedPortalAcknowledgementUiClaimed: true,
    });
    const unsafeReadModel = TrackingParentAcknowledgementActionReadModelSchema.safeParse({
      ...readModel,
      liveServiceMutationClaimed: true,
    });

    expect(unsafeRow.success).toBe(false);
    expect(unsafeReadModel.success).toBe(false);
  });
});

function rowState(readModel: ReturnType<typeof buildTrackingParentAcknowledgementActionReadModel>, alertId: string) {
  return readModel.rows.find((row) => row.alertId === alertId)?.actionState;
}

function rowAction(readModel: ReturnType<typeof buildTrackingParentAcknowledgementActionReadModel>, alertId: string) {
  return readModel.rows.find((row) => row.alertId === alertId)?.primaryAction;
}

function trackingPolicyReadModelFixture() {
  const alerts = [
    alert('tracking-alert-safe', 'warning', 'tracking-decision-safe', 'tracking-ack-safe'),
    alert('tracking-alert-expected', 'warning', 'tracking-decision-expected', 'tracking-ack-expected'),
    alert('tracking-alert-false-alarm', 'warning', 'tracking-decision-false-alarm', 'tracking-ack-false-alarm'),
    alert('tracking-alert-check-in', 'watch', 'tracking-decision-check-in', null),
    alert('tracking-alert-critical-review', 'critical', 'tracking-decision-critical-review', null),
  ] as const;

  return {
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: '2026-06-06T17:45:00.000Z',
    rules: alerts.map((entry) => rule(entry.policyDecisionId, entry.alertId)),
    decisions: [
      decision('tracking-decision-safe', 'tracking-alert-safe', 'request-parent-acknowledgement'),
      decision('tracking-decision-expected', 'tracking-alert-expected', 'request-parent-acknowledgement'),
      decision('tracking-decision-false-alarm', 'tracking-alert-false-alarm', 'request-parent-acknowledgement'),
      decision('tracking-decision-check-in', 'tracking-alert-check-in', 'ask-child-check-in'),
      decision('tracking-decision-critical-review', 'tracking-alert-critical-review', 'escalate'),
    ],
    acknowledgements: [
      acknowledgement('tracking-ack-safe', 'tracking-alert-safe', 'acknowledged-safe', null),
      acknowledgement('tracking-ack-expected', 'tracking-alert-expected', 'expected', '2026-06-06T20:00:00.000Z'),
      acknowledgement('tracking-ack-false-alarm', 'tracking-alert-false-alarm', 'false-alarm', null),
    ],
    checkInRequests: [],
    checkInResponses: [],
    aiRoutes: [],
    aiResults: [],
    alerts,
    escalations: [escalation('tracking-escalation-critical-review', 'tracking-alert-critical-review')],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  };
}

function rule(ruleId: string, alertId: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    ruleId,
    familyId: 'family-1',
    childProfileId: 'child-1',
    deviceId: 'parent-device-1',
    policyVersion: 'tracking-policy-v1',
    targetKind: 'geofence-transition',
    action: alertId === 'tracking-alert-check-in' ? 'ask-child-check-in' : 'request-parent-acknowledgement',
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: true,
    reasonCodes: ['tracking-parent-action-policy-rule'],
    auditRefs: [`tracking-rule-audit-${ruleId}`],
  } as const;
}

function decision(decisionId: string, alertId: string, action: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-06T17:41:00.000Z',
    ruleId: decisionId,
    action,
    dryRun: false,
    evidenceReferences: [EvidenceTrace],
    aiAnalysisId: null,
    alertIntentId: alertId,
    reasonCodes: ['tracking-parent-action-decision'],
    auditRefs: [`tracking-decision-audit-${alertId}`],
  } as const;
}

function alert(alertId: string, severity: string, policyDecisionId: string, acknowledgementId: string | null) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-06T17:41:30.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [EvidenceTrace],
    sensitiveDetailMode: 'authenticated-drill-in-only',
    notificationStatusRefs: [`notification-status-${alertId}`],
    acknowledgementId,
    reasonCodes: ['tracking-parent-action-alert'],
  } as const;
}

function acknowledgement(acknowledgementId: string, alertId: string, state: string, expiresAt: string | null) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    acknowledgementId,
    alertId,
    state,
    acknowledgedAt: '2026-06-06T17:42:00.000Z',
    expiresAt,
    stillAlertForCritical: true,
    reasonCodes: ['tracking-parent-action-acknowledgement'],
    auditRefs: [`tracking-acknowledgement-audit-${alertId}`],
  } as const;
}

function escalation(escalationId: string, alertId: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    escalationId,
    alertId,
    state: 'manual-required',
    startedAt: '2026-06-06T17:42:30.000Z',
    nextActionAt: '2026-06-06T18:00:00.000Z',
    steps: ['parent-manual-review', 'second-guardian-review'],
    auditRefs: [`tracking-escalation-audit-${alertId}`],
  } as const;
}
