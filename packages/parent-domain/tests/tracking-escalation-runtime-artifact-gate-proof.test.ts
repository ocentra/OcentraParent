import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingEscalationRuntimeReadinessBlockers,
  type TrackingEscalationRuntimeReadinessBlockerProof,
} from '../src/tracking-escalation-runtime-readiness-blocker-proof';
import {
  buildTrackingEscalationRuntimeArtifactGateProof,
  TrackingEscalationRuntimeArtifactGateRowSchema,
} from '../src/tracking-escalation-runtime-artifact-gate-proof';
import { RequiredTrackingProviderRuntimeReadinessBlockers } from '../src/tracking-provider-runtime-readiness-blocker-proof';

const generatedAt = '2026-06-08T00:30:00.000Z';

describe('tracking escalation runtime artifact gate proof', () => {
  it('keeps escalation runtime manual-required while required runtime artifacts are missing', () => {
    const escalationReadinessProof = buildEscalationReadinessProof();
    const proof = buildTrackingEscalationRuntimeArtifactGateProof(generatedAt, escalationReadinessProof, {
      presentArtifacts: [],
    });

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('manual-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_MANUAL_ESCALATION_RUNTIME');
    expect(proof.rows[0].sourceRuntimeReadinessBlockers).toEqual([
      ...RequiredTrackingEscalationRuntimeReadinessBlockers,
    ]);
    expect(proof.rows[0].requiredArtifacts).toContain('provider-credentials');
    expect(proof.rows[0].missingArtifacts).toEqual(proof.rows[0].requiredArtifacts);
    expect(proof.productClaims.productionEscalationWorkerRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productionQuietHoursTimerRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('marks artifact-set-present only when each escalation runtime artifact exists', () => {
    const escalationReadinessProof = buildEscalationReadinessProof();
    const requiredArtifacts = [
      ...new Set(escalationReadinessProof.blockers.flatMap((row) => row.blockingArtifactRefs)),
    ];
    const proof = buildTrackingEscalationRuntimeArtifactGateProof(generatedAt, escalationReadinessProof, {
      presentArtifacts: requiredArtifacts,
    });

    expect(proof.rows[0].status).toBe('artifact-set-present');
    expect(proof.rows[0].escalationRuntimeArtifactSetComplete).toBe(true);
    expect(proof.rows[0].missingArtifacts).toEqual([]);
    expect(proof.productClaims.parentNotificationHistoryRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childDeviceDeliveryRuntimeClaimed).toBe(false);
    expect(proof.productClaims.authorityProofClaimed).toBe(false);
  });

  it('rejects rows that claim escalation runtime without product proof', () => {
    const invalid = TrackingEscalationRuntimeArtifactGateRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-escalation-runtime-artifacts-invalid',
      generatedAt,
      proofRoot: 'output/tracking-plan-proof',
      requiredProofTier: 'P4_MANUAL_ESCALATION_RUNTIME',
      currentProofTier: 'P1_FIXTURE_SIMULATION',
      status: 'manual-required',
      sourceRuntimeReadinessProofRef:
        'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/53-escalation-runtime-readiness-blocker-proof.json',
      sourceRuntimeReadinessBlockers: [...RequiredTrackingEscalationRuntimeReadinessBlockers],
      requiredArtifacts: ['provider-credentials'],
      presentArtifacts: [],
      missingArtifacts: ['provider-credentials'],
      auditRefs: ['tracking-escalation-runtime-artifacts-invalid-audit'],
      escalationRuntimeArtifactSetComplete: false,
      productionEscalationWorkerRuntimeClaimed: true,
      productionQuietHoursTimerRuntimeClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      providerReceiptIngestionRuntimeClaimed: false,
      parentNotificationHistoryRuntimeClaimed: false,
      childDeviceDeliveryRuntimeClaimed: false,
      durableEscalationStorageClaimed: false,
      emergencyAutoContactPolicyClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});

function buildEscalationReadinessProof(): TrackingEscalationRuntimeReadinessBlockerProof {
  const sourceProofRefs = [
    'test-results/tracking-escalation-readiness-proof/proof.json',
    'test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json',
  ];
  const providerBlockerRefs = RequiredTrackingProviderRuntimeReadinessBlockers.map(
    (blockerId) => `tracking-provider-runtime-${blockerId}-artifact`
  );

  return {
    schemaVersion: 'v0.6',
    proofId: 'tracking-escalation-runtime-readiness-blocker-proof',
    generatedAt,
    proofMode: 'tracking-escalation-runtime-readiness-blocker-proof',
    sourceProofRefs,
    escalationReadinessRows: 1,
    escalationManualRequiredRows: 1,
    providerRuntimeBlockerRows: RequiredTrackingProviderRuntimeReadinessBlockers.length,
    blockers: RequiredTrackingEscalationRuntimeReadinessBlockers.map((blockerId) => ({
      blockerId,
      status: 'manual-required',
      sourceProofRefs,
      blockingArtifactRefs: ['provider-credentials', ...providerBlockerRefs],
      requiredProofTier: 'P4_MANUAL_ESCALATION_RUNTIME',
      currentProofTier: 'P1_FIXTURE_SIMULATION',
      productClaimReady: false,
    })),
    productClaims: {
      productionEscalationWorkerRuntimeClaimed: false,
      productionQuietHoursTimerRuntimeClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      providerReceiptIngestionRuntimeClaimed: false,
      providerCredentialsClaimed: false,
      parentNotificationHistoryRuntimeClaimed: false,
      childDeviceDeliveryRuntimeClaimed: false,
      durableEscalationStorageClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      emergencyAutoContactPolicyClaimed: false,
      productClaimReady: false,
    },
  };
}
