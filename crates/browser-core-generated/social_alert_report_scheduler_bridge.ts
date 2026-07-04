/* generated from crates/browser-core/src/social_alert_report_scheduler_bridge.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  NotificationLocalOutboxSchedulerRecordSchema,
  RequiredNotificationLocalOutboxSchedulerNonClaims,
  type NotificationLocalOutboxSchedulerRecord,
} from './social_alert_report_scheduler_bridge_support';
import {
  countRows,
  schedulerStatusForOutboxBridgeRow,
  socialAlertReportSchedulerBridgeReadModelCountsMatch,
  socialAlertReportSchedulerBridgeRowIsHonest,
} from './social_alert_report_scheduler_bridge_helpers';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/schema-domain/family-references';
import {
  SocialAlertReportLocalOutboxBridgeReadModelSchema,
  SocialAlertReportLocalOutboxBridgeStatus,
  type SocialAlertReportLocalOutboxBridgeReadModel,
  type SocialAlertReportLocalOutboxBridgeRow,
} from './social-alert-report-local-outbox-bridge';

export const SocialAlertReportSchedulerBridgeStatus = {
  ScheduledLocal: 'scheduled-local-proof-row',
  ManualRequired: 'not-scheduled-manual-required',
  Unavailable: 'not-scheduled-unavailable',
} as const;

export const SocialAlertReportSchedulerBridgeStatusSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportSchedulerBridgeStatus))
);

export const SocialAlertReportSchedulerBridgeIdSchema = brandedNonEmptyStringSchema(
  'SocialAlertReportSchedulerBridgeId'
);
export const SocialAlertReportSchedulerBridgeReferenceSchema = brandedNonEmptyStringSchema(
  'SocialAlertReportSchedulerBridgeReference'
);

const SocialAlertReportSchedulerBridgeRowBaseSchema = Schema.Struct({
  schedulerBridgeRecordId: SocialAlertReportSchedulerBridgeReferenceSchema,
  sourceBridgeRecordId: SocialAlertReportSchedulerBridgeReferenceSchema,
  status: SocialAlertReportSchedulerBridgeStatusSchema,
  sourceOutboxRecordRef: Schema.Union(SocialAlertReportSchedulerBridgeReferenceSchema, Schema.Null),
  schedulerRecord: Schema.Union(NotificationLocalOutboxSchedulerRecordSchema, Schema.Null),
  blockedReasonRefs: Schema.Array(SocialAlertReportSchedulerBridgeReferenceSchema),
});

export const SocialAlertReportSchedulerBridgeRowSchema = withParser(
  SocialAlertReportSchedulerBridgeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialAlertReportSchedulerBridgeRowIsHonest(row) ||
        'Expected social alert/report scheduler bridge rows to schedule only linked local outbox records and keep manual/unavailable rows unscheduled'
    )
  )
);

const SocialAlertReportSchedulerBridgeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  schedulerBridgeId: SocialAlertReportSchedulerBridgeIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceOutboxBridgeId: SocialAlertReportSchedulerBridgeReferenceSchema,
  schedulerArtifactRootRef: SocialAlertReportSchedulerBridgeReferenceSchema,
  schedulerNowAt: ParentTimestampSchema,
  rows: Schema.Array(SocialAlertReportSchedulerBridgeRowSchema),
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
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportSchedulerBridgeReadModelSchema = withParser(
  SocialAlertReportSchedulerBridgeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialAlertReportSchedulerBridgeReadModelCountsMatch(
          readModel,
          RequiredNotificationLocalOutboxSchedulerNonClaims
        ) ||
        'Expected social alert/report scheduler bridge counts and non-claims to match scheduled manual and unavailable rows'
    )
  )
);

export type SocialAlertReportSchedulerBridgeStatus = Infer<typeof SocialAlertReportSchedulerBridgeStatusSchema>;
export type SocialAlertReportSchedulerBridgeRow = Infer<typeof SocialAlertReportSchedulerBridgeRowSchema>;
export type SocialAlertReportSchedulerBridgeReadModel = Infer<typeof SocialAlertReportSchedulerBridgeReadModelSchema>;

export type SocialAlertReportSchedulerBridgeOptions = {
  readonly generatedAt: string;
  readonly schedulerBridgeId: string;
  readonly schedulerArtifactRootRef: string;
  readonly schedulerArtifactRef: string;
  readonly schedulerNowAt: string;
};

export function buildSocialAlertReportSchedulerBridgeReadModel(
  options: SocialAlertReportSchedulerBridgeOptions,
  sourceReadModel: SocialAlertReportLocalOutboxBridgeReadModel
): SocialAlertReportSchedulerBridgeReadModel {
  const parsedSource = SocialAlertReportLocalOutboxBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => outboxRowToSchedulerBridgeRow(options, row));

  return SocialAlertReportSchedulerBridgeReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    schedulerBridgeId: options.schedulerBridgeId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceOutboxBridgeId: parsedSource.bridgeId,
    schedulerArtifactRootRef: options.schedulerArtifactRootRef,
    schedulerNowAt: options.schedulerNowAt,
    rows,
    scheduledRecordCount: countRows(rows, SocialAlertReportSchedulerBridgeStatus.ScheduledLocal),
    unscheduledManualRequiredCount: countRows(rows, SocialAlertReportSchedulerBridgeStatus.ManualRequired),
    unscheduledUnavailableCount: countRows(rows, SocialAlertReportSchedulerBridgeStatus.Unavailable),
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
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

export function serializeSocialAlertReportSchedulerJsonl(readModel: SocialAlertReportSchedulerBridgeReadModel): string {
  const records = readModel.rows
    .map((row) => row.schedulerRecord)
    .filter((record): record is NotificationLocalOutboxSchedulerRecord => record !== null);
  return `${records.map((record) => JSON.stringify(record)).join('\n')}\n`;
}

export function parseSocialAlertReportSchedulerJsonl(jsonl: string): NotificationLocalOutboxSchedulerRecord[] {
  const lines = jsonl.split('\n').filter((line) => line.trim().length > 0);
  return lines.map((line) => NotificationLocalOutboxSchedulerRecordSchema.parse(JSON.parse(line)));
}

function outboxRowToSchedulerBridgeRow(
  options: SocialAlertReportSchedulerBridgeOptions,
  row: SocialAlertReportLocalOutboxBridgeRow
): SocialAlertReportSchedulerBridgeRow {
  const status = schedulerStatusForOutboxBridgeRow(row);
  const sourceOutboxRecordRef = row.outboxRecord?.entryId ?? null;

  return SocialAlertReportSchedulerBridgeRowSchema.parse({
    schedulerBridgeRecordId: `social-alert-report-scheduler-bridge-${row.bridgeRecordId}`,
    sourceBridgeRecordId: row.bridgeRecordId,
    status,
    sourceOutboxRecordRef,
    schedulerRecord:
      status === SocialAlertReportSchedulerBridgeStatus.ScheduledLocal && row.outboxRecord !== null
        ? schedulerRecordForOutboxRecord(options, row)
        : null,
    blockedReasonRefs: status === SocialAlertReportSchedulerBridgeStatus.ScheduledLocal ? [] : row.blockedReasonRefs,
  });
}

function schedulerRecordForOutboxRecord(
  options: SocialAlertReportSchedulerBridgeOptions,
  row: SocialAlertReportLocalOutboxBridgeRow
): NotificationLocalOutboxSchedulerRecord {
  const record = row.outboxRecord;
  if (record === null) {
    throw new Error(`Missing social alert/report outbox record for scheduler row: ${row.bridgeRecordId}`);
  }

  return NotificationLocalOutboxSchedulerRecordSchema.parse({
    schedulerEntryId: `social-alert-report-scheduler-${record.entryId}`,
    sourceEntryId: record.entryId,
    sourceState: record.state,
    schedulerState: 'due-local',
    reasonCode: record.envelope.reasonCode,
    providerChannel: record.envelope.providerChannel,
    severity: record.envelope.severity,
    schedulerDecisionRef: `social-alert-report-scheduler-decision-${row.intent.alertReportIntentId}`,
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
      'alert id, social reason, evidence ref, explanation ref, scheduler due marker, parent action link',
  });
}
