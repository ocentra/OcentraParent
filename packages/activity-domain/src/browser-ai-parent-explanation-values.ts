import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyParentExplanationText = Schema.String.pipe(Schema.minLength(1));

export const BrowserAiParentExplanationIdSchema = withParser(
  NonEmptyParentExplanationText.pipe(Schema.brand('BrowserAiParentExplanationId'))
);
export const BrowserAiParentExplanationAuditRefSchema = withParser(
  NonEmptyParentExplanationText.pipe(Schema.brand('BrowserAiParentExplanationAuditRef'))
);

export const BrowserAiParentExplanationStateSchema = withParser(
  Schema.Literal('ready', 'preview', 'degraded', 'manual_required', 'unavailable')
);
export const BrowserAiParentExplanationSectionSchema = withParser(
  Schema.Literal(
    'summary',
    'evidence',
    'ai-analysis',
    'policy-decision',
    'action-taken',
    'child-experience',
    'memory-cache',
    'knowledge-graph',
    'degraded-fallback',
    'audit'
  )
);
export const BrowserAiParentExplanationTextTokenSchema = withParser(
  Schema.Literal(
    'browser.parent.explanation.title',
    'browser.parent.explanation.summary',
    'browser.parent.explanation.evidence',
    'browser.parent.explanation.ai',
    'browser.parent.explanation.policy',
    'browser.parent.explanation.action',
    'browser.parent.explanation.childExperience',
    'browser.parent.explanation.degraded',
    'browser.parent.explanation.audit'
  )
);

export type BrowserAiParentExplanationSection = Infer<typeof BrowserAiParentExplanationSectionSchema>;
export type BrowserAiParentExplanationState = Infer<typeof BrowserAiParentExplanationStateSchema>;
export type BrowserAiParentExplanationTextToken = Infer<typeof BrowserAiParentExplanationTextTokenSchema>;

export const BrowserAiParentExplanationTextToken = {
  Title: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.title'),
  Summary: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.summary'),
  Evidence: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.evidence'),
  Ai: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.ai'),
  Policy: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.policy'),
  Action: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.action'),
  ChildExperience: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.childExperience'),
  Degraded: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.degraded'),
  Audit: BrowserAiParentExplanationTextTokenSchema.parse('browser.parent.explanation.audit'),
} as const;
