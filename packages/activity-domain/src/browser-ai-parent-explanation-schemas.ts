import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { BrowserUrlAiAnalysisResultSchema } from './browser-ai-analysis-schemas';
import { BrowserAiMemoryCacheEntryIdSchema } from './browser-ai-memory-cache-store-values';
import { BrowserKnowledgeGraphRefSchema } from './browser-ai-knowledge-graph-values';
import { BrowserPolicyDecisionSchema } from './browser-ai-policy-evaluator-schemas';
import { BrowserAiPostAnalysisActionPlanSchema } from './browser-ai-post-analysis-action-schemas';
import { BrowserAiChildUxSnapshotSchema } from './browser-ai-child-ux-schemas';
import {
  BrowserAiParentExplanationAuditRefSchema,
  BrowserAiParentExplanationIdSchema,
  type BrowserAiParentExplanationSection,
  BrowserAiParentExplanationSectionSchema,
  type BrowserAiParentExplanationState,
  BrowserAiParentExplanationStateSchema,
  BrowserAiParentExplanationTextToken,
  BrowserAiParentExplanationTextTokenSchema,
} from './browser-ai-parent-explanation-values';

export {
  BrowserAiParentExplanationAuditRefSchema,
  BrowserAiParentExplanationIdSchema,
  BrowserAiParentExplanationSectionSchema,
  BrowserAiParentExplanationStateSchema,
  BrowserAiParentExplanationTextToken,
  BrowserAiParentExplanationTextTokenSchema,
} from './browser-ai-parent-explanation-values';

const EvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected parent explanation evidence ids')
);
const ExplanationSectionsSchema = Schema.Array(BrowserAiParentExplanationSectionSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected parent explanation sections')
);
const ExplanationAuditRefsSchema = Schema.Array(BrowserAiParentExplanationAuditRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected parent explanation audit refs')
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

function browserAiParentExplanationBundleIsConsistent(value: Infer<typeof BrowserAiParentExplanationBundleBaseSchema>) {
  if (parentExplanationClaimsAuthorityOrRawContent(value)) {
    return false;
  }
  if (!requiredExplanationSectionsArePresent(value.sections)) {
    return false;
  }
  if (!requiredVisibilityIsPresent(value)) {
    return false;
  }
  if (!degradedAndManualFallbackVisibilityIsHonest(value)) {
    return false;
  }
  if (!childExperienceVisibilityIsHonest(value)) {
    return false;
  }
  if (!linkedRecordsShareEvidenceAndAction(value)) {
    return false;
  }
  return stateMatchesExplanationReadiness(value.state, value);
}

function parentExplanationClaimsAuthorityOrRawContent(value: Infer<typeof BrowserAiParentExplanationBundleBaseSchema>) {
  return (
    value.rawPageContentIncluded ||
    value.rawPromptTextIncluded ||
    value.portalEvaluatedClaimed ||
    value.policyAuthorityClaimed ||
    value.directEnforcementClaimed
  );
}

function requiredExplanationSectionsArePresent(sections: ReadonlyArray<BrowserAiParentExplanationSection>) {
  return (
    hasSections(sections, 'summary', 'evidence', 'ai-analysis') &&
    hasSections(sections, 'policy-decision', 'action-taken', 'audit')
  );
}

function requiredVisibilityIsPresent(value: Infer<typeof BrowserAiParentExplanationBundleBaseSchema>) {
  return (
    value.evidenceVisible &&
    value.modelRuntimeVisible &&
    value.promptVersionVisible &&
    value.policyRuleVisible &&
    value.actionVisible &&
    value.childExperienceVisible &&
    value.auditTrailVisible
  );
}

function degradedAndManualFallbackVisibilityIsHonest(value: Infer<typeof BrowserAiParentExplanationBundleBaseSchema>) {
  const hasDegradedState =
    value.aiAnalysis.degradedState !== 'none' ||
    value.policyDecision.fallbackUsed ||
    value.childUxSnapshot.state === 'unavailable';
  const hasManualState =
    value.policyDecision.outcome === 'unknown' ||
    value.childUxSnapshot.state === 'manual_required' ||
    value.childUxSnapshot.state === 'unavailable';

  if (hasDegradedState && !value.degradedStateVisible) {
    return false;
  }
  if (hasManualState && !value.manualFallbackVisible) {
    return false;
  }
  return true;
}

function childExperienceVisibilityIsHonest(value: Infer<typeof BrowserAiParentExplanationBundleBaseSchema>) {
  if (!value.postAnalysisActionPlan.childAlreadyEngaged) {
    return true;
  }
  return value.childSawPageVisible;
}

function linkedRecordsShareEvidenceAndAction(value: Infer<typeof BrowserAiParentExplanationBundleBaseSchema>) {
  if (!containsAll(value.sourceEvidenceIds, value.aiAnalysis.sourceEvidenceIds)) {
    return false;
  }
  if (!containsAll(value.sourceEvidenceIds, value.policyDecision.sourceEvidenceIds)) {
    return false;
  }
  if (!containsAll(value.sourceEvidenceIds, value.postAnalysisActionPlan.sourceEvidenceIds)) {
    return false;
  }
  if (value.childUxSnapshot.postAnalysisActionPlan === null) {
    return true;
  }
  return value.childUxSnapshot.postAnalysisActionPlan.actionPlanId === value.postAnalysisActionPlan.actionPlanId;
}

function stateMatchesExplanationReadiness(
  state: BrowserAiParentExplanationState,
  value: Infer<typeof BrowserAiParentExplanationBundleBaseSchema>
) {
  switch (state) {
    case 'ready':
      return value.aiAnalysis.degradedState === 'none' && !value.policyDecision.fallbackUsed;
    case 'preview':
      return value.policyDecision.evaluatorMode === 'dry_run';
    case 'degraded':
      return value.degradedStateVisible;
    case 'manual_required':
      return value.manualFallbackVisible;
    case 'unavailable':
      return value.degradedStateVisible && value.manualFallbackVisible;
  }
  return false;
}

function containsAll(source: ReadonlyArray<unknown>, expected: ReadonlyArray<unknown>) {
  return expected.every((item) => source.includes(item));
}

function hasSections(
  sections: ReadonlyArray<BrowserAiParentExplanationSection>,
  first: BrowserAiParentExplanationSection,
  second: BrowserAiParentExplanationSection,
  third: BrowserAiParentExplanationSection
) {
  return sections.includes(first) && sections.includes(second) && sections.includes(third);
}

export function browserAiParentExplanationPrimaryTokenForState(
  state: BrowserAiParentExplanationState
): BrowserAiParentExplanationTextToken {
  if (state === 'degraded' || state === 'manual_required' || state === 'unavailable') {
    return BrowserAiParentExplanationTextToken.Degraded;
  }
  return BrowserAiParentExplanationTextToken.Title;
}
