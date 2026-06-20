import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './family-references';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleReasonCodeSchema,
} from './notification-v3-provider-retry';

export const RequiredNotificationLocalOutboxStates = [
  'queued-local',
  'deferred-quiet-hours',
  'retry-scheduled',
  'dead-lettered',
  'receipt-required',
  'manual-required',
] as const;
export const RequiredNotificationLocalOutboxNonClaims = [
  'no-provider-delivery',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-raw-child-evidence',
  'no-sensitive-provider-metadata',
] as const;
export const NotificationLocalOutboxForbiddenDetailFragments = [
  'http://',
  'https://',
  'screenshot-bytes',
  'raw-title-value',
  'raw-message-body',
  'sqlite-private-path',
  'oauth-secret',
  'provider-token',
  'report-body',
] as const;
export const NotificationLocalOutboxProviderChannels = ['push', 'email', 'sms', 'whatsapp', 'in-app'] as const;

const NotificationOutboxRetryCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());
export const NotificationLocalOutboxAdapterProofSchemaVersionSchema = withParser(
  Schema.Literal('notification-local-outbox-adapter-proof')
);
export const NotificationLocalOutboxStateSchema = withParser(Schema.Literal(...RequiredNotificationLocalOutboxStates));
export const NotificationLocalOutboxNonClaimSchema = withParser(Schema.Literal(...RequiredNotificationLocalOutboxNonClaims));
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
    Schema.filter((envelope) => notificationEnvelopeIsSafe(envelope) || 'Expected safe notification envelope')
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
    Schema.filter((record) => notificationOutboxRecordIsSafe(record) || 'Expected safe local outbox record')
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
    Schema.filter((proof) => notificationOutboxProofIsSafe(proof) || 'Expected safe notification local outbox proof')
  )
);

type NotificationEnvelopeCandidate = Infer<typeof NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema>;
type NotificationOutboxRecordCandidate = Infer<typeof NotificationLocalOutboxRecordBaseSchema>;
type NotificationOutboxProofCandidate = Infer<typeof NotificationLocalOutboxProofBaseSchema>;

export type NotificationLocalOutboxState = Infer<typeof NotificationLocalOutboxStateSchema>;
export type NotificationLocalOutboxNonClaim = Infer<typeof NotificationLocalOutboxNonClaimSchema>;
export type NotificationLocalOutboxSeverity = Infer<typeof NotificationLocalOutboxSeveritySchema>;
export type NotificationLocalOutboxDeliveryClaimState = Infer<typeof NotificationLocalOutboxDeliveryClaimStateSchema>;
export type NotificationLocalOutboxMinimalAlertEnvelope = Infer<typeof NotificationLocalOutboxMinimalAlertEnvelopeSchema>;
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
  if (record.state !== 'receipt-required' && record.providerReceiptRef !== null) return false;
  if (record.state === 'queued-local') return record.visibleAfterAt === null && record.retryAttemptCount === 0 && !record.manualActionRequired;
  if (record.state === 'deferred-quiet-hours') return record.visibleAfterAt !== null && record.quietHoursRef !== null && !record.manualActionRequired;
  if (record.state === 'retry-scheduled') return record.retryAttemptCount > 0 && record.retryPolicyRef !== null && record.visibleAfterAt !== null;
  if (record.state === 'dead-lettered') return record.deadLetterRef !== null && record.manualActionRequired && record.manualProofRequirements.length > 0;
  if (record.state === 'receipt-required') return record.deliveryClaimState === 'provider-receipt-required' && record.providerReceiptRef !== null && record.manualActionRequired && record.manualProofRequirements.length > 0;
  return record.state === 'manual-required' && record.deliveryClaimState === 'manual-required' && record.manualActionRequired && record.manualProofRequirements.length > 0;
}

function notificationOutboxProofIsSafe(proof: NotificationOutboxProofCandidate): boolean {
  return (
    RequiredNotificationLocalOutboxStates.every((state) => proof.records.some((record) => record.state === state)) &&
    NotificationLocalOutboxProviderChannels.every((channel) => proof.records.some((record) => record.envelope.providerChannel === channel)) &&
    RequiredNotificationLocalOutboxNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    ClaimFlags.every((flag) => proof[flag] === false)
  );
}

function textContainsForbiddenDetail(text: string): boolean {
  const lowerText = text.toLowerCase();
  return NotificationLocalOutboxForbiddenDetailFragments.some((fragment) => lowerText.includes(fragment));
}
