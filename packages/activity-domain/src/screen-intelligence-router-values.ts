import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyScreenRouterText = Schema.String.pipe(Schema.minLength(1));
const BoundedPolicyQuestionText = NonEmptyScreenRouterText.pipe(
  Schema.filter((value) => value.length <= 240 || 'Expected screen policy question text within 240 characters')
);
const BoundedStructuredText = NonEmptyScreenRouterText.pipe(
  Schema.filter((value) => value.length <= 480 || 'Expected managed browser structured text within 480 characters')
);

export const ScreenIntelligenceRouterSchemaVersion = 1;
export const ScreenManagedBrowserStructuredTextLimit = 480;

export const ScreenIntelligenceRouteIdSchema = withParser(
  NonEmptyScreenRouterText.pipe(Schema.brand('ScreenIntelligenceRouteId'))
);
export const ScreenIntelligenceRouteRequestIdSchema = withParser(
  NonEmptyScreenRouterText.pipe(Schema.brand('ScreenIntelligenceRouteRequestId'))
);
export const ScreenStructuredExtractionIdSchema = withParser(
  NonEmptyScreenRouterText.pipe(Schema.brand('ScreenStructuredExtractionId'))
);
export const ScreenStructuredEvidenceSummarySchema = withParser(
  BoundedStructuredText.pipe(Schema.brand('ScreenStructuredEvidenceSummary'))
);
export const ScreenPolicyQuestionTextSchema = withParser(
  BoundedPolicyQuestionText.pipe(Schema.brand('ScreenPolicyQuestionText'))
);

export const ScreenIntelligenceSourceKindSchema = withParser(
  Schema.Literal(
    'managedBrowser',
    'nativeApp',
    'nativeGame',
    'launcher',
    'unknownProcess',
    'networkOrSessionSummary',
    'screenAdjacentEvidence'
  )
);

export const ScreenIntelligenceRouteKindSchema = withParser(
  Schema.Literal(
    'noScreenNeeded',
    'managedBrowserStructuredExtraction',
    'screenCaptureActiveWindow',
    'screenCaptureSelectedWindow',
    'manualRequired',
    'unavailable'
  )
);

export const ScreenIntelligencePolicySensitivitySchema = withParser(
  Schema.Literal('ordinary', 'private', 'credentialRisk', 'protectedSurface')
);

export const ScreenStructuredExtractionStateSchema = withParser(
  Schema.Literal('enoughForPolicy', 'needsScreenshot', 'unavailable')
);

export const ScreenStructuredExtractionRedactionStateSchema = withParser(
  Schema.Literal('none', 'privateTextRedacted', 'overflowRedacted', 'protectedContentSkipped')
);

export type ScreenIntelligenceSourceKind = Infer<typeof ScreenIntelligenceSourceKindSchema>;
export type ScreenIntelligenceRouteKind = Infer<typeof ScreenIntelligenceRouteKindSchema>;
export type ScreenIntelligencePolicySensitivity = Infer<typeof ScreenIntelligencePolicySensitivitySchema>;
