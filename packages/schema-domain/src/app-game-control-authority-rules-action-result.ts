import type { AppGameControlApprovalDecisionRuleInput } from './app-game-control-authority-rules-decision';

export interface AppGameControlActionResultRuleInput {
  readonly request: { readonly policyKind: 'app-control' | 'game-control' };
  readonly decision: AppGameControlApprovalDecisionRuleInput & {
    readonly policyKind: 'app-control' | 'game-control';
  };
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

const approvalStateValidators: Record<
  AppGameControlActionResultRuleInput['decision']['decisionState'],
  (result: AppGameControlActionResultRuleInput) => boolean
> = {
  approved: approvedOrOverrideApprovalStateIsConsistent,
  override: approvedOrOverrideApprovalStateIsConsistent,
  denied: deniedApprovalStateIsConsistent,
  expired: expiredApprovalStateIsConsistent,
  'manual-required': manualRequiredApprovalStateIsConsistent,
};

const capabilityStateValidators: Record<
  AppGameControlActionResultRuleInput['capabilityState'],
  (result: AppGameControlActionResultRuleInput) => boolean
> = {
  supported: supportedCapabilityStateAllowsResult,
  unavailable: unavailableCapabilityStateAllowsResult,
  degraded: unavailableCapabilityStateAllowsResult,
  'dry-run': dryRunCapabilityStateAllowsResult,
  'observe-only': observeOnlyCapabilityStateAllowsResult,
  'manual-required': manualRequiredCapabilityStateAllowsResult,
};

const dispatchResultStatuses = new Set<AppGameControlActionResultRuleInput['resultStatus']>([
  'dispatch-ready',
  'would-enforce',
  'enforced',
]);

const disallowedEvidenceProofKindsByPolicyKind: Record<
  AppGameControlActionResultRuleInput['request']['policyKind'],
  ReadonlySet<AppGameControlActionResultRuleInput['evidenceProofKind']>
> = {
  'app-control': new Set(['unknown-app']),
  'game-control': new Set(['launcher-only', 'unknown-game-like']),
};

export function actionResultApprovalStateIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  return approvalStateValidators[result.decision.decisionState](result);
}

export function actionResultCapabilityIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  return capabilityRowStateMatches(result) && capabilityStateValidators[result.capabilityState](result);
}

export function actionResultEvidenceProofIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  return (
    !dispatchResultStatuses.has(result.resultStatus) ||
    !disallowedEvidenceProofKindsByPolicyKind[result.request.policyKind].has(result.evidenceProofKind)
  );
}

function approvedOrOverrideApprovalStateIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  return result.approvalState === 'approved' || result.approvalState === 'override-active';
}

function deniedApprovalStateIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  return result.approvalState === 'denied' && result.resultStatus === 'not-dispatched';
}

function expiredApprovalStateIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  return result.approvalState === 'expired' && result.resultStatus === 'not-dispatched';
}

function manualRequiredApprovalStateIsConsistent(result: AppGameControlActionResultRuleInput): boolean {
  return result.approvalState === 'manual-required' && result.resultStatus === 'manual-required';
}

function capabilityRowStateMatches(result: AppGameControlActionResultRuleInput): boolean {
  return result.capability.capabilityState === result.capabilityState;
}

function supportedCapabilityStateAllowsResult(result: AppGameControlActionResultRuleInput): boolean {
  return result.resultStatus === 'enforced'
    ? result.enforcementResult?.status === 'actually-enforced'
    : result.resultStatus !== 'would-enforce';
}

function unavailableCapabilityStateAllowsResult(result: AppGameControlActionResultRuleInput): boolean {
  return result.resultStatus === 'unavailable' && result.enforcementResult === null;
}

function dryRunCapabilityStateAllowsResult(result: AppGameControlActionResultRuleInput): boolean {
  return result.resultStatus === 'would-enforce'
    ? result.enforcementResult?.status === 'would-enforce'
    : result.resultStatus !== 'enforced';
}

function observeOnlyCapabilityStateAllowsResult(result: AppGameControlActionResultRuleInput): boolean {
  return result.resultStatus !== 'enforced' && result.resultStatus !== 'would-enforce';
}

function manualRequiredCapabilityStateAllowsResult(result: AppGameControlActionResultRuleInput): boolean {
  return result.resultStatus === 'manual-required' && result.enforcementResult === null;
}
