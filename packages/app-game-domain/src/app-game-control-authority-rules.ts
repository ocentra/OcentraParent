interface AppGameControlApprovalAuthorityRuleInput {
  readonly authorityState: 'active' | 'observe-only' | 'manual-required' | 'unavailable';
  readonly allowedPolicyKinds: readonly unknown[];
  readonly canApprove: boolean;
  readonly canDeny: boolean;
  readonly canExtend: boolean;
  readonly canOverride: boolean;
  readonly canObserveOnly: boolean;
}

interface AppGameControlApprovalRequestRuleInput {
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

interface AppGameControlApprovalDecisionRuleInput {
  readonly decisionState: 'approved' | 'denied' | 'expired' | 'override' | 'manual-required';
  readonly parentAction: { readonly policyVersion: unknown } | null;
  readonly policyVersion: unknown;
  readonly responseScope:
    | 'allow-once'
    | 'allow-this-app-game'
    | 'allow-category'
    | 'ask-child-why'
    | 'deny'
    | 'report-only'
    | 'block-if-supported'
    | null;
  readonly decisionExpiresAt: unknown | null;
  readonly auditReferences: readonly unknown[];
  readonly persistenceState: 'not-persisted' | 'replayable' | 'replayed' | 'storage-unavailable';
}

interface AppGameControlActionResultRuleInput {
  readonly request: { readonly policyKind: 'app-control' | 'game-control' };
  readonly decision: AppGameControlApprovalDecisionRuleInput & { readonly policyKind: 'app-control' | 'game-control' };
  readonly approvalState:
    | 'not-required'
    | 'pending'
    | 'approved'
    | 'denied'
    | 'expired'
    | 'override-active'
    | 'manual-required';
  readonly capabilityState: 'supported' | 'unavailable' | 'degraded' | 'dry-run' | 'observe-only' | 'manual-required';
  readonly capability: { readonly capabilityState: unknown };
  readonly evidenceProofKind:
    | 'app-identity-proof'
    | 'gameplay-proof'
    | 'launcher-only'
    | 'unknown-app'
    | 'unknown-game-like'
    | 'catalog-match'
    | 'process-observation';
  readonly resultStatus:
    | 'not-dispatched'
    | 'dispatch-ready'
    | 'would-enforce'
    | 'enforced'
    | 'manual-required'
    | 'unavailable';
  readonly enforcementResult: { readonly status: unknown } | null;
}

export function authorityGrantStateIsConsistent(authority: AppGameControlApprovalAuthorityRuleInput): boolean {
  if (authority.authorityState === 'active') {
    return authority.allowedPolicyKinds.length > 0;
  }

  return (
    !authority.canApprove &&
    !authority.canDeny &&
    !authority.canExtend &&
    !authority.canOverride &&
    authority.canObserveOnly
  );
}

export function requestSettingRefsMatchPolicyKind(request: AppGameControlApprovalRequestRuleInput): boolean {
  const expectedPrefix = request.policyKind === 'app-control' ? '/appPolicy/' : '/gamePolicy/';
  return request.requestedSettingRefs.every((settingRef) => String(settingRef.writesTo).startsWith(expectedPrefix));
}

export function approvalRequestCandidateRefsAreConsistent(request: AppGameControlApprovalRequestRuleInput): boolean {
  if (request.candidate === null) {
    return true;
  }

  return (
    request.candidate.evidenceReferences.length > 0 &&
    request.childStatusReferences.length > 0 &&
    childReasonRefsMatchState(request) &&
    weakGameCandidateFallbackIsSafe(request)
  );
}

export function decisionPolicyVersionMatchesParentAction(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.parentAction === null || decision.parentAction.policyVersion === decision.policyVersion;
}

export function decisionParentActionPresenceIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  if (decision.decisionState === 'approved' || decision.decisionState === 'override') {
    return decision.parentAction !== null;
  }

  return decision.decisionState === 'manual-required' || decision.parentAction === null;
}

export function approvalDecisionResponseScopeIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  if (decision.responseScope === null) {
    return true;
  }

  switch (decision.decisionState) {
    case 'approved':
    case 'override':
      return approvedResponseScopeIsExecutable(decision.responseScope, decision.decisionExpiresAt);
    case 'denied':
      return decision.responseScope === 'deny';
    case 'expired':
      return decision.responseScope === 'allow-once' && decision.decisionExpiresAt !== null;
    case 'manual-required':
      return decision.responseScope === 'block-if-supported' || decision.responseScope === 'report-only';
  }
}

export function approvalDecisionPersistenceIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  if (decision.persistenceState === 'storage-unavailable') {
    return decision.auditReferences.length === 0;
  }

  if (decision.persistenceState === 'replayable' || decision.persistenceState === 'replayed') {
    return decision.auditReferences.length > 0;
  }

  return true;
}

export function actionResultApprovalStateIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  switch (result.decision.decisionState) {
    case 'approved':
    case 'override':
      return result.approvalState === 'approved' || result.approvalState === 'override-active';
    case 'denied':
      return result.approvalState === 'denied' && result.resultStatus === 'not-dispatched';
    case 'expired':
      return result.approvalState === 'expired' && result.resultStatus === 'not-dispatched';
    case 'manual-required':
      return result.approvalState === 'manual-required' && result.resultStatus === 'manual-required';
  }
}

export function actionResultCapabilityIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  return capabilityRowStateMatches(result) && capabilityStateAllowsResult(result);
}

export function actionResultEvidenceProofIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  const weakLauncherGameProof =
    result.evidenceProofKind === 'launcher-only' && result.request.policyKind === 'game-control';
  const weakUnknownGameProof =
    result.evidenceProofKind === 'unknown-game-like' && result.request.policyKind === 'game-control';
  const unknownAppProof = result.evidenceProofKind === 'unknown-app' && result.request.policyKind === 'app-control';
  return !(actionResultClaimsDispatch(result) && (weakLauncherGameProof || weakUnknownGameProof || unknownAppProof));
}

function actionResultClaimsDispatch(result: AppGameControlActionResultRuleInput): boolean {
  return (
    result.resultStatus === 'dispatch-ready' ||
    result.resultStatus === 'would-enforce' ||
    result.resultStatus === 'enforced'
  );
}

function capabilityRowStateMatches(result: AppGameControlActionResultRuleInput): boolean {
  return result.capability.capabilityState === result.capabilityState;
}

function capabilityStateAllowsResult(result: AppGameControlActionResultRuleInput): boolean {
  if (result.resultStatus === 'enforced') {
    return result.enforcementResult?.status === 'actually-enforced' && result.capabilityState === 'supported';
  }

  if (result.resultStatus === 'would-enforce') {
    return result.enforcementResult?.status === 'would-enforce' && result.capabilityState === 'dry-run';
  }

  if (result.capabilityState === 'manual-required') {
    return result.resultStatus === 'manual-required' && result.enforcementResult === null;
  }

  if (capabilityStateRequiresUnavailableResult(result)) {
    return result.resultStatus === 'unavailable' && result.enforcementResult === null;
  }

  return true;
}

function capabilityStateRequiresUnavailableResult(result: AppGameControlActionResultRuleInput): boolean {
  return result.capabilityState === 'degraded' || result.capabilityState === 'unavailable';
}

function childReasonRefsMatchState(request: AppGameControlApprovalRequestRuleInput): boolean {
  if (request.childReasonState === 'reason-ref-backed') {
    return request.childReasonReferences.length > 0;
  }

  return request.childReasonReferences.length === 0;
}

function weakGameCandidateFallbackIsSafe(request: AppGameControlApprovalRequestRuleInput): boolean {
  if (request.policyKind !== 'game-control' || request.candidate === null) {
    return true;
  }

  if (
    request.candidate.candidateKind === 'launcher-game-candidate' ||
    request.candidate.candidateKind === 'unknown-game-like-executable'
  ) {
    return request.unansweredFallback === 'observe-only' || request.unansweredFallback === 'manual-required';
  }

  return true;
}

function approvedResponseScopeIsExecutable(
  responseScope: NonNullable<AppGameControlApprovalDecisionRuleInput['responseScope']>,
  decisionExpiresAt: unknown | null
): boolean {
  if (responseScope === 'allow-once') {
    return decisionExpiresAt !== null;
  }

  return responseScope !== 'deny' && responseScope !== 'report-only';
}
