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
  'android-system-geofence-blocker',
  'notification-receipt-boundary',
  'notification-preference-preflight',
  'notification-preference-status-handoff',
  'notification-local-outbox-readiness',
  'authority-enrollment-manual-required',
  'authority-runtime-readiness-blocker',
  'child-runtime-artifact-gate',
  'physical-device-artifact-gate',
  'provider-delivery-artifact-gate',
  'provider-runtime-readiness-blocker',
  'escalation-runtime-readiness-blocker',
  'child-runtime-product-readiness-blocker',
  'full-product-ui-readiness-blocker',
  'production-durable-workers-readiness-blocker',
  'retention-product-readiness-blocker'
);

export const RequiredTrackingProductReadinessClosureCoverageTags = [
  'pre-device-gate',
  'android-emulator-proof',
  'ios-simulator-proof',
  'ios-privacy-disclosure-release-gate',
  'wsl-local-replay',
  'hosted-ui-artifact-inventory',
  'android-system-geofence-blocker',
  'notification-receipt-boundary',
  'notification-preference-preflight',
  'notification-preference-status-handoff',
  'notification-local-outbox-readiness',
  'authority-enrollment-manual-required',
  'authority-runtime-readiness-blocker',
  'child-runtime-artifact-gate',
  'physical-device-artifact-gate',
  'provider-delivery-artifact-gate',
  'provider-runtime-readiness-blocker',
  'escalation-runtime-readiness-blocker',
  'child-runtime-product-readiness-blocker',
  'full-product-ui-readiness-blocker',
  'production-durable-workers-readiness-blocker',
  'retention-product-readiness-blocker',
] as const;

export const TrackingProductReadinessClosureBlockerSchema = Schema.Literal(
  'android-physical-background-proof-required',
  'ios-physical-region-proof-required',
  'actual-child-device-runtime-required',
  'full-product-parent-child-ui-required',
  'authority-enrollment-proof-required',
  'provider-delivery-receipt-runtime-required',
  'production-durable-workers-required'
);

export const RequiredTrackingProductReadinessClosureBlockers = [
  'android-physical-background-proof-required',
  'ios-physical-region-proof-required',
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
type TrackingProductReadinessClosureRowInput = Infer<typeof TrackingProductReadinessClosureRowBaseSchema>;

export function buildTrackingProductReadinessClosureProof(
  generatedAt: string,
  sourceProofs: readonly Infer<typeof TrackingProductReadinessClosureSourceProofSchema>[]
): TrackingProductReadinessClosureProof {
  const parsedSourceProofs = sourceProofs.map((sourceProof) =>
    TrackingProductReadinessClosureSourceProofSchema.parse(sourceProof)
  );

  return TrackingProductReadinessClosureProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-product-readiness-closure-proof',
    generatedAt,
    sourceProofs: parsedSourceProofs,
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
