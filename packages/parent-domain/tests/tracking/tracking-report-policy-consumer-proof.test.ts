import { describe, expect, it } from 'vitest';
import { TrackingPolicySchemaVersion } from '../../src/tracking-location-policy';
import {
  TrackingReportPolicyConsumerRowSchema,
  buildTrackingReportPolicyConsumerProof,
  type TrackingReportPolicyConsumerProof,
  type TrackingReportPolicyConsumerRow,
} from '../../src/tracking-report-policy-consumer-proof';

const GeneratedAt = '2026-06-05T20:25:00.000Z';

describe('tracking report policy consumer proof', () => {
  it('builds report, policy drill-in, and retention consumer rows without product overclaims', () => {
    const proof = buildTrackingReportPolicyConsumerProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-report-policy-consumer-proof');
    expect(proof.rows.map((row) => row.consumerKind)).toEqual([
      'parent-report-summary',
      'policy-evidence-drill-in',
      'retention-audit-export',
    ]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    for (const row of proof.rows) {
      expect(row.readinessState).toBe('consumer-ready');
      expect(row.requiredProofTier).toBe('P2_HOSTED_CI');
      expect(row.currentProofTier).toBe('P2_HOSTED_CI');
      expect(row.sourceProofRefs.length).toBeGreaterThan(0);
      expect(row.productSurfaceSummaryRefs).toContain(
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json'
      );
      expect(row.evidenceReferences.length).toBeGreaterThan(0);
      expect(row.storedJournalRefs.length).toBeGreaterThan(0);
      expect(row.storedReadModelRowRefs.length).toBeGreaterThan(0);
      expect(row.reportConsumerClaimed).toBe(true);
      expect(row.portalUiClaimed).toBe(false);
      expect(row.childDeviceDeliveryClaimed).toBe(false);
      expect(row.providerDeliveryClaimed).toBe(false);
      expect(row.notificationReceiptClaimed).toBe(false);
      expect(row.physicalDeviceClaimed).toBe(false);
      expect(row.authorityClaimed).toBe(false);
      expect(row.productClaimReady).toBe(false);
    }
  });

  it('keeps the policy drill-in row tied to a parsed policy decision and evidence references', () => {
    const proof = buildTrackingReportPolicyConsumerProof(GeneratedAt);
    const policyRow = rowFor(proof, 'policy-evidence-drill-in');

    expect(policyRow.policyDecision?.schemaVersion).toBe(TrackingPolicySchemaVersion);
    expect(policyRow.policyDecision?.action).toBe('notify-parent');
    expect(policyRow.policyDecision?.evidenceReferences).toEqual(policyRow.evidenceReferences);
    expect(policyRow.storedJournalRefs).toContain('tracking-journal-row-policy-drill-in');
    expect(policyRow.storedReadModelRowRefs).toContain('tracking-read-model-row-policy-drill-in');
    expect(policyRow.reasonCodes).toContain('tracking-policy-decision-drill-in-consumed');
    expect(policyRow.auditRefs).toContain('tracking-report-policy-audit-policy-drill-in');
  });

  it('rejects policy drill-in rows that omit the policy decision', () => {
    const policyRow = rowFor(buildTrackingReportPolicyConsumerProof(GeneratedAt), 'policy-evidence-drill-in');
    const parsed = TrackingReportPolicyConsumerRowSchema.safeParse({
      ...policyRow,
      policyDecision: null,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects consumer rows without source, summary, or evidence references', () => {
    const reportRow = rowFor(buildTrackingReportPolicyConsumerProof(GeneratedAt), 'parent-report-summary');

    expect(TrackingReportPolicyConsumerRowSchema.safeParse({ ...reportRow, sourceProofRefs: [] }).success).toBe(false);
    expect(
      TrackingReportPolicyConsumerRowSchema.safeParse({ ...reportRow, productSurfaceSummaryRefs: [] }).success
    ).toBe(false);
    expect(TrackingReportPolicyConsumerRowSchema.safeParse({ ...reportRow, evidenceReferences: [] }).success).toBe(
      false
    );
    expect(TrackingReportPolicyConsumerRowSchema.safeParse({ ...reportRow, storedJournalRefs: [] }).success).toBe(
      false
    );
    expect(TrackingReportPolicyConsumerRowSchema.safeParse({ ...reportRow, storedReadModelRowRefs: [] }).success).toBe(
      false
    );
  });
});

function rowFor(
  proof: TrackingReportPolicyConsumerProof,
  consumerKind: TrackingReportPolicyConsumerRow['consumerKind']
): TrackingReportPolicyConsumerRow {
  const row = proof.rows.find((entry) => entry.consumerKind === consumerKind);
  if (row === undefined) {
    throw new Error(`Missing tracking report policy consumer row: ${consumerKind}`);
  }
  return row;
}
