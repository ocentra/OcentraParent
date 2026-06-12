import { describe, expect, it } from 'vitest';
import {
  buildSocialParentNotificationDeliveryReadinessReadModel,
  SocialParentNotificationDeliveryReadinessRowSchema,
  SocialParentNotificationDeliveryReadinessState,
  summarizeSocialParentNotificationDeliveryReadiness,
} from '../../src/social-parent-notification-delivery-readiness';
import {
  buildSocialReportWriterDeliveryProofFromReceiptIngestionReadiness,
  SocialReportWriterDeliveryProofReadModel,
} from '../../src/social-report-writer-delivery-proof';
import {
  RequiredSocialAlertReportProviderReceiptIngestionReadinessNonClaims,
  SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema,
} from '../../src/social-alert-report-provider-receipt-ingestion-readiness';
import { RequiredSocialAlertReportProviderReceiptBoundaryNonClaims } from '../../src/social-alert-report-provider-receipt-boundary-proof';

const Timestamp = '2026-06-08T09:55:00Z';

describe('social parent notification delivery readiness from report writer rows', () => {
  it('projects parent-owned report writer rows into parent-visible status readiness without UI delivery claims', () => {
    const readModel = buildSocialParentNotificationDeliveryReadinessReadModel(
      {
        generatedAt: Timestamp,
        readinessId: 'social-parent-notification-delivery-readiness-proof',
        sourceReportWriterProofRef: 'social-report-writer-delivery-proof',
      },
      SocialReportWriterDeliveryProofReadModel
    );
    const summary = summarizeSocialParentNotificationDeliveryReadiness(readModel);
    const readyRow = readModel.rows.find(
      (row) =>
        row.notificationDeliveryReadinessState ===
        SocialParentNotificationDeliveryReadinessState.ParentReportStatusReady
    );

    expect(summary).toEqual({
      totalRows: 2,
      parentReportStatusReadyCount: 1,
      manualRequiredCount: 1,
      unavailableCount: 0,
      parentLocalDeliveryResultCount: 1,
      parentNotificationUiDeliveryClaimed: false,
      externalRuntimeReportDeliveryClaimed: false,
      finalPolicyExecutionClaimed: false,
      enforcementClaimed: false,
    });
    expect(readyRow?.parentVisibleReportStatusRef).toBe('parent-visible-social-weekly-report-status');
    expect(readyRow?.parentNotificationUiRef).toBe(null);
    expect(readyRow?.parentLocalDeliveryResultRef).toBe(
      'social-parent-local-delivery-result-social-report-delivery-weekly-summary'
    );
    expect(readyRow?.parentLocalDeliveryResultRecorded).toBe(true);
    expect(readyRow?.parentNotificationUiDelivered).toBe(false);
  });

  it('projects receipt-ingestion report-writer rows into manual-required and unavailable notification readiness', () => {
    const reportWriterReadModel = buildSocialReportWriterDeliveryProofFromReceiptIngestionReadiness(
      {
        generatedAt: Timestamp,
        proofId: 'social-report-writer-delivery-from-receipt-ingestion-proof',
        sourceAlertReportIntentProofRef: 'social-provider-receipt-ingestion-readiness-proof',
      },
      receiptIngestionReadinessReadModel()
    );
    const readModel = buildSocialParentNotificationDeliveryReadinessReadModel(
      {
        generatedAt: Timestamp,
        readinessId: 'social-parent-notification-delivery-from-receipt-ingestion-proof',
        sourceReportWriterProofRef: 'social-report-writer-delivery-from-receipt-ingestion-proof',
      },
      reportWriterReadModel
    );
    const summary = summarizeSocialParentNotificationDeliveryReadiness(readModel);

    expect(summary).toEqual({
      totalRows: 3,
      parentReportStatusReadyCount: 0,
      manualRequiredCount: 2,
      unavailableCount: 1,
      parentLocalDeliveryResultCount: 0,
      parentNotificationUiDeliveryClaimed: false,
      externalRuntimeReportDeliveryClaimed: false,
      finalPolicyExecutionClaimed: false,
      enforcementClaimed: false,
    });
    expect(readModel.rows.every((row) => row.parentNotificationUiRef === null)).toBe(true);
    expect(readModel.rows.every((row) => row.parentLocalDeliveryResultRef === null)).toBe(true);
    expect(readModel.rows.every((row) => row.parentLocalDeliveryResultRecorded === false)).toBe(true);
    expect(readModel.rows.every((row) => row.parentNotificationUiDelivered === false)).toBe(true);
    expect(readModel.rows.every((row) => row.providerReceiptIngested === false)).toBe(true);
  });
});

describe('social parent notification delivery readiness dishonest claim rejection', () => {
  it('rejects forged parent UI delivery external report delivery final policy and enforcement claims', () => {
    const row = buildSocialParentNotificationDeliveryReadinessReadModel(
      {
        generatedAt: Timestamp,
        readinessId: 'social-parent-notification-delivery-readiness-proof',
        sourceReportWriterProofRef: 'social-report-writer-delivery-proof',
      },
      SocialReportWriterDeliveryProofReadModel
    ).rows[0];

    for (const invalidRow of [
      { ...row, notificationDeliveryReadinessRowId: 'invalid-ui-ref', parentNotificationUiRef: 'forged-ui-ref' },
      { ...row, notificationDeliveryReadinessRowId: 'invalid-ui-delivered', parentNotificationUiDelivered: true },
      {
        ...row,
        notificationDeliveryReadinessRowId: 'invalid-missing-local-delivery-result',
        parentLocalDeliveryResultRef: null,
      },
      {
        ...row,
        notificationDeliveryReadinessRowId: 'invalid-missing-local-delivery-recorded',
        parentLocalDeliveryResultRecorded: false,
      },
      {
        ...row,
        notificationDeliveryReadinessRowId: 'invalid-external-runtime-report',
        externalRuntimeReportDeliveryClaimed: true,
      },
      { ...row, notificationDeliveryReadinessRowId: 'invalid-provider-delivery', providerDeliveryAttempted: true },
      { ...row, notificationDeliveryReadinessRowId: 'invalid-provider-receipt', providerReceiptIngested: true },
      { ...row, notificationDeliveryReadinessRowId: 'invalid-final-policy', finalPolicyDecisionClaimed: true },
      { ...row, notificationDeliveryReadinessRowId: 'invalid-enforcement', enforcementClaimed: true },
    ]) {
      expect(SocialParentNotificationDeliveryReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });

  it('rejects forged local delivery results on rows that still need manual proof', () => {
    const manualRow = buildSocialParentNotificationDeliveryReadinessReadModel(
      {
        generatedAt: Timestamp,
        readinessId: 'social-parent-notification-delivery-from-receipt-ingestion-proof',
        sourceReportWriterProofRef: 'social-report-writer-delivery-from-receipt-ingestion-proof',
      },
      buildSocialReportWriterDeliveryProofFromReceiptIngestionReadiness(
        {
          generatedAt: Timestamp,
          proofId: 'social-report-writer-delivery-from-receipt-ingestion-proof',
          sourceAlertReportIntentProofRef: 'social-provider-receipt-ingestion-readiness-proof',
        },
        receiptIngestionReadinessReadModel()
      )
    ).rows[0];

    expect(manualRow?.notificationDeliveryReadinessState).toBe(
      SocialParentNotificationDeliveryReadinessState.ManualRequired
    );
    expect(
      SocialParentNotificationDeliveryReadinessRowSchema.safeParse({
        ...manualRow,
        parentLocalDeliveryResultRef: 'forged-local-delivery-result',
      }).success
    ).toBe(false);
    expect(
      SocialParentNotificationDeliveryReadinessRowSchema.safeParse({
        ...manualRow,
        parentLocalDeliveryResultRecorded: true,
      }).success
    ).toBe(false);
  });
});

function receiptIngestionReadinessReadModel() {
  return SocialAlertReportProviderReceiptIngestionReadinessReadModelSchema.parse({
    schemaVersion: SocialReportWriterDeliveryProofReadModel.schemaVersion,
    readinessId: 'social-parent-notification-provider-receipt-ingestion-readiness',
    generatedAt: Timestamp,
    sourceReceiptBoundaryId: 'social-parent-notification-provider-receipt-boundary',
    sourceContractRefs: ['social-alert-report-provider-receipt-boundary-proof'],
    sourceReceiptBoundaryNonClaims: RequiredSocialAlertReportProviderReceiptBoundaryNonClaims,
    rows: [
      receiptIngestionRow('social-parent-notification-high-risk', 'provider-dispatch-required'),
      receiptIngestionRow('social-parent-notification-manual-required', 'manual-receipt-required'),
      receiptIngestionRow('social-parent-notification-unavailable', 'provider-unavailable'),
    ],
    ingestionContractRequiredCount: 1,
    manualReceiptRequiredCount: 1,
    providerUnavailableCount: 1,
    providerReceiptObservedCount: 0,
    receiptIngestionReadinessNonClaims: RequiredSocialAlertReportProviderReceiptIngestionReadinessNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerWebhookRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    providerReceiptObservedClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

function receiptIngestionRow(
  sourceIntentRef: string,
  sourceReceiptBoundaryState: 'provider-dispatch-required' | 'manual-receipt-required' | 'provider-unavailable'
) {
  const ingestionReadinessState =
    sourceReceiptBoundaryState === 'provider-dispatch-required'
      ? 'ingestion-contract-required'
      : sourceReceiptBoundaryState;

  return {
    ingestionRowId: `social-provider-receipt-ingestion-${sourceIntentRef}`,
    sourceReceiptRowRef: `social-provider-receipt-row-${sourceIntentRef}`,
    sourceIntentRef,
    sourceProviderAttemptRef: `social-provider-attempt-${sourceIntentRef}`,
    sourceReceiptBoundaryState,
    ingestionReadinessState,
    webhookEndpointRef: null,
    providerCredentialRef: null,
    durableReceiptResultRef: null,
    providerReceiptObservedRefs: [],
    receiptProofRequirements: [`social-provider-receipt-proof-required-${sourceIntentRef}`],
    ingestionProofRequirements: [`social-provider-receipt-ingestion-proof-required-${sourceIntentRef}`],
    providerDeliveryExecutionClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerWebhookRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    providerReceiptObservedClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  } as const;
}
