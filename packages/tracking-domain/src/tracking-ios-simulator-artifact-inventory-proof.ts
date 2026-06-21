import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from '@ocentra-parent/schema-domain/tracking-location-policy-primitives';
const TrackingIosSimulatorArtifactInventoryCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingIosSimulatorArtifactInventoryRefSchema = brandedNonEmptyStringSchema('TrackingIosSimulatorArtifactInventoryRef');

export const TrackingIosSimulatorArtifactInventoryRowIdSchema = brandedNonEmptyStringSchema('TrackingIosSimulatorArtifactInventoryRowId');

export const TrackingIosSimulatorArtifactInventoryStatusSchema = Schema.Literal(
  'ios-simulator-local-artifacts-present-physical-device-required'
);

export const TrackingIosSimulatorArtifactInventoryCategorySchema = Schema.Literal(
  'simulator-package-proof',
  'location-manual-required-proof',
  'privacy-disclosure-proof',
  'platform-proof',
  'validation-log'
);

export const RequiredTrackingIosSimulatorArtifactRefs = [
  'test-results/tracking-plan-ios-simulator-proof/proof.json',
  'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/18-ios-simulator-proof.json',
  'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/18-ios-simulator-proof.json',
  'output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/18-ios-simulator-proof.json',
  'test-results/tracking-ios-location-manual-required-proof/proof.json',
  'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/19-ios-location-manual-required-proof.json',
  'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/19-ios-location-manual-required-proof.json',
  'test-results/tracking-ios-privacy-disclosure-release-proof/proof.json',
  'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/20-ios-privacy-disclosure-release-proof.json',
  'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/02-platform-permission-proof.md',
  'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/02-platform-permission-proof.md',
  'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/16-validation-commands.log',
  'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/16-validation-commands.log',
] as const;

const TrackingIosSimulatorArtifactInventoryArtifactRowSchema = Schema.Struct({
  artifactRef: TrackingIosSimulatorArtifactInventoryRefSchema,
  category: TrackingIosSimulatorArtifactInventoryCategorySchema,
  required: Schema.Literal(true),
  present: Schema.Boolean,
  byteSize: TrackingIosSimulatorArtifactInventoryCountSchema,
});

export const TrackingIosSimulatorArtifactInventoryInputSchema = withParser(
  Schema.Struct({
    sourceIosSimulatorProofRef: TrackingIosSimulatorArtifactInventoryRefSchema,
    iosSimulatorProofStatus: TrackingIosSimulatorArtifactInventoryRefSchema,
    iosSimulatorCurrentProofTier: TrackingIosSimulatorArtifactInventoryRefSchema,
    hostPlatform: TrackingIosSimulatorArtifactInventoryRefSchema,
    hostArch: TrackingIosSimulatorArtifactInventoryRefSchema,
    canRunXcodeSimulator: Schema.Boolean,
    iosManualRequiredRowCount: TrackingIosSimulatorArtifactInventoryCountSchema,
    iosRequiredRuntimeArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
    iosPresentRuntimeArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
    iosMissingRuntimeArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
    privacyReleaseGateRowCount: TrackingIosSimulatorArtifactInventoryCountSchema,
    privacyReleaseBlockedCount: TrackingIosSimulatorArtifactInventoryCountSchema,
    artifactRows: Schema.Array(TrackingIosSimulatorArtifactInventoryArtifactRowSchema).pipe(
      Schema.minItems(RequiredTrackingIosSimulatorArtifactRefs.length)
    ),
  })
);

const TrackingIosSimulatorArtifactInventoryRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  rowId: TrackingIosSimulatorArtifactInventoryRowIdSchema,
  generatedAt: ParentTimestampSchema,
  requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
  currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
  status: TrackingIosSimulatorArtifactInventoryStatusSchema,
  sourceIosSimulatorProofRef: TrackingIosSimulatorArtifactInventoryRefSchema,
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema).pipe(Schema.minItems(1)),
  iosSimulatorProofStatus: TrackingIosSimulatorArtifactInventoryRefSchema,
  iosSimulatorCurrentProofTier: TrackingIosSimulatorArtifactInventoryRefSchema,
  hostPlatform: TrackingIosSimulatorArtifactInventoryRefSchema,
  hostArch: TrackingIosSimulatorArtifactInventoryRefSchema,
  canRunXcodeSimulator: Schema.Boolean,
  requiredArtifacts: Schema.Array(TrackingIosSimulatorArtifactInventoryRefSchema).pipe(
    Schema.minItems(RequiredTrackingIosSimulatorArtifactRefs.length)
  ),
  presentArtifacts: Schema.Array(TrackingIosSimulatorArtifactInventoryRefSchema),
  missingArtifacts: Schema.Array(TrackingIosSimulatorArtifactInventoryRefSchema),
  artifactRows: Schema.Array(TrackingIosSimulatorArtifactInventoryArtifactRowSchema).pipe(
    Schema.minItems(RequiredTrackingIosSimulatorArtifactRefs.length)
  ),
  iosManualRequiredRowCount: TrackingIosSimulatorArtifactInventoryCountSchema,
  iosRequiredRuntimeArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
  iosPresentRuntimeArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
  iosMissingRuntimeArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
  privacyReleaseGateRowCount: TrackingIosSimulatorArtifactInventoryCountSchema,
  privacyReleaseBlockedCount: TrackingIosSimulatorArtifactInventoryCountSchema,
  simulatorArtifactInventoryComplete: Schema.Boolean,
  coreLocationRuntimeClaimed: Schema.Literal(false),
  backgroundRegionRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  productionRuntimeClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingIosSimulatorArtifactInventoryRowSchema = withParser(
  TrackingIosSimulatorArtifactInventoryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingIosSimulatorArtifactInventoryRowIsHonest(row) ||
        'Expected iOS simulator artifact inventory rows to classify required local artifacts without claiming Core Location runtime or physical-device readiness'
    )
  )
);

export const TrackingIosSimulatorArtifactInventoryProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-ios-simulator-artifact-inventory-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingIosSimulatorArtifactInventoryRowSchema).pipe(Schema.minItems(1)),
    summary: Schema.Struct({
      requiredArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      presentArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      missingArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      simulatorPackageArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      locationManualRequiredArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      privacyDisclosureArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      platformProofArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      validationLogArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      iosManualRequiredRowCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      iosMissingRuntimeArtifactCount: TrackingIosSimulatorArtifactInventoryCountSchema,
      simulatorArtifactInventoryComplete: Schema.Boolean,
    }),
    proofClaims: Schema.Struct({
      simulatorPackageArtifactsObserved: Schema.Literal(true),
      locationManualRequiredArtifactsObserved: Schema.Literal(true),
      privacyDisclosureArtifactsObserved: Schema.Literal(true),
      platformProofArtifactsObserved: Schema.Literal(true),
      noCoreLocationRuntimeClaim: Schema.Literal(true),
      noBackgroundRegionRuntimeClaim: Schema.Literal(true),
      noPhysicalDeviceClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryClaim: Schema.Literal(true),
      noProductionClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      simulatorArtifactInventoryComplete: Schema.Boolean,
      coreLocationRuntimeClaimed: Schema.Literal(false),
      backgroundRegionRuntimeClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      productionRuntimeClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          proof.summary.requiredArtifactCount ===
            proof.summary.presentArtifactCount + proof.summary.missingArtifactCount ||
          'iOS simulator artifact inventory summary must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          proof.rows.every(
            (row) => row.simulatorArtifactInventoryComplete === proof.productClaims.simulatorArtifactInventoryComplete
          ) || 'iOS simulator artifact inventory rows and product claims must agree on artifact completeness'
      )
    )
);

export type TrackingIosSimulatorArtifactInventoryInput = Infer<typeof TrackingIosSimulatorArtifactInventoryInputSchema>;
export type TrackingIosSimulatorArtifactInventoryProof = Infer<typeof TrackingIosSimulatorArtifactInventoryProofSchema>;
type TrackingIosSimulatorArtifactInventoryRowInput = Infer<typeof TrackingIosSimulatorArtifactInventoryRowBaseSchema>;

export function buildTrackingIosSimulatorArtifactInventoryProof(
  generatedAt: string,
  input: TrackingIosSimulatorArtifactInventoryInput
): TrackingIosSimulatorArtifactInventoryProof {
  const parsedInput = TrackingIosSimulatorArtifactInventoryInputSchema.parse(input);
  const row = artifactInventoryRow(generatedAt, parsedInput);
  const summary = summaryFrom(row);

  return TrackingIosSimulatorArtifactInventoryProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-ios-simulator-artifact-inventory-proof',
    generatedAt,
    rows: [row],
    summary,
    proofClaims: {
      simulatorPackageArtifactsObserved: true,
      locationManualRequiredArtifactsObserved: true,
      privacyDisclosureArtifactsObserved: true,
      platformProofArtifactsObserved: true,
      noCoreLocationRuntimeClaim: true,
      noBackgroundRegionRuntimeClaim: true,
      noPhysicalDeviceClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      simulatorArtifactInventoryComplete: row.simulatorArtifactInventoryComplete,
      coreLocationRuntimeClaimed: false,
      backgroundRegionRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryClaimed: false,
      productionRuntimeClaimed: false,
      productClaimReady: false,
    },
  });
}

function artifactInventoryRow(generatedAt: string, input: TrackingIosSimulatorArtifactInventoryInput) {
  const presentArtifacts = input.artifactRows
    .filter((artifact) => artifact.present)
    .map((artifact) => artifact.artifactRef);
  const missingArtifacts = input.artifactRows
    .filter((artifact) => !artifact.present)
    .map((artifact) => artifact.artifactRef);

  return TrackingIosSimulatorArtifactInventoryRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-ios-simulator-artifact-inventory',
    generatedAt,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'ios-simulator-local-artifacts-present-physical-device-required',
    sourceIosSimulatorProofRef: input.sourceIosSimulatorProofRef,
    auditRefs: ['tracking-ios-simulator-artifact-inventory-audit'],
    iosSimulatorProofStatus: input.iosSimulatorProofStatus,
    iosSimulatorCurrentProofTier: input.iosSimulatorCurrentProofTier,
    hostPlatform: input.hostPlatform,
    hostArch: input.hostArch,
    canRunXcodeSimulator: input.canRunXcodeSimulator,
    requiredArtifacts: [...RequiredTrackingIosSimulatorArtifactRefs],
    presentArtifacts,
    missingArtifacts,
    artifactRows: input.artifactRows,
    iosManualRequiredRowCount: input.iosManualRequiredRowCount,
    iosRequiredRuntimeArtifactCount: input.iosRequiredRuntimeArtifactCount,
    iosPresentRuntimeArtifactCount: input.iosPresentRuntimeArtifactCount,
    iosMissingRuntimeArtifactCount: input.iosMissingRuntimeArtifactCount,
    privacyReleaseGateRowCount: input.privacyReleaseGateRowCount,
    privacyReleaseBlockedCount: input.privacyReleaseBlockedCount,
    simulatorArtifactInventoryComplete: missingArtifacts.length === 0,
    coreLocationRuntimeClaimed: false,
    backgroundRegionRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryClaimed: false,
    productionRuntimeClaimed: false,
    productClaimReady: false,
  });
}

function summaryFrom(row: TrackingIosSimulatorArtifactInventoryRowInput) {
  return {
    requiredArtifactCount: row.requiredArtifacts.length,
    presentArtifactCount: row.presentArtifacts.length,
    missingArtifactCount: row.missingArtifacts.length,
    simulatorPackageArtifactCount: countByCategory(row, 'simulator-package-proof'),
    locationManualRequiredArtifactCount: countByCategory(row, 'location-manual-required-proof'),
    privacyDisclosureArtifactCount: countByCategory(row, 'privacy-disclosure-proof'),
    platformProofArtifactCount: countByCategory(row, 'platform-proof'),
    validationLogArtifactCount: countByCategory(row, 'validation-log'),
    iosManualRequiredRowCount: row.iosManualRequiredRowCount,
    iosMissingRuntimeArtifactCount: row.iosMissingRuntimeArtifactCount,
    simulatorArtifactInventoryComplete: row.simulatorArtifactInventoryComplete,
  };
}

function countByCategory(row: TrackingIosSimulatorArtifactInventoryRowInput, category: string): number {
  return row.artifactRows.filter((artifact) => artifact.category === category).length;
}

function trackingIosSimulatorArtifactInventoryRowIsHonest(row: TrackingIosSimulatorArtifactInventoryRowInput): boolean {
  const requiredArtifactSet = new Set(row.requiredArtifacts.map((artifactRef) => String(artifactRef)));
  const artifactRowSet = new Set(row.artifactRows.map((artifact) => String(artifact.artifactRef)));
  return (
    trackingIosSimulatorArtifactsCoverRequirements(row, requiredArtifactSet, artifactRowSet) &&
    trackingIosSimulatorManualRowsAreHonest(row) &&
    trackingIosSimulatorArtifactInventoryNonClaimsAreHonest(row)
  );
}

function trackingIosSimulatorArtifactsCoverRequirements(
  row: TrackingIosSimulatorArtifactInventoryRowInput,
  requiredArtifactSet: ReadonlySet<string>,
  artifactRowSet: ReadonlySet<string>
): boolean {
  return (
    RequiredTrackingIosSimulatorArtifactRefs.every(
      (artifactRef) => requiredArtifactSet.has(artifactRef) && artifactRowSet.has(artifactRef)
    ) &&
    row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length &&
    row.artifactRows.every((artifact) => artifact.required === true) &&
    row.presentArtifacts.every((artifactRef) => requiredArtifactSet.has(String(artifactRef))) &&
    row.missingArtifacts.every((artifactRef) => requiredArtifactSet.has(String(artifactRef)))
  );
}

function trackingIosSimulatorManualRowsAreHonest(row: TrackingIosSimulatorArtifactInventoryRowInput): boolean {
  return (
    row.iosRequiredRuntimeArtifactCount === row.iosPresentRuntimeArtifactCount + row.iosMissingRuntimeArtifactCount &&
    row.iosManualRequiredRowCount > 0 &&
    row.iosMissingRuntimeArtifactCount > 0 &&
    row.privacyReleaseGateRowCount > 0 &&
    row.privacyReleaseBlockedCount > 0
  );
}

function trackingIosSimulatorArtifactInventoryNonClaimsAreHonest(
  row: TrackingIosSimulatorArtifactInventoryRowInput
): boolean {
  return (
    row.coreLocationRuntimeClaimed === false &&
    row.backgroundRegionRuntimeClaimed === false &&
    row.physicalDeviceProofClaimed === false &&
    row.authorityProofClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.productionRuntimeClaimed === false &&
    row.productClaimReady === false
  );
}

