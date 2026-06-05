import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityTimestampSchema } from './primitives';
import {
  ScreenCaptureScopeSchema,
  ScreenCapabilityStatusSchema,
  ScreenVisibleCategorySchema,
} from './screen-evidence-states';
import {
  ScreenEvidenceConfidenceSchema,
  ScreenEvidenceSchemaVersion,
  ScreenEvidenceSummaryTextSchema,
} from './screen-evidence-primitives';
import {
  ScreenIntelligenceDegradedStateSchema,
  ScreenIntelligenceEvidenceKindSchema,
  ScreenIntelligenceNextStepSchema,
  ScreenIntelligencePolicyQuestionSchema,
  ScreenIntelligenceRouteDecisionIdSchema,
  ScreenIntelligenceRouteReasonSchema,
  ScreenIntelligenceRouteRequestIdSchema,
  ScreenIntelligenceRouteSelectionSchema,
  ScreenIntelligenceRouterSchemaVersion,
  ScreenIntelligenceSensitivityFlagSchema,
  ScreenIntelligenceSurfaceKindSchema,
} from './screen-intelligence-router-values';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const OptionalCaptureScopeSchema = Schema.Union(ScreenCaptureScopeSchema, Schema.Null);
const OptionalScreenCategorySchema = Schema.Union(ScreenVisibleCategorySchema, Schema.Null);

export const ScreenIntelligenceExistingEvidenceSchema = withParser(
  Schema.Struct({
    evidenceRef: ActivityEvidenceRefSchema,
    evidenceKind: ScreenIntelligenceEvidenceKindSchema,
    observedAt: ActivityTimestampSchema,
    category: OptionalScreenCategorySchema,
    confidence: ScreenEvidenceConfidenceSchema,
    canAnswerPolicyQuestion: Schema.Boolean,
    privacySafeForPolicy: Schema.Boolean,
    rawScreenshotEvidence: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) =>
        !value.canAnswerPolicyQuestion ||
        (value.privacySafeForPolicy && value.category !== null) ||
        'Expected policy-answering structured evidence to be privacy-safe and category-backed'
    )
  )
);

const ScreenIntelligenceRouteRequestBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenIntelligenceRouterSchemaVersion),
  routeRequestId: ScreenIntelligenceRouteRequestIdSchema,
  requestedAt: ActivityTimestampSchema,
  policyQuestion: ScreenIntelligencePolicyQuestionSchema,
  surfaceKind: ScreenIntelligenceSurfaceKindSchema,
  capabilityStatus: ScreenCapabilityStatusSchema,
  parentScreenAnalysisEnabled: Schema.Boolean,
  captureAllowedByParent: Schema.Boolean,
  allowedCaptureScope: ScreenCaptureScopeSchema,
  managedBrowserStructuredExtractionAvailable: Schema.Boolean,
  managedBrowserStructuredExtractionAttempted: Schema.Boolean,
  sensitivityFlags: Schema.Array(ScreenIntelligenceSensitivityFlagSchema),
  availableEvidence: Schema.Array(ScreenIntelligenceExistingEvidenceSchema),
  routeReason: ScreenEvidenceSummaryTextSchema,
});

export const ScreenIntelligenceRouteRequestSchema = withParser(
  ScreenIntelligenceRouteRequestBaseSchema.pipe(
    Schema.filter(
      (value) =>
        value.parentScreenAnalysisEnabled ||
        !value.captureAllowedByParent ||
        'Expected disabled screen analysis to keep capture disabled before routing'
    ),
    Schema.filter(
      (value) =>
        value.surfaceKind === 'managedBrowser' ||
        (!value.managedBrowserStructuredExtractionAvailable && !value.managedBrowserStructuredExtractionAttempted) ||
        'Expected managed-browser structured extraction flags only for managed browser surfaces'
    )
  )
);

const ScreenIntelligenceRouteDecisionBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
  decisionId: ScreenIntelligenceRouteDecisionIdSchema,
  requestId: ScreenIntelligenceRouteRequestIdSchema,
  decidedAt: ActivityTimestampSchema,
  selectedRoute: ScreenIntelligenceRouteSelectionSchema,
  nextStep: ScreenIntelligenceNextStepSchema,
  reason: ScreenIntelligenceRouteReasonSchema,
  policyQuestion: ScreenIntelligencePolicyQuestionSchema,
  surfaceKind: ScreenIntelligenceSurfaceKindSchema,
  existingEvidenceChecked: RequiredTrue,
  checkedEvidenceKinds: Schema.Array(ScreenIntelligenceEvidenceKindSchema),
  sourceEvidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
  structuredExtractionAttemptedBeforeScreenshot: Schema.Boolean,
  screenshotQueued: Schema.Boolean,
  captureScope: OptionalCaptureScopeSchema,
  rawScreenshotRetainedByDefault: RequiredFalse,
  remoteRawScreenshotUploadAllowed: RequiredFalse,
  parentVisibleSummary: ScreenEvidenceSummaryTextSchema,
  sensitivityFlags: Schema.Array(ScreenIntelligenceSensitivityFlagSchema),
  degradedStates: Schema.Array(ScreenIntelligenceDegradedStateSchema),
});

export const ScreenIntelligenceRouteDecisionSchema = withParser(
  ScreenIntelligenceRouteDecisionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        value.checkedEvidenceKinds.length > 0 ||
        'Expected screen intelligence routing to record which existing evidence families were checked'
    ),
    Schema.filter(
      (value) =>
        routeDecisionIsConsistent(value) ||
        'Expected screen intelligence route to check existing evidence before capture and select a safe next step'
    )
  )
);

export const ScreenIntelligenceRoutePlanRequestSchema = withParser(
  Schema.Struct({
    decisionId: ScreenIntelligenceRouteDecisionIdSchema,
    decidedAt: ActivityTimestampSchema,
    request: ScreenIntelligenceRouteRequestSchema,
  })
);

export const decodeScreenIntelligenceRouteDecision = Schema.decodeUnknownSync(ScreenIntelligenceRouteDecisionSchema);

function routeDecisionIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>): boolean {
  if (value.rawScreenshotRetainedByDefault || value.remoteRawScreenshotUploadAllowed) {
    return false;
  }
  if (value.selectedRoute === 'noScreenNeeded') {
    return noScreenNeededRouteIsConsistent(value);
  }
  if (value.selectedRoute === 'managedBrowserStructuredExtraction') {
    return managedBrowserStructuredRouteIsConsistent(value);
  }
  if (value.selectedRoute === 'managedBrowserScreenshot') {
    return managedBrowserScreenshotRouteIsConsistent(value);
  }
  if (value.selectedRoute === 'nativeActiveWindowCapture' || value.selectedRoute === 'selectedWindowCapture') {
    return nativeCaptureRouteIsConsistent(value);
  }

  return degradedRouteIsConsistent(value);
}

function noScreenNeededRouteIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>): boolean {
  return (
    value.sourceEvidenceRefs.length > 0 &&
    !value.screenshotQueued &&
    value.captureScope === null &&
    value.nextStep === 'deterministicSummary'
  );
}

function managedBrowserStructuredRouteIsConsistent(
  value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>
): boolean {
  return (
    value.structuredExtractionAttemptedBeforeScreenshot &&
    !value.screenshotQueued &&
    value.captureScope === null &&
    value.nextStep === 'structuredExtraction'
  );
}

function managedBrowserScreenshotRouteIsConsistent(
  value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>
): boolean {
  return (
    value.structuredExtractionAttemptedBeforeScreenshot &&
    value.screenshotQueued &&
    value.captureScope === 'managedBrowserWindow' &&
    value.nextStep === 'encryptedImageQueue'
  );
}

function nativeCaptureRouteIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>): boolean {
  return value.screenshotQueued && value.captureScope !== null && value.nextStep === 'encryptedImageQueue';
}

function degradedRouteIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>): boolean {
  return (
    !value.screenshotQueued &&
    value.captureScope === null &&
    value.degradedStates.length > 0 &&
    (value.nextStep === 'manualReviewRequired' || value.nextStep === 'unavailable')
  );
}

export type ScreenIntelligenceExistingEvidence = Infer<typeof ScreenIntelligenceExistingEvidenceSchema>;
export type ScreenIntelligenceRouteRequest = Infer<typeof ScreenIntelligenceRouteRequestSchema>;
export type ScreenIntelligenceRouteDecision = Infer<typeof ScreenIntelligenceRouteDecisionSchema>;
export type ScreenIntelligenceRoutePlanRequest = Infer<typeof ScreenIntelligenceRoutePlanRequestSchema>;
