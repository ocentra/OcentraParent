import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserUrlAiAnalysisResultSchema } from './browser-ai-analysis-schemas';
import { BrowserAiMemoryCacheEntryIdSchema } from './browser-ai-memory-cache-store-values';
import { BrowserKnowledgeGraphRefSchema } from './browser-ai-knowledge-graph-values';
import { BrowserPolicyDecisionSchema } from './browser-ai-policy-evaluator-schemas';
import { BrowserAiPostAnalysisActionPlanSchema } from './browser-ai-post-analysis-action-schemas';
import { BrowserAiChildUxSnapshotSchema } from './browser-ai-child-ux-schemas';
import { nonEmptyArraySchema } from './browser-ai-schema-shared';
import {
  browserAiParentExplanationBundleIsConsistent,
  browserAiParentExplanationPrimaryTokenForState as browserAiParentExplanationPrimaryTokenForStateRule,
} from './browser-ai-parent-explanation-rules';
import {
  BrowserAiParentExplanationAuditRefSchema,
  BrowserAiParentExplanationIdSchema,
  type BrowserAiParentExplanationState,
  BrowserAiParentExplanationSectionSchema,
  BrowserAiParentExplanationStateSchema,
  BrowserAiParentExplanationTextToken,
  BrowserAiParentExplanationTextTokenSchema,
} from '@ocentra-parent/schema-domain/browser-ai-parent-explanation-values';

export {
  BrowserAiParentExplanationAuditRefSchema,
  BrowserAiParentExplanationIdSchema,
  BrowserAiParentExplanationSectionSchema,
  BrowserAiParentExplanationStateSchema,
  BrowserAiParentExplanationTextToken,
  BrowserAiParentExplanationTextTokenSchema,
};

const EvidenceIdsSchema = nonEmptyArraySchema(
  ActivityEvidenceIdSchema,
  'Expected parent explanation evidence ids'
);
const ExplanationSectionsSchema = nonEmptyArraySchema(
  BrowserAiParentExplanationSectionSchema,
  'Expected parent explanation sections'
);
const ExplanationAuditRefsSchema = nonEmptyArraySchema(
  BrowserAiParentExplanationAuditRefSchema,
  'Expected parent explanation audit refs'
);

export const BrowserAiParentExplanationSchemaVersion = 1;

const BrowserAiParentExplanationBundleBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiParentExplanationSchemaVersion),
  explanationId: BrowserAiParentExplanationIdSchema,
  createdAt: ActivityTimestampSchema,
  state: BrowserAiParentExplanationStateSchema,
  titleTextToken: BrowserAiParentExplanationTextTokenSchema,
  summaryTextToken: BrowserAiParentExplanationTextTokenSchema,
  sections: ExplanationSectionsSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  aiAnalysis: BrowserUrlAiAnalysisResultSchema,
  policyDecision: BrowserPolicyDecisionSchema,
  postAnalysisActionPlan: BrowserAiPostAnalysisActionPlanSchema,
  childUxSnapshot: BrowserAiChildUxSnapshotSchema,
  memoryCacheEntryIds: Schema.Array(BrowserAiMemoryCacheEntryIdSchema),
  knowledgeGraphRefs: Schema.Array(BrowserKnowledgeGraphRefSchema),
  explanationAuditRefs: ExplanationAuditRefsSchema,
  evidenceVisible: Schema.Boolean,
  modelRuntimeVisible: Schema.Boolean,
  promptVersionVisible: Schema.Boolean,
  policyRuleVisible: Schema.Boolean,
  actionVisible: Schema.Boolean,
  memoryCacheVisible: Schema.Boolean,
  childExperienceVisible: Schema.Boolean,
  childSawPageVisible: Schema.Boolean,
  degradedStateVisible: Schema.Boolean,
  manualFallbackVisible: Schema.Boolean,
  auditTrailVisible: Schema.Boolean,
  rawPageContentIncluded: Schema.Boolean,
  rawPromptTextIncluded: Schema.Boolean,
  portalEvaluatedClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  directEnforcementClaimed: Schema.Boolean,
});

export const BrowserAiParentExplanationBundleSchema = withParser(
  BrowserAiParentExplanationBundleBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiParentExplanationBundleIsConsistent(value) ||
        'Expected parent explanation to be evidence-linked, audit-visible, and non-authoritative'
    )
  )
);

export const decodeBrowserAiParentExplanationBundle = Schema.decodeUnknownSync(BrowserAiParentExplanationBundleSchema);

export type BrowserAiParentExplanationBundle = Infer<typeof BrowserAiParentExplanationBundleSchema>;

export function browserAiParentExplanationPrimaryTokenForState(
  state: BrowserAiParentExplanationState
): BrowserAiParentExplanationTextToken {
  return browserAiParentExplanationPrimaryTokenForStateRule(state);
}
