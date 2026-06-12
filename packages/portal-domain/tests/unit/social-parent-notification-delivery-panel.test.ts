import { describe, expect, it } from 'vitest';
import { createSocialParentNotificationDeliveryPanelIntent } from '../../src/contracts';

describe('social parent notification delivery panel intent', () => {
  it('renders service readiness rows without making delivery claims', () => {
    const intent = createSocialParentNotificationDeliveryPanelIntent(snapshot());

    expect(intent.summary).toBe('2 parent notification readiness rows');
    expect(intent.rows).toHaveLength(2);
    expect(intent.rows[0]?.title).toBe('Parent report status ready');
    expect(intent.rows[1]?.title).toBe('Parent notification manual proof required');
    expect(intent.productClaim).toContain('Parent report readiness projection only');
    expect(JSON.stringify(intent)).not.toMatch(/delivered|enforcement active|provider delivery complete/iu);
  });

  it('keeps an empty not-reported state for invalid snapshots', () => {
    const intent = createSocialParentNotificationDeliveryPanelIntent({ rows: [] });

    expect(intent.summary).toBe('0 parent notification readiness rows');
    expect(intent.rows).toHaveLength(0);
    expect(intent.details.some((detail) => detail.value === 'not reported')).toBe(true);
  });
});

function snapshot() {
  return {
    schemaVersion: 'social-parent-notification-delivery-read-model',
    readinessId: 'social-parent-notification-delivery-readiness-service',
    generatedAt: '2026-06-08T11:45:00Z',
    sourceReportWriterProofRef: 'social-report-writer-delivery-proof-service',
    rows: [
      row(
        'social-parent-notification-ready-high-risk-service',
        'parent-report-status-ready',
        'parent-owned-report-ready',
        []
      ),
      row('social-parent-notification-manual-required-service', 'manual-required', 'manual-required', [
        'manual-parent-notification-ui-runtime-proof-required',
      ]),
    ],
    nonClaims: [
      'no-parent-notification-ui-delivery',
      'no-external-runtime-report-delivery',
      'no-provider-delivery',
      'no-provider-receipt-ingestion',
      'no-final-policy-execution',
      'no-enforcement',
    ],
    parentReportStatusReadyCount: 1,
    manualRequiredCount: 1,
    unavailableCount: 0,
    parentNotificationUiDeliveryClaimed: false,
    externalRuntimeReportDeliveryClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  };
}

function row(
  id: string,
  notificationDeliveryReadinessState: 'parent-report-status-ready' | 'manual-required',
  reportDeliveryExecutionState: 'parent-owned-report-ready' | 'manual-required',
  manualProofRequirements: readonly string[]
) {
  return {
    notificationDeliveryReadinessRowId: id,
    sourceReportWriterDeliveryRowRef: 'social-report-writer-delivery-row-service',
    sourceIntentRef: 'social-alert-report-high-risk-service',
    parentVisibleReportStatusRef: 'social-parent-visible-report-status-high-risk-service',
    parentNotificationUiRef: null,
    parentReportRef:
      notificationDeliveryReadinessState === 'parent-report-status-ready'
        ? 'social-parent-report-high-risk-service'
        : null,
    reportArtifactRef:
      notificationDeliveryReadinessState === 'parent-report-status-ready'
        ? 'social-report-artifact-high-risk-service'
        : null,
    reportReceiptRef:
      notificationDeliveryReadinessState === 'parent-report-status-ready'
        ? 'social-report-receipt-high-risk-service'
        : null,
    sourceEvidenceRefs: ['evidence-social-route-gate'],
    sourcePolicyRefs: ['policy-ref-social-high-risk'],
    sourceAuditRefs: ['audit-ref-social-alert-report'],
    manualProofRequirements,
    notificationDeliveryReadinessState,
    reportDeliveryExecutionState,
    parentOwnedReportArtifactWritten: notificationDeliveryReadinessState === 'parent-report-status-ready',
    parentOwnedReportReceiptRecorded: notificationDeliveryReadinessState === 'parent-report-status-ready',
    parentNotificationUiDelivered: false,
    externalRuntimeReportDeliveryClaimed: false,
    providerDeliveryAttempted: false,
    providerReceiptIngested: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    createdAt: '2026-06-08T11:45:00Z',
  };
}
