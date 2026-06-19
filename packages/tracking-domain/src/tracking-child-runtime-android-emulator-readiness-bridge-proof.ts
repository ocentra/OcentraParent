import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
const TrackingChildRuntimeAndroidEmulatorBridgeCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingChildRuntimeAndroidEmulatorBridgeRefSchema =
  brandedNonEmptyStringSchema('TrackingChildRuntimeAndroidEmulatorBridgeRef');

export const TrackingChildRuntimeAndroidEmulatorBridgeRowIdSchema =
  brandedNonEmptyStringSchema('TrackingChildRuntimeAndroidEmulatorBridgeRowId');

export const TrackingChildRuntimeAndroidEmulatorBridgeStatusSchema = Schema.Literal(
  'emulator-prerequisites-observed-manual-runtime-required'
);

export const RequiredTrackingChildRuntimeAndroidEmulatorBridgeSourceRefs = [
  'test-results/tracking-plan-android-emulator-proof/proof.json',
  'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json',
] as const;

export const TrackingChildRuntimeAndroidEmulatorBridgeInputSchema = withParser(
  Schema.Struct({
    androidEmulatorProofRef: TrackingChildRuntimeAndroidEmulatorBridgeRefSchema,
    childRuntimeArtifactGateProofRef: TrackingChildRuntimeAndroidEmulatorBridgeRefSchema,
    androidProofStatus: TrackingChildRuntimeAndroidEmulatorBridgeRefSchema,
    packageLaunchObserved: Schema.Boolean,
    foregroundServiceObserved: Schema.Boolean,
    foregroundPermissionGranted: Schema.Boolean,
    backgroundPermissionGranted: Schema.Boolean,
    localGeofenceTransitionCount: TrackingChildRuntimeAndroidEmulatorBridgeCountSchema,
    androidEvidenceRefs: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema).pipe(Schema.minItems(1)),
    childRuntimeRequiredArtifacts: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema).pipe(
      Schema.minItems(1)
    ),
    childRuntimePresentArtifacts: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema),
    childRuntimeMissingArtifacts: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema).pipe(
      Schema.minItems(1)
    ),
  })
);

const TrackingChildRuntimeAndroidEmulatorBridgeRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  rowId: TrackingChildRuntimeAndroidEmulatorBridgeRowIdSchema,
  generatedAt: ParentTimestampSchema,
  requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
  currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
  status: TrackingChildRuntimeAndroidEmulatorBridgeStatusSchema,
  sourceProofRefs: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema).pipe(Schema.minItems(2)),
  androidEmulatorProofRef: TrackingChildRuntimeAndroidEmulatorBridgeRefSchema,
  childRuntimeArtifactGateProofRef: TrackingChildRuntimeAndroidEmulatorBridgeRefSchema,
  androidProofStatus: TrackingChildRuntimeAndroidEmulatorBridgeRefSchema,
  androidEvidenceRefs: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema).pipe(Schema.minItems(1)),
  childRuntimeRequiredArtifacts: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema).pipe(
    Schema.minItems(1)
  ),
  childRuntimePresentArtifacts: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema),
  childRuntimeMissingArtifacts: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema).pipe(
    Schema.minItems(1)
  ),
  missingProofReasonRefs: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRefSchema).pipe(Schema.minItems(1)),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema).pipe(Schema.minItems(1)),
  packageLaunchObserved: Schema.Literal(true),
  foregroundServiceObserved: Schema.Literal(true),
  foregroundPermissionGranted: Schema.Boolean,
  backgroundPermissionGranted: Schema.Boolean,
  localGeofenceTransitionCount: TrackingChildRuntimeAndroidEmulatorBridgeCountSchema,
  emulatorPrerequisitesObserved: Schema.Literal(true),
  childRuntimeArtifactSetComplete: Schema.Literal(false),
  childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
  childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
  renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
  parentReceiptRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingChildRuntimeAndroidEmulatorBridgeRowSchema = withParser(
  TrackingChildRuntimeAndroidEmulatorBridgeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingChildRuntimeAndroidEmulatorBridgeRowIsHonest(row) ||
        'Expected child-runtime Android emulator bridge rows to cite emulator evidence and child-runtime missing artifacts without claiming physical child-device runtime'
    )
  )
);

export const TrackingChildRuntimeAndroidEmulatorBridgeProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-child-runtime-android-emulator-readiness-bridge-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingChildRuntimeAndroidEmulatorBridgeRowSchema).pipe(Schema.minItems(1)),
    proofClaims: Schema.Struct({
      androidEmulatorPrerequisitesObserved: Schema.Literal(true),
      childRuntimeArtifactGateLinked: Schema.Literal(true),
      childRuntimePhysicalProofStillRequired: Schema.Literal(true),
      noChildDeviceDeliveryRuntimeClaim: Schema.Literal(true),
      noChildDeviceExecutionRuntimeClaim: Schema.Literal(true),
      noRenderedChildDeviceUiRuntimeClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryClaim: Schema.Literal(true),
      noProductionClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      androidEmulatorPrerequisitesObserved: Schema.Literal(true),
      childRuntimeArtifactSetComplete: Schema.Literal(false),
      childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
      childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
      renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
      parentReceiptRuntimeClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        proof.rows.every((row) => row.sourceProofRefs.length >= 2 && row.childRuntimeMissingArtifacts.length > 0) ||
        'Expected child-runtime Android emulator bridge proof to retain source proof refs and missing child-runtime artifacts'
    )
  )
);

export type TrackingChildRuntimeAndroidEmulatorBridgeInput = Infer<
  typeof TrackingChildRuntimeAndroidEmulatorBridgeInputSchema
>;
export type TrackingChildRuntimeAndroidEmulatorBridgeProof = Infer<
  typeof TrackingChildRuntimeAndroidEmulatorBridgeProofSchema
>;
type TrackingChildRuntimeAndroidEmulatorBridgeRowInput = Infer<
  typeof TrackingChildRuntimeAndroidEmulatorBridgeRowBaseSchema
>;

export function buildTrackingChildRuntimeAndroidEmulatorBridgeProof(
  generatedAt: string,
  input: TrackingChildRuntimeAndroidEmulatorBridgeInput
): TrackingChildRuntimeAndroidEmulatorBridgeProof {
  const parsedInput = TrackingChildRuntimeAndroidEmulatorBridgeInputSchema.parse(input);

  return TrackingChildRuntimeAndroidEmulatorBridgeProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-child-runtime-android-emulator-readiness-bridge-proof',
    generatedAt,
    rows: [bridgeRow(generatedAt, parsedInput)],
    proofClaims: {
      androidEmulatorPrerequisitesObserved: true,
      childRuntimeArtifactGateLinked: true,
      childRuntimePhysicalProofStillRequired: true,
      noChildDeviceDeliveryRuntimeClaim: true,
      noChildDeviceExecutionRuntimeClaim: true,
      noRenderedChildDeviceUiRuntimeClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      androidEmulatorPrerequisitesObserved: true,
      childRuntimeArtifactSetComplete: false,
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      parentReceiptRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function bridgeRow(generatedAt: string, input: TrackingChildRuntimeAndroidEmulatorBridgeInput) {
  return TrackingChildRuntimeAndroidEmulatorBridgeRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-child-runtime-android-emulator-readiness-bridge',
    generatedAt,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'emulator-prerequisites-observed-manual-runtime-required',
    sourceProofRefs: [input.androidEmulatorProofRef, input.childRuntimeArtifactGateProofRef],
    androidEmulatorProofRef: input.androidEmulatorProofRef,
    childRuntimeArtifactGateProofRef: input.childRuntimeArtifactGateProofRef,
    androidProofStatus: input.androidProofStatus,
    androidEvidenceRefs: input.androidEvidenceRefs,
    childRuntimeRequiredArtifacts: input.childRuntimeRequiredArtifacts,
    childRuntimePresentArtifacts: input.childRuntimePresentArtifacts,
    childRuntimeMissingArtifacts: input.childRuntimeMissingArtifacts,
    missingProofReasonRefs: [
      'child-runtime-delivery-envelope-physical-run-required',
      'child-runtime-execution-result-physical-run-required',
      'rendered-child-device-ui-snapshot-physical-run-required',
      'parent-receipt-runtime-physical-run-required',
    ],
    auditRefs: ['tracking-child-runtime-android-emulator-readiness-bridge-audit'],
    packageLaunchObserved: input.packageLaunchObserved,
    foregroundServiceObserved: input.foregroundServiceObserved,
    foregroundPermissionGranted: input.foregroundPermissionGranted,
    backgroundPermissionGranted: input.backgroundPermissionGranted,
    localGeofenceTransitionCount: input.localGeofenceTransitionCount,
    emulatorPrerequisitesObserved: true,
    childRuntimeArtifactSetComplete: false,
    childDeviceDeliveryRuntimeClaimed: false,
    childDeviceExecutionRuntimeClaimed: false,
    renderedChildDeviceUiRuntimeClaimed: false,
    parentReceiptRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function trackingChildRuntimeAndroidEmulatorBridgeRowIsHonest(
  row: TrackingChildRuntimeAndroidEmulatorBridgeRowInput
): boolean {
  return (
    trackingChildRuntimeAndroidEmulatorBridgeRefsAreHonest(row) &&
    trackingChildRuntimeAndroidEmulatorBridgePrereqsAreHonest(row) &&
    trackingChildRuntimeAndroidEmulatorBridgeNonClaimsAreHonest(row)
  );
}

function trackingChildRuntimeAndroidEmulatorBridgeRefsAreHonest(
  row: TrackingChildRuntimeAndroidEmulatorBridgeRowInput
): boolean {
  return (
    row.sourceProofRefs.includes(row.androidEmulatorProofRef) &&
    row.sourceProofRefs.includes(row.childRuntimeArtifactGateProofRef) &&
    row.androidEvidenceRefs.length > 0 &&
    row.childRuntimeRequiredArtifacts.length > 0 &&
    row.childRuntimeRequiredArtifacts.length ===
      row.childRuntimePresentArtifacts.length + row.childRuntimeMissingArtifacts.length &&
    row.childRuntimeMissingArtifacts.length > 0 &&
    row.missingProofReasonRefs.length > 0
  );
}

function trackingChildRuntimeAndroidEmulatorBridgePrereqsAreHonest(
  row: TrackingChildRuntimeAndroidEmulatorBridgeRowInput
): boolean {
  return (
    row.packageLaunchObserved === true &&
    row.foregroundServiceObserved === true &&
    row.emulatorPrerequisitesObserved === true &&
    row.childRuntimeArtifactSetComplete === false
  );
}

function trackingChildRuntimeAndroidEmulatorBridgeNonClaimsAreHonest(
  row: TrackingChildRuntimeAndroidEmulatorBridgeRowInput
): boolean {
  return (
    row.childDeviceDeliveryRuntimeClaimed === false &&
    row.childDeviceExecutionRuntimeClaimed === false &&
    row.renderedChildDeviceUiRuntimeClaimed === false &&
    row.parentReceiptRuntimeClaimed === false &&
    row.physicalDeviceProofClaimed === false &&
    row.authorityProofClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.productionWorkerClaimed === false &&
    row.productClaimReady === false
  );
}

