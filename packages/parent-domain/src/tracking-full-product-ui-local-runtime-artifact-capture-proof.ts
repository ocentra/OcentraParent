import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { TrackingFullProductUiReadinessBlockerReferenceSchema } from './tracking-full-product-ui-readiness-blocker-proof';

const TrackingFullProductUiLocalRuntimeArtifactCaptureText = Schema.String.pipe(Schema.minLength(1));

export const TrackingFullProductUiLocalRuntimeArtifactCaptureIdSchema = withParser(
  Schema.Literal('parent-overview-runtime-ui', 'parent-device-detail-runtime-ui', 'cross-surface-accessibility-report')
);

export const TrackingFullProductUiLocalRuntimeArtifactCaptureStatusSchema = withParser(
  Schema.Literal('local-artifact-captured')
);

export const TrackingFullProductUiLocalRuntimeArtifactCapturePathSchema =
  TrackingFullProductUiLocalRuntimeArtifactCaptureText.pipe(
    Schema.brand('TrackingFullProductUiLocalRuntimeArtifactCapturePath')
  );

export const TrackingFullProductUiLocalRuntimeArtifactCaptureRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    artifactId: TrackingFullProductUiLocalRuntimeArtifactCaptureIdSchema,
    status: TrackingFullProductUiLocalRuntimeArtifactCaptureStatusSchema,
    generatedAt: ParentTimestampSchema,
    sourceArtifactRef: TrackingFullProductUiReadinessBlockerReferenceSchema,
    outputArtifactRef: TrackingFullProductUiReadinessBlockerReferenceSchema,
    sourceBytes: Schema.Number.pipe(Schema.int(), Schema.positive()),
    outputBytes: Schema.Number.pipe(Schema.int(), Schema.positive()),
    width: Schema.optional(Schema.Number.pipe(Schema.int(), Schema.positive())),
    height: Schema.optional(Schema.Number.pipe(Schema.int(), Schema.positive())),
    currentProofTier: Schema.Literal('P2_HOSTED_CI'),
    requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
    localParentUiArtifactCaptured: Schema.Literal(true),
    fullProductUiRuntimeClaimed: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    productionProductUiClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (row) =>
        row.sourceBytes === row.outputBytes ||
        row.artifactId === 'cross-surface-accessibility-report' ||
        'Copied UI artifact rows must preserve screenshot byte size'
    )
  )
);

export const TrackingFullProductUiLocalRuntimeArtifactCaptureProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofMode: Schema.Literal('tracking-full-product-ui-local-runtime-artifact-capture-proof'),
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
    rows: Schema.Array(TrackingFullProductUiLocalRuntimeArtifactCaptureRowSchema),
    localArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    productClaims: Schema.Struct({
      parentOverviewLocalArtifactCaptured: Schema.Literal(true),
      parentDeviceDetailLocalArtifactCaptured: Schema.Literal(true),
      crossSurfaceAccessibilityLocalArtifactCaptured: Schema.Literal(true),
      fullProductUiRuntimeClaimed: Schema.Literal(false),
      childDeviceRuntimeClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryRuntimeClaimed: Schema.Literal(false),
      productionProductUiClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        proof.rows.length === RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures.length &&
        proof.localArtifactCount === RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures.length &&
        proof.rows.every((row) => row.localParentUiArtifactCaptured === true) &&
        proof.rows.every((row) => row.productClaimReady === false) &&
        proof.sourceProofRefs.length > 0
    )
  )
);

export type TrackingFullProductUiLocalRuntimeArtifactCaptureId = Infer<
  typeof TrackingFullProductUiLocalRuntimeArtifactCaptureIdSchema
>;
export type TrackingFullProductUiLocalRuntimeArtifactCaptureProof = Infer<
  typeof TrackingFullProductUiLocalRuntimeArtifactCaptureProofSchema
>;
export type TrackingFullProductUiLocalRuntimeArtifactCaptureRow = Infer<
  typeof TrackingFullProductUiLocalRuntimeArtifactCaptureRowSchema
>;

export type TrackingFullProductUiLocalRuntimeArtifactCaptureInput = {
  readonly artifactId: TrackingFullProductUiLocalRuntimeArtifactCaptureId;
  readonly sourceArtifactRef: string;
  readonly outputArtifactRef: string;
  readonly sourceBytes: number;
  readonly outputBytes: number;
  readonly width?: number;
  readonly height?: number;
};

export const RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures = [
  'parent-overview-runtime-ui',
  'parent-device-detail-runtime-ui',
  'cross-surface-accessibility-report',
] as const;

export function buildTrackingFullProductUiLocalRuntimeArtifactCaptureProof(
  generatedAt: string,
  sourceProofRefs: readonly string[],
  captures: readonly TrackingFullProductUiLocalRuntimeArtifactCaptureInput[]
): TrackingFullProductUiLocalRuntimeArtifactCaptureProof {
  const rows = RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures.map((artifactId) => {
    const capture = captures.find((candidate) => candidate.artifactId === artifactId);
    if (!capture) throw new Error(`Missing local full product UI artifact capture: ${artifactId}`);
    return captureRow(generatedAt, capture);
  });

  return TrackingFullProductUiLocalRuntimeArtifactCaptureProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofMode: 'tracking-full-product-ui-local-runtime-artifact-capture-proof',
    generatedAt,
    sourceProofRefs: uniqueRefs(sourceProofRefs),
    rows,
    localArtifactCount: rows.length,
    productClaims: {
      parentOverviewLocalArtifactCaptured: true,
      parentDeviceDetailLocalArtifactCaptured: true,
      crossSurfaceAccessibilityLocalArtifactCaptured: true,
      fullProductUiRuntimeClaimed: false,
      childDeviceRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    },
  });
}

function captureRow(
  generatedAt: string,
  capture: TrackingFullProductUiLocalRuntimeArtifactCaptureInput
): TrackingFullProductUiLocalRuntimeArtifactCaptureRow {
  return TrackingFullProductUiLocalRuntimeArtifactCaptureRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    artifactId: capture.artifactId,
    status: 'local-artifact-captured',
    generatedAt,
    sourceArtifactRef: capture.sourceArtifactRef,
    outputArtifactRef: capture.outputArtifactRef,
    sourceBytes: capture.sourceBytes,
    outputBytes: capture.outputBytes,
    width: capture.width,
    height: capture.height,
    currentProofTier: 'P2_HOSTED_CI',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    localParentUiArtifactCaptured: true,
    fullProductUiRuntimeClaimed: false,
    childDeviceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    productionProductUiClaimed: false,
    productClaimReady: false,
  });
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}
