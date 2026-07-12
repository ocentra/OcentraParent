/* generated support for crates/browser-core/src/social_alert_report_scheduler_bridge.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleReasonCodeSchema,
} from '@ocentra-parent/schema-domain/notification-v3-provider-retry';

const RequiredNotificationLocalOutboxStates = [
  'queued-local',
  'deferred-quiet-hours',
  'retry-scheduled',
  'dead-lettered',
  'receipt-required',
  'manual-required',
] as const;

const RequiredNotificationLocalOutboxSchedulerStates = [
  'due-local',
  'held-quiet-hours',
  'retry-window-scheduled',
  'dead-letter-review',
  'receipt-required',
  'manual-required',
] as const;

export const RequiredNotificationLocalOutboxSchedulerNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-production-durable-outbox-storage',
  'no-sensitive-detail-storage',
] as const;

const NotificationSchedulerAttemptCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());
const NotificationLocalOutboxStateSchema = withParser(Schema.Literal(...RequiredNotificationLocalOutboxStates));
const NotificationLocalOutboxSchedulerStateSchema = withParser(
  Schema.Literal(...RequiredNotificationLocalOutboxSchedulerStates)
);
const NotificationLocalOutboxSeveritySchema = withParser(Schema.Literal('info', 'attention', 'urgent'));
const NotificationLocalOutboxReferenceSchema = withParser(
  brandedNonEmptyStringSchema('NotificationLocalOutboxReference')
);
const NotificationLocalOutboxEntryIdSchema = withParser(brandedNonEmptyStringSchema('NotificationLocalOutboxEntryId'));
const NotificationLocalOutboxSchedulerEntryIdSchema = withParser(
  brandedNonEmptyStringSchema('NotificationLocalOutboxSchedulerEntryId')
);
const NotificationLocalOutboxPayloadPreviewSchema = withParser(
  brandedNonEmptyStringSchema('NotificationLocalOutboxPayloadPreview')
);

const NotificationLocalOutboxQuietHoursWindowSchema = Schema.Struct({
  quietHoursWindowRef: NotificationLocalOutboxReferenceSchema,
  startsAt: ParentTimestampSchema,
  endsAt: ParentTimestampSchema,
  holdReasonRef: NotificationLocalOutboxReferenceSchema,
});

const NotificationLocalOutboxRetryWindowSchema = Schema.Struct({
  retryWindowRef: NotificationLocalOutboxReferenceSchema,
  opensAt: ParentTimestampSchema,
  closesAt: ParentTimestampSchema,
  attemptNumber: NotificationSchedulerAttemptCountSchema,
  maxAttempts: NotificationSchedulerAttemptCountSchema,
});

export const NotificationLocalOutboxSchedulerRecordSchema = withParser(
  Schema.Struct({
    schedulerEntryId: NotificationLocalOutboxSchedulerEntryIdSchema,
    sourceEntryId: NotificationLocalOutboxEntryIdSchema,
    sourceState: NotificationLocalOutboxStateSchema,
    schedulerState: NotificationLocalOutboxSchedulerStateSchema,
    reasonCode: V3NotificationRuleReasonCodeSchema,
    providerChannel: V3NotificationProviderChannelSchema,
    severity: NotificationLocalOutboxSeveritySchema,
    schedulerDecisionRef: NotificationLocalOutboxReferenceSchema,
    schedulerArtifactRef: NotificationLocalOutboxReferenceSchema,
    sourceOutboxFileRef: NotificationLocalOutboxReferenceSchema,
    localDataPathRef: NotificationLocalOutboxReferenceSchema,
    schedulerNowAt: ParentTimestampSchema,
    nextAttemptAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    quietHoursWindow: Schema.Union(NotificationLocalOutboxQuietHoursWindowSchema, Schema.Null),
    retryWindow: Schema.Union(NotificationLocalOutboxRetryWindowSchema, Schema.Null),
    deadLetterReviewRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
    providerReceiptRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
    manualProofRequirements: Schema.Array(NotificationLocalOutboxReferenceSchema),
    manualActionRequired: Schema.Boolean,
    parentOwnedArtifactWritten: Schema.Boolean,
    rawChildEvidenceIncluded: Schema.Boolean,
    rawUrlOrTitleIncluded: Schema.Boolean,
    rawMessageTextIncluded: Schema.Boolean,
    screenshotOrReportIncluded: Schema.Boolean,
    providerDeliveryAttempted: Schema.Boolean,
    providerDeliveryObserved: Schema.Boolean,
    providerReceiptIngested: Schema.Boolean,
    providerCredentialsStored: Schema.Boolean,
    cloudRoutingClaimed: Schema.Boolean,
    parentNotificationUiClaimed: Schema.Boolean,
    productionDurableOutboxStorageClaimed: Schema.Boolean,
    sensitiveProviderMetadataStored: Schema.Boolean,
    schedulerPayloadPreview: NotificationLocalOutboxPayloadPreviewSchema,
  })
);

export type NotificationLocalOutboxSchedulerRecord = Infer<typeof NotificationLocalOutboxSchedulerRecordSchema>;
