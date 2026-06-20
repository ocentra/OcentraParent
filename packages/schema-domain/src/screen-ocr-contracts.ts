import { type Infer, Schema, withParser } from './effect';

export const ScreenOcrRedactionSchemaVersion = 1;
export const ScreenOcrRedactionMaxSnippetLimit = 5;

export const ScreenOcrTextRetentionModeSchema = withParser(
  Schema.Literal('disabled', 'redactedSnippets', 'boundedSnippets')
);

export const ScreenOcrSensitiveTextKindSchema = withParser(
  Schema.Literal('credentialLikeText', 'emailLikeText', 'phoneLikeText')
);

export type ScreenOcrTextRetentionMode = Infer<typeof ScreenOcrTextRetentionModeSchema>;
export type ScreenOcrSensitiveTextKind = Infer<typeof ScreenOcrSensitiveTextKindSchema>;
