import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingProductReadinessClosureBlockers,
  RequiredTrackingProductReadinessClosureCoverageTags,
  TrackingProductReadinessClosureRowSchema,
  buildTrackingProductReadinessClosureProof,
} from '../src/tracking-product-readiness-closure-proof';

const GeneratedAt = '2026-06-07T16:30:00.000Z';

describe('tracking product readiness closure proof', () => {
  it('enumerates remaining product blockers while preserving local CI proof refs', () => {
    const proof = buildTrackingProductReadinessClosureProof(GeneratedAt, sourceProofs(), aggregateEvidence());

    expectClosureCoverage(proof);
    expectFullProductUiAggregate(proof);
    expectPlatformAggregate(proof);
    expectRuntimeAndProductionAggregate(proof);
  });
});

describe('tracking product readiness closure row schema', () => {
  it('rejects product-ready overclaims', () => {
    const [row] = buildTrackingProductReadinessClosureProof(GeneratedAt, sourceProofs(), aggregateEvidence()).rows;

    expect(
      TrackingProductReadinessClosureRowSchema.safeParse({
        ...row,
        productReadyClaimed: true,
      }).success
    ).toBe(false);
  });

  it('rejects incomplete blocker accounting', () => {
    const [row] = buildTrackingProductReadinessClosureProof(GeneratedAt, sourceProofs(), aggregateEvidence()).rows;

    expect(
      TrackingProductReadinessClosureRowSchema.safeParse({
        ...row,
        remainingBlockers: RequiredTrackingProductReadinessClosureBlockers.slice(0, 2),
      }).success
    ).toBe(false);
  });
});

function sourceProofs() {
  return RequiredTrackingProductReadinessClosureCoverageTags.map((coverageTag) => ({
    coverageTag,
    proofRef: `output/tracking-plan-proof/${coverageTag}/proof.json`,
    status: 'proved',
    proofTier: 'P3_LOCAL_DEV_MACHINE',
  }));
}

type TrackingProductReadinessClosureProof = ReturnType<typeof buildTrackingProductReadinessClosureProof>;

function expectClosureCoverage(proof: TrackingProductReadinessClosureProof): void {
  expect(proof.sourceProofs.map((sourceProof) => sourceProof.coverageTag)).toEqual([
    ...RequiredTrackingProductReadinessClosureCoverageTags,
  ]);
  expect(proof.rows).toHaveLength(1);
  expect(proof.rows[0].remainingBlockers).toEqual([...RequiredTrackingProductReadinessClosureBlockers]);
  expect(proof.productClaims.localCiProofAccountingReady).toBe(true);
  expect(proof.productClaims.physicalAndroidBackgroundClaimed).toBe(false);
  expect(proof.productClaims.physicalIosBackgroundClaimed).toBe(false);
  expect(proof.productClaims.productReadyClaimed).toBe(false);
}

function expectFullProductUiAggregate(proof: TrackingProductReadinessClosureProof): void {
  expect(proof.aggregateEvidence.fullProductUiLocalArtifactCount).toBe(5);
  expect(proof.aggregateEvidence.fullProductUiClosureRetentionWritableExecutionRowCount).toBe(1);
  expect(proof.aggregateEvidence.fullProductUiClosureChildRuntimeMissingArtifactCount).toBe(10);
  expect(proof.aggregateEvidence.fullProductUiRuntimePreflightRowCount).toBe(4);
  expect(proof.aggregateEvidence.fullProductUiRuntimePreflightManualRequiredRowCount).toBe(4);
  expect(proof.aggregateEvidence.fullProductUiRuntimePreflightRequiredArtifactCount).toBe(4);
  expect(proof.aggregateEvidence.fullProductUiRuntimePreflightPresentArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.fullProductUiRuntimePreflightMissingArtifactCount).toBe(4);
  expect(proof.aggregateEvidence.fullProductUiRuntimePreflightProductReadyRowCount).toBe(0);
}

function expectPlatformAggregate(proof: TrackingProductReadinessClosureProof): void {
  expect(proof.aggregateEvidence.androidEmulatorRequiredArtifactCount).toBe(12);
  expect(proof.aggregateEvidence.androidEmulatorPresentArtifactCount).toBe(12);
  expect(proof.aggregateEvidence.androidEmulatorMissingArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.androidEmulatorPermissionUiArtifactCount).toBe(3);
  expect(proof.aggregateEvidence.androidEmulatorRuntimeArtifactCount).toBe(8);
  expect(proof.aggregateEvidence.androidEmulatorLocalGeofenceTransitionCount).toBe(3);
  expect(proof.aggregateEvidence.iosSimulatorRequiredArtifactCount).toBe(13);
  expect(proof.aggregateEvidence.iosSimulatorPresentArtifactCount).toBe(13);
  expect(proof.aggregateEvidence.iosSimulatorMissingArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.iosSimulatorPackageArtifactCount).toBe(4);
  expect(proof.aggregateEvidence.iosSimulatorLocationManualRequiredArtifactCount).toBe(3);
  expect(proof.aggregateEvidence.iosSimulatorPrivacyDisclosureArtifactCount).toBe(2);
  expect(proof.aggregateEvidence.iosSimulatorManualRequiredRowCount).toBe(7);
  expect(proof.aggregateEvidence.iosSimulatorMissingRuntimeArtifactCount).toBe(9);
  expect(proof.aggregateEvidence.authorityRuntimeRequiredArtifactCount).toBe(20);
  expect(proof.aggregateEvidence.authorityRuntimePresentArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.authorityRuntimeMissingArtifactCount).toBe(20);
  expect(proof.aggregateEvidence.authorityRuntimeBlockerCount).toBe(10);
}

function expectRuntimeAndProductionAggregate(proof: TrackingProductReadinessClosureProof): void {
  expect(proof.aggregateEvidence.childRuntimeRequiredArtifactCount).toBe(10);
  expect(proof.aggregateEvidence.childRuntimePresentArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.childRuntimeMissingArtifactCount).toBe(10);
  expect(proof.aggregateEvidence.physicalDeviceEvidenceReviewRowCount).toBe(2);
  expect(proof.aggregateEvidence.physicalDeviceEvidenceReviewArtifactMissingRowCount).toBe(2);
  expect(proof.aggregateEvidence.physicalDeviceEvidenceReviewContentReviewRequiredRowCount).toBe(0);
  expect(proof.aggregateEvidence.physicalDeviceEvidenceReviewContentAcceptedRowCount).toBe(0);
  expect(proof.aggregateEvidence.physicalDeviceEvidenceReviewProductReadyRowCount).toBe(0);
  expect(proof.aggregateEvidence.physicalDeviceEvidenceReviewStatusObservedRowCount).toBe(1);
  expect(proof.aggregateEvidence.physicalDeviceEvidenceReviewSupportingStatusArtifactCount).toBe(13);
  expect(proof.aggregateEvidence.providerRuntimeRequiredArtifactCount).toBe(11);
  expect(proof.aggregateEvidence.providerRuntimePresentArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.providerRuntimeMissingArtifactCount).toBe(11);
  expect(proof.aggregateEvidence.providerRuntimeBlockerCount).toBe(12);
  expect(proof.aggregateEvidence.escalationRuntimeRequiredArtifactCount).toBe(13);
  expect(proof.aggregateEvidence.escalationRuntimePresentArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.escalationRuntimeMissingArtifactCount).toBe(13);
  expect(proof.aggregateEvidence.escalationRuntimeBlockerCount).toBe(12);
  expect(proof.aggregateEvidence.retentionRuntimeRequiredArtifactCount).toBe(2);
  expect(proof.aggregateEvidence.retentionRuntimePresentArtifactCount).toBe(1);
  expect(proof.aggregateEvidence.retentionRuntimeMissingArtifactCount).toBe(1);
  expect(proof.aggregateEvidence.retentionRuntimeManualRequiredRowCount).toBe(1);
  expect(proof.aggregateEvidence.retentionRuntimeArtifactSetPresentRowCount).toBe(0);
  expect(proof.aggregateEvidence.retentionPlatformPreflightRowCount).toBe(3);
  expect(proof.aggregateEvidence.retentionPlatformPreflightManualRequiredRowCount).toBe(3);
  expect(proof.aggregateEvidence.retentionPlatformPreflightRequiredArtifactCount).toBe(6);
  expect(proof.aggregateEvidence.retentionPlatformPreflightPresentArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.retentionPlatformPreflightMissingArtifactCount).toBe(6);
  expect(proof.aggregateEvidence.retentionPlatformPreflightProductReadyRowCount).toBe(0);
  expect(proof.aggregateEvidence.productionWorkerRequiredArtifactCount).toBe(8);
  expect(proof.aggregateEvidence.productionWorkerPresentArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.productionWorkerMissingArtifactCount).toBe(8);
  expect(proof.aggregateEvidence.productionWorkerPreflightRowCount).toBe(8);
  expect(proof.aggregateEvidence.productionWorkerPreflightManualRequiredRowCount).toBe(8);
  expect(proof.aggregateEvidence.productionWorkerPreflightRequiredArtifactCount).toBe(8);
  expect(proof.aggregateEvidence.productionWorkerPreflightPresentArtifactCount).toBe(0);
  expect(proof.aggregateEvidence.productionWorkerPreflightMissingArtifactCount).toBe(8);
  expect(proof.aggregateEvidence.productionWorkerPreflightProductReadyRowCount).toBe(0);
  expect(proof.aggregateEvidence.claimAuditPresentArtifactCount).toBe(5);
  expect(proof.aggregateEvidence.claimAuditPhysicalDeviceRequiredRowCount).toBe(7);
  expect(proof.aggregateEvidence.claimAuditApprovedManualRequiredRowCount).toBe(1);
  expect(proof.aggregateEvidence.claimAuditManualProviderRuntimeRequiredRowCount).toBe(1);
  expect(proof.aggregateEvidence.claimAuditProductionRuntimeRequiredRowCount).toBe(2);
  expect(proof.aggregateEvidence.productClaimReady).toBe(false);
}

function aggregateEvidence() {
  return {
    fullProductUiLocalArtifactCount: 5,
    fullProductUiClosureRetentionWritableExecutionRowCount: 1,
    fullProductUiClosureRetentionWritableExecutionDerivationCount: 1,
    fullProductUiClosureChildRuntimeMissingArtifactCount: 10,
    fullProductUiRuntimePreflightRowCount: 4,
    fullProductUiRuntimePreflightManualRequiredRowCount: 4,
    fullProductUiRuntimePreflightRequiredArtifactCount: 4,
    fullProductUiRuntimePreflightPresentArtifactCount: 0,
    fullProductUiRuntimePreflightMissingArtifactCount: 4,
    fullProductUiRuntimePreflightProductReadyRowCount: 0,
    androidEmulatorRequiredArtifactCount: 12,
    androidEmulatorPresentArtifactCount: 12,
    androidEmulatorMissingArtifactCount: 0,
    androidEmulatorPermissionUiArtifactCount: 3,
    androidEmulatorRuntimeArtifactCount: 8,
    androidEmulatorLocalGeofenceTransitionCount: 3,
    iosSimulatorRequiredArtifactCount: 13,
    iosSimulatorPresentArtifactCount: 13,
    iosSimulatorMissingArtifactCount: 0,
    iosSimulatorPackageArtifactCount: 4,
    iosSimulatorLocationManualRequiredArtifactCount: 3,
    iosSimulatorPrivacyDisclosureArtifactCount: 2,
    iosSimulatorManualRequiredRowCount: 7,
    iosSimulatorMissingRuntimeArtifactCount: 9,
    authorityRuntimeRequiredArtifactCount: 20,
    authorityRuntimePresentArtifactCount: 0,
    authorityRuntimeMissingArtifactCount: 20,
    authorityRuntimeBlockerCount: 10,
    childRuntimeRequiredArtifactCount: 10,
    childRuntimePresentArtifactCount: 0,
    childRuntimeMissingArtifactCount: 10,
    physicalDeviceEvidenceReviewRowCount: 2,
    physicalDeviceEvidenceReviewArtifactMissingRowCount: 2,
    physicalDeviceEvidenceReviewContentReviewRequiredRowCount: 0,
    physicalDeviceEvidenceReviewContentAcceptedRowCount: 0,
    physicalDeviceEvidenceReviewProductReadyRowCount: 0,
    physicalDeviceEvidenceReviewStatusObservedRowCount: 1,
    physicalDeviceEvidenceReviewSupportingStatusArtifactCount: 13,
    providerRuntimeRequiredArtifactCount: 11,
    providerRuntimePresentArtifactCount: 0,
    providerRuntimeMissingArtifactCount: 11,
    providerRuntimeBlockerCount: 12,
    escalationRuntimeRequiredArtifactCount: 13,
    escalationRuntimePresentArtifactCount: 0,
    escalationRuntimeMissingArtifactCount: 13,
    escalationRuntimeBlockerCount: 12,
    retentionRuntimeRequiredArtifactCount: 2,
    retentionRuntimePresentArtifactCount: 1,
    retentionRuntimeMissingArtifactCount: 1,
    retentionRuntimeManualRequiredRowCount: 1,
    retentionRuntimeArtifactSetPresentRowCount: 0,
    retentionPlatformPreflightRowCount: 3,
    retentionPlatformPreflightManualRequiredRowCount: 3,
    retentionPlatformPreflightRequiredArtifactCount: 6,
    retentionPlatformPreflightPresentArtifactCount: 0,
    retentionPlatformPreflightMissingArtifactCount: 6,
    retentionPlatformPreflightProductReadyRowCount: 0,
    productionWorkerRequiredArtifactCount: 8,
    productionWorkerPresentArtifactCount: 0,
    productionWorkerMissingArtifactCount: 8,
    productionWorkerPreflightRowCount: 8,
    productionWorkerPreflightManualRequiredRowCount: 8,
    productionWorkerPreflightRequiredArtifactCount: 8,
    productionWorkerPreflightPresentArtifactCount: 0,
    productionWorkerPreflightMissingArtifactCount: 8,
    productionWorkerPreflightProductReadyRowCount: 0,
    claimAuditPresentArtifactCount: 5,
    claimAuditMissingArtifactCount: 50,
    claimAuditManualRequiredRowCount: 11,
    claimAuditPhysicalDeviceRequiredRowCount: 7,
    claimAuditApprovedManualRequiredRowCount: 1,
    claimAuditManualProviderRuntimeRequiredRowCount: 1,
    claimAuditProductionRuntimeRequiredRowCount: 2,
    claimAuditProductReadyRowCount: 0,
    productClaimReady: false,
  };
}
