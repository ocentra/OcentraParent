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
}

interface AppGameControlApprovalDecisionRuleInput {
  readonly decisionState: 'approved' | 'denied' | 'expired' | 'override' | 'manual-required';
  readonly parentAction: { readonly policyVersion: unknown } | null;
  readonly policyVersion: unknown;
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

export function decisionPolicyVersionMatchesParentAction(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.parentAction === null || decision.parentAction.policyVersion === decision.policyVersion;
}

export function decisionParentActionPresenceIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  if (decision.decisionState === 'approved' || decision.decisionState === 'override') {
    return decision.parentAction !== null;
  }

  return decision.decisionState === 'manual-required' || decision.parentAction === null;
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
  const unknownAppProof = result.evidenceProofKind === 'unknown-app' && result.request.policyKind === 'app-control';
  return !(actionResultClaimsDispatch(result) && (weakLauncherGameProof || unknownAppProof));
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
