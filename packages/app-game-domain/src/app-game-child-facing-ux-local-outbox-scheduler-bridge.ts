import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxLocalOutboxBridgeReadModelSchema,
  AppGameChildUxLocalOutboxBridgeStatus,
  type AppGameChildUxLocalOutboxBridgeReadModel,
  type AppGameChildUxLocalOutboxBridgeRow,
} from './app-game-child-facing-ux-local-outbox-bridge';
import {
  NotificationLocalOutboxSchedulerRecordSchema,
  type NotificationLocalOutboxSchedulerRecord,
} from './notification-local-outbox-scheduler-proof';
import { RequiredNotificationLocalOutboxSchedulerNonClaims } from './notification-local-outbox-scheduler-proof-values';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/family-domain/references';

const SchedulerBridgeText = Schema.String.pipe(Schema.minLength(1));

export const AppGameChildUxLocalOutboxSchedulerBridgeStatus = {
  ScheduledLocal: 'scheduled-local-proof-row',
  ManualRequired: 'not-scheduled-manual-required',
  Unavailable: 'not-scheduled-unavailable',
} as const;

export const AppGameChildUxLocalOutboxSchedulerBridgeStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxLocalOutboxSchedulerBridgeStatus))
);
export const AppGameChildUxLocalOutboxSchedulerBridgeIdSchema = SchedulerBridgeText.pipe(
  Schema.brand('AppGameChildUxLocalOutboxSchedulerBridgeId')
);
export const AppGameChildUxLocalOutboxSchedulerBridgeReferenceSchema = SchedulerBridgeText.pipe(
  Schema.brand('AppGameChildUxLocalOutboxSchedulerBridgeReference')
);

const AppGameChildUxLocalOutboxSchedulerBridgeRowBaseSchema = Schema.Struct({
  schedulerBridgeRecordId: AppGameChildUxLocalOutboxSchedulerBridgeReferenceSchema,
  sourceOutboxBridgeRecordId: AppGameChildUxLocalOutboxSchedulerBridgeReferenceSchema,
  status: AppGameChildUxLocalOutboxSchedulerBridgeStatusSchema,
  sourceOutboxRecordRef: Schema.Union(AppGameChildUxLocalOutboxSchedulerBridgeReferenceSchema, Schema.Null),
  schedulerRecord: Schema.Union(NotificationLocalOutboxSchedulerRecordSchema, Schema.Null),
  blockedReasonRefs: Schema.Array(AppGameChildUxLocalOutboxSchedulerBridgeReferenceSchema),
});

export const AppGameChildUxLocalOutboxSchedulerBridgeRowSchema = withParser(
  AppGameChildUxLocalOutboxSchedulerBridgeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameChildUxLocalOutboxSchedulerBridgeRowIsHonest(row) ||
        'Expected child UX local outbox scheduler rows to schedule only linked local outbox records and keep manual or unavailable rows unscheduled'
    )
  )
);

const AppGameChildUxLocalOutboxSchedulerBridgeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  schedulerBridgeId: AppGameChildUxLocalOutboxSchedulerBridgeIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceOutboxBridgeId: AppGameChildUxLocalOutboxSchedulerBridgeReferenceSchema,
  schedulerArtifactRootRef: AppGameChildUxLocalOutboxSchedulerBridgeReferenceSchema,
  schedulerNowAt: ParentTimestampSchema,
  rows: Schema.Array(AppGameChildUxLocalOutboxSchedulerBridgeRowSchema),
  scheduledRecordCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unscheduledManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unscheduledUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  schedulerNonClaims: Schema.Array(Schema.Literal(...RequiredNotificationLocalOutboxSchedulerNonClaims)),
  childDeliveryRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema = withParser(
  AppGameChildUxLocalOutboxSchedulerBridgeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameChildUxLocalOutboxSchedulerBridgeReadModelIsHonest(readModel) ||
        'Expected child UX local outbox scheduler bridge counts and no-claim flags to match scheduled manual-required and unavailable rows'
    )
  )
);

export type AppGameChildUxLocalOutboxSchedulerBridgeStatus = Infer<
  typeof AppGameChildUxLocalOutboxSchedulerBridgeStatusSchema
>;
export type AppGameChildUxLocalOutboxSchedulerBridgeRow = Infer<
  typeof AppGameChildUxLocalOutboxSchedulerBridgeRowSchema
>;
export type AppGameChildUxLocalOutboxSchedulerBridgeReadModel = Infer<
  typeof AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema
>;

export type AppGameChildUxLocalOutboxSchedulerBridgeOptions = {
  readonly generatedAt: string;
  readonly schedulerBridgeId: string;
  readonly schedulerArtifactRootRef: string;
  readonly schedulerArtifactRef: string;
  readonly schedulerNowAt: string;
};

export function buildAppGameChildUxLocalOutboxSchedulerBridgeReadModel(
  options: AppGameChildUxLocalOutboxSchedulerBridgeOptions,
  sourceReadModel: AppGameChildUxLocalOutboxBridgeReadModel
): AppGameChildUxLocalOutboxSchedulerBridgeReadModel {
  const parsedSource = AppGameChildUxLocalOutboxBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => childUxLocalOutboxRowToSchedulerBridgeRow(options, row));

  return AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    schedulerBridgeId: options.schedulerBridgeId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceOutboxBridgeId: parsedSource.bridgeId,
    schedulerArtifactRootRef: options.schedulerArtifactRootRef,
    schedulerNowAt: options.schedulerNowAt,
    rows,
    scheduledRecordCount: countRows(rows, AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal),
    unscheduledManualRequiredCount: countRows(rows, AppGameChildUxLocalOutboxSchedulerBridgeStatus.ManualRequired),
    unscheduledUnavailableCount: countRows(rows, AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable),
    schedulerNonClaims: RequiredNotificationLocalOutboxSchedulerNonClaims,
    childDeliveryRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function serializeAppGameChildUxLocalOutboxSchedulerJsonl(
  readModel: AppGameChildUxLocalOutboxSchedulerBridgeReadModel
): string {
  return `${readModel.rows
    .flatMap((row) => (row.schedulerRecord === null ? [] : [row.schedulerRecord]))
    .map((record) => JSON.stringify(record))
    .join('\n')}\n`;
}

export function parseAppGameChildUxLocalOutboxSchedulerJsonl(jsonl: string): NotificationLocalOutboxSchedulerRecord[] {
  return jsonl
    .split('\n')
    .filter((line) => line.trim().length > 0)
    .map((line) => NotificationLocalOutboxSchedulerRecordSchema.parse(JSON.parse(line)));
}

function childUxLocalOutboxRowToSchedulerBridgeRow(
  options: AppGameChildUxLocalOutboxSchedulerBridgeOptions,
  row: AppGameChildUxLocalOutboxBridgeRow
): AppGameChildUxLocalOutboxSchedulerBridgeRow {
  const status = schedulerStatusForChildUxLocalOutboxRow(row);
  const sourceOutboxRecordRef = row.outboxRecord?.entryId ?? null;

  return AppGameChildUxLocalOutboxSchedulerBridgeRowSchema.parse({
    schedulerBridgeRecordId: `app-game-child-ux-local-outbox-scheduler-bridge-${row.bridgeRecordId}`,
    sourceOutboxBridgeRecordId: row.bridgeRecordId,
    status,
    sourceOutboxRecordRef,
    schedulerRecord:
      status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal && row.outboxRecord !== null
        ? schedulerRecordForChildUxLocalOutboxRecord(options, row)
        : null,
    blockedReasonRefs:
      status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal ? [] : row.blockedReasonRefs,
  });
}

function schedulerRecordForChildUxLocalOutboxRecord(
  options: AppGameChildUxLocalOutboxSchedulerBridgeOptions,
  row: AppGameChildUxLocalOutboxBridgeRow
): NotificationLocalOutboxSchedulerRecord {
  const record = row.outboxRecord;
  if (record === null) {
    throw new Error(`Missing child UX local outbox record for scheduler row: ${row.bridgeRecordId}`);
  }

  return NotificationLocalOutboxSchedulerRecordSchema.parse({
    schedulerEntryId: `app-game-child-ux-local-outbox-scheduler-${record.entryId}`,
    sourceEntryId: record.entryId,
    sourceState: record.state,
    schedulerState: 'due-local',
    reasonCode: record.envelope.reasonCode,
    providerChannel: record.envelope.providerChannel,
    severity: record.envelope.severity,
    schedulerDecisionRef: `app-game-child-ux-local-outbox-scheduler-decision-${row.sourceArtifactRecord.recordId}`,
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
    schedulerPayloadPreview:
      'child UX alert id, family device scope, reason, evidence ref, policy ref, scheduler due marker, parent action link',
  });
}

function schedulerStatusForChildUxLocalOutboxRow(
  row: AppGameChildUxLocalOutboxBridgeRow
): AppGameChildUxLocalOutboxSchedulerBridgeStatus {
  if (row.status === AppGameChildUxLocalOutboxBridgeStatus.Linked) {
    return AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal;
  }
  return row.status === AppGameChildUxLocalOutboxBridgeStatus.Unavailable
    ? AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable
    : AppGameChildUxLocalOutboxSchedulerBridgeStatus.ManualRequired;
}

function appGameChildUxLocalOutboxSchedulerBridgeRowIsHonest(
  row: Infer<typeof AppGameChildUxLocalOutboxSchedulerBridgeRowBaseSchema>
): boolean {
  if (row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal) {
    return row.schedulerRecord !== null && row.sourceOutboxRecordRef !== null && row.blockedReasonRefs.length === 0;
  }
  return row.schedulerRecord === null && row.sourceOutboxRecordRef === null && row.blockedReasonRefs.length > 0;
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function appGameChildUxLocalOutboxSchedulerBridgeReadModelIsHonest(
  readModel: Infer<typeof AppGameChildUxLocalOutboxSchedulerBridgeReadModelBaseSchema>
): boolean {
  return (
    readModel.scheduledRecordCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal) &&
    readModel.unscheduledManualRequiredCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxSchedulerBridgeStatus.ManualRequired) &&
    readModel.unscheduledUnavailableCount ===
      countRows(readModel.rows, AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable) &&
    RequiredNotificationLocalOutboxSchedulerNonClaims.every((claim) => readModel.schedulerNonClaims.includes(claim)) &&
    !readModel.childDeliveryRuntimeClaimed &&
    !readModel.providerDeliveryRuntimeClaimed &&
    !readModel.providerReceiptIngestionClaimed &&
    !readModel.providerCredentialsClaimed &&
    !readModel.cloudRoutingClaimed &&
    !readModel.parentNotificationUiClaimed &&
    !readModel.retryExecutionRuntimeClaimed &&
    !readModel.quietHoursTimerRuntimeClaimed &&
    !readModel.productionDurableOutboxStorageClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameChildUxLocalOutboxSchedulerBridgeStatus }>,
  status: AppGameChildUxLocalOutboxSchedulerBridgeStatus
): number {
  return rows.filter((row) => row.status === status).length;
}
