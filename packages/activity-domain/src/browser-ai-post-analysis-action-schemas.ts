import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { BrowserUrlAiAnalysisIdSchema, BrowserParentRuleRefSchema } from './browser-ai-analysis-values';
import {
  BrowserPolicyAdapterProofRefSchema,
  BrowserPolicyDecisionAuditRefSchema,
  type BrowserPolicyDecisionOutcome,
} from './browser-ai-policy-evaluator-values';
import { BrowserPolicyDecisionSchema } from './browser-ai-policy-evaluator-schemas';
import {
  BrowserAiPostAnalysisActionAuditRefSchema,
  type BrowserAiPostAnalysisActionLabel,
  BrowserAiPostAnalysisActionLabelSchema,
  BrowserAiPostAnalysisActionPlanIdSchema,
  BrowserAiPostAnalysisActionTimingSchema,
  BrowserAiPostAnalysisActionTriggerSchema,
  type BrowserAiPostAnalysisDeliveryState,
  BrowserAiPostAnalysisDeliveryStateSchema,
} from './browser-ai-post-analysis-action-values';

export {
  BrowserAiPostAnalysisActionLabelSchema,
  BrowserAiPostAnalysisActionTimingSchema,
  BrowserAiPostAnalysisActionTriggerSchema,
  BrowserAiPostAnalysisDeliveryStateSchema,
} from './browser-ai-post-analysis-action-values';

const EvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected post-analysis action evidence ids')
);
const ActionLabelsSchema = Schema.Array(BrowserAiPostAnalysisActionLabelSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one post-analysis action label')
);
const ActionAuditRefsSchema = Schema.Array(BrowserAiPostAnalysisActionAuditRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected post-analysis action audit refs')
);
const OptionalAnalysisIdSchema = Schema.Union(BrowserUrlAiAnalysisIdSchema, Schema.Null);
const OptionalAdapterProofRefSchema = Schema.Union(BrowserPolicyAdapterProofRefSchema, Schema.Null);
const OptionalRememberUntilSchema = Schema.Union(ActivityTimestampSchema, Schema.Null);

export const BrowserAiPostAnalysisActionSchemaVersion = 1;

const BrowserAiPostAnalysisActionPlanBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiPostAnalysisActionSchemaVersion),
  actionPlanId: BrowserAiPostAnalysisActionPlanIdSchema,
  createdAt: ActivityTimestampSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  aiAnalysisId: OptionalAnalysisIdSchema,
  policyDecision: BrowserPolicyDecisionSchema,
  policyDecisionAuditRefs: Schema.Array(BrowserPolicyDecisionAuditRefSchema),
  parentRuleRefs: Schema.Array(BrowserParentRuleRefSchema),
  actionLabels: ActionLabelsSchema,
  trigger: BrowserAiPostAnalysisActionTriggerSchema,
  timing: BrowserAiPostAnalysisActionTimingSchema,
  childAlreadyEngaged: Schema.Boolean,
  deliveryState: BrowserAiPostAnalysisDeliveryStateSchema,
  adapterProofRef: OptionalAdapterProofRefSchema,
  rememberUntil: OptionalRememberUntilSchema,
  actionAuditRefs: ActionAuditRefsSchema,
  realtimeBlockClaimed: Schema.Boolean,
  browserRuntimeMutationClaimed: Schema.Boolean,
  directEnforcementClaimed: Schema.Boolean,
});
export const BrowserAiPostAnalysisActionPlanSchema = withParser(
  BrowserAiPostAnalysisActionPlanBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiPostAnalysisActionPlanIsConsistent(value) ||
        'Expected post-analysis action plan to be proof-backed and not claim real-time enforcement'
    )
  )
);

export const decodeBrowserAiPostAnalysisActionPlan = Schema.decodeUnknownSync(BrowserAiPostAnalysisActionPlanSchema);

export type BrowserAiPostAnalysisActionPlan = Infer<typeof BrowserAiPostAnalysisActionPlanSchema>;

function browserAiPostAnalysisActionPlanIsConsistent(value: Infer<typeof BrowserAiPostAnalysisActionPlanBaseSchema>) {
  if (postAnalysisActionAuthorityCreepClaimed(value)) {
    return false;
  }
  if (!childEngagementTimingIsHonest(value)) {
    return false;
  }
  if (!deliveredActionsHaveAdapterProof(value.actionLabels, value.deliveryState, value.adapterProofRef)) {
    return false;
  }
  if (!rememberedActionsHaveExpiry(value.actionLabels, value.rememberUntil)) {
    return false;
  }
  return decisionOutcomeMatchesActionLabels(value.policyDecision.outcome, value.actionLabels);
}

function postAnalysisActionAuthorityCreepClaimed(value: Infer<typeof BrowserAiPostAnalysisActionPlanBaseSchema>) {
  return value.realtimeBlockClaimed || value.browserRuntimeMutationClaimed || value.directEnforcementClaimed;
}

function childEngagementTimingIsHonest(value: Infer<typeof BrowserAiPostAnalysisActionPlanBaseSchema>) {
  if (!value.childAlreadyEngaged) {
    return true;
  }
  return value.timing !== 'before_playback';
}

function deliveredActionsHaveAdapterProof(
  actionLabels: ReadonlyArray<BrowserAiPostAnalysisActionLabel>,
  deliveryState: BrowserAiPostAnalysisDeliveryState,
  adapterProofRef: Infer<typeof OptionalAdapterProofRefSchema>
) {
  if (deliveryState !== 'delivered') {
    return true;
  }
  if (!actionNeedsAdapterProof(actionLabels)) {
    return true;
  }
  return adapterProofRef !== null;
}

function actionNeedsAdapterProof(actionLabels: ReadonlyArray<BrowserAiPostAnalysisActionLabel>) {
  return actionLabels.some(
    (label) =>
      label === 'warning_shown_after_review' ||
      label === 'playback_stopped_after_review' ||
      label === 'parent_approval_requested_after_review' ||
      label === 'future_visits_blocked'
  );
}

function rememberedActionsHaveExpiry(
  actionLabels: ReadonlyArray<BrowserAiPostAnalysisActionLabel>,
  rememberUntil: Infer<typeof OptionalRememberUntilSchema>
) {
  return !actionLabels.includes('remembered_with_expiry') || rememberUntil !== null;
}

function decisionOutcomeMatchesActionLabels(
  outcome: BrowserPolicyDecisionOutcome,
  actionLabels: ReadonlyArray<BrowserAiPostAnalysisActionLabel>
) {
  switch (outcome) {
    case 'allow':
      return hasAnyAction(actionLabels, 'continue_allowed', 'background_reviewed', 'remembered_with_expiry');
    case 'warn':
      return actionLabels.includes('warning_shown_after_review');
    case 'ask_parent':
      return actionLabels.includes('parent_approval_requested_after_review');
    case 'time_limit':
      return hasAnyAction(actionLabels, 'warning_shown_after_review', 'manual_required');
    case 'block':
      return hasAnyAction(actionLabels, 'playback_stopped_after_review', 'future_visits_blocked');
    case 'unknown':
      return hasAnyAction(actionLabels, 'manual_required', 'parent_approval_requested_after_review');
  }
}

function hasAnyAction(
  actionLabels: ReadonlyArray<BrowserAiPostAnalysisActionLabel>,
  first: BrowserAiPostAnalysisActionLabel,
  second: BrowserAiPostAnalysisActionLabel,
  third?: BrowserAiPostAnalysisActionLabel
) {
  return [first, second, third].some((label) => label !== undefined && actionLabels.includes(label));
}
