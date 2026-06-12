import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import {
  type BrowserInterventionDeliveryState,
  BrowserInterventionDeliveryStateSchema,
} from './browser-intervention-schemas';
import { BrowserPolicyAdapterProofRefSchema } from './browser-ai-policy-evaluator-values';
import { BrowserAiPostAnalysisActionPlanSchema } from './browser-ai-post-analysis-action-schemas';
import { type BrowserAiPostAnalysisActionLabel } from './browser-ai-post-analysis-action-values';
import {
  BrowserAiChildUxSnapshotIdSchema,
  type BrowserAiChildUxState,
  BrowserAiChildUxStateSchema,
  BrowserAiChildUxSurfaceSchema,
  BrowserAiChildUxTextToken,
  BrowserAiChildUxTextTokenSchema,
  BrowserAiChildUxToneSchema,
} from './browser-ai-child-ux-values';

export {
  BrowserAiChildUxSnapshotIdSchema,
  BrowserAiChildUxStateSchema,
  BrowserAiChildUxSurfaceSchema,
  BrowserAiChildUxTextToken,
  BrowserAiChildUxTextTokenSchema,
  BrowserAiChildUxToneSchema,
} from './browser-ai-child-ux-values';

const EvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected child UX evidence ids')
);
const OptionalAdapterProofRefSchema = Schema.Union(BrowserPolicyAdapterProofRefSchema, Schema.Null);
const OptionalPostAnalysisActionPlanSchema = Schema.Union(BrowserAiPostAnalysisActionPlanSchema, Schema.Null);
const OptionalTextTokenSchema = Schema.Union(BrowserAiChildUxTextTokenSchema, Schema.Null);

export const BrowserAiChildUxSchemaVersion = 1;

const BrowserAiChildUxSnapshotBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiChildUxSchemaVersion),
  snapshotId: BrowserAiChildUxSnapshotIdSchema,
  createdAt: ActivityTimestampSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  state: BrowserAiChildUxStateSchema,
  tone: BrowserAiChildUxToneSchema,
  surface: BrowserAiChildUxSurfaceSchema,
  primaryTextToken: BrowserAiChildUxTextTokenSchema,
  secondaryTextToken: OptionalTextTokenSchema,
  deliveryState: BrowserInterventionDeliveryStateSchema,
  adapterProofRef: OptionalAdapterProofRefSchema,
  postAnalysisActionPlan: OptionalPostAnalysisActionPlanSchema,
  rawCopyClaimed: Schema.Boolean,
  visualRenderClaimed: Schema.Boolean,
  surveillanceCopyClaimed: Schema.Boolean,
  shamingCopyClaimed: Schema.Boolean,
});

export const BrowserAiChildUxSnapshotSchema = withParser(
  BrowserAiChildUxSnapshotBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiChildUxSnapshotIsConsistent(value) ||
        'Expected child UX snapshot to use calm tokenized copy and proof-backed delivery'
    )
  )
);

export const decodeBrowserAiChildUxSnapshot = Schema.decodeUnknownSync(BrowserAiChildUxSnapshotSchema);

export type BrowserAiChildUxSnapshot = Infer<typeof BrowserAiChildUxSnapshotSchema>;

function browserAiChildUxSnapshotIsConsistent(value: Infer<typeof BrowserAiChildUxSnapshotBaseSchema>) {
  if (value.rawCopyClaimed || value.visualRenderClaimed || value.surveillanceCopyClaimed || value.shamingCopyClaimed) {
    return false;
  }
  if (!stateMatchesPrimaryToken(value.state, value.primaryTextToken)) {
    return false;
  }
  if (!deliveryStateMatchesChildState(value.state, value.deliveryState)) {
    return false;
  }
  if (!renderedDeliveryHasAdapterProof(value.deliveryState, value.adapterProofRef)) {
    return false;
  }
  return actionPlanMatchesChildState(value.state, value.postAnalysisActionPlan);
}

function stateMatchesPrimaryToken(state: BrowserAiChildUxState, token: BrowserAiChildUxTextToken) {
  return stateToPrimaryToken(state) === token;
}

function stateToPrimaryToken(state: BrowserAiChildUxState): BrowserAiChildUxTextToken {
  switch (state) {
    case 'opening':
      return BrowserAiChildUxTextToken.Opening;
    case 'checking':
      return BrowserAiChildUxTextToken.Checking;
    case 'allowed':
      return BrowserAiChildUxTextToken.Allowed;
    case 'warning':
      return BrowserAiChildUxTextToken.Warning;
    case 'approval_required':
      return BrowserAiChildUxTextToken.Approval;
    case 'limited':
      return BrowserAiChildUxTextToken.Limited;
    case 'blocked':
      return BrowserAiChildUxTextToken.Blocked;
    case 'unclassified':
      return BrowserAiChildUxTextToken.Unclassified;
    case 'manual_required':
      return BrowserAiChildUxTextToken.Manual;
    case 'unavailable':
      return BrowserAiChildUxTextToken.Unavailable;
  }
}

function deliveryStateMatchesChildState(state: BrowserAiChildUxState, deliveryState: BrowserInterventionDeliveryState) {
  switch (deliveryState) {
    case 'checking-hold-rendered':
      return state === 'checking';
    case 'warn-page-rendered':
      return state === 'warning' || state === 'limited';
    case 'block-page-rendered':
      return state === 'blocked';
    case 'approval-hold-rendered':
      return state === 'approval_required';
    default:
      return true;
  }
}

function renderedDeliveryHasAdapterProof(
  deliveryState: BrowserInterventionDeliveryState,
  adapterProofRef: Infer<typeof OptionalAdapterProofRefSchema>
) {
  if (
    deliveryState === 'checking-hold-rendered' ||
    deliveryState === 'warn-page-rendered' ||
    deliveryState === 'block-page-rendered' ||
    deliveryState === 'approval-hold-rendered'
  ) {
    return adapterProofRef !== null;
  }
  return true;
}

function actionPlanMatchesChildState(
  state: BrowserAiChildUxState,
  actionPlan: Infer<typeof OptionalPostAnalysisActionPlanSchema>
) {
  if (state === 'opening' || state === 'checking' || state === 'unavailable') {
    return true;
  }
  if (state === 'unclassified' || state === 'manual_required') {
    return actionPlan === null || actionPlan.actionLabels.includes('manual_required');
  }
  if (actionPlan === null) {
    return false;
  }
  return actionLabelsMatchChildState(state, actionPlan.actionLabels);
}

function actionLabelsMatchChildState(
  state: BrowserAiChildUxState,
  actionLabels: ReadonlyArray<BrowserAiPostAnalysisActionLabel>
) {
  switch (state) {
    case 'allowed':
      return actionLabelsContainAny(actionLabels, 'continue_allowed', 'background_reviewed', 'remembered_with_expiry');
    case 'warning':
    case 'limited':
      return actionLabels.includes('warning_shown_after_review');
    case 'approval_required':
      return actionLabels.includes('parent_approval_requested_after_review');
    case 'blocked':
      return actionLabelsContainAny(actionLabels, 'playback_stopped_after_review', 'future_visits_blocked');
    case 'unclassified':
    case 'manual_required':
      return actionLabels.includes('manual_required');
    default:
      return true;
  }
}

function actionLabelsContainAny(
  actionLabels: ReadonlyArray<BrowserAiPostAnalysisActionLabel>,
  first: BrowserAiPostAnalysisActionLabel,
  second: BrowserAiPostAnalysisActionLabel,
  third?: BrowserAiPostAnalysisActionLabel
) {
  return [first, second, third].some((label) => label !== undefined && actionLabels.includes(label));
}
