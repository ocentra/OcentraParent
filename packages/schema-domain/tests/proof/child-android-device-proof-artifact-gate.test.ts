import { describe, expect, it } from 'vitest';
import {
  type ChildAndroidDeviceProofArtifactGateReadModel,
  ChildAndroidAddDevicePairingReadinessStateSchema,
  ChildAndroidDeviceProofArtifactGateReadModelSchema,
} from '@ocentra-parent/schema-domain/child-android-device-proof-artifact-gate';

describe('child Android device proof artifact gate contracts', () => {
  acceptsHonestDeviceProofGate();
  rejectsMissingSourceProof();
  rejectsDeviceReadinessUpgrade();
  rejectsPairingInputOverclaim();
  rejectsUsageStatsDeviceEvidenceUpgrade();
  rejectsEnrollmentOrStoreSigningUpgrade();
  rejectsExternalTransportOrParityClaim();
  rejectsAuthorityBoundaryOverclaim();
  rejectsCiArtifactWithoutEvidencePath();
});

function acceptsHonestDeviceProofGate(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: accepts honest CI/package-only gate state', () => {
    const parsed = ChildAndroidDeviceProofArtifactGateReadModelSchema.parse(validReadModel());

    expect(parsed.schemaVersion).toBe('child-android-device-proof-artifact-gate');
    expect(parsed.readinessDecision).toBe('manual-device-evidence-required-before-child-android-readiness');
    expect(parsed.packageMechanicalProofState).toBe('ci-package-only');
    expect(parsed.installAuthorityState).toEqual({
      childAgentArtifactState: 'debug-apk-built',
      installMode: 'debug-apk-sideload',
      installState: 'manual-install-proof-required',
      launchState: 'manual-launch-proof-required',
      removalState: 'manual-removal-proof-required',
      deviceOwnerAuthorityState: 'manual-required',
      managedProfileAuthorityState: 'manual-required',
      childAgentArtifactBoundary:
        'debug APK is the Android child-agent artifact proved by CI package output and checksum only',
      installModeBoundary:
        'proof is limited to debug APK sideload mode and does not claim managed-profile or device-owner packaging',
      installStateBoundary: 'Android install remains manual-required until emulator or physical-device proof exists',
      launchStateBoundary:
        'Android launcher runtime remains manual-required until emulator or physical-device proof exists',
      removalStateBoundary: 'Android uninstall and removal behavior remain manual-required until device proof exists',
      deviceOwnerBoundary: 'manual-required; no device-owner claim is made without enrollment evidence',
      managedProfileBoundary: 'manual-required; no managed-profile claim is made without enrollment evidence',
    });
    expect(parsed.addDevicePairingReadiness).toEqual({
      surface: 'parent-add-device-pairing',
      readinessState: 'manual-required',
      inputs: addDevicePairingInputs(),
      parentVisibleSummary:
        'Android add-device/pairing readiness remains manual-required until emulator or physical-device artifacts exist',
    });
    expect(ChildAndroidAddDevicePairingReadinessStateSchema.parse('not-implemented')).toBe('not-implemented');
    expect(parsed.childAndroidDeviceReadinessState).toBe('manual-required');
    expect(parsed.sourceProofs.map((entry) => entry.source)).toEqual([
      'child-android-protocol-package-lifecycle-proof',
      'child-android-storage-protocol-capability-proof',
      'child-android-service-protocol-capability-proof',
      'child-android-permission-capability-proof',
      'child-android-privileged-capability-proof',
    ]);
    expect(requirementState(parsed, 'debug-apk-build')).toEqual({
      parentCapabilityStatus: 'manual-required',
      artifactClass: 'ci-package-artifact',
      artifactStatus: 'ci-mechanical-proof',
      evidencePath: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
    });
    expect(requirementState(parsed, 'usage-stats-observation-artifact')).toEqual({
      parentCapabilityStatus: 'manual-required',
      artifactClass: 'emulator-device-artifact',
      artifactStatus: 'device-proof-required',
      evidencePath: null,
    });
    expect(requirementState(parsed, 'launch-activity-runtime-artifact')).toEqual({
      parentCapabilityStatus: 'manual-required',
      artifactClass: 'emulator-device-artifact',
      artifactStatus: 'device-proof-required',
      evidencePath: null,
    });
    expect(requirementState(parsed, 'package-removal-artifact')).toEqual({
      parentCapabilityStatus: 'manual-required',
      artifactClass: 'emulator-device-artifact',
      artifactStatus: 'device-proof-required',
      evidencePath: null,
    });
    expect(parsed.manualEvidenceStatus).toEqual({
      custodyState: 'ci-artifacts-only',
      requiredArtifactCount: 17,
      ciArtifactCount: 3,
      collectedDeviceArtifactCount: 0,
      missingDeviceArtifactCount: 14,
      reviewerSummary: 'CI artifacts exist, but real Android device proof artifacts are not collected',
    });
  });
}

function rejectsMissingSourceProof(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: rejects missing composed proof source', () => {
    const model = validReadModel();

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        sourceProofs: model.sourceProofs.filter(
          (entry) => entry.source !== 'child-android-privileged-capability-proof'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsDeviceReadinessUpgrade(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: rejects child Android device readiness upgrade', () => {
    const model = validReadModel();

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        childAndroidDeviceReadinessState: 'ci-package-only',
      }).success
    ).toBe(false);

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        addDevicePairingReadiness: {
          ...model.addDevicePairingReadiness,
          readinessState: 'implemented',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsPairingInputOverclaim(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: rejects add-device input overclaim', () => {
    const model = validReadModel();

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        addDevicePairingReadiness: {
          ...model.addDevicePairingReadiness,
          inputs: model.addDevicePairingReadiness.inputs.map((entry) =>
            entry.input === 'privileged'
              ? {
                  ...entry,
                  readinessState: 'implemented',
                  parentVisibleSummary: 'implemented Android privileged controls',
                }
              : entry
          ),
        },
      }).success
    ).toBe(false);
  });
}

function rejectsUsageStatsDeviceEvidenceUpgrade(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: rejects UsageStats observation as CI proof', () => {
    const model = validReadModel();

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        artifactRequirements: model.artifactRequirements.map((entry) =>
          entry.requirement === 'usage-stats-observation-artifact'
            ? {
                ...entry,
                artifactStatus: 'ci-mechanical-proof',
                evidencePath: 'test-results/child-android-device-proof-artifact-gate/usage-stats.json',
                evidenceCapturedAt: '2026-06-01T00:00:00.000Z',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsEnrollmentOrStoreSigningUpgrade(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: rejects enrollment or store signing upgrades', () => {
    const model = validReadModel();

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        artifactRequirements: model.artifactRequirements.map((entry) =>
          entry.requirement === 'device-owner-enrollment-artifact'
            ? { ...entry, artifactStatus: 'ci-mechanical-proof', parentCapabilityStatus: 'implemented' }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        artifactRequirements: model.artifactRequirements.map((entry) =>
          entry.requirement === 'play-store-signing-artifact'
            ? { ...entry, artifactStatus: 'ci-mechanical-proof', parentCapabilityStatus: 'implemented' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsExternalTransportOrParityClaim(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: rejects external transport or parity claims', () => {
    const model = validReadModel();

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        artifactRequirements: model.artifactRequirements.map((entry) =>
          entry.requirement === 'external-child-agent-transport-artifact'
            ? { ...entry, artifactStatus: 'ci-mechanical-proof', parentCapabilityStatus: 'scaffold' }
            : entry
        ),
      }).success
    ).toBe(false);

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        claimsProved: ['Android child enforcement parity is proved by package-local status bundles'],
      }).success
    ).toBe(false);
  });
}

function rejectsAuthorityBoundaryOverclaim(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: rejects device-owner or managed-profile authority overclaims', () => {
    const model = validReadModel();

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        installAuthorityState: {
          ...model.installAuthorityState,
          deviceOwnerBoundary: 'device-owner claim is proved by CI package output',
        },
      }).success
    ).toBe(false);

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        installAuthorityState: {
          ...model.installAuthorityState,
          managedProfileBoundary: 'managed-profile claim is proved by CI package output',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsCiArtifactWithoutEvidencePath(): void {
  it('ChildAndroidDeviceProofArtifactGateReadModelSchema: rejects CI artifact rows without evidence paths', () => {
    const model = validReadModel();

    expect(
      ChildAndroidDeviceProofArtifactGateReadModelSchema.safeParse({
        ...model,
        artifactRequirements: model.artifactRequirements.map((entry) =>
          entry.requirement === 'debug-apk-build' ? { ...entry, evidencePath: null, evidenceCapturedAt: null } : entry
        ),
      }).success
    ).toBe(false);
  });
}

function validReadModel(): ChildAndroidDeviceProofArtifactGateReadModel {
  return ChildAndroidDeviceProofArtifactGateReadModelSchema.parse({
    schemaVersion: 'child-android-device-proof-artifact-gate',
    checkedAt: '2026-06-01T00:00:00.000Z',
    readinessDecision: 'manual-device-evidence-required-before-child-android-readiness',
    packageMechanicalProofState: 'ci-package-only',
    installAuthorityState: {
      childAgentArtifactState: 'debug-apk-built',
      installMode: 'debug-apk-sideload',
      installState: 'manual-install-proof-required',
      launchState: 'manual-launch-proof-required',
      removalState: 'manual-removal-proof-required',
      deviceOwnerAuthorityState: 'manual-required',
      managedProfileAuthorityState: 'manual-required',
      childAgentArtifactBoundary:
        'debug APK is the Android child-agent artifact proved by CI package output and checksum only',
      installModeBoundary:
        'proof is limited to debug APK sideload mode and does not claim managed-profile or device-owner packaging',
      installStateBoundary: 'Android install remains manual-required until emulator or physical-device proof exists',
      launchStateBoundary:
        'Android launcher runtime remains manual-required until emulator or physical-device proof exists',
      removalStateBoundary: 'Android uninstall and removal behavior remain manual-required until device proof exists',
      deviceOwnerBoundary: 'manual-required; no device-owner claim is made without enrollment evidence',
      managedProfileBoundary: 'manual-required; no managed-profile claim is made without enrollment evidence',
    },
    addDevicePairingReadiness: {
      surface: 'parent-add-device-pairing',
      readinessState: 'manual-required',
      inputs: addDevicePairingInputs(),
      parentVisibleSummary:
        'Android add-device/pairing readiness remains manual-required until emulator or physical-device artifacts exist',
    },
    childAndroidDeviceReadinessState: 'manual-required',
    sourceProofs: sourceProofs(),
    artifactRequirements: artifactRequirements(),
    manualEvidenceStatus: {
      custodyState: 'ci-artifacts-only',
      requiredArtifactCount: 17,
      ciArtifactCount: 3,
      collectedDeviceArtifactCount: 0,
      missingDeviceArtifactCount: 14,
      reviewerSummary: 'CI artifacts exist, but real Android device proof artifacts are not collected',
    },
    claimBoundaries: {
      addDevicePairingReadiness:
        'Parent-visible add-device/pairing readiness remains manual-required and is not remote-control proof',
      childAndroidDeviceReadiness:
        'Android child device readiness remains manual-required until emulator or physical-device artifacts exist',
      emulatorRuntime: 'no emulator install, runtime grant, foreground service, or UsageStats observation is claimed',
      physicalDeviceRuntime: 'no physical-device run, enrollment, managed profile, or privileged behavior is claimed',
      privilegedPermissions: 'UsageStats, Accessibility, VPN, and DNS remain manual-required or unavailable',
      deviceOwnerManagedProfile: 'device-owner and managed-profile states remain blocked without enrollment artifacts',
      playStoreSigning: 'Play Store signing and release-track proof remain planned and not collected',
      externalChildAgentTransport: 'no external LAN/WebSocket Android child-agent transport is claimed',
    },
    claimsProved: ['debug APK, checksum, and package-local status bundles are CI/package proof only'],
    claimsNotProved: [
      'Android add-device/pairing readiness remains manual-required without emulator or physical-device artifacts',
      'Android child device readiness remains manual-required without emulator or physical-device artifacts',
      'Android child enforcement parity is not proved by package-local proof outputs',
      'Android install, launch, removal, UsageStats grant, Accessibility, VPN/DNS, device-owner, managed-profile, signing, and external transport remain unproved',
    ],
  });
}

function sourceProofs() {
  return [
    sourceProof('child-android-protocol-package-lifecycle-proof'),
    sourceProof('child-android-storage-protocol-capability-proof'),
    sourceProof('child-android-service-protocol-capability-proof'),
    sourceProof('child-android-permission-capability-proof'),
    sourceProof('child-android-privileged-capability-proof'),
  ];
}

function addDevicePairingInputs() {
  return [
    addDevicePairingInput('package', 'child-android-protocol-package-lifecycle-proof', 'scaffold'),
    addDevicePairingInput('service', 'child-android-service-protocol-capability-proof', 'manual-required'),
    addDevicePairingInput('storage', 'child-android-storage-protocol-capability-proof', 'scaffold'),
    addDevicePairingInput('protocol', 'child-android-storage-protocol-capability-proof', 'scaffold'),
    addDevicePairingInput('permission', 'child-android-permission-capability-proof', 'manual-required'),
    addDevicePairingInput('privileged', 'child-android-privileged-capability-proof', 'not-implemented'),
  ];
}

function addDevicePairingInput(input: string, source: string, readinessState: string) {
  return {
    input,
    source,
    readinessState,
    parentVisibleSummary: `${input} add-device input remains ${readinessState}`,
  };
}

function sourceProof(source: string) {
  return {
    source,
    outputPath: `test-results/${source}/proof.json`,
    command: `node scripts/test/${source}.mjs`,
    sourceStatus: 'ci-mechanical-proof',
  };
}

function artifactRequirements() {
  return [
    artifactRequirement(
      'debug-apk-build',
      'package-lifecycle',
      'manual-required',
      'ci-package-artifact',
      'ci-mechanical-proof',
      'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      'child-android-protocol-package-lifecycle-proof'
    ),
    artifactRequirement(
      'apk-sha256-checksum',
      'package-lifecycle',
      'manual-required',
      'ci-package-artifact',
      'ci-mechanical-proof',
      'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk.sha256',
      'child-android-protocol-package-lifecycle-proof'
    ),
    artifactRequirement(
      'package-local-status-bundles',
      'typed-protocol-bridge',
      'scaffold',
      'package-local-status',
      'package-local-scaffold',
      'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/ChildAndroidPrivilegedCapabilityProof.java',
      'child-android-privileged-capability-proof'
    ),
    ...manualDeviceRequirements(),
  ];
}

function manualDeviceRequirements() {
  return [
    ...runtimeArtifactRequirements(),
    ...privilegedAdapterArtifactRequirements(),
    ...enrollmentAndDistributionArtifactRequirements(),
  ];
}

function runtimeArtifactRequirements() {
  return [
    artifactRequirement(
      'real-device-install-artifact',
      'package-lifecycle',
      'manual-required',
      'emulator-device-artifact',
      'device-proof-required',
      null,
      'child-android-protocol-package-lifecycle-proof'
    ),
    artifactRequirement(
      'launch-activity-runtime-artifact',
      'package-lifecycle',
      'manual-required',
      'emulator-device-artifact',
      'device-proof-required',
      null,
      'child-android-protocol-package-lifecycle-proof'
    ),
    artifactRequirement(
      'foreground-service-runtime-artifact',
      'foreground-mobile-service',
      'manual-required',
      'emulator-device-artifact',
      'device-proof-required',
      null,
      'child-android-service-protocol-capability-proof'
    ),
    artifactRequirement(
      'notification-runtime-grant-artifact',
      'notifications',
      'manual-required',
      'permission-grant-artifact',
      'manual-required',
      null,
      'child-android-permission-capability-proof'
    ),
    artifactRequirement(
      'usage-stats-settings-grant-artifact',
      'usage-stats',
      'manual-required',
      'permission-grant-artifact',
      'settings-grant-required',
      null,
      'child-android-privileged-capability-proof'
    ),
    artifactRequirement(
      'usage-stats-observation-artifact',
      'usage-stats',
      'manual-required',
      'emulator-device-artifact',
      'device-proof-required',
      null,
      'child-android-privileged-capability-proof'
    ),
  ];
}

function privilegedAdapterArtifactRequirements() {
  return [
    artifactRequirement(
      'accessibility-service-grant-artifact',
      'accessibility-service',
      'not-implemented',
      'privileged-adapter-artifact',
      'not-implemented',
      null,
      'child-android-privileged-capability-proof'
    ),
    artifactRequirement(
      'vpn-service-grant-artifact',
      'vpn-dns-filtering',
      'not-implemented',
      'privileged-adapter-artifact',
      'not-implemented',
      null,
      'child-android-privileged-capability-proof'
    ),
    artifactRequirement(
      'dns-filtering-behavior-artifact',
      'vpn-dns-filtering',
      'not-implemented',
      'privileged-adapter-artifact',
      'not-implemented',
      null,
      'child-android-privileged-capability-proof'
    ),
  ];
}

function enrollmentAndDistributionArtifactRequirements() {
  return [
    artifactRequirement(
      'package-removal-artifact',
      'package-lifecycle',
      'manual-required',
      'emulator-device-artifact',
      'device-proof-required',
      null,
      'child-android-protocol-package-lifecycle-proof'
    ),
    artifactRequirement(
      'device-owner-enrollment-artifact',
      'device-owner-policy',
      'manual-required',
      'enrollment-artifact',
      'blocked',
      null,
      'child-android-privileged-capability-proof'
    ),
    artifactRequirement(
      'managed-profile-enrollment-artifact',
      'managed-profile',
      'manual-required',
      'enrollment-artifact',
      'blocked',
      null,
      'child-android-privileged-capability-proof'
    ),
    artifactRequirement(
      'play-store-signing-artifact',
      'store-distribution',
      'planned',
      'store-signing-artifact',
      'planned',
      null,
      'child-android-protocol-package-lifecycle-proof'
    ),
    artifactRequirement(
      'external-child-agent-transport-artifact',
      'typed-protocol-bridge',
      'not-implemented',
      'external-transport-artifact',
      'not-implemented',
      null,
      'child-android-privileged-capability-proof'
    ),
  ];
}

function artifactRequirement(
  requirement: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  artifactClass: string,
  artifactStatus: string,
  evidencePath: string | null,
  source: string
) {
  return {
    requirement,
    parentCapability,
    parentCapabilityStatus,
    artifactClass,
    artifactStatus,
    requiredArtifactSummary: `${requirement} remains ${artifactStatus}`,
    evidencePath,
    evidenceCapturedAt: evidencePath === null ? null : '2026-06-01T00:00:00.000Z',
    source,
  };
}

function requirementState(
  model: ChildAndroidDeviceProofArtifactGateReadModel,
  requirement: ChildAndroidDeviceProofArtifactGateReadModel['artifactRequirements'][number]['requirement']
) {
  const entry = model.artifactRequirements.find((candidate) => candidate.requirement === requirement);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    artifactClass: entry?.artifactClass,
    artifactStatus: entry?.artifactStatus,
    evidencePath: entry?.evidencePath,
  };
}
