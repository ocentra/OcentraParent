import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameNotificationLocalOutboxBridgeReadModelSchema,
  AppGameNotificationLocalOutboxBridgeStatus,
  type AppGameNotificationLocalOutboxBridgeReadModel,
  type AppGameNotificationLocalOutboxBridgeRow,
} from './app-game-notification-local-outbox-bridge';
import {
  NotificationLocalOutboxSchedulerRecordSchema,
  type NotificationLocalOutboxSchedulerRecord,
} from './notification-local-outbox-scheduler-proof';
import { RequiredNotificationLocalOutboxSchedulerNonClaims } from './notification-local-outbox-scheduler-proof-values';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema } from './references';

const BridgeText = Schema.String.pipe(Schema.minLength(1));

export const AppGameNotificationSchedulerBridgeStatus = {
  ScheduledLocal: 'scheduled-local-proof-row',
  ManualRequired: 'not-scheduled-manual-required',
  Unavailable: 'not-scheduled-unavailable',
} as const;

export const AppGameNotificationSchedulerBridgeStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationSchedulerBridgeStatus))
);

// prettier-ignore
export const AppGameNotificationSchedulerBridgeIdSchema = BridgeText.pipe(Schema.brand('AppGameNotificationSchedulerBridgeId'));
// prettier-ignore
export const AppGameNotificationSchedulerBridgeReferenceSchema = BridgeText.pipe(Schema.brand('AppGameNotificationSchedulerBridgeReference'));

const AppGameNotificationSchedulerBridgeRowBaseSchema = Schema.Struct({
  schedulerBridgeRecordId: AppGameNotificationSchedulerBridgeReferenceSchema,
  sourceBridgeRecordId: AppGameNotificationSchedulerBridgeReferenceSchema,
  status: AppGameNotificationSchedulerBridgeStatusSchema,
  sourceOutboxRecordRef: Schema.Union(AppGameNotificationSchedulerBridgeReferenceSchema, Schema.Null),
  schedulerRecord: Schema.Union(NotificationLocalOutboxSchedulerRecordSchema, Schema.Null),
  blockedReasonRefs: Schema.Array(AppGameNotificationSchedulerBridgeReferenceSchema),
});

export const AppGameNotificationSchedulerBridgeRowSchema = withParser(
  AppGameNotificationSchedulerBridgeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        rowIsHonest(row) ||
        'Expected app/game notification scheduler bridge rows to schedule only linked local outbox records and keep manual/unavailable rows unscheduled'
    )
  )
);

const AppGameNotificationSchedulerBridgeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  schedulerBridgeId: AppGameNotificationSchedulerBridgeIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceOutboxBridgeId: AppGameNotificationSchedulerBridgeReferenceSchema,
  schedulerArtifactRootRef: AppGameNotificationSchedulerBridgeReferenceSchema,
  schedulerNowAt: ParentTimestampSchema,
  rows: Schema.Array(AppGameNotificationSchedulerBridgeRowSchema),
  scheduledRecordCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unscheduledManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unscheduledUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  schedulerNonClaims: Schema.Array(Schema.Literal(...RequiredNotificationLocalOutboxSchedulerNonClaims)),
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

export const AppGameNotificationSchedulerBridgeReadModelSchema = withParser(
  AppGameNotificationSchedulerBridgeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        readModelIsHonest(readModel) ||
        'Expected app/game notification scheduler bridge counts and non-claims to match scheduled manual and unavailable rows'
    )
  )
);

export type AppGameNotificationSchedulerBridgeStatus = Infer<typeof AppGameNotificationSchedulerBridgeStatusSchema>;
export type AppGameNotificationSchedulerBridgeRow = Infer<typeof AppGameNotificationSchedulerBridgeRowSchema>;
export type AppGameNotificationSchedulerBridgeReadModel = Infer<
  typeof AppGameNotificationSchedulerBridgeReadModelSchema
>;

export type AppGameNotificationSchedulerBridgeOptions = {
  readonly generatedAt: string;
  readonly schedulerBridgeId: string;
  readonly schedulerArtifactRootRef: string;
  readonly schedulerArtifactRef: string;
  readonly schedulerNowAt: string;
};

export function buildAppGameNotificationSchedulerBridgeReadModel(
  options: AppGameNotificationSchedulerBridgeOptions,
  sourceReadModel: AppGameNotificationLocalOutboxBridgeReadModel
): AppGameNotificationSchedulerBridgeReadModel {
  const parsedSource = AppGameNotificationLocalOutboxBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => outboxRowToSchedulerBridgeRow(options, row));

  return AppGameNotificationSchedulerBridgeReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    schedulerBridgeId: options.schedulerBridgeId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceOutboxBridgeId: parsedSource.bridgeId,
    schedulerArtifactRootRef: options.schedulerArtifactRootRef,
    schedulerNowAt: options.schedulerNowAt,
    rows,
    scheduledRecordCount: countRows(rows, AppGameNotificationSchedulerBridgeStatus.ScheduledLocal),
    unscheduledManualRequiredCount: countRows(rows, AppGameNotificationSchedulerBridgeStatus.ManualRequired),
    unscheduledUnavailableCount: countRows(rows, AppGameNotificationSchedulerBridgeStatus.Unavailable),
    schedulerNonClaims: RequiredNotificationLocalOutboxSchedulerNonClaims,
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

export function serializeAppGameNotificationSchedulerJsonl(
  readModel: AppGameNotificationSchedulerBridgeReadModel
): string {
  const records = readModel.rows.flatMap((row) => (row.schedulerRecord === null ? [] : [row.schedulerRecord]));
  return `${records.map((record) => JSON.stringify(record)).join('\n')}\n`;
}

export function parseAppGameNotificationSchedulerJsonl(jsonl: string): NotificationLocalOutboxSchedulerRecord[] {
  const lines = jsonl.split('\n').filter((line) => line.trim().length > 0);
  return lines.map((line) => NotificationLocalOutboxSchedulerRecordSchema.parse(JSON.parse(line)));
}

function outboxRowToSchedulerBridgeRow(
  options: AppGameNotificationSchedulerBridgeOptions,
  row: AppGameNotificationLocalOutboxBridgeRow
): AppGameNotificationSchedulerBridgeRow {
  const status = schedulerStatusForOutboxBridgeRow(row);
  const sourceOutboxRecordRef = row.outboxRecord?.entryId ?? null;

  return AppGameNotificationSchedulerBridgeRowSchema.parse({
    schedulerBridgeRecordId: `app-game-notification-scheduler-bridge-${row.bridgeRecordId}`,
    sourceBridgeRecordId: row.bridgeRecordId,
    status,
    sourceOutboxRecordRef,
    schedulerRecord:
      status === AppGameNotificationSchedulerBridgeStatus.ScheduledLocal && row.outboxRecord !== null
        ? schedulerRecordForOutboxRecord(options, row)
        : null,
    blockedReasonRefs: status === AppGameNotificationSchedulerBridgeStatus.ScheduledLocal ? [] : row.blockedReasonRefs,
  });
}

function schedulerRecordForOutboxRecord(
  options: AppGameNotificationSchedulerBridgeOptions,
  row: AppGameNotificationLocalOutboxBridgeRow
): NotificationLocalOutboxSchedulerRecord {
  const record = row.outboxRecord;
  if (record === null) {
    throw new Error(`Missing app/game notification outbox record for scheduler row: ${row.bridgeRecordId}`);
  }

  return NotificationLocalOutboxSchedulerRecordSchema.parse({
    schedulerEntryId: `app-game-notification-scheduler-${record.entryId}`,
    sourceEntryId: record.entryId,
    sourceState: record.state,
    schedulerState: 'due-local',
    reasonCode: record.envelope.reasonCode,
    providerChannel: record.envelope.providerChannel,
    severity: record.envelope.severity,
    schedulerDecisionRef: `app-game-notification-scheduler-decision-${row.intent.notificationIntentId}`,
    schedulerArtifactRef: options.schedulerArtifactRef,
    sourceOutboxFileRef: record.outboxFileRef,
    localDataPathRef: record.localDataPathRef,
    schedulerNowAt: options.schedulerNowAt,
    nextAttemptAt: options.schedulerNowAt,
    quietHoursWindow: null,
    retryWindow: null,
    deadLetterReviewRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    parentOwnedArtifactWritten: true,
    rawChildEvidenceIncluded: false,
    rawUrlOrTitleIncluded: false,
    rawMessageTextIncluded: false,
    screenshotOrReportIncluded: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerCredentialsStored: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    sensitiveProviderMetadataStored: false,
    schedulerPayloadPreview: 'alert id, app game reason, evidence ref, scheduler due marker, parent action link',
  });
}

function schedulerStatusForOutboxBridgeRow(
  row: AppGameNotificationLocalOutboxBridgeRow
): AppGameNotificationSchedulerBridgeStatus {
  return row.status === AppGameNotificationLocalOutboxBridgeStatus.Linked
    ? AppGameNotificationSchedulerBridgeStatus.ScheduledLocal
    : row.status === AppGameNotificationLocalOutboxBridgeStatus.Unavailable
      ? AppGameNotificationSchedulerBridgeStatus.Unavailable
      : AppGameNotificationSchedulerBridgeStatus.ManualRequired;
}

function rowIsHonest(row: Infer<typeof AppGameNotificationSchedulerBridgeRowBaseSchema>): boolean {
  if (row.status === AppGameNotificationSchedulerBridgeStatus.ScheduledLocal) {
    return row.schedulerRecord !== null && row.sourceOutboxRecordRef !== null && row.blockedReasonRefs.length === 0;
  }
  return row.schedulerRecord === null && row.sourceOutboxRecordRef === null && row.blockedReasonRefs.length > 0;
}

function readModelIsHonest(readModel: Infer<typeof AppGameNotificationSchedulerBridgeReadModelBaseSchema>): boolean {
  return (
    readModel.scheduledRecordCount ===
      countRows(readModel.rows, AppGameNotificationSchedulerBridgeStatus.ScheduledLocal) &&
    readModel.unscheduledManualRequiredCount ===
      countRows(readModel.rows, AppGameNotificationSchedulerBridgeStatus.ManualRequired) &&
    readModel.unscheduledUnavailableCount ===
      countRows(readModel.rows, AppGameNotificationSchedulerBridgeStatus.Unavailable) &&
    RequiredNotificationLocalOutboxSchedulerNonClaims.every((claim) => readModel.schedulerNonClaims.includes(claim))
  );
}

const countRows = (
  rows: ReadonlyArray<{ readonly status: AppGameNotificationSchedulerBridgeStatus }>,
  status: AppGameNotificationSchedulerBridgeStatus
): number => rows.filter((row) => row.status === status).length;
