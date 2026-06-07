import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  TrackingChildRuntimeArtifactGateProofSchema,
  type TrackingChildRuntimeArtifactGateProof,
} from './tracking-child-runtime-artifact-gate-proof';

const TrackingFullProductUiReadinessBlockerText = Schema.String.pipe(Schema.minLength(1));

export const TrackingFullProductUiReadinessBlockerIdSchema = withParser(
  Schema.Literal(
    'hosted-route-only-boundary',
    'full-parent-overview-runtime-ui',
    'full-parent-device-detail-runtime-ui',
    'parent-notification-history-preferences-runtime',
    'retention-settings-production-runtime-ui',
    'rendered-child-device-runtime-ui',
    'child-runtime-parent-receipt-ui',
    'cross-surface-accessibility-regression-proof',
    'physical-device-ui-proof',
    'authority-gated-ui-proof',
    'provider-delivery-ui-proof',
    'production-product-ui-proof',
    'product-ready-tracking-ui'
  )
);

export const TrackingFullProductUiReadinessBlockerReferenceSchema = TrackingFullProductUiReadinessBlockerText.pipe(
  Schema.brand('TrackingFullProductUiReadinessBlockerReference')
);
export const TrackingFullProductUiReadinessBlockerProofIdSchema = TrackingFullProductUiReadinessBlockerText.pipe(
  Schema.brand('TrackingFullProductUiReadinessBlockerProofId')
);
export const TrackingFullProductUiReadinessBlockerStatusSchema = withParser(Schema.Literal('manual-required'));

const TrackingFullProductUiReadinessBlockerRowBaseSchema = Schema.Struct({
  blockerId: TrackingFullProductUiReadinessBlockerIdSchema,
  status: TrackingFullProductUiReadinessBlockerStatusSchema,
  sourceProofRefs: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
  hostedUiArtifactRefs: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
  blockingArtifactRefs: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
  requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
  currentProofTier: Schema.Literal('P2_HOSTED_CI'),
  fullProductUiClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingFullProductUiReadinessBlockerRowSchema = withParser(
  TrackingFullProductUiReadinessBlockerRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.sourceProofRefs.length > 0 &&
        row.hostedUiArtifactRefs.length > 0 &&
        row.blockingArtifactRefs.length > 0 &&
        row.fullProductUiClaimed === false &&
        row.productClaimReady === false
    )
  )
);

const TrackingFullProductUiReadinessBlockerProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingFullProductUiReadinessBlockerProofIdSchema,
  generatedAt: ParentTimestampSchema,
  proofMode: Schema.Literal('tracking-full-product-ui-readiness-blocker-proof'),
  sourceProofRefs: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
  hostedScreenshotRefs: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
  hostedAssertionRefs: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
  childRuntimeArtifactRows: Schema.Number.pipe(Schema.int(), Schema.positive()),
  missingChildRuntimeArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
  missingFullProductUiArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
  blockers: Schema.Array(TrackingFullProductUiReadinessBlockerRowSchema),
  productClaims: Schema.Struct({
    hostedRouteOnlyClaimed: Schema.Literal(true),
    fullParentOverviewRuntimeUiClaimed: Schema.Literal(false),
    fullParentDeviceDetailRuntimeUiClaimed: Schema.Literal(false),
    parentNotificationHistoryPreferencesRuntimeClaimed: Schema.Literal(false),
    retentionSettingsProductionRuntimeUiClaimed: Schema.Literal(false),
    renderedChildDeviceRuntimeUiClaimed: Schema.Literal(false),
    childRuntimeParentReceiptUiClaimed: Schema.Literal(false),
    crossSurfaceAccessibilityRegressionClaimed: Schema.Literal(false),
    physicalDeviceUiProofClaimed: Schema.Literal(false),
    authorityGatedUiProofClaimed: Schema.Literal(false),
    providerDeliveryUiProofClaimed: Schema.Literal(false),
    productionProductUiClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  }),
});

export const TrackingFullProductUiReadinessBlockerProofSchema = withParser(
  TrackingFullProductUiReadinessBlockerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        trackingFullProductUiReadinessProofIsHonest(proof) ||
        'Expected full product UI blocker proof to cite hosted UI artifacts and child-runtime missing artifacts while keeping product UI claims false'
    )
  )
);

export type TrackingFullProductUiReadinessBlockerId = Infer<typeof TrackingFullProductUiReadinessBlockerIdSchema>;
export type TrackingFullProductUiReadinessBlockerProof = Infer<typeof TrackingFullProductUiReadinessBlockerProofSchema>;
export type TrackingFullProductUiReadinessBlockerRow = Infer<typeof TrackingFullProductUiReadinessBlockerRowSchema>;

export type TrackingFullProductUiReadinessBlockerProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceProofRefs: readonly string[];
  readonly hostedScreenshotRefs: readonly string[];
  readonly hostedAssertionRefs: readonly string[];
  readonly fullProductUiArtifactRefs: readonly string[];
};

type TrackingFullProductUiReadinessBlockerProofInput = Infer<
  typeof TrackingFullProductUiReadinessBlockerProofBaseSchema
>;

export function buildTrackingFullProductUiReadinessBlockerProof(
  options: TrackingFullProductUiReadinessBlockerProofOptions,
  childRuntimeArtifactGateProof: TrackingChildRuntimeArtifactGateProof
): TrackingFullProductUiReadinessBlockerProof {
  const parsedChildArtifactGate = TrackingChildRuntimeArtifactGateProofSchema.parse(childRuntimeArtifactGateProof);
  const sourceProofRefs = uniqueRefs(options.sourceProofRefs);
  const hostedUiArtifactRefs = uniqueRefs([...options.hostedScreenshotRefs, ...options.hostedAssertionRefs]);
  const childMissingArtifacts = uniqueRefs(
    parsedChildArtifactGate.rows.flatMap((row) =>
      row.missingArtifacts.map((artifact) => `${row.proofRoot}/${artifact}`)
    )
  );
  const missingFullProductUiArtifacts = uniqueRefs([...options.fullProductUiArtifactRefs, ...childMissingArtifacts]);

  return TrackingFullProductUiReadinessBlockerProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    proofMode: 'tracking-full-product-ui-readiness-blocker-proof',
    sourceProofRefs,
    hostedScreenshotRefs: uniqueRefs(options.hostedScreenshotRefs),
    hostedAssertionRefs: uniqueRefs(options.hostedAssertionRefs),
    childRuntimeArtifactRows: parsedChildArtifactGate.rows.length,
    missingChildRuntimeArtifactCount: childMissingArtifacts.length,
    missingFullProductUiArtifactCount: missingFullProductUiArtifacts.length,
    blockers: RequiredTrackingFullProductUiReadinessBlockers.map((blockerId) =>
      buildBlockerRow(blockerId, sourceProofRefs, hostedUiArtifactRefs, missingFullProductUiArtifacts)
    ),
    productClaims: {
      hostedRouteOnlyClaimed: true,
      fullParentOverviewRuntimeUiClaimed: false,
      fullParentDeviceDetailRuntimeUiClaimed: false,
      parentNotificationHistoryPreferencesRuntimeClaimed: false,
      retentionSettingsProductionRuntimeUiClaimed: false,
      renderedChildDeviceRuntimeUiClaimed: false,
      childRuntimeParentReceiptUiClaimed: false,
      crossSurfaceAccessibilityRegressionClaimed: false,
      physicalDeviceUiProofClaimed: false,
      authorityGatedUiProofClaimed: false,
      providerDeliveryUiProofClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    },
  });
}

export const RequiredTrackingFullProductUiReadinessBlockers = [
  'hosted-route-only-boundary',
  'full-parent-overview-runtime-ui',
  'full-parent-device-detail-runtime-ui',
  'parent-notification-history-preferences-runtime',
  'retention-settings-production-runtime-ui',
  'rendered-child-device-runtime-ui',
  'child-runtime-parent-receipt-ui',
  'cross-surface-accessibility-regression-proof',
  'physical-device-ui-proof',
  'authority-gated-ui-proof',
  'provider-delivery-ui-proof',
  'production-product-ui-proof',
  'product-ready-tracking-ui',
] as const;

function buildBlockerRow(
  blockerId: TrackingFullProductUiReadinessBlockerId,
  sourceProofRefs: readonly string[],
  hostedUiArtifactRefs: readonly string[],
  blockingArtifactRefs: readonly string[]
): TrackingFullProductUiReadinessBlockerRow {
  return TrackingFullProductUiReadinessBlockerRowSchema.parse({
    blockerId,
    status: 'manual-required',
    sourceProofRefs,
    hostedUiArtifactRefs,
    blockingArtifactRefs,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P2_HOSTED_CI',
    fullProductUiClaimed: false,
    productClaimReady: false,
  });
}

function trackingFullProductUiReadinessProofIsHonest(proof: TrackingFullProductUiReadinessBlockerProofInput): boolean {
  const productClaims = proof.productClaims;
  return (
    proof.sourceProofRefs.length >= 2 &&
    proof.hostedScreenshotRefs.length > 0 &&
    proof.hostedAssertionRefs.length > 0 &&
    proof.childRuntimeArtifactRows > 0 &&
    proof.missingChildRuntimeArtifactCount > 0 &&
    proof.missingFullProductUiArtifactCount > 0 &&
    proof.blockers.length === RequiredTrackingFullProductUiReadinessBlockers.length &&
    proof.blockers.every((row) => row.status === 'manual-required') &&
    productClaims.hostedRouteOnlyClaimed === true &&
    Object.entries(productClaims)
      .filter(([key]) => key !== 'hostedRouteOnlyClaimed')
      .every(([, claim]) => claim === false)
  );
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}
