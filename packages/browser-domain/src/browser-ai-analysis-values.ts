import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyBrowserAiValueText = Schema.String.pipe(Schema.minLength(1));

export const BrowserUrlAiAnalysisRequestIdSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserUrlAiAnalysisRequestId'))
);
export const BrowserUrlAiAnalysisIdSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserUrlAiAnalysisId'))
);
export const BrowserAiPromptTemplateIdSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserAiPromptTemplateId'))
);
export const BrowserAiPromptTemplateVersionSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserAiPromptTemplateVersion'))
);
export const BrowserAiModelRuntimeRefSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserAiModelRuntimeRef'))
);
export const BrowserPolicyVersionRefSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserPolicyVersionRef'))
);
export const BrowserParentRuleRefSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserParentRuleRef'))
);
export const BrowserScheduleContextRefSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserScheduleContextRef'))
);
export const BrowserEvidenceBackedContentRefSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserEvidenceBackedContentRef'))
);
export const BrowserKnowledgeGraphRefSchema = withParser(
  NonEmptyBrowserAiValueText.pipe(Schema.brand('BrowserKnowledgeGraphRef'))
);

export const BrowserAiRequestedTaskSchema = withParser(
  Schema.Literal(
    'url-safety',
    'video-safety',
    'educational-relevance',
    'category-classification',
    'parent-summary',
    'policy-decision-support'
  )
);
export const BrowserAiModelRuntimePreferenceSchema = withParser(
  Schema.Literal('local-only', 'local-preferred', 'parent-approved-remote', 'manual-required')
);
export const BrowserContentCategorySchema = withParser(
  Schema.Literal(
    'educational',
    'entertainment',
    'social',
    'gaming',
    'shopping',
    'adult',
    'violence',
    'misinformation',
    'unknown'
  )
);
export const BrowserContentModifierSchema = withParser(
  Schema.Literal(
    'age-restricted',
    'live',
    'short-form',
    'autoplay',
    'user-generated',
    'ads-heavy',
    'comments-enabled',
    'login-required',
    'transcript-unavailable',
    'metadata-only'
  )
);
export const BrowserBenefitSignalSchema = withParser(
  Schema.Literal('curriculum-aligned', 'homework-help', 'skill-building', 'creativity', 'neutral', 'unknown-benefit')
);
export const BrowserRiskSignalSchema = withParser(
  Schema.Literal(
    'mature-content',
    'addictive-design',
    'unsafe-chat',
    'misinformation',
    'purchase-risk',
    'privacy-risk',
    'unknown-risk'
  )
);
export const BrowserAiConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));
export const BrowserAiUncertaintyReasonSchema = withParser(
  Schema.Literal(
    'model-unavailable',
    'transcript-missing',
    'metadata-missing',
    'hidden-load-unavailable',
    'language-unsupported',
    'timeout',
    'source-stale',
    'low-confidence'
  )
);
export const BrowserAiRecommendedPolicyInputSchema = withParser(
  Schema.Literal(
    'allow-candidate',
    'warn-candidate',
    'limit-candidate',
    'parent-review-candidate',
    'block-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);
export const BrowserAiDegradedStateSchema = withParser(
  Schema.Literal('none', 'degraded', 'manual-required', 'unavailable')
);
export const BrowserVideoKindSchema = withParser(
  Schema.Literal('video', 'short-video', 'livestream', 'channel', 'playlist', 'non-video', 'unknown')
);

export type BrowserContentCategory = Infer<typeof BrowserContentCategorySchema>;
export type BrowserContentModifier = Infer<typeof BrowserContentModifierSchema>;
