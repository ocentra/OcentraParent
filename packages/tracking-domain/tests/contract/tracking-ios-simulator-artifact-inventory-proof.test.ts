import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingIosSimulatorArtifactRefs,
  TrackingIosSimulatorArtifactInventoryRowSchema,
  buildTrackingIosSimulatorArtifactInventoryProof,
} from '@ocentra-parent/schema-domain/tracking-ios-simulator-artifact-inventory-proof';

const GeneratedAt = '2026-06-08T11:00:00.000Z';

describe('tracking iOS simulator artifact inventory proof', () => {
  it('classifies simulator and manual-required artifacts without claiming Core Location runtime', () => {
    const proof = buildTrackingIosSimulatorArtifactInventoryProof(GeneratedAt, input());

    expect(proof.rows).toHaveLength(1);
    expect(proof.summary.requiredArtifactCount).toBe(RequiredTrackingIosSimulatorArtifactRefs.length);
    expect(proof.summary.presentArtifactCount).toBe(RequiredTrackingIosSimulatorArtifactRefs.length);
    expect(proof.summary.missingArtifactCount).toBe(0);
    expect(proof.summary.simulatorPackageArtifactCount).toBe(4);
    expect(proof.summary.locationManualRequiredArtifactCount).toBe(3);
    expect(proof.summary.privacyDisclosureArtifactCount).toBe(2);
    expect(proof.summary.platformProofArtifactCount).toBe(2);
    expect(proof.summary.validationLogArtifactCount).toBe(2);
    expect(proof.summary.iosManualRequiredRowCount).toBe(7);
    expect(proof.summary.iosMissingRuntimeArtifactCount).toBe(9);
    expect(proof.productClaims.simulatorArtifactInventoryComplete).toBe(true);
    expect(proof.productClaims.coreLocationRuntimeClaimed).toBe(false);
    expect(proof.productClaims.backgroundRegionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.physicalDeviceProofClaimed).toBe(false);
    expect(proof.productClaims.authorityProofClaimed).toBe(false);
    expect(proof.productClaims.providerDeliveryClaimed).toBe(false);
    expect(proof.productClaims.productionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('keeps missing simulator artifact accounting explicit', () => {
    const [firstArtifact, ...remainingArtifacts] = artifactRows();
    const proof = buildTrackingIosSimulatorArtifactInventoryProof(GeneratedAt, {
      ...input(),
      artifactRows: [
        {
          ...firstArtifact,
          present: false,
          byteSize: 0,
        },
        ...remainingArtifacts,
      ],
    });

    expect(proof.summary.presentArtifactCount).toBe(RequiredTrackingIosSimulatorArtifactRefs.length - 1);
    expect(proof.summary.missingArtifactCount).toBe(1);
    expect(proof.productClaims.simulatorArtifactInventoryComplete).toBe(false);
  });

  it('rejects rows that claim iOS physical-device readiness', () => {
    const [row] = buildTrackingIosSimulatorArtifactInventoryProof(GeneratedAt, input()).rows;

    expect(
      TrackingIosSimulatorArtifactInventoryRowSchema.safeParse({
        ...row,
        physicalDeviceProofClaimed: true,
      }).success
    ).toBe(false);
  });
});

function input() {
  return {
    sourceIosSimulatorProofRef: 'test-results/tracking-plan-ios-simulator-proof/proof.json',
    iosSimulatorProofStatus: 'manual_required',
    iosSimulatorCurrentProofTier: 'P2_HOSTED_CI',
    hostPlatform: 'win32',
    hostArch: 'x64',
    canRunXcodeSimulator: false,
    iosManualRequiredRowCount: 7,
    iosRequiredRuntimeArtifactCount: 9,
    iosPresentRuntimeArtifactCount: 0,
    iosMissingRuntimeArtifactCount: 9,
    privacyReleaseGateRowCount: 6,
    privacyReleaseBlockedCount: 3,
    artifactRows: artifactRows(),
  };
}

function artifactRows() {
  return RequiredTrackingIosSimulatorArtifactRefs.map((artifactRef) => ({
    artifactRef,
    category: categoryFor(artifactRef),
    required: true,
    present: true,
    byteSize: 10,
  }));
}

function categoryFor(artifactRef: string) {
  if (artifactRef.includes('18-ios-simulator-proof') || artifactRef.includes('tracking-plan-ios-simulator-proof')) {
    return 'simulator-package-proof';
  }
  if (artifactRef.includes('19-ios-location-manual-required') || artifactRef.includes('tracking-ios-location-manual')) {
    return 'location-manual-required-proof';
  }
  if (artifactRef.includes('20-ios-privacy-disclosure') || artifactRef.includes('tracking-ios-privacy')) {
    return 'privacy-disclosure-proof';
  }
  if (artifactRef.includes('validation-commands')) return 'validation-log';
  return 'platform-proof';
}
