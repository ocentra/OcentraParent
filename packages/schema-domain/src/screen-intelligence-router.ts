import { type Infer, Schema, withParser } from './effect';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityDeviceIdSchema, ActivityTimestampSchema } from './evidence-primitives';
import {
  ScreenCaptureReasonSchema,
  ScreenCaptureScopeSchema,
  ScreenEvidenceCustodyStateSchema,
  ScreenRiskSignalSchema,
  ScreenVisibleCategorySchema,
} from './screen-evidence-states';
import {
  ScreenEvidenceConfidenceSchema,
  ScreenEvidenceReasonSchema,
} from './screen-evidence-primitives';
import {
  ScreenIntelligencePolicySensitivitySchema,
  ScreenIntelligenceRouteIdSchema,
  ScreenIntelligenceRouteKindSchema,
  ScreenIntelligenceRouteRequestIdSchema,
  ScreenIntelligenceRouterSchemaVersion,
  ScreenIntelligenceSourceKindSchema,
  ScreenManagedBrowserStructuredTextLimit,
  ScreenPolicyQuestionTextSchema,
  ScreenStructuredEvidenceSummarySchema,
  ScreenStructuredExtractionIdSchema,
  ScreenStructuredExtractionRedactionStateSchema,
  ScreenStructuredExtractionStateSchema,
} from './screen-intelligence-router-values';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const NonNegativeInteger = Schema.Number.pipe(Schema.int(), Schema.nonNegative());
const EvidenceRefsSchema = Schema.Array(ActivityEvidenceRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one screen router evidence reference')
);
const CaptureScopesSchema = Schema.Array(ScreenCaptureScopeSchema);

const ScreenStructuredExtractionBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenIntelligenceRouterSchemaVersion),
  extractionId: ScreenStructuredExtractionIdSchema,
  capturedAt: ActivityTimestampSchema,
  evidenceRefs: EvidenceRefsSchema,
  extractionState: ScreenStructuredExtractionStateSchema,
  urlTitleMetadataCaptured: RequiredTrue,
  visibleTextSummary: Schema.Union(ScreenStructuredEvidenceSummarySchema, Schema.Null),
  visibleTextCharacterCount: NonNegativeInteger,
  domOverflowRedacted: Schema.Boolean,
  privateContentRedacted: Schema.Boolean,
  rawDomIncluded: RequiredFalse,
  redactionState: ScreenStructuredExtractionRedactionStateSchema,
  enoughForPolicy: Schema.Boolean,
  policyQuestionAnswered: Schema.Boolean,
  noScreenNeeded: Schema.Boolean,
  screenshotRequired: Schema.Boolean,
  categoryCandidate: Schema.Union(ScreenVisibleCategorySchema, Schema.Null),
  riskSignals: Schema.Array(ScreenRiskSignalSchema),
  confidence: ScreenEvidenceConfidenceSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  reason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
});

export const ScreenManagedBrowserStructuredExtractionSchema = withParser(
  ScreenStructuredExtractionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        structuredExtractionIsConsistent(value) ||
        'Expected managed browser structured extraction to stay bounded, redacted, and enough before no-screen-needed'
    )
  )
);

const ScreenIntelligenceRouteRequestBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenIntelligenceRouterSchemaVersion),
  requestId: ScreenIntelligenceRouteRequestIdSchema,
  requestedAt: ActivityTimestampSchema,
  deviceRef: ActivityDeviceIdSchema,
  sourceKind: ScreenIntelligenceSourceKindSchema,
  captureReason: ScreenCaptureReasonSchema,
  policyQuestion: ScreenPolicyQuestionTextSchema,
  policySensitivity: ScreenIntelligencePolicySensitivitySchema,
  existingEvidenceRefs: EvidenceRefsSchema,
  structuredExtraction: Schema.Union(ScreenManagedBrowserStructuredExtractionSchema, Schema.Null),
  parentAllowsManagedBrowserStructuredExtraction: Schema.Boolean,
  parentAllowsScreenCapture: Schema.Boolean,
  allowedCaptureScopes: CaptureScopesSchema,
  protectedSurfaceSuspected: Schema.Boolean,
  credentialPromptSuspected: Schema.Boolean,
});

export const ScreenIntelligenceRouteRequestSchema = withParser(
  ScreenIntelligenceRouteRequestBaseSchema.pipe(
    Schema.filter(
      (value) =>
        routeRequestIsConsistent(value) ||
        'Expected screen route request to expose existing evidence and parent capture permissions'
    )
  )
);

const ScreenIntelligenceRouteDecisionBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenIntelligenceRouterSchemaVersion),
  routeId: ScreenIntelligenceRouteIdSchema,
  requestId: ScreenIntelligenceRouteRequestIdSchema,
  decidedAt: ActivityTimestampSchema,
  sourceKind: ScreenIntelligenceSourceKindSchema,
  routeKind: ScreenIntelligenceRouteKindSchema,
  captureScope: Schema.Union(ScreenCaptureScopeSchema, Schema.Null),
  structuredExtractionId: Schema.Union(ScreenStructuredExtractionIdSchema, Schema.Null),
  screenshotSkipped: Schema.Boolean,
  checkedExistingEvidenceFirst: RequiredTrue,
  managedBrowserStructuredExtractionFirst: Schema.Boolean,
  policyQuestion: ScreenPolicyQuestionTextSchema,
  policySensitivity: ScreenIntelligencePolicySensitivitySchema,
  evidenceRefs: EvidenceRefsSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  manualRequiredReason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
  unavailableReason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
  remoteAiAllowed: RequiredFalse,
  rawScreenshotRetained: RequiredFalse,
});

export const ScreenIntelligenceRouteDecisionSchema = withParser(
  ScreenIntelligenceRouteDecisionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        routeDecisionIsConsistent(value) ||
        'Expected screen intelligence route to prefer safe evidence before screenshots and reject remote/raw defaults'
    )
  )
);

export type ScreenManagedBrowserStructuredExtraction = Infer<typeof ScreenManagedBrowserStructuredExtractionSchema>;
export type ScreenIntelligenceRouteRequest = Infer<typeof ScreenIntelligenceRouteRequestSchema>;
export type ScreenIntelligenceRouteDecision = Infer<typeof ScreenIntelligenceRouteDecisionSchema>;

export const decodeScreenManagedBrowserStructuredExtraction = Schema.decodeUnknownSync(
  ScreenManagedBrowserStructuredExtractionSchema
);
export const decodeScreenIntelligenceRouteDecision = Schema.decodeUnknownSync(ScreenIntelligenceRouteDecisionSchema);

function structuredExtractionIsConsistent(value: Infer<typeof ScreenStructuredExtractionBaseSchema>): boolean {
  if (
    value.rawDomIncluded ||
    value.visibleTextCharacterCount > ScreenManagedBrowserStructuredTextLimit ||
    value.custodyState === 'ocentra-hosted-non-activity'
  ) {
    return false;
  }
  if (value.extractionState === 'enoughForPolicy') {
    return (
      value.enoughForPolicy &&
      value.policyQuestionAnswered &&
      value.noScreenNeeded &&
      !value.screenshotRequired &&
      value.categoryCandidate !== null
    );
  }
  return !value.noScreenNeeded && value.screenshotRequired === (value.extractionState === 'needsScreenshot');
}

function routeRequestIsConsistent(value: Infer<typeof ScreenIntelligenceRouteRequestBaseSchema>): boolean {
  if (value.allowedCaptureScopes.includes('fullScreen')) {
    return false;
  }
  return value.structuredExtraction === null || value.sourceKind === 'managedBrowser';
}

function routeDecisionIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>): boolean {
  if (!value.checkedExistingEvidenceFirst || value.remoteAiAllowed || value.rawScreenshotRetained) {
    return false;
  }
  switch (value.routeKind) {
    case 'noScreenNeeded':
      return noScreenNeededDecisionIsConsistent(value);
    case 'managedBrowserStructuredExtraction':
      return structuredFirstDecisionIsConsistent(value);
    case 'screenCaptureActiveWindow':
      return captureDecisionIsConsistent(value, 'activeWindow');
    case 'screenCaptureSelectedWindow':
      return captureDecisionIsConsistent(value, 'selectedWindow');
    case 'manualRequired':
      return manualDecisionIsConsistent(value);
    case 'unavailable':
      return unavailableDecisionIsConsistent(value);
  }
}

function noScreenNeededDecisionIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>) {
  return value.screenshotSkipped && value.captureScope === null && value.structuredExtractionId !== null;
}

function structuredFirstDecisionIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>) {
  return value.screenshotSkipped && value.managedBrowserStructuredExtractionFirst && value.captureScope === null;
}

function captureDecisionIsConsistent(
  value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>,
  captureScope: Infer<typeof ScreenCaptureScopeSchema>
) {
  return !value.screenshotSkipped && value.captureScope === captureScope;
}

function manualDecisionIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>) {
  return value.screenshotSkipped && value.captureScope === null && value.manualRequiredReason !== null;
}

function unavailableDecisionIsConsistent(value: Infer<typeof ScreenIntelligenceRouteDecisionBaseSchema>) {
  return value.screenshotSkipped && value.captureScope === null && value.unavailableReason !== null;
}
