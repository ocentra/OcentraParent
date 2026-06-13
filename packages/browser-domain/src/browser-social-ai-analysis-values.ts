import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema } from '@ocentra-parent/evidence-domain/primitives';
import { BrowserAiModelRuntimeRefSchema } from './browser-ai-analysis-schemas';
import { BrowserAiPromptTemplateIdSchema, BrowserAiPromptTemplateVersionSchema } from './browser-ai-analysis-values';

export const BrowserSocialAiAnalysisSchemaVersion = 1;

export const NonEmptySocialAiTextSchema = NonEmptyStringSchema;
export const OptionalSocialAiTextSchema = Schema.Union(NonEmptySocialAiTextSchema, Schema.Null);
export const OptionalSocialAiRuntimeRefSchema = Schema.Union(BrowserAiModelRuntimeRefSchema, Schema.Null);

export const BrowserSocialAiAnalysisRequestIdSchema = withParser(
  NonEmptySocialAiTextSchema.pipe(Schema.brand('BrowserSocialAiAnalysisRequestId'))
);
export const BrowserSocialAiAnalysisIdSchema = withParser(
  NonEmptySocialAiTextSchema.pipe(Schema.brand('BrowserSocialAiAnalysisId'))
);
export const BrowserSocialAiAnalysisTaskSchema = withParser(
  Schema.Literal(
    'signup-attempt-classification',
    'secondary-account-risk',
    'feed-risk-classification',
    'video-safety',
    'messaging-risk-summary',
    'platform-policy-support'
  )
);
export const BrowserSocialAiClassificationSchema = withParser(
  Schema.Literal(
    'new-account-attempt',
    'existing-account-login',
    'secondary-account-suspected',
    'feed-browsing',
    'short-video-browsing',
    'video-watch',
    'messaging',
    'upload-posting',
    'livestream',
    'educational-video',
    'entertainment-video',
    'risky-content',
    'unknown'
  )
);

export const SocialAiSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social AI source evidence ids')
);
export const SocialAiClassificationsSchema = Schema.Array(BrowserSocialAiClassificationSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one social AI classification')
);
export const SocialAiTextRefsSchema = Schema.Array(NonEmptySocialAiTextSchema);

const BrowserSocialAiPromptTemplateBaseSchema = Schema.Struct({
  promptTemplateId: BrowserAiPromptTemplateIdSchema,
  promptTemplateVersion: BrowserAiPromptTemplateVersionSchema,
  requestedTask: BrowserSocialAiAnalysisTaskSchema,
  allowedInputFieldRefs: SocialAiSourceEvidenceIdsSchema,
  rawPromptTextIncluded: Schema.Boolean,
  capturesRawPageBody: Schema.Boolean,
  capturesTranscriptText: Schema.Boolean,
  capturesMessageContent: Schema.Boolean,
  capturesFeedContent: Schema.Boolean,
  capturesScreenshot: Schema.Boolean,
});
export const BrowserSocialAiPromptTemplateSchema = withParser(
  BrowserSocialAiPromptTemplateBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialAiPromptTemplateIsConsistent(value) ||
        'Expected social AI prompt template to be versioned without raw prompt or content capture'
    )
  )
);

export type BrowserSocialAiAnalysisTask = Infer<typeof BrowserSocialAiAnalysisTaskSchema>;
export type BrowserSocialAiClassification = Infer<typeof BrowserSocialAiClassificationSchema>;
export type BrowserSocialAiPromptTemplate = Infer<typeof BrowserSocialAiPromptTemplateSchema>;

function browserSocialAiPromptTemplateIsConsistent(value: Infer<typeof BrowserSocialAiPromptTemplateBaseSchema>) {
  return (
    !value.rawPromptTextIncluded &&
    !value.capturesRawPageBody &&
    !value.capturesTranscriptText &&
    !value.capturesMessageContent &&
    !value.capturesFeedContent &&
    !value.capturesScreenshot
  );
}

