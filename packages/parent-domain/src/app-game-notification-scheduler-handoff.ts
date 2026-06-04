import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGameNotificationIntentKind, AppGameNotificationReferenceSchema } from './app-game-notification-intent';
import {
  AppGameNotificationLocalOutboxBridgeBlockReasonSchema,
  AppGameNotificationLocalOutboxBridgeReferenceSchema,
  AppGameNotificationLocalOutboxBridgeSchemaVersionSchema,
  type AppGameNotificationLocalOutboxBridgeProof,
} from './app-game-notification-local-outbox-bridge';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from './notification-local-outbox-adapter-proof';
import {
  NotificationLocalOutboxSchedulerEntryIdSchema,
  NotificationLocalOutboxSchedulerRecordSchema,
  type NotificationLocalOutboxSchedulerRecord,
  type NotificationLocalOutboxSchedulerState,
} from './notification-local-outbox-scheduler-proof';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
  type ParentTimestamp,
} from './reference-primitives';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleReasonCodeSchema,
  type V3NotificationProviderChannel,
} from './v3-notification-rule-provider-retry-contract';

const NonEmptyAppGameSchedulerHandoffText = Schema.String.pipe(Schema.minLength(1));

export const AppGameNotificationSchedulerHandoffSchemaVersionSchema = withParser(
  Schema.Literal('app-game-notification-scheduler-handoff')
);
export const AppGameNotificationSchedulerHandoffReadModelIdSchema = NonEmptyAppGameSchedulerHandoffText.pipe(
  Schema.brand('AppGameNotificationSchedulerHandoffReadModelId')
);
export const AppGameNotificationSchedulerHandoffReferenceSchema = NonEmptyAppGameSchedulerHandoffText.pipe(
  Schema.brand('AppGameNotificationSchedulerHandoffReference')
);
export const AppGameNotificationSchedulerHandoffNonClaimSchema = withParser(
  Schema.Literal(
    'no-provider-delivery',
    'no-provider-receipt-ingestion',
    'no-provider-credentials',
    'no-cloud-routing',
    'no-parent-notification-ui',
    'no-retry-worker-execution',
    'no-quiet-hours-timer-runtime',
    'no-production-durable-outbox-storage',
    'no-durable-service-persistence',
    'no-child-device-delivery',
    'no-adapter-dispatch',
    'no-broad-app-blocking',
    'no-platform-support',
    'no-raw-child-evidence'
  )
);

export const AppGameNotificationSchedulerHandoffNonClaims = [
  'no-provider-delivery',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-retry-worker-execution',
  'no-quiet-hours-timer-runtime',
  'no-production-durable-outbox-storage',
  'no-durable-service-persistence',
  'no-child-device-delivery',
  'no-adapter-dispatch',
  'no-broad-app-blocking',
  'no-platform-support',
  'no-raw-child-evidence',
] as const;

export const AppGameNotificationSchedulerHandoffKnownGaps = [
  'App/game scheduler handoff rows remain parent-domain proof rows and do not run a production timer or retry worker.',
  'Provider delivery, receipt ingestion, credentials, cloud routing, parent notification UI, and adapter dispatch remain unclaimed.',
  'Durable service persistence, child-device delivery, broad app/game blocking, platform support, and raw child evidence remain future proof work.',
] as const;

const AppGameNotificationSchedulerHandoffScheduledIntentSchema = withParser(
  Schema.Struct({
    notificationIntentRef: AppGameNotificationReferenceSchema,
    localOutboxRecordRef: AppGameNotificationReferenceSchema,
    schedulerEntryRef: NotificationLocalOutboxSchedulerEntryIdSchema,
    intentKind: Schema.Literal(
      AppGameNotificationIntentKind.TimeLimitReached,
      AppGameNotificationIntentKind.ApprovalRequested,
      AppGameNotificationIntentKind.SuspiciousUnknown
    ),
    outboxReasonCode: V3NotificationRuleReasonCodeSchema,
    providerChannel: V3NotificationProviderChannelSchema,
    schedulerState: Schema.Literal('due-local'),
  })
);

const AppGameNotificationSchedulerHandoffBlockedIntentSchema = withParser(
  Schema.Struct({
    notificationIntentRef: AppGameNotificationReferenceSchema,
    blockReason: AppGameNotificationLocalOutboxBridgeBlockReasonSchema,
    manualProofRequirements: Schema.Array(AppGameNotificationReferenceSchema),
  })
);

const AppGameNotificationSchedulerHandoffProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameNotificationSchedulerHandoffSchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  sourceBridgeSchemaVersion: AppGameNotificationLocalOutboxBridgeSchemaVersionSchema,
  readModelId: AppGameNotificationSchedulerHandoffReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  schedulerNowAt: ParentTimestampSchema,
  sourceBridgeReadModelRef: AppGameNotificationLocalOutboxBridgeReferenceSchema,
  sourceIntentReadModelRef: AppGameNotificationLocalOutboxBridgeReferenceSchema,
  localOutboxReadModelRef: AppGameNotificationLocalOutboxBridgeReferenceSchema,
  schedulerArtifactRootRef: AppGameNotificationSchedulerHandoffReferenceSchema,
  scheduledIntentRefs: Schema.Array(AppGameNotificationSchedulerHandoffScheduledIntentSchema),
  blockedIntentRefs: Schema.Array(AppGameNotificationSchedulerHandoffBlockedIntentSchema),
  sourceLocalOutboxRecords: Schema.Array(NotificationLocalOutboxRecordSchema),
  records: Schema.Array(NotificationLocalOutboxSchedulerRecordSchema),
  nonClaims: Schema.Array(AppGameNotificationSchedulerHandoffNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Boolean,
  providerReceiptIngestionClaimed: Schema.Boolean,
  providerCredentialsClaimed: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
  retryExecutionRuntimeClaimed: Schema.Boolean,
  quietHoursTimerRuntimeClaimed: Schema.Boolean,
  productionDurableOutboxStorageClaimed: Schema.Boolean,
  durableServicePersistenceClaimed: Schema.Boolean,
  childDeviceDeliveryClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  broadAppBlockingClaimed: Schema.Boolean,
  platformSupportClaimed: Schema.Boolean,
});

type AppGameNotificationSchedulerHandoffProofCandidate = Infer<
  typeof AppGameNotificationSchedulerHandoffProofBaseSchema
>;

export const AppGameNotificationSchedulerHandoffProofSchema = withParser(
  AppGameNotificationSchedulerHandoffProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        appGameNotificationSchedulerHandoffProofIsHonest(proof) ||
        'Expected app/game notification scheduler handoff proof to map every bridged local outbox row into a due-local scheduler row, preserve blocked manual/unavailable intents, and keep provider, timer, UI, durable storage, adapter, broad-blocking, platform, and child-device claims false'
    )
  )
);

export type AppGameNotificationSchedulerHandoffNonClaim = Infer<
  typeof AppGameNotificationSchedulerHandoffNonClaimSchema
>;
export type AppGameNotificationSchedulerHandoffScheduledIntent = Infer<
  typeof AppGameNotificationSchedulerHandoffScheduledIntentSchema
>;
export type AppGameNotificationSchedulerHandoffBlockedIntent = Infer<
  typeof AppGameNotificationSchedulerHandoffBlockedIntentSchema
>;
export type AppGameNotificationSchedulerHandoffProof = Infer<typeof AppGameNotificationSchedulerHandoffProofSchema>;
type AppGameNotificationSchedulerHandoffBridgeLink =
  AppGameNotificationLocalOutboxBridgeProof['bridgedIntentRefs'][number];

export type AppGameNotificationSchedulerHandoffInput = {
  readonly generatedAt: ParentTimestamp;
  readonly schedulerNowAt: ParentTimestamp;
  readonly schedulerArtifactRootRef: string;
  readonly bridgeProof: AppGameNotificationLocalOutboxBridgeProof;
};

const HandoffClaimFlags = [
  'providerDeliveryRuntimeClaimed',
  'providerReceiptIngestionClaimed',
  'providerCredentialsClaimed',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
  'retryExecutionRuntimeClaimed',
  'quietHoursTimerRuntimeClaimed',
  'productionDurableOutboxStorageClaimed',
  'durableServicePersistenceClaimed',
  'childDeviceDeliveryClaimed',
  'adapterDispatchClaimed',
  'broadAppBlockingClaimed',
  'platformSupportClaimed',
] as const;

const SchedulerStatesForSummary = [
  'due-local',
  'held-quiet-hours',
  'retry-window-scheduled',
  'dead-letter-review',
  'receipt-required',
  'manual-required',
] as const;

export function buildAppGameNotificationSchedulerHandoffProof(
  input: AppGameNotificationSchedulerHandoffInput
): AppGameNotificationSchedulerHandoffProof {
  const recordsByEntry = new Map(input.bridgeProof.records.map((record) => [String(record.entryId), record] as const));
  const schedulerRecords = input.bridgeProof.bridgedIntentRefs.map((link) =>
    schedulerRecordForBridgeLink(input, recordForLink(recordsByEntry, String(link.outboxEntryRef)), link)
  );

  return AppGameNotificationSchedulerHandoffProofSchema.parse({
    schemaVersion: 'app-game-notification-scheduler-handoff',
    contractVersion: ParentContractSchemaVersion.V0_6,
    sourceBridgeSchemaVersion: input.bridgeProof.schemaVersion,
    readModelId: 'app-game-notification-scheduler-handoff',
    generatedAt: input.generatedAt,
    schedulerNowAt: input.schedulerNowAt,
    sourceBridgeReadModelRef: input.bridgeProof.readModelId,
    sourceIntentReadModelRef: input.bridgeProof.sourceIntentReadModelRef,
    localOutboxReadModelRef: input.bridgeProof.localOutboxReadModelRef,
    schedulerArtifactRootRef: input.schedulerArtifactRootRef,
    scheduledIntentRefs: input.bridgeProof.bridgedIntentRefs.map((link) => ({
      notificationIntentRef: link.notificationIntentRef,
      localOutboxRecordRef: link.localOutboxRecordRef,
      schedulerEntryRef: schedulerEntryRefForLocalOutboxRecord(link.localOutboxRecordRef),
      intentKind: link.intentKind,
      outboxReasonCode: link.outboxReasonCode,
      providerChannel: link.providerChannel,
      schedulerState: 'due-local',
    })),
    blockedIntentRefs: input.bridgeProof.blockedIntentRefs.map((blocked) => ({
      notificationIntentRef: blocked.notificationIntentRef,
      blockReason: blocked.blockReason,
      manualProofRequirements: blocked.manualProofRequirements,
    })),
    sourceLocalOutboxRecords: input.bridgeProof.records,
    records: schedulerRecords,
    nonClaims: AppGameNotificationSchedulerHandoffNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    durableServicePersistenceClaimed: false,
    childDeviceDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    broadAppBlockingClaimed: false,
    platformSupportClaimed: false,
  });
}

export function summarizeAppGameNotificationSchedulerHandoffStates(
  proof: AppGameNotificationSchedulerHandoffProof
): Record<NotificationLocalOutboxSchedulerState, number> {
  return countBy(
    proof.records.map((record) => record.schedulerState),
    SchedulerStatesForSummary
  );
}

export function summarizeAppGameNotificationSchedulerHandoffChannels(
  proof: AppGameNotificationSchedulerHandoffProof
): Record<V3NotificationProviderChannel, number> {
  return countBy(
    proof.records.map((record) => record.providerChannel),
    ['push', 'email', 'sms', 'whatsapp', 'in-app']
  );
}

export function dueAppGameNotificationSchedulerHandoffRecords(
  proof: AppGameNotificationSchedulerHandoffProof
): NotificationLocalOutboxSchedulerRecord[] {
  return proof.records.filter((record) => record.schedulerState === 'due-local');
}

function appGameNotificationSchedulerHandoffProofIsHonest(
  proof: AppGameNotificationSchedulerHandoffProofCandidate
): boolean {
  return (
    proof.records.length > 0 &&
    proof.records.length === proof.scheduledIntentRefs.length &&
    proof.records.length === proof.sourceLocalOutboxRecords.length &&
    AppGameNotificationSchedulerHandoffNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    HandoffClaimFlags.every((flag) => proof[flag] === false) &&
    proof.records.every((record) => record.schedulerState === 'due-local' && record.sourceState === 'queued-local') &&
    appGameNotificationSchedulerHandoffLinksMatchRecords(proof) &&
    proof.blockedIntentRefs.every(
      (blocked) => blocked.blockReason === 'intent-only-no-local-outbox' || blocked.manualProofRequirements.length > 0
    )
  );
}

function appGameNotificationSchedulerHandoffLinksMatchRecords(
  proof: AppGameNotificationSchedulerHandoffProofCandidate
): boolean {
  const recordsBySourceEntry = new Map(proof.records.map((record) => [String(record.sourceEntryId), record] as const));
  const sourceIds = new Set(proof.sourceLocalOutboxRecords.map((record) => String(record.entryId)));
  const schedulerIds = new Set(proof.records.map((record) => String(record.schedulerEntryId)));

  return (
    sourceIds.size === proof.sourceLocalOutboxRecords.length &&
    schedulerIds.size === proof.records.length &&
    proof.scheduledIntentRefs.every((link) => {
      const record = recordsBySourceEntry.get(String(link.localOutboxRecordRef));
      return (
        record !== undefined &&
        sourceIds.has(String(link.localOutboxRecordRef)) &&
        schedulerIds.has(String(link.schedulerEntryRef)) &&
        String(record.schedulerEntryId) === String(link.schedulerEntryRef) &&
        record.reasonCode === link.outboxReasonCode &&
        record.providerChannel === link.providerChannel &&
        record.schedulerState === link.schedulerState
      );
    })
  );
}

function schedulerRecordForBridgeLink(
  input: AppGameNotificationSchedulerHandoffInput,
  record: NotificationLocalOutboxRecord,
  link: AppGameNotificationSchedulerHandoffBridgeLink
): NotificationLocalOutboxSchedulerRecord {
  return NotificationLocalOutboxSchedulerRecordSchema.parse({
    schedulerEntryId: schedulerEntryRefForLocalOutboxRecord(link.localOutboxRecordRef),
    sourceEntryId: record.entryId,
    sourceState: record.state,
    schedulerState: 'due-local',
    reasonCode: record.envelope.reasonCode,
    providerChannel: record.envelope.providerChannel,
    severity: record.envelope.severity,
    schedulerDecisionRef: `app-game-scheduler-decision-${link.notificationIntentRef}`,
    schedulerArtifactRef: `${input.schedulerArtifactRootRef}-${link.notificationIntentRef}`,
    sourceOutboxFileRef: record.outboxFileRef,
    localDataPathRef: record.localDataPathRef,
    schedulerNowAt: input.schedulerNowAt,
    nextAttemptAt: input.schedulerNowAt,
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
    schedulerPayloadPreview: `app-game alert ref ${link.notificationIntentRef} due locally with ${link.outboxReasonCode} refs only`,
  });
}

function schedulerEntryRefForLocalOutboxRecord(localOutboxRecordRef: string): string {
  return `app-game-scheduler-due-${localOutboxRecordRef}`;
}

function recordForLink(
  recordsByEntry: ReadonlyMap<string, NotificationLocalOutboxRecord>,
  localOutboxRecordRef: string
): NotificationLocalOutboxRecord {
  const record = recordsByEntry.get(localOutboxRecordRef);
  if (record === undefined) {
    throw new Error(`Missing local outbox source record for app/game scheduler handoff: ${localOutboxRecordRef}`);
  }
  return record;
}

function countBy<const T extends string>(values: ReadonlyArray<T>, keys: readonly T[]): Record<T, number> {
  return Object.fromEntries(keys.map((key) => [key, values.filter((value) => value === key).length])) as Record<
    T,
    number
  >;
}

export const decodeAppGameNotificationSchedulerHandoffProof = Schema.decodeUnknownSync(
  AppGameNotificationSchedulerHandoffProofSchema
);
