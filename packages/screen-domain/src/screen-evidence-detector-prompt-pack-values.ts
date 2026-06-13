import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ScreenDetectorPromptPackSchemaVersion = 1;

export const ScreenDetectorPromptPackIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenDetectorPromptPackId')
);
export const ScreenDetectorPromptPackVersionSchema = withParser(
  brandedNonEmptyStringSchema('ScreenDetectorPromptPackVersion')
);
export const ScreenDetectorPromptHashRefSchema = withParser(
  brandedNonEmptyStringSchema('ScreenDetectorPromptHashRef')
);

export const ScreenDetectorIdSchema = withParser(
  Schema.Literal(
    'socialVideo',
    'chatMessaging',
    'browserGame',
    'schoolProductivity',
    'bypassTool',
    'adultContent',
    'violenceSafety',
    'shoppingPayment',
    'signupIdentity'
  )
);
export const ScreenDetectorPromptPackStatusSchema = withParser(Schema.Literal('draft', 'active', 'retired'));
export const ScreenDetectorInputFieldSchema = withParser(
  Schema.Literal(
    'ocrSnippets',
    'visibleCategoryCandidates',
    'riskSignals',
    'sourceEvidenceRefs',
    'safeImageCropRef',
    'windowTitleCategory',
    'appEvidenceRef',
    'browserEvidenceRef',
    'networkDigestRef'
  )
);
export const ScreenDetectorOutputFieldSchema = withParser(
  Schema.Literal(
    'detectorId',
    'categoryCandidates',
    'riskSignals',
    'confidence',
    'uncertaintyReasons',
    'evidenceRefs',
    'redactionNotes',
    'childSafeSummary'
  )
);
export const ScreenDetectorForbiddenOutputFieldSchema = withParser(
  Schema.Literal(
    'privateMessageText',
    'personName',
    'credentialText',
    'fullOcrText',
    'rawScreenshotRef',
    'rawPromptText',
    'accountIdentifier',
    'addressOrPhone'
  )
);
export const ScreenDetectorPromptPackDegradedStateSchema = withParser(
  Schema.Literal('detectorMissing', 'duplicateDetector', 'privacyUnsafe', 'schemaMismatch')
);

export const ScreenDetectorRequiredIds = [
  'socialVideo',
  'chatMessaging',
  'browserGame',
  'schoolProductivity',
  'bypassTool',
  'adultContent',
  'violenceSafety',
  'shoppingPayment',
  'signupIdentity',
] as const;

export type ScreenDetectorId = Infer<typeof ScreenDetectorIdSchema>;

