import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import {
  TrackingChildRuntimeExecutionReadinessReadModelSchema,
  type TrackingChildRuntimeExecutionReadinessRow,
} from './tracking-child-runtime-execution-readiness-proof';
import { TrackingPolicySchemaVersion } from './tracking-location-policy';

export const TrackingChildRuntimeSnapshotRequirementKinds = [
  'delivery-envelope',
  'execution-result',
  'visible-snapshot',
  'parent-receipt',
  'runtime-observation',
] as const;

export const TrackingChildRuntimeSnapshotRequirementKindSchema = withParser(
  Schema.Literal(...TrackingChildRuntimeSnapshotRequirementKinds)
);

export const RequiredTrackingChildRuntimeSnapshotRequirementsNonClaims = [
  'no-child-device-delivery-runtime',
  'no-child-device-execution-runtime',
  'no-rendered-child-device-ui-runtime',
  'no-provider-delivery',
  'no-notification-receipt-ingestion',
  'no-live-location-runtime',
  'no-physical-device-proof',
  'no-authority-proof',
  'no-production-worker',
  'no-product-ready-claim',
] as const;

export const TrackingChildRuntimeSnapshotRequirementsNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingChildRuntimeSnapshotRequirementsNonClaims)
);

export const TrackingChildRuntimeSnapshotRequirementsIdSchema = brandedNonEmptyStringSchema(
  'TrackingChildRuntimeSnapshotRequirementsId'
);

export const TrackingChildRuntimeSnapshotRequirementsRowIdSchema = brandedNonEmptyStringSchema(
  'TrackingChildRuntimeSnapshotRequirementsRowId'
);

const TrackingChildRuntimeSnapshotRequirementsRowBaseSchema = Schema.Struct({
  rowId: TrackingChildRuntimeSnapshotRequirementsRowIdSchema,
  sourceReadinessRowId: NonEmptyStringSchema,
  sourceCheckInId: NonEmptyStringSchema,
  sourceReadinessState: NonEmptyStringSchema,
  sourceSnapshotKind: NonEmptyStringSchema,
  requiredSnapshotKinds: Schema.Array(TrackingChildRuntimeSnapshotRequirementKindSchema),
  deliveryEnvelopeRef: NonEmptyStringSchema,
  executionResultRequirementRefs: Schema.Array(NonEmptyStringSchema),
  visibleSnapshotRequirementRefs: Schema.Array(NonEmptyStringSchema),
  parentReceiptRequirementRefs: Schema.Array(NonEmptyStringSchema),
  runtimeObservationRequirementRefs: Schema.Array(NonEmptyStringSchema),
  hostedUiProofRefs: Schema.Array(NonEmptyStringSchema),
  sourceEvidenceRefs: Schema.Array(NonEmptyStringSchema),
  parentVisibleStatusRefs: Schema.Array(NonEmptyStringSchema),
  boundaryRuntimeProofRefs: Schema.Array(NonEmptyStringSchema),
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

export const TrackingChildRuntimeSnapshotRequirementsRowSchema = withParser(
  TrackingChildRuntimeSnapshotRequirementsRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingChildRuntimeSnapshotRequirementsRowIsHonest(row) ||
        'Expected child runtime snapshot requirement rows to preserve refs for delivery, execution, visible snapshot, receipt, and observation without runtime/device/product claims'
    )
  )
);

const TrackingChildRuntimeSnapshotRequirementsReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  snapshotRequirementsId: TrackingChildRuntimeSnapshotRequirementsIdSchema,
  generatedAt: NonEmptyStringSchema,
  sourceExecutionReadinessId: NonEmptyStringSchema,
  sourceExecutionReadinessGeneratedAt: NonEmptyStringSchema,
  sourceContractRefs: Schema.Array(NonEmptyStringSchema),
  rows: Schema.Array(TrackingChildRuntimeSnapshotRequirementsRowSchema),
  requiredSnapshotKindCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  deliveryEnvelopeRequirementCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  executionResultRequirementCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  visibleSnapshotRequirementCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  parentReceiptRequirementCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  runtimeObservationRequirementCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  snapshotRequirementsNonClaims: Schema.Array(TrackingChildRuntimeSnapshotRequirementsNonClaimSchema),
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

export const TrackingChildRuntimeSnapshotRequirementsReadModelSchema = withParser(
  TrackingChildRuntimeSnapshotRequirementsReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingChildRuntimeSnapshotRequirementsReadModelIsHonest(readModel) ||
        'Expected child runtime snapshot requirements proof to include complete per-row requirement counts and non-claims'
    )
  )
);

export type TrackingChildRuntimeSnapshotRequirementsRow = Infer<
  typeof TrackingChildRuntimeSnapshotRequirementsRowSchema
>;
export type TrackingChildRuntimeSnapshotRequirementsReadModel = Infer<
  typeof TrackingChildRuntimeSnapshotRequirementsReadModelSchema
>;
type TrackingChildRuntimeSnapshotRequirementsRowInput = Infer<
  typeof TrackingChildRuntimeSnapshotRequirementsRowBaseSchema
>;
type TrackingChildRuntimeSnapshotRequirementsReadModelInput = Infer<
  typeof TrackingChildRuntimeSnapshotRequirementsReadModelBaseSchema
>;

export type TrackingChildRuntimeSnapshotRequirementsOptions = {
  readonly generatedAt: string;
  readonly snapshotRequirementsId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildTrackingChildRuntimeSnapshotRequirementsReadModel(
  options: TrackingChildRuntimeSnapshotRequirementsOptions,
  sourceExecutionReadinessReadModel: unknown
): TrackingChildRuntimeSnapshotRequirementsReadModel {
  const parsed = TrackingChildRuntimeExecutionReadinessReadModelSchema.parse(sourceExecutionReadinessReadModel);
  const rows = parsed.rows.map(childRuntimeSnapshotRequirementsRowFor);

  return TrackingChildRuntimeSnapshotRequirementsReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    snapshotRequirementsId: options.snapshotRequirementsId,
    generatedAt: options.generatedAt,
    sourceExecutionReadinessId: parsed.readinessId,
    sourceExecutionReadinessGeneratedAt: parsed.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    requiredSnapshotKindCount: rows.reduce((count, row) => count + row.requiredSnapshotKinds.length, 0),
    deliveryEnvelopeRequirementCount: rows.filter((row) => row.deliveryEnvelopeRef.length > 0).length,
    executionResultRequirementCount: rows.reduce((count, row) => count + row.executionResultRequirementRefs.length, 0),
    visibleSnapshotRequirementCount: rows.reduce((count, row) => count + row.visibleSnapshotRequirementRefs.length, 0),
    parentReceiptRequirementCount: rows.reduce((count, row) => count + row.parentReceiptRequirementRefs.length, 0),
    runtimeObservationRequirementCount: rows.reduce(
      (count, row) => count + row.runtimeObservationRequirementRefs.length,
      0
    ),
    snapshotRequirementsNonClaims: RequiredTrackingChildRuntimeSnapshotRequirementsNonClaims,
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

function childRuntimeSnapshotRequirementsRowFor(
  sourceRow: TrackingChildRuntimeExecutionReadinessRow
): TrackingChildRuntimeSnapshotRequirementsRow {
  return TrackingChildRuntimeSnapshotRequirementsRowSchema.parse({
    rowId: `tracking-child-runtime-snapshot-requirements-${sourceRow.sourceCheckInId}`,
    sourceReadinessRowId: sourceRow.rowId,
    sourceCheckInId: sourceRow.sourceCheckInId,
    sourceReadinessState: sourceRow.readinessState,
    sourceSnapshotKind: sourceRow.snapshotKind,
    requiredSnapshotKinds: TrackingChildRuntimeSnapshotRequirementKinds,
    deliveryEnvelopeRef: sourceRow.deliveryEnvelopeRef,
    executionResultRequirementRefs: matchingRefs(sourceRow.executionRequirementRefs, 'execution-result'),
    visibleSnapshotRequirementRefs: matchingRefs(sourceRow.executionRequirementRefs, 'visible-snapshot'),
    parentReceiptRequirementRefs: matchingRefs(sourceRow.executionRequirementRefs, 'parent-receipt'),
    runtimeObservationRequirementRefs: sourceRow.runtimeObservationRequirementRefs,
    hostedUiProofRefs: sourceRow.hostedUiProofRefs,
    sourceEvidenceRefs: sourceRow.sourceEvidenceRefs,
    parentVisibleStatusRefs: sourceRow.parentVisibleStatusRefs,
    boundaryRuntimeProofRefs: sourceRow.boundaryRuntimeProofRefs,
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

function matchingRefs(refs: readonly string[], pattern: string): readonly string[] {
  return refs.filter((ref) => ref.includes(pattern));
}

function trackingChildRuntimeSnapshotRequirementsRowIsHonest(
  row: TrackingChildRuntimeSnapshotRequirementsRowInput
): boolean {
  return (
    row.requiredSnapshotKinds.length === TrackingChildRuntimeSnapshotRequirementKinds.length &&
    row.deliveryEnvelopeRef.length > 0 &&
    row.executionResultRequirementRefs.length > 0 &&
    row.visibleSnapshotRequirementRefs.length > 0 &&
    row.parentReceiptRequirementRefs.length > 0 &&
    row.runtimeObservationRequirementRefs.length > 0 &&
    row.hostedUiProofRefs.length > 0 &&
    row.sourceEvidenceRefs.length > 0 &&
    row.parentVisibleStatusRefs.length > 0 &&
    row.boundaryRuntimeProofRefs.length > 0 &&
    trackingChildRuntimeSnapshotRequirementsRowNonClaimsAreHonest(row)
  );
}

function trackingChildRuntimeSnapshotRequirementsRowNonClaimsAreHonest(
  row: TrackingChildRuntimeSnapshotRequirementsRowInput
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

function trackingChildRuntimeSnapshotRequirementsReadModelIsHonest(
  readModel: TrackingChildRuntimeSnapshotRequirementsReadModelInput
): boolean {
  return (
    readModel.rows.length > 0 &&
    readModel.snapshotRequirementsNonClaims.length ===
      RequiredTrackingChildRuntimeSnapshotRequirementsNonClaims.length &&
    readModel.requiredSnapshotKindCount ===
      readModel.rows.length * TrackingChildRuntimeSnapshotRequirementKinds.length &&
    readModel.deliveryEnvelopeRequirementCount === readModel.rows.length &&
    readModel.executionResultRequirementCount ===
      readModel.rows.reduce((count, row) => count + row.executionResultRequirementRefs.length, 0) &&
    readModel.visibleSnapshotRequirementCount ===
      readModel.rows.reduce((count, row) => count + row.visibleSnapshotRequirementRefs.length, 0) &&
    readModel.parentReceiptRequirementCount ===
      readModel.rows.reduce((count, row) => count + row.parentReceiptRequirementRefs.length, 0) &&
    readModel.runtimeObservationRequirementCount ===
      readModel.rows.reduce((count, row) => count + row.runtimeObservationRequirementRefs.length, 0) &&
    trackingChildRuntimeSnapshotRequirementsReadModelNonClaimsAreHonest(readModel)
  );
}

function trackingChildRuntimeSnapshotRequirementsReadModelNonClaimsAreHonest(
  readModel: TrackingChildRuntimeSnapshotRequirementsReadModelInput
): boolean {
  return (
    readModel.childDeviceDeliveryRuntimeClaimed === false &&
    readModel.childDeviceExecutionRuntimeClaimed === false &&
    readModel.renderedChildDeviceUiRuntimeClaimed === false &&
    readModel.providerDeliveryClaimed === false &&
    readModel.notificationReceiptIngestionClaimed === false &&
    readModel.liveLocationRuntimeClaimed === false &&
    readModel.physicalDeviceProofClaimed === false &&
    readModel.authorityProofClaimed === false &&
    readModel.productionWorkerClaimed === false &&
    readModel.productReadyClaimed === false
  );
}
