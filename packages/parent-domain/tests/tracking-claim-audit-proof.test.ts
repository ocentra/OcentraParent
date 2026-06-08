import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingClaimAuditPlans,
  TrackingClaimAuditRowSchema,
  buildTrackingClaimAuditProof,
} from '../src/tracking-claim-audit-proof';

const GeneratedAt = '2026-06-08T04:05:00.000Z';

describe('tracking claim audit proof', () => {
  it('keeps every final tracking claim manual-required without real artifacts', () => {
    const proof = buildTrackingClaimAuditProof(GeneratedAt, []);

    expect(proof.rows).toHaveLength(RequiredTrackingClaimAuditPlans.length);
    expect(proof.summary.manualRequiredRowCount).toBe(RequiredTrackingClaimAuditPlans.length);
    expect(proof.summary.approvedClaimCount).toBe(0);
    expect(proof.summary.productReadyRowCount).toBe(0);
    expect(proof.productClaims.physicalDeviceBehaviorClaimed).toBe(false);
    expect(proof.productClaims.manualDesktopLocationClaimed).toBe(false);
    expect(proof.productClaims.childDeviceRuntimeClaimed).toBe(false);
    expect(proof.productClaims.fullProductUiClaimed).toBe(false);
    expect(proof.productClaims.productReadyClaimed).toBe(false);
  });

  it('records complete artifact sets as review-required instead of product-ready', () => {
    const [plan] = RequiredTrackingClaimAuditPlans;
    const proof = buildTrackingClaimAuditProof(GeneratedAt, [
      {
        auditArea: plan.auditArea,
        presentArtifacts: plan.requiredArtifacts,
      },
    ]);

    expect(proof.rows[0].status).toBe('artifact-set-present-review-required');
    expect(proof.rows[0].artifactSetComplete).toBe(true);
    expect(proof.rows[0].claimApproved).toBe(false);
    expect(proof.rows[0].productClaimReady).toBe(false);
    expect(proof.summary.artifactSetPresentReviewRequiredRowCount).toBe(1);
  });

  it('rejects claim approval overclaims', () => {
    const [row] = buildTrackingClaimAuditProof(GeneratedAt, []).rows;

    expect(
      TrackingClaimAuditRowSchema.safeParse({
        ...row,
        claimApproved: true,
      }).success
    ).toBe(false);
  });

  it('rejects product-ready overclaims', () => {
    const [row] = buildTrackingClaimAuditProof(GeneratedAt, []).rows;

    expect(
      TrackingClaimAuditRowSchema.safeParse({
        ...row,
        productClaimReady: true,
      }).success
    ).toBe(false);
  });
});
