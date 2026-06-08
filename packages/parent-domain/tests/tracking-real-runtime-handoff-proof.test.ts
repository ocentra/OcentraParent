import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingRealRuntimeHandoffGates,
  TrackingRealRuntimeHandoffRowSchema,
  buildTrackingRealRuntimeHandoffProof,
} from '../src/tracking-real-runtime-handoff-proof';

const requiredArtifacts = ['00-runtime-metadata.json', '01-runtime-result.json'];
const closureAccounting = {
  fullProductUiLocalArtifactCount: 6,
  fullProductUiClosureRetentionWritableExecutionRowCount: 1,
  fullProductUiClosureChildRuntimeMissingArtifactCount: 10,
  childRuntimeRequiredArtifactCount: 10,
  childRuntimePresentArtifactCount: 0,
  childRuntimeMissingArtifactCount: 10,
  retentionRuntimeRequiredArtifactCount: 2,
  retentionRuntimePresentArtifactCount: 1,
  retentionRuntimeMissingArtifactCount: 1,
  retentionRuntimeManualRequiredRowCount: 1,
  retentionRuntimeArtifactSetPresentRowCount: 0,
  productionWorkerRequiredArtifactCount: 8,
  productionWorkerPresentArtifactCount: 0,
  productionWorkerMissingArtifactCount: 8,
  claimAuditPresentArtifactCount: 5,
  claimAuditMissingArtifactCount: 61,
  claimAuditManualRequiredRowCount: 10,
  claimAuditPhysicalDeviceRequiredRowCount: 6,
  claimAuditApprovedManualRequiredRowCount: 1,
  claimAuditManualProviderRuntimeRequiredRowCount: 1,
  claimAuditProductionRuntimeRequiredRowCount: 2,
  claimAuditProductReadyRowCount: 0,
  productClaimReady: false,
} as const;

describe('tracking real runtime handoff proof', () => {
  it('keeps Android and iOS physical-device handoff rows separate', () => {
    expect(RequiredTrackingRealRuntimeHandoffGates.map((gate) => gate.handoffArea)).toContain(
      'android-physical-background-and-geofence'
    );
    expect(RequiredTrackingRealRuntimeHandoffGates.map((gate) => gate.handoffArea)).toContain(
      'ios-physical-background-and-region'
    );
  });

  it('derives one manual-required handoff row per runtime gate', () => {
    const inventories = RequiredTrackingRealRuntimeHandoffGates.map((gate) => ({
      handoffArea: gate.handoffArea,
      proofRoot: `output/tracking-plan-proof/${gate.handoffArea}`,
      requiredArtifacts,
      presentArtifacts: ['00-runtime-metadata.json'],
      auditRefs: [`${gate.handoffArea}-audit`],
    }));

    const proof = buildTrackingRealRuntimeHandoffProof('2026-06-08T02:20:00.000Z', inventories, closureAccounting);

    expect(proof.handoffRows).toHaveLength(RequiredTrackingRealRuntimeHandoffGates.length);
    expect(proof.sourceGateRefs).toEqual(RequiredTrackingRealRuntimeHandoffGates.map((gate) => gate.sourceProofRef));
    expect(proof.summary.manualRequiredRowCount).toBe(RequiredTrackingRealRuntimeHandoffGates.length);
    expect(proof.summary.missingArtifactCount).toBe(RequiredTrackingRealRuntimeHandoffGates.length);
    expect(proof.summary.requiredValidationCommandCount).toBeGreaterThanOrEqual(
      RequiredTrackingRealRuntimeHandoffGates.length
    );
    expect(proof.summary.productReadyRowCount).toBe(0);
    expect(proof.summary.ciRunnableRowCount).toBe(0);
    expect(proof.summary.physicalDeviceRequiredRowCount).toBe(6);
    expect(proof.summary.manualProviderRuntimeRequiredRowCount).toBe(1);
    expect(proof.summary.productionRuntimeRequiredRowCount).toBe(2);
    expect(proof.handoffRows.every((row) => row.requiredValidationCommands.length > 0)).toBe(true);
    expect(proof.handoffRows.every((row) => row.artifactAcceptanceNotes.length > 0)).toBe(true);
    expect(proof.handoffRows.every((row) => row.ciRunnable === false)).toBe(true);
    expect(proof.closureAccounting.fullProductUiLocalArtifactCount).toBe(6);
    expect(proof.closureAccounting.childRuntimeMissingArtifactCount).toBe(10);
    expect(proof.closureAccounting.retentionRuntimeMissingArtifactCount).toBe(1);
    expect(proof.closureAccounting.retentionRuntimeArtifactSetPresentRowCount).toBe(0);
    expect(proof.closureAccounting.productionWorkerMissingArtifactCount).toBe(8);
    expect(proof.closureAccounting.claimAuditMissingArtifactCount).toBe(61);
    expect(proof.closureAccounting.claimAuditPhysicalDeviceRequiredRowCount).toBe(6);
    expect(proof.closureAccounting.claimAuditApprovedManualRequiredRowCount).toBe(1);
    expect(proof.closureAccounting.claimAuditManualProviderRuntimeRequiredRowCount).toBe(1);
    expect(proof.closureAccounting.claimAuditProductionRuntimeRequiredRowCount).toBe(2);
    expect(Object.values(proof.productClaims).every((claim) => claim === false)).toBe(true);
  });

  it('keeps product-ready false even when runtime artifact sets are complete', () => {
    const inventories = RequiredTrackingRealRuntimeHandoffGates.map((gate) => ({
      handoffArea: gate.handoffArea,
      proofRoot: `output/tracking-plan-proof/${gate.handoffArea}`,
      requiredArtifacts,
      presentArtifacts: requiredArtifacts,
      auditRefs: [`${gate.handoffArea}-audit`],
    }));

    const proof = buildTrackingRealRuntimeHandoffProof('2026-06-08T02:20:00.000Z', inventories, closureAccounting);

    expect(proof.handoffRows.every((row) => row.status === 'artifact-set-present')).toBe(true);
    expect(proof.handoffRows.every((row) => row.artifactSetComplete)).toBe(true);
    expect(proof.productClaims.productReadyClaimed).toBe(false);
    expect(proof.summary.productReadyRowCount).toBe(0);
  });
});

describe('tracking real runtime handoff overclaim rejection', () => {
  it('rejects handoff rows that claim product readiness', () => {
    const invalid = TrackingRealRuntimeHandoffRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-real-runtime-handoff-invalid',
      generatedAt: '2026-06-08T02:20:00.000Z',
      handoffArea: 'android-physical-background-and-geofence',
      blockerId: 'android-physical-background-proof-required',
      sourceProofRef: 'test-results/tracking-physical-device-artifact-gate-proof/proof.json',
      proofRoot: 'output/tracking-plan-proof/android-background-geofence',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      status: 'manual-required',
      requiredArtifacts,
      presentArtifacts: ['00-runtime-metadata.json'],
      missingArtifacts: ['01-runtime-result.json'],
      readinessCategory: 'physical-device-required',
      ciRunnable: false,
      requiredValidationCommands: ['Run Android physical-device proof'],
      artifactAcceptanceNotes: ['Require physical-device artifact evidence'],
      auditRefs: ['tracking-real-runtime-handoff-invalid-audit'],
      artifactSetComplete: false,
      productClaimReady: true,
    });

    expect(invalid.success).toBe(false);
  });
});
