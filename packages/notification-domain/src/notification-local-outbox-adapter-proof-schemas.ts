import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  NotificationLocalOutboxForbiddenDetailFragments,
  NotificationLocalOutboxProviderChannels,
  RequiredNotificationLocalOutboxNonClaims,
  RequiredNotificationLocalOutboxStates,
} from './notification-local-outbox-adapter-proof-values';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleReasonCodeSchema,
} from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';
const NotificationOutboxRetryCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());
export const NotificationLocalOutboxAdapterProofSchemaVersionSchema = withParser(
  Schema.Literal('notification-local-outbox-adapter-proof')
);
export const NotificationLocalOutboxStateSchema = withParser(Schema.Literal(...RequiredNotificationLocalOutboxStates));
export const NotificationLocalOutboxNonClaimSchema = withParser(
  Schema.Literal(...RequiredNotificationLocalOutboxNonClaims)
);
export const NotificationLocalOutboxSeveritySchema = withParser(Schema.Literal('info', 'attention', 'urgent'));
export const NotificationLocalOutboxDeliveryClaimStateSchema = withParser(
  Schema.Literal('local-outbox-only', 'provider-receipt-required', 'manual-required')
);
export const NotificationLocalOutboxReadModelIdSchema = brandedNonEmptyStringSchema('NotificationLocalOutboxReadModelId');
export const NotificationLocalOutboxEntryIdSchema = brandedNonEmptyStringSchema('NotificationLocalOutboxEntryId');
export const NotificationLocalOutboxReferenceSchema = brandedNonEmptyStringSchema('NotificationLocalOutboxReference');
export const NotificationLocalOutboxPayloadPreviewSchema = brandedNonEmptyStringSchema('NotificationLocalOutboxPayloadPreview');

const NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema = Schema.Struct({
  alertRef: NotificationLocalOutboxReferenceSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  parentAction: ParentActionReferenceSchema,
  severity: NotificationLocalOutboxSeveritySchema,
  reasonCode: V3NotificationRuleReasonCodeSchema,
  providerChannel: V3NotificationProviderChannelSchema,
  evidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  policyRefs: Schema.Array(NotificationLocalOutboxReferenceSchema),
  auditRefs: Schema.Array(NotificationLocalOutboxReferenceSchema),
  payloadTemplateRef: NotificationLocalOutboxReferenceSchema,
  providerPayloadPreview: NotificationLocalOutboxPayloadPreviewSchema,
  sensitiveDetailMinimized: Schema.Boolean,
  rawChildEvidenceIncluded: Schema.Boolean,
  rawUrlOrTitleIncluded: Schema.Boolean,
  rawMessageTextIncluded: Schema.Boolean,
  screenshotOrReportIncluded: Schema.Boolean,
});

export const NotificationLocalOutboxMinimalAlertEnvelopeSchema = withParser(
  NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema.pipe(
    Schema.filter(
      (envelope) =>
        notificationEnvelopeIsSafe(envelope) ||
        'Expected local notification outbox envelopes to carry minimal refs only, without raw child evidence, URLs, titles, message text, screenshots, reports, or forbidden payload fragments'
    )
  )
);

const NotificationLocalOutboxRecordBaseSchema = Schema.Struct({
  entryId: NotificationLocalOutboxEntryIdSchema,
  state: NotificationLocalOutboxStateSchema,
  envelope: NotificationLocalOutboxMinimalAlertEnvelopeSchema,
  outboxFileRef: NotificationLocalOutboxReferenceSchema,
  localDataPathRef: NotificationLocalOutboxReferenceSchema,
  deliveryClaimState: NotificationLocalOutboxDeliveryClaimStateSchema,
  visibleAfterAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  retryAttemptCount: NotificationOutboxRetryCountSchema,
  quietHoursRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  retryPolicyRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  deadLetterRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  providerReceiptRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  manualProofRequirements: Schema.Array(NotificationLocalOutboxReferenceSchema),
  manualActionRequired: Schema.Boolean,
  providerDeliveryAttempted: Schema.Boolean,
  providerDeliveryObserved: Schema.Boolean,
  providerReceiptIngested: Schema.Boolean,
  providerCredentialsStored: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
  sensitiveProviderMetadataStored: Schema.Boolean,
});

export const NotificationLocalOutboxRecordSchema = withParser(
  NotificationLocalOutboxRecordBaseSchema.pipe(
    Schema.filter(
      (record) =>
        notificationOutboxRecordIsSafe(record) ||
        'Expected local outbox records to be filesystem/local-data-path refs only, with coherent defer/retry/dead-letter/receipt/manual states and no provider delivery or sensitive metadata claims'
    )
  )
);

const NotificationLocalOutboxProofBaseSchema = Schema.Struct({
  schemaVersion: NotificationLocalOutboxAdapterProofSchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  readModelId: NotificationLocalOutboxReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  outboxRootRef: NotificationLocalOutboxReferenceSchema,
  records: Schema.Array(NotificationLocalOutboxRecordSchema),
  nonClaims: Schema.Array(NotificationLocalOutboxNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Boolean,
  providerReceiptIngestionClaimed: Schema.Boolean,
  providerCredentialsClaimed: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
});

export const NotificationLocalOutboxAdapterProofSchema = withParser(
  NotificationLocalOutboxProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        notificationOutboxProofIsSafe(proof) ||
        'Expected notification local outbox proof to cover required local states/channels and keep provider delivery, receipt, credentials, cloud routing, and UI non-claims explicit'
    )
  )
);

type NotificationEnvelopeCandidate = Infer<typeof NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema>;
type NotificationOutboxRecordCandidate = Infer<typeof NotificationLocalOutboxRecordBaseSchema>;
type NotificationOutboxProofCandidate = Infer<typeof NotificationLocalOutboxProofBaseSchema>;

export type NotificationLocalOutboxState = Infer<typeof NotificationLocalOutboxStateSchema>;
export type NotificationLocalOutboxNonClaim = Infer<typeof NotificationLocalOutboxNonClaimSchema>;
export type NotificationLocalOutboxSeverity = Infer<typeof NotificationLocalOutboxSeveritySchema>;
export type NotificationLocalOutboxDeliveryClaimState = Infer<typeof NotificationLocalOutboxDeliveryClaimStateSchema>;
export type NotificationLocalOutboxMinimalAlertEnvelope = Infer<
  typeof NotificationLocalOutboxMinimalAlertEnvelopeSchema
>;
export type NotificationLocalOutboxRecord = Infer<typeof NotificationLocalOutboxRecordSchema>;
export type NotificationLocalOutboxAdapterProof = Infer<typeof NotificationLocalOutboxAdapterProofSchema>;

const ClaimFlags = [
  'providerDeliveryRuntimeClaimed',
  'providerReceiptIngestionClaimed',
  'providerCredentialsClaimed',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
] as const;
const RecordClaimFlags = [
  'providerDeliveryAttempted',
  'providerDeliveryObserved',
  'providerReceiptIngested',
  'providerCredentialsStored',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
  'sensitiveProviderMetadataStored',
] as const;

function notificationEnvelopeIsSafe(envelope: NotificationEnvelopeCandidate): boolean {
  return (
    envelope.sensitiveDetailMinimized &&
    !envelope.rawChildEvidenceIncluded &&
    !envelope.rawUrlOrTitleIncluded &&
    !envelope.rawMessageTextIncluded &&
    !envelope.screenshotOrReportIncluded &&
    envelope.evidenceRefs.length > 0 &&
    envelope.policyRefs.length > 0 &&
    envelope.auditRefs.length > 0 &&
    !textContainsForbiddenDetail(envelope.providerPayloadPreview)
  );
}

function notificationOutboxRecordIsSafe(record: NotificationOutboxRecordCandidate): boolean {
  return (
    !RecordClaimFlags.some((flag) => record[flag]) &&
    record.outboxFileRef.trim().length > 0 &&
    record.localDataPathRef.trim().length > 0 &&
    notificationOutboxStateIsCoherent(record)
  );
}

function notificationOutboxStateIsCoherent(record: NotificationOutboxRecordCandidate): boolean {
  if (record.state !== 'receipt-required' && record.providerReceiptRef !== null) {
    return false;
  }
  if (record.state === 'queued-local') {
    return record.visibleAfterAt === null && record.retryAttemptCount === 0 && !record.manualActionRequired;
  }
  if (record.state === 'deferred-quiet-hours') {
    return record.visibleAfterAt !== null && record.quietHoursRef !== null && !record.manualActionRequired;
  }
  if (record.state === 'retry-scheduled') {
    return record.retryAttemptCount > 0 && record.retryPolicyRef !== null && record.visibleAfterAt !== null;
  }
  return notificationOutboxTerminalStateIsCoherent(record);
}

function notificationOutboxTerminalStateIsCoherent(record: NotificationOutboxRecordCandidate): boolean {
  if (record.state === 'dead-lettered') {
    return record.deadLetterRef !== null && record.manualActionRequired && record.manualProofRequirements.length > 0;
  }
  if (record.state === 'receipt-required') {
    return (
      record.deliveryClaimState === 'provider-receipt-required' &&
      record.providerReceiptRef !== null &&
      record.manualActionRequired &&
      record.manualProofRequirements.length > 0
    );
  }
  return (
    record.state === 'manual-required' &&
    record.deliveryClaimState === 'manual-required' &&
    record.manualActionRequired &&
    record.manualProofRequirements.length > 0
  );
}

function notificationOutboxProofIsSafe(proof: NotificationOutboxProofCandidate): boolean {
  return (
    requiredStatesAreCovered(proof.records) &&
    requiredChannelsAreCovered(proof.records) &&
    RequiredNotificationLocalOutboxNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    ClaimFlags.every((flag) => proof[flag] === false)
  );
}

function requiredStatesAreCovered(records: ReadonlyArray<NotificationOutboxRecordCandidate>): boolean {
  return RequiredNotificationLocalOutboxStates.every((state) => records.some((record) => record.state === state));
}

function requiredChannelsAreCovered(records: ReadonlyArray<NotificationOutboxRecordCandidate>): boolean {
  return NotificationLocalOutboxProviderChannels.every((channel) =>
    records.some((record) => record.envelope.providerChannel === channel)
  );
}

function textContainsForbiddenDetail(text: string): boolean {
  const lowerText = text.toLowerCase();
  return NotificationLocalOutboxForbiddenDetailFragments.some((fragment) => lowerText.includes(fragment));
}

