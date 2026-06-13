import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from '@ocentra-parent/notification-domain/notification-local-outbox-adapter-proof';
import {
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentSchema,
  SocialAlertReportIntentStatus,
  type SocialAlertReportIntent,
} from './social-alert-report-intent';
import { FamilyReferenceSchema, type FamilyReference, type ParentActionReference } from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { V3NotificationRuleReasonCodeSchema } from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';

export const SocialAlertReportLocalOutboxBridgeStatus = {
  Linked: 'linked-local-outbox-record',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialAlertReportLocalOutboxBridgeStatusSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportLocalOutboxBridgeStatus))
);
export const SocialAlertReportLocalOutboxBridgeIdSchema = brandedNonEmptyStringSchema('SocialAlertReportLocalOutboxBridgeId');
export const SocialAlertReportLocalOutboxBridgeReferenceSchema = brandedNonEmptyStringSchema('SocialAlertReportLocalOutboxBridgeReference');

const SocialAlertReportLocalOutboxBridgeRowBaseSchema = Schema.Struct({
  bridgeRecordId: SocialAlertReportLocalOutboxBridgeReferenceSchema,
  status: SocialAlertReportLocalOutboxBridgeStatusSchema,
  intent: SocialAlertReportIntentSchema,
  outboxRecord: Schema.Union(NotificationLocalOutboxRecordSchema, Schema.Null),
  blockedReasonRefs: Schema.Array(SocialAlertReportLocalOutboxBridgeReferenceSchema),
});

export const SocialAlertReportLocalOutboxBridgeRowSchema = withParser(
  SocialAlertReportLocalOutboxBridgeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialAlertReportBridgeRowIsHonest(row) ||
        'Expected social alert/report outbox bridge rows to link only local-outbox-eligible intents and keep manual/unavailable intents out of queued records'
    )
  )
);

const SocialAlertReportLocalOutboxBridgeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  bridgeId: SocialAlertReportLocalOutboxBridgeIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  outboxRootRef: SocialAlertReportLocalOutboxBridgeReferenceSchema,
  rows: Schema.Array(SocialAlertReportLocalOutboxBridgeRowSchema),
  linkedRecordCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  schedulerRuntimeClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportLocalOutboxBridgeReadModelSchema = withParser(
  SocialAlertReportLocalOutboxBridgeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialAlertReportBridgeReadModelCountsMatch(readModel) ||
        'Expected social alert/report outbox bridge counts to match linked manual-required and unavailable rows'
    )
  )
);

export type SocialAlertReportLocalOutboxBridgeStatus = Infer<typeof SocialAlertReportLocalOutboxBridgeStatusSchema>;
export type SocialAlertReportLocalOutboxBridgeRow = Infer<typeof SocialAlertReportLocalOutboxBridgeRowSchema>;
export type SocialAlertReportLocalOutboxBridgeReadModel = Infer<
  typeof SocialAlertReportLocalOutboxBridgeReadModelSchema
>;
export type SocialAlertReportLocalOutboxBridgeOptions = {
  readonly family: FamilyReference;
  readonly parentAction: ParentActionReference;
  readonly generatedAt: string;
  readonly bridgeId: string;
  readonly outboxRootRef: string;
  readonly outboxFileRef: string;
  readonly localDataPathRef: string;
};

export function buildSocialAlertReportLocalOutboxBridgeReadModel(
  options: SocialAlertReportLocalOutboxBridgeOptions,
  intents: ReadonlyArray<SocialAlertReportIntent>
): SocialAlertReportLocalOutboxBridgeReadModel {
  const rows = intents.map((intent) => socialAlertReportIntentToLocalOutboxBridgeRow(options, intent));
  return SocialAlertReportLocalOutboxBridgeReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    bridgeId: options.bridgeId,
    generatedAt: options.generatedAt,
    family: options.family,
    outboxRootRef: options.outboxRootRef,
    rows,
    linkedRecordCount: countRows(rows, SocialAlertReportLocalOutboxBridgeStatus.Linked),
    manualRequiredCount: countRows(rows, SocialAlertReportLocalOutboxBridgeStatus.ManualRequired),
    unavailableCount: countRows(rows, SocialAlertReportLocalOutboxBridgeStatus.Unavailable),
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    schedulerRuntimeClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

export function socialAlertReportIntentToLocalOutboxBridgeRow(
  options: SocialAlertReportLocalOutboxBridgeOptions,
  candidate: SocialAlertReportIntent
): SocialAlertReportLocalOutboxBridgeRow {
  const intent = SocialAlertReportIntentSchema.parse(candidate);
  const status = bridgeStatusForIntent(intent);

  return SocialAlertReportLocalOutboxBridgeRowSchema.parse({
    bridgeRecordId: `social-alert-report-outbox-bridge-${intent.alertReportIntentId}`,
    status,
    intent,
    outboxRecord:
      status === SocialAlertReportLocalOutboxBridgeStatus.Linked ? localOutboxRecordForIntent(options, intent) : null,
    blockedReasonRefs: status === SocialAlertReportLocalOutboxBridgeStatus.Linked ? [] : intent.manualProofRequirements,
  });
}

export function serializeSocialAlertReportLocalOutboxJsonl(
  readModel: SocialAlertReportLocalOutboxBridgeReadModel
): string {
  return `${readModel.rows
    .flatMap((row) => (row.outboxRecord === null ? [] : [row.outboxRecord]))
    .map((record) => JSON.stringify(record))
    .join('\n')}\n`;
}

export function parseSocialAlertReportLocalOutboxJsonl(jsonl: string): NotificationLocalOutboxRecord[] {
  return jsonl
    .split('\n')
    .filter((line) => line.trim().length > 0)
    .map((line) => NotificationLocalOutboxRecordSchema.parse(JSON.parse(line)));
}

function bridgeStatusForIntent(intent: SocialAlertReportIntent): SocialAlertReportLocalOutboxBridgeStatus {
  if (intent.intentStatus === SocialAlertReportIntentStatus.LocalOutboxEligible) {
    return SocialAlertReportLocalOutboxBridgeStatus.Linked;
  }
  if (intent.intentStatus === SocialAlertReportIntentStatus.Unavailable) {
    return SocialAlertReportLocalOutboxBridgeStatus.Unavailable;
  }
  return SocialAlertReportLocalOutboxBridgeStatus.ManualRequired;
}

function localOutboxRecordForIntent(
  options: SocialAlertReportLocalOutboxBridgeOptions,
  intent: SocialAlertReportIntent
): NotificationLocalOutboxRecord {
  return NotificationLocalOutboxRecordSchema.parse({
    entryId: intent.localOutboxRecordRef,
    state: 'queued-local',
    envelope: {
      alertRef: `social-alert-report-notification-${intent.alertReportIntentId}`,
      family: options.family,
      device: intent.device,
      parentAction: options.parentAction,
      severity: intent.priority,
      reasonCode: socialAlertReportReasonToProviderReason(intent.notificationReasonCode),
      providerChannel: intent.providerChannelPreference,
      evidenceRefs: intent.evidenceReferences,
      policyRefs: intent.policyRefs,
      auditRefs: intent.auditRefs,
      payloadTemplateRef: intent.parentBodyToken,
      providerPayloadPreview:
        'alert id, family device scope, severity, reason code, evidence ref, policy ref, parent action link',
      sensitiveDetailMinimized: true,
      rawChildEvidenceIncluded: false,
      rawUrlOrTitleIncluded: false,
      rawMessageTextIncluded: false,
      screenshotOrReportIncluded: false,
    },
    outboxFileRef: options.outboxFileRef,
    localDataPathRef: options.localDataPathRef,
    deliveryClaimState: 'local-outbox-only',
    visibleAfterAt: null,
    retryAttemptCount: 0,
    quietHoursRef: null,
    retryPolicyRef: null,
    deadLetterRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerCredentialsStored: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    sensitiveProviderMetadataStored: false,
  });
}

function socialAlertReportReasonToProviderReason(reasonCode: string) {
  if (reasonCode === 'social-high-risk-signal' || reasonCode === 'social-feed-video-gate') {
    return V3NotificationRuleReasonCodeSchema.parse('policy-violation');
  }
  if (reasonCode === 'social-capability-unavailable') {
    return V3NotificationRuleReasonCodeSchema.parse('provider-failure');
  }
  return V3NotificationRuleReasonCodeSchema.parse('parent-request');
}

function socialAlertReportBridgeRowIsHonest(
  row: Infer<typeof SocialAlertReportLocalOutboxBridgeRowBaseSchema>
): boolean {
  if (row.status === SocialAlertReportLocalOutboxBridgeStatus.Linked) {
    return (
      row.intent.intentStatus === SocialAlertReportIntentStatus.LocalOutboxEligible &&
      row.intent.deliveryClaimState === SocialAlertReportDeliveryClaimState.LocalOutboxOnly &&
      row.outboxRecord !== null &&
      String(row.outboxRecord.entryId) === String(row.intent.localOutboxRecordRef) &&
      row.blockedReasonRefs.length === 0
    );
  }
  return row.outboxRecord === null && row.blockedReasonRefs.length > 0;
}

function socialAlertReportBridgeReadModelCountsMatch(
  readModel: Infer<typeof SocialAlertReportLocalOutboxBridgeReadModelBaseSchema>
): boolean {
  return (
    readModel.linkedRecordCount === countRows(readModel.rows, SocialAlertReportLocalOutboxBridgeStatus.Linked) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, SocialAlertReportLocalOutboxBridgeStatus.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, SocialAlertReportLocalOutboxBridgeStatus.Unavailable)
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: SocialAlertReportLocalOutboxBridgeStatus }>,
  status: SocialAlertReportLocalOutboxBridgeStatus
): number {
  return rows.filter((row) => row.status === status).length;
}

