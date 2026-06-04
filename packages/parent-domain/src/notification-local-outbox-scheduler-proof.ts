import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  NotificationLocalOutboxAdapterProofReadModel,
  type NotificationLocalOutboxRecord,
} from './notification-local-outbox-adapter-proof';
import {
  NotificationLocalOutboxSchedulerArtifactRef,
  NotificationLocalOutboxSchedulerKnownGaps,
  NotificationLocalOutboxSchedulerProofNow,
  NotificationLocalOutboxSchedulerProofRows,
  NotificationLocalOutboxSchedulerProofTimestamp,
  RequiredNotificationLocalOutboxSchedulerNonClaims,
  RequiredNotificationLocalOutboxSchedulerStates,
} from './notification-local-outbox-scheduler-proof-values';
import {
  NotificationLocalOutboxSchedulerProofSchema,
  NotificationLocalOutboxSchedulerRecordSchema,
  type NotificationLocalOutboxSchedulerRecord,
  type NotificationLocalOutboxSchedulerState,
} from './notification-local-outbox-scheduler-proof-schemas';
import { NotificationLocalOutboxProviderChannels } from './notification-local-outbox-adapter-proof-values';
import { type V3NotificationProviderChannel } from './v3-notification-rule-provider-retry-contract';
import { ParentContractSchemaVersion } from './reference-primitives';

export {
  NotificationLocalOutboxSchedulerEntryIdSchema,
  NotificationLocalOutboxSchedulerNonClaimSchema,
  NotificationLocalOutboxSchedulerProofSchema,
  NotificationLocalOutboxSchedulerProofSchemaVersionSchema,
  NotificationLocalOutboxSchedulerReadModelIdSchema,
  NotificationLocalOutboxSchedulerRecordSchema,
  NotificationLocalOutboxSchedulerStateSchema,
} from './notification-local-outbox-scheduler-proof-schemas';
export type {
  NotificationLocalOutboxSchedulerNonClaim,
  NotificationLocalOutboxSchedulerProof,
  NotificationLocalOutboxSchedulerRecord,
  NotificationLocalOutboxSchedulerState,
} from './notification-local-outbox-scheduler-proof-schemas';
export { NotificationLocalOutboxSchedulerKnownGaps };

type SchedulerInput = (typeof NotificationLocalOutboxSchedulerProofRows)[number];

function schedulerRecord(input: SchedulerInput): NotificationLocalOutboxSchedulerRecord {
  const source = sourceRecordFor(input.sourceEntryId);

  return NotificationLocalOutboxSchedulerRecordSchema.parse({
    schedulerEntryId: `notification-local-outbox-scheduler-${input.schedulerState}-${source.entryId}`,
    sourceEntryId: source.entryId,
    sourceState: source.state,
    schedulerState: input.schedulerState,
    reasonCode: source.envelope.reasonCode,
    providerChannel: source.envelope.providerChannel,
    severity: source.envelope.severity,
    schedulerDecisionRef: input.schedulerDecisionRef,
    schedulerArtifactRef: NotificationLocalOutboxSchedulerArtifactRef,
    sourceOutboxFileRef: source.outboxFileRef,
    localDataPathRef: source.localDataPathRef,
    schedulerNowAt: NotificationLocalOutboxSchedulerProofNow,
    nextAttemptAt: input.nextAttemptAt,
    quietHoursWindow: input.quietHoursWindow,
    retryWindow: input.retryWindow,
    deadLetterReviewRef: input.deadLetterReviewRef,
    providerReceiptRef: input.providerReceiptRef,
    manualProofRequirements: input.manualProofRequirements,
    manualActionRequired: input.manualActionRequired,
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
    schedulerPayloadPreview: input.schedulerPayloadPreview,
  });
}

export const NotificationLocalOutboxSchedulerProofReadModel = NotificationLocalOutboxSchedulerProofSchema.parse({
  schemaVersion: 'notification-local-outbox-scheduler-proof',
  contractVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'notification-local-outbox-scheduler-proof',
  generatedAt: NotificationLocalOutboxSchedulerProofTimestamp,
  schedulerNowAt: NotificationLocalOutboxSchedulerProofNow,
  schedulerArtifactRootRef: 'parent-owned-local-notification-outbox-scheduler-root',
  sourceAdapterReadModelId: NotificationLocalOutboxAdapterProofReadModel.readModelId,
  records: NotificationLocalOutboxSchedulerProofRows.map((row) => schedulerRecord(row)),
  nonClaims: RequiredNotificationLocalOutboxSchedulerNonClaims,
  providerDeliveryRuntimeClaimed: false,
  providerReceiptIngestionClaimed: false,
  providerCredentialsClaimed: false,
  cloudRoutingClaimed: false,
  parentNotificationUiClaimed: false,
  retryExecutionRuntimeClaimed: false,
  quietHoursTimerRuntimeClaimed: false,
  productionDurableOutboxStorageClaimed: false,
});

export function summarizeNotificationLocalOutboxSchedulerStates(
  records: ReadonlyArray<NotificationLocalOutboxSchedulerRecord>
): Record<NotificationLocalOutboxSchedulerState, number> {
  return countBy(
    records.map((record) => record.schedulerState),
    RequiredNotificationLocalOutboxSchedulerStates
  );
}

export function summarizeNotificationLocalOutboxSchedulerChannels(
  records: ReadonlyArray<NotificationLocalOutboxSchedulerRecord>
): Record<V3NotificationProviderChannel, number> {
  return countBy(
    records.map((record) => record.providerChannel),
    NotificationLocalOutboxProviderChannels
  );
}

export function dueNotificationLocalOutboxSchedulerRecords(
  records: ReadonlyArray<NotificationLocalOutboxSchedulerRecord>
): ReadonlyArray<NotificationLocalOutboxSchedulerRecord> {
  return records.filter(
    (record) => record.schedulerState === 'due-local' && record.nextAttemptAt === record.schedulerNowAt
  );
}

function sourceRecordFor(sourceEntryId: SchedulerInput['sourceEntryId']): NotificationLocalOutboxRecord {
  const record = NotificationLocalOutboxAdapterProofReadModel.records.find(
    (candidate) => candidate.entryId === sourceEntryId
  );
  if (record === undefined) {
    throw new Error(`Missing notification local outbox source record: ${sourceEntryId}`);
  }
  return record;
}

function countBy<const T extends string>(values: ReadonlyArray<T>, keys: readonly T[]): Record<T, number> {
  return Object.fromEntries(keys.map((key) => [key, values.filter((value) => value === key).length])) as Record<
    T,
    number
  >;
}

export const decodeNotificationLocalOutboxSchedulerRecord = Schema.decodeUnknownSync(
  NotificationLocalOutboxSchedulerRecordSchema
);
export const decodeNotificationLocalOutboxSchedulerProof = Schema.decodeUnknownSync(
  NotificationLocalOutboxSchedulerProofSchema
);
