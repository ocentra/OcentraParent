export const AppGameChildUxSurfaceState = {
  FamilyRuleWarning: 'family-rule-warning',
  NewAppApprovalNeeded: 'new-app-approval-needed',
  TimeAlmostFinished: 'time-almost-finished',
  TimeLimitReached: 'time-limit-reached',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
  RequestSubmitted: 'request-submitted',
  RequestApproved: 'request-approved',
  RequestDenied: 'request-denied',
} as const;

export const AppGameChildUxTargetKind = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
  LauncherGameCandidate: 'launcher-game-candidate',
  UnknownApp: 'unknown-app',
  UnknownGame: 'unknown-game',
} as const;

export const AppGameChildUxCapabilityState = {
  Supported: 'supported',
  DryRunOnly: 'dry-run-only',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AppGameChildUxClaimState = {
  WarningOnly: 'warning-only',
  ApprovalNeeded: 'approval-needed',
  AlmostFinished: 'almost-finished',
  LimitReached: 'limit-reached',
  RequestSubmitted: 'request-submitted',
  RequestApproved: 'request-approved',
  RequestDenied: 'request-denied',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AppGameChildUxPrimaryAction = {
  Dismiss: 'dismiss',
  AskParent: 'ask-parent',
  RequestMoreTime: 'request-more-time',
  TryLater: 'try-later',
  ContinueApproved: 'continue-approved',
} as const;

export const AppGameChildUxExplanationSource = {
  ParentRule: 'parent-rule',
  Capability: 'capability',
  ApprovalState: 'approval-state',
} as const;

export const AppGameChildUxCopyToken = {
  FamilyRuleTitle: 'appGame.childUx.familyRule.title',
  FamilyRuleBody: 'appGame.childUx.familyRule.body',
  NewAppTitle: 'appGame.childUx.newApp.title',
  NewAppBody: 'appGame.childUx.newApp.body',
  AlmostFinishedTitle: 'appGame.childUx.almostFinished.title',
  AlmostFinishedBody: 'appGame.childUx.almostFinished.body',
  LimitReachedTitle: 'appGame.childUx.limitReached.title',
  LimitReachedBody: 'appGame.childUx.limitReached.body',
  ManualRequiredTitle: 'appGame.childUx.manualRequired.title',
  ManualRequiredBody: 'appGame.childUx.manualRequired.body',
  UnavailableTitle: 'appGame.childUx.unavailable.title',
  UnavailableBody: 'appGame.childUx.unavailable.body',
  RequestSubmittedTitle: 'appGame.childUx.requestSubmitted.title',
  RequestSubmittedBody: 'appGame.childUx.requestSubmitted.body',
  RequestApprovedTitle: 'appGame.childUx.requestApproved.title',
  RequestApprovedBody: 'appGame.childUx.requestApproved.body',
  RequestDeniedTitle: 'appGame.childUx.requestDenied.title',
  RequestDeniedBody: 'appGame.childUx.requestDenied.body',
  AskParentAction: 'appGame.childUx.action.askParent',
  RequestMoreTimeAction: 'appGame.childUx.action.requestMoreTime',
  DismissAction: 'appGame.childUx.action.dismiss',
  TryLaterAction: 'appGame.childUx.action.tryLater',
  ContinueApprovedAction: 'appGame.childUx.action.continueApproved',
} as const;

type SurfaceStateValue = (typeof AppGameChildUxSurfaceState)[keyof typeof AppGameChildUxSurfaceState];
type CapabilityStateValue = (typeof AppGameChildUxCapabilityState)[keyof typeof AppGameChildUxCapabilityState];
type ClaimStateValue = (typeof AppGameChildUxClaimState)[keyof typeof AppGameChildUxClaimState];
type PrimaryActionValue = (typeof AppGameChildUxPrimaryAction)[keyof typeof AppGameChildUxPrimaryAction];
type ExplanationSourceValue = (typeof AppGameChildUxExplanationSource)[keyof typeof AppGameChildUxExplanationSource];
type CopyTokenValue = (typeof AppGameChildUxCopyToken)[keyof typeof AppGameChildUxCopyToken];

type ChildUxLike = {
  readonly surfaceState: SurfaceStateValue;
  readonly capabilityState: CapabilityStateValue;
  readonly claimState: ClaimStateValue;
  readonly primaryAction: PrimaryActionValue;
  readonly explanationSource: ExplanationSourceValue;
  readonly titleToken: CopyTokenValue;
  readonly bodyToken: CopyTokenValue;
  readonly primaryActionToken: CopyTokenValue;
  readonly evidenceReferences: ReadonlyArray<unknown>;
  readonly childReasonReferences: ReadonlyArray<unknown>;
  readonly childStatusReferences: ReadonlyArray<unknown>;
  readonly approvalRequestRef: unknown;
  readonly privateDiagnosticReferences: ReadonlyArray<unknown>;
  readonly adapterActionRef: unknown;
};

const surfaceTokenPairs: Record<SurfaceStateValue, readonly [CopyTokenValue, CopyTokenValue]> = {
  [AppGameChildUxSurfaceState.FamilyRuleWarning]: [
    AppGameChildUxCopyToken.FamilyRuleTitle,
    AppGameChildUxCopyToken.FamilyRuleBody,
  ],
  [AppGameChildUxSurfaceState.NewAppApprovalNeeded]: [
    AppGameChildUxCopyToken.NewAppTitle,
    AppGameChildUxCopyToken.NewAppBody,
  ],
  [AppGameChildUxSurfaceState.TimeAlmostFinished]: [
    AppGameChildUxCopyToken.AlmostFinishedTitle,
    AppGameChildUxCopyToken.AlmostFinishedBody,
  ],
  [AppGameChildUxSurfaceState.TimeLimitReached]: [
    AppGameChildUxCopyToken.LimitReachedTitle,
    AppGameChildUxCopyToken.LimitReachedBody,
  ],
  [AppGameChildUxSurfaceState.ManualRequired]: [
    AppGameChildUxCopyToken.ManualRequiredTitle,
    AppGameChildUxCopyToken.ManualRequiredBody,
  ],
  [AppGameChildUxSurfaceState.Unavailable]: [
    AppGameChildUxCopyToken.UnavailableTitle,
    AppGameChildUxCopyToken.UnavailableBody,
  ],
  [AppGameChildUxSurfaceState.RequestSubmitted]: [
    AppGameChildUxCopyToken.RequestSubmittedTitle,
    AppGameChildUxCopyToken.RequestSubmittedBody,
  ],
  [AppGameChildUxSurfaceState.RequestApproved]: [
    AppGameChildUxCopyToken.RequestApprovedTitle,
    AppGameChildUxCopyToken.RequestApprovedBody,
  ],
  [AppGameChildUxSurfaceState.RequestDenied]: [
    AppGameChildUxCopyToken.RequestDeniedTitle,
    AppGameChildUxCopyToken.RequestDeniedBody,
  ],
};

const actionTokens: Record<PrimaryActionValue, CopyTokenValue> = {
  [AppGameChildUxPrimaryAction.Dismiss]: AppGameChildUxCopyToken.DismissAction,
  [AppGameChildUxPrimaryAction.AskParent]: AppGameChildUxCopyToken.AskParentAction,
  [AppGameChildUxPrimaryAction.RequestMoreTime]: AppGameChildUxCopyToken.RequestMoreTimeAction,
  [AppGameChildUxPrimaryAction.TryLater]: AppGameChildUxCopyToken.TryLaterAction,
  [AppGameChildUxPrimaryAction.ContinueApproved]: AppGameChildUxCopyToken.ContinueApprovedAction,
};

const claimBySurface: Record<SurfaceStateValue, ClaimStateValue> = {
  [AppGameChildUxSurfaceState.FamilyRuleWarning]: AppGameChildUxClaimState.WarningOnly,
  [AppGameChildUxSurfaceState.NewAppApprovalNeeded]: AppGameChildUxClaimState.ApprovalNeeded,
  [AppGameChildUxSurfaceState.TimeAlmostFinished]: AppGameChildUxClaimState.AlmostFinished,
  [AppGameChildUxSurfaceState.TimeLimitReached]: AppGameChildUxClaimState.LimitReached,
  [AppGameChildUxSurfaceState.ManualRequired]: AppGameChildUxClaimState.ManualRequired,
  [AppGameChildUxSurfaceState.Unavailable]: AppGameChildUxClaimState.Unavailable,
  [AppGameChildUxSurfaceState.RequestSubmitted]: AppGameChildUxClaimState.RequestSubmitted,
  [AppGameChildUxSurfaceState.RequestApproved]: AppGameChildUxClaimState.RequestApproved,
  [AppGameChildUxSurfaceState.RequestDenied]: AppGameChildUxClaimState.RequestDenied,
};

export function appGameChildUxCopyTokensMatchSurface(childUx: ChildUxLike) {
  const [titleToken, bodyToken] = surfaceTokenPairs[childUx.surfaceState];
  return (
    childUx.titleToken === titleToken &&
    childUx.bodyToken === bodyToken &&
    childUx.primaryActionToken === actionTokens[childUx.primaryAction]
  );
}

export function appGameChildUxClaimMatchesSurface(childUx: ChildUxLike) {
  return childUx.claimState === claimBySurface[childUx.surfaceState];
}

export function appGameChildUxRequestRefsAreAuditable(childUx: ChildUxLike) {
  const asksParent =
    childUx.primaryAction === AppGameChildUxPrimaryAction.AskParent ||
    childUx.primaryAction === AppGameChildUxPrimaryAction.RequestMoreTime;

  return (
    (!asksParent && childUx.approvalRequestRef === null) ||
    (asksParent &&
      childUx.approvalRequestRef !== null &&
      childUx.evidenceReferences.length > 0 &&
      childUx.childReasonReferences.length > 0 &&
      childUx.childStatusReferences.length > 0)
  );
}

export function appGameChildUxStateIsHonest(childUx: ChildUxLike) {
  const manualOrUnavailable =
    childUx.capabilityState === AppGameChildUxCapabilityState.ManualRequired ||
    childUx.capabilityState === AppGameChildUxCapabilityState.Unavailable;
  const honestExplanation =
    childUx.explanationSource !== AppGameChildUxExplanationSource.ParentRule || childUx.evidenceReferences.length > 0;

  return (
    childUx.privateDiagnosticReferences.length === 0 &&
    honestExplanation &&
    (!manualOrUnavailable || childUx.adapterActionRef === null)
  );
}
