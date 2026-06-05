import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityDeviceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  ScreenCaptureReasonSchema,
  ScreenCaptureScopeSchema,
  ScreenEvidenceCustodyStateSchema,
  ScreenRiskSignalSchema,
  ScreenVisibleCategorySchema,
} from './screen-evidence-states';
import { ScreenEvidenceConfidenceSchema, ScreenEvidenceReasonSchema } from './screen-evidence-primitives';
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

export * from './screen-intelligence-router-values';

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

export function planScreenIntelligenceRoute(
  request: ScreenIntelligenceRouteRequest,
  routeId: Infer<typeof ScreenIntelligenceRouteIdSchema>
): ScreenIntelligenceRouteDecision {
  const parsed = ScreenIntelligenceRouteRequestSchema.parse(request);
  const routeKind = routeKindFor(parsed);
  return buildDecision(
    parsed,
    routeId,
    routeKind,
    captureScopeForRoute(parsed, routeKind),
    structuredExtractionForRoute(parsed)
  );
}

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

function screenCaptureIsUnsafe(request: ScreenIntelligenceRouteRequest): boolean {
  return (
    request.protectedSurfaceSuspected ||
    request.credentialPromptSuspected ||
    request.policySensitivity === 'protectedSurface' ||
    request.policySensitivity === 'credentialRisk'
  );
}

function routeKindFor(request: ScreenIntelligenceRouteRequest): Infer<typeof ScreenIntelligenceRouteKindSchema> {
  if (screenCaptureIsUnsafe(request)) {
    return 'unavailable';
  }
  if (request.structuredExtraction?.noScreenNeeded) {
    return 'noScreenNeeded';
  }
  if (request.sourceKind === 'managedBrowser' && request.parentAllowsManagedBrowserStructuredExtraction) {
    return 'managedBrowserStructuredExtraction';
  }
  if (!request.parentAllowsScreenCapture) {
    return 'manualRequired';
  }
  return captureRouteKindFor(preferredCaptureScope(request.allowedCaptureScopes));
}

function captureRouteKindFor(
  captureScope: Infer<typeof ScreenCaptureScopeSchema> | null
): Infer<typeof ScreenIntelligenceRouteKindSchema> {
  if (captureScope === 'activeWindow') {
    return 'screenCaptureActiveWindow';
  }
  if (captureScope === 'selectedWindow') {
    return 'screenCaptureSelectedWindow';
  }
  return 'manualRequired';
}

function preferredCaptureScope(scopes: readonly Infer<typeof ScreenCaptureScopeSchema>[]) {
  if (scopes.includes('activeWindow')) {
    return 'activeWindow' as const;
  }
  if (scopes.includes('selectedWindow')) {
    return 'selectedWindow' as const;
  }
  return null;
}

function captureScopeForRoute(
  request: ScreenIntelligenceRouteRequest,
  routeKind: Infer<typeof ScreenIntelligenceRouteKindSchema>
) {
  if (routeKind === 'screenCaptureActiveWindow' || routeKind === 'screenCaptureSelectedWindow') {
    return preferredCaptureScope(request.allowedCaptureScopes);
  }
  return null;
}

function structuredExtractionForRoute(request: ScreenIntelligenceRouteRequest) {
  return request.structuredExtraction?.extractionId ?? null;
}

function buildDecision(
  request: ScreenIntelligenceRouteRequest,
  routeId: Infer<typeof ScreenIntelligenceRouteIdSchema>,
  routeKind: Infer<typeof ScreenIntelligenceRouteKindSchema>,
  captureScope: Infer<typeof ScreenCaptureScopeSchema> | null,
  structuredExtractionId: Infer<typeof ScreenStructuredExtractionIdSchema> | null
) {
  return ScreenIntelligenceRouteDecisionSchema.parse({
    schemaVersion: ScreenIntelligenceRouterSchemaVersion,
    routeId,
    requestId: request.requestId,
    decidedAt: request.requestedAt,
    sourceKind: request.sourceKind,
    routeKind,
    captureScope,
    structuredExtractionId,
    screenshotSkipped: captureScope === null,
    checkedExistingEvidenceFirst: true,
    managedBrowserStructuredExtractionFirst:
      request.sourceKind === 'managedBrowser' &&
      (routeKind === 'managedBrowserStructuredExtraction' || routeKind === 'noScreenNeeded'),
    policyQuestion: request.policyQuestion,
    policySensitivity: request.policySensitivity,
    evidenceRefs: request.structuredExtraction?.evidenceRefs ?? request.existingEvidenceRefs,
    custodyState: request.structuredExtraction?.custodyState ?? 'child-device-query-store',
    manualRequiredReason: routeKind === 'manualRequired' ? manualReasonFor(request) : null,
    unavailableReason: routeKind === 'unavailable' ? unavailableReasonFor(request) : null,
    remoteAiAllowed: false,
    rawScreenshotRetained: false,
  });
}

function manualReasonFor(request: ScreenIntelligenceRouteRequest) {
  if (!request.parentAllowsScreenCapture) {
    return 'parent setting requires manual review before screen capture';
  }
  return 'no allowed active-window or selected-window capture scope is available';
}

function unavailableReasonFor(request: ScreenIntelligenceRouteRequest) {
  if (request.protectedSurfaceSuspected || request.policySensitivity === 'protectedSurface') {
    return 'protected surface is not eligible for screen capture or model analysis';
  }
  return 'credential prompt risk is not eligible for screen capture or model analysis';
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
