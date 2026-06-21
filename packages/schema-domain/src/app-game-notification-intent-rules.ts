import { AppGameChildUxCopyToken } from './app-game-child-facing-ux-rules';

export const AppGameNotificationIntentKind = {
  TimeLimitReached: 'time-limit-reached',
  ApprovalRequested: 'approval-requested',
  SuspiciousUnknown: 'suspicious-unknown',
  ManualRequired: 'manual-required',
  CapabilityUnavailable: 'capability-unavailable',
} as const;

export const AppGameNotificationIntentStatus = {
  IntentOnly: 'intent-only',
  LocalOutboxEligible: 'local-outbox-eligible',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AppGameNotificationDeliveryClaimState = {
  NotClaimed: 'not-claimed',
  LocalOutboxOnly: 'local-outbox-only',
  ManualRequired: 'manual-required',
} as const;

export const AppGameNotificationPriority = {
  Info: 'info',
  Attention: 'attention',
  Urgent: 'urgent',
} as const;

export const AppGameNotificationReasonCode = {
  TimeLimit: 'app-game-time-limit',
  ApprovalRequest: 'app-game-approval-request',
  SuspiciousUnknown: 'app-game-suspicious-unknown',
  ManualReviewRequired: 'app-game-manual-review-required',
  CapabilityUnavailable: 'app-game-capability-unavailable',
} as const;

export const AppGameNotificationAdapterDispatchState = {
  NotDispatched: 'not-dispatched',
} as const;

export const AppGameNotificationParentCopyToken = {
  TimeLimitTitle: 'appGame.notification.timeLimit.title',
  TimeLimitBody: 'appGame.notification.timeLimit.body',
  ApprovalTitle: 'appGame.notification.approval.title',
  ApprovalBody: 'appGame.notification.approval.body',
  SuspiciousUnknownTitle: 'appGame.notification.suspiciousUnknown.title',
  SuspiciousUnknownBody: 'appGame.notification.suspiciousUnknown.body',
  ManualRequiredTitle: 'appGame.notification.manualRequired.title',
  ManualRequiredBody: 'appGame.notification.manualRequired.body',
  UnavailableTitle: 'appGame.notification.unavailable.title',
  UnavailableBody: 'appGame.notification.unavailable.body',
  OpenParentReviewAction: 'appGame.notification.action.openParentReview',
  ReviewManuallyAction: 'appGame.notification.action.reviewManually',
} as const;

export const AppGameNotificationPayloadField = {
  AlertId: 'alert-id',
  FamilyDeviceScope: 'family-device-scope',
  Severity: 'severity',
  ReasonCode: 'reason-code',
  EvidenceRef: 'evidence-ref',
  PolicyRef: 'policy-ref',
  ParentActionLinkRef: 'parent-action-link-ref',
} as const;

type IntentKindValue = (typeof AppGameNotificationIntentKind)[keyof typeof AppGameNotificationIntentKind];
type IntentStatusValue = (typeof AppGameNotificationIntentStatus)[keyof typeof AppGameNotificationIntentStatus];
type DeliveryClaimValue =
  (typeof AppGameNotificationDeliveryClaimState)[keyof typeof AppGameNotificationDeliveryClaimState];
type ReasonCodeValue = (typeof AppGameNotificationReasonCode)[keyof typeof AppGameNotificationReasonCode];
type ParentCopyTokenValue =
  (typeof AppGameNotificationParentCopyToken)[keyof typeof AppGameNotificationParentCopyToken];
type PayloadFieldValue = (typeof AppGameNotificationPayloadField)[keyof typeof AppGameNotificationPayloadField];
type AdapterDispatchStateValue =
  (typeof AppGameNotificationAdapterDispatchState)[keyof typeof AppGameNotificationAdapterDispatchState];
type ChildCopyTokenValue = (typeof AppGameChildUxCopyToken)[keyof typeof AppGameChildUxCopyToken];

type AppGameNotificationIntentLike = {
  readonly intentKind: IntentKindValue;
  readonly intentStatus: IntentStatusValue;
  readonly deliveryClaimState: DeliveryClaimValue;
  readonly notificationReasonCode: ReasonCodeValue;
  readonly parentTitleToken: ParentCopyTokenValue;
  readonly parentBodyToken: ParentCopyTokenValue;
  readonly parentActionToken: ParentCopyTokenValue;
  readonly childTitleToken: ChildCopyTokenValue;
  readonly childBodyToken: ChildCopyTokenValue;
  readonly evidenceReferences: ReadonlyArray<unknown>;
  readonly policyRefs: ReadonlyArray<unknown>;
  readonly auditRefs: ReadonlyArray<unknown>;
  readonly childReasonReferences: ReadonlyArray<unknown>;
  readonly childStatusReferences: ReadonlyArray<unknown>;
  readonly approvalActionRef: unknown;
  readonly timeBudgetDecisionRef: unknown;
  readonly unknownCandidateRef: unknown;
  readonly localOutboxRecordRef: unknown;
  readonly manualProofRequirements: ReadonlyArray<unknown>;
  readonly minimalPayloadFields: ReadonlyArray<PayloadFieldValue>;
  readonly rawChildEvidenceIncluded: boolean;
  readonly rawUrlOrTitleIncluded: boolean;
  readonly rawMessageTextIncluded: boolean;
  readonly screenshotOrReportIncluded: boolean;
  readonly providerDeliveryAttempted: boolean;
  readonly providerDeliveryObserved: boolean;
  readonly providerReceiptIngested: boolean;
  readonly cloudRoutingClaimed: boolean;
  readonly parentNotificationUiClaimed: boolean;
  readonly adapterDispatchState: AdapterDispatchStateValue;
  readonly adapterActionClaimed: boolean;
  readonly providerAttemptRefs: ReadonlyArray<unknown>;
  readonly providerReceiptRefs: ReadonlyArray<unknown>;
};

const reasonCodeByKind: Record<IntentKindValue, ReasonCodeValue> = {
  [AppGameNotificationIntentKind.TimeLimitReached]: AppGameNotificationReasonCode.TimeLimit,
  [AppGameNotificationIntentKind.ApprovalRequested]: AppGameNotificationReasonCode.ApprovalRequest,
  [AppGameNotificationIntentKind.SuspiciousUnknown]: AppGameNotificationReasonCode.SuspiciousUnknown,
  [AppGameNotificationIntentKind.ManualRequired]: AppGameNotificationReasonCode.ManualReviewRequired,
  [AppGameNotificationIntentKind.CapabilityUnavailable]: AppGameNotificationReasonCode.CapabilityUnavailable,
};

const copyTokensByKind = {
  [AppGameNotificationIntentKind.TimeLimitReached]: {
    parentTitleToken: AppGameNotificationParentCopyToken.TimeLimitTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.TimeLimitBody,
    parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
    childTitleToken: AppGameChildUxCopyToken.LimitReachedTitle,
    childBodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  },
  [AppGameNotificationIntentKind.ApprovalRequested]: {
    parentTitleToken: AppGameNotificationParentCopyToken.ApprovalTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.ApprovalBody,
    parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
    childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: AppGameChildUxCopyToken.NewAppBody,
  },
  [AppGameNotificationIntentKind.SuspiciousUnknown]: {
    parentTitleToken: AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    parentActionToken: AppGameNotificationParentCopyToken.OpenParentReviewAction,
    childTitleToken: AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: AppGameChildUxCopyToken.NewAppBody,
  },
  [AppGameNotificationIntentKind.ManualRequired]: {
    parentTitleToken: AppGameNotificationParentCopyToken.ManualRequiredTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.ManualRequiredBody,
    parentActionToken: AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: AppGameChildUxCopyToken.ManualRequiredTitle,
    childBodyToken: AppGameChildUxCopyToken.ManualRequiredBody,
  },
  [AppGameNotificationIntentKind.CapabilityUnavailable]: {
    parentTitleToken: AppGameNotificationParentCopyToken.UnavailableTitle,
    parentBodyToken: AppGameNotificationParentCopyToken.UnavailableBody,
    parentActionToken: AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: AppGameChildUxCopyToken.UnavailableTitle,
    childBodyToken: AppGameChildUxCopyToken.UnavailableBody,
  },
} satisfies Record<
  IntentKindValue,
  {
    readonly parentTitleToken: ParentCopyTokenValue;
    readonly parentBodyToken: ParentCopyTokenValue;
    readonly parentActionToken: ParentCopyTokenValue;
    readonly childTitleToken: ChildCopyTokenValue;
    readonly childBodyToken: ChildCopyTokenValue;
  }
>;

const RequiredMinimalPayloadFields = Object.values(AppGameNotificationPayloadField);

export function appGameNotificationIntentReasonMatchesKind(intent: AppGameNotificationIntentLike): boolean {
  return intent.notificationReasonCode === reasonCodeByKind[intent.intentKind];
}

export function appGameNotificationIntentCopyMatchesKind(intent: AppGameNotificationIntentLike): boolean {
  const expected = copyTokensByKind[intent.intentKind];
  return (
    intent.parentTitleToken === expected.parentTitleToken &&
    intent.parentBodyToken === expected.parentBodyToken &&
    intent.parentActionToken === expected.parentActionToken &&
    intent.childTitleToken === expected.childTitleToken &&
    intent.childBodyToken === expected.childBodyToken
  );
}

export function appGameNotificationIntentHasAuditAndEvidence(intent: AppGameNotificationIntentLike): boolean {
  return intent.evidenceReferences.length > 0 && intent.policyRefs.length > 0 && intent.auditRefs.length > 0;
}

export function appGameNotificationIntentKindRefsAreCoherent(intent: AppGameNotificationIntentLike): boolean {
  switch (intent.intentKind) {
    case AppGameNotificationIntentKind.TimeLimitReached:
      return intent.timeBudgetDecisionRef !== null && intent.childStatusReferences.length > 0;
    case AppGameNotificationIntentKind.ApprovalRequested:
      return (
        intent.approvalActionRef !== null &&
        intent.childReasonReferences.length > 0 &&
        intent.childStatusReferences.length > 0
      );
    case AppGameNotificationIntentKind.SuspiciousUnknown:
      return intent.unknownCandidateRef !== null;
    case AppGameNotificationIntentKind.ManualRequired:
      return (
        intent.intentStatus === AppGameNotificationIntentStatus.ManualRequired &&
        intent.manualProofRequirements.length > 0
      );
    case AppGameNotificationIntentKind.CapabilityUnavailable:
      return (
        intent.intentStatus === AppGameNotificationIntentStatus.Unavailable && intent.manualProofRequirements.length > 0
      );
  }
}

export function appGameNotificationIntentStatusIsHonest(intent: AppGameNotificationIntentLike): boolean {
  if (intent.intentStatus === AppGameNotificationIntentStatus.LocalOutboxEligible) {
    return (
      intent.deliveryClaimState === AppGameNotificationDeliveryClaimState.LocalOutboxOnly &&
      intent.localOutboxRecordRef !== null
    );
  }

  if (
    intent.intentStatus === AppGameNotificationIntentStatus.ManualRequired ||
    intent.intentStatus === AppGameNotificationIntentStatus.Unavailable
  ) {
    return (
      intent.deliveryClaimState === AppGameNotificationDeliveryClaimState.ManualRequired &&
      intent.localOutboxRecordRef === null &&
      intent.manualProofRequirements.length > 0
    );
  }

  return (
    intent.deliveryClaimState === AppGameNotificationDeliveryClaimState.NotClaimed &&
    intent.localOutboxRecordRef === null
  );
}

export function appGameNotificationIntentPayloadIsMinimal(intent: AppGameNotificationIntentLike): boolean {
  const payloadFields = new Set(intent.minimalPayloadFields);
  return (
    RequiredMinimalPayloadFields.every((field) => payloadFields.has(field)) &&
    !intent.rawChildEvidenceIncluded &&
    !intent.rawUrlOrTitleIncluded &&
    !intent.rawMessageTextIncluded &&
    !intent.screenshotOrReportIncluded
  );
}

export function appGameNotificationIntentHasNoRuntimeClaims(intent: AppGameNotificationIntentLike): boolean {
  return (
    intent.adapterDispatchState === AppGameNotificationAdapterDispatchState.NotDispatched &&
    !intent.providerDeliveryAttempted &&
    !intent.providerDeliveryObserved &&
    !intent.providerReceiptIngested &&
    !intent.cloudRoutingClaimed &&
    !intent.parentNotificationUiClaimed &&
    !intent.adapterActionClaimed &&
    intent.providerAttemptRefs.length === 0 &&
    intent.providerReceiptRefs.length === 0
  );
}
