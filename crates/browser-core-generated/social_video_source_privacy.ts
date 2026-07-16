/* generated from crates/browser-core/src/social_video_source_privacy.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  BrowserAiConfidenceSchema,
  BrowserAiDegradedStateSchema,
} from '@ocentra-parent/schema-domain/browser-ai-analysis-values';
import { BrowserCustodyLabelSchema } from '@ocentra-parent/schema-domain/browser-schemas';
import {
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialPlatformSchema,
  BrowserSocialVideoMetadataEvidenceIdSchema,
} from './social_video_ai_signal_aggregate_support';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivitySubjectIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/schema-domain/evidence-primitives';
const SourcePrivacySourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video source privacy source evidence ids')
);

export const SocialVideoSourcePrivacySchemaVersion = 1;
export const SocialVideoSourcePrivacyEvidenceIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialVideoSourcePrivacyEvidenceId')
);
export const SocialVideoParentProvidedTargetRefSchema = withParser(
  brandedNonEmptyStringSchema('SocialVideoParentProvidedTargetRef')
);
export const SocialVideoConnectorAuthorizationRefSchema = withParser(
  brandedNonEmptyStringSchema('SocialVideoConnectorAuthorizationRef')
);

export const SocialVideoSourcePrivacyTargetKindSchema = withParser(
  Schema.Literal(
    'social-platform',
    'video-url',
    'video-channel',
    'short-video',
    'social-feed',
    'messaging-route',
    'native-social-app',
    'unknown'
  )
);
export const SocialVideoSourceTypeSchema = withParser(
  Schema.Literal(
    'managed-browser-social-route-ref',
    'managed-browser-video-metadata-ref',
    'parent-provided-url-ref',
    'parent-provided-channel-ref',
    'screen-summary-ref',
    'connector-authorization-ref',
    'android-native-manual-required',
    'ios-native-manual-required',
    'platform-manual-required'
  )
);
export const SocialVideoPermittedDownstreamUseSchema = withParser(
  Schema.Literal('ai-analysis-input', 'policy-candidate-input', 'parent-explanation', 'manual-review', 'audit-summary')
);
export const SocialVideoManualRequiredReasonSchema = withParser(
  Schema.Literal(
    'native-app-source-unavailable',
    'connector-not-authorized',
    'screen-summary-unavailable',
    'platform-proof-missing',
    'parent-review-required'
  )
);
const OptionalSocialVideoManualRequiredReasonSchema = Schema.Union(SocialVideoManualRequiredReasonSchema, Schema.Null);

const SocialVideoSourceTypesSchema = Schema.Array(SocialVideoSourceTypeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one social/video source type')
);
const SocialVideoPermittedDownstreamUsesSchema = Schema.Array(SocialVideoPermittedDownstreamUseSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one permitted downstream use')
);

const SocialVideoSourcePrivacySummaryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(SocialVideoSourcePrivacySchemaVersion),
  sourcePrivacyEvidenceId: SocialVideoSourcePrivacyEvidenceIdSchema,
  summarizedAt: ActivityTimestampSchema,
  childProfileRef: ActivitySubjectIdSchema,
  deviceId: ActivityDeviceIdSchema,
  sourceEvidenceIds: SourcePrivacySourceEvidenceIdsSchema,
  platform: BrowserSocialPlatformSchema,
  targetKind: SocialVideoSourcePrivacyTargetKindSchema,
  sourceTypes: SocialVideoSourceTypesSchema,
  socialRouteEvidenceIds: Schema.Array(BrowserSocialRouteEvidenceIdSchema),
  socialVideoMetadataEvidenceIds: Schema.Array(BrowserSocialVideoMetadataEvidenceIdSchema),
  parentProvidedUrlRefs: Schema.Array(SocialVideoParentProvidedTargetRefSchema),
  parentProvidedChannelRefs: Schema.Array(SocialVideoParentProvidedTargetRefSchema),
  screenSummaryEvidenceRefs: Schema.Array(ActivityEvidenceIdSchema),
  connectorAuthorizationRefs: Schema.Array(SocialVideoConnectorAuthorizationRefSchema),
  manualRequiredReason: OptionalSocialVideoManualRequiredReasonSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  confidence: BrowserAiConfidenceSchema,
  degradedState: BrowserAiDegradedStateSchema,
  permittedDownstreamUses: SocialVideoPermittedDownstreamUsesSchema,
  rawContentCaptured: Schema.Boolean,
  rawMessageContentCaptured: Schema.Boolean,
  rawVideoCaptured: Schema.Boolean,
  screenshotCaptured: Schema.Boolean,
  connectorTokenStored: Schema.Boolean,
  connectorApiCalled: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

export const SocialVideoSourcePrivacySummarySchema = withParser(
  SocialVideoSourcePrivacySummaryBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialVideoSourcePrivacySummaryIsConsistent(value) ||
        'Expected social/video source privacy summary to cite typed evidence refs without raw content or authority claims'
    )
  )
);

const SocialVideoSourcePrivacyInputSchema = withParser(
  SocialVideoSourcePrivacySummaryBaseSchema.omit(
    'schemaVersion',
    'rawContentCaptured',
    'rawMessageContentCaptured',
    'rawVideoCaptured',
    'screenshotCaptured',
    'connectorTokenStored',
    'connectorApiCalled',
    'nativeAppControlClaimed',
    'finalPolicyDecisionClaimed',
    'enforcementClaimed'
  )
);

export const decodeSocialVideoSourcePrivacySummary = Schema.decodeUnknownSync(SocialVideoSourcePrivacySummarySchema);

export type SocialVideoConnectorAuthorizationRef = Infer<typeof SocialVideoConnectorAuthorizationRefSchema>;
export type SocialVideoParentProvidedTargetRef = Infer<typeof SocialVideoParentProvidedTargetRefSchema>;
export type SocialVideoPermittedDownstreamUse = Infer<typeof SocialVideoPermittedDownstreamUseSchema>;
export type SocialVideoSourcePrivacySummary = Infer<typeof SocialVideoSourcePrivacySummarySchema>;
export type SocialVideoSourceType = Infer<typeof SocialVideoSourceTypeSchema>;

type SocialVideoSourcePrivacySummaryCandidate = Infer<typeof SocialVideoSourcePrivacySummaryBaseSchema>;

export function buildSocialVideoSourcePrivacySummary(
  input: Infer<typeof SocialVideoSourcePrivacyInputSchema>
): SocialVideoSourcePrivacySummary {
  const parsed = SocialVideoSourcePrivacyInputSchema.parse(input);

  return SocialVideoSourcePrivacySummarySchema.parse({
    ...parsed,
    schemaVersion: SocialVideoSourcePrivacySchemaVersion,
    rawContentCaptured: false,
    rawMessageContentCaptured: false,
    rawVideoCaptured: false,
    screenshotCaptured: false,
    connectorTokenStored: false,
    connectorApiCalled: false,
    nativeAppControlClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
  });
}

function socialVideoSourcePrivacySummaryIsConsistent(value: SocialVideoSourcePrivacySummaryCandidate) {
  return (
    !sourcePrivacyClaimsForbiddenState(value) &&
    sourceTypesMatchRefs(value) &&
    sourcePrivacyManualStateIsConsistent(value) &&
    sourcePrivacyDownstreamUseIsConsistent(value)
  );
}

function sourcePrivacyClaimsForbiddenState(value: SocialVideoSourcePrivacySummaryCandidate) {
  return (
    value.rawContentCaptured ||
    value.rawMessageContentCaptured ||
    value.rawVideoCaptured ||
    value.screenshotCaptured ||
    value.connectorTokenStored ||
    value.connectorApiCalled ||
    value.nativeAppControlClaimed ||
    value.finalPolicyDecisionClaimed ||
    value.enforcementClaimed
  );
}

function sourceTypesMatchRefs(value: SocialVideoSourcePrivacySummaryCandidate) {
  return (
    sourceTypeMatchesRef(value, 'managed-browser-social-route-ref', value.socialRouteEvidenceIds.length) &&
    sourceTypeMatchesRef(value, 'managed-browser-video-metadata-ref', value.socialVideoMetadataEvidenceIds.length) &&
    sourceTypeMatchesRef(value, 'parent-provided-url-ref', value.parentProvidedUrlRefs.length) &&
    sourceTypeMatchesRef(value, 'parent-provided-channel-ref', value.parentProvidedChannelRefs.length) &&
    sourceTypeMatchesRef(value, 'screen-summary-ref', value.screenSummaryEvidenceRefs.length) &&
    sourceTypeMatchesRef(value, 'connector-authorization-ref', value.connectorAuthorizationRefs.length) &&
    hasAtLeastOneSourceRef(value)
  );
}

function sourceTypeMatchesRef(
  value: SocialVideoSourcePrivacySummaryCandidate,
  sourceType: SocialVideoSourceType,
  refCount: number
) {
  return value.sourceTypes.includes(sourceType) === refCount > 0;
}

function hasAtLeastOneSourceRef(value: SocialVideoSourcePrivacySummaryCandidate) {
  return (
    value.socialRouteEvidenceIds.length > 0 ||
    value.socialVideoMetadataEvidenceIds.length > 0 ||
    value.parentProvidedUrlRefs.length > 0 ||
    value.parentProvidedChannelRefs.length > 0 ||
    value.screenSummaryEvidenceRefs.length > 0 ||
    value.connectorAuthorizationRefs.length > 0 ||
    sourcePrivacyHasManualRequiredSource(value)
  );
}

function sourcePrivacyManualStateIsConsistent(value: SocialVideoSourcePrivacySummaryCandidate) {
  if (sourcePrivacyHasManualRequiredSource(value)) {
    return (
      value.degradedState === 'manual-required' && value.confidence === 'unknown' && value.manualRequiredReason !== null
    );
  }
  if (value.degradedState === 'none') {
    return value.confidence !== 'unknown' && value.manualRequiredReason === null;
  }
  return value.degradedState !== 'manual-required' && value.manualRequiredReason !== null;
}

function sourcePrivacyHasManualRequiredSource(value: SocialVideoSourcePrivacySummaryCandidate) {
  return (
    value.sourceTypes.includes('android-native-manual-required') ||
    value.sourceTypes.includes('ios-native-manual-required') ||
    value.sourceTypes.includes('platform-manual-required')
  );
}

function sourcePrivacyDownstreamUseIsConsistent(value: SocialVideoSourcePrivacySummaryCandidate) {
  if (value.degradedState === 'unavailable' || value.degradedState === 'manual-required') {
    return !value.permittedDownstreamUses.includes('policy-candidate-input');
  }
  return true;
}
