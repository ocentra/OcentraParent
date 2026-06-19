import { describe, expect, it } from 'vitest';
import {
  buildSocialAlertReportProviderReceiptIngestionReadinessReadModel,
  type SocialAlertReportProviderReceiptIngestionReadinessReadModel,
} from '../../src/social-alert-report-provider-receipt-ingestion-readiness';
import { buildSocialAlertReportProviderReceiptBoundaryReadModel } from '../../src/social-alert-report-provider-receipt-boundary-proof';
import { buildSocialAlertReportProviderPreflightReadModel } from '../../src/social-alert-report-provider-preflight-proof';
import { buildSocialAlertReportProviderStatusHandoffReadModel } from '../../src/social-alert-report-provider-status-handoff-proof';
import {
  buildSocialReportWriterDeliveryProofFromReceiptIngestionReadiness,
  SocialReportWriterDeliveryProofReadModel,
  SocialReportWriterDeliveryRowSchema,
  SocialReportWriterDeliveryState,
  SocialReportWriterReceiptState,
  summarizeSocialReportWriterDeliveryProof,
} from '../../src/social-report-writer-delivery-proof';
import {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
} from '../../src/social-alert-report-intent';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

const ReadyRow = SocialReportWriterDeliveryProofReadModel.reportWriterDeliveryRows[0];
const ManualRow = SocialReportWriterDeliveryProofReadModel.reportWriterDeliveryRows[1];
const Timestamp = '2026-06-08T06:20:00Z';
const MinimalPayloadFields = Object.values(SocialAlertReportPayloadField);

describe('social report writer delivery proof contracts', () => {
  it('accepts parent-owned report delivery ready rows without external runtime delivery claims', () => {
    const parsed = SocialReportWriterDeliveryRowSchema.parse(ReadyRow);
    const summary = summarizeSocialReportWriterDeliveryProof(SocialReportWriterDeliveryProofReadModel);

    expect(parsed.reportWriterDeliveryState).toBe(SocialReportWriterDeliveryState.ReportDeliveryReady);
    expect(parsed.reportWriterReceiptState).toBe(SocialReportWriterReceiptState.ParentOwnedReceiptRecorded);
    expect(parsed.parentOwnedReportArtifactWritten).toBe(true);
    expect(parsed.externalRuntimeReportDeliveryClaimed).toBe(false);
    expect(parsed.providerDeliveryAttempted).toBe(false);
    expect(summary).toEqual({
      totalRows: 2,
      reportDeliveryReadyRows: 1,
      manualRequiredRows: 1,
      unavailableRows: 0,
      externalRuntimeReportDeliveryClaimed: false,
      providerDeliveryAttempted: false,
      enforcementClaimed: false,
    });
  });

  it('keeps manual-required rows out of report artifact and receipt claims', () => {
    const parsed = SocialReportWriterDeliveryRowSchema.parse(ManualRow);

    expect(parsed.reportWriterDeliveryState).toBe(SocialReportWriterDeliveryState.ManualRequired);
    expect(parsed.reportArtifactRef).toBe(null);
    expect(parsed.reportReceiptRef).toBe(null);
    expect(parsed.manualProofRequirements).toEqual(['manual-proof-social-provider-report-runtime-required']);
  });

  it('rejects dishonest external delivery provider receipt raw content final policy and enforcement claims', () => {
    for (const invalidRow of [
      {
        ...ReadyRow,
        reportWriterDeliveryRowId: 'invalid-external-delivery',
        externalRuntimeReportDeliveryClaimed: true,
      },
      { ...ReadyRow, reportWriterDeliveryRowId: 'invalid-provider-delivery', providerDeliveryAttempted: true },
      { ...ReadyRow, reportWriterDeliveryRowId: 'invalid-provider-receipt', providerReceiptIngested: true },
      { ...ReadyRow, reportWriterDeliveryRowId: 'invalid-raw-account', rawAccountDataIncluded: true },
      { ...ReadyRow, reportWriterDeliveryRowId: 'invalid-raw-video', rawVideoContentIncluded: true },
      { ...ReadyRow, reportWriterDeliveryRowId: 'invalid-raw-message', rawMessageContentIncluded: true },
      { ...ReadyRow, reportWriterDeliveryRowId: 'invalid-screenshot', screenshotIncluded: true },
      { ...ReadyRow, reportWriterDeliveryRowId: 'invalid-final-policy', finalPolicyDecisionClaimed: true },
      { ...ReadyRow, reportWriterDeliveryRowId: 'invalid-enforcement', enforcementClaimed: true },
    ]) {
      expect(SocialReportWriterDeliveryRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });

  it('rejects ready rows without report artifacts and manual rows with false receipts', () => {
    const missingArtifact = SocialReportWriterDeliveryRowSchema.safeParse({
      ...ReadyRow,
      reportWriterDeliveryRowId: 'invalid-missing-report-artifact',
      reportArtifactRef: null,
    });
    const falseManualReceipt = SocialReportWriterDeliveryRowSchema.safeParse({
      ...ManualRow,
      reportWriterDeliveryRowId: 'invalid-manual-receipt',
      reportReceiptRef: 'false-manual-report-receipt',
      parentOwnedReportReceiptRecorded: true,
      reportWriterReceiptState: SocialReportWriterReceiptState.ParentOwnedReceiptRecorded,
    });

    expect(missingArtifact.success).toBe(false);
    expect(falseManualReceipt.success).toBe(false);
  });
});

describe('social report writer delivery from receipt ingestion readiness', () => {
  it('projects receipt ingestion readiness into manual or unavailable report-writer rows', () => {
    const readModel = buildReportWriterDeliveryFromReceiptIngestion();
    const summary = summarizeSocialReportWriterDeliveryProof(readModel);

    expect(summary).toEqual({
      totalRows: 3,
      reportDeliveryReadyRows: 0,
      manualRequiredRows: 2,
      unavailableRows: 1,
      externalRuntimeReportDeliveryClaimed: false,
      providerDeliveryAttempted: false,
      enforcementClaimed: false,
    });
    expect(readModel.reportWriterDeliveryRows.map((row) => row.reportWriterDeliveryState)).toEqual([
      SocialReportWriterDeliveryState.ManualRequired,
      SocialReportWriterDeliveryState.ManualRequired,
      SocialReportWriterDeliveryState.Unavailable,
    ]);
    expect(readModel.reportWriterDeliveryRows.every((row) => row.parentReportRef === null)).toBe(true);
    expect(readModel.reportWriterDeliveryRows.every((row) => row.reportArtifactRef === null)).toBe(true);
    expect(readModel.reportWriterDeliveryRows.every((row) => row.providerDeliveryAttempted === false)).toBe(true);
    expect(readModel.reportWriterDeliveryRows.every((row) => row.providerReceiptIngested === false)).toBe(true);
  });

  it('keeps receipt ingestion readiness rows from forging report artifacts', () => {
    const readModel = buildReportWriterDeliveryFromReceiptIngestion();
    const manualRow = readModel.reportWriterDeliveryRows[0];

    expect(
      SocialReportWriterDeliveryRowSchema.safeParse({
        ...manualRow,
        reportWriterDeliveryState: SocialReportWriterDeliveryState.ReportDeliveryReady,
        reportWriterReceiptState: SocialReportWriterReceiptState.ParentOwnedReceiptRecorded,
        parentReportRef: 'forged-parent-report',
        reportArtifactRef: 'forged-report-artifact',
        reportReceiptRef: 'forged-report-receipt',
        parentOwnedReportArtifactWritten: true,
        parentOwnedReportReceiptRecorded: true,
      }).success
    ).toBe(false);
  });
});

function buildReportWriterDeliveryFromReceiptIngestion() {
  return buildSocialReportWriterDeliveryProofFromReceiptIngestionReadiness(
    {
      generatedAt: Timestamp,
      proofId: 'social-report-writer-delivery-from-receipt-ingestion-proof',
      sourceAlertReportIntentProofRef: 'social-alert-report-provider-receipt-ingestion-readiness-proof',
    },
    buildReceiptIngestionReadinessReadModel()
  );
}

function buildReceiptIngestionReadinessReadModel(): SocialAlertReportProviderReceiptIngestionReadinessReadModel {
  const preflight = buildSocialAlertReportProviderPreflightReadModel(
    {
      generatedAt: Timestamp,
      providerPreflightId: 'social-report-writer-provider-preflight',
      sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-provider-status-handoff-proof'],
    },
    [baseIntent(), manualRequiredIntent(), unavailableIntent()]
  );
  const statusHandoff = buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt: Timestamp,
      handoffId: 'social-report-writer-provider-status-handoff',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    preflight
  );
  const receiptBoundary = buildSocialAlertReportProviderReceiptBoundaryReadModel(
    {
      generatedAt: Timestamp,
      receiptBoundaryId: 'social-report-writer-provider-receipt-boundary',
      sourceContractRefs: [
        'social-alert-report-provider-status-handoff-proof',
        'v0-8-notification-provider-status-boundary',
        'notifications-expectation-provider-boundary',
      ],
    },
    statusHandoff
  );

  return buildSocialAlertReportProviderReceiptIngestionReadinessReadModel(
    {
      generatedAt: Timestamp,
      readinessId: 'social-report-writer-provider-receipt-ingestion-readiness',
      sourceContractRefs: [
        'social-alert-report-provider-receipt-boundary-proof',
        'provider-receipt-webhook-contract',
        'provider-receipt-durable-store-contract',
      ],
    },
    receiptBoundary
  );
}

function baseIntent() {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    alertReportIntentId: 'social-report-writer-high-risk',
    intentKind: SocialAlertReportIntentKind.HighRiskSignal,
    intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-report-writer',
      childProfileId: 'child-social-report-writer',
      label: 'Study Phone',
      platform: ParentPlatform.Android,
    },
    notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
    providerChannelPreference: 'in-app',
    parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
    parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-report-writer',
    explanationEventRefs: ['social-explanation-event-report-writer'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-report-writer',
        kind: ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: Timestamp,
      },
    ],
    policyRefs: ['policy-ref-social-report-writer'],
    auditRefs: ['audit-ref-social-report-writer'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-social-report-writer',
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: [],
    minimalPayloadFields: MinimalPayloadFields,
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
  } as const;
}

function manualRequiredIntent() {
  return {
    ...baseIntent(),
    alertReportIntentId: 'social-report-writer-manual-required',
    intentKind: SocialAlertReportIntentKind.ManualRequired,
    intentStatus: SocialAlertReportIntentStatus.ManualRequired,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: SocialAlertReportReasonCode.ManualRequired,
    parentTitleToken: SocialAlertReportParentCopyToken.ManualRequiredTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ManualRequiredBody,
    parentActionToken: SocialAlertReportParentCopyToken.ReviewManuallyAction,
    localOutboxRecordRef: null,
    deliveryClaimState: SocialAlertReportDeliveryClaimState.ManualRequired,
    manualProofRequirements: ['manual-proof-social-report-writer-required'],
  } as const;
}

function unavailableIntent() {
  return {
    ...manualRequiredIntent(),
    alertReportIntentId: 'social-report-writer-unavailable',
    intentKind: SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-report-writer-unavailable'],
  } as const;
}
