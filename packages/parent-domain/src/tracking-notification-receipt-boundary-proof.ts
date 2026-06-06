import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema } from './references';
import {
  RequiredTrackingProviderNotificationProofNonClaims,
  TrackingProviderNotificationProofReadModelSchema,
  type TrackingProviderNotificationProofReadModel,
  type TrackingProviderNotificationProofRow,
} from './tracking-provider-notification-proof';
import { V08NotificationProviderStatusBoundaryReadModel } from './v0-8-notification-provider-status-boundary';

const TrackingNotificationReceiptText = Schema.String.pipe(Schema.minLength(1));

export const RequiredTrackingNotificationReceiptBoundaryNonClaims = [
  'no-webhook-receipt-ingestion-runtime',
  'no-provider-delivery-execution',
  'no-provider-credentials',
  'no-adapter-dispatch',
  'no-retry-worker-runtime',
  'no-quiet-hours-timer-runtime',
  'no-parent-notification-ui',
  'no-child-device-delivery',
  'no-mobile-physical-device-proof',
  'no-authority-proof',
  'no-production-durable-outbox-storage',
] as const;

export const TrackingNotificationReceiptBoundaryNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingNotificationReceiptBoundaryNonClaims)
);

export const TrackingNotificationReceiptBoundaryProofIdSchema = TrackingNotificationReceiptText.pipe(
  Schema.brand('TrackingNotificationReceiptBoundaryProofId')
);
export const TrackingNotificationReceiptBoundaryReferenceSchema = TrackingNotificationReceiptText.pipe(
  Schema.brand('TrackingNotificationReceiptBoundaryReference')
);

export const TrackingNotificationReceiptBoundaryStateSchema = withParser(
  Schema.Literal('receipt-ingestion-required', 'manual-receipt-required', 'provider-unavailable')
);

const TrackingNotificationReceiptBoundaryRowBaseSchema = Schema.Struct({
  rowId: TrackingNotificationReceiptBoundaryReferenceSchema,
  sourceProviderProofRowRef: TrackingNotificationReceiptBoundaryReferenceSchema,
  sourceAlertId: TrackingNotificationReceiptBoundaryReferenceSchema,
  sourcePolicyDecisionId: TrackingNotificationReceiptBoundaryReferenceSchema,
  sourceProviderStatusEntryRef: TrackingNotificationReceiptBoundaryReferenceSchema,
  receiptBoundaryState: TrackingNotificationReceiptBoundaryStateSchema,
  evidenceRefs: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  notificationStatusRefs: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  reasonCodeRefs: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  providerAttemptRef: TrackingNotificationReceiptBoundaryReferenceSchema,
  providerReceiptRefs: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  auditRefs: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  manualProofRequirements: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  receiptIngestionProofRequirements: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  providerDeliveryClaimed: Schema.Literal(false),
  webhookReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
});

export const TrackingNotificationReceiptBoundaryRowSchema = withParser(
  TrackingNotificationReceiptBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingNotificationReceiptBoundaryRowIsHonest(row) ||
        'Expected tracking notification receipt boundary rows to preserve provider proof refs and keep receipt ingestion/delivery unclaimed'
    )
  )
);

const TrackingNotificationReceiptBoundaryReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingNotificationReceiptBoundaryProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceProviderNotificationProofRef: TrackingNotificationReceiptBoundaryReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  providerStatusBoundaryReadModelRef: TrackingNotificationReceiptBoundaryReferenceSchema,
  providerReceiptRequiredCoverageRef: TrackingNotificationReceiptBoundaryReferenceSchema,
  sourceProviderProofNonClaims: Schema.Array(TrackingNotificationReceiptBoundaryReferenceSchema),
  rows: Schema.Array(TrackingNotificationReceiptBoundaryRowSchema),
  receiptIngestionRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualReceiptRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  proofNonClaims: Schema.Array(TrackingNotificationReceiptBoundaryNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  webhookReceiptIngestionRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
});

export const TrackingNotificationReceiptBoundaryReadModelSchema = withParser(
  TrackingNotificationReceiptBoundaryReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingNotificationReceiptBoundaryReadModelIsHonest(readModel) ||
        'Expected tracking notification receipt proof counts, source non-claims, and no-claim flags to match receipt-boundary rows'
    )
  )
);

export type TrackingNotificationReceiptBoundaryState = Infer<typeof TrackingNotificationReceiptBoundaryStateSchema>;
export type TrackingNotificationReceiptBoundaryRow = Infer<typeof TrackingNotificationReceiptBoundaryRowSchema>;
export type TrackingNotificationReceiptBoundaryReadModel = Infer<
  typeof TrackingNotificationReceiptBoundaryReadModelSchema
>;

export type TrackingNotificationReceiptBoundaryProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly sourceProviderNotificationProofRef: string;
  readonly sourceContractRefs: readonly string[];
};

type TrackingNotificationReceiptBoundaryRowInput = Infer<typeof TrackingNotificationReceiptBoundaryRowBaseSchema>;
type TrackingNotificationReceiptBoundaryReadModelInput = Infer<
  typeof TrackingNotificationReceiptBoundaryReadModelBaseSchema
>;

export function buildTrackingNotificationReceiptBoundaryReadModel(
  options: TrackingNotificationReceiptBoundaryProofOptions,
  sourceProviderProof: TrackingProviderNotificationProofReadModel
): TrackingNotificationReceiptBoundaryReadModel {
  const parsedSource = TrackingProviderNotificationProofReadModelSchema.parse(sourceProviderProof);
  const rows = parsedSource.rows.map((row) => trackingNotificationReceiptBoundaryRowForProviderRow(row));

  return TrackingNotificationReceiptBoundaryReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: { familyId: options.familyId },
    sourceProviderNotificationProofRef: options.sourceProviderNotificationProofRef,
    sourceContractRefs: options.sourceContractRefs,
    providerStatusBoundaryReadModelRef: V08NotificationProviderStatusBoundaryReadModel.readModelId,
    providerReceiptRequiredCoverageRef: 'notification-provider-delivered-receipt-required',
    sourceProviderProofNonClaims: RequiredTrackingProviderNotificationProofNonClaims,
    rows,
    receiptIngestionRequiredCount: countReceiptState(rows, 'receipt-ingestion-required'),
    manualReceiptRequiredCount: countReceiptState(rows, 'manual-receipt-required'),
    providerUnavailableCount: countReceiptState(rows, 'provider-unavailable'),
    proofNonClaims: RequiredTrackingNotificationReceiptBoundaryNonClaims,
    providerDeliveryRuntimeClaimed: false,
    webhookReceiptIngestionRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    adapterDispatchClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    parentNotificationUiClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionDurableOutboxStorageClaimed: false,
  });
}

function trackingNotificationReceiptBoundaryRowForProviderRow(
  row: TrackingProviderNotificationProofRow
): TrackingNotificationReceiptBoundaryRow {
  const receiptBoundaryState = receiptBoundaryStateForProviderRow(row);
  const receiptIngestionProofRequirements = receiptRequirementsFor(row, receiptBoundaryState);

  return TrackingNotificationReceiptBoundaryRowSchema.parse({
    rowId: `tracking-notification-receipt-${row.sourceAlertId}`,
    sourceProviderProofRowRef: row.rowId,
    sourceAlertId: row.sourceAlertId,
    sourcePolicyDecisionId: row.sourcePolicyDecisionId,
    sourceProviderStatusEntryRef: row.providerStatusBoundaryEntry.statusEntryId,
    receiptBoundaryState,
    evidenceRefs: row.evidenceRefs,
    notificationStatusRefs: row.notificationStatusRefs,
    reasonCodeRefs: row.reasonCodeRefs,
    providerAttemptRef: row.providerStatusBoundaryEntry.providerAttemptRef,
    providerReceiptRefs: row.providerStatusBoundaryEntry.providerReceiptRefs,
    auditRefs: row.providerStatusBoundaryEntry.auditRefs,
    manualProofRequirements: row.manualProofRequirements,
    receiptIngestionProofRequirements,
    providerDeliveryClaimed: false,
    webhookReceiptIngestionClaimed: false,
    providerCredentialClaimed: false,
    adapterDispatchClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
  });
}

function receiptBoundaryStateForProviderRow(
  row: TrackingProviderNotificationProofRow
): TrackingNotificationReceiptBoundaryState {
  if (row.providerStatusKind === 'unavailable') {
    return 'provider-unavailable';
  }
  if (row.providerStatusKind === 'manual-required') {
    return 'manual-receipt-required';
  }
  return 'receipt-ingestion-required';
}

function receiptRequirementsFor(
  row: TrackingProviderNotificationProofRow,
  state: TrackingNotificationReceiptBoundaryState
): readonly string[] {
  if (state === 'provider-unavailable') {
    return [`tracking-receipt-provider-unavailable-${row.sourceAlertId}`];
  }
  if (state === 'manual-receipt-required') {
    return [
      `tracking-receipt-manual-provider-setup-${row.sourceAlertId}`,
      `tracking-receipt-critical-escalation-review-${row.sourceAlertId}`,
    ];
  }
  return [
    `tracking-receipt-webhook-contract-required-${row.sourceAlertId}`,
    `tracking-receipt-provider-attempt-required-${row.sourceAlertId}`,
  ];
}

function trackingNotificationReceiptBoundaryRowIsHonest(row: TrackingNotificationReceiptBoundaryRowInput): boolean {
  return (
    row.evidenceRefs.length > 0 &&
    row.reasonCodeRefs.length > 0 &&
    row.auditRefs.length > 0 &&
    row.manualProofRequirements.length > 0 &&
    row.receiptIngestionProofRequirements.length > 0 &&
    row.providerReceiptRefs.length === 0 &&
    trackingNotificationReceiptBoundaryClaimsStayFalse(row)
  );
}

function trackingNotificationReceiptBoundaryClaimsStayFalse(row: TrackingNotificationReceiptBoundaryRowInput): boolean {
  return [
    row.providerDeliveryClaimed,
    row.webhookReceiptIngestionClaimed,
    row.providerCredentialClaimed,
    row.adapterDispatchClaimed,
    row.childDeviceDeliveryClaimed,
    row.mobilePhysicalDeviceProofClaimed,
    row.authorityProofClaimed,
  ].every((claim) => claim === false);
}

function trackingNotificationReceiptBoundaryReadModelIsHonest(
  readModel: TrackingNotificationReceiptBoundaryReadModelInput
): boolean {
  const sourceProviderProofNonClaims = new Set<string>(readModel.sourceProviderProofNonClaims);

  return (
    readModel.receiptIngestionRequiredCount === countReceiptState(readModel.rows, 'receipt-ingestion-required') &&
    readModel.manualReceiptRequiredCount === countReceiptState(readModel.rows, 'manual-receipt-required') &&
    readModel.providerUnavailableCount === countReceiptState(readModel.rows, 'provider-unavailable') &&
    RequiredTrackingProviderNotificationProofNonClaims.every((claim) => sourceProviderProofNonClaims.has(claim)) &&
    RequiredTrackingNotificationReceiptBoundaryNonClaims.every((claim) => readModel.proofNonClaims.includes(claim))
  );
}

const countReceiptState = (
  rows: ReadonlyArray<{ readonly receiptBoundaryState: TrackingNotificationReceiptBoundaryState }>,
  receiptBoundaryState: TrackingNotificationReceiptBoundaryState
): number => rows.filter((row) => row.receiptBoundaryState === receiptBoundaryState).length;

export const decodeTrackingNotificationReceiptBoundaryReadModel = Schema.decodeUnknownSync(
  TrackingNotificationReceiptBoundaryReadModelSchema
);
