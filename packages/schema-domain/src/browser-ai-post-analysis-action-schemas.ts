import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserUrlAiAnalysisIdSchema, BrowserParentRuleRefSchema } from './browser-ai-analysis-values';
import { nonEmptyArraySchema, optionalSchema } from './browser-ai-schema-shared';
import {
  BrowserPolicyAdapterProofRefSchema,
  BrowserPolicyDecisionAuditRefSchema,
} from './browser-ai-policy-evaluator-values';
import { BrowserPolicyDecisionSchema } from './browser-ai-policy-evaluator-schemas';
import { browserAiPostAnalysisActionPlanIsConsistent } from './browser-ai-post-analysis-action-rules';
import {
  BrowserAiPostAnalysisActionAuditRefSchema,
  BrowserAiPostAnalysisActionLabelSchema,
  BrowserAiPostAnalysisActionPlanIdSchema,
  BrowserAiPostAnalysisActionTimingSchema,
  BrowserAiPostAnalysisActionTriggerSchema,
  BrowserAiPostAnalysisDeliveryStateSchema,
} from './browser-ai-post-analysis-action-values';

export {
  BrowserAiPostAnalysisActionLabelSchema,
  BrowserAiPostAnalysisActionTimingSchema,
  BrowserAiPostAnalysisActionTriggerSchema,
  BrowserAiPostAnalysisDeliveryStateSchema,
};

const EvidenceIdsSchema = nonEmptyArraySchema(
  ActivityEvidenceIdSchema,
  'Expected post-analysis action evidence ids'
);
const ActionLabelsSchema = nonEmptyArraySchema(
  BrowserAiPostAnalysisActionLabelSchema,
  'Expected at least one post-analysis action label'
);
const ActionAuditRefsSchema = nonEmptyArraySchema(
  BrowserAiPostAnalysisActionAuditRefSchema,
  'Expected post-analysis action audit refs'
);
const OptionalAnalysisIdSchema = optionalSchema(BrowserUrlAiAnalysisIdSchema);
const OptionalAdapterProofRefSchema = optionalSchema(BrowserPolicyAdapterProofRefSchema);
const OptionalRememberUntilSchema = optionalSchema(ActivityTimestampSchema);

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
