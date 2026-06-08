import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import { TrackingRetentionSettingsProofRefSchema } from './tracking-retention-settings-read-model-proof';

const TrackingProductReadinessClosureTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingProductReadinessClosureProofIdSchema = TrackingProductReadinessClosureTextSchema.pipe(
  Schema.brand('TrackingProductReadinessClosureProofId')
);

export const TrackingProductReadinessClosureCoverageTagSchema = Schema.Literal(
  'pre-device-gate',
  'android-emulator-proof',
  'ios-simulator-proof',
  'ios-privacy-disclosure-release-gate',
  'wsl-local-replay',
  'hosted-ui-artifact-inventory',
  'android-emulator-artifact-inventory',
  'ios-simulator-artifact-inventory',
  'android-system-geofence-blocker',
  'notification-receipt-boundary',
  'notification-preference-preflight',
  'notification-preference-status-handoff',
  'notification-local-outbox-readiness',
  'authority-enrollment-manual-required',
  'authority-runtime-readiness-blocker',
  'authority-runtime-artifact-gate',
  'child-runtime-artifact-gate',
  'child-runtime-android-emulator-readiness-bridge',
  'physical-device-artifact-gate',
  'provider-delivery-artifact-gate',
  'provider-runtime-readiness-blocker',
  'escalation-runtime-readiness-blocker',
  'escalation-runtime-artifact-gate',
  'child-runtime-product-readiness-blocker',
  'full-product-ui-readiness-blocker',
  'full-product-ui-local-runtime-artifact-capture',
  'full-product-ui-runtime-artifact-gate',
  'full-product-ui-runtime-preflight',
  'production-durable-workers-readiness-blocker',
  'production-worker-runtime-artifact-gate',
  'production-worker-runtime-preflight',
  'retention-product-readiness-blocker',
  'retention-runtime-artifact-gate',
  'retention-platform-enforcement-preflight',
  'tracking-claim-audit'
);

export const RequiredTrackingProductReadinessClosureCoverageTags = [
  'pre-device-gate',
  'android-emulator-proof',
  'ios-simulator-proof',
  'ios-privacy-disclosure-release-gate',
  'wsl-local-replay',
  'hosted-ui-artifact-inventory',
  'android-emulator-artifact-inventory',
  'ios-simulator-artifact-inventory',
  'android-system-geofence-blocker',
  'notification-receipt-boundary',
  'notification-preference-preflight',
  'notification-preference-status-handoff',
  'notification-local-outbox-readiness',
  'authority-enrollment-manual-required',
  'authority-runtime-readiness-blocker',
  'authority-runtime-artifact-gate',
  'child-runtime-artifact-gate',
  'child-runtime-android-emulator-readiness-bridge',
  'physical-device-artifact-gate',
  'provider-delivery-artifact-gate',
  'provider-runtime-readiness-blocker',
  'escalation-runtime-readiness-blocker',
  'escalation-runtime-artifact-gate',
  'child-runtime-product-readiness-blocker',
  'full-product-ui-readiness-blocker',
  'full-product-ui-local-runtime-artifact-capture',
  'full-product-ui-runtime-artifact-gate',
  'full-product-ui-runtime-preflight',
  'production-durable-workers-readiness-blocker',
  'production-worker-runtime-artifact-gate',
  'production-worker-runtime-preflight',
  'retention-product-readiness-blocker',
  'retention-runtime-artifact-gate',
  'retention-platform-enforcement-preflight',
  'tracking-claim-audit',
] as const;

export const TrackingProductReadinessClosureBlockerSchema = Schema.Literal(
  'android-physical-background-proof-required',
  'ios-physical-region-proof-required',
  'retention-writable-product-settings-required',
  'retention-platform-runtime-enforcement-required',
  'actual-child-device-runtime-required',
  'full-product-parent-child-ui-required',
  'authority-enrollment-proof-required',
  'provider-delivery-receipt-runtime-required',
  'production-durable-workers-required'
);

export const RequiredTrackingProductReadinessClosureBlockers = [
  'android-physical-background-proof-required',
  'ios-physical-region-proof-required',
  'retention-writable-product-settings-required',
  'retention-platform-runtime-enforcement-required',
  'actual-child-device-runtime-required',
  'full-product-parent-child-ui-required',
  'authority-enrollment-proof-required',
  'provider-delivery-receipt-runtime-required',
  'production-durable-workers-required',
] as const;

export const TrackingProductReadinessClosureSourceProofSchema = withParser(
  Schema.Struct({
    coverageTag: TrackingProductReadinessClosureCoverageTagSchema,
    proofRef: TrackingRetentionSettingsProofRefSchema,
    status: TrackingProductReadinessClosureTextSchema,
    proofTier: TrackingProductReadinessClosureTextSchema,
  })
);

export const TrackingProductReadinessClosureAggregateEvidenceSchema = withParser(
  Schema.Struct({
    fullProductUiLocalArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiClosureRetentionWritableExecutionRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiClosureRetentionWritableExecutionDerivationCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiClosureChildRuntimeMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    fullProductUiRuntimePreflightRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiRuntimePreflightManualRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiRuntimePreflightRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiRuntimePreflightPresentArtifactCount: Schema.Literal(0),
    fullProductUiRuntimePreflightMissingArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiRuntimePreflightProductReadyRowCount: Schema.Literal(0),
    androidEmulatorRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    androidEmulatorPresentArtifactCount: Schema.Number.pipe(Schema.int()),
    androidEmulatorMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    androidEmulatorPermissionUiArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    androidEmulatorRuntimeArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    androidEmulatorLocalGeofenceTransitionCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    iosSimulatorRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    iosSimulatorPresentArtifactCount: Schema.Number.pipe(Schema.int()),
    iosSimulatorMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    iosSimulatorPackageArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    iosSimulatorLocationManualRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    iosSimulatorPrivacyDisclosureArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    iosSimulatorManualRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    iosSimulatorMissingRuntimeArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    childRuntimeRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    childRuntimePresentArtifactCount: Schema.Number.pipe(Schema.int()),
    childRuntimeMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    retentionRuntimeRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    retentionRuntimePresentArtifactCount: Schema.Number.pipe(Schema.int()),
    retentionRuntimeMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    retentionRuntimeManualRequiredRowCount: Schema.Number.pipe(Schema.int()),
    retentionRuntimeArtifactSetPresentRowCount: Schema.Number.pipe(Schema.int()),
    retentionPlatformPreflightRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    retentionPlatformPreflightManualRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    retentionPlatformPreflightRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    retentionPlatformPreflightPresentArtifactCount: Schema.Literal(0),
    retentionPlatformPreflightMissingArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    retentionPlatformPreflightProductReadyRowCount: Schema.Literal(0),
    productionWorkerRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    productionWorkerPresentArtifactCount: Schema.Number.pipe(Schema.int()),
    productionWorkerMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    productionWorkerPreflightRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    productionWorkerPreflightManualRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    productionWorkerPreflightRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    productionWorkerPreflightPresentArtifactCount: Schema.Literal(0),
    productionWorkerPreflightMissingArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    productionWorkerPreflightProductReadyRowCount: Schema.Literal(0),
    claimAuditPresentArtifactCount: Schema.Number.pipe(Schema.int()),
    claimAuditMissingArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    claimAuditManualRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    claimAuditPhysicalDeviceRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    claimAuditApprovedManualRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    claimAuditManualProviderRuntimeRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    claimAuditProductionRuntimeRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    claimAuditProductReadyRowCount: Schema.Literal(0),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.fullProductUiClosureRetentionWritableExecutionRowCount ===
            evidence.fullProductUiClosureRetentionWritableExecutionDerivationCount ||
          'Aggregate closure evidence must preserve retention writable derivation count'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.fullProductUiClosureChildRuntimeMissingArtifactCount >= 0 ||
          'Aggregate closure evidence cannot record negative child-runtime missing artifacts'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.fullProductUiRuntimePreflightRequiredArtifactCount ===
            evidence.fullProductUiRuntimePreflightPresentArtifactCount +
              evidence.fullProductUiRuntimePreflightMissingArtifactCount ||
          'Aggregate closure evidence must classify every full product UI runtime preflight artifact'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.fullProductUiRuntimePreflightRowCount ===
            evidence.fullProductUiRuntimePreflightManualRequiredRowCount ||
          'Aggregate closure evidence must keep full product UI runtime preflight manual-required'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.androidEmulatorRequiredArtifactCount ===
            evidence.androidEmulatorPresentArtifactCount + evidence.androidEmulatorMissingArtifactCount ||
          'Aggregate closure evidence must classify every Android emulator artifact'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.iosSimulatorRequiredArtifactCount ===
            evidence.iosSimulatorPresentArtifactCount + evidence.iosSimulatorMissingArtifactCount ||
          'Aggregate closure evidence must classify every iOS simulator artifact'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.childRuntimeRequiredArtifactCount ===
            evidence.childRuntimePresentArtifactCount + evidence.childRuntimeMissingArtifactCount ||
          'Aggregate closure evidence must classify every child-runtime artifact'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.retentionRuntimeRequiredArtifactCount ===
            evidence.retentionRuntimePresentArtifactCount + evidence.retentionRuntimeMissingArtifactCount ||
          'Aggregate closure evidence must classify every retention runtime artifact'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          (evidence.retentionRuntimePresentArtifactCount >= 0 &&
            evidence.retentionRuntimeMissingArtifactCount >= 0 &&
            evidence.retentionRuntimeManualRequiredRowCount >= 0 &&
            evidence.retentionRuntimeArtifactSetPresentRowCount >= 0) ||
          'Aggregate closure evidence cannot record negative retention runtime counts'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.retentionPlatformPreflightRequiredArtifactCount ===
            evidence.retentionPlatformPreflightPresentArtifactCount +
              evidence.retentionPlatformPreflightMissingArtifactCount ||
          'Aggregate closure evidence must classify every retention platform preflight artifact'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.retentionPlatformPreflightRowCount === evidence.retentionPlatformPreflightManualRequiredRowCount ||
          'Aggregate closure evidence must keep retention platform preflight manual-required'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.productionWorkerRequiredArtifactCount ===
            evidence.productionWorkerPresentArtifactCount + evidence.productionWorkerMissingArtifactCount ||
          'Aggregate closure evidence must classify every production worker runtime artifact'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.productionWorkerPreflightRequiredArtifactCount ===
            evidence.productionWorkerPreflightPresentArtifactCount +
              evidence.productionWorkerPreflightMissingArtifactCount ||
          'Aggregate closure evidence must classify every production worker preflight artifact'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.productionWorkerPreflightRowCount === evidence.productionWorkerPreflightManualRequiredRowCount ||
          'Aggregate closure evidence must keep production worker preflight manual-required'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.claimAuditManualRequiredRowCount ===
            evidence.claimAuditPhysicalDeviceRequiredRowCount +
              evidence.claimAuditApprovedManualRequiredRowCount +
              evidence.claimAuditManualProviderRuntimeRequiredRowCount +
              evidence.claimAuditProductionRuntimeRequiredRowCount ||
          'Aggregate closure evidence must classify every claim-audit manual-required row by proof tier'
      )
    )
);

const TrackingProductReadinessClosureRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  closureProofId: TrackingProductReadinessClosureProofIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  remainingBlockers: Schema.Array(TrackingProductReadinessClosureBlockerSchema),
  localCiProofAccountingReady: Schema.Literal(true),
  physicalAndroidBackgroundClaimed: Schema.Literal(false),
  physicalIosBackgroundClaimed: Schema.Literal(false),
  childDeviceRuntimeClaimed: Schema.Literal(false),
  fullProductUiClaimed: Schema.Literal(false),
  authorityClaimed: Schema.Literal(false),
  providerDeliveryReceiptClaimed: Schema.Literal(false),
  productionWorkersClaimed: Schema.Literal(false),
  productReadyClaimed: Schema.Literal(false),
});

export const TrackingProductReadinessClosureRowSchema = withParser(
  TrackingProductReadinessClosureRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        (row.sourceProofRefs.length >= RequiredTrackingProductReadinessClosureCoverageTags.length &&
          row.auditRefs.length > 0 &&
          row.remainingBlockers.length === RequiredTrackingProductReadinessClosureBlockers.length &&
          trackingProductReadinessClosureRowNonClaimsAreHonest(row)) ||
        'Expected tracking closure rows to cite all proof refs, enumerate all blockers, and avoid product overclaims'
    )
  )
);

export const TrackingProductReadinessClosureProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-product-readiness-closure-proof'),
    generatedAt: ParentTimestampSchema,
    sourceProofs: Schema.Array(TrackingProductReadinessClosureSourceProofSchema),
    aggregateEvidence: TrackingProductReadinessClosureAggregateEvidenceSchema,
    rows: Schema.Array(TrackingProductReadinessClosureRowSchema),
    proofClaims: Schema.Struct({
      localCiProofRefsObserved: Schema.Literal(true),
      remainingProductBlockersEnumerated: Schema.Literal(true),
      noPhysicalDeviceClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProductionClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      localCiProofAccountingReady: Schema.Literal(true),
      physicalAndroidBackgroundClaimed: Schema.Literal(false),
      physicalIosBackgroundClaimed: Schema.Literal(false),
      childDeviceRuntimeClaimed: Schema.Literal(false),
      fullProductUiClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      providerDeliveryReceiptClaimed: Schema.Literal(false),
      productionWorkersClaimed: Schema.Literal(false),
      productReadyClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === 1 && sourceProofsCoverRequiredTags(proof.sourceProofs)) ||
        'Expected tracking closure proof to include one closure row and all required source proof coverage tags'
    )
  )
);

export type TrackingProductReadinessClosureProof = Infer<typeof TrackingProductReadinessClosureProofSchema>;
export type TrackingProductReadinessClosureAggregateEvidence = Infer<
  typeof TrackingProductReadinessClosureAggregateEvidenceSchema
>;
type TrackingProductReadinessClosureRowInput = Infer<typeof TrackingProductReadinessClosureRowBaseSchema>;

export function buildTrackingProductReadinessClosureProof(
  generatedAt: string,
  sourceProofs: readonly Infer<typeof TrackingProductReadinessClosureSourceProofSchema>[],
  aggregateEvidenceInput: TrackingProductReadinessClosureAggregateEvidence
): TrackingProductReadinessClosureProof {
  const parsedSourceProofs = sourceProofs.map((sourceProof) =>
    TrackingProductReadinessClosureSourceProofSchema.parse(sourceProof)
  );

  return TrackingProductReadinessClosureProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-product-readiness-closure-proof',
    generatedAt,
    sourceProofs: parsedSourceProofs,
    aggregateEvidence: TrackingProductReadinessClosureAggregateEvidenceSchema.parse(aggregateEvidenceInput),
    rows: [closureRow(generatedAt, parsedSourceProofs)],
    proofClaims: {
      localCiProofRefsObserved: true,
      remainingProductBlockersEnumerated: true,
      noPhysicalDeviceClaim: true,
      noAuthorityClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      localCiProofAccountingReady: true,
      physicalAndroidBackgroundClaimed: false,
      physicalIosBackgroundClaimed: false,
      childDeviceRuntimeClaimed: false,
      fullProductUiClaimed: false,
      authorityClaimed: false,
      providerDeliveryReceiptClaimed: false,
      productionWorkersClaimed: false,
      productReadyClaimed: false,
    },
  });
}

function closureRow(
  generatedAt: string,
  sourceProofs: readonly Infer<typeof TrackingProductReadinessClosureSourceProofSchema>[]
) {
  return TrackingProductReadinessClosureRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    closureProofId: 'tracking-product-readiness-closure-local-ci-accounting',
    generatedAt,
    sourceProofRefs: sourceProofs.map((sourceProof) => sourceProof.proofRef),
    auditRefs: ['tracking-product-readiness-closure-audit'],
    remainingBlockers: [...RequiredTrackingProductReadinessClosureBlockers],
    localCiProofAccountingReady: true,
    physicalAndroidBackgroundClaimed: false,
    physicalIosBackgroundClaimed: false,
    childDeviceRuntimeClaimed: false,
    fullProductUiClaimed: false,
    authorityClaimed: false,
    providerDeliveryReceiptClaimed: false,
    productionWorkersClaimed: false,
    productReadyClaimed: false,
  });
}

function sourceProofsCoverRequiredTags(
  sourceProofs: readonly Infer<typeof TrackingProductReadinessClosureSourceProofSchema>[]
): boolean {
  const tags = new Set(sourceProofs.map((sourceProof) => sourceProof.coverageTag));
  return RequiredTrackingProductReadinessClosureCoverageTags.every((tag) => tags.has(tag));
}

function trackingProductReadinessClosureRowNonClaimsAreHonest(row: TrackingProductReadinessClosureRowInput): boolean {
  return (
    row.localCiProofAccountingReady === true &&
    row.physicalAndroidBackgroundClaimed === false &&
    row.physicalIosBackgroundClaimed === false &&
    row.childDeviceRuntimeClaimed === false &&
    row.fullProductUiClaimed === false &&
    row.authorityClaimed === false &&
    row.providerDeliveryReceiptClaimed === false &&
    row.productionWorkersClaimed === false &&
    row.productReadyClaimed === false
  );
}
