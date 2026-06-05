import { describe, expect, it } from 'vitest';
import {
  TrackingExpectedPlaceAlertRowSchema,
  buildTrackingExpectedPlaceAlertPolicyProof,
} from '../src/tracking-expected-place-alert-policy-proof';

const Proof = buildTrackingExpectedPlaceAlertPolicyProof({
  generatedAt: '2026-06-05T22:00:00.000Z',
  proofId: 'tracking-expected-place-alert-policy-proof',
  sourceExpectedPlaceProofRef:
    'output/tracking-plan-proof/16-expected-place-schedule-engine/06-expected-place-proof.json',
  sourcePolicyCompilerProofRef: 'output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/proof.json',
  sourceNotificationBoundaryRef:
    'output/tracking-plan-proof/26-alert-severity-and-notification-model/proof-summary.json',
});

describe('tracking expected-place alert policy proof', () => {
  it('maps expected-place decisions into alert policy rows with schedule, place, rule, and evidence refs', () => {
    expect(Proof.rows.map((row) => row.alertPolicyState)).toEqual([
      'no-alert-expected',
      'alert-ready',
      'manual-required',
      'suppressed-disabled-rule',
    ]);
    expect(Proof.rows.every((row) => row.scheduleRefs.length === 1)).toBe(true);
    expect(Proof.rows.every((row) => row.placeRefs.length === 1)).toBe(true);
    expect(Proof.rows.every((row) => row.evidenceRefs.length === 1)).toBe(true);
    expect(Proof.rows.every((row) => row.rule.targetKind === 'expected-place')).toBe(true);
  });

  it('creates only the missed-arrival alert row and keeps stale evidence manual', () => {
    const alertRows = Proof.rows.filter((row) => row.alertIntent !== null);
    const manualRows = Proof.rows.filter((row) => row.alertPolicyState === 'manual-required');

    expect(alertRows.map((row) => row.sourceExpectedPlaceDecisionId)).toEqual(['expected-place-practice-missed']);
    expect(alertRows[0]?.alertIntent?.severity).toBe('urgent');
    expect(alertRows[0]?.alertIntent?.sensitiveDetailMode).toBe('minimal-provider-body');
    expect(manualRows.map((row) => row.decision.action)).toEqual(['manual-required']);
    expect(manualRows[0]?.reasonCodeRefs).toContain('expected-place-evidence-stale');
  });

  it('suppresses disabled rules without dropping audit or evidence refs', () => {
    const disabled = Proof.rows.find((row) => row.alertPolicyState === 'suppressed-disabled-rule');

    expect(disabled?.rule.enabled).toBe(false);
    expect(disabled?.decision.action).toBe('no-action');
    expect(disabled?.alertIntent).toBeNull();
    expect(disabled?.auditRefs).toEqual(['tracking-expected-alert-proof-audit-expected-place-home-disabled-rule']);
    expect(disabled?.evidenceRefs[0]?.evidenceReferenceId).toBe(
      'tracking-expected-evidence-expected-place-home-disabled-rule'
    );
  });

  it('rejects alert-ready rows that do not carry a matching alert intent', () => {
    const alertReady = Proof.rows.find((row) => row.alertPolicyState === 'alert-ready');
    expect(alertReady).toBeDefined();
    const invalid = TrackingExpectedPlaceAlertRowSchema.safeParse({
      ...alertReady,
      alertIntent: null,
    });

    expect(invalid.success).toBe(false);
  });

  it('keeps provider, receipt, UI, child delivery, platform runtime, physical-device, and production claims false', () => {
    expect(Proof.alertReadyCount).toBe(1);
    expect(Proof.manualRequiredCount).toBe(1);
    expect(Proof.suppressedCount).toBe(1);
    expect(Proof.noAlertExpectedCount).toBe(1);
    expect(Proof.proofNonClaims).toEqual([
      'no-provider-delivery',
      'no-notification-receipt-ingestion',
      'no-parent-notification-ui',
      'no-child-device-delivery',
      'no-platform-adapter-runtime',
      'no-physical-device-proof',
      'no-production-worker',
    ]);
    expect(Proof.providerDeliveryClaimed).toBe(false);
    expect(Proof.notificationReceiptClaimed).toBe(false);
    expect(Proof.parentNotificationUiClaimed).toBe(false);
    expect(Proof.childDeviceDeliveryClaimed).toBe(false);
    expect(Proof.platformAdapterRuntimeClaimed).toBe(false);
    expect(Proof.physicalDeviceProofClaimed).toBe(false);
    expect(Proof.productionWorkerClaimed).toBe(false);
  });
});
