/* generated support for crates/browser-core/src/social_alert_report_local_outbox_bridge.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialAuditExplanationEventIdSchema,
  SocialAuditExplanationSnapshotIdSchema,
} from '@ocentra-parent/schema-domain/social-audit-explanation-read-model-values';
import { SocialDashboardPanelIdSchema } from '@ocentra-parent/schema-domain/social-dashboard-ux-values';
import { V3NotificationProviderChannelSchema } from '@ocentra-parent/schema-domain/notification-v3-provider-retry';

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

export type SocialAlertReportIntent = Infer<typeof SocialAlertReportIntentSchema>;
