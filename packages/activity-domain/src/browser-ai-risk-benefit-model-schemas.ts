import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  BrowserAiConfidenceSchema,
  BrowserAiUncertaintyReasonSchema,
  BrowserUrlAiAnalysisIdSchema,
} from './browser-ai-analysis-values';
import {
  BrowserAiRiskBenefitAssessmentIdSchema,
  BrowserAiRiskBenefitSourceSupportSchema,
  BrowserAiRiskBenefitTaxonomyVersionRefSchema,
  BrowserAiStructuredBenefitSignalSchema,
  BrowserAiStructuredContentCategory,
  BrowserAiStructuredContentCategorySchema,
  BrowserAiStructuredContentModifierSchema,
  BrowserAiStructuredRiskSignal,
  BrowserAiStructuredRiskSignalSchema,
} from './browser-ai-risk-benefit-model-values';

const OptionalAnalysisIdSchema = Schema.Union(BrowserUrlAiAnalysisIdSchema, Schema.Null);
const EvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one risk/benefit evidence id')
);
const CategoryListSchema = Schema.Array(BrowserAiStructuredContentCategorySchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one content category')
);
const ModifierListSchema = Schema.Array(BrowserAiStructuredContentModifierSchema);
const BenefitSignalListSchema = Schema.Array(BrowserAiStructuredBenefitSignalSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one benefit signal')
);
const RiskSignalListSchema = Schema.Array(BrowserAiStructuredRiskSignalSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one risk signal')
);

export const BrowserAiRiskBenefitModelSchemaVersion = 1;

const BrowserAiRiskBenefitTaxonomyBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiRiskBenefitModelSchemaVersion),
  taxonomyVersionRef: BrowserAiRiskBenefitTaxonomyVersionRefSchema,
  publishedAt: ActivityTimestampSchema,
  categories: CategoryListSchema,
  modifiers: Schema.Array(BrowserAiStructuredContentModifierSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected at least one content modifier')
  ),
  benefitSignals: BenefitSignalListSchema,
  riskSignals: RiskSignalListSchema,
});
export const BrowserAiRiskBenefitTaxonomySchema = withParser(
  BrowserAiRiskBenefitTaxonomyBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiRiskBenefitTaxonomyIsConsistent(value) ||
        'Expected risk/benefit taxonomy to include unknown and explicit risk/benefit fallbacks'
    )
  )
);

const BrowserAiRiskBenefitAssessmentBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiRiskBenefitModelSchemaVersion),
  assessmentId: BrowserAiRiskBenefitAssessmentIdSchema,
  taxonomyVersionRef: BrowserAiRiskBenefitTaxonomyVersionRefSchema,
  assessedAt: ActivityTimestampSchema,
  sourceAnalysisId: OptionalAnalysisIdSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  primaryCategory: BrowserAiStructuredContentCategorySchema,
  secondaryCategories: CategoryListSchema,
  contentModifiers: ModifierListSchema,
  benefitSignals: BenefitSignalListSchema,
  riskSignals: RiskSignalListSchema,
  confidence: BrowserAiConfidenceSchema,
  uncertaintyReasons: Schema.Array(BrowserAiUncertaintyReasonSchema),
  sourceSupport: BrowserAiRiskBenefitSourceSupportSchema,
  platformLabelUsedAsAuthority: Schema.Boolean,
  finalPolicyActionClaimed: Schema.Boolean,
  enforcementActionClaimed: Schema.Boolean,
});
export const BrowserAiRiskBenefitAssessmentSchema = withParser(
  BrowserAiRiskBenefitAssessmentBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiRiskBenefitAssessmentIsConsistent(value) ||
        'Expected risk/benefit assessment to stay evidence-backed and candidate-only'
    )
  )
);

export const decodeBrowserAiRiskBenefitTaxonomy = Schema.decodeUnknownSync(BrowserAiRiskBenefitTaxonomySchema);
export const decodeBrowserAiRiskBenefitAssessment = Schema.decodeUnknownSync(BrowserAiRiskBenefitAssessmentSchema);

export type BrowserAiRiskBenefitTaxonomy = Infer<typeof BrowserAiRiskBenefitTaxonomySchema>;
export type BrowserAiRiskBenefitAssessment = Infer<typeof BrowserAiRiskBenefitAssessmentSchema>;

function browserAiRiskBenefitTaxonomyIsConsistent(value: Infer<typeof BrowserAiRiskBenefitTaxonomyBaseSchema>) {
  return (
    value.categories.includes('unknown') &&
    value.benefitSignals.includes('unknown-benefit') &&
    value.riskSignals.includes('unknown-risk')
  );
}

function browserAiRiskBenefitAssessmentIsConsistent(value: Infer<typeof BrowserAiRiskBenefitAssessmentBaseSchema>) {
  if (value.platformLabelUsedAsAuthority || value.finalPolicyActionClaimed || value.enforcementActionClaimed) {
    return false;
  }
  if (!lowConfidenceStateIsExplicit(value)) {
    return false;
  }
  if (!primaryCategoryHasMatchingEvidenceSignal(value)) {
    return false;
  }
  return value.sourceSupport !== 'platform-label-only' || value.confidence !== 'high';
}

function lowConfidenceStateIsExplicit(value: Infer<typeof BrowserAiRiskBenefitAssessmentBaseSchema>) {
  if (value.confidence !== 'low' && value.confidence !== 'unknown') {
    return true;
  }
  return value.contentModifiers.includes('low-confidence') && value.uncertaintyReasons.length > 0;
}

function primaryCategoryHasMatchingEvidenceSignal(value: Infer<typeof BrowserAiRiskBenefitAssessmentBaseSchema>) {
  if (value.primaryCategory === 'unknown') {
    return value.confidence !== 'high' && value.riskSignals.includes('unknown-risk');
  }
  if (educationalCategoryHasBenefitSignal(value.primaryCategory)) {
    return value.benefitSignals.some((signal) => signal !== 'neutral' && signal !== 'unknown-benefit');
  }
  const requiredRiskSignal = riskSignalForCategory(value.primaryCategory);
  return requiredRiskSignal === null || value.riskSignals.includes(requiredRiskSignal);
}

function educationalCategoryHasBenefitSignal(category: BrowserAiStructuredContentCategory) {
  return category === 'education' || category === 'homework' || category === 'research';
}

function riskSignalForCategory(category: BrowserAiStructuredContentCategory): BrowserAiStructuredRiskSignal | null {
  switch (category) {
    case 'adult':
      return 'adult';
    case 'violence':
      return 'violence';
    case 'self-harm':
      return 'self-harm';
    case 'drugs-alcohol':
      return 'drugs-alcohol';
    case 'gambling':
      return 'gambling';
    case 'hate-harassment':
      return 'hate-harassment';
    case 'misinformation':
      return 'misinformation';
    default:
      return null;
  }
}
