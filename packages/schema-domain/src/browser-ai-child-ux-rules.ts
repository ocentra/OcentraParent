import type { BrowserInterventionDeliveryState } from './browser-intervention-schemas';
import { BrowserAiChildUxTextToken, type BrowserAiChildUxState } from '@ocentra-parent/schema-domain/browser-ai-child-ux-values';
import type { BrowserAiPostAnalysisActionLabel } from './browser-ai-post-analysis-action-values';
import { includesAny } from './browser-ai-schema-shared';

type ActionPlanShape = {
  readonly actionLabels: readonly BrowserAiPostAnalysisActionLabel[];
} | null;

type SnapshotShape = {
  readonly state: BrowserAiChildUxState;
  readonly primaryTextToken: BrowserAiChildUxTextToken;
  readonly deliveryState: BrowserInterventionDeliveryState;
  readonly adapterProofRef: unknown | null;
  readonly postAnalysisActionPlan: ActionPlanShape;
  readonly rawCopyClaimed: boolean;
  readonly visualRenderClaimed: boolean;
  readonly surveillanceCopyClaimed: boolean;
  readonly shamingCopyClaimed: boolean;
};

const PrimaryTokenByState = {
  opening: BrowserAiChildUxTextToken.Opening,
  checking: BrowserAiChildUxTextToken.Checking,
  allowed: BrowserAiChildUxTextToken.Allowed,
  warning: BrowserAiChildUxTextToken.Warning,
  approval_required: BrowserAiChildUxTextToken.Approval,
  limited: BrowserAiChildUxTextToken.Limited,
  blocked: BrowserAiChildUxTextToken.Blocked,
  unclassified: BrowserAiChildUxTextToken.Unclassified,
  manual_required: BrowserAiChildUxTextToken.Manual,
  unavailable: BrowserAiChildUxTextToken.Unavailable,
} as const satisfies Record<BrowserAiChildUxState, BrowserAiChildUxTextToken>;

const ChildStatesByDeliveryState = {
  'checking-hold-rendered': ['checking'],
  'warn-page-rendered': ['warning', 'limited'],
  'block-page-rendered': ['blocked'],
  'approval-hold-rendered': ['approval_required'],
} as const satisfies Partial<Record<BrowserInterventionDeliveryState, readonly BrowserAiChildUxState[]>>;

const RenderedDeliveryStates = new Set<BrowserInterventionDeliveryState>([
  'checking-hold-rendered',
  'warn-page-rendered',
  'block-page-rendered',
  'approval-hold-rendered',
]);

const AllowedActionsByState = {
  allowed: ['continue_allowed', 'background_reviewed', 'remembered_with_expiry'],
  warning: ['warning_shown_after_review'],
  limited: ['warning_shown_after_review'],
  approval_required: ['parent_approval_requested_after_review'],
  blocked: ['playback_stopped_after_review', 'future_visits_blocked'],
  unclassified: ['manual_required'],
  manual_required: ['manual_required'],
} as const satisfies Partial<Record<BrowserAiChildUxState, readonly BrowserAiPostAnalysisActionLabel[]>>;

const ActionOptionalStates = new Set<BrowserAiChildUxState>(['opening', 'checking', 'unavailable']);

export function browserAiChildUxSnapshotIsConsistent(value: SnapshotShape) {
  return (
    !claimsUnsafeCopy(value) &&
    value.primaryTextToken === PrimaryTokenByState[value.state] &&
    deliveryStateMatchesChildState(value.state, value.deliveryState) &&
    renderedDeliveryHasAdapterProof(value.deliveryState, value.adapterProofRef) &&
    actionPlanMatchesChildState(value.state, value.postAnalysisActionPlan)
  );
}

function claimsUnsafeCopy(value: SnapshotShape) {
  return value.rawCopyClaimed || value.visualRenderClaimed || value.surveillanceCopyClaimed || value.shamingCopyClaimed;
}

function deliveryStateMatchesChildState(
  state: BrowserAiChildUxState,
  deliveryState: BrowserInterventionDeliveryState
) {
  const allowedStates = ChildStatesByDeliveryState[deliveryState];
  return allowedStates === undefined || allowedStates.includes(state);
}

function renderedDeliveryHasAdapterProof(
  deliveryState: BrowserInterventionDeliveryState,
  adapterProofRef: unknown | null
) {
  return !RenderedDeliveryStates.has(deliveryState) || adapterProofRef !== null;
}

function actionPlanMatchesChildState(state: BrowserAiChildUxState, actionPlan: ActionPlanShape) {
  if (ActionOptionalStates.has(state)) {
    return true;
  }
  if (actionPlan === null) {
    return false;
  }
  const allowedActions = AllowedActionsByState[state];
  return allowedActions !== undefined && includesAny(actionPlan.actionLabels, allowedActions);
}
