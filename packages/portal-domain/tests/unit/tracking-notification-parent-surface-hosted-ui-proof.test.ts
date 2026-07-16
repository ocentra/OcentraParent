import { describe, expect, it } from 'vitest';
import { GeneratedPortalTrackingContracts } from '../../src/generated-portal-contracts';
import {
  trackingNotificationParentSurfaceHostedUiProof,
  trackingNotificationParentSurfaceHostedUiProofFromReadModel,
} from '../../src/tracking-notification-parent-surface-hosted-ui-proof';

describe('tracking notification parent-surface hosted ui proof', () => {
  it('maps schema-backed history rows into portal proof rows', () => {
    const proof = trackingNotificationParentSurfaceHostedUiProofFromReadModel(customHistoryReadModel());

    expect(proof.rowsReturned).toBe('3');
    expect(proof.renderedParentNotificationUiRows).toBe('3');
    expect(proof.parentPreferenceMutationRows).toBe('0');
    expect(proof.rows).toEqual([
      {
        title: 'Notification history ready',
        status: 'history-intent-ready',
        policyDecisionRef: 'policy-ready',
        evidenceRefs: 'evidence-ready',
        providerAttemptRef: 'provider-attempt-ready',
        receiptRequirementRefs: 'receipt-ready',
        preferenceRequirementRefs: 'preference-ready',
        manualProofRequirements: 'manual-proof-ready-a | manual-proof-ready-b',
        redactedSummaryRef: 'summary-ready',
      },
      {
        title: 'Manual notification action required',
        status: 'manual-action-required',
        policyDecisionRef: 'policy-manual',
        evidenceRefs: 'evidence-manual',
        providerAttemptRef: 'provider-attempt-manual',
        receiptRequirementRefs: 'receipt-manual',
        preferenceRequirementRefs: 'quiet-hours-manual',
        manualProofRequirements: 'manual-proof-manual',
        redactedSummaryRef: 'summary-manual',
      },
      {
        title: 'Notification provider unavailable',
        status: 'provider-unavailable',
        policyDecisionRef: 'policy-unavailable',
        evidenceRefs: 'evidence-unavailable',
        providerAttemptRef: 'provider-attempt-unavailable',
        receiptRequirementRefs: 'receipt-unavailable',
        preferenceRequirementRefs: 'preference-unavailable',
        manualProofRequirements: 'manual-proof-unavailable',
        redactedSummaryRef: 'summary-unavailable',
      },
    ]);
  });

  it('keeps invalid input explicit instead of inventing rows', () => {
    const proof = trackingNotificationParentSurfaceHostedUiProofFromReadModel({ rows: [] });

    expect(proof.rowsReturned).toBe('0');
    expect(proof.renderedParentNotificationUiRows).toBe('0');
    expect(proof.rows).toEqual([]);
    expect(proof.productClaim).toBe('No product claim');
  });

  it('keeps the public no-arg portal proof wrapper stable', () => {
    const proof = trackingNotificationParentSurfaceHostedUiProof();

    expect(proof.rowsReturned).toBe('3');
    expect(proof.rows.map((row) => row.title)).toEqual([
      'Notification history ready',
      'Manual notification action required',
      'Notification provider unavailable',
    ]);
  });
});

function customHistoryReadModel() {
  return {
    schemaVersion: 'v0.6',
    proofId: 'tracking-notification-parent-surface-history-proof-custom',
    generatedAt: '2026-06-14T00:40:00.000Z',
    family: {
      familyId: 'family-tracking-notification-history-custom',
    },
    sourceProviderNotificationProofRef: 'provider-proof-custom',
    sourceReceiptBoundaryProofRef: 'receipt-proof-custom',
    sourcePreferencePreflightProofRef: 'preference-proof-custom',
    sourceContractRefs: ['tracking-provider-notification-proof', 'tracking-notification-parent-surface-history-proof'],
    rows: [
      historyRow('ready', {
        status: 'history-intent-ready',
        sourcePolicyDecisionId: 'policy-ready',
        evidenceRefs: ['evidence-ready'],
        providerAttemptRef: 'provider-attempt-ready',
        receiptRequirementRefs: ['receipt-ready'],
        parentPreferenceRequirementRefs: ['preference-ready'],
        quietHoursRequirementRefs: ['quiet-hours-ready'],
        manualProofRequirements: ['manual-proof-ready-a', 'manual-proof-ready-b'],
        redactedParentSummaryRef: 'summary-ready',
      }),
      historyRow('manual', {
        status: 'manual-action-required',
        sourcePolicyDecisionId: 'policy-manual',
        evidenceRefs: ['evidence-manual'],
        providerAttemptRef: 'provider-attempt-manual',
        receiptRequirementRefs: ['receipt-manual'],
        parentPreferenceRequirementRefs: ['preference-manual'],
        quietHoursRequirementRefs: ['quiet-hours-manual'],
        manualProofRequirements: ['manual-proof-manual'],
        redactedParentSummaryRef: 'summary-manual',
      }),
      historyRow('unavailable', {
        status: 'provider-unavailable',
        sourcePolicyDecisionId: 'policy-unavailable',
        evidenceRefs: ['evidence-unavailable'],
        providerAttemptRef: 'provider-attempt-unavailable',
        receiptRequirementRefs: ['receipt-unavailable'],
        parentPreferenceRequirementRefs: ['preference-unavailable'],
        quietHoursRequirementRefs: [],
        manualProofRequirements: ['manual-proof-unavailable'],
        redactedParentSummaryRef: 'summary-unavailable',
      }),
    ],
    historyIntentReadyCount: 1,
    manualActionRequiredCount: 1,
    providerUnavailableCount: 1,
    proofNonClaims: GeneratedPortalTrackingContracts.NotificationParentSurfaceHistory.RequiredNonClaims,
    renderedParentNotificationUiClaimed: false,
    parentPreferenceMutationRuntimeClaimed: false,
    parentFrequencyControlUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableHistoryStorageClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  } as const;
}

function historyRow(
  suffix: string,
  input: {
    readonly status: 'history-intent-ready' | 'manual-action-required' | 'provider-unavailable';
    readonly sourcePolicyDecisionId: string;
    readonly evidenceRefs: readonly string[];
    readonly providerAttemptRef: string;
    readonly receiptRequirementRefs: readonly string[];
    readonly parentPreferenceRequirementRefs: readonly string[];
    readonly quietHoursRequirementRefs: readonly string[];
    readonly manualProofRequirements: readonly string[];
    readonly redactedParentSummaryRef: string;
  }
) {
  return {
    historyRowId: `history-row-${suffix}`,
    sourceAlertId: `alert-${suffix}`,
    sourceProviderNotificationRowId: `provider-row-${suffix}`,
    sourceReceiptBoundaryRowId: `receipt-row-${suffix}`,
    sourcePreferencePreflightRowId: `preference-row-${suffix}`,
    status: input.status,
    sourcePolicyDecisionId: input.sourcePolicyDecisionId,
    evidenceRefs: input.evidenceRefs,
    notificationStatusRefs: [`notification-status-${suffix}`],
    reasonCodeRefs: [`reason-${suffix}`],
    providerStatusEntryRef: `provider-status-entry-${suffix}`,
    providerAttemptRef: input.providerAttemptRef,
    auditRefs: [`audit-${suffix}`],
    providerPreferenceRefs: [`provider-preference-${suffix}`],
    parentPreferenceRequirementRefs: input.parentPreferenceRequirementRefs,
    quietHoursRequirementRefs: input.quietHoursRequirementRefs,
    receiptRequirementRefs: input.receiptRequirementRefs,
    manualProofRequirements: input.manualProofRequirements,
    drillInRefs: [`drill-in-${suffix}`],
    redactedParentSummaryRef: input.redactedParentSummaryRef,
    renderedParentNotificationUiClaimed: false,
    parentPreferenceMutationRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    receiptIngestionRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
  } as const;
}
