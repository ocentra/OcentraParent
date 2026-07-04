export interface AppGameControlApprovalAuthorityRuleInput {
  readonly authorityState: 'active' | 'observe-only' | 'manual-required' | 'unavailable';
  readonly allowedPolicyKinds: readonly unknown[];
  readonly canApprove: boolean;
  readonly canDeny: boolean;
  readonly canExtend: boolean;
  readonly canOverride: boolean;
  readonly canObserveOnly: boolean;
}

export interface AppGameControlApprovalRequestRuleInput {
  readonly policyKind: 'app-control' | 'game-control';
  readonly requestedSettingRefs: readonly { readonly writesTo: unknown }[];
  readonly unansweredFallback: 'deny' | 'expire' | 'observe-only' | 'manual-required';
  readonly candidate: {
    readonly candidateKind:
      | 'new-inventory-app'
      | 'unknown-runtime-process'
      | 'portable-executable'
      | 'installer-or-updater'
      | 'launcher-game-candidate'
      | 'unknown-game-like-executable';
    readonly evidenceReferences: readonly unknown[];
  } | null;
  readonly childReasonState: 'not-requested' | 'reason-ref-backed' | 'unavailable' | 'manual-required';
  readonly childReasonReferences: readonly unknown[];
  readonly childStatusReferences: readonly unknown[];
}

const authorityStateValidators: Record<
  AppGameControlApprovalAuthorityRuleInput['authorityState'],
  (authority: AppGameControlApprovalAuthorityRuleInput) => boolean
> = {
  active: (authority) => authority.allowedPolicyKinds.length > 0,
  'observe-only': (authority) => restrictedAuthorityGrantIsConsistent(authority),
  'manual-required': (authority) => restrictedAuthorityGrantIsConsistent(authority),
  unavailable: (authority) => restrictedAuthorityGrantIsConsistent(authority),
};

const policyKindSettingRefPrefixes: Record<AppGameControlApprovalRequestRuleInput['policyKind'], string> = {
  'app-control': '/appPolicy/',
  'game-control': '/gamePolicy/',
};

const childReasonStateValidators: Record<
  AppGameControlApprovalRequestRuleInput['childReasonState'],
  (request: AppGameControlApprovalRequestRuleInput) => boolean
> = {
  'not-requested': childReasonRefsAreAbsent,
  'reason-ref-backed': childReasonRefsArePresent,
  unavailable: childReasonRefsAreAbsent,
  'manual-required': childReasonRefsAreAbsent,
};

const weakGameCandidateKinds = new Set<AppGameControlApprovalRequestRuleInput['candidate'] extends null
  ? never
  : AppGameControlApprovalRequestRuleInput['candidate']['candidateKind']>([
  'launcher-game-candidate',
  'unknown-game-like-executable',
]);

const safeWeakGameFallbacks = new Set<AppGameControlApprovalRequestRuleInput['unansweredFallback']>([
  'observe-only',
  'manual-required',
]);

export function authorityGrantStateIsConsistent(
  authority: AppGameControlApprovalAuthorityRuleInput
): boolean {
  return authorityStateValidators[authority.authorityState](authority);
}

export function requestSettingRefsMatchPolicyKind(request: AppGameControlApprovalRequestRuleInput): boolean {
  const expectedPrefix = policyKindSettingRefPrefixes[request.policyKind];
  return request.requestedSettingRefs.every((settingRef) => String(settingRef.writesTo).startsWith(expectedPrefix));
}

export function approvalRequestCandidateRefsAreConsistent(request: AppGameControlApprovalRequestRuleInput): boolean {
  return (
    request.candidate === null ||
    (approvalRequestCandidateHasSupportingRefs(request) &&
      childReasonRefsMatchState(request) &&
      weakGameCandidateFallbackIsSafe(request))
  );
}

function approvalRequestCandidateHasSupportingRefs(request: AppGameControlApprovalRequestRuleInput): boolean {
  return request.candidate.evidenceReferences.length > 0 && request.childStatusReferences.length > 0;
}

function childReasonRefsMatchState(request: AppGameControlApprovalRequestRuleInput): boolean {
  return childReasonStateValidators[request.childReasonState](request);
}

function childReasonRefsArePresent(request: AppGameControlApprovalRequestRuleInput): boolean {
  return request.childReasonReferences.length > 0;
}

function childReasonRefsAreAbsent(request: AppGameControlApprovalRequestRuleInput): boolean {
  return request.childReasonReferences.length === 0;
}

function weakGameCandidateFallbackIsSafe(request: AppGameControlApprovalRequestRuleInput): boolean {
  return (
    request.policyKind !== 'game-control' ||
    request.candidate === null ||
    !weakGameCandidateKinds.has(request.candidate.candidateKind) ||
    safeWeakGameFallbacks.has(request.unansweredFallback)
  );
}

function restrictedAuthorityGrantIsConsistent(authority: AppGameControlApprovalAuthorityRuleInput): boolean {
  return (
    !authority.canApprove &&
    !authority.canDeny &&
    !authority.canExtend &&
    !authority.canOverride &&
    authority.canObserveOnly
  );
}
