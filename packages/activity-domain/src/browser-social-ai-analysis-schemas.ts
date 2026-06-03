import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivitySubjectIdSchema,
  ActivityTimestampSchema,
} from './primitives';
import { BrowserCustodyLabelSchema } from './browser-schemas';
import {
  BrowserAiConfidenceSchema,
  BrowserAiDegradedStateSchema,
  BrowserAiModelRuntimePreferenceSchema,
  BrowserAiRecommendedPolicyInputSchema,
  BrowserAiUncertaintyReasonSchema,
  BrowserParentRuleRefSchema,
} from './browser-ai-analysis-schemas';
import {
  BrowserUrlIntelligenceMemoryHitIdSchema,
  BrowserUrlShapeClassificationIdSchema,
} from './browser-url-intelligence-schemas';
import { BrowserSocialAccountFlowEvidenceIdSchema } from './browser-social-account-flow-schemas';
import { BrowserSocialAccountIdentityRefSchema } from './browser-social-account-identity-registry';
import { BrowserSocialFeedRouteClassificationIdSchema } from './browser-social-feed-route-classification';
import {
  BrowserSocialPlatformSchema,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRouteKind,
  BrowserSocialRouteKindSchema,
} from './browser-social-platform-route-schemas';
import { BrowserSocialVideoMetadataEvidenceIdSchema } from './browser-social-video-metadata';
import {
  BrowserSocialAiAnalysisIdSchema,
  BrowserSocialAiAnalysisRequestIdSchema,
  BrowserSocialAiAnalysisSchemaVersion,
  BrowserSocialAiAnalysisTaskSchema,
  BrowserSocialAiPromptTemplateSchema,
  NonEmptySocialAiTextSchema,
  OptionalSocialAiRuntimeRefSchema,
  OptionalSocialAiTextSchema,
  SocialAiClassificationsSchema,
  SocialAiSourceEvidenceIdsSchema,
  SocialAiTextRefsSchema,
} from './browser-social-ai-analysis-values';

const BrowserSocialAiAnalysisInputBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialAiAnalysisSchemaVersion),
  requestId: BrowserSocialAiAnalysisRequestIdSchema,
  requestedAt: ActivityTimestampSchema,
  childProfileRef: ActivitySubjectIdSchema,
  deviceId: ActivityDeviceIdSchema,
  sourceEvidenceIds: SocialAiSourceEvidenceIdsSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  urlShapeClassificationId: BrowserUrlShapeClassificationIdSchema,
  platform: BrowserSocialPlatformSchema,
  routeKind: BrowserSocialRouteKindSchema,
  feedRouteClassificationIds: Schema.Array(BrowserSocialFeedRouteClassificationIdSchema),
  metadataEvidenceIds: Schema.Array(BrowserSocialVideoMetadataEvidenceIdSchema),
  accountFlowEvidenceIds: Schema.Array(BrowserSocialAccountFlowEvidenceIdSchema),
  accountIdentityRefs: Schema.Array(BrowserSocialAccountIdentityRefSchema),
  screenSummaryEvidenceRefs: Schema.Array(ActivityEvidenceIdSchema),
  parentRuleRefs: Schema.Array(BrowserParentRuleRefSchema),
  memoryHitIds: Schema.Array(BrowserUrlIntelligenceMemoryHitIdSchema),
  requestedTask: BrowserSocialAiAnalysisTaskSchema,
  modelRuntimePreference: BrowserAiModelRuntimePreferenceSchema,
  promptTemplate: BrowserSocialAiPromptTemplateSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  rawBrowserStateIncluded: Schema.Boolean,
  rawPageBodyIncluded: Schema.Boolean,
  rawMessageContentIncluded: Schema.Boolean,
  rawFeedContentIncluded: Schema.Boolean,
  transcriptTextIncluded: Schema.Boolean,
  screenshotIncluded: Schema.Boolean,
  nativeAppStateIncluded: Schema.Boolean,
  platformConnectorIncluded: Schema.Boolean,
});
export const BrowserSocialAiAnalysisInputSchema = withParser(
  BrowserSocialAiAnalysisInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialAiAnalysisInputIsConsistent(value) ||
        'Expected social AI input to use typed social evidence refs without raw content or connector state'
    )
  )
);

const BrowserSocialAiAnalysisResultBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialAiAnalysisSchemaVersion),
  analysisId: BrowserSocialAiAnalysisIdSchema,
  requestId: BrowserSocialAiAnalysisRequestIdSchema,
  analyzedAt: ActivityTimestampSchema,
  expiresAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialAiSourceEvidenceIdsSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  platform: BrowserSocialPlatformSchema,
  routeKind: BrowserSocialRouteKindSchema,
  requestedTask: BrowserSocialAiAnalysisTaskSchema,
  classifications: SocialAiClassificationsSchema,
  riskSignalRefs: SocialAiTextRefsSchema,
  benefitSignalRefs: SocialAiTextRefsSchema,
  recommendedPolicyInput: BrowserAiRecommendedPolicyInputSchema,
  confidence: BrowserAiConfidenceSchema,
  uncertaintyReasons: Schema.Array(BrowserAiUncertaintyReasonSchema),
  parentSummaryRef: NonEmptySocialAiTextSchema,
  childSafeSummaryRef: OptionalSocialAiTextSchema,
  modelRuntimeRef: OptionalSocialAiRuntimeRefSchema,
  promptTemplate: BrowserSocialAiPromptTemplateSchema,
  degradedState: BrowserAiDegradedStateSchema,
  finalPolicyActionClaimed: Schema.Boolean,
  enforcementActionClaimed: Schema.Boolean,
  rawModelTextStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  transcriptTextStored: Schema.Boolean,
  rawMessageContentStored: Schema.Boolean,
  rawFeedContentStored: Schema.Boolean,
  screenshotStored: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
});
export const BrowserSocialAiAnalysisResultSchema = withParser(
  BrowserSocialAiAnalysisResultBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialAiAnalysisResultIsConsistent(value) ||
        'Expected social AI result to remain candidate-only, evidence-backed, and degradation-aware'
    )
  )
);

export const decodeBrowserSocialAiAnalysisInput = Schema.decodeUnknownSync(BrowserSocialAiAnalysisInputSchema);
export const decodeBrowserSocialAiAnalysisResult = Schema.decodeUnknownSync(BrowserSocialAiAnalysisResultSchema);

export type BrowserSocialAiAnalysisInput = Infer<typeof BrowserSocialAiAnalysisInputSchema>;
export type BrowserSocialAiAnalysisResult = Infer<typeof BrowserSocialAiAnalysisResultSchema>;
export type { BrowserSocialAiAnalysisTask, BrowserSocialAiPromptTemplate } from './browser-social-ai-analysis-values';

function browserSocialAiAnalysisInputIsConsistent(value: Infer<typeof BrowserSocialAiAnalysisInputBaseSchema>) {
  if (socialAiInputHasForbiddenRawState(value) || value.promptTemplate.requestedTask !== value.requestedTask) {
    return false;
  }
  return socialAiTaskHasEvidence(value);
}

function browserSocialAiAnalysisResultIsConsistent(value: Infer<typeof BrowserSocialAiAnalysisResultBaseSchema>) {
  if (socialAiResultClaimsAuthority(value) || value.promptTemplate.requestedTask !== value.requestedTask) {
    return false;
  }
  if (value.degradedState === 'none') {
    return (
      value.modelRuntimeRef !== null &&
      value.uncertaintyReasons.length === 0 &&
      value.confidence !== 'unknown' &&
      !value.classifications.includes('unknown')
    );
  }
  return value.uncertaintyReasons.length > 0 && value.confidence !== 'high';
}

function socialAiInputHasForbiddenRawState(value: Infer<typeof BrowserSocialAiAnalysisInputBaseSchema>) {
  return (
    value.rawBrowserStateIncluded ||
    value.rawPageBodyIncluded ||
    value.rawMessageContentIncluded ||
    value.rawFeedContentIncluded ||
    value.transcriptTextIncluded ||
    value.screenshotIncluded ||
    value.nativeAppStateIncluded ||
    value.platformConnectorIncluded
  );
}

function socialAiTaskHasEvidence(value: Infer<typeof BrowserSocialAiAnalysisInputBaseSchema>) {
  if (value.requestedTask === 'video-safety') {
    return routeCanSupportVideoSafety(value.routeKind) && value.metadataEvidenceIds.length > 0;
  }
  if (value.requestedTask === 'feed-risk-classification') {
    return value.routeKind === 'feed' && value.feedRouteClassificationIds.length > 0;
  }
  if (value.requestedTask === 'messaging-risk-summary') {
    return value.routeKind === 'messaging-route';
  }
  if (value.requestedTask === 'signup-attempt-classification' || value.requestedTask === 'secondary-account-risk') {
    return accountRouteCanSupportAnalysis(value.routeKind) && value.accountFlowEvidenceIds.length > 0;
  }
  return true;
}

function routeCanSupportVideoSafety(value: BrowserSocialRouteKind) {
  return value === 'video' || value === 'post' || value === 'feed' || value === 'livestream';
}

function accountRouteCanSupportAnalysis(value: BrowserSocialRouteKind) {
  return value === 'account-signup' || value === 'login' || value === 'account-switch';
}

function socialAiResultClaimsAuthority(value: Infer<typeof BrowserSocialAiAnalysisResultBaseSchema>) {
  return (
    value.finalPolicyActionClaimed ||
    value.enforcementActionClaimed ||
    value.rawModelTextStored ||
    value.rawPageBodyStored ||
    value.transcriptTextStored ||
    value.rawMessageContentStored ||
    value.rawFeedContentStored ||
    value.screenshotStored ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed
  );
}
