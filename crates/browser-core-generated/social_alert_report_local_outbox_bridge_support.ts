/* generated support for crates/browser-core/src/social_alert_report_local_outbox_bridge.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleReasonCodeSchema,
} from '@ocentra-parent/schema-domain/notification-v3-provider-retry';
import {
  notificationEnvelopeIsSafe,
  notificationOutboxRecordIsSafe,
} from './social_alert_report_local_outbox_bridge_record_honesty';

export const SocialAlertReportIntentStatus = {
  IntentOnly: 'intent-only',
  LocalOutboxEligible: 'local-outbox-eligible',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialAlertReportDeliveryClaimState = {
  NotClaimed: 'not-claimed',
  LocalOutboxOnly: 'local-outbox-only',
  ManualRequired: 'manual-required',
} as const;

export const SocialAuditExplanationEventIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialAuditExplanationEventId')
);
export const SocialAuditExplanationSnapshotIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialAuditExplanationSnapshotId')
);
export const SocialDashboardPanelIdSchema = withParser(brandedNonEmptyStringSchema('SocialDashboardPanelId'));

const RequiredNotificationLocalOutboxStates = [
  'queued-local',
  'deferred-quiet-hours',
  'retry-scheduled',
  'dead-lettered',
  'receipt-required',
  'manual-required',
] as const;

export const SocialAlertReportPrioritySchema = withParser(Schema.Literal('info', 'attention', 'urgent'));
export const SocialAlertReportReasonCodeSchema = withParser(
  Schema.Literal(
    'social-high-risk-signal',
    'social-account-approval-needed',
    'social-feed-video-gate',
    'social-weekly-summary',
    'social-manual-review-required',
    'social-capability-unavailable'
  )
);
export const SocialAlertReportParentCopyTokenSchema = withParser(
  Schema.Literal(
    'social.alert.highRisk.title',
    'social.alert.highRisk.body',
    'social.alert.accountApproval.title',
    'social.alert.accountApproval.body',
    'social.alert.feedVideoGate.title',
    'social.alert.feedVideoGate.body',
    'social.report.weeklySummary.title',
    'social.report.weeklySummary.body',
    'social.alert.manualRequired.title',
    'social.alert.manualRequired.body',
    'social.alert.unavailable.title',
    'social.alert.unavailable.body',
    'social.alert.action.openParentReview',
    'social.alert.action.reviewManually'
  )
);
export const SocialAlertReportIntentIdSchema = withParser(brandedNonEmptyStringSchema('SocialAlertReportIntentId'));
export const SocialAlertReportReferenceSchema = withParser(brandedNonEmptyStringSchema('SocialAlertReportReference'));
export const SocialAlertReportIntentStatusSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportIntentStatus))
);
export const SocialAlertReportDeliveryClaimStateSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportDeliveryClaimState))
);
export const NotificationLocalOutboxStateSchema = withParser(Schema.Literal(...RequiredNotificationLocalOutboxStates));
export const NotificationLocalOutboxDeliveryClaimStateSchema = withParser(
  Schema.Literal('local-outbox-only', 'provider-receipt-required', 'manual-required')
);
export const NotificationLocalOutboxEntryIdSchema = withParser(
  brandedNonEmptyStringSchema('NotificationLocalOutboxEntryId')
);
export const NotificationLocalOutboxReferenceSchema = withParser(
  brandedNonEmptyStringSchema('NotificationLocalOutboxReference')
);
export const NotificationLocalOutboxPayloadPreviewSchema = withParser(
  brandedNonEmptyStringSchema('NotificationLocalOutboxPayloadPreview')
);

const SocialAlertReportRefsSchema = Schema.Array(SocialAlertReportReferenceSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social alert/report refs')
);
const SocialAlertReportEvidenceReferencesSchema = Schema.Array(ParentEvidenceReferenceSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social alert/report evidence refs')
);
const SocialAlertReportExplanationEventRefsSchema = Schema.Array(SocialAuditExplanationEventIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social alert/report explanation refs')
);
const SocialAlertReportDashboardPanelRefsSchema = Schema.Array(SocialDashboardPanelIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social alert/report dashboard refs')
);
const NotificationOutboxRetryCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());
const NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema = Schema.Struct({
  alertRef: NotificationLocalOutboxReferenceSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  parentAction: ParentActionReferenceSchema,
  severity: SocialAlertReportPrioritySchema,
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
const NotificationLocalOutboxRecordBaseSchema = Schema.Struct({
  entryId: NotificationLocalOutboxEntryIdSchema,
  state: NotificationLocalOutboxStateSchema,
  envelope: Schema.suspend(() => NotificationLocalOutboxMinimalAlertEnvelopeSchema),
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

export const SocialAlertReportIntentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    alertReportIntentId: SocialAlertReportIntentIdSchema,
    intentStatus: SocialAlertReportIntentStatusSchema,
    priority: SocialAlertReportPrioritySchema,
    device: ParentDeviceReferenceSchema,
    notificationReasonCode: SocialAlertReportReasonCodeSchema,
    providerChannelPreference: V3NotificationProviderChannelSchema,
    parentBodyToken: SocialAlertReportParentCopyTokenSchema,
    dashboardPanelRefs: SocialAlertReportDashboardPanelRefsSchema,
    explanationSnapshotRef: SocialAuditExplanationSnapshotIdSchema,
    explanationEventRefs: SocialAlertReportExplanationEventRefsSchema,
    evidenceReferences: SocialAlertReportEvidenceReferencesSchema,
    policyRefs: SocialAlertReportRefsSchema,
    auditRefs: SocialAlertReportRefsSchema,
    localOutboxRecordRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
    manualProofRequirements: Schema.Array(SocialAlertReportReferenceSchema),
    deliveryClaimState: SocialAlertReportDeliveryClaimStateSchema,
  })
);
export const NotificationLocalOutboxMinimalAlertEnvelopeSchema = withParser(
  NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema.pipe(
    Schema.filter(
      (envelope) =>
        notificationEnvelopeIsSafe(envelope) ||
        'Expected local notification outbox envelopes to carry minimal refs only, without raw child evidence, URLs, titles, message text, screenshots, reports, or forbidden payload fragments'
    )
  )
);
export const NotificationLocalOutboxRecordSchema = withParser(
  NotificationLocalOutboxRecordBaseSchema.pipe(
    Schema.filter(
      (record) =>
        notificationOutboxRecordIsSafe(record) ||
        'Expected local outbox records to be filesystem/local-data-path refs only, with coherent defer/retry/dead-letter/receipt/manual states and no provider delivery or sensitive metadata claims'
    )
  )
);

export type SocialAlertReportIntent = Infer<typeof SocialAlertReportIntentSchema>;
export type NotificationLocalOutboxRecord = Infer<typeof NotificationLocalOutboxRecordSchema>;
