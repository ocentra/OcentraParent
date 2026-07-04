export interface AppGameControlApprovalDecisionRuleInput {
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

const decisionParentActionValidators: Record<
  AppGameControlApprovalDecisionRuleInput['decisionState'],
  (decision: AppGameControlApprovalDecisionRuleInput) => boolean
> = {
  approved: decisionHasParentAction,
  override: decisionHasParentAction,
  denied: decisionHasOptionalParentAction,
  expired: decisionHasOptionalParentAction,
  'manual-required': decisionHasOptionalParentAction,
};

const decisionResponseScopeValidators: Record<
  AppGameControlApprovalDecisionRuleInput['decisionState'],
  (decision: AppGameControlApprovalDecisionRuleInput) => boolean
> = {
  approved: approvedResponseScopeIsExecutable,
  override: approvedResponseScopeIsExecutable,
  denied: deniedResponseScopeIsConsistent,
  expired: expiredResponseScopeIsConsistent,
  'manual-required': manualRequiredResponseScopeIsConsistent,
};

const decisionPersistenceValidators: Record<
  AppGameControlApprovalDecisionRuleInput['persistenceState'],
  (decision: AppGameControlApprovalDecisionRuleInput) => boolean
> = {
  'not-persisted': persistenceWithoutAuditRefsIsConsistent,
  replayable: persistenceWithAuditRefsIsConsistent,
  replayed: persistenceWithAuditRefsIsConsistent,
  'storage-unavailable': persistenceWithoutAuditRefsIsConsistent,
};

export function decisionPolicyVersionMatchesParentAction(
  decision: AppGameControlApprovalDecisionRuleInput
): boolean {
  return decision.parentAction === null || decision.parentAction.policyVersion === decision.policyVersion;
}

export function decisionParentActionPresenceIsConsistent(
  decision: AppGameControlApprovalDecisionRuleInput
): boolean {
  return decisionParentActionValidators[decision.decisionState](decision);
}

export function approvalDecisionResponseScopeIsConsistent(
  decision: AppGameControlApprovalDecisionRuleInput
): boolean {
  return decision.responseScope === null || decisionResponseScopeValidators[decision.decisionState](decision);
}

export function approvalDecisionPersistenceIsConsistent(
  decision: AppGameControlApprovalDecisionRuleInput
): boolean {
  return decisionPersistenceValidators[decision.persistenceState](decision);
}

function decisionHasParentAction(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.parentAction !== null;
}

function decisionHasOptionalParentAction(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.parentAction === null || decision.decisionState === 'manual-required';
}

function approvedResponseScopeIsExecutable(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return (
    decision.responseScope !== 'deny' &&
    decision.responseScope !== 'report-only' &&
    (decision.responseScope !== 'allow-once' || decision.decisionExpiresAt !== null)
  );
}

function deniedResponseScopeIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.responseScope === 'deny';
}

function expiredResponseScopeIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.responseScope === 'allow-once' && decision.decisionExpiresAt !== null;
}

function manualRequiredResponseScopeIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.responseScope === 'block-if-supported' || decision.responseScope === 'report-only';
}

function persistenceWithoutAuditRefsIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.auditReferences.length === 0;
}

function persistenceWithAuditRefsIsConsistent(decision: AppGameControlApprovalDecisionRuleInput): boolean {
  return decision.auditReferences.length > 0;
}
