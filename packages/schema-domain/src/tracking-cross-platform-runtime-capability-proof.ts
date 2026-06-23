import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
const CountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingCrossPlatformRuntimeCapabilityAreaSchema = withParser(
  Schema.Literal(
    'windows-host-toolchain',
    'wsl-linux-replay',
    'docker-container-runtime',
    'android-sdk-toolchain',
    'android-gradle-project-build',
    'android-emulator-runtime',
    'android-physical-device-status',
    'macos-ios-ci-manual-routing'
  )
);

export const TrackingCrossPlatformRuntimeCapabilityStatusSchema = withParser(
  Schema.Literal('local-proof-passed', 'ci-manual-required', 'host-tool-unavailable')
);

export const TrackingCrossPlatformRuntimeCapabilityRefSchema = brandedNonEmptyStringSchema(
  'TrackingCrossPlatformRuntimeCapabilityRef'
);

export const RequiredTrackingCrossPlatformRuntimeCapabilityAreas = [
  'windows-host-toolchain',
  'wsl-linux-replay',
  'docker-container-runtime',
  'android-sdk-toolchain',
  'android-gradle-project-build',
  'android-emulator-runtime',
  'android-physical-device-status',
  'macos-ios-ci-manual-routing',
] as const;

export const TrackingCrossPlatformRuntimeCapabilityRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    area: TrackingCrossPlatformRuntimeCapabilityAreaSchema,
    status: TrackingCrossPlatformRuntimeCapabilityStatusSchema,
    generatedAt: ParentTimestampSchema,
    proofRef: TrackingCrossPlatformRuntimeCapabilityRefSchema,
    sourceRefs: Schema.Array(TrackingCrossPlatformRuntimeCapabilityRefSchema),
    currentProofTier: NonEmptyStringSchema,
    requiredProofTier: NonEmptyStringSchema,
    observedTooling: Schema.Array(NonEmptyStringSchema),
    observedCapabilityRefs: Schema.Array(NonEmptyStringSchema),
    passedAssertions: Schema.Array(NonEmptyStringSchema),
    remainingBlockers: Schema.Array(NonEmptyStringSchema),
    artifactCount: CountSchema,
    ciRunnable: Schema.Boolean,
    localRuntimeClaimed: Schema.Boolean,
    physicalDeviceBehaviorClaimed: Schema.Literal(false),
    authorityRuntimeClaimed: Schema.Literal(false),
    productionRuntimeClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (row) =>
        row.passedAssertions.length > 0 ||
        row.remainingBlockers.length > 0 ||
        'Cross-platform capability rows need assertions or blockers'
    )
  )
);

export const TrackingCrossPlatformRuntimeCapabilityProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-cross-platform-runtime-capability-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingCrossPlatformRuntimeCapabilityRowSchema).pipe(
      Schema.minItems(RequiredTrackingCrossPlatformRuntimeCapabilityAreas.length)
    ),
    summary: Schema.Struct({
      rowCount: CountSchema,
      localProofPassedRows: CountSchema,
      ciManualRequiredRows: CountSchema,
      hostToolUnavailableRows: CountSchema,
      ciRunnableRows: CountSchema,
      localRuntimeClaimedRows: CountSchema,
      physicalDeviceBehaviorClaimedRows: Schema.Literal(0),
      authorityRuntimeClaimedRows: Schema.Literal(0),
      productionRuntimeClaimedRows: Schema.Literal(0),
      productReadyRows: Schema.Literal(0),
    }),
    productClaims: Schema.Struct({
      windowsHostToolchainObserved: Schema.Boolean,
      wslLinuxReplayObserved: Schema.Boolean,
      dockerContainerRuntimeObserved: Schema.Boolean,
      androidSdkToolchainObserved: Schema.Boolean,
      androidGradleProjectBuildObserved: Schema.Boolean,
      androidEmulatorRuntimeObserved: Schema.Boolean,
      androidPhysicalStatusObserved: Schema.Boolean,
      androidPhysicalGeofenceRegistrationObserved: Schema.Boolean,
      androidPhysicalSystemProximityRegistrationObserved: Schema.Boolean,
      macosIosCiRoutingPresent: Schema.Boolean,
      physicalDeviceBehaviorClaimed: Schema.Literal(false),
      authorityRuntimeClaimed: Schema.Literal(false),
      productionRuntimeClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        proof.rows.length === RequiredTrackingCrossPlatformRuntimeCapabilityAreas.length ||
        'Cross-platform runtime proof must include every required area'
    )
  )
);

export type TrackingCrossPlatformRuntimeCapabilityArea = Infer<typeof TrackingCrossPlatformRuntimeCapabilityAreaSchema>;
export type TrackingCrossPlatformRuntimeCapabilityProof = Infer<
  typeof TrackingCrossPlatformRuntimeCapabilityProofSchema
>;
export type TrackingCrossPlatformRuntimeCapabilityRow = Infer<typeof TrackingCrossPlatformRuntimeCapabilityRowSchema>;

export type TrackingCrossPlatformRuntimeCapabilityRowInput = {
  readonly area: TrackingCrossPlatformRuntimeCapabilityArea;
  readonly status: 'local-proof-passed' | 'ci-manual-required' | 'host-tool-unavailable';
  readonly proofRef: string;
  readonly sourceRefs: readonly string[];
  readonly currentProofTier: string;
  readonly requiredProofTier: string;
  readonly observedTooling: readonly string[];
  readonly observedCapabilityRefs?: readonly string[];
  readonly passedAssertions: readonly string[];
  readonly remainingBlockers: readonly string[];
  readonly artifactCount: number;
  readonly ciRunnable: boolean;
  readonly localRuntimeClaimed: boolean;
};

export function buildTrackingCrossPlatformRuntimeCapabilityProof(
  generatedAt: string,
  rowsInput: readonly TrackingCrossPlatformRuntimeCapabilityRowInput[]
): TrackingCrossPlatformRuntimeCapabilityProof {
  const rows = RequiredTrackingCrossPlatformRuntimeCapabilityAreas.map((area) => {
    const input = rowsInput.find((candidate) => candidate.area === area);
    if (!input) throw new Error(`Missing cross-platform runtime capability area: ${area}`);
    return row(generatedAt, input);
  });
  const summary = summaryFrom(rows);

  return TrackingCrossPlatformRuntimeCapabilityProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-cross-platform-runtime-capability-proof',
    generatedAt,
    rows,
    summary,
    productClaims: {
      windowsHostToolchainObserved: hasPassed(rows, 'windows-host-toolchain'),
      wslLinuxReplayObserved: hasPassed(rows, 'wsl-linux-replay'),
      dockerContainerRuntimeObserved: hasPassed(rows, 'docker-container-runtime'),
      androidSdkToolchainObserved: hasPassed(rows, 'android-sdk-toolchain'),
      androidGradleProjectBuildObserved: hasPassed(rows, 'android-gradle-project-build'),
      androidEmulatorRuntimeObserved: hasPassed(rows, 'android-emulator-runtime'),
      androidPhysicalStatusObserved: hasPassed(rows, 'android-physical-device-status'),
      androidPhysicalGeofenceRegistrationObserved: hasObservedCapability(
        rows,
        'android-physical-geofence-registration'
      ),
      androidPhysicalSystemProximityRegistrationObserved: hasObservedCapability(
        rows,
        'android-physical-system-proximity-registration'
      ),
      macosIosCiRoutingPresent: rows.some(
        (candidate) => candidate.area === 'macos-ios-ci-manual-routing' && candidate.status === 'ci-manual-required'
      ),
      physicalDeviceBehaviorClaimed: false,
      authorityRuntimeClaimed: false,
      productionRuntimeClaimed: false,
      productClaimReady: false,
    },
  });
}

function row(
  generatedAt: string,
  input: TrackingCrossPlatformRuntimeCapabilityRowInput
): TrackingCrossPlatformRuntimeCapabilityRow {
  return TrackingCrossPlatformRuntimeCapabilityRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    area: input.area,
    status: input.status,
    generatedAt,
    proofRef: input.proofRef,
    sourceRefs: [...new Set(input.sourceRefs)],
    currentProofTier: input.currentProofTier,
    requiredProofTier: input.requiredProofTier,
    observedTooling: [...input.observedTooling],
    observedCapabilityRefs: [...new Set(input.observedCapabilityRefs ?? [])],
    passedAssertions: [...input.passedAssertions],
    remainingBlockers: [...input.remainingBlockers],
    artifactCount: input.artifactCount,
    ciRunnable: input.ciRunnable,
    localRuntimeClaimed: input.localRuntimeClaimed,
    physicalDeviceBehaviorClaimed: false,
    authorityRuntimeClaimed: false,
    productionRuntimeClaimed: false,
    productClaimReady: false,
  });
}

function hasObservedCapability(
  rows: readonly TrackingCrossPlatformRuntimeCapabilityRow[],
  capabilityRef: string
): boolean {
  return rows.some((row) => row.observedCapabilityRefs.includes(capabilityRef));
}

function summaryFrom(rows: readonly TrackingCrossPlatformRuntimeCapabilityRow[]) {
  return {
    rowCount: rows.length,
    localProofPassedRows: rows.filter((row) => row.status === 'local-proof-passed').length,
    ciManualRequiredRows: rows.filter((row) => row.status === 'ci-manual-required').length,
    hostToolUnavailableRows: rows.filter((row) => row.status === 'host-tool-unavailable').length,
    ciRunnableRows: rows.filter((row) => row.ciRunnable).length,
    localRuntimeClaimedRows: rows.filter((row) => row.localRuntimeClaimed).length,
    physicalDeviceBehaviorClaimedRows: 0,
    authorityRuntimeClaimedRows: 0,
    productionRuntimeClaimedRows: 0,
    productReadyRows: 0,
  } as const;
}

function hasPassed(
  rows: readonly TrackingCrossPlatformRuntimeCapabilityRow[],
  area: TrackingCrossPlatformRuntimeCapabilityArea
): boolean {
  return rows.some((row) => row.area === area && row.status === 'local-proof-passed');
}
