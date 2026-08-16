/* generated from crates/schema/src/child_android_device_proof_artifact_gate_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildAndroidDeviceProofArtifactGateSchemaVersionSchema = withParser(
  Schema.Literal('child-android-device-proof-artifact-gate')
);
export const ChildAndroidDeviceProofSourceSchema = withParser(
  Schema.Literal(
    'child-android-protocol-package-lifecycle-proof',
    'child-android-storage-protocol-capability-proof',
    'child-android-service-protocol-capability-proof',
    'child-android-permission-capability-proof',
    'child-android-privileged-capability-proof'
  )
);
export const ChildAndroidDeviceProofReadinessDecisionSchema = withParser(
  Schema.Literal('manual-device-evidence-required-before-child-android-readiness')
);
export const ChildAndroidDeviceProofReadinessStateSchema = withParser(
  Schema.Literal('ci-package-only', 'manual-required', 'device-proof-required')
);
export const ChildAndroidInstallModeSchema = withParser(Schema.Literal('debug-apk-sideload'));
export const ChildAndroidChildAgentArtifactStateSchema = withParser(Schema.Literal('debug-apk-built'));
export const ChildAndroidInstallStateSchema = withParser(Schema.Literal('manual-install-proof-required'));
export const ChildAndroidLaunchStateSchema = withParser(Schema.Literal('manual-launch-proof-required'));
export const ChildAndroidRemovalStateSchema = withParser(Schema.Literal('manual-removal-proof-required'));
export const ChildAndroidPlatformAuthorityStateSchema = withParser(Schema.Literal('manual-required'));
export const ChildAndroidAddDevicePairingReadinessStateSchema = withParser(
  Schema.Literal('implemented', 'scaffold', 'manual-required', 'unavailable', 'not-implemented')
);
export const ChildAndroidAddDevicePairingReadinessSurfaceSchema = withParser(
  Schema.Literal('parent-add-device-pairing')
);
export const ChildAndroidAddDevicePairingReadinessInputSchema = withParser(
  Schema.Literal('package', 'service', 'storage', 'protocol', 'permission', 'privileged')
);
export const ChildAndroidDeviceProofArtifactClassSchema = withParser(
  Schema.Literal(
    'ci-package-artifact',
    'package-local-status',
    'emulator-device-artifact',
    'permission-grant-artifact',
    'privileged-adapter-artifact',
    'enrollment-artifact',
    'store-signing-artifact',
    'external-transport-artifact'
  )
);
export const ChildAndroidDeviceProofArtifactRequirementSchema = withParser(
  Schema.Literal(
    'debug-apk-build',
    'apk-sha256-checksum',
    'package-local-status-bundles',
    'real-device-install-artifact',
    'launch-activity-runtime-artifact',
    'foreground-service-runtime-artifact',
    'notification-runtime-grant-artifact',
    'usage-stats-settings-grant-artifact',
    'usage-stats-observation-artifact',
    'accessibility-service-grant-artifact',
    'vpn-service-grant-artifact',
    'dns-filtering-behavior-artifact',
    'package-removal-artifact',
    'device-owner-enrollment-artifact',
    'managed-profile-enrollment-artifact',
    'play-store-signing-artifact',
    'external-child-agent-transport-artifact'
  )
);
export const ChildAndroidDeviceProofArtifactStatusSchema = withParser(
  Schema.Literal(
    'ci-mechanical-proof',
    'package-local-scaffold',
    'manual-required',
    'device-proof-required',
    'settings-grant-required',
    'unavailable',
    'blocked',
    'not-implemented',
    'planned'
  )
);
export const ChildAndroidDeviceProofCustodyStateSchema = withParser(
  Schema.Literal('not-collected', 'ci-artifacts-only', 'ready-for-human-review', 'rejected-overclaim')
);

export const ChildAndroidDeviceProofPathSchema = brandedNonEmptyStringSchema('ChildAndroidDeviceProofPath');
export const ChildAndroidDeviceProofCommandSchema = brandedNonEmptyStringSchema('ChildAndroidDeviceProofCommand');
export const ChildAndroidDeviceProofSummarySchema = brandedNonEmptyStringSchema('ChildAndroidDeviceProofSummary');
export const ChildAndroidDeviceProofBoundarySchema = brandedNonEmptyStringSchema('ChildAndroidDeviceProofBoundary');

export const ChildAndroidDeviceProofSourceInputSchema = withParser(
  Schema.Struct({
    source: ChildAndroidDeviceProofSourceSchema,
    outputPath: ChildAndroidDeviceProofPathSchema,
    command: ChildAndroidDeviceProofCommandSchema,
    sourceStatus: ChildAndroidDeviceProofArtifactStatusSchema,
  })
);

export const ChildAndroidDeviceProofArtifactRequirementEvidenceSchema = withParser(
  Schema.Struct({
    requirement: ChildAndroidDeviceProofArtifactRequirementSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    artifactClass: ChildAndroidDeviceProofArtifactClassSchema,
    artifactStatus: ChildAndroidDeviceProofArtifactStatusSchema,
    requiredArtifactSummary: ChildAndroidDeviceProofSummarySchema,
    evidencePath: Schema.Union(ChildAndroidDeviceProofPathSchema, Schema.Null),
    evidenceCapturedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    source: ChildAndroidDeviceProofSourceSchema,
  })
);

export const ChildAndroidDeviceProofManualEvidenceCustodySchema = withParser(
  Schema.Struct({
    custodyState: ChildAndroidDeviceProofCustodyStateSchema,
    requiredArtifactCount: Schema.Number,
    ciArtifactCount: Schema.Number,
    collectedDeviceArtifactCount: Schema.Number,
    missingDeviceArtifactCount: Schema.Number,
    reviewerSummary: ChildAndroidDeviceProofBoundarySchema,
  })
);

export const ChildAndroidAddDevicePairingReadinessSchema = withParser(
  Schema.Struct({
    surface: ChildAndroidAddDevicePairingReadinessSurfaceSchema,
    readinessState: ChildAndroidAddDevicePairingReadinessStateSchema,
    inputs: Schema.Array(
      Schema.Struct({
        input: ChildAndroidAddDevicePairingReadinessInputSchema,
        source: ChildAndroidDeviceProofSourceSchema,
        readinessState: ChildAndroidAddDevicePairingReadinessStateSchema,
        parentVisibleSummary: ChildAndroidDeviceProofBoundarySchema,
      })
    ),
    parentVisibleSummary: ChildAndroidDeviceProofBoundarySchema,
  })
);

export const ChildAndroidInstallAuthorityStateSchema = withParser(
  Schema.Struct({
    childAgentArtifactState: ChildAndroidChildAgentArtifactStateSchema,
    installMode: ChildAndroidInstallModeSchema,
    installState: ChildAndroidInstallStateSchema,
    launchState: ChildAndroidLaunchStateSchema,
    removalState: ChildAndroidRemovalStateSchema,
    deviceOwnerAuthorityState: ChildAndroidPlatformAuthorityStateSchema,
    managedProfileAuthorityState: ChildAndroidPlatformAuthorityStateSchema,
    childAgentArtifactBoundary: ChildAndroidDeviceProofBoundarySchema,
    installModeBoundary: ChildAndroidDeviceProofBoundarySchema,
    installStateBoundary: ChildAndroidDeviceProofBoundarySchema,
    launchStateBoundary: ChildAndroidDeviceProofBoundarySchema,
    removalStateBoundary: ChildAndroidDeviceProofBoundarySchema,
    deviceOwnerBoundary: ChildAndroidDeviceProofBoundarySchema,
    managedProfileBoundary: ChildAndroidDeviceProofBoundarySchema,
  })
);

export const ChildAndroidDeviceProofClaimBoundariesSchema = withParser(
  Schema.Struct({
    addDevicePairingReadiness: ChildAndroidDeviceProofBoundarySchema,
    childAndroidDeviceReadiness: ChildAndroidDeviceProofBoundarySchema,
    emulatorRuntime: ChildAndroidDeviceProofBoundarySchema,
    physicalDeviceRuntime: ChildAndroidDeviceProofBoundarySchema,
    privilegedPermissions: ChildAndroidDeviceProofBoundarySchema,
    deviceOwnerManagedProfile: ChildAndroidDeviceProofBoundarySchema,
    playStoreSigning: ChildAndroidDeviceProofBoundarySchema,
    externalChildAgentTransport: ChildAndroidDeviceProofBoundarySchema,
  })
);

const ChildAndroidDeviceProofArtifactGateReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildAndroidDeviceProofArtifactGateSchemaVersionSchema,
  checkedAt: ParentTimestampSchema,
  readinessDecision: ChildAndroidDeviceProofReadinessDecisionSchema,
  packageMechanicalProofState: ChildAndroidDeviceProofReadinessStateSchema,
  installAuthorityState: ChildAndroidInstallAuthorityStateSchema,
  addDevicePairingReadiness: ChildAndroidAddDevicePairingReadinessSchema,
  childAndroidDeviceReadinessState: ChildAndroidDeviceProofReadinessStateSchema,
  sourceProofs: Schema.Array(ChildAndroidDeviceProofSourceInputSchema),
  artifactRequirements: Schema.Array(ChildAndroidDeviceProofArtifactRequirementEvidenceSchema),
  manualEvidenceStatus: ChildAndroidDeviceProofManualEvidenceCustodySchema,
  claimBoundaries: ChildAndroidDeviceProofClaimBoundariesSchema,
  claimsProved: Schema.Array(ChildAndroidDeviceProofSummarySchema),
  claimsNotProved: Schema.Array(ChildAndroidDeviceProofBoundarySchema),
});

type ChildAndroidDeviceProofArtifactGateReadModelCandidate = Infer<
  typeof ChildAndroidDeviceProofArtifactGateReadModelBaseSchema
>;

export const ChildAndroidDeviceProofArtifactGateReadModelSchema = withParser(
  ChildAndroidDeviceProofArtifactGateReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childAndroidDeviceProofArtifactGateIsHonest(readModel) ||
        'Expected Child Android device proof artifact gate to keep device readiness manual-required while only APK/checksum/package-local status bundles have CI or package-local evidence'
    )
  )
);

const RequiredSourceProofs = [
  'child-android-protocol-package-lifecycle-proof',
  'child-android-storage-protocol-capability-proof',
  'child-android-service-protocol-capability-proof',
  'child-android-permission-capability-proof',
  'child-android-privileged-capability-proof',
] as const satisfies ReadonlyArray<ChildAndroidDeviceProofSource>;

const RequiredArtifactRequirements = [
  'debug-apk-build',
  'apk-sha256-checksum',
  'package-local-status-bundles',
  'real-device-install-artifact',
  'launch-activity-runtime-artifact',
  'foreground-service-runtime-artifact',
  'notification-runtime-grant-artifact',
  'usage-stats-settings-grant-artifact',
  'usage-stats-observation-artifact',
  'accessibility-service-grant-artifact',
  'vpn-service-grant-artifact',
  'dns-filtering-behavior-artifact',
  'package-removal-artifact',
  'device-owner-enrollment-artifact',
  'managed-profile-enrollment-artifact',
  'play-store-signing-artifact',
  'external-child-agent-transport-artifact',
] as const satisfies ReadonlyArray<ChildAndroidDeviceProofArtifactRequirement>;

const RequirementExpectations = {
  'debug-apk-build': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'ci-package-artifact',
    artifactStatus: 'ci-mechanical-proof',
    source: 'child-android-protocol-package-lifecycle-proof',
  },
  'apk-sha256-checksum': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'ci-package-artifact',
    artifactStatus: 'ci-mechanical-proof',
    source: 'child-android-protocol-package-lifecycle-proof',
  },
  'package-local-status-bundles': {
    parentCapability: 'typed-protocol-bridge',
    parentCapabilityStatus: 'scaffold',
    artifactClass: 'package-local-status',
    artifactStatus: 'package-local-scaffold',
    source: 'child-android-privileged-capability-proof',
  },
  'real-device-install-artifact': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'emulator-device-artifact',
    artifactStatus: 'device-proof-required',
    source: 'child-android-protocol-package-lifecycle-proof',
  },
  'launch-activity-runtime-artifact': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'emulator-device-artifact',
    artifactStatus: 'device-proof-required',
    source: 'child-android-protocol-package-lifecycle-proof',
  },
  'foreground-service-runtime-artifact': {
    parentCapability: 'foreground-mobile-service',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'emulator-device-artifact',
    artifactStatus: 'device-proof-required',
    source: 'child-android-service-protocol-capability-proof',
  },
  'notification-runtime-grant-artifact': {
    parentCapability: 'notifications',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'permission-grant-artifact',
    artifactStatus: 'manual-required',
    source: 'child-android-permission-capability-proof',
  },
  'usage-stats-settings-grant-artifact': {
    parentCapability: 'usage-stats',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'permission-grant-artifact',
    artifactStatus: 'settings-grant-required',
    source: 'child-android-privileged-capability-proof',
  },
  'usage-stats-observation-artifact': {
    parentCapability: 'usage-stats',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'emulator-device-artifact',
    artifactStatus: 'device-proof-required',
    source: 'child-android-privileged-capability-proof',
  },
  'accessibility-service-grant-artifact': {
    parentCapability: 'accessibility-service',
    parentCapabilityStatus: 'not-implemented',
    artifactClass: 'privileged-adapter-artifact',
    artifactStatus: 'not-implemented',
    source: 'child-android-privileged-capability-proof',
  },
  'vpn-service-grant-artifact': {
    parentCapability: 'vpn-dns-filtering',
    parentCapabilityStatus: 'not-implemented',
    artifactClass: 'privileged-adapter-artifact',
    artifactStatus: 'not-implemented',
    source: 'child-android-privileged-capability-proof',
  },
  'dns-filtering-behavior-artifact': {
    parentCapability: 'vpn-dns-filtering',
    parentCapabilityStatus: 'not-implemented',
    artifactClass: 'privileged-adapter-artifact',
    artifactStatus: 'not-implemented',
    source: 'child-android-privileged-capability-proof',
  },
  'package-removal-artifact': {
    parentCapability: 'package-lifecycle',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'emulator-device-artifact',
    artifactStatus: 'device-proof-required',
    source: 'child-android-protocol-package-lifecycle-proof',
  },
  'device-owner-enrollment-artifact': {
    parentCapability: 'device-owner-policy',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'enrollment-artifact',
    artifactStatus: 'blocked',
    source: 'child-android-privileged-capability-proof',
  },
  'managed-profile-enrollment-artifact': {
    parentCapability: 'managed-profile',
    parentCapabilityStatus: 'manual-required',
    artifactClass: 'enrollment-artifact',
    artifactStatus: 'blocked',
    source: 'child-android-privileged-capability-proof',
  },
  'play-store-signing-artifact': {
    parentCapability: 'store-distribution',
    parentCapabilityStatus: 'planned',
    artifactClass: 'store-signing-artifact',
    artifactStatus: 'planned',
    source: 'child-android-protocol-package-lifecycle-proof',
  },
  'external-child-agent-transport-artifact': {
    parentCapability: 'typed-protocol-bridge',
    parentCapabilityStatus: 'not-implemented',
    artifactClass: 'external-transport-artifact',
    artifactStatus: 'not-implemented',
    source: 'child-android-privileged-capability-proof',
  },
} as const satisfies Record<
  ChildAndroidDeviceProofArtifactRequirement,
  Pick<
    ChildAndroidDeviceProofArtifactRequirementEvidence,
    'parentCapability' | 'parentCapabilityStatus' | 'artifactClass' | 'artifactStatus' | 'source'
  >
>;

const CiEvidenceStatuses = new Set<ChildAndroidDeviceProofArtifactStatus>([
  'ci-mechanical-proof',
  'package-local-scaffold',
]);

const AddDevicePairingInputExpectations = {
  package: {
    source: 'child-android-protocol-package-lifecycle-proof',
    readinessState: 'scaffold',
  },
  service: {
    source: 'child-android-service-protocol-capability-proof',
    readinessState: 'manual-required',
  },
  storage: {
    source: 'child-android-storage-protocol-capability-proof',
    readinessState: 'scaffold',
  },
  protocol: {
    source: 'child-android-storage-protocol-capability-proof',
    readinessState: 'scaffold',
  },
  permission: {
    source: 'child-android-permission-capability-proof',
    readinessState: 'manual-required',
  },
  privileged: {
    source: 'child-android-privileged-capability-proof',
    readinessState: 'not-implemented',
  },
} as const satisfies Record<
  ChildAndroidAddDevicePairingReadinessInput,
  Pick<ChildAndroidAddDevicePairingReadiness['inputs'][number], 'source' | 'readinessState'>
>;

function childAndroidDeviceProofArtifactGateIsHonest(
  readModel: ChildAndroidDeviceProofArtifactGateReadModelCandidate
): boolean {
  return (
    readModel.readinessDecision === 'manual-device-evidence-required-before-child-android-readiness' &&
    readModel.packageMechanicalProofState === 'ci-package-only' &&
    installAuthorityStateIsHonest(readModel.installAuthorityState) &&
    addDevicePairingReadinessIsHonest(readModel.addDevicePairingReadiness) &&
    readModel.childAndroidDeviceReadinessState === 'manual-required' &&
    sourceProofsAreComplete(readModel.sourceProofs) &&
    artifactRequirementsAreHonest(readModel.artifactRequirements) &&
    manualEvidenceCustodyIsHonest(readModel.manualEvidenceStatus, readModel.artifactRequirements) &&
    claimsStayInsideCiBoundary(readModel.claimsProved, readModel.claimsNotProved)
  );
}

function installAuthorityStateIsHonest(
  state: ChildAndroidDeviceProofArtifactGateReadModelCandidate['installAuthorityState']
): boolean {
  return [
    state.childAgentArtifactState === 'debug-apk-built',
    state.installMode === 'debug-apk-sideload',
    state.installState === 'manual-install-proof-required',
    state.launchState === 'manual-launch-proof-required',
    state.removalState === 'manual-removal-proof-required',
    state.deviceOwnerAuthorityState === 'manual-required',
    state.managedProfileAuthorityState === 'manual-required',
    state.childAgentArtifactBoundary.includes('child-agent artifact'),
    state.installModeBoundary.includes('debug APK sideload mode'),
    state.installStateBoundary.includes('manual-required'),
    state.launchStateBoundary.includes('manual-required'),
    state.removalStateBoundary.includes('manual-required'),
    state.deviceOwnerBoundary.includes('no device-owner claim'),
    state.managedProfileBoundary.includes('no managed-profile claim'),
  ].every(Boolean);
}

function addDevicePairingReadinessIsHonest(
  readiness: ChildAndroidDeviceProofArtifactGateReadModelCandidate['addDevicePairingReadiness']
): boolean {
  return (
    readiness.surface === 'parent-add-device-pairing' &&
    readiness.readinessState === 'manual-required' &&
    addDevicePairingInputsAreHonest(readiness.inputs) &&
    readiness.parentVisibleSummary.includes('add-device/pairing readiness remains manual-required')
  );
}

function addDevicePairingInputsAreHonest(
  inputs: ReadonlyArray<ChildAndroidAddDevicePairingReadiness['inputs'][number]>
): boolean {
  const byInput = new Map(inputs.map((entry) => [entry.input, entry] as const));
  return (
    byInput.size === inputs.length &&
    Object.entries(AddDevicePairingInputExpectations).every(([input, expected]) => {
      const entry = byInput.get(input as ChildAndroidAddDevicePairingReadinessInput);
      return (
        entry?.source === expected.source &&
        entry.readinessState === expected.readinessState &&
        entry.parentVisibleSummary.includes(entry.readinessState)
      );
    })
  );
}

function sourceProofsAreComplete(proofs: ReadonlyArray<ChildAndroidDeviceProofSourceInput>): boolean {
  const bySource = new Map(proofs.map((proof) => [proof.source, proof] as const));
  return (
    bySource.size === proofs.length &&
    RequiredSourceProofs.every((source) => bySource.get(source)?.sourceStatus === 'ci-mechanical-proof')
  );
}

function artifactRequirementsAreHonest(
  requirements: ReadonlyArray<ChildAndroidDeviceProofArtifactRequirementEvidence>
): boolean {
  const byRequirement = new Map(requirements.map((entry) => [entry.requirement, entry] as const));
  return (
    byRequirement.size === requirements.length &&
    RequiredArtifactRequirements.every((requirement) =>
      artifactRequirementIsHonest(byRequirement.get(requirement), requirement)
    )
  );
}

function artifactRequirementIsHonest(
  evidence: ChildAndroidDeviceProofArtifactRequirementEvidence | undefined,
  requirement: ChildAndroidDeviceProofArtifactRequirement
): boolean {
  const expected = RequirementExpectations[requirement];
  return Boolean(
    evidence &&
    evidence.requirement === requirement &&
    evidence.parentCapability === expected.parentCapability &&
    evidence.parentCapabilityStatus === expected.parentCapabilityStatus &&
    evidence.artifactClass === expected.artifactClass &&
    evidence.artifactStatus === expected.artifactStatus &&
    evidence.source === expected.source &&
    artifactEvidenceReferenceIsHonest(evidence)
  );
}

function artifactEvidenceReferenceIsHonest(evidence: ChildAndroidDeviceProofArtifactRequirementEvidence): boolean {
  if (CiEvidenceStatuses.has(evidence.artifactStatus)) {
    return evidence.evidencePath !== null && evidence.evidenceCapturedAt !== null;
  }
  return evidence.evidencePath === null && evidence.evidenceCapturedAt === null;
}

function manualEvidenceCustodyIsHonest(
  status: ChildAndroidDeviceProofManualEvidenceCustody,
  requirements: ReadonlyArray<ChildAndroidDeviceProofArtifactRequirementEvidence>
): boolean {
  const ciArtifactCount = requirements.filter((requirement) =>
    CiEvidenceStatuses.has(requirement.artifactStatus)
  ).length;
  return (
    status.custodyState === 'ci-artifacts-only' &&
    status.requiredArtifactCount === RequiredArtifactRequirements.length &&
    status.ciArtifactCount === ciArtifactCount &&
    status.collectedDeviceArtifactCount === 0 &&
    status.missingDeviceArtifactCount === RequiredArtifactRequirements.length - ciArtifactCount
  );
}

function claimsStayInsideCiBoundary(
  claimsProved: ReadonlyArray<ChildAndroidDeviceProofSummary>,
  claimsNotProved: ReadonlyArray<ChildAndroidDeviceProofBoundary>
): boolean {
  return (
    claimsProved.length === 1 &&
    claimsProved[0] === 'debug APK, checksum, and package-local status bundles are CI/package proof only' &&
    claimsNotProved.some((claim) => claim.includes('Android add-device/pairing readiness remains manual-required')) &&
    claimsNotProved.some((claim) => claim.includes('Android child device readiness remains manual-required')) &&
    claimsNotProved.some((claim) => claim.includes('Android child enforcement parity is not proved'))
  );
}

export type ChildAndroidDeviceProofSource = Infer<typeof ChildAndroidDeviceProofSourceSchema>;
export type ChildAndroidDeviceProofReadinessDecision = Infer<typeof ChildAndroidDeviceProofReadinessDecisionSchema>;
export type ChildAndroidDeviceProofReadinessState = Infer<typeof ChildAndroidDeviceProofReadinessStateSchema>;
export type ChildAndroidInstallMode = Infer<typeof ChildAndroidInstallModeSchema>;
export type ChildAndroidChildAgentArtifactState = Infer<typeof ChildAndroidChildAgentArtifactStateSchema>;
export type ChildAndroidInstallState = Infer<typeof ChildAndroidInstallStateSchema>;
export type ChildAndroidLaunchState = Infer<typeof ChildAndroidLaunchStateSchema>;
export type ChildAndroidRemovalState = Infer<typeof ChildAndroidRemovalStateSchema>;
export type ChildAndroidPlatformAuthorityState = Infer<typeof ChildAndroidPlatformAuthorityStateSchema>;
export type ChildAndroidAddDevicePairingReadinessState = Infer<typeof ChildAndroidAddDevicePairingReadinessStateSchema>;
export type ChildAndroidAddDevicePairingReadinessSurface = Infer<
  typeof ChildAndroidAddDevicePairingReadinessSurfaceSchema
>;
export type ChildAndroidAddDevicePairingReadinessInput = Infer<typeof ChildAndroidAddDevicePairingReadinessInputSchema>;
export type ChildAndroidDeviceProofArtifactClass = Infer<typeof ChildAndroidDeviceProofArtifactClassSchema>;
export type ChildAndroidDeviceProofArtifactRequirement = Infer<typeof ChildAndroidDeviceProofArtifactRequirementSchema>;
export type ChildAndroidDeviceProofArtifactStatus = Infer<typeof ChildAndroidDeviceProofArtifactStatusSchema>;
export type ChildAndroidDeviceProofCustodyState = Infer<typeof ChildAndroidDeviceProofCustodyStateSchema>;
export type ChildAndroidDeviceProofPath = Infer<typeof ChildAndroidDeviceProofPathSchema>;
export type ChildAndroidDeviceProofCommand = Infer<typeof ChildAndroidDeviceProofCommandSchema>;
export type ChildAndroidDeviceProofSummary = Infer<typeof ChildAndroidDeviceProofSummarySchema>;
export type ChildAndroidDeviceProofBoundary = Infer<typeof ChildAndroidDeviceProofBoundarySchema>;
export type ChildAndroidDeviceProofSourceInput = Infer<typeof ChildAndroidDeviceProofSourceInputSchema>;
export type ChildAndroidDeviceProofArtifactRequirementEvidence = Infer<
  typeof ChildAndroidDeviceProofArtifactRequirementEvidenceSchema
>;
export type ChildAndroidDeviceProofManualEvidenceCustody = Infer<
  typeof ChildAndroidDeviceProofManualEvidenceCustodySchema
>;
export type ChildAndroidAddDevicePairingReadiness = Infer<typeof ChildAndroidAddDevicePairingReadinessSchema>;
export type ChildAndroidInstallAuthorityState = Infer<typeof ChildAndroidInstallAuthorityStateSchema>;
export type ChildAndroidDeviceProofClaimBoundaries = Infer<typeof ChildAndroidDeviceProofClaimBoundariesSchema>;
export type ChildAndroidDeviceProofArtifactGateReadModel = Infer<
  typeof ChildAndroidDeviceProofArtifactGateReadModelSchema
>;
