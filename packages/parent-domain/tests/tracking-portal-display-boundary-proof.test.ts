import { describe, expect, it } from 'vitest';
import {
  TrackingPortalDisplayBoundaryRowSchema,
  buildTrackingPortalDisplayBoundaryProof,
  type TrackingPortalDisplayBoundaryKind,
  type TrackingPortalDisplayBoundaryProof,
  type TrackingPortalDisplayBoundaryRow,
} from '../src/tracking-portal-display-boundary-proof';

const GeneratedAt = '2026-06-06T06:38:00.000Z';

describe('tracking portal display boundary proof', () => {
  it('builds display and authoring boundary rows from existing tracking proof refs', () => {
    const proof = buildTrackingPortalDisplayBoundaryProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-portal-display-boundary-proof');
    expect(proof.rows.map((row) => row.boundaryKind)).toEqual([
      'service-read-model-display',
      'retention-settings-display',
      'family-dashboard-rollup-display',
      'unsupported-platform-manual-display',
    ]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    for (const row of proof.rows) {
      expect(row.requiredProofTier).toBe('P2_HOSTED_CI');
      expect(row.currentProofTier).toBe('P2_HOSTED_CI');
      expect(row.sourceProofRefs).toContain(
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json'
      );
      expect(row.hostedProofRefs).toContain('test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json');
      expect(row.evidenceReferences.length).toBe(1);
      expect(row.visibleStatusCount).toBeGreaterThan(0);
      expect(row.portalDisplayClaimed).toBe(true);
      expect(row.portalEvaluatorClaimed).toBe(false);
      expect(row.policyEvaluationClaimed).toBe(false);
      expect(row.serviceMutationClaimed).toBe(false);
      expect(row.platformWriterExecutionClaimed).toBe(false);
      expect(row.childRuntimeExecutionClaimed).toBe(false);
      expect(row.providerDeliveryClaimed).toBe(false);
      expect(row.notificationReceiptClaimed).toBe(false);
      expect(row.physicalDeviceClaimed).toBe(false);
      expect(row.authorityClaimed).toBe(false);
      expect(row.productClaimReady).toBe(false);
    }
  });

  it('keeps retention settings authoring separate from service mutation', () => {
    const retentionRow = rowFor(buildTrackingPortalDisplayBoundaryProof(GeneratedAt), 'retention-settings-display');

    expect(retentionRow.boundaryState).toBe('authoring-ready');
    expect(retentionRow.editableSettingCount).toBe(5);
    expect(retentionRow.portalAuthoringClaimed).toBe(true);
    expect(retentionRow.reasonCodes).toContain('tracking-portal-retention-settings-authoring-ready');
    expect(retentionRow.serviceMutationClaimed).toBe(false);
    expect(retentionRow.platformWriterExecutionClaimed).toBe(false);
  });

  it('rejects display rows without proof refs or evidence refs', () => {
    const serviceRow = rowFor(buildTrackingPortalDisplayBoundaryProof(GeneratedAt), 'service-read-model-display');

    expect(TrackingPortalDisplayBoundaryRowSchema.safeParse({ ...serviceRow, sourceProofRefs: [] }).success).toBe(
      false
    );
    expect(TrackingPortalDisplayBoundaryRowSchema.safeParse({ ...serviceRow, hostedProofRefs: [] }).success).toBe(
      false
    );
    expect(TrackingPortalDisplayBoundaryRowSchema.safeParse({ ...serviceRow, evidenceReferences: [] }).success).toBe(
      false
    );
  });

  it('rejects evaluator, mutation, platform writer, and product-ready overclaims', () => {
    const retentionRow = rowFor(buildTrackingPortalDisplayBoundaryProof(GeneratedAt), 'retention-settings-display');

    expect(
      TrackingPortalDisplayBoundaryRowSchema.safeParse({ ...retentionRow, portalEvaluatorClaimed: true }).success
    ).toBe(false);
    expect(
      TrackingPortalDisplayBoundaryRowSchema.safeParse({ ...retentionRow, serviceMutationClaimed: true }).success
    ).toBe(false);
    expect(
      TrackingPortalDisplayBoundaryRowSchema.safeParse({ ...retentionRow, platformWriterExecutionClaimed: true })
        .success
    ).toBe(false);
    expect(TrackingPortalDisplayBoundaryRowSchema.safeParse({ ...retentionRow, productClaimReady: true }).success).toBe(
      false
    );
  });
});

function rowFor(
  proof: TrackingPortalDisplayBoundaryProof,
  boundaryKind: TrackingPortalDisplayBoundaryKind
): TrackingPortalDisplayBoundaryRow {
  const row = proof.rows.find((entry) => entry.boundaryKind === boundaryKind);
  if (row === undefined) {
    throw new Error(`Missing tracking portal display boundary row: ${boundaryKind}`);
  }
  return row;
}
