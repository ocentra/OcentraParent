import { describe, expect, it } from 'vitest';
import {
  TrackingExpectedPlaceAlertPolicyProofSchema,
  TrackingExpectedPlaceAlertPolicyRowStatus,
  buildTrackingExpectedPlaceAlertPolicyProof,
} from '../../src/tracking-expected-place-alert-policy-proof';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from '@ocentra-parent/schema-domain/tracking-location-policy';

const Timestamp = '2026-06-06T17:16:00.000Z';

describe('tracking expected-place alert policy proof', () => {
  it('derives expected-place alert, check-in, suppressed, and manual UI-readiness rows', () => {
    const proof = expectedPlaceProof();

    expect(proof.alertPolicyReadyCount).toBe(1);
    expect(proof.checkInPolicyReadyCount).toBe(1);
    expect(proof.suppressedNoActionCount).toBe(1);
    expect(proof.manualRequiredCount).toBe(1);
    expect(proof.rows.map((row) => row.status)).toEqual([
      TrackingExpectedPlaceAlertPolicyRowStatus.AlertPolicyReady,
      TrackingExpectedPlaceAlertPolicyRowStatus.CheckInPolicyReady,
      TrackingExpectedPlaceAlertPolicyRowStatus.SuppressedNoAction,
      TrackingExpectedPlaceAlertPolicyRowStatus.ManualRequired,
    ]);
  });

  it('preserves schedule rule, decision, alert, evidence, reason, and UI surface refs', () => {
    const [alertReady, checkInReady, suppressed, manual] = expectedPlaceProof().rows;

    expect(alertReady.sourceRule.ruleId).toBe('expected-place-rule-school-arrival');
    expect(alertReady.sourceDecision.alertIntentId).toBe('expected-place-alert-school-arrival');
    expect(alertReady.sourceAlert?.notificationStatusRefs).toEqual(['expected-place-notification-intent-school']);
    expect(alertReady.uiSurfaceRef).toBe('tracking-expected-place-ui-readiness-expected-place-decision-school');
    expect(alertReady.evidenceReferences[0]?.evidenceReferenceId).toBe('expected-place-evidence-school');
    expect(alertReady.reasonCodes).toContain('expected-place-school-arrival');
    expect(checkInReady.sourceDecision.action).toBe('ask-child-check-in');
    expect(suppressed.sourceDecision.action).toBe('no-action');
    expect(manual.manualProofRequirements).toContain(
      'tracking-expected-place-manual-proof-expected-place-decision-low-accuracy'
    );
  });

  it('rejects rendered UI, delivery, runtime, physical-device, authority, and production overclaims', () => {
    const proof = expectedPlaceProof();

    expect(proof.renderedParentUiClaimed).toBe(false);
    expect(proof.alertDeliveryRuntimeClaimed).toBe(false);
    expect(proof.providerDeliveryClaimed).toBe(false);
    expect(proof.notificationReceiptRuntimeClaimed).toBe(false);
    expect(proof.childDeviceRuntimeClaimed).toBe(false);
    expect(proof.physicalDeviceProofClaimed).toBe(false);
    expect(proof.authorityProofClaimed).toBe(false);
    expect(
      TrackingExpectedPlaceAlertPolicyProofSchema.safeParse({
        ...proof,
        renderedParentUiClaimed: true,
      }).success
    ).toBe(false);
  });
});

function expectedPlaceProof() {
  return buildTrackingExpectedPlaceAlertPolicyProof({
    generatedAt: Timestamp,
    sourceReadModelRef: 'tracking-location-policy-read-model-expected-place-alert-policy',
    sourceProofRefs: [
      'output/tracking-plan-proof/16-expected-place-schedule-engine/06-expected-place-proof.json',
      'output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/proof.json',
    ],
    readModel: sourceReadModel(),
  });
}

function sourceReadModel() {
  return TrackingLocationPolicyReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: Timestamp,
    rules: [
      expectedPlaceRule('school-arrival', 'notify-parent'),
      expectedPlaceRule('late-bus', 'ask-child-check-in'),
      expectedPlaceRule('holiday', 'no-action'),
      expectedPlaceRule('low-accuracy', 'manual-required'),
    ],
    decisions: [
      decision('school', 'school-arrival', 'notify-parent', 'expected-place-alert-school-arrival'),
      decision('late-bus', 'late-bus', 'ask-child-check-in', null),
      decision('holiday', 'holiday', 'no-action', null),
      decision('low-accuracy', 'low-accuracy', 'manual-required', null),
    ],
    acknowledgements: [],
    checkInRequests: [],
    checkInResponses: [],
    aiRoutes: [],
    aiResults: [],
    alerts: [alert()],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  });
}

function expectedPlaceRule(suffix: string, action: string) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    ruleId: `expected-place-rule-${suffix}`,
    familyId: 'family-expected-place-alert-policy',
    childProfileId: 'child-expected-place-alert-policy',
    deviceId: 'parent-device-expected-place-alert-policy',
    policyVersion: 'tracking-policy-expected-place-alert-policy',
    targetKind: 'expected-place',
    action,
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: action === 'manual-required',
    reasonCodes: [`expected-place-${suffix}`],
    auditRefs: [`expected-place-rule-audit-${suffix}`],
  };
}

function decision(label: string, ruleSuffix: string, action: string, alertIntentId: string | null) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    decisionId: `expected-place-decision-${label}`,
    decidedAt: Timestamp,
    ruleId: `expected-place-rule-${ruleSuffix}`,
    action,
    dryRun: false,
    evidenceReferences: [evidence(label)],
    aiAnalysisId: null,
    alertIntentId,
    reasonCodes: [`expected-place-decision-${label}`],
    auditRefs: [`expected-place-decision-audit-${label}`],
  };
}

function alert() {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    alertId: 'expected-place-alert-school-arrival',
    createdAt: Timestamp,
    severity: 'warning',
    policyDecisionId: 'expected-place-decision-school',
    evidenceReferences: [evidence('school')],
    sensitiveDetailMode: 'minimal-provider-body',
    notificationStatusRefs: ['expected-place-notification-intent-school'],
    acknowledgementId: null,
    reasonCodes: ['expected-place-school-arrival-alert'],
  };
}

function evidence(label: string) {
  return {
    evidenceReferenceId: `expected-place-evidence-${label}`,
    kind: 'journal-event',
    observedAt: Timestamp,
  };
}
