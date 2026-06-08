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
  childRuntimeRequiredArtifactCount: 10,
  childRuntimePresentArtifactCount: 0,
  childRuntimeMissingArtifactCount: 10,
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

const claimAuditInventories = RequiredTrackingRealRuntimeHandoffGates.map((gate) => ({
  auditArea: gate.handoffArea,
  sourceProofRef: `test-results/tracking-claim-audit-proof/${gate.handoffArea}.json`,
  acceptanceCriteria: [
    `Collect every required artifact for ${gate.handoffArea} before review.`,
    `Keep required proof tier ${gate.requiredProofTier}; local P3 artifacts cannot approve the claim.`,
  ],
  manualValidationCommands: [
    'node scripts/test/tracking-claim-audit-proof.mjs',
    'node scripts/test/tracking-real-runtime-handoff-proof.mjs',
  ],
  artifactAcceptanceNotes: [
    'Status can move only to review-required when all required artifacts are present; claimApproved remains false here.',
  ],
}));

type TrackingRealRuntimeHandoffProof = ReturnType<typeof buildTrackingRealRuntimeHandoffProof>;

function expectHandoffSummary(proof: TrackingRealRuntimeHandoffProof): void {
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
}

function expectClaimAuditAcceptance(proof: TrackingRealRuntimeHandoffProof): void {
  expect(proof.summary.claimAuditAcceptanceCriteriaCount).toBe(RequiredTrackingRealRuntimeHandoffGates.length * 2);
  expect(proof.summary.claimAuditManualValidationCommandCount).toBe(RequiredTrackingRealRuntimeHandoffGates.length * 2);
  expect(proof.summary.claimAuditArtifactAcceptanceNoteCount).toBe(RequiredTrackingRealRuntimeHandoffGates.length);
  expect(proof.handoffRows.every((row) => row.requiredValidationCommands.length > 0)).toBe(true);
  expect(proof.handoffRows.every((row) => row.artifactAcceptanceNotes.length > 0)).toBe(true);
  expect(
    proof.handoffRows.every((row) =>
      row.claimAuditAcceptance.artifactAcceptanceNotes.some((note) => note.includes('claimApproved remains false'))
    )
  ).toBe(true);
  expect(proof.handoffRows.every((row) => row.ciRunnable === false)).toBe(true);
}

function expectClosureAccounting(proof: TrackingRealRuntimeHandoffProof): void {
  expect(proof.closureAccounting.fullProductUiLocalArtifactCount).toBe(6);
  expect(proof.closureAccounting.fullProductUiRuntimePreflightRowCount).toBe(4);
  expect(proof.closureAccounting.fullProductUiRuntimePreflightManualRequiredRowCount).toBe(4);
  expect(proof.closureAccounting.fullProductUiRuntimePreflightRequiredArtifactCount).toBe(4);
  expect(proof.closureAccounting.fullProductUiRuntimePreflightPresentArtifactCount).toBe(0);
  expect(proof.closureAccounting.fullProductUiRuntimePreflightMissingArtifactCount).toBe(4);
  expect(proof.closureAccounting.fullProductUiRuntimePreflightProductReadyRowCount).toBe(0);
  expect(proof.closureAccounting.androidEmulatorRequiredArtifactCount).toBe(12);
  expect(proof.closureAccounting.androidEmulatorPresentArtifactCount).toBe(12);
  expect(proof.closureAccounting.androidEmulatorMissingArtifactCount).toBe(0);
  expect(proof.closureAccounting.androidEmulatorPermissionUiArtifactCount).toBe(3);
  expect(proof.closureAccounting.androidEmulatorRuntimeArtifactCount).toBe(8);
  expect(proof.closureAccounting.androidEmulatorLocalGeofenceTransitionCount).toBe(3);
  expect(proof.closureAccounting.iosSimulatorRequiredArtifactCount).toBe(13);
  expect(proof.closureAccounting.iosSimulatorPresentArtifactCount).toBe(13);
  expect(proof.closureAccounting.iosSimulatorMissingArtifactCount).toBe(0);
  expect(proof.closureAccounting.iosSimulatorPackageArtifactCount).toBe(4);
  expect(proof.closureAccounting.iosSimulatorLocationManualRequiredArtifactCount).toBe(3);
  expect(proof.closureAccounting.iosSimulatorPrivacyDisclosureArtifactCount).toBe(2);
  expect(proof.closureAccounting.iosSimulatorManualRequiredRowCount).toBe(7);
  expect(proof.closureAccounting.iosSimulatorMissingRuntimeArtifactCount).toBe(9);
  expect(proof.closureAccounting.childRuntimeMissingArtifactCount).toBe(10);
  expect(proof.closureAccounting.retentionRuntimeMissingArtifactCount).toBe(1);
  expect(proof.closureAccounting.retentionRuntimeArtifactSetPresentRowCount).toBe(0);
  expect(proof.closureAccounting.retentionPlatformPreflightRowCount).toBe(3);
  expect(proof.closureAccounting.retentionPlatformPreflightManualRequiredRowCount).toBe(3);
  expect(proof.closureAccounting.retentionPlatformPreflightRequiredArtifactCount).toBe(6);
  expect(proof.closureAccounting.retentionPlatformPreflightPresentArtifactCount).toBe(0);
  expect(proof.closureAccounting.retentionPlatformPreflightMissingArtifactCount).toBe(6);
  expect(proof.closureAccounting.retentionPlatformPreflightProductReadyRowCount).toBe(0);
  expect(proof.closureAccounting.productionWorkerMissingArtifactCount).toBe(8);
  expect(proof.closureAccounting.claimAuditMissingArtifactCount).toBe(61);
  expect(proof.closureAccounting.claimAuditPhysicalDeviceRequiredRowCount).toBe(6);
  expect(proof.closureAccounting.claimAuditApprovedManualRequiredRowCount).toBe(1);
  expect(proof.closureAccounting.claimAuditManualProviderRuntimeRequiredRowCount).toBe(1);
  expect(proof.closureAccounting.claimAuditProductionRuntimeRequiredRowCount).toBe(2);
}

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

    const proof = buildTrackingRealRuntimeHandoffProof(
      '2026-06-08T02:20:00.000Z',
      inventories,
      closureAccounting,
      claimAuditInventories
    );

    expectHandoffSummary(proof);
    expectClaimAuditAcceptance(proof);
    expectClosureAccounting(proof);
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

    const proof = buildTrackingRealRuntimeHandoffProof(
      '2026-06-08T02:20:00.000Z',
      inventories,
      closureAccounting,
      claimAuditInventories
    );

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
      claimAuditAcceptance: {
        sourceProofRef: 'test-results/tracking-claim-audit-proof/proof.json',
        acceptanceCriteria: ['Collect real Android artifacts before review.'],
        manualValidationCommands: ['node scripts/test/tracking-claim-audit-proof.mjs'],
        artifactAcceptanceNotes: ['claimApproved remains false here.'],
      },
      auditRefs: ['tracking-real-runtime-handoff-invalid-audit'],
      artifactSetComplete: false,
      productClaimReady: true,
    });

    expect(invalid.success).toBe(false);
  });
});
