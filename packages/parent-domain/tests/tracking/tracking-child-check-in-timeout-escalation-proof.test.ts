import { describe, expect, it } from 'vitest';
import {
  TrackingChildCheckInTimeoutReadModelSchema,
  TrackingChildCheckInTimeoutRowSchema,
  buildTrackingChildCheckInTimeoutReadModel,
} from '../../src/tracking-child-check-in-timeout-escalation-proof';
import { TrackingPolicySchemaVersion } from '../../src/tracking-location-policy';

const EvidenceTrace = {
  evidenceReferenceId: 'tracking-child-check-in-evidence-1',
  kind: 'journal-event',
  observedAt: '2026-06-06T18:00:00.000Z',
} as const;

describe('tracking child check-in timeout escalation proof', () => {
  it('derives waiting, safe, help, call-parent, and timeout escalation rows', () => {
    const readModel = proofReadModel();

    expect(readModel.rows).toHaveLength(5);
    expect(readModel.waitingCount).toBe(1);
    expect(readModel.resolvedCount).toBe(1);
    expect(readModel.escalationReadyCount).toBe(3);
    expect(readModel.locationSampleRequestedCount).toBe(5);
    expect(readModel.attachedLocationSampleCount).toBe(2);
    expect(readModel.auditedPromptCount).toBe(2);
    expect(readModel.auditedResponseCount).toBe(3);
    expect(readModel.ruleOnlyEscalationCount).toBe(1);
    expect(readModel.safeAlertOutcomeCount).toBe(1);
    expect(readModel.productClaimReady).toBe(false);
    expect(readModel.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.renderedChildDeviceUiClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);

    expect(rowState(readModel, 'tracking-check-in-waiting')).toBe('waiting-for-child');
    expect(rowState(readModel, 'tracking-check-in-safe')).toBe('safe-response-recorded');
    expect(rowState(readModel, 'tracking-check-in-help')).toBe('help-response-escalation-ready');
    expect(rowState(readModel, 'tracking-check-in-call-parent')).toBe('call-parent-response-escalation-ready');
    expect(rowState(readModel, 'tracking-check-in-expired')).toBe('expired-timeout-escalation-ready');
  });

  it('preserves evidence, audit, location, and parent action refs for help escalation rows', () => {
    const readModel = proofReadModel();
    const help = readModel.rows.find((row) => row.checkInId === 'tracking-check-in-help');

    expect(help?.escalates).toBe(true);
    expect(help?.locationEvidenceReferenceId).toBe('tracking-child-check-in-help-location');
    expect(help?.locationSampleState).toBe('attached-from-child-response');
    expect(help?.auditCoverageState).toBe('prompt-and-response-audited');
    expect(help?.alertOutcome).toBe('parent-review-required');
    expect(help?.escalationBasis).toBe('child-help-response');
    expect(help?.parentActionRefs).toContain('tracking-parent-review-child-check-in-tracking-check-in-help');
  });

  it('preserves audit, timeout basis, and no-claim refs for timeout escalation rows', () => {
    const readModel = proofReadModel();
    const timeout = readModel.rows.find((row) => row.checkInId === 'tracking-check-in-expired');

    expect(timeout?.locationSampleState).toBe('requested-not-yet-attached');
    expect(timeout?.auditCoverageState).toBe('prompt-audited');
    expect(timeout?.alertOutcome).toBe('parent-review-required');
    expect(timeout?.escalationBasis).toBe('expired-rule-only-timeout');
    expect(timeout?.manualProofRequirements).toContain('timeout-worker-proof-required');
    expect(timeout?.providerDeliveryClaimed).toBe(false);
  });

  it('rejects rows and read models that claim child runtime or physical proof', () => {
    const readModel = proofReadModel();
    const unsafeRow = TrackingChildCheckInTimeoutRowSchema.safeParse({
      ...readModel.rows[0],
      childDeviceDeliveryRuntimeClaimed: true,
    });
    const unsafeReadModel = TrackingChildCheckInTimeoutReadModelSchema.safeParse({
      ...readModel,
      physicalDeviceProofClaimed: true,
    });

    expect(unsafeRow.success).toBe(false);
    expect(unsafeReadModel.success).toBe(false);
  });
});

function rowState(readModel: ReturnType<typeof buildTrackingChildCheckInTimeoutReadModel>, checkInId: string) {
  return readModel.rows.find((row) => row.checkInId === checkInId)?.resolutionState;
}

function proofReadModel() {
  return buildTrackingChildCheckInTimeoutReadModel(
    {
      generatedAt: '2026-06-06T18:10:00.000Z',
      readinessId: 'tracking-child-check-in-timeout-escalation-proof',
      sourceContractRefs: ['tracking-location-policy', 'tracking-wp18-child-check-in'],
    },
    trackingPolicyReadModelFixture()
  );
}

function trackingPolicyReadModelFixture() {
  const alerts = [
    alert('tracking-alert-waiting', 'watch', 'tracking-decision-waiting'),
    alert('tracking-alert-safe', 'warning', 'tracking-decision-safe'),
    alert('tracking-alert-help', 'urgent', 'tracking-decision-help'),
    alert('tracking-alert-call-parent', 'urgent', 'tracking-decision-call-parent'),
    alert('tracking-alert-expired', 'critical', 'tracking-decision-expired'),
  ] as const;
  const requests = [
    checkInRequest('tracking-check-in-waiting', 'tracking-alert-waiting', 'sent', '2026-06-06T18:30:00.000Z'),
    checkInRequest('tracking-check-in-safe', 'tracking-alert-safe', 'sent', '2026-06-06T18:30:00.000Z'),
    checkInRequest('tracking-check-in-help', 'tracking-alert-help', 'sent', '2026-06-06T18:30:00.000Z'),
    checkInRequest('tracking-check-in-call-parent', 'tracking-alert-call-parent', 'sent', '2026-06-06T18:30:00.000Z'),
    checkInRequest('tracking-check-in-expired', 'tracking-alert-expired', 'sent', '2026-06-06T18:05:00.000Z'),
  ] as const;

  return {
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: '2026-06-06T18:10:00.000Z',
    rules: alerts.map((entry) => rule(entry.policyDecisionId)),
    decisions: alerts.map((entry) => decision(entry.policyDecisionId, entry.alertId)),
    acknowledgements: [],
    checkInRequests: requests,
    checkInResponses: [
      checkInResponse('tracking-check-in-safe', 'safe', 'tracking-child-check-in-safe-location'),
      checkInResponse('tracking-check-in-help', 'help', 'tracking-child-check-in-help-location'),
      checkInResponse('tracking-check-in-call-parent', 'call-parent', null),
    ],
    aiRoutes: [],
    aiResults: [],
    alerts,
    escalations: [],
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
    action: 'ask-child-check-in',
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: true,
    reasonCodes: ['tracking-child-check-in-policy-rule'],
    auditRefs: [`tracking-rule-audit-${ruleId}`],
  } as const;
}

function decision(decisionId: string, alertId: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-06T18:01:00.000Z',
    ruleId: decisionId,
    action: 'ask-child-check-in',
    dryRun: false,
    evidenceReferences: [EvidenceTrace],
    aiAnalysisId: null,
    alertIntentId: alertId,
    reasonCodes: ['tracking-child-check-in-decision'],
    auditRefs: [`tracking-decision-audit-${alertId}`],
  } as const;
}

function alert(alertId: string, severity: string, policyDecisionId: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-06T18:01:30.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [EvidenceTrace],
    sensitiveDetailMode: 'authenticated-drill-in-only',
    notificationStatusRefs: [`notification-status-${alertId}`],
    acknowledgementId: null,
    reasonCodes: ['tracking-child-check-in-alert'],
  } as const;
}

function checkInRequest(checkInId: string, relatedAlertId: string, state: string, expiresAt: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    checkInId,
    requestedAt: '2026-06-06T18:02:00.000Z',
    state,
    relatedAlertId,
    includeLocationIfPermitted: true,
    expiresAt,
    evidenceReferences: [EvidenceTrace],
    auditRefs: [`tracking-child-check-in-request-${checkInId}`],
  } as const;
}

function checkInResponse(checkInId: string, response: string, locationEvidenceReferenceId: string | null) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    checkInId,
    respondedAt: '2026-06-06T18:04:00.000Z',
    response,
    locationEvidenceReference:
      locationEvidenceReferenceId === null
        ? null
        : {
            evidenceReferenceId: locationEvidenceReferenceId,
            kind: 'journal-event',
            observedAt: '2026-06-06T18:03:30.000Z',
          },
    auditRefs: [`tracking-child-check-in-response-${checkInId}`],
  } as const;
}
