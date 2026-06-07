import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  TrackingChildRuntimeSnapshotRequirementsReadModelSchema,
  type TrackingChildRuntimeSnapshotRequirementsRow,
} from './tracking-child-runtime-snapshot-requirements-proof';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import { TrackingRetentionSettingsProofRefSchema } from './tracking-retention-settings-read-model-proof';

const TrackingChildRuntimeProductReadinessTextSchema = Schema.String.pipe(Schema.minLength(1));
const TrackingChildRuntimeProductReadinessCounterSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingChildRuntimeProductReadinessBlockerProofIdSchema =
  TrackingChildRuntimeProductReadinessTextSchema.pipe(
    Schema.brand('TrackingChildRuntimeProductReadinessBlockerProofId')
  );

export const TrackingChildRuntimeProductReadinessBlockerSchema = Schema.Literal(
  'child-device-delivery-runtime-proof-required',
  'child-device-execution-result-proof-required',
  'rendered-child-device-ui-snapshot-proof-required',
  'parent-receipt-runtime-proof-required',
  'runtime-observation-proof-required',
  'physical-device-proof-required',
  'authority-proof-required'
);

export const RequiredTrackingChildRuntimeProductReadinessBlockers = [
  'child-device-delivery-runtime-proof-required',
  'child-device-execution-result-proof-required',
  'rendered-child-device-ui-snapshot-proof-required',
  'parent-receipt-runtime-proof-required',
  'runtime-observation-proof-required',
  'physical-device-proof-required',
  'authority-proof-required',
] as const;

const TrackingChildRuntimeProductReadinessBlockerRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  blockerProofId: TrackingChildRuntimeProductReadinessBlockerProofIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceSnapshotRequirementsProofRef: TrackingRetentionSettingsProofRefSchema,
  sourceCheckInId: TrackingChildRuntimeProductReadinessTextSchema,
  sourceSnapshotRequirementRowId: TrackingChildRuntimeProductReadinessTextSchema,
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  deliveryEnvelopeRef: TrackingChildRuntimeProductReadinessTextSchema,
  executionResultRequirementRefCount: TrackingChildRuntimeProductReadinessCounterSchema,
  visibleSnapshotRequirementRefCount: TrackingChildRuntimeProductReadinessCounterSchema,
  parentReceiptRequirementRefCount: TrackingChildRuntimeProductReadinessCounterSchema,
  runtimeObservationRequirementRefCount: TrackingChildRuntimeProductReadinessCounterSchema,
  blockerRefs: Schema.Array(TrackingChildRuntimeProductReadinessBlockerSchema),
  deliveryEnvelopeRequirementClaimed: Schema.Literal(true),
  executionResultRequirementClaimed: Schema.Literal(true),
  visibleSnapshotRequirementClaimed: Schema.Literal(true),
  parentReceiptRequirementClaimed: Schema.Literal(true),
  runtimeObservationRequirementClaimed: Schema.Literal(true),
  childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
  childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
  renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptIngestionClaimed: Schema.Literal(false),
  liveLocationRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
  productReadyClaimed: Schema.Literal(false),
});

export const TrackingChildRuntimeProductReadinessBlockerRowSchema = withParser(
  TrackingChildRuntimeProductReadinessBlockerRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        (row.auditRefs.length > 0 &&
          row.blockerRefs.length === RequiredTrackingChildRuntimeProductReadinessBlockers.length &&
          trackingChildRuntimeProductReadinessBlockerRowIsHonest(row)) ||
        'Expected child runtime product-readiness blocker rows to preserve requirement refs and avoid runtime/product claims'
    )
  )
);

export const TrackingChildRuntimeProductReadinessBlockerProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-child-runtime-product-readiness-blocker-proof'),
    generatedAt: ParentTimestampSchema,
    sourceSnapshotRequirementsProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceSnapshotRequirementsStatus: TrackingChildRuntimeProductReadinessTextSchema,
    rows: Schema.Array(TrackingChildRuntimeProductReadinessBlockerRowSchema),
    proofClaims: Schema.Struct({
      snapshotRequirementRowsObserved: Schema.Literal(true),
      deliveryEnvelopeRequirementsObserved: Schema.Literal(true),
      executionResultRequirementsObserved: Schema.Literal(true),
      visibleSnapshotRequirementsObserved: Schema.Literal(true),
      parentReceiptRequirementsObserved: Schema.Literal(true),
      runtimeObservationRequirementsObserved: Schema.Literal(true),
      productReadinessBlocked: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      childRuntimeRequirementCoverageClaimed: Schema.Literal(true),
      childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
      childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
      renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptIngestionClaimed: Schema.Literal(false),
      liveLocationRuntimeClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productReadyClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length > 0 &&
          proof.rows.every(
            (row) => row.sourceSnapshotRequirementsProofRef === proof.sourceSnapshotRequirementsProofRef
          )) ||
        'Expected child runtime product-readiness blocker proof rows to cite one snapshot-requirements proof'
    )
  )
);

export type TrackingChildRuntimeProductReadinessBlockerProof = Infer<
  typeof TrackingChildRuntimeProductReadinessBlockerProofSchema
>;
type TrackingChildRuntimeProductReadinessBlockerRowInput = Infer<
  typeof TrackingChildRuntimeProductReadinessBlockerRowBaseSchema
>;

export function buildTrackingChildRuntimeProductReadinessBlockerProof(
  generatedAt: string,
  sourceSnapshotRequirementsProofRef: string,
  sourceSnapshotRequirementsProof: unknown
): TrackingChildRuntimeProductReadinessBlockerProof {
  const snapshotRequirements = TrackingChildRuntimeSnapshotRequirementsReadModelSchema.parse(
    snapshotRequirementsReadModelFrom(sourceSnapshotRequirementsProof)
  );

  return TrackingChildRuntimeProductReadinessBlockerProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-child-runtime-product-readiness-blocker-proof',
    generatedAt,
    sourceSnapshotRequirementsProofRef,
    sourceSnapshotRequirementsStatus: statusFrom(sourceSnapshotRequirementsProof),
    rows: snapshotRequirements.rows.map((row) => blockerRow(generatedAt, sourceSnapshotRequirementsProofRef, row)),
    proofClaims: {
      snapshotRequirementRowsObserved: true,
      deliveryEnvelopeRequirementsObserved: true,
      executionResultRequirementsObserved: true,
      visibleSnapshotRequirementsObserved: true,
      parentReceiptRequirementsObserved: true,
      runtimeObservationRequirementsObserved: true,
      productReadinessBlocked: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      childRuntimeRequirementCoverageClaimed: true,
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptIngestionClaimed: false,
      liveLocationRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productReadyClaimed: false,
    },
  });
}

function blockerRow(
  generatedAt: string,
  sourceSnapshotRequirementsProofRef: string,
  sourceRow: TrackingChildRuntimeSnapshotRequirementsRow
) {
  return TrackingChildRuntimeProductReadinessBlockerRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    blockerProofId: `tracking-child-runtime-product-readiness-blocked-${sourceRow.sourceCheckInId}`,
    generatedAt,
    sourceSnapshotRequirementsProofRef,
    sourceCheckInId: sourceRow.sourceCheckInId,
    sourceSnapshotRequirementRowId: sourceRow.rowId,
    auditRefs: [`tracking-child-runtime-product-readiness-blocker-audit-${sourceRow.sourceCheckInId}`],
    deliveryEnvelopeRef: sourceRow.deliveryEnvelopeRef,
    executionResultRequirementRefCount: sourceRow.executionResultRequirementRefs.length,
    visibleSnapshotRequirementRefCount: sourceRow.visibleSnapshotRequirementRefs.length,
    parentReceiptRequirementRefCount: sourceRow.parentReceiptRequirementRefs.length,
    runtimeObservationRequirementRefCount: sourceRow.runtimeObservationRequirementRefs.length,
    blockerRefs: [...RequiredTrackingChildRuntimeProductReadinessBlockers],
    deliveryEnvelopeRequirementClaimed: true,
    executionResultRequirementClaimed: true,
    visibleSnapshotRequirementClaimed: true,
    parentReceiptRequirementClaimed: true,
    runtimeObservationRequirementClaimed: true,
    childDeviceDeliveryRuntimeClaimed: false,
    childDeviceExecutionRuntimeClaimed: false,
    renderedChildDeviceUiRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptIngestionClaimed: false,
    liveLocationRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    productReadyClaimed: false,
  });
}

function snapshotRequirementsReadModelFrom(sourceSnapshotRequirementsProof: unknown): unknown {
  const candidate = sourceSnapshotRequirementsProof as { readonly readModel?: unknown };
  if (candidate.readModel === undefined) {
    throw new Error('Child runtime snapshot-requirements proof is missing readModel');
  }
  return candidate.readModel;
}

function statusFrom(sourceSnapshotRequirementsProof: unknown): string {
  const candidate = sourceSnapshotRequirementsProof as { readonly status?: unknown };
  if (typeof candidate.status !== 'string' || candidate.status.length === 0) {
    throw new Error('Child runtime snapshot-requirements proof is missing status');
  }
  return candidate.status;
}

function trackingChildRuntimeProductReadinessBlockerRowIsHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    row.deliveryEnvelopeRef.length > 0 &&
    row.executionResultRequirementRefCount > 0 &&
    row.visibleSnapshotRequirementRefCount > 0 &&
    row.parentReceiptRequirementRefCount > 0 &&
    row.runtimeObservationRequirementRefCount > 0 &&
    row.deliveryEnvelopeRequirementClaimed === true &&
    row.executionResultRequirementClaimed === true &&
    row.visibleSnapshotRequirementClaimed === true &&
    row.parentReceiptRequirementClaimed === true &&
    row.runtimeObservationRequirementClaimed === true &&
    trackingChildRuntimeProductReadinessBlockerRowNonClaimsAreHonest(row)
  );
}

function trackingChildRuntimeProductReadinessBlockerRowNonClaimsAreHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    row.childDeviceDeliveryRuntimeClaimed === false &&
    row.childDeviceExecutionRuntimeClaimed === false &&
    row.renderedChildDeviceUiRuntimeClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.notificationReceiptIngestionClaimed === false &&
    row.liveLocationRuntimeClaimed === false &&
    row.physicalDeviceProofClaimed === false &&
    row.authorityProofClaimed === false &&
    row.productionWorkerClaimed === false &&
    row.productReadyClaimed === false
  );
}
