import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import {
  BrowserAiConfidenceSchema,
  BrowserAiDegradedStateSchema,
  BrowserAiRecommendedPolicyInputSchema,
  BrowserAiUncertaintyReasonSchema,
} from './browser-ai-analysis-schemas';
import {
  BrowserSocialAiAnalysisInputSchema,
  type BrowserSocialAiAnalysisResult,
  BrowserSocialAiAnalysisResultSchema,
} from './browser-social-ai-analysis-schemas';
import {
  BrowserSocialAiAnalysisIdSchema,
  BrowserSocialAiAnalysisSchemaVersion,
  NonEmptySocialAiTextSchema,
  OptionalSocialAiRuntimeRefSchema,
  OptionalSocialAiTextSchema,
  SocialAiClassificationsSchema,
  SocialAiTextRefsSchema,
} from './browser-social-ai-analysis-values';

const BrowserSocialAiAnalysisResultRequestSchema = withParser(
  Schema.Struct({
    analysisId: BrowserSocialAiAnalysisIdSchema,
    analyzedAt: ActivityTimestampSchema,
    expiresAt: ActivityTimestampSchema,
    input: BrowserSocialAiAnalysisInputSchema,
    classifications: SocialAiClassificationsSchema,
    riskSignalRefs: SocialAiTextRefsSchema,
    benefitSignalRefs: SocialAiTextRefsSchema,
    recommendedPolicyInput: BrowserAiRecommendedPolicyInputSchema,
    confidence: BrowserAiConfidenceSchema,
    uncertaintyReasons: Schema.Array(BrowserAiUncertaintyReasonSchema),
    parentSummaryRef: NonEmptySocialAiTextSchema,
    childSafeSummaryRef: OptionalSocialAiTextSchema,
    modelRuntimeRef: OptionalSocialAiRuntimeRefSchema,
    degradedState: BrowserAiDegradedStateSchema,
  })
);

export type BrowserSocialAiAnalysisResultRequest = Infer<typeof BrowserSocialAiAnalysisResultRequestSchema>;

export function buildBrowserSocialAiAnalysisResult(
  request: BrowserSocialAiAnalysisResultRequest
): BrowserSocialAiAnalysisResult {
  const parsed = BrowserSocialAiAnalysisResultRequestSchema.parse(request);

  return BrowserSocialAiAnalysisResultSchema.parse({
    schemaVersion: BrowserSocialAiAnalysisSchemaVersion,
    analysisId: parsed.analysisId,
    requestId: parsed.input.requestId,
    analyzedAt: parsed.analyzedAt,
    expiresAt: parsed.expiresAt,
    sourceEvidenceIds: parsed.input.sourceEvidenceIds,
    socialRouteEvidenceId: parsed.input.socialRouteEvidenceId,
    platform: parsed.input.platform,
    routeKind: parsed.input.routeKind,
    requestedTask: parsed.input.requestedTask,
    classifications: parsed.classifications,
    riskSignalRefs: parsed.riskSignalRefs,
    benefitSignalRefs: parsed.benefitSignalRefs,
    recommendedPolicyInput: parsed.recommendedPolicyInput,
    confidence: parsed.confidence,
    uncertaintyReasons: parsed.uncertaintyReasons,
    parentSummaryRef: parsed.parentSummaryRef,
    childSafeSummaryRef: parsed.childSafeSummaryRef,
    modelRuntimeRef: parsed.modelRuntimeRef,
    promptTemplate: parsed.input.promptTemplate,
    degradedState: parsed.degradedState,
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
    rawModelTextStored: false,
    rawPageBodyStored: false,
    transcriptTextStored: false,
    rawMessageContentStored: false,
    rawFeedContentStored: false,
    screenshotStored: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  });
}
