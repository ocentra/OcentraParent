import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  NotificationAuditHistoryEntrySchema,
  NotificationAuditHistoryRequiredPayloadFields,
  type NotificationAuditHistoryEntry,
  type NotificationAuditHistoryProviderStatus,
  type NotificationAuditHistoryQuietHoursState,
  type NotificationAuditHistoryRetryLifecycleState,
  type NotificationAuditHistoryEscalationState,
} from './notification-audit-history';

const HandoffText = Schema.String.pipe(Schema.minLength(1));

export const NotificationAuditHistoryHandoffSourceStatus = {
  QueuedLocalOutbox: 'queued-local-outbox',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const NotificationAuditHistoryHandoffSourceStatusSchema = withParser(
  Schema.Literal(...Object.values(NotificationAuditHistoryHandoffSourceStatus))
);

export const NotificationAuditHistoryHandoffIdSchema = HandoffText.pipe(
  Schema.brand('NotificationAuditHistoryHandoffId')
);
export const NotificationAuditHistoryHandoffReferenceSchema = HandoffText.pipe(
  Schema.brand('NotificationAuditHistoryHandoffReference')
);
export const NotificationAuditHistoryHandoffTimestampSchema = HandoffText.pipe(
  Schema.brand('NotificationAuditHistoryHandoffTimestamp')
);

const NotificationAuditHistoryHandoffSourceRowBaseSchema = Schema.Struct({
  handoffEntryId: NotificationAuditHistoryHandoffReferenceSchema,
  sourceStatus: NotificationAuditHistoryHandoffSourceStatusSchema,
  sourceNotificationIntentRef: NotificationAuditHistoryHandoffReferenceSchema,
  sourceOutboxRecordRef: Schema.Union(NotificationAuditHistoryHandoffReferenceSchema, Schema.Null),
  providerChannelRef: NotificationAuditHistoryHandoffReferenceSchema,
  reasonCodeRef: NotificationAuditHistoryHandoffReferenceSchema,
  auditRefs: Schema.Array(NotificationAuditHistoryHandoffReferenceSchema),
  evidenceRefs: Schema.Array(NotificationAuditHistoryHandoffReferenceSchema),
  policyRefs: Schema.Array(NotificationAuditHistoryHandoffReferenceSchema),
  blockedReasonRefs: Schema.Array(NotificationAuditHistoryHandoffReferenceSchema),
});

export const NotificationAuditHistoryHandoffSourceRowSchema = withParser(
  NotificationAuditHistoryHandoffSourceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        notificationAuditHistoryHandoffSourceRowIsHonest(row) ||
        'Expected notification audit-history handoff rows to queue only linked local outbox refs and keep manual/unavailable rows blocked'
    )
  )
);

const NotificationAuditHistoryHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  handoffReadModelId: NotificationAuditHistoryHandoffIdSchema,
  generatedAt: NotificationAuditHistoryHandoffTimestampSchema,
  sourceReadModelRef: NotificationAuditHistoryHandoffReferenceSchema,
  sourceContractRefs: Schema.Array(NotificationAuditHistoryHandoffReferenceSchema),
  sourceRows: Schema.Array(NotificationAuditHistoryHandoffSourceRowSchema),
  auditHistoryEntries: Schema.Array(NotificationAuditHistoryEntrySchema),
  queuedAuditEntryCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredAuditEntryCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableAuditEntryCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const NotificationAuditHistoryHandoffReadModelSchema = withParser(
  NotificationAuditHistoryHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        notificationAuditHistoryHandoffReadModelIsHonest(readModel) ||
        'Expected notification audit-history handoff counts to match source rows without provider runtime UI child delivery adapter or durable outbox claims'
    )
  )
);

export type NotificationAuditHistoryHandoffSourceStatus = Infer<
  typeof NotificationAuditHistoryHandoffSourceStatusSchema
>;
export type NotificationAuditHistoryHandoffSourceRow = Infer<typeof NotificationAuditHistoryHandoffSourceRowSchema>;
export type NotificationAuditHistoryHandoffReadModel = Infer<typeof NotificationAuditHistoryHandoffReadModelSchema>;

export type NotificationAuditHistoryHandoffOptions = {
  readonly handoffReadModelId: string;
  readonly generatedAt: string;
  readonly sourceReadModelRef: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildNotificationAuditHistoryHandoffReadModel(
  options: NotificationAuditHistoryHandoffOptions,
  sourceRows: readonly NotificationAuditHistoryHandoffSourceRow[]
): NotificationAuditHistoryHandoffReadModel {
  const rows = sourceRows.map((row) => NotificationAuditHistoryHandoffSourceRowSchema.parse(row));
  const auditHistoryEntries = rows.map((row) => notificationAuditHistoryEntryForHandoffRow(options, row));

  return NotificationAuditHistoryHandoffReadModelSchema.parse({
    schemaVersion: 1,
    handoffReadModelId: options.handoffReadModelId,
    generatedAt: options.generatedAt,
    sourceReadModelRef: options.sourceReadModelRef,
    sourceContractRefs: options.sourceContractRefs,
    sourceRows: rows,
    auditHistoryEntries,
    queuedAuditEntryCount: countRows(rows, NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox),
    manualRequiredAuditEntryCount: countRows(rows, NotificationAuditHistoryHandoffSourceStatus.ManualRequired),
    unavailableAuditEntryCount: countRows(rows, NotificationAuditHistoryHandoffSourceStatus.Unavailable),
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function notificationAuditHistoryEntryForHandoffRow(
  options: NotificationAuditHistoryHandoffOptions,
  row: NotificationAuditHistoryHandoffSourceRow
): NotificationAuditHistoryEntry {
  const states = statesForHandoffSource(row.sourceStatus);
  const manualRefs =
    row.sourceStatus === NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox ? [] : row.blockedReasonRefs;
  const quietHoursRefs = states.quietHoursState === 'allow' ? [] : [`notification-quiet-hours-${row.handoffEntryId}`];
  const escalationRefs = states.escalationState === 'none' ? [] : [`notification-escalation-${row.handoffEntryId}`];

  return NotificationAuditHistoryEntrySchema.parse({
    schemaVersion: 1,
    auditEntryId: `notification-audit-history-${row.handoffEntryId}`,
    providerStatus: states.providerStatus,
    retryLifecycleState: states.retryLifecycleState,
    quietHoursState: states.quietHoursState,
    escalationState: states.escalationState,
    payloadRedactionState: 'minimal-operational-fields-only',
    childDataCustodyState: 'no-ocentra-hosted-child-data',
    notificationIntentRef: row.sourceNotificationIntentRef,
    providerStatusRef: `provider-status-${row.handoffEntryId}`,
    providerAttemptRef: `provider-attempt-not-executed-${row.handoffEntryId}`,
    deliveryResultRef: `delivery-result-not-claimed-${row.handoffEntryId}`,
    retryLifecycleRef: `retry-lifecycle-${row.handoffEntryId}`,
    redactionPolicyRef: 'notification-redaction-minimal-operational-fields',
    custodyPolicyRef: 'notification-no-ocentra-hosted-child-data',
    auditRefs: row.auditRefs,
    evidenceRefs: row.evidenceRefs,
    receiptRefs: [],
    retryRefs: [],
    manualRequiredRefs: manualRefs,
    quietHoursRefs,
    escalationRefs,
    manualProofRequirements: manualRefs,
    redactionSafePayloadFields: [...NotificationAuditHistoryRequiredPayloadFields],
    providerAdapterImplemented: false,
    sendAttemptExecuted: false,
    retryExecutionObserved: false,
    webhookReceiptIngested: false,
    providerCredentialPresent: false,
    notificationHistoryUiClaimed: false,
    rawChildDataIncluded: false,
    rawEvidencePayloadIncluded: false,
    sensitiveProviderPayloadIncluded: false,
    ocentraHostedChildDataStored: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: options.generatedAt,
  });
}

function statesForHandoffSource(sourceStatus: NotificationAuditHistoryHandoffSourceStatus): {
  readonly providerStatus: NotificationAuditHistoryProviderStatus;
  readonly retryLifecycleState: NotificationAuditHistoryRetryLifecycleState;
  readonly quietHoursState: NotificationAuditHistoryQuietHoursState;
  readonly escalationState: NotificationAuditHistoryEscalationState;
} {
  if (sourceStatus === NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox) {
    return {
      providerStatus: 'queued',
      retryLifecycleState: 'not-scheduled',
      quietHoursState: 'allow',
      escalationState: 'none',
    };
  }
  if (sourceStatus === NotificationAuditHistoryHandoffSourceStatus.Unavailable) {
    return {
      providerStatus: 'unavailable',
      retryLifecycleState: 'provider-unavailable',
      quietHoursState: 'unavailable',
      escalationState: 'unavailable',
    };
  }
  return {
    providerStatus: 'manual-required',
    retryLifecycleState: 'manual-review-required',
    quietHoursState: 'manual-required',
    escalationState: 'manual-required',
  };
}

function notificationAuditHistoryHandoffSourceRowIsHonest(
  row: Infer<typeof NotificationAuditHistoryHandoffSourceRowBaseSchema>
): boolean {
  if (row.auditRefs.length === 0 || row.evidenceRefs.length === 0 || row.policyRefs.length === 0) {
    return false;
  }
  if (row.sourceStatus === NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox) {
    return row.sourceOutboxRecordRef !== null && row.blockedReasonRefs.length === 0;
  }
  return row.sourceOutboxRecordRef === null && row.blockedReasonRefs.length > 0;
}

function notificationAuditHistoryHandoffReadModelIsHonest(
  readModel: Infer<typeof NotificationAuditHistoryHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.auditHistoryEntries.length === readModel.sourceRows.length &&
    readModel.queuedAuditEntryCount ===
      countRows(readModel.sourceRows, NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox) &&
    readModel.manualRequiredAuditEntryCount ===
      countRows(readModel.sourceRows, NotificationAuditHistoryHandoffSourceStatus.ManualRequired) &&
    readModel.unavailableAuditEntryCount ===
      countRows(readModel.sourceRows, NotificationAuditHistoryHandoffSourceStatus.Unavailable)
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly sourceStatus: NotificationAuditHistoryHandoffSourceStatus }>,
  sourceStatus: NotificationAuditHistoryHandoffSourceStatus
): number {
  return rows.filter((row) => row.sourceStatus === sourceStatus).length;
}
