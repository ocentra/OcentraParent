import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { BrowserAiModelRuntimeRefSchema, BrowserUrlAiAnalysisRequestIdSchema } from './browser-ai-analysis-schemas';
import { BrowserAiFamilyHubRouteSchema, type BrowserAiFamilyHubRoute } from './browser-ai-family-hub-routing-schemas';
import { BrowserAiProviderKindSchema, BrowserAiProviderRouteSchema } from './browser-ai-provider-routing-schemas';
import { BrowserAiRemoteRouteSchema, type BrowserAiRemoteRoute } from './browser-ai-remote-boundary-schemas';

const NonEmptyProviderFallbackText = Schema.String.pipe(Schema.minLength(1));
const OptionalProviderFallbackRuntimeRefSchema = Schema.Union(BrowserAiModelRuntimeRefSchema, Schema.Null);
const OptionalFamilyHubRouteSchema = Schema.Union(BrowserAiFamilyHubRouteSchema, Schema.Null);
const OptionalRemoteRouteSchema = Schema.Union(BrowserAiRemoteRouteSchema, Schema.Null);

export const BrowserAiProviderFallbackDecisionSchemaVersion = 1;

export const BrowserAiProviderFallbackDecisionIdSchema = withParser(
  NonEmptyProviderFallbackText.pipe(Schema.brand('BrowserAiProviderFallbackDecisionId'))
);

export const BrowserAiProviderFallbackReasonSchema = withParser(
  Schema.Literal(
    'local-selected',
    'family-hub-selected',
    'remote-selected',
    'metadata-only',
    'parent-review',
    'manual-required',
    'model-missing',
    'provider-unavailable',
    'resource-exhausted',
    'unsupported-task',
    'parent-approval-missing',
    'local-safety-fallback-missing',
    'timeout',
    'language-unsupported',
    'transcript-unavailable',
    'metadata-degraded'
  )
);

export const BrowserAiProviderFallbackActionSchema = withParser(
  Schema.Literal(
    'continue-selected-runtime',
    'metadata-only-review',
    'parent-review',
    'warn-child',
    'hold-temporary',
    'allow-with-background-review',
    'manual-review',
    'block-until-parent'
  )
);

const ProviderFallbackReasonsSchema = Schema.Array(BrowserAiProviderFallbackReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one provider fallback reason')
);
const ProviderFallbackAuditEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one provider fallback audit evidence id')
);

const BrowserAiProviderFallbackDecisionBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiProviderFallbackDecisionSchemaVersion),
  fallbackDecisionId: BrowserAiProviderFallbackDecisionIdSchema,
  requestId: BrowserUrlAiAnalysisRequestIdSchema,
  decidedAt: ActivityTimestampSchema,
  localProviderRoute: BrowserAiProviderRouteSchema,
  familyHubRoute: OptionalFamilyHubRouteSchema,
  remoteRoute: OptionalRemoteRouteSchema,
  selectedProviderKind: BrowserAiProviderKindSchema,
  selectedRuntimeRef: OptionalProviderFallbackRuntimeRefSchema,
  fallbackAction: BrowserAiProviderFallbackActionSchema,
  fallbackReasons: ProviderFallbackReasonsSchema,
  auditEvidenceIds: ProviderFallbackAuditEvidenceIdsSchema,
  parentFallbackVisible: Schema.Boolean,
  childFallbackVisible: Schema.Boolean,
  analysisResultClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  localSafetyPreserved: Schema.Boolean,
  remoteDefaultForBlocking: Schema.Boolean,
  remoteOutageDisablesLocalSafety: Schema.Boolean,
});

export const BrowserAiProviderFallbackDecisionSchema = withParser(
  BrowserAiProviderFallbackDecisionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiProviderFallbackDecisionIsConsistent(value) ||
        'Expected provider fallback decision to be visible, auditable, and unable to claim AI or policy authority'
    )
  )
);

export const decodeBrowserAiProviderFallbackDecision = Schema.decodeUnknownSync(
  BrowserAiProviderFallbackDecisionSchema
);

export type BrowserAiProviderFallbackAction = Infer<typeof BrowserAiProviderFallbackActionSchema>;
export type BrowserAiProviderFallbackDecision = Infer<typeof BrowserAiProviderFallbackDecisionSchema>;
export type BrowserAiProviderFallbackDecisionId = Infer<typeof BrowserAiProviderFallbackDecisionIdSchema>;
export type BrowserAiProviderFallbackReason = Infer<typeof BrowserAiProviderFallbackReasonSchema>;

function browserAiProviderFallbackDecisionIsConsistent(
  value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>
) {
  if (providerFallbackDecisionClaimsAuthority(value) || providerFallbackDecisionHidesFallback(value)) {
    return false;
  }
  if (value.selectedProviderKind === 'child-device-local-ai') {
    return localSelectedFallbackIsConsistent(value);
  }
  if (value.selectedProviderKind === 'family-ai-hub') {
    return familyHubFallbackIsConsistent(value);
  }
  if (value.selectedProviderKind === 'parent-approved-remote-ai') {
    return remoteFallbackIsConsistent(value);
  }
  if (value.selectedProviderKind === 'metadata-only') {
    return metadataOnlyFallbackIsConsistent(value);
  }
  return noAiFallbackIsConsistent(value);
}

function providerFallbackDecisionClaimsAuthority(value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>) {
  return (
    value.analysisResultClaimed ||
    value.policyDecisionClaimed ||
    !value.localSafetyPreserved ||
    value.remoteDefaultForBlocking ||
    value.remoteOutageDisablesLocalSafety
  );
}

function providerFallbackDecisionHidesFallback(value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>) {
  return !value.parentFallbackVisible || !value.childFallbackVisible;
}

function localSelectedFallbackIsConsistent(value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>) {
  return (
    value.localProviderRoute.executionState === 'selected' &&
    value.selectedRuntimeRef === value.localProviderRoute.selectedRuntimeRef &&
    value.fallbackAction === 'continue-selected-runtime' &&
    value.fallbackReasons.includes('local-selected') &&
    !familyHubRouteIsSelected(value.familyHubRoute) &&
    !remoteRouteIsSelected(value.remoteRoute)
  );
}

function familyHubFallbackIsConsistent(value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>) {
  return (
    value.familyHubRoute !== null &&
    value.localProviderRoute.executionState !== 'selected' &&
    value.familyHubRoute.executionState === 'selected' &&
    value.selectedRuntimeRef === value.familyHubRoute.selectedRuntimeRef &&
    value.fallbackAction === 'continue-selected-runtime' &&
    value.fallbackReasons.includes('family-hub-selected') &&
    !remoteRouteIsSelected(value.remoteRoute)
  );
}

function remoteFallbackIsConsistent(value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>) {
  return (
    value.remoteRoute !== null &&
    value.localProviderRoute.executionState !== 'selected' &&
    !familyHubRouteIsSelected(value.familyHubRoute) &&
    value.remoteRoute.executionState === 'selected' &&
    value.remoteRoute.parentExplicitRemoteApproval &&
    value.remoteRoute.localSafetyFallbackAvailable &&
    value.selectedRuntimeRef === value.remoteRoute.selectedRuntimeRef &&
    value.fallbackAction === 'continue-selected-runtime' &&
    value.fallbackReasons.includes('remote-selected')
  );
}

function metadataOnlyFallbackIsConsistent(value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>) {
  return (
    noProviderRouteSelected(value) &&
    value.selectedRuntimeRef === null &&
    value.fallbackAction === 'metadata-only-review' &&
    value.fallbackReasons.includes('metadata-only')
  );
}

function noAiFallbackIsConsistent(value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>) {
  return (
    noProviderRouteSelected(value) &&
    value.selectedRuntimeRef === null &&
    value.fallbackAction !== 'continue-selected-runtime' &&
    !value.fallbackReasons.includes('local-selected') &&
    !value.fallbackReasons.includes('family-hub-selected') &&
    !value.fallbackReasons.includes('remote-selected')
  );
}

function noProviderRouteSelected(value: Infer<typeof BrowserAiProviderFallbackDecisionBaseSchema>) {
  return (
    value.localProviderRoute.executionState !== 'selected' &&
    !familyHubRouteIsSelected(value.familyHubRoute) &&
    !remoteRouteIsSelected(value.remoteRoute)
  );
}

function familyHubRouteIsSelected(value: BrowserAiFamilyHubRoute | null) {
  return value !== null && value.executionState === 'selected';
}

function remoteRouteIsSelected(value: BrowserAiRemoteRoute | null) {
  return value !== null && value.executionState === 'selected';
}
