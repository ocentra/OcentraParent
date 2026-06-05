import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyRouterText = Schema.String.pipe(Schema.minLength(1));

export const ScreenIntelligenceRouterSchemaVersion = 1;

export const ScreenIntelligenceRouteRequestIdSchema = NonEmptyRouterText.pipe(
  Schema.brand('ScreenIntelligenceRouteRequestId')
);
export const ScreenIntelligenceRouteDecisionIdSchema = NonEmptyRouterText.pipe(
  Schema.brand('ScreenIntelligenceRouteDecisionId')
);

export const ScreenIntelligenceSurfaceKindSchema = withParser(
  Schema.Literal(
    'managedBrowser',
    'nativeApp',
    'nativeGame',
    'launcher',
    'unknownProcess',
    'networkOnly',
    'sessionOnly'
  )
);

export const ScreenIntelligencePolicyQuestionSchema = withParser(
  Schema.Literal('categoryReview', 'riskReview', 'timeLimitEligibility', 'accountOrPayment', 'bypassCheck', 'unknown')
);

export const ScreenIntelligenceEvidenceKindSchema = withParser(
  Schema.Literal(
    'managedBrowserStructured',
    'appForeground',
    'gameForeground',
    'networkDigest',
    'sessionState',
    'previousScreenSummary',
    'parentRule'
  )
);

export const ScreenIntelligenceSensitivityFlagSchema = withParser(
  Schema.Literal(
    'lowSensitivity',
    'credentialLikeText',
    'privateMessageLikely',
    'protectedSurfaceLikely',
    'childIdentityLikely',
    'paymentOrSignupLikely'
  )
);

export const ScreenIntelligenceRouteSelectionSchema = withParser(
  Schema.Literal(
    'noScreenNeeded',
    'managedBrowserStructuredExtraction',
    'managedBrowserScreenshot',
    'nativeActiveWindowCapture',
    'selectedWindowCapture',
    'manualRequired',
    'unavailable'
  )
);

export const ScreenIntelligenceNextStepSchema = withParser(
  Schema.Literal(
    'deterministicSummary',
    'structuredExtraction',
    'encryptedImageQueue',
    'manualReviewRequired',
    'unavailable'
  )
);

export const ScreenIntelligenceRouteReasonSchema = withParser(
  Schema.Literal(
    'existingEvidenceAnswered',
    'managedBrowserStructuredFirst',
    'managedBrowserStructuredExhausted',
    'nativeSurfaceCaptureAllowed',
    'selectedWindowCaptureAllowed',
    'captureDisabledByParent',
    'protectedOrSensitiveSurface',
    'capabilityUnavailable'
  )
);

export const ScreenIntelligenceDegradedStateSchema = withParser(
  Schema.Literal(
    'captureDisabled',
    'capabilityUnavailable',
    'protectedSurface',
    'structuredEvidenceMissing',
    'manualReviewRequired'
  )
);

export type ScreenIntelligenceSurfaceKind = Infer<typeof ScreenIntelligenceSurfaceKindSchema>;
export type ScreenIntelligencePolicyQuestion = Infer<typeof ScreenIntelligencePolicyQuestionSchema>;
export type ScreenIntelligenceEvidenceKind = Infer<typeof ScreenIntelligenceEvidenceKindSchema>;
export type ScreenIntelligenceSensitivityFlag = Infer<typeof ScreenIntelligenceSensitivityFlagSchema>;
export type ScreenIntelligenceRouteSelection = Infer<typeof ScreenIntelligenceRouteSelectionSchema>;
