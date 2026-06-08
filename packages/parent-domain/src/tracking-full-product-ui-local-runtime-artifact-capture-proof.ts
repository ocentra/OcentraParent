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
import { TrackingFullProductUiReadinessBlockerReferenceSchema } from './tracking-full-product-ui-readiness-blocker-proof';
import { TrackingRetentionProductSettingsWritableExecutionProofSchema } from './tracking-retention-product-settings-writable-execution-proof';

const TrackingFullProductUiLocalRuntimeArtifactCaptureText = Schema.String.pipe(Schema.minLength(1));

export const TrackingFullProductUiLocalRuntimeArtifactCaptureIdSchema = withParser(
  Schema.Literal(
    'parent-overview-runtime-ui',
    'parent-device-detail-runtime-ui',
    'parent-notification-history-preferences-runtime',
    'retention-settings-local-write-result',
    'child-check-in-hosted-local-readiness-ui',
    'child-runtime-hosted-local-readiness-ui',
    'cross-surface-accessibility-report',
    'product-ui-end-to-end-trace'
  )
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
        row.artifactId === 'product-ui-end-to-end-trace' ||
        'Copied UI artifact rows must preserve screenshot byte size'
    )
  )
);

export const TrackingFullProductUiLocalRuntimeClosureEvidenceSchema = withParser(
  Schema.Struct({
    retentionWritableExecutionProofRef: TrackingFullProductUiReadinessBlockerReferenceSchema,
    retentionWritableExecutionRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    retentionWritableExecutionDerivationCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    retentionWritableExecutionArtifactRefs: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
    retentionLocalProductSettingsWritableExecutionObserved: Schema.Literal(true),
    retentionPortalWritableUiClaimed: Schema.Literal(false),
    retentionPlatformRuntimeEnforcementClaimed: Schema.Literal(false),
    childRuntimeArtifactGateProofRef: TrackingFullProductUiReadinessBlockerReferenceSchema,
    childRuntimeRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    childRuntimeMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    childRuntimeArtifactSetComplete: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
    parentReceiptRuntimeClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    productionProductUiClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (evidence) =>
          evidence.retentionWritableExecutionRowCount === evidence.retentionWritableExecutionDerivationCount ||
          'Retention writable execution closure evidence must preserve one derivation per row'
      )
    )
    .pipe(
      Schema.filter(
        (evidence) => evidence.childRuntimeMissingArtifactCount >= 0 || 'Child runtime missing count cannot be negative'
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
    closureEvidence: TrackingFullProductUiLocalRuntimeClosureEvidenceSchema,
    localArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    productClaims: Schema.Struct({
      parentOverviewLocalArtifactCaptured: Schema.Literal(true),
      parentDeviceDetailLocalArtifactCaptured: Schema.Literal(true),
      parentNotificationHistoryPreferencesLocalArtifactCaptured: Schema.Literal(true),
      retentionSettingsLocalWriteResultCaptured: Schema.Literal(true),
      childCheckInHostedLocalReadinessArtifactCaptured: Schema.Literal(true),
      childRuntimeHostedLocalReadinessArtifactCaptured: Schema.Literal(true),
      crossSurfaceAccessibilityLocalArtifactCaptured: Schema.Literal(true),
      productUiEndToEndTraceCaptured: Schema.Literal(true),
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
export type TrackingFullProductUiLocalRuntimeClosureEvidence = Infer<
  typeof TrackingFullProductUiLocalRuntimeClosureEvidenceSchema
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

export type TrackingFullProductUiLocalRuntimeClosureEvidenceInput = {
  readonly retentionWritableExecutionProofRef: string;
  readonly retentionWritableExecutionProof: unknown;
  readonly childRuntimeArtifactGateProofRef: string;
  readonly childRuntimeArtifactGateProof: unknown;
};

export const RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures = [
  'parent-overview-runtime-ui',
  'parent-device-detail-runtime-ui',
  'parent-notification-history-preferences-runtime',
  'retention-settings-local-write-result',
  'child-check-in-hosted-local-readiness-ui',
  'child-runtime-hosted-local-readiness-ui',
  'cross-surface-accessibility-report',
  'product-ui-end-to-end-trace',
] as const;

export function buildTrackingFullProductUiLocalRuntimeArtifactCaptureProof(
  generatedAt: string,
  sourceProofRefs: readonly string[],
  captures: readonly TrackingFullProductUiLocalRuntimeArtifactCaptureInput[],
  closureEvidenceInput: TrackingFullProductUiLocalRuntimeClosureEvidenceInput
): TrackingFullProductUiLocalRuntimeArtifactCaptureProof {
  const rows = RequiredTrackingFullProductUiLocalRuntimeArtifactCaptures.map((artifactId) => {
    const capture = captures.find((candidate) => candidate.artifactId === artifactId);
    if (!capture) throw new Error(`Missing local full product UI artifact capture: ${artifactId}`);
    return captureRow(generatedAt, capture);
  });
  const closureEvidence = buildClosureEvidence(closureEvidenceInput);

  return TrackingFullProductUiLocalRuntimeArtifactCaptureProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofMode: 'tracking-full-product-ui-local-runtime-artifact-capture-proof',
    generatedAt,
    sourceProofRefs: uniqueRefs(sourceProofRefs),
    rows,
    closureEvidence,
    localArtifactCount: rows.length,
    productClaims: {
      parentOverviewLocalArtifactCaptured: true,
      parentDeviceDetailLocalArtifactCaptured: true,
      parentNotificationHistoryPreferencesLocalArtifactCaptured: true,
      retentionSettingsLocalWriteResultCaptured: true,
      childCheckInHostedLocalReadinessArtifactCaptured: true,
      childRuntimeHostedLocalReadinessArtifactCaptured: true,
      crossSurfaceAccessibilityLocalArtifactCaptured: true,
      productUiEndToEndTraceCaptured: true,
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

function buildClosureEvidence(
  input: TrackingFullProductUiLocalRuntimeClosureEvidenceInput
): TrackingFullProductUiLocalRuntimeClosureEvidence {
  const retentionProof = TrackingRetentionProductSettingsWritableExecutionProofSchema.parse(
    input.retentionWritableExecutionProof
  );
  const childRuntimeProof = TrackingChildRuntimeArtifactGateProofSchema.parse(input.childRuntimeArtifactGateProof);
  const childRuntimeRow = singleChildRuntimeRow(childRuntimeProof);

  return TrackingFullProductUiLocalRuntimeClosureEvidenceSchema.parse({
    retentionWritableExecutionProofRef: input.retentionWritableExecutionProofRef,
    retentionWritableExecutionRowCount: retentionProof.rows.length,
    retentionWritableExecutionDerivationCount: retentionProof.derivationMatrix.length,
    retentionWritableExecutionArtifactRefs: uniqueRefs(retentionProof.rows.map((row) => row.outputArtifactRef)),
    retentionLocalProductSettingsWritableExecutionObserved:
      retentionProof.proofClaims.localProductSettingsWritableExecutionObserved,
    retentionPortalWritableUiClaimed: retentionProof.productClaims.portalWritableUiClaimed,
    retentionPlatformRuntimeEnforcementClaimed: retentionProof.productClaims.platformRuntimeRetentionEnforcementClaimed,
    childRuntimeArtifactGateProofRef: input.childRuntimeArtifactGateProofRef,
    childRuntimeRequiredArtifactCount: childRuntimeRow.requiredArtifacts.length,
    childRuntimeMissingArtifactCount: childRuntimeRow.missingArtifacts.length,
    childRuntimeArtifactSetComplete: childRuntimeRow.childRuntimeArtifactSetComplete,
    childDeviceRuntimeClaimed: childRuntimeProof.productClaims.childDeviceExecutionRuntimeClaimed,
    renderedChildDeviceUiRuntimeClaimed: childRuntimeProof.productClaims.renderedChildDeviceUiRuntimeClaimed,
    parentReceiptRuntimeClaimed: childRuntimeProof.productClaims.parentReceiptRuntimeClaimed,
    physicalDeviceProofClaimed: childRuntimeProof.productClaims.physicalDeviceProofClaimed,
    authorityProofClaimed: childRuntimeProof.productClaims.authorityProofClaimed,
    providerDeliveryRuntimeClaimed: childRuntimeProof.productClaims.providerDeliveryClaimed,
    productionProductUiClaimed: false,
    productClaimReady: false,
  });
}

function singleChildRuntimeRow(
  proof: TrackingChildRuntimeArtifactGateProof
): TrackingChildRuntimeArtifactGateProof['rows'][number] {
  const [row] = proof.rows;
  if (!row) throw new Error('Child runtime artifact gate proof has no rows');
  return row;
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
