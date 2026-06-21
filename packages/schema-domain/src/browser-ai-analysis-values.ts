import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const BrowserUrlAiAnalysisRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserUrlAiAnalysisRequestId')
);
export const BrowserUrlAiAnalysisIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserUrlAiAnalysisId')
);
export const BrowserAiPromptTemplateIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiPromptTemplateId')
);
export const BrowserAiPromptTemplateVersionSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiPromptTemplateVersion')
);
export const BrowserAiModelRuntimeRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiModelRuntimeRef')
);
export const BrowserPolicyVersionRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserPolicyVersionRef')
);
export const BrowserParentRuleRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserParentRuleRef')
);
export const BrowserScheduleContextRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserScheduleContextRef')
);
export const BrowserEvidenceBackedContentRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserEvidenceBackedContentRef')
);
export const BrowserKnowledgeGraphRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserKnowledgeGraphRef')
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

