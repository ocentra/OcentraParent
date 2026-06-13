import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const BrowserAiPromptTemplateRegistryIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiPromptTemplateRegistryId')
);
export const BrowserAiPromptTemplateHashRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiPromptTemplateHashRef')
);
export const BrowserAiPromptTemplateChangeRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiPromptTemplateChangeRef')
);

export const BrowserAiPromptTemplateStatusSchema = withParser(
  Schema.Literal('draft', 'active', 'deprecated', 'retired', 'manual-required')
);
export const BrowserAiPromptTemplateChangeReasonSchema = withParser(
  Schema.Literal(
    'new-task',
    'input-field-change',
    'model-change',
    'policy-change',
    'risk-taxonomy-change',
    'security-fix'
  )
);
export const BrowserAiPromptTemplateSelectionStateSchema = withParser(
  Schema.Literal('selected', 'manual-required', 'unavailable')
);
export const BrowserAiPromptTemplateSelectionDegradedStateSchema = withParser(
  Schema.Literal(
    'none',
    'template-missing',
    'duplicate-active-version',
    'model-unsupported',
    'policy-version-unsupported',
    'template-retired',
    'manual-required'
  )
);

export type BrowserAiPromptTemplateStatus = Infer<typeof BrowserAiPromptTemplateStatusSchema>;
export type BrowserAiPromptTemplateSelectionDegradedState = Infer<
  typeof BrowserAiPromptTemplateSelectionDegradedStateSchema
>;

