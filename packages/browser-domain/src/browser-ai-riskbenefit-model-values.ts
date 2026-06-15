import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyRiskBenefitValueText = Schema.String.pipe(Schema.minLength(1));

export const BrowserAiRiskBenefitAssessmentIdSchema = withParser(
  NonEmptyRiskBenefitValueText.pipe(Schema.brand('BrowserAiRiskBenefitAssessmentId'))
);
export const BrowserAiRiskBenefitTaxonomyVersionRefSchema = withParser(
  NonEmptyRiskBenefitValueText.pipe(Schema.brand('BrowserAiRiskBenefitTaxonomyVersionRef'))
);

export const BrowserAiStructuredContentCategorySchema = withParser(
  Schema.Literal(
    'education',
    'homework',
    'research',
    'news',
    'entertainment',
    'gaming',
    'music',
    'social',
    'shopping',
    'communication',
    'adult',
    'violence',
    'self-harm',
    'drugs-alcohol',
    'gambling',
    'hate-harassment',
    'weapons',
    'misinformation',
    'unknown'
  )
);
export const BrowserAiStructuredContentModifierSchema = withParser(
  Schema.Literal(
    'video',
    'short-video',
    'livestream',
    'comments-heavy',
    'recommendation-feed',
    'search-results',
    'download',
    'login-required',
    'dynamic-feed',
    'user-generated',
    'platform-restricted',
    'low-confidence'
  )
);
export const BrowserAiStructuredBenefitSignalSchema = withParser(
  Schema.Literal(
    'educational-value',
    'homework-relevance',
    'age-appropriate',
    'skill-building',
    'creativity',
    'physical-activity',
    'neutral',
    'unknown-benefit'
  )
);
export const BrowserAiStructuredRiskSignalSchema = withParser(
  Schema.Literal(
    'adult',
    'violence',
    'self-harm',
    'drugs-alcohol',
    'gambling',
    'hate-harassment',
    'misinformation',
    'social-manipulation',
    'distraction',
    'addictive-pattern',
    'privacy-risk',
    'unknown-risk'
  )
);
export const BrowserAiRiskBenefitSourceSupportSchema = withParser(
  Schema.Literal('evidence-backed', 'metadata-only', 'platform-label-only', 'model-derived', 'unknown')
);

export type BrowserAiStructuredContentCategory = Infer<typeof BrowserAiStructuredContentCategorySchema>;
export type BrowserAiStructuredRiskSignal = Infer<typeof BrowserAiStructuredRiskSignalSchema>;
