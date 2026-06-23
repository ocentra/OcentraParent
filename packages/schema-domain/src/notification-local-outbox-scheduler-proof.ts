import {
  NotificationLocalOutboxProviderChannels,
  NotificationLocalOutboxSchedulerEntryIdSchema,
  NotificationLocalOutboxSchedulerKnownGaps,
  NotificationLocalOutboxSchedulerNonClaimSchema,
  NotificationLocalOutboxSchedulerProofReadModel,
  NotificationLocalOutboxSchedulerProofSchema,
  NotificationLocalOutboxSchedulerProofSchemaVersionSchema,
  NotificationLocalOutboxSchedulerReadModelIdSchema,
  NotificationLocalOutboxSchedulerRecordSchema,
  NotificationLocalOutboxSchedulerStateSchema,
  RequiredNotificationLocalOutboxSchedulerStates,
  decodeNotificationLocalOutboxSchedulerProof,
  decodeNotificationLocalOutboxSchedulerRecord,
} from './notification-local-outbox';
import type {
  NotificationLocalOutboxSchedulerNonClaim,
  NotificationLocalOutboxSchedulerProof,
  NotificationLocalOutboxSchedulerRecord,
  NotificationLocalOutboxSchedulerState,
} from './notification-local-outbox';
import type { V3NotificationProviderChannel } from './notification-v3-provider-retry';

export {
  NotificationLocalOutboxProviderChannels,
  NotificationLocalOutboxSchedulerKnownGaps,
  NotificationLocalOutboxSchedulerProofReadModel,
  NotificationLocalOutboxSchedulerProofSchema,
  NotificationLocalOutboxSchedulerProofSchemaVersionSchema,
  NotificationLocalOutboxSchedulerRecordSchema,
  NotificationLocalOutboxSchedulerStateSchema,
  RequiredNotificationLocalOutboxSchedulerStates,
  NotificationLocalOutboxSchedulerEntryIdSchema,
  NotificationLocalOutboxSchedulerNonClaimSchema,
  NotificationLocalOutboxSchedulerReadModelIdSchema,
  decodeNotificationLocalOutboxSchedulerProof,
  decodeNotificationLocalOutboxSchedulerRecord,
};

export type {
  NotificationLocalOutboxSchedulerNonClaim,
  NotificationLocalOutboxSchedulerProof,
  NotificationLocalOutboxSchedulerRecord,
  NotificationLocalOutboxSchedulerState,
};

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

function countBy<const T extends string>(values: ReadonlyArray<T>, keys: readonly T[]): Record<T, number> {
  return Object.fromEntries(keys.map((key) => [key, values.filter((value) => value === key).length])) as Record<
    T,
    number
  >;
}
