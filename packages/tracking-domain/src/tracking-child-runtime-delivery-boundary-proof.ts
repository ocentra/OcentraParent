import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  TrackingChildCheckInTimeoutReadModelSchema,
  type TrackingChildCheckInTimeoutRow,
} from './tracking-child-check-in-timeout-escalation-proof';
import { TrackingPolicySchemaVersion } from '@ocentra-parent/schema-domain/tracking-location-policy';

export const RequiredTrackingChildRuntimeDeliveryBoundaryNonClaims = [
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

export const TrackingChildRuntimeDeliveryBoundaryNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingChildRuntimeDeliveryBoundaryNonClaims)
);

export const TrackingChildRuntimeDeliveryBoundaryReadinessIdSchema = brandedNonEmptyStringSchema('TrackingChildRuntimeDeliveryBoundaryReadinessId');
export const TrackingChildRuntimeDeliveryBoundaryRowIdSchema = brandedNonEmptyStringSchema('TrackingChildRuntimeDeliveryBoundaryRowId');
export const TrackingChildRuntimeDeliveryBoundaryStateSchema = withParser(
  Schema.Literal(
    'hosted-copy-only-waiting',
    'hosted-copy-only-safe-response',
    'hosted-copy-only-escalation-ready',
    'manual-runtime-proof-required'
  )
);
export const TrackingChildRuntimeDeliveryBoundaryUiStateSchema = withParser(
  Schema.Literal('delivery-disclosure', 'safe-response-disclosure', 'help-response-disclosure', 'timeout-disclosure')
);

const TrackingChildRuntimeDeliveryBoundaryRowBaseSchema = Schema.Struct({
  rowId: TrackingChildRuntimeDeliveryBoundaryRowIdSchema,
  sourceCheckInId: NonEmptyStringSchema,
  sourceResolutionState: NonEmptyStringSchema,
  boundaryState: TrackingChildRuntimeDeliveryBoundaryStateSchema,
  hostedUiState: TrackingChildRuntimeDeliveryBoundaryUiStateSchema,
  hostedUiProofRefs: Schema.Array(NonEmptyStringSchema),
  sourceEvidenceRefs: Schema.Array(NonEmptyStringSchema),
  sourceAuditRefs: Schema.Array(NonEmptyStringSchema),
  requiredRuntimeProofRefs: Schema.Array(NonEmptyStringSchema),
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

export const TrackingChildRuntimeDeliveryBoundaryRowSchema = withParser(
  TrackingChildRuntimeDeliveryBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingChildRuntimeDeliveryBoundaryRowIsHonest(row) ||
        'Expected child runtime delivery boundary rows to preserve refs and avoid device/runtime/provider/authority/product claims'
    )
  )
);

const TrackingChildRuntimeDeliveryBoundaryReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  readinessId: TrackingChildRuntimeDeliveryBoundaryReadinessIdSchema,
  generatedAt: NonEmptyStringSchema,
  sourceTimeoutReadinessId: NonEmptyStringSchema,
  sourceTimeoutGeneratedAt: NonEmptyStringSchema,
  sourceContractRefs: Schema.Array(NonEmptyStringSchema),
  hostedUiProofRefs: Schema.Array(NonEmptyStringSchema),
  rows: Schema.Array(TrackingChildRuntimeDeliveryBoundaryRowSchema),
  hostedCopyOnlyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  safeResponseDisclosureCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  escalationDisclosureCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRuntimeProofRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  requiredRuntimeProofRefCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  readinessNonClaims: Schema.Array(TrackingChildRuntimeDeliveryBoundaryNonClaimSchema),
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

export const TrackingChildRuntimeDeliveryBoundaryReadModelSchema = withParser(
  TrackingChildRuntimeDeliveryBoundaryReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingChildRuntimeDeliveryBoundaryReadModelIsHonest(readModel) ||
        'Expected child runtime delivery boundary proof to include rows, counts, proof refs, and all required non-claims'
    )
  )
);

export type TrackingChildRuntimeDeliveryBoundaryRow = Infer<typeof TrackingChildRuntimeDeliveryBoundaryRowSchema>;
export type TrackingChildRuntimeDeliveryBoundaryReadModel = Infer<
  typeof TrackingChildRuntimeDeliveryBoundaryReadModelSchema
>;

type TrackingChildRuntimeDeliveryBoundaryRowInput = Infer<typeof TrackingChildRuntimeDeliveryBoundaryRowBaseSchema>;
type TrackingChildRuntimeDeliveryBoundaryReadModelInput = Infer<
  typeof TrackingChildRuntimeDeliveryBoundaryReadModelBaseSchema
>;

export type TrackingChildRuntimeDeliveryBoundaryOptions = {
  readonly generatedAt: string;
  readonly readinessId: string;
  readonly sourceContractRefs: readonly string[];
  readonly hostedUiProofRefs: readonly string[];
};

export function buildTrackingChildRuntimeDeliveryBoundaryReadModel(
  options: TrackingChildRuntimeDeliveryBoundaryOptions,
  sourceTimeoutReadModel: unknown
): TrackingChildRuntimeDeliveryBoundaryReadModel {
  const parsed = TrackingChildCheckInTimeoutReadModelSchema.parse(sourceTimeoutReadModel);
  const rows = parsed.rows.map((row) => childRuntimeDeliveryBoundaryRowFor(row, options.hostedUiProofRefs));

  return TrackingChildRuntimeDeliveryBoundaryReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    readinessId: options.readinessId,
    generatedAt: options.generatedAt,
    sourceTimeoutReadinessId: parsed.readinessId,
    sourceTimeoutGeneratedAt: parsed.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    hostedUiProofRefs: options.hostedUiProofRefs,
    rows,
    hostedCopyOnlyCount: rows.length,
    safeResponseDisclosureCount: rows.filter((row) => row.hostedUiState === 'safe-response-disclosure').length,
    escalationDisclosureCount: rows.filter(
      (row) => row.hostedUiState === 'help-response-disclosure' || row.hostedUiState === 'timeout-disclosure'
    ).length,
    manualRuntimeProofRequiredCount: rows.filter((row) => row.boundaryState === 'manual-runtime-proof-required').length,
    requiredRuntimeProofRefCount: rows.reduce((count, row) => count + row.requiredRuntimeProofRefs.length, 0),
    readinessNonClaims: RequiredTrackingChildRuntimeDeliveryBoundaryNonClaims,
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

function childRuntimeDeliveryBoundaryRowFor(
  sourceRow: TrackingChildCheckInTimeoutRow,
  hostedUiProofRefs: readonly string[]
): TrackingChildRuntimeDeliveryBoundaryRow {
  return TrackingChildRuntimeDeliveryBoundaryRowSchema.parse({
    rowId: `tracking-child-runtime-delivery-boundary-${sourceRow.checkInId}`,
    sourceCheckInId: sourceRow.checkInId,
    sourceResolutionState: sourceRow.resolutionState,
    boundaryState: boundaryStateFor(sourceRow),
    hostedUiState: hostedUiStateFor(sourceRow),
    hostedUiProofRefs,
    sourceEvidenceRefs: sourceRow.evidenceReferenceIds,
    sourceAuditRefs: sourceRow.auditRefs,
    requiredRuntimeProofRefs: requiredRuntimeProofRefsFor(sourceRow),
    parentVisibleStatusRefs: parentVisibleStatusRefsFor(sourceRow),
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

function boundaryStateFor(
  sourceRow: TrackingChildCheckInTimeoutRow
):
  | 'hosted-copy-only-waiting'
  | 'hosted-copy-only-safe-response'
  | 'hosted-copy-only-escalation-ready'
  | 'manual-runtime-proof-required' {
  if (sourceRow.resolutionState === 'waiting-for-child') {
    return 'hosted-copy-only-waiting';
  }
  if (sourceRow.resolutionState === 'safe-response-recorded') {
    return 'hosted-copy-only-safe-response';
  }
  if (sourceRow.resolutionState === 'manual-required') {
    return 'manual-runtime-proof-required';
  }
  return 'hosted-copy-only-escalation-ready';
}

function hostedUiStateFor(
  sourceRow: TrackingChildCheckInTimeoutRow
): 'delivery-disclosure' | 'safe-response-disclosure' | 'help-response-disclosure' | 'timeout-disclosure' {
  if (sourceRow.resolutionState === 'safe-response-recorded') {
    return 'safe-response-disclosure';
  }
  if (
    sourceRow.escalationBasis === 'child-help-response' ||
    sourceRow.escalationBasis === 'child-call-parent-response'
  ) {
    return 'help-response-disclosure';
  }
  if (sourceRow.escalationBasis === 'expired-rule-only-timeout') {
    return 'timeout-disclosure';
  }
  return 'delivery-disclosure';
}

function requiredRuntimeProofRefsFor(sourceRow: TrackingChildCheckInTimeoutRow): readonly string[] {
  return [
    `child-device-delivery-runtime-proof-required-${sourceRow.checkInId}`,
    `child-device-execution-runtime-proof-required-${sourceRow.checkInId}`,
    `rendered-child-device-ui-runtime-proof-required-${sourceRow.checkInId}`,
    `physical-device-proof-required-${sourceRow.checkInId}`,
    `authority-proof-required-${sourceRow.checkInId}`,
  ];
}

function parentVisibleStatusRefsFor(sourceRow: TrackingChildCheckInTimeoutRow): readonly string[] {
  return [
    `hosted-child-runtime-disclosure-${sourceRow.checkInId}`,
    ...sourceRow.parentActionRefs,
    ...sourceRow.manualProofRequirements,
  ];
}

function trackingChildRuntimeDeliveryBoundaryRowIsHonest(row: TrackingChildRuntimeDeliveryBoundaryRowInput): boolean {
  return (
    row.hostedUiProofRefs.length > 0 &&
    row.sourceEvidenceRefs.length > 0 &&
    row.sourceAuditRefs.length > 0 &&
    row.requiredRuntimeProofRefs.length >= 5 &&
    row.parentVisibleStatusRefs.length > 0 &&
    trackingChildRuntimeDeliveryBoundaryRowNonClaimsAreHonest(row)
  );
}

function trackingChildRuntimeDeliveryBoundaryRowNonClaimsAreHonest(
  row: TrackingChildRuntimeDeliveryBoundaryRowInput
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

function trackingChildRuntimeDeliveryBoundaryReadModelIsHonest(
  readModel: TrackingChildRuntimeDeliveryBoundaryReadModelInput
): boolean {
  return (
    readModel.rows.length > 0 &&
    readModel.hostedUiProofRefs.length > 0 &&
    readModel.readinessNonClaims.length === RequiredTrackingChildRuntimeDeliveryBoundaryNonClaims.length &&
    readModel.hostedCopyOnlyCount === readModel.rows.length &&
    readModel.safeResponseDisclosureCount ===
      readModel.rows.filter((row) => row.hostedUiState === 'safe-response-disclosure').length &&
    readModel.escalationDisclosureCount ===
      readModel.rows.filter(
        (row) => row.hostedUiState === 'help-response-disclosure' || row.hostedUiState === 'timeout-disclosure'
      ).length &&
    readModel.manualRuntimeProofRequiredCount ===
      readModel.rows.filter((row) => row.boundaryState === 'manual-runtime-proof-required').length &&
    readModel.requiredRuntimeProofRefCount ===
      readModel.rows.reduce((count, row) => count + row.requiredRuntimeProofRefs.length, 0) &&
    trackingChildRuntimeDeliveryBoundaryReadModelNonClaimsAreHonest(readModel)
  );
}

function trackingChildRuntimeDeliveryBoundaryReadModelNonClaimsAreHonest(
  readModel: TrackingChildRuntimeDeliveryBoundaryReadModelInput
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

