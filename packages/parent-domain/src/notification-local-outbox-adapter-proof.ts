import { Schema } from '@ocentra-parent/schema-domain/effect';
import { type V3NotificationProviderChannel } from './v3-notification-rule-provider-retry-contract';
import {
  NotificationLocalOutboxForbiddenDetailFragments,
  NotificationLocalOutboxKnownGaps,
  NotificationLocalOutboxProofDevice,
  NotificationLocalOutboxProofEvidenceRef,
  NotificationLocalOutboxProofFamily,
  NotificationLocalOutboxProofParentAction,
  NotificationLocalOutboxProofRows,
  NotificationLocalOutboxProofTimestamp,
  NotificationLocalOutboxProviderChannels,
  RequiredNotificationLocalOutboxNonClaims,
  RequiredNotificationLocalOutboxStates,
} from './notification-local-outbox-adapter-proof-values';
import {
  NotificationLocalOutboxAdapterProofSchema,
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
  type NotificationLocalOutboxState,
} from './notification-local-outbox-adapter-proof-schemas';
import { ParentContractSchemaVersion } from './reference-primitives';

export {
  NotificationLocalOutboxAdapterProofSchema,
  NotificationLocalOutboxAdapterProofSchemaVersionSchema,
  NotificationLocalOutboxDeliveryClaimStateSchema,
  NotificationLocalOutboxEntryIdSchema,
  NotificationLocalOutboxMinimalAlertEnvelopeSchema,
  NotificationLocalOutboxNonClaimSchema,
  NotificationLocalOutboxPayloadPreviewSchema,
  NotificationLocalOutboxReadModelIdSchema,
  NotificationLocalOutboxRecordSchema,
  NotificationLocalOutboxReferenceSchema,
  NotificationLocalOutboxSeveritySchema,
  NotificationLocalOutboxStateSchema,
} from './notification-local-outbox-adapter-proof-schemas';
export type {
  NotificationLocalOutboxAdapterProof,
  NotificationLocalOutboxDeliveryClaimState,
  NotificationLocalOutboxMinimalAlertEnvelope,
  NotificationLocalOutboxNonClaim,
  NotificationLocalOutboxRecord,
  NotificationLocalOutboxSeverity,
  NotificationLocalOutboxState,
} from './notification-local-outbox-adapter-proof-schemas';
export { NotificationLocalOutboxForbiddenDetailFragments, NotificationLocalOutboxKnownGaps };

type OutboxInput = (typeof NotificationLocalOutboxProofRows)[number];

function outboxRecord(input: OutboxInput): NotificationLocalOutboxRecord {
  const { providerPayloadPreview, ...recordInput } = input;
  return NotificationLocalOutboxRecordSchema.parse({
    ...recordInput,
    envelope: {
      alertRef: `notification-alert-${input.entryId}`,
      family: NotificationLocalOutboxProofFamily,
      device: NotificationLocalOutboxProofDevice,
      parentAction: NotificationLocalOutboxProofParentAction,
      severity: input.severity,
      reasonCode: input.reasonCode,
      providerChannel: input.providerChannel,
      evidenceRefs: [NotificationLocalOutboxProofEvidenceRef],
      policyRefs: ['notification-policy-ref-1'],
      auditRefs: [`notification-audit-${input.entryId}`],
      payloadTemplateRef: `notification-minimal-template-${input.reasonCode}`,
      providerPayloadPreview,
      sensitiveDetailMinimized: true,
      rawChildEvidenceIncluded: false,
      rawUrlOrTitleIncluded: false,
      rawMessageTextIncluded: false,
      screenshotOrReportIncluded: false,
    },
    outboxFileRef: 'local-notification-outbox-jsonl-ref',
    localDataPathRef: 'parent-owned-local-notification-outbox-data-path-ref',
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerCredentialsStored: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    sensitiveProviderMetadataStored: false,
  });
}

export const NotificationLocalOutboxAdapterProofReadModel = NotificationLocalOutboxAdapterProofSchema.parse({
  schemaVersion: 'notification-local-outbox-adapter-proof',
  contractVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'notification-local-outbox-adapter-proof',
  generatedAt: NotificationLocalOutboxProofTimestamp,
  outboxRootRef: 'parent-owned-local-notification-outbox-root',
  nonClaims: RequiredNotificationLocalOutboxNonClaims,
  providerDeliveryRuntimeClaimed: false,
  providerReceiptIngestionClaimed: false,
  providerCredentialsClaimed: false,
  cloudRoutingClaimed: false,
  parentNotificationUiClaimed: false,
  records: NotificationLocalOutboxProofRows.map((row) => outboxRecord(row)),
});

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

export const decodeNotificationLocalOutboxRecord = Schema.decodeUnknownSync(NotificationLocalOutboxRecordSchema);
export const decodeNotificationLocalOutboxAdapterProof = Schema.decodeUnknownSync(
  NotificationLocalOutboxAdapterProofSchema
);
