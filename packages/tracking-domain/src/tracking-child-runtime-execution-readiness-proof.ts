import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  TrackingChildRuntimeDeliveryBoundaryReadModelSchema,
  type TrackingChildRuntimeDeliveryBoundaryRow,
} from './tracking-child-runtime-delivery-boundary-proof';
import { TrackingPolicySchemaVersion } from './tracking-location-policy';

export const RequiredTrackingChildRuntimeExecutionReadinessNonClaims = [
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

export const TrackingChildRuntimeExecutionReadinessNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingChildRuntimeExecutionReadinessNonClaims)
);

export const TrackingChildRuntimeExecutionReadinessIdSchema = brandedNonEmptyStringSchema('TrackingChildRuntimeExecutionReadinessId');
export const TrackingChildRuntimeExecutionReadinessRowIdSchema = brandedNonEmptyStringSchema('TrackingChildRuntimeExecutionReadinessRowId');
export const TrackingChildRuntimeExecutionReadinessStateSchema = withParser(
  Schema.Literal(
    'delivery-envelope-ready',
    'safe-response-execution-ready',
    'escalation-execution-ready',
    'manual-runtime-proof-required'
  )
);
export const TrackingChildRuntimeSnapshotKindSchema = withParser(
  Schema.Literal('delivery-disclosure-snapshot', 'safe-response-snapshot', 'help-response-snapshot', 'timeout-snapshot')
);

const TrackingChildRuntimeExecutionReadinessRowBaseSchema = Schema.Struct({
  rowId: TrackingChildRuntimeExecutionReadinessRowIdSchema,
  sourceBoundaryRowId: NonEmptyStringSchema,
  sourceCheckInId: NonEmptyStringSchema,
  sourceBoundaryState: NonEmptyStringSchema,
  readinessState: TrackingChildRuntimeExecutionReadinessStateSchema,
  snapshotKind: TrackingChildRuntimeSnapshotKindSchema,
  deliveryEnvelopeRef: NonEmptyStringSchema,
  executionRequirementRefs: Schema.Array(NonEmptyStringSchema),
  runtimeObservationRequirementRefs: Schema.Array(NonEmptyStringSchema),
  hostedUiProofRefs: Schema.Array(NonEmptyStringSchema),
  sourceEvidenceRefs: Schema.Array(NonEmptyStringSchema),
  sourceAuditRefs: Schema.Array(NonEmptyStringSchema),
  boundaryRuntimeProofRefs: Schema.Array(NonEmptyStringSchema),
  parentVisibleStatusRefs: Schema.Array(NonEmptyStringSchema),
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

export const TrackingChildRuntimeExecutionReadinessRowSchema = withParser(
  TrackingChildRuntimeExecutionReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingChildRuntimeExecutionReadinessRowIsHonest(row) ||
        'Expected child runtime execution readiness rows to preserve boundary refs and avoid device/runtime/provider/authority/product claims'
    )
  )
);

const TrackingChildRuntimeExecutionReadinessReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  readinessId: TrackingChildRuntimeExecutionReadinessIdSchema,
  generatedAt: NonEmptyStringSchema,
  sourceBoundaryReadinessId: NonEmptyStringSchema,
  sourceBoundaryGeneratedAt: NonEmptyStringSchema,
  sourceContractRefs: Schema.Array(NonEmptyStringSchema),
  rows: Schema.Array(TrackingChildRuntimeExecutionReadinessRowSchema),
  deliveryEnvelopeReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  safeResponseExecutionReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  escalationExecutionReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRuntimeProofRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  executionRequirementRefCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  runtimeObservationRequirementRefCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  readinessNonClaims: Schema.Array(TrackingChildRuntimeExecutionReadinessNonClaimSchema),
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

export const TrackingChildRuntimeExecutionReadinessReadModelSchema = withParser(
  TrackingChildRuntimeExecutionReadinessReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingChildRuntimeExecutionReadinessReadModelIsHonest(readModel) ||
        'Expected child runtime execution readiness proof to include rows, counts, requirement refs, and all required non-claims'
    )
  )
);

export type TrackingChildRuntimeExecutionReadinessRow = Infer<typeof TrackingChildRuntimeExecutionReadinessRowSchema>;
export type TrackingChildRuntimeExecutionReadinessReadModel = Infer<
  typeof TrackingChildRuntimeExecutionReadinessReadModelSchema
>;

type TrackingChildRuntimeExecutionReadinessRowInput = Infer<typeof TrackingChildRuntimeExecutionReadinessRowBaseSchema>;
type TrackingChildRuntimeExecutionReadinessReadModelInput = Infer<
  typeof TrackingChildRuntimeExecutionReadinessReadModelBaseSchema
>;

export type TrackingChildRuntimeExecutionReadinessOptions = {
  readonly generatedAt: string;
  readonly readinessId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildTrackingChildRuntimeExecutionReadinessReadModel(
  options: TrackingChildRuntimeExecutionReadinessOptions,
  sourceBoundaryReadModel: unknown
): TrackingChildRuntimeExecutionReadinessReadModel {
  const parsed = TrackingChildRuntimeDeliveryBoundaryReadModelSchema.parse(sourceBoundaryReadModel);
  const rows = parsed.rows.map(childRuntimeExecutionReadinessRowFor);

  return TrackingChildRuntimeExecutionReadinessReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    readinessId: options.readinessId,
    generatedAt: options.generatedAt,
    sourceBoundaryReadinessId: parsed.readinessId,
    sourceBoundaryGeneratedAt: parsed.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    deliveryEnvelopeReadyCount: rows.length,
    safeResponseExecutionReadyCount: rows.filter((row) => row.readinessState === 'safe-response-execution-ready')
      .length,
    escalationExecutionReadyCount: rows.filter((row) => row.readinessState === 'escalation-execution-ready').length,
    manualRuntimeProofRequiredCount: rows.filter((row) => row.readinessState === 'manual-runtime-proof-required')
      .length,
    executionRequirementRefCount: rows.reduce((count, row) => count + row.executionRequirementRefs.length, 0),
    runtimeObservationRequirementRefCount: rows.reduce(
      (count, row) => count + row.runtimeObservationRequirementRefs.length,
      0
    ),
    readinessNonClaims: RequiredTrackingChildRuntimeExecutionReadinessNonClaims,
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

function childRuntimeExecutionReadinessRowFor(
  sourceRow: TrackingChildRuntimeDeliveryBoundaryRow
): TrackingChildRuntimeExecutionReadinessRow {
  return TrackingChildRuntimeExecutionReadinessRowSchema.parse({
    rowId: `tracking-child-runtime-execution-readiness-${sourceRow.sourceCheckInId}`,
    sourceBoundaryRowId: sourceRow.rowId,
    sourceCheckInId: sourceRow.sourceCheckInId,
    sourceBoundaryState: sourceRow.boundaryState,
    readinessState: readinessStateFor(sourceRow),
    snapshotKind: snapshotKindFor(sourceRow),
    deliveryEnvelopeRef: `tracking-child-runtime-delivery-envelope-${sourceRow.sourceCheckInId}`,
    executionRequirementRefs: executionRequirementRefsFor(sourceRow),
    runtimeObservationRequirementRefs: runtimeObservationRequirementRefsFor(sourceRow),
    hostedUiProofRefs: sourceRow.hostedUiProofRefs,
    sourceEvidenceRefs: sourceRow.sourceEvidenceRefs,
    sourceAuditRefs: sourceRow.sourceAuditRefs,
    boundaryRuntimeProofRefs: sourceRow.requiredRuntimeProofRefs,
    parentVisibleStatusRefs: sourceRow.parentVisibleStatusRefs,
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

function readinessStateFor(
  sourceRow: TrackingChildRuntimeDeliveryBoundaryRow
):
  | 'delivery-envelope-ready'
  | 'safe-response-execution-ready'
  | 'escalation-execution-ready'
  | 'manual-runtime-proof-required' {
  if (sourceRow.boundaryState === 'manual-runtime-proof-required') {
    return 'manual-runtime-proof-required';
  }
  if (sourceRow.hostedUiState === 'safe-response-disclosure') {
    return 'safe-response-execution-ready';
  }
  if (sourceRow.hostedUiState === 'help-response-disclosure' || sourceRow.hostedUiState === 'timeout-disclosure') {
    return 'escalation-execution-ready';
  }
  return 'delivery-envelope-ready';
}

function snapshotKindFor(
  sourceRow: TrackingChildRuntimeDeliveryBoundaryRow
): 'delivery-disclosure-snapshot' | 'safe-response-snapshot' | 'help-response-snapshot' | 'timeout-snapshot' {
  if (sourceRow.hostedUiState === 'safe-response-disclosure') {
    return 'safe-response-snapshot';
  }
  if (sourceRow.hostedUiState === 'help-response-disclosure') {
    return 'help-response-snapshot';
  }
  if (sourceRow.hostedUiState === 'timeout-disclosure') {
    return 'timeout-snapshot';
  }
  return 'delivery-disclosure-snapshot';
}

function executionRequirementRefsFor(sourceRow: TrackingChildRuntimeDeliveryBoundaryRow): readonly string[] {
  return [
    `child-runtime-delivery-envelope-proof-required-${sourceRow.sourceCheckInId}`,
    `child-runtime-execution-result-proof-required-${sourceRow.sourceCheckInId}`,
    `child-runtime-visible-snapshot-proof-required-${sourceRow.sourceCheckInId}`,
    `child-runtime-parent-receipt-proof-required-${sourceRow.sourceCheckInId}`,
  ];
}

function runtimeObservationRequirementRefsFor(sourceRow: TrackingChildRuntimeDeliveryBoundaryRow): readonly string[] {
  return [
    `child-runtime-device-observation-required-${sourceRow.sourceCheckInId}`,
    `child-runtime-result-receipt-required-${sourceRow.sourceCheckInId}`,
  ];
}

function trackingChildRuntimeExecutionReadinessRowIsHonest(
  row: TrackingChildRuntimeExecutionReadinessRowInput
): boolean {
  return (
    row.deliveryEnvelopeRef.length > 0 &&
    row.executionRequirementRefs.length === 4 &&
    row.runtimeObservationRequirementRefs.length === 2 &&
    row.hostedUiProofRefs.length > 0 &&
    row.sourceEvidenceRefs.length > 0 &&
    row.sourceAuditRefs.length > 0 &&
    row.boundaryRuntimeProofRefs.length > 0 &&
    row.parentVisibleStatusRefs.length > 0 &&
    trackingChildRuntimeExecutionReadinessRowNonClaimsAreHonest(row)
  );
}

function trackingChildRuntimeExecutionReadinessRowNonClaimsAreHonest(
  row: TrackingChildRuntimeExecutionReadinessRowInput
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

function trackingChildRuntimeExecutionReadinessReadModelIsHonest(
  readModel: TrackingChildRuntimeExecutionReadinessReadModelInput
): boolean {
  return (
    readModel.rows.length > 0 &&
    readModel.readinessNonClaims.length === RequiredTrackingChildRuntimeExecutionReadinessNonClaims.length &&
    readModel.deliveryEnvelopeReadyCount === readModel.rows.length &&
    readModel.safeResponseExecutionReadyCount ===
      readModel.rows.filter((row) => row.readinessState === 'safe-response-execution-ready').length &&
    readModel.escalationExecutionReadyCount ===
      readModel.rows.filter((row) => row.readinessState === 'escalation-execution-ready').length &&
    readModel.manualRuntimeProofRequiredCount ===
      readModel.rows.filter((row) => row.readinessState === 'manual-runtime-proof-required').length &&
    readModel.executionRequirementRefCount ===
      readModel.rows.reduce((count, row) => count + row.executionRequirementRefs.length, 0) &&
    readModel.runtimeObservationRequirementRefCount ===
      readModel.rows.reduce((count, row) => count + row.runtimeObservationRequirementRefs.length, 0) &&
    trackingChildRuntimeExecutionReadinessReadModelNonClaimsAreHonest(readModel)
  );
}

function trackingChildRuntimeExecutionReadinessReadModelNonClaimsAreHonest(
  readModel: TrackingChildRuntimeExecutionReadinessReadModelInput
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

