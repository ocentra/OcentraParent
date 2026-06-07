import { describe, expect, it } from 'vitest';
import {
  SocialReportWriterDeliveryProofReadModel,
  SocialReportWriterDeliveryRowSchema,
  SocialReportWriterDeliveryState,
  SocialReportWriterReceiptState,
  summarizeSocialReportWriterDeliveryProof,
} from '../src/social-report-writer-delivery-proof';

const ReadyRow = SocialReportWriterDeliveryProofReadModel.reportWriterDeliveryRows[0];
const ManualRow = SocialReportWriterDeliveryProofReadModel.reportWriterDeliveryRows[1];

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
