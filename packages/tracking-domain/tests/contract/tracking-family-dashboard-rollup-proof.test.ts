import { describe, expect, it } from 'vitest';
import {
  TrackingFamilyDashboardRollupRowSchema,
  buildTrackingFamilyDashboardRollupProof,
  type TrackingFamilyDashboardRollupProof,
  type TrackingFamilyDashboardRollupRow,
} from '../../src/tracking-family-dashboard-rollup-proof';

const GeneratedAt = '2026-06-05T23:10:00.000Z';

describe('tracking family dashboard rollup proof', () => {
  it('builds family dashboard rollup rows from existing service read-model proof refs', () => {
    const proof = buildTrackingFamilyDashboardRollupProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-family-dashboard-rollup-proof');
    expect(proof.rows.map((row) => row.rollupKind)).toEqual([
      'family-active-summary',
      'child-attention-summary',
      'retention-audit-summary',
    ]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    for (const row of proof.rows) {
      expect(row.rollupState).toBe('rollup-ready');
      expect(row.requiredProofTier).toBe('P2_HOSTED_CI');
      expect(row.currentProofTier).toBe('P2_HOSTED_CI');
      expect(row.sourceProofRefs).toContain(
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json'
      );
      expect(row.productSurfaceSummaryRefs).toContain(
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json'
      );
      expect(row.reportConsumerRefs).toContain(
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json'
      );
      expect(row.evidenceReferences.length).toBeGreaterThan(0);
      expect(row.dashboardRollupClaimed).toBe(true);
      expect(row.portalUiClaimed).toBe(false);
      expect(row.childDeviceDeliveryClaimed).toBe(false);
      expect(row.providerDeliveryClaimed).toBe(false);
      expect(row.notificationReceiptClaimed).toBe(false);
      expect(row.physicalDeviceClaimed).toBe(false);
      expect(row.authorityClaimed).toBe(false);
      expect(row.productClaimReady).toBe(false);
    }
  });

  it('keeps the child attention rollup visibly attention-bearing', () => {
    const attentionRow = rowFor(buildTrackingFamilyDashboardRollupProof(GeneratedAt), 'child-attention-summary');

    expect(attentionRow.attentionItemCount).toBeGreaterThan(0);
    expect(attentionRow.severity).toBe('attention');
    expect(attentionRow.reasonCodes).toContain('tracking-family-dashboard-child-attention-ready');
    expect(attentionRow.auditRefs).toContain('tracking-family-dashboard-audit-child-attention');
  });

  it('rejects rollups without proof refs or evidence refs', () => {
    const activeRow = rowFor(buildTrackingFamilyDashboardRollupProof(GeneratedAt), 'family-active-summary');

    expect(TrackingFamilyDashboardRollupRowSchema.safeParse({ ...activeRow, sourceProofRefs: [] }).success).toBe(false);
    expect(
      TrackingFamilyDashboardRollupRowSchema.safeParse({ ...activeRow, productSurfaceSummaryRefs: [] }).success
    ).toBe(false);
    expect(TrackingFamilyDashboardRollupRowSchema.safeParse({ ...activeRow, evidenceReferences: [] }).success).toBe(
      false
    );
  });

  it('rejects child attention rollups that hide the attention count', () => {
    const attentionRow = rowFor(buildTrackingFamilyDashboardRollupProof(GeneratedAt), 'child-attention-summary');

    expect(
      TrackingFamilyDashboardRollupRowSchema.safeParse({
        ...attentionRow,
        attentionItemCount: 0,
      }).success
    ).toBe(false);
  });
});

function rowFor(
  proof: TrackingFamilyDashboardRollupProof,
  rollupKind: TrackingFamilyDashboardRollupRow['rollupKind']
): TrackingFamilyDashboardRollupRow {
  const row = proof.rows.find((entry) => entry.rollupKind === rollupKind);
  if (row === undefined) {
    throw new Error(`Missing tracking family dashboard rollup row: ${rollupKind}`);
  }
  return row;
}
