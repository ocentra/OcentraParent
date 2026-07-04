import type { BrowserAiPostAnalysisActionLabel } from './browser-ai-post-analysis-action-values';
import type { BrowserPolicyDecisionOutcome } from './browser-ai-policy-evaluator-values';
import { includesAny } from './browser-ai-schema-shared';

type PostAnalysisActionShape = {
  readonly actionLabels: readonly BrowserAiPostAnalysisActionLabel[];
  readonly timing: string;
  readonly childAlreadyEngaged: boolean;
  readonly deliveryState: string;
  readonly adapterProofRef: unknown | null;
  readonly rememberUntil: unknown | null;
  readonly realtimeBlockClaimed: boolean;
  readonly browserRuntimeMutationClaimed: boolean;
  readonly directEnforcementClaimed: boolean;
  readonly policyDecision: { readonly outcome: BrowserPolicyDecisionOutcome };
};

const AdapterProofActionLabels = [
  'warning_shown_after_review',
  'playback_stopped_after_review',
  'parent_approval_requested_after_review',
  'future_visits_blocked',
] as const satisfies readonly BrowserAiPostAnalysisActionLabel[];

const AllowedActionsByOutcome = {
  allow: ['continue_allowed', 'background_reviewed', 'remembered_with_expiry'],
  warn: ['warning_shown_after_review'],
  ask_parent: ['parent_approval_requested_after_review'],
  time_limit: ['warning_shown_after_review', 'manual_required'],
  block: ['playback_stopped_after_review', 'future_visits_blocked'],
  unknown: ['manual_required', 'parent_approval_requested_after_review'],
} as const satisfies Record<BrowserPolicyDecisionOutcome, readonly BrowserAiPostAnalysisActionLabel[]>;

export function browserAiPostAnalysisActionPlanIsConsistent(value: PostAnalysisActionShape) {
  return (
    !claimsAuthority(value) &&
    (!value.childAlreadyEngaged || value.timing !== 'before_playback') &&
    (value.deliveryState !== 'delivered' ||
      !includesAny(value.actionLabels, AdapterProofActionLabels) ||
      value.adapterProofRef !== null) &&
    (!value.actionLabels.includes('remembered_with_expiry') || value.rememberUntil !== null) &&
    includesAny(value.actionLabels, AllowedActionsByOutcome[value.policyDecision.outcome])
  );
}

function claimsAuthority(value: PostAnalysisActionShape) {
  return value.realtimeBlockClaimed || value.browserRuntimeMutationClaimed || value.directEnforcementClaimed;
}
