import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
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
  BrowserAiStructuredContentCategorySchema,
  BrowserAiStructuredContentModifierSchema,
  BrowserAiStructuredRiskSignalSchema,
} from '@ocentra-parent/schema-domain/browser-ai-riskbenefit-model-values';
import {
  browserAiRiskBenefitAssessmentIsConsistent,
  browserAiRiskBenefitTaxonomyIsConsistent,
} from './browser-ai-riskbenefit-model-rules';

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
