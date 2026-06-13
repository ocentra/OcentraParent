import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  NotificationLocalOutboxAdapterProofReadModel,
  NotificationLocalOutboxAdapterProofSchema,
  type NotificationLocalOutboxAdapterProof,
  type NotificationLocalOutboxRecord,
} from '@ocentra-parent/notification-domain/notification-local-outbox-adapter-proof';
import {
  NotificationLocalOutboxSchedulerProofReadModel,
  NotificationLocalOutboxSchedulerProofSchema,
  type NotificationLocalOutboxSchedulerProof,
  type NotificationLocalOutboxSchedulerRecord,
} from '@ocentra-parent/notification-domain/notification-local-outbox-scheduler-proof';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  TrackingNotificationReceiptBoundaryReadModelSchema,
  type TrackingNotificationReceiptBoundaryReadModel,
  type TrackingNotificationReceiptBoundaryRow,
} from './tracking-notification-receipt-boundary-proof';

export const RequiredTrackingNotificationLocalOutboxReadinessNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion-runtime',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-retry-worker-runtime',
  'no-quiet-hours-timer-runtime',
  'no-production-durable-outbox-storage',
  'no-child-device-delivery',
  'no-mobile-physical-device-proof',
  'no-authority-proof',
] as const;

export const TrackingNotificationLocalOutboxReadinessNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingNotificationLocalOutboxReadinessNonClaims)
);

export const TrackingNotificationLocalOutboxReadinessProofIdSchema = brandedNonEmptyStringSchema('TrackingNotificationLocalOutboxReadinessProofId');
export const TrackingNotificationLocalOutboxReadinessReferenceSchema =
  brandedNonEmptyStringSchema('TrackingNotificationLocalOutboxReadinessReference');
export const TrackingNotificationLocalOutboxReadinessStateSchema = withParser(
  Schema.Literal('local-outbox-receipt-required', 'local-outbox-manual-required', 'local-outbox-provider-unavailable')
);

const TrackingNotificationLocalOutboxReadinessRowBaseSchema = Schema.Struct({
  readinessRowId: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  sourceReceiptBoundaryRowId: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  sourceAlertId: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  sourcePolicyDecisionId: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  readinessState: TrackingNotificationLocalOutboxReadinessStateSchema,
  evidenceRefs: Schema.Array(TrackingNotificationLocalOutboxReadinessReferenceSchema),
  notificationStatusRefs: Schema.Array(TrackingNotificationLocalOutboxReadinessReferenceSchema),
  reasonCodeRefs: Schema.Array(TrackingNotificationLocalOutboxReadinessReferenceSchema),
  providerAttemptRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  receiptRequirementRefs: Schema.Array(TrackingNotificationLocalOutboxReadinessReferenceSchema),
  manualProofRequirements: Schema.Array(TrackingNotificationLocalOutboxReadinessReferenceSchema),
  localOutboxReadModelRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  localOutboxEntryRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  localOutboxStateRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  localOutboxFileRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  localDataPathRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  schedulerReadModelRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  schedulerEntryRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  schedulerStateRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  schedulerArtifactRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  localOutboxParentOwnedArtifactWritten: Schema.Literal(true),
  schedulerParentOwnedArtifactWritten: Schema.Literal(true),
  providerDeliveryClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
});

export const TrackingNotificationLocalOutboxReadinessRowSchema = withParser(
  TrackingNotificationLocalOutboxReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingNotificationLocalOutboxReadinessRowIsHonest(row) ||
        'Expected tracking notification local outbox readiness rows to preserve receipt refs, local outbox refs, scheduler refs, and no-claim flags'
    )
  )
);

const TrackingNotificationLocalOutboxReadinessReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingNotificationLocalOutboxReadinessProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceReceiptBoundaryProofRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  sourceLocalOutboxAdapterProofRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  sourceLocalOutboxSchedulerProofRef: TrackingNotificationLocalOutboxReadinessReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingNotificationLocalOutboxReadinessReferenceSchema),
  rows: Schema.Array(TrackingNotificationLocalOutboxReadinessRowSchema),
  receiptRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  proofNonClaims: Schema.Array(TrackingNotificationLocalOutboxReadinessNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
});

export const TrackingNotificationLocalOutboxReadinessReadModelSchema = withParser(
  TrackingNotificationLocalOutboxReadinessReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingNotificationLocalOutboxReadinessReadModelIsHonest(readModel) ||
        'Expected tracking notification local outbox readiness counts and no-claims to match receipt rows'
    )
  )
);

export type TrackingNotificationLocalOutboxReadinessState = Infer<
  typeof TrackingNotificationLocalOutboxReadinessStateSchema
>;
export type TrackingNotificationLocalOutboxReadinessRow = Infer<
  typeof TrackingNotificationLocalOutboxReadinessRowSchema
>;
export type TrackingNotificationLocalOutboxReadinessReadModel = Infer<
  typeof TrackingNotificationLocalOutboxReadinessReadModelSchema
>;

export type TrackingNotificationLocalOutboxReadinessProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceContractRefs: readonly string[];
};

type ReadinessRowInput = Infer<typeof TrackingNotificationLocalOutboxReadinessRowBaseSchema>;
type ReadinessReadModelInput = Infer<typeof TrackingNotificationLocalOutboxReadinessReadModelBaseSchema>;

export function buildTrackingNotificationLocalOutboxReadinessReadModel(
  options: TrackingNotificationLocalOutboxReadinessProofOptions,
  receiptProof: TrackingNotificationReceiptBoundaryReadModel,
  localOutboxProof: NotificationLocalOutboxAdapterProof = NotificationLocalOutboxAdapterProofReadModel,
  schedulerProof: NotificationLocalOutboxSchedulerProof = NotificationLocalOutboxSchedulerProofReadModel
): TrackingNotificationLocalOutboxReadinessReadModel {
  const parsedReceiptProof = TrackingNotificationReceiptBoundaryReadModelSchema.parse(receiptProof);
  const parsedOutboxProof = NotificationLocalOutboxAdapterProofSchema.parse(localOutboxProof);
  const parsedSchedulerProof = NotificationLocalOutboxSchedulerProofSchema.parse(schedulerProof);
  const rows = parsedReceiptProof.rows.map((receiptRow) =>
    trackingNotificationLocalOutboxReadinessRowForReceipt(receiptRow, parsedOutboxProof, parsedSchedulerProof)
  );

  return TrackingNotificationLocalOutboxReadinessReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: parsedReceiptProof.family,
    sourceReceiptBoundaryProofRef: parsedReceiptProof.proofId,
    sourceLocalOutboxAdapterProofRef: parsedOutboxProof.readModelId,
    sourceLocalOutboxSchedulerProofRef: parsedSchedulerProof.readModelId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    receiptRequiredCount: countReadinessState(rows, 'local-outbox-receipt-required'),
    manualRequiredCount: countReadinessState(rows, 'local-outbox-manual-required'),
    providerUnavailableCount: countReadinessState(rows, 'local-outbox-provider-unavailable'),
    proofNonClaims: RequiredTrackingNotificationLocalOutboxReadinessNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
  });
}

function trackingNotificationLocalOutboxReadinessRowForReceipt(
  receiptRow: TrackingNotificationReceiptBoundaryRow,
  localOutboxProof: NotificationLocalOutboxAdapterProof,
  schedulerProof: NotificationLocalOutboxSchedulerProof
): TrackingNotificationLocalOutboxReadinessRow {
  const readinessState = readinessStateForReceiptRow(receiptRow);
  const localOutboxRecord = localOutboxRecordForReadinessState(readinessState, localOutboxProof.records);
  const schedulerRecord = schedulerRecordForOutboxEntry(localOutboxRecord.entryId, schedulerProof.records);

  return TrackingNotificationLocalOutboxReadinessRowSchema.parse({
    readinessRowId: `tracking-notification-local-outbox-${receiptRow.sourceAlertId}`,
    sourceReceiptBoundaryRowId: receiptRow.rowId,
    sourceAlertId: receiptRow.sourceAlertId,
    sourcePolicyDecisionId: receiptRow.sourcePolicyDecisionId,
    readinessState,
    evidenceRefs: receiptRow.evidenceRefs,
    notificationStatusRefs: receiptRow.notificationStatusRefs,
    reasonCodeRefs: receiptRow.reasonCodeRefs,
    providerAttemptRef: receiptRow.providerAttemptRef,
    receiptRequirementRefs: receiptRow.receiptIngestionProofRequirements,
    manualProofRequirements: uniqueRefs([
      ...receiptRow.manualProofRequirements,
      ...localOutboxRecord.manualProofRequirements,
    ]),
    localOutboxReadModelRef: NotificationLocalOutboxAdapterProofReadModel.readModelId,
    localOutboxEntryRef: localOutboxRecord.entryId,
    localOutboxStateRef: localOutboxRecord.state,
    localOutboxFileRef: localOutboxRecord.outboxFileRef,
    localDataPathRef: localOutboxRecord.localDataPathRef,
    schedulerReadModelRef: NotificationLocalOutboxSchedulerProofReadModel.readModelId,
    schedulerEntryRef: schedulerRecord.schedulerEntryId,
    schedulerStateRef: schedulerRecord.schedulerState,
    schedulerArtifactRef: schedulerRecord.schedulerArtifactRef,
    localOutboxParentOwnedArtifactWritten: true,
    schedulerParentOwnedArtifactWritten: schedulerRecord.parentOwnedArtifactWritten,
    providerDeliveryClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
  });
}

function readinessStateForReceiptRow(
  row: TrackingNotificationReceiptBoundaryRow
): TrackingNotificationLocalOutboxReadinessState {
  if (row.receiptBoundaryState === 'receipt-ingestion-required') {
    return 'local-outbox-receipt-required';
  }
  if (row.receiptBoundaryState === 'provider-unavailable') {
    return 'local-outbox-provider-unavailable';
  }
  return 'local-outbox-manual-required';
}

function localOutboxRecordForReadinessState(
  state: TrackingNotificationLocalOutboxReadinessState,
  records: readonly NotificationLocalOutboxRecord[]
): NotificationLocalOutboxRecord {
  if (state === 'local-outbox-receipt-required') {
    return requiredRecord(records, 'receipt-required', 'local outbox receipt-required');
  }
  if (state === 'local-outbox-provider-unavailable') {
    return requiredRecord(records, 'dead-lettered', 'local outbox provider-unavailable dead-letter');
  }
  return requiredRecord(records, 'manual-required', 'local outbox manual-required');
}

function requiredRecord(
  records: readonly NotificationLocalOutboxRecord[],
  state: NotificationLocalOutboxRecord['state'],
  label: string
): NotificationLocalOutboxRecord {
  const record = records.find((candidate) => candidate.state === state);
  if (record === undefined) {
    throw new Error(`Missing ${label} record`);
  }
  return record;
}

function schedulerRecordForOutboxEntry(
  entryId: string,
  records: readonly NotificationLocalOutboxSchedulerRecord[]
): NotificationLocalOutboxSchedulerRecord {
  const record = records.find((candidate) => candidate.sourceEntryId === entryId);
  if (record === undefined) {
    throw new Error(`Missing local outbox scheduler record for ${entryId}`);
  }
  return record;
}

function trackingNotificationLocalOutboxReadinessRowIsHonest(row: ReadinessRowInput): boolean {
  return (
    row.evidenceRefs.length > 0 &&
    row.reasonCodeRefs.length > 0 &&
    row.receiptRequirementRefs.length > 0 &&
    row.localOutboxFileRef.length > 0 &&
    row.localDataPathRef.length > 0 &&
    row.schedulerArtifactRef.length > 0 &&
    row.localOutboxParentOwnedArtifactWritten &&
    row.schedulerParentOwnedArtifactWritten &&
    trackingNotificationLocalOutboxReadinessClaimsStayFalse(row)
  );
}

function trackingNotificationLocalOutboxReadinessClaimsStayFalse(row: ReadinessRowInput): boolean {
  return [
    row.providerDeliveryClaimed,
    row.providerReceiptIngestionClaimed,
    row.providerCredentialsClaimed,
    row.cloudRoutingClaimed,
    row.parentNotificationUiClaimed,
    row.retryExecutionRuntimeClaimed,
    row.quietHoursTimerRuntimeClaimed,
    row.productionDurableOutboxStorageClaimed,
    row.childDeviceDeliveryClaimed,
    row.mobilePhysicalDeviceProofClaimed,
    row.authorityProofClaimed,
  ].every((claim) => claim === false);
}

function trackingNotificationLocalOutboxReadinessReadModelIsHonest(readModel: ReadinessReadModelInput): boolean {
  return (
    readModel.receiptRequiredCount === countReadinessState(readModel.rows, 'local-outbox-receipt-required') &&
    readModel.manualRequiredCount === countReadinessState(readModel.rows, 'local-outbox-manual-required') &&
    readModel.providerUnavailableCount === countReadinessState(readModel.rows, 'local-outbox-provider-unavailable') &&
    RequiredTrackingNotificationLocalOutboxReadinessNonClaims.every((claim) => readModel.proofNonClaims.includes(claim))
  );
}

const countReadinessState = (
  rows: ReadonlyArray<{ readonly readinessState: TrackingNotificationLocalOutboxReadinessState }>,
  readinessState: TrackingNotificationLocalOutboxReadinessState
): number => rows.filter((row) => row.readinessState === readinessState).length;

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return Array.from(new Set(refs));
}

export const decodeTrackingNotificationLocalOutboxReadinessReadModel = Schema.decodeUnknownSync(
  TrackingNotificationLocalOutboxReadinessReadModelSchema
);

