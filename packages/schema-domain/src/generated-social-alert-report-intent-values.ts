/* generated from crates/browser-core/src/social_schema_generated_values.rs */
import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';

export const SocialAlertReportIntentKind = {
  HighRiskSignal: 'high-risk-signal',
  AccountApprovalNeeded: 'account-approval-needed',
  FeedVideoGate: 'feed-video-gate',
  WeeklySummary: 'weekly-summary',
  ManualRequired: 'manual-required',
  CapabilityUnavailable: 'capability-unavailable',
} as const;

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

export const SocialAlertReportPriority = {
  Info: 'info',
  Attention: 'attention',
  Urgent: 'urgent',
} as const;

export const SocialAlertReportReasonCode = {
  HighRiskSignal: 'social-high-risk-signal',
  AccountApproval: 'social-account-approval-needed',
  FeedVideoGate: 'social-feed-video-gate',
  WeeklySummary: 'social-weekly-summary',
  ManualRequired: 'social-manual-review-required',
  CapabilityUnavailable: 'social-capability-unavailable',
} as const;

export const SocialAlertReportParentCopyToken = {
  HighRiskTitle: 'social.alert.highRisk.title',
  HighRiskBody: 'social.alert.highRisk.body',
  ApprovalTitle: 'social.alert.accountApproval.title',
  ApprovalBody: 'social.alert.accountApproval.body',
  FeedVideoGateTitle: 'social.alert.feedVideoGate.title',
  FeedVideoGateBody: 'social.alert.feedVideoGate.body',
  WeeklySummaryTitle: 'social.report.weeklySummary.title',
  WeeklySummaryBody: 'social.report.weeklySummary.body',
  ManualRequiredTitle: 'social.alert.manualRequired.title',
  ManualRequiredBody: 'social.alert.manualRequired.body',
  UnavailableTitle: 'social.alert.unavailable.title',
  UnavailableBody: 'social.alert.unavailable.body',
  OpenParentReviewAction: 'social.alert.action.openParentReview',
  ReviewManuallyAction: 'social.alert.action.reviewManually',
} as const;

export const SocialAlertReportPayloadField = {
  AlertId: 'alert-id',
  FamilyDeviceScope: 'family-device-scope',
  Severity: 'severity',
  ReasonCode: 'reason-code',
  EvidenceRef: 'evidence-ref',
  PolicyRef: 'policy-ref',
  ExplanationRef: 'explanation-ref',
  ParentActionLinkRef: 'parent-action-link-ref',
} as const;

export const SocialAlertReportAdapterDispatchState = {
  NotDispatched: 'not-dispatched',
} as const;

export const SocialAlertReportIntentIdSchema = withParser(brandedNonEmptyStringSchema('SocialAlertReportIntentId'));
export const SocialAlertReportReferenceSchema = withParser(brandedNonEmptyStringSchema('SocialAlertReportReference'));
export const SocialAlertReportIntentKindSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportIntentKind))
);
export const SocialAlertReportIntentStatusSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportIntentStatus))
);
export const SocialAlertReportDeliveryClaimStateSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportDeliveryClaimState))
);
export const SocialAlertReportPrioritySchema = withParser(Schema.Literal(...Object.values(SocialAlertReportPriority)));
export const SocialAlertReportReasonCodeSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportReasonCode))
);
export const SocialAlertReportParentCopyTokenSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportParentCopyToken))
);
export const SocialAlertReportPayloadFieldSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportPayloadField))
);
export const SocialAlertReportAdapterDispatchStateSchema = withParser(
  Schema.Literal(SocialAlertReportAdapterDispatchState.NotDispatched)
);

export type SocialAlertReportIntentKindValue = Infer<typeof SocialAlertReportIntentKindSchema>;
export type SocialAlertReportIntentStatusValue = Infer<typeof SocialAlertReportIntentStatusSchema>;
export type SocialAlertReportDeliveryClaimStateValue = Infer<typeof SocialAlertReportDeliveryClaimStateSchema>;
export type SocialAlertReportReasonCodeValue = Infer<typeof SocialAlertReportReasonCodeSchema>;
export type SocialAlertReportParentCopyTokenValue = Infer<typeof SocialAlertReportParentCopyTokenSchema>;
export type SocialAlertReportPayloadFieldValue = Infer<typeof SocialAlertReportPayloadFieldSchema>;
export type SocialAlertReportAdapterDispatchStateValue = Infer<typeof SocialAlertReportAdapterDispatchStateSchema>;
