import {
  NotificationLocalOutboxAdapterProofReadModel,
  NotificationLocalOutboxAdapterProofSchema,
  NotificationLocalOutboxAdapterProofSchemaVersionSchema,
  NotificationLocalOutboxDeliveryClaimStateSchema,
  NotificationLocalOutboxEntryIdSchema,
  NotificationLocalOutboxForbiddenDetailFragments,
  NotificationLocalOutboxKnownGaps,
  NotificationLocalOutboxMinimalAlertEnvelopeSchema,
  NotificationLocalOutboxNonClaimSchema,
  NotificationLocalOutboxPayloadPreviewSchema,
  NotificationLocalOutboxProviderChannels,
  NotificationLocalOutboxReadModelIdSchema,
  NotificationLocalOutboxRecordSchema,
  NotificationLocalOutboxReferenceSchema,
  NotificationLocalOutboxSeveritySchema,
  NotificationLocalOutboxStateSchema,
  RequiredNotificationLocalOutboxStates,
  decodeNotificationLocalOutboxAdapterProof,
  decodeNotificationLocalOutboxRecord,
} from './notification-local-outbox';
import type {
  NotificationLocalOutboxAdapterProof,
  NotificationLocalOutboxDeliveryClaimState,
  NotificationLocalOutboxMinimalAlertEnvelope,
  NotificationLocalOutboxNonClaim,
  NotificationLocalOutboxRecord,
  NotificationLocalOutboxSeverity,
  NotificationLocalOutboxState,
} from './notification-local-outbox';
import type { V3NotificationProviderChannel } from './notification-v3-provider-retry';

export {
  NotificationLocalOutboxAdapterProofReadModel,
  NotificationLocalOutboxAdapterProofSchema,
  NotificationLocalOutboxAdapterProofSchemaVersionSchema,
  NotificationLocalOutboxDeliveryClaimStateSchema,
  NotificationLocalOutboxEntryIdSchema,
  NotificationLocalOutboxForbiddenDetailFragments,
  NotificationLocalOutboxKnownGaps,
  NotificationLocalOutboxMinimalAlertEnvelopeSchema,
  NotificationLocalOutboxNonClaimSchema,
  NotificationLocalOutboxPayloadPreviewSchema,
  NotificationLocalOutboxProviderChannels,
  NotificationLocalOutboxReadModelIdSchema,
  NotificationLocalOutboxRecordSchema,
  NotificationLocalOutboxReferenceSchema,
  NotificationLocalOutboxSeveritySchema,
  NotificationLocalOutboxStateSchema,
  RequiredNotificationLocalOutboxStates,
  decodeNotificationLocalOutboxAdapterProof,
  decodeNotificationLocalOutboxRecord,
};

export type {
  NotificationLocalOutboxAdapterProof,
  NotificationLocalOutboxDeliveryClaimState,
  NotificationLocalOutboxMinimalAlertEnvelope,
  NotificationLocalOutboxNonClaim,
  NotificationLocalOutboxRecord,
  NotificationLocalOutboxSeverity,
  NotificationLocalOutboxState,
};

export function summarizeNotificationLocalOutboxStates(
  records: ReadonlyArray<NotificationLocalOutboxRecord>
): Record<NotificationLocalOutboxState, number> {
  return countBy(
    records.map((record) => record.state),
    RequiredNotificationLocalOutboxStates
  );
}

export function summarizeNotificationLocalOutboxChannels(
  records: ReadonlyArray<NotificationLocalOutboxRecord>
): Record<V3NotificationProviderChannel, number> {
  return countBy(
    records.map((record) => record.envelope.providerChannel),
    NotificationLocalOutboxProviderChannels
  );
}

function countBy<const T extends string>(values: ReadonlyArray<T>, keys: readonly T[]): Record<T, number> {
  return Object.fromEntries(keys.map((key) => [key, values.filter((value) => value === key).length])) as Record<
    T,
    number
  >;
}
