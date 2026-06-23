import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const BrowserGameAiAnalysisSchemaVersionSchema = withParser(Schema.Literal('browser-game-ai-analysis-contract'));

export const BrowserGameAiAnalysisRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameAiAnalysisRequestId')
);

export const BrowserGameAiAnalysisResultIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameAiAnalysisResultId')
);

export const BrowserGameAiModelRuntimeRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameAiModelRuntimeRef')
);

export const BrowserGameAiPromptTemplateVersionSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameAiPromptTemplateVersion')
);

export const BrowserGameAiSummaryRefSchema = withParser(brandedNonEmptyStringSchema('BrowserGameAiSummaryRef'));

export const BrowserGameAiTaskSchema = withParser(
  Schema.Literal(
    'game-classification',
    'educational-game-check',
    'risk-classification',
    'cloud-gaming-detection',
    'ugc-game-risk',
    'purchase-risk',
    'policy-support'
  )
);

export const BrowserGameAiCustodyLabelSchema = withParser(
  Schema.Literal('managed-browser', 'hidden-analysis-profile', 'unmanaged-browser-bypass', 'manual-required')
);

export const BrowserGameAiSurfaceKindSchema = withParser(
  Schema.Literal(
    'browser-game',
    'game-portal',
    'educational-game',
    'cloud-gaming',
    'ugc-multiplayer-game',
    'account-or-purchase-flow',
    'unblocked-game-site',
    'unknown'
  )
);

export const BrowserGameAiModifierSchema = withParser(
  Schema.Literal(
    'webgl',
    'canvas',
    'iframe-embedded',
    'fullscreen',
    'pointer-lock',
    'gamepad',
    'multiplayer',
    'chat',
    'purchase',
    'cloud-streaming',
    'school-context',
    'unknown'
  )
);

export const BrowserGameAiBenefitSignalSchema = withParser(
  Schema.Literal(
    'educational-value',
    'homework-relevance',
    'skill-building',
    'creativity',
    'problem-solving',
    'unknown-benefit'
  )
);

export const BrowserGameAiRiskSignalSchema = withParser(
  Schema.Literal(
    'violence',
    'horror',
    'adult-theme',
    'gambling',
    'addiction-loop',
    'multiplayer-contact',
    'chat-risk',
    'purchase-risk',
    'loot-box-risk',
    'user-generated-content-risk',
    'privacy-risk',
    'unknown-risk'
  )
);

export const BrowserGameAiRecommendedPolicyInputSchema = withParser(
  Schema.Literal(
    'allow-candidate',
    'warn-candidate',
    'parent-review-candidate',
    'block-candidate',
    'time-limit-candidate',
    'hold-for-classification-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);

export const BrowserGameAiConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));
export const BrowserGameAiDegradedStateSchema = withParser(Schema.Literal('none', 'degraded', 'manual-required'));

export const BrowserGameAiUncertaintyReasonSchema = withParser(
  Schema.Literal(
    'low-confidence',
    'missing-runtime-signal',
    'missing-metadata',
    'conflicting-evidence',
    'model-unavailable',
    'manual-required'
  )
);

export const BrowserGameAiEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game AI evidence refs')
);

export const BrowserGameAiEvidenceRefListSchema = Schema.Array(ParentEvidenceReferenceIdSchema);
export const BrowserGameAiModifiersSchema = Schema.Array(BrowserGameAiModifierSchema);
export const BrowserGameAiBenefitSignalsSchema = Schema.Array(BrowserGameAiBenefitSignalSchema);
export const BrowserGameAiRiskSignalsSchema = Schema.Array(BrowserGameAiRiskSignalSchema);
export const BrowserGameAiUncertaintyReasonsSchema = Schema.Array(BrowserGameAiUncertaintyReasonSchema);

export type BrowserGameAiBenefitSignal = Infer<typeof BrowserGameAiBenefitSignalSchema>;
export type BrowserGameAiConfidence = Infer<typeof BrowserGameAiConfidenceSchema>;
export type BrowserGameAiCustodyLabel = Infer<typeof BrowserGameAiCustodyLabelSchema>;
export type BrowserGameAiDegradedState = Infer<typeof BrowserGameAiDegradedStateSchema>;
export type BrowserGameAiModifier = Infer<typeof BrowserGameAiModifierSchema>;
export type BrowserGameAiRecommendedPolicyInput = Infer<typeof BrowserGameAiRecommendedPolicyInputSchema>;
export type BrowserGameAiRiskSignal = Infer<typeof BrowserGameAiRiskSignalSchema>;
export type BrowserGameAiSurfaceKind = Infer<typeof BrowserGameAiSurfaceKindSchema>;
export type BrowserGameAiTask = Infer<typeof BrowserGameAiTaskSchema>;
export type BrowserGameAiUncertaintyReason = Infer<typeof BrowserGameAiUncertaintyReasonSchema>;
