import { describe, expect, it } from 'vitest';
import {
  TrackingChildRuntimeDeliveryBoundaryReadModelSchema,
  TrackingChildRuntimeDeliveryBoundaryRowSchema,
  buildTrackingChildRuntimeDeliveryBoundaryReadModel,
} from '../src/tracking-child-runtime-delivery-boundary-proof';
import { TrackingPolicySchemaVersion } from '../src/tracking-location-policy';
import { buildTrackingChildCheckInTimeoutReadModel } from '../src/tracking-child-check-in-timeout-escalation-proof';

const EvidenceTrace = {
  evidenceReferenceId: 'tracking-child-runtime-evidence-1',
  kind: 'journal-event',
  observedAt: '2026-06-07T14:00:00.000Z',
} as const;
const HostedUiProofRefs = [
  'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/19-child-runtime-ui-proof.json',
  'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-runtime-ui.png',
] as const;

describe('tracking child runtime delivery boundary proof', () => {
  it('derives hosted-copy rows from child check-in timeout rows without child runtime claims', () => {
    const readModel = proofReadModel();

    expect(readModel.rows).toHaveLength(5);
    expect(readModel.hostedCopyOnlyCount).toBe(5);
    expect(readModel.safeResponseDisclosureCount).toBe(1);
    expect(readModel.escalationDisclosureCount).toBe(3);
    expect(readModel.manualRuntimeProofRequiredCount).toBe(0);
    expect(readModel.requiredRuntimeProofRefCount).toBe(25);
    expect(readModel.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(readModel.childDeviceExecutionRuntimeClaimed).toBe(false);
    expect(readModel.renderedChildDeviceUiRuntimeClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);
    expect(readModel.productReadyClaimed).toBe(false);
  });

  it('keeps hosted UI proof refs, evidence refs, and runtime proof requirements on each row', () => {
    const readModel = proofReadModel();
    const help = readModel.rows.find((row) => row.sourceCheckInId === 'tracking-check-in-help');
    const safe = readModel.rows.find((row) => row.sourceCheckInId === 'tracking-check-in-safe');

    expect(help?.boundaryState).toBe('hosted-copy-only-escalation-ready');
    expect(help?.hostedUiState).toBe('help-response-disclosure');
    expect(help?.hostedUiProofRefs).toEqual([...HostedUiProofRefs]);
    expect(help?.sourceEvidenceRefs).toContain('tracking-child-check-in-help-location');
    expect(help?.requiredRuntimeProofRefs).toContain(
      'child-device-execution-runtime-proof-required-tracking-check-in-help'
    );
    expect(help?.parentVisibleStatusRefs).toContain('tracking-parent-review-child-check-in-tracking-check-in-help');

    expect(safe?.boundaryState).toBe('hosted-copy-only-safe-response');
    expect(safe?.hostedUiState).toBe('safe-response-disclosure');
  });

  it('rejects rows and read models that claim child runtime, provider, or authority proof', () => {
    const readModel = proofReadModel();
    const unsafeRow = TrackingChildRuntimeDeliveryBoundaryRowSchema.safeParse({
      ...readModel.rows[0],
      childDeviceExecutionRuntimeClaimed: true,
    });
    const unsafeReadModel = TrackingChildRuntimeDeliveryBoundaryReadModelSchema.safeParse({
      ...readModel,
      authorityProofClaimed: true,
    });

    expect(unsafeRow.success).toBe(false);
    expect(unsafeReadModel.success).toBe(false);
  });
});

function proofReadModel() {
  return buildTrackingChildRuntimeDeliveryBoundaryReadModel(
    {
      generatedAt: '2026-06-07T14:10:00.000Z',
      readinessId: 'tracking-child-runtime-delivery-boundary-proof',
      sourceContractRefs: ['tracking-child-check-in-timeout-escalation-proof', 'tracking-hosted-ui-child-runtime'],
      hostedUiProofRefs: HostedUiProofRefs,
    },
    timeoutReadModel()
  );
}

function timeoutReadModel() {
  return buildTrackingChildCheckInTimeoutReadModel(
    {
      generatedAt: '2026-06-07T14:05:00.000Z',
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
    checkInRequest('tracking-check-in-waiting', 'tracking-alert-waiting', 'sent', '2026-06-07T14:30:00.000Z'),
    checkInRequest('tracking-check-in-safe', 'tracking-alert-safe', 'sent', '2026-06-07T14:30:00.000Z'),
    checkInRequest('tracking-check-in-help', 'tracking-alert-help', 'sent', '2026-06-07T14:30:00.000Z'),
    checkInRequest('tracking-check-in-call-parent', 'tracking-alert-call-parent', 'sent', '2026-06-07T14:30:00.000Z'),
    checkInRequest('tracking-check-in-expired', 'tracking-alert-expired', 'sent', '2026-06-07T14:05:00.000Z'),
  ] as const;

  return {
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: '2026-06-07T14:05:00.000Z',
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
    reasonCodes: ['tracking-child-runtime-policy-rule'],
    auditRefs: [`tracking-rule-audit-${ruleId}`],
  } as const;
}

function decision(decisionId: string, alertId: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-07T14:01:00.000Z',
    ruleId: decisionId,
    action: 'ask-child-check-in',
    dryRun: false,
    evidenceReferences: [EvidenceTrace],
    aiAnalysisId: null,
    alertIntentId: alertId,
    reasonCodes: ['tracking-child-runtime-decision'],
    auditRefs: [`tracking-decision-audit-${alertId}`],
  } as const;
}

function alert(alertId: string, severity: string, policyDecisionId: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-07T14:01:30.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [EvidenceTrace],
    sensitiveDetailMode: 'authenticated-drill-in-only',
    notificationStatusRefs: [`notification-status-${alertId}`],
    acknowledgementId: null,
    reasonCodes: ['tracking-child-runtime-alert'],
  } as const;
}

function checkInRequest(checkInId: string, relatedAlertId: string, state: string, expiresAt: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    checkInId,
    requestedAt: '2026-06-07T14:02:00.000Z',
    state,
    relatedAlertId,
    includeLocationIfPermitted: true,
    expiresAt,
    evidenceReferences: [EvidenceTrace],
    auditRefs: [`tracking-child-runtime-request-${checkInId}`],
  } as const;
}

function checkInResponse(checkInId: string, response: string, locationEvidenceReferenceId: string | null) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    checkInId,
    respondedAt: '2026-06-07T14:04:00.000Z',
    response,
    locationEvidenceReference:
      locationEvidenceReferenceId === null
        ? null
        : {
            evidenceReferenceId: locationEvidenceReferenceId,
            kind: 'journal-event',
            observedAt: '2026-06-07T14:03:30.000Z',
          },
    auditRefs: [`tracking-child-runtime-response-${checkInId}`],
  } as const;
}
