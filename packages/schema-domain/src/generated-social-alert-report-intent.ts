/* generated from crates/browser-core/src/social_schema_generated_alert_report.rs */

import { type Infer, Schema, withParser } from './effect';
import {
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './family-references';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  SocialAuditExplanationEventIdSchema,
  SocialAuditExplanationSnapshotIdSchema,
} from './social-audit-explanation-read-model-values';
import { SocialDashboardPanelIdSchema, SocialDashboardPanelSeveritySchema } from './social-dashboard-ux-values';
import {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportAdapterDispatchStateSchema,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportDeliveryClaimStateSchema,
  SocialAlertReportIntentIdSchema,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentKindSchema,
  type SocialAlertReportIntentKindValue,
  SocialAlertReportIntentStatus,
  SocialAlertReportIntentStatusSchema,
  SocialAlertReportParentCopyToken,
  SocialAlertReportParentCopyTokenSchema,
  type SocialAlertReportParentCopyTokenValue,
  SocialAlertReportPayloadField,
  SocialAlertReportPayloadFieldSchema,
  type SocialAlertReportPayloadFieldValue,
  SocialAlertReportPrioritySchema,
  SocialAlertReportReasonCode,
  SocialAlertReportReasonCodeSchema,
  type SocialAlertReportReasonCodeValue,
  SocialAlertReportReferenceSchema,
} from './social-alert-report-intent-values';
import { V3NotificationProviderChannelSchema } from './notification-v3-provider-retry';

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

const SocialAlertReportIntentBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  alertReportIntentId: SocialAlertReportIntentIdSchema,
  intentKind: SocialAlertReportIntentKindSchema,
  intentStatus: SocialAlertReportIntentStatusSchema,
  priority: SocialAlertReportPrioritySchema,
  severity: SocialDashboardPanelSeveritySchema,
  device: ParentDeviceReferenceSchema,
  notificationReasonCode: SocialAlertReportReasonCodeSchema,
  providerChannelPreference: V3NotificationProviderChannelSchema,
  parentTitleToken: SocialAlertReportParentCopyTokenSchema,
  parentBodyToken: SocialAlertReportParentCopyTokenSchema,
  parentActionToken: SocialAlertReportParentCopyTokenSchema,
  dashboardPanelRefs: SocialAlertReportDashboardPanelRefsSchema,
  explanationSnapshotRef: SocialAuditExplanationSnapshotIdSchema,
  explanationEventRefs: SocialAlertReportExplanationEventRefsSchema,
  evidenceReferences: SocialAlertReportEvidenceReferencesSchema,
  policyRefs: SocialAlertReportRefsSchema,
  auditRefs: SocialAlertReportRefsSchema,
  parentReportRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  parentActionRef: Schema.Union(ParentActionReferenceSchema, Schema.Null),
  localOutboxRecordRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  providerAttemptRefs: Schema.Array(SocialAlertReportReferenceSchema),
  providerReceiptRefs: Schema.Array(SocialAlertReportReferenceSchema),
  manualProofRequirements: Schema.Array(SocialAlertReportReferenceSchema),
  minimalPayloadFields: Schema.Array(SocialAlertReportPayloadFieldSchema),
  deliveryClaimState: SocialAlertReportDeliveryClaimStateSchema,
  rawAccountDataIncluded: Schema.Boolean,
  rawVideoContentIncluded: Schema.Boolean,
  rawMessageContentIncluded: Schema.Boolean,
  screenshotIncluded: Schema.Boolean,
  providerDeliveryAttempted: Schema.Boolean,
  providerDeliveryObserved: Schema.Boolean,
  providerReceiptIngested: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
  reportDeliveryClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  adapterDispatchState: SocialAlertReportAdapterDispatchStateSchema,
  adapterActionClaimed: Schema.Boolean,
  createdAt: ParentTimestampSchema,
});

type SocialAlertReportIntentCandidate = Infer<typeof SocialAlertReportIntentBaseSchema>;

export const SocialAlertReportIntentSchema = withParser(
  SocialAlertReportIntentBaseSchema.pipe(
    Schema.filter(
      (intent) =>
        socialAlertReportReasonMatchesKind(intent) || 'Expected social alert/report reason code to match intent kind'
    )
  )
    .pipe(
      Schema.filter(
        (intent) =>
          socialAlertReportCopyMatchesKind(intent) || 'Expected social alert/report copy tokens to match intent kind'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          socialAlertReportRefsAreCoherent(intent) ||
          'Expected social alert/report intents to cite required dashboard explanation evidence policy and audit refs'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          socialAlertReportStatusIsHonest(intent) ||
          'Expected social alert/report delivery status to match local outbox manual or unavailable claims'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          socialAlertReportPayloadIsMinimal(intent) ||
          'Expected social alert/report payloads to carry minimal refs and exclude raw child/social details'
      )
    )
    .pipe(
      Schema.filter(
        (intent) =>
          socialAlertReportHasNoRuntimeClaims(intent) ||
          'Expected social alert/report intents to avoid provider delivery UI final-policy and enforcement claims'
      )
    )
);

export type SocialAlertReportIntent = Infer<typeof SocialAlertReportIntentSchema>;

export {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
};

export const decodeSocialAlertReportIntent = (input: unknown) => SocialAlertReportIntentSchema.parse(input);

const reasonCodeByKind: Record<SocialAlertReportIntentKindValue, SocialAlertReportReasonCodeValue> = {
  [SocialAlertReportIntentKind.HighRiskSignal]: SocialAlertReportReasonCode.HighRiskSignal,
  [SocialAlertReportIntentKind.AccountApprovalNeeded]: SocialAlertReportReasonCode.AccountApproval,
  [SocialAlertReportIntentKind.FeedVideoGate]: SocialAlertReportReasonCode.FeedVideoGate,
  [SocialAlertReportIntentKind.WeeklySummary]: SocialAlertReportReasonCode.WeeklySummary,
  [SocialAlertReportIntentKind.ManualRequired]: SocialAlertReportReasonCode.ManualRequired,
  [SocialAlertReportIntentKind.CapabilityUnavailable]: SocialAlertReportReasonCode.CapabilityUnavailable,
};

const copyTokensByKind = {
  [SocialAlertReportIntentKind.HighRiskSignal]: {
    parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
    parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  },
  [SocialAlertReportIntentKind.AccountApprovalNeeded]: {
    parentTitleToken: SocialAlertReportParentCopyToken.ApprovalTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ApprovalBody,
    parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  },
  [SocialAlertReportIntentKind.FeedVideoGate]: {
    parentTitleToken: SocialAlertReportParentCopyToken.FeedVideoGateTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.FeedVideoGateBody,
    parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  },
  [SocialAlertReportIntentKind.WeeklySummary]: {
    parentTitleToken: SocialAlertReportParentCopyToken.WeeklySummaryTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.WeeklySummaryBody,
    parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  },
  [SocialAlertReportIntentKind.ManualRequired]: {
    parentTitleToken: SocialAlertReportParentCopyToken.ManualRequiredTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ManualRequiredBody,
    parentActionToken: SocialAlertReportParentCopyToken.ReviewManuallyAction,
  },
  [SocialAlertReportIntentKind.CapabilityUnavailable]: {
    parentTitleToken: SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.UnavailableBody,
    parentActionToken: SocialAlertReportParentCopyToken.ReviewManuallyAction,
  },
} satisfies Record<
  SocialAlertReportIntentKindValue,
  {
    readonly parentTitleToken: SocialAlertReportParentCopyTokenValue;
    readonly parentBodyToken: SocialAlertReportParentCopyTokenValue;
    readonly parentActionToken: SocialAlertReportParentCopyTokenValue;
  }
>;

const RequiredMinimalPayloadFields = Object.values(SocialAlertReportPayloadField);

function socialAlertReportReasonMatchesKind(intent: SocialAlertReportIntentCandidate): boolean {
  const intentKind = intent.intentKind as SocialAlertReportIntentKindValue;
  return intent.notificationReasonCode === reasonCodeByKind[intentKind];
}

function socialAlertReportCopyMatchesKind(intent: SocialAlertReportIntentCandidate): boolean {
  const expected = copyTokensByKind[intent.intentKind as SocialAlertReportIntentKindValue];
  return (
    intent.parentTitleToken === expected.parentTitleToken &&
    intent.parentBodyToken === expected.parentBodyToken &&
    intent.parentActionToken === expected.parentActionToken
  );
}

function socialAlertReportRefsAreCoherent(intent: SocialAlertReportIntentCandidate): boolean {
  if (
    intent.dashboardPanelRefs.length === 0 ||
    intent.explanationEventRefs.length === 0 ||
    intent.evidenceReferences.length === 0 ||
    intent.policyRefs.length === 0 ||
    intent.auditRefs.length === 0
  ) {
    return false;
  }
  if (intent.intentKind === SocialAlertReportIntentKind.AccountApprovalNeeded) {
    return intent.parentActionRef !== null;
  }
  if (intent.intentKind === SocialAlertReportIntentKind.WeeklySummary) {
    return intent.parentReportRef !== null;
  }
  if (
    intent.intentKind === SocialAlertReportIntentKind.ManualRequired ||
    intent.intentKind === SocialAlertReportIntentKind.CapabilityUnavailable
  ) {
    return intent.manualProofRequirements.length > 0;
  }
  return true;
}

function socialAlertReportStatusIsHonest(intent: SocialAlertReportIntentCandidate): boolean {
  if (intent.intentStatus === SocialAlertReportIntentStatus.LocalOutboxEligible) {
    return (
      intent.deliveryClaimState === SocialAlertReportDeliveryClaimState.LocalOutboxOnly &&
      intent.localOutboxRecordRef !== null
    );
  }
  if (
    intent.intentStatus === SocialAlertReportIntentStatus.ManualRequired ||
    intent.intentStatus === SocialAlertReportIntentStatus.Unavailable
  ) {
    return (
      intent.deliveryClaimState === SocialAlertReportDeliveryClaimState.ManualRequired &&
      intent.localOutboxRecordRef === null &&
      intent.manualProofRequirements.length > 0
    );
  }
  return (
    intent.deliveryClaimState === SocialAlertReportDeliveryClaimState.NotClaimed && intent.localOutboxRecordRef === null
  );
}

function socialAlertReportPayloadIsMinimal(intent: SocialAlertReportIntentCandidate): boolean {
  const payloadFields = new Set<SocialAlertReportPayloadFieldValue>(intent.minimalPayloadFields);
  return (
    RequiredMinimalPayloadFields.every((field) => payloadFields.has(field)) &&
    !intent.rawAccountDataIncluded &&
    !intent.rawVideoContentIncluded &&
    !intent.rawMessageContentIncluded &&
    !intent.screenshotIncluded
  );
}

function socialAlertReportHasNoRuntimeClaims(intent: SocialAlertReportIntentCandidate): boolean {
  return (
    intent.adapterDispatchState === SocialAlertReportAdapterDispatchState.NotDispatched &&
    !intent.providerDeliveryAttempted &&
    !intent.providerDeliveryObserved &&
    !intent.providerReceiptIngested &&
    !intent.cloudRoutingClaimed &&
    !intent.parentNotificationUiClaimed &&
    !intent.reportDeliveryClaimed &&
    !intent.finalPolicyDecisionClaimed &&
    !intent.enforcementClaimed &&
    !intent.adapterActionClaimed &&
    intent.providerAttemptRefs.length === 0 &&
    intent.providerReceiptRefs.length === 0
  );
}
