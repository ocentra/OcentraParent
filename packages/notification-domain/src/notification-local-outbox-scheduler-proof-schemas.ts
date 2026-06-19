import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  NotificationLocalOutboxEntryIdSchema,
  NotificationLocalOutboxPayloadPreviewSchema,
  NotificationLocalOutboxReferenceSchema,
  NotificationLocalOutboxSeveritySchema,
  NotificationLocalOutboxStateSchema,
} from './notification-local-outbox-adapter-proof-schemas';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  RequiredNotificationLocalOutboxSchedulerNonClaims,
  RequiredNotificationLocalOutboxSchedulerStates,
} from '@ocentra-parent/notification-domain/notification-local-outbox-scheduler-proof-values';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleReasonCodeSchema,
} from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';
import {
  notificationOutboxSchedulerProofIsSafe,
  notificationOutboxSchedulerRecordIsSafe,
} from './notification-local-outbox-scheduler-proof-guards';
const NotificationSchedulerAttemptCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const NotificationLocalOutboxSchedulerProofSchemaVersionSchema = withParser(
  Schema.Literal('notification-local-outbox-scheduler-proof')
);
export const NotificationLocalOutboxSchedulerStateSchema = withParser(
  Schema.Literal(...RequiredNotificationLocalOutboxSchedulerStates)
);
export const NotificationLocalOutboxSchedulerNonClaimSchema = withParser(
  Schema.Literal(...RequiredNotificationLocalOutboxSchedulerNonClaims)
);
export const NotificationLocalOutboxSchedulerReadModelIdSchema = brandedNonEmptyStringSchema('NotificationLocalOutboxSchedulerReadModelId');
export const NotificationLocalOutboxSchedulerEntryIdSchema = brandedNonEmptyStringSchema('NotificationLocalOutboxSchedulerEntryId');

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

const NotificationLocalOutboxSchedulerRecordBaseSchema = Schema.Struct({
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
});

export const NotificationLocalOutboxSchedulerRecordSchema = withParser(
  NotificationLocalOutboxSchedulerRecordBaseSchema.pipe(
    Schema.filter(
      (record) =>
        notificationOutboxSchedulerRecordIsSafe(record) ||
        'Expected local notification scheduler rows to use parent-owned artifact refs, deterministic next-at/retry windows, coherent manual/receipt/dead-letter states, and no provider delivery or sensitive-detail claims'
    )
  )
);

const NotificationLocalOutboxSchedulerProofBaseSchema = Schema.Struct({
  schemaVersion: NotificationLocalOutboxSchedulerProofSchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  readModelId: NotificationLocalOutboxSchedulerReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  schedulerNowAt: ParentTimestampSchema,
  schedulerArtifactRootRef: NotificationLocalOutboxReferenceSchema,
  sourceAdapterReadModelId: NotificationLocalOutboxReferenceSchema,
  records: Schema.Array(NotificationLocalOutboxSchedulerRecordSchema),
  nonClaims: Schema.Array(NotificationLocalOutboxSchedulerNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Boolean,
  providerReceiptIngestionClaimed: Schema.Boolean,
  providerCredentialsClaimed: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
  retryExecutionRuntimeClaimed: Schema.Boolean,
  quietHoursTimerRuntimeClaimed: Schema.Boolean,
  productionDurableOutboxStorageClaimed: Schema.Boolean,
});

export const NotificationLocalOutboxSchedulerProofSchema = withParser(
  NotificationLocalOutboxSchedulerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        notificationOutboxSchedulerProofIsSafe(proof) ||
        'Expected notification local outbox scheduler proof to cover required scheduler states/channels, write only parent-owned artifacts, and keep provider delivery, retry execution, quiet-hours timer, UI, cloud, credential, and durable storage non-claims explicit'
    )
  )
);

export type NotificationOutboxSchedulerRecordCandidate = Infer<typeof NotificationLocalOutboxSchedulerRecordBaseSchema>;
export type NotificationOutboxSchedulerProofCandidate = Infer<typeof NotificationLocalOutboxSchedulerProofBaseSchema>;

export type NotificationLocalOutboxSchedulerState = Infer<typeof NotificationLocalOutboxSchedulerStateSchema>;
export type NotificationLocalOutboxSchedulerNonClaim = Infer<typeof NotificationLocalOutboxSchedulerNonClaimSchema>;
export type NotificationLocalOutboxSchedulerRecord = Infer<typeof NotificationLocalOutboxSchedulerRecordSchema>;
export type NotificationLocalOutboxSchedulerProof = Infer<typeof NotificationLocalOutboxSchedulerProofSchema>;

