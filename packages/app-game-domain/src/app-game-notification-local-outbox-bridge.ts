import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameNotificationDeliveryClaimState,
  AppGameNotificationIntentSchema,
  AppGameNotificationIntentStatus,
  type AppGameNotificationIntent,
} from './app-game-notification-intent';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from './notification-local-outbox-adapter-proof';
import { FamilyReferenceSchema, type FamilyReference, type ParentActionReference } from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import { V3NotificationRuleReasonCodeSchema } from './v3-notification-rule-provider-retry-contract';
const NonEmptyAppGameNotificationOutboxBridgeText = Schema.String.pipe(Schema.minLength(1));
export const AppGameNotificationLocalOutboxBridgeStatus = {
  Linked: 'linked-local-outbox-record',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;
export const AppGameNotificationLocalOutboxBridgeStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameNotificationLocalOutboxBridgeStatus))
);
export const AppGameNotificationLocalOutboxBridgeIdSchema = NonEmptyAppGameNotificationOutboxBridgeText.pipe(
  Schema.brand('AppGameNotificationLocalOutboxBridgeId')
);
export const AppGameNotificationLocalOutboxBridgeReferenceSchema = NonEmptyAppGameNotificationOutboxBridgeText.pipe(
  Schema.brand('AppGameNotificationLocalOutboxBridgeReference')
);
const AppGameNotificationLocalOutboxBridgeRowBaseSchema = Schema.Struct({
  bridgeRecordId: AppGameNotificationLocalOutboxBridgeReferenceSchema,
  status: AppGameNotificationLocalOutboxBridgeStatusSchema,
  intent: AppGameNotificationIntentSchema,
  outboxRecord: Schema.Union(NotificationLocalOutboxRecordSchema, Schema.Null),
  blockedReasonRefs: Schema.Array(AppGameNotificationLocalOutboxBridgeReferenceSchema),
});
export const AppGameNotificationLocalOutboxBridgeRowSchema = withParser(
  AppGameNotificationLocalOutboxBridgeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameNotificationBridgeRowIsHonest(row) ||
        'Expected app/game notification outbox bridge rows to link only local-outbox-eligible intents and keep manual/unavailable intents out of queued records'
    )
  )
);
const AppGameNotificationLocalOutboxBridgeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  bridgeId: AppGameNotificationLocalOutboxBridgeIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  outboxRootRef: AppGameNotificationLocalOutboxBridgeReferenceSchema,
  rows: Schema.Array(AppGameNotificationLocalOutboxBridgeRowSchema),
  linkedRecordCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  schedulerRuntimeClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});
export const AppGameNotificationLocalOutboxBridgeReadModelSchema = withParser(
  AppGameNotificationLocalOutboxBridgeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameNotificationBridgeReadModelCountsMatch(readModel) ||
        'Expected app/game notification outbox bridge counts to match linked manual-required and unavailable rows'
    )
  )
);
export type AppGameNotificationLocalOutboxBridgeStatus = Infer<typeof AppGameNotificationLocalOutboxBridgeStatusSchema>;
export type AppGameNotificationLocalOutboxBridgeRow = Infer<typeof AppGameNotificationLocalOutboxBridgeRowSchema>;
export type AppGameNotificationLocalOutboxBridgeReadModel = Infer<
  typeof AppGameNotificationLocalOutboxBridgeReadModelSchema
>;
export type AppGameNotificationLocalOutboxBridgeOptions = {
  readonly family: FamilyReference;
  readonly parentAction: ParentActionReference;
  readonly generatedAt: string;
  readonly bridgeId: string;
  readonly outboxRootRef: string;
  readonly outboxFileRef: string;
  readonly localDataPathRef: string;
};
export function buildAppGameNotificationLocalOutboxBridgeReadModel(
  options: AppGameNotificationLocalOutboxBridgeOptions,
  intents: ReadonlyArray<AppGameNotificationIntent>
): AppGameNotificationLocalOutboxBridgeReadModel {
  const rows = intents.map((intent) => appGameNotificationIntentToLocalOutboxBridgeRow(options, intent));
  return AppGameNotificationLocalOutboxBridgeReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    bridgeId: options.bridgeId,
    generatedAt: options.generatedAt,
    family: options.family,
    outboxRootRef: options.outboxRootRef,
    rows,
    linkedRecordCount: countRows(rows, AppGameNotificationLocalOutboxBridgeStatus.Linked),
    manualRequiredCount: countRows(rows, AppGameNotificationLocalOutboxBridgeStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameNotificationLocalOutboxBridgeStatus.Unavailable),
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    schedulerRuntimeClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    childDeliveryClaimed: false,
    adapterDispatchClaimed: false,
  });
}
export function appGameNotificationIntentToLocalOutboxBridgeRow(
  options: AppGameNotificationLocalOutboxBridgeOptions,
  candidate: AppGameNotificationIntent
): AppGameNotificationLocalOutboxBridgeRow {
  const intent = AppGameNotificationIntentSchema.parse(candidate);
  const status = bridgeStatusForIntent(intent);

  return AppGameNotificationLocalOutboxBridgeRowSchema.parse({
    bridgeRecordId: `app-game-notification-outbox-bridge-${intent.notificationIntentId}`,
    status,
    intent,
    outboxRecord:
      status === AppGameNotificationLocalOutboxBridgeStatus.Linked ? localOutboxRecordForIntent(options, intent) : null,
    blockedReasonRefs:
      status === AppGameNotificationLocalOutboxBridgeStatus.Linked ? [] : intent.manualProofRequirements,
  });
}
export function serializeAppGameNotificationLocalOutboxJsonl(
  readModel: AppGameNotificationLocalOutboxBridgeReadModel
): string {
  return `${readModel.rows
    .flatMap((row) => (row.outboxRecord === null ? [] : [row.outboxRecord]))
    .map((record) => JSON.stringify(record))
    .join('\n')}\n`;
}
export function parseAppGameNotificationLocalOutboxJsonl(jsonl: string): NotificationLocalOutboxRecord[] {
  return jsonl
    .split('\n')
    .filter((line) => line.trim().length > 0)
    .map((line) => NotificationLocalOutboxRecordSchema.parse(JSON.parse(line)));
}
function bridgeStatusForIntent(intent: AppGameNotificationIntent): AppGameNotificationLocalOutboxBridgeStatus {
  if (intent.intentStatus === AppGameNotificationIntentStatus.LocalOutboxEligible) {
    return AppGameNotificationLocalOutboxBridgeStatus.Linked;
  }
  if (intent.intentStatus === AppGameNotificationIntentStatus.Unavailable) {
    return AppGameNotificationLocalOutboxBridgeStatus.Unavailable;
  }
  return AppGameNotificationLocalOutboxBridgeStatus.ManualRequired;
}
function localOutboxRecordForIntent(
  options: AppGameNotificationLocalOutboxBridgeOptions,
  intent: AppGameNotificationIntent
): NotificationLocalOutboxRecord {
  return NotificationLocalOutboxRecordSchema.parse({
    entryId: intent.localOutboxRecordRef,
    state: 'queued-local',
    envelope: {
      alertRef: `app-game-notification-alert-${intent.notificationIntentId}`,
      family: options.family,
      device: intent.device,
      parentAction: options.parentAction,
      severity: intent.priority,
      reasonCode: appGameNotificationReasonToProviderReason(intent.notificationReasonCode),
      providerChannel: intent.providerChannelPreference,
      evidenceRefs: intent.evidenceReferences,
      policyRefs: intent.policyRefs,
      auditRefs: intent.auditRefs,
      payloadTemplateRef: intent.notificationRuleRef,
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
function appGameNotificationReasonToProviderReason(reasonCode: string) {
  if (reasonCode === 'app-game-time-limit') {
    return V3NotificationRuleReasonCodeSchema.parse('policy-violation');
  }
  if (reasonCode === 'app-game-suspicious-unknown') {
    return V3NotificationRuleReasonCodeSchema.parse('suspicious-unknown');
  }
  return V3NotificationRuleReasonCodeSchema.parse('parent-request');
}
function appGameNotificationBridgeRowIsHonest(
  row: Infer<typeof AppGameNotificationLocalOutboxBridgeRowBaseSchema>
): boolean {
  if (row.status === AppGameNotificationLocalOutboxBridgeStatus.Linked) {
    return (
      row.intent.intentStatus === AppGameNotificationIntentStatus.LocalOutboxEligible &&
      row.intent.deliveryClaimState === AppGameNotificationDeliveryClaimState.LocalOutboxOnly &&
      row.outboxRecord !== null &&
      String(row.outboxRecord.entryId) === String(row.intent.localOutboxRecordRef) &&
      row.blockedReasonRefs.length === 0
    );
  }
  return row.outboxRecord === null && row.blockedReasonRefs.length > 0;
}
function appGameNotificationBridgeReadModelCountsMatch(
  readModel: Infer<typeof AppGameNotificationLocalOutboxBridgeReadModelBaseSchema>
): boolean {
  return (
    readModel.linkedRecordCount === countRows(readModel.rows, AppGameNotificationLocalOutboxBridgeStatus.Linked) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, AppGameNotificationLocalOutboxBridgeStatus.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, AppGameNotificationLocalOutboxBridgeStatus.Unavailable)
  );
}
function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameNotificationLocalOutboxBridgeStatus }>,
  status: AppGameNotificationLocalOutboxBridgeStatus
): number {
  return rows.filter((row) => row.status === status).length;
}
