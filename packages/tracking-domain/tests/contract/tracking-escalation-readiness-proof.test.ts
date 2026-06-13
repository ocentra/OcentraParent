import { describe, expect, it } from 'vitest';
import {
  TrackingEscalationReadinessReadModelSchema,
  TrackingEscalationReadinessRowSchema,
  buildTrackingEscalationReadinessReadModel,
} from '../../src/tracking-escalation-readiness-proof';
import { TrackingPolicySchemaVersion } from '../../src/tracking-location-policy';

const EvidenceTrace = {
  evidenceReferenceId: 'tracking-escalation-location-evidence-1',
  kind: 'journal-event',
  observedAt: '2026-06-05T12:00:00.000Z',
} as const;

describe('tracking escalation readiness proof', () => {
  it('derives acknowledgement, check-in, guardian, and critical escalation readiness rows', () => {
    const readModel = buildTrackingEscalationReadinessReadModel(
      {
        generatedAt: '2026-06-05T12:10:00.000Z',
        readinessId: 'tracking-escalation-readiness-proof',
        sourceContractRefs: ['tracking-location-policy', 'notification-escalation-expectations'],
      },
      trackingPolicyReadModelFixture()
    );

    expect(readModel.rows).toHaveLength(4);
    expect(readModel.waitingCount).toBe(0);
    expect(readModel.resolvedCount).toBe(2);
    expect(readModel.manualRequiredCount).toBe(2);
    expect(readModel.aiEscalationBlockedCount).toBe(4);
    expect(readModel.productClaimReady).toBe(false);
    expect(readModel.emergencyServicesAutoContactClaimed).toBe(false);
    expect(readModel.providerDeliveryClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);

    expect(rowState(readModel, 'tracking-alert-acknowledged')).toBe('resolved-by-parent-acknowledgement');
    expect(rowState(readModel, 'tracking-alert-safe-check-in')).toBe('resolved-by-child-check-in');
    expect(rowState(readModel, 'tracking-alert-urgent-no-response')).toBe('second-guardian-required');
    expect(rowState(readModel, 'tracking-alert-critical')).toBe('critical-multi-channel-manual-required');
  });

  it('keeps urgent and critical rows manual-required with proof refs instead of delivery claims', () => {
    const readModel = buildTrackingEscalationReadinessReadModel(
      {
        generatedAt: '2026-06-05T12:10:00.000Z',
        readinessId: 'tracking-escalation-readiness-proof',
        sourceContractRefs: ['tracking-location-policy', 'notification-escalation-expectations'],
      },
      trackingPolicyReadModelFixture()
    );
    const urgent = readModel.rows.find((row) => row.alertId === 'tracking-alert-urgent-no-response');
    const critical = readModel.rows.find((row) => row.alertId === 'tracking-alert-critical');

    expect(urgent?.guardianActionRefs).toContain('tracking-second-guardian-review-tracking-alert-urgent-no-response');
    expect(urgent?.manualProofRequirements).toContain('provider-delivery-proof-required');
    expect(critical?.guardianActionRefs).toContain('tracking-critical-parent-call-tracking-alert-critical');
    expect(critical?.emergencyServicesAutoContactClaimed).toBe(false);
    expect(critical?.providerDeliveryClaimed).toBe(false);
  });

  it('rejects rows and read models that overclaim emergency contact or provider delivery', () => {
    const readModel = buildTrackingEscalationReadinessReadModel(
      {
        generatedAt: '2026-06-05T12:10:00.000Z',
        readinessId: 'tracking-escalation-readiness-proof',
        sourceContractRefs: ['tracking-location-policy', 'notification-escalation-expectations'],
      },
      trackingPolicyReadModelFixture()
    );
    const row = readModel.rows[0];
    const unsafeRow = TrackingEscalationReadinessRowSchema.safeParse({
      ...row,
      providerDeliveryClaimed: true,
    });
    const unsafeReadModel = TrackingEscalationReadinessReadModelSchema.safeParse({
      ...readModel,
      emergencyServicesAutoContactClaimed: true,
    });

    expect(unsafeRow.success).toBe(false);
    expect(unsafeReadModel.success).toBe(false);
  });
});

function rowState(readModel: ReturnType<typeof buildTrackingEscalationReadinessReadModel>, alertId: string) {
  return readModel.rows.find((row) => row.alertId === alertId)?.readinessState;
}

function trackingPolicyReadModelFixture() {
  const alerts = [
    alert('tracking-alert-acknowledged', 'warning', 'tracking-decision-acknowledged', 'tracking-ack-1'),
    alert('tracking-alert-safe-check-in', 'warning', 'tracking-decision-safe-check-in', null),
    alert('tracking-alert-urgent-no-response', 'urgent', 'tracking-decision-urgent', null),
    alert('tracking-alert-critical', 'critical', 'tracking-decision-critical', null),
  ] as const;

  return {
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: '2026-06-05T12:10:00.000Z',
    rules: alerts.map((entry) => rule(entry.policyDecisionId)),
    decisions: alerts.map((entry) => decision(entry.policyDecisionId, entry.alertId)),
    acknowledgements: [
      {
        schemaVersion: TrackingPolicySchemaVersion,
        acknowledgementId: 'tracking-ack-1',
        alertId: 'tracking-alert-acknowledged',
        state: 'acknowledged-safe',
        acknowledgedAt: '2026-06-05T12:02:00.000Z',
        expiresAt: null,
        stillAlertForCritical: true,
        reasonCodes: ['parent-confirmed-safe'],
        auditRefs: ['tracking-acknowledgement-audit-1'],
      },
    ],
    checkInRequests: [
      checkInRequest('tracking-check-in-safe', 'tracking-alert-safe-check-in', 'sent', '2026-06-05T12:20:00.000Z'),
      checkInRequest(
        'tracking-check-in-urgent',
        'tracking-alert-urgent-no-response',
        'sent',
        '2026-06-05T12:05:00.000Z'
      ),
    ],
    checkInResponses: [
      {
        schemaVersion: TrackingPolicySchemaVersion,
        checkInId: 'tracking-check-in-safe',
        respondedAt: '2026-06-05T12:04:00.000Z',
        response: 'safe',
        locationEvidenceReference: EvidenceTrace,
        auditRefs: ['tracking-check-in-safe-response'],
      },
    ],
    aiRoutes: [],
    aiResults: [],
    alerts,
    escalations: [
      escalation('tracking-escalation-acknowledged', 'tracking-alert-acknowledged', 'waiting-for-parent'),
      escalation('tracking-escalation-safe-check-in', 'tracking-alert-safe-check-in', 'waiting-for-child'),
      escalation('tracking-escalation-urgent', 'tracking-alert-urgent-no-response', 'waiting-for-child'),
      escalation('tracking-escalation-critical', 'tracking-alert-critical', 'waiting-for-parent'),
    ],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  };
}

function rule(ruleId: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    ruleId,
    familyId: 'family-1',
    childProfileId: 'child-1',
    deviceId: 'parent-device-1',
    policyVersion: 'tracking-policy-v1',
    targetKind: 'geofence-transition',
    action: 'escalate',
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: true,
    reasonCodes: ['tracking-escalation-policy-rule'],
    auditRefs: [`tracking-rule-audit-${ruleId}`],
  } as const;
}

function decision(decisionId: string, alertId: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-05T12:01:00.000Z',
    ruleId: decisionId,
    action: 'escalate',
    dryRun: false,
    evidenceReferences: [EvidenceTrace],
    aiAnalysisId: `tracking-ai-analysis-${alertId}`,
    alertIntentId: alertId,
    reasonCodes: ['tracking-parent-policy-authorized-escalation'],
    auditRefs: [`tracking-decision-audit-${alertId}`],
  } as const;
}

function alert(alertId: string, severity: string, policyDecisionId: string, acknowledgementId: string | null) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-05T12:01:30.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [EvidenceTrace],
    sensitiveDetailMode: 'authenticated-drill-in-only',
    notificationStatusRefs: [`notification-status-${alertId}`],
    acknowledgementId,
    reasonCodes: ['tracking-alert-escalation-candidate'],
  } as const;
}

function checkInRequest(checkInId: string, relatedAlertId: string, state: string, expiresAt: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    checkInId,
    requestedAt: '2026-06-05T12:02:00.000Z',
    state,
    relatedAlertId,
    includeLocationIfPermitted: true,
    expiresAt,
    evidenceReferences: [EvidenceTrace],
    auditRefs: [`tracking-check-in-request-${checkInId}`],
  } as const;
}

function escalation(escalationId: string, alertId: string, state: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    escalationId,
    alertId,
    state,
    startedAt: '2026-06-05T12:01:30.000Z',
    nextActionAt: '2026-06-05T12:15:00.000Z',
    steps: ['wait-parent-acknowledgement', 'request-child-check-in', 'guardian-manual-review'],
    auditRefs: [`tracking-escalation-audit-${alertId}`],
  } as const;
}
