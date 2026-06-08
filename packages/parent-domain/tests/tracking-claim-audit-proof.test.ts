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
    expect(proof.summary.physicalDeviceRequiredRowCount).toBe(7);
    expect(proof.summary.approvedManualRequiredRowCount).toBe(1);
    expect(proof.summary.manualProviderRuntimeRequiredRowCount).toBe(1);
    expect(proof.summary.productionRuntimeRequiredRowCount).toBe(2);
    expect(proof.summary.acceptanceCriteriaCount).toBe(RequiredTrackingClaimAuditPlans.length * 4);
    expect(proof.summary.manualValidationCommandCount).toBe(RequiredTrackingClaimAuditPlans.length * 3);
    expect(proof.summary.artifactAcceptanceNoteCount).toBe(RequiredTrackingClaimAuditPlans.length * 4);
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
});

describe('tracking claim audit acceptance matrix', () => {
  it('derives a manual acceptance matrix for every hard claim row', () => {
    const proof = buildTrackingClaimAuditProof(GeneratedAt, []);

    for (const row of proof.rows) {
      expect(row.acceptanceCriteria).toHaveLength(4);
      expect(row.manualValidationCommands).toEqual([
        'node scripts/test/tracking-claim-audit-proof.mjs',
        'node scripts/test/tracking-product-readiness-closure-proof.mjs',
        'node scripts/test/tracking-real-runtime-handoff-proof.mjs',
      ]);
      expect(row.artifactAcceptanceNotes).toHaveLength(4);
      expect(row.acceptanceCriteria.join('\n')).toContain(row.proofRoot);
      expect(row.acceptanceCriteria.join('\n')).toContain(row.requiredProofTier);
      expect(row.artifactAcceptanceNotes.join('\n')).toContain('claimApproved remains false');
    }
  });

  it('routes physical Android and iOS claims through evidence-review proof', () => {
    const proof = buildTrackingClaimAuditProof(GeneratedAt, []);
    const physicalRows = proof.rows.filter((row) =>
      ['android-physical-background-and-geofence', 'ios-physical-background-and-region'].includes(row.auditArea)
    );

    expect(physicalRows).toHaveLength(2);
    expect(
      physicalRows.every(
        (row) => row.sourceProofRef === 'test-results/tracking-physical-device-evidence-review-proof/proof.json'
      )
    ).toBe(true);
    expect(
      physicalRows.every((row) =>
        row.supportingProofRefs.includes('test-results/tracking-physical-device-artifact-gate-proof/proof.json')
      )
    ).toBe(true);
  });
});

describe('tracking claim audit local artifact evidence', () => {
  it('carries full-product UI local artifact evidence without approving the claim', () => {
    const fullProductPlan = RequiredTrackingClaimAuditPlans.find(
      (plan) => plan.auditArea === 'full-product-parent-child-ui-runtime'
    );
    if (!fullProductPlan) throw new Error('Missing full product UI claim audit plan');

    const proof = buildTrackingClaimAuditProof(GeneratedAt, [
      {
        auditArea: fullProductPlan.auditArea,
        presentArtifacts: fullProductPlan.requiredArtifacts.filter((artifact) =>
          ['01-', '02-', '03-', '08-', '09-'].some((prefix) => artifact.startsWith(prefix))
        ),
      },
    ]);
    const row = proof.rows.find((candidate) => candidate.auditArea === fullProductPlan.auditArea);

    expect(row?.supportingProofRefs).toContain(
      'test-results/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json'
    );
    expect(row?.acceptanceCriteria.join('\n')).toContain('local P3 artifacts cannot approve the claim');
    expect(row?.artifactAcceptanceNotes.join('\n')).toContain('P4_PHYSICAL_DEVICE');
    expect(row?.presentArtifacts).toHaveLength(5);
    expect(row?.missingArtifacts).toEqual([
      '04-retention-settings-production-write-result.png',
      '05-child-device-rendered-check-in-runtime.png',
      '06-child-device-rendered-location-consent-runtime.png',
      '07-child-device-safe-help-response-runtime.png',
    ]);
    expect(row?.artifactSetComplete).toBe(false);
    expect(row?.fullProductUiClaimed).toBe(false);
    expect(row?.productClaimReady).toBe(false);
  });

  it('gives retention writable product settings its own runtime claim row', () => {
    const retentionRuntimePlan = RequiredTrackingClaimAuditPlans.find(
      (plan) => plan.auditArea === 'retention-product-settings-writable-runtime'
    );
    if (!retentionRuntimePlan) throw new Error('Missing retention writable runtime claim audit plan');

    const proof = buildTrackingClaimAuditProof(GeneratedAt, []);
    const row = proof.rows.find((candidate) => candidate.auditArea === retentionRuntimePlan.auditArea);

    expect(row?.sourceProofRef).toBe('test-results/tracking-full-product-ui-runtime-preflight-proof/proof.json');
    expect(row?.supportingProofRefs).toContain(
      'test-results/tracking-retention-product-settings-writable-execution-proof/proof.json'
    );
    expect(row?.requiredArtifacts).toEqual(['04-retention-settings-production-write-result.png']);
    expect(row?.missingArtifacts).toEqual(['04-retention-settings-production-write-result.png']);
    expect(row?.fullProductUiClaimed).toBe(false);
    expect(row?.retentionProductRuntimeClaimed).toBe(false);
    expect(row?.productClaimReady).toBe(false);
  });
});

describe('tracking claim audit proof overclaim rejection', () => {
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
