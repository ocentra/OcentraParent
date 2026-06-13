import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivitySubjectIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/evidence-domain/primitives';
import {
  BrowserCustodyLabelSchema,
  BrowserDomainSchema,
  BrowserPageTitleSchema,
  BrowserUrlSchema,
} from './browser-schemas';
import {
  BrowserUrlIntelligenceMemoryHitIdSchema,
  BrowserUrlShapeClassificationIdSchema,
  BrowserUrlShapePlatformIdsSchema,
  BrowserUrlShapePlatformSchema,
  BrowserUrlShapeTargetKindSchema,
} from './browser-url-intelligence-schemas';
import { BrowserUrlMetadataEvidenceIdSchema } from './browser-url-metadata-schemas';
import {
  BrowserAiConfidenceSchema,
  BrowserAiDegradedStateSchema,
  BrowserAiModelRuntimePreferenceSchema,
  BrowserAiModelRuntimeRefSchema,
  BrowserAiPromptTemplateIdSchema,
  BrowserAiPromptTemplateVersionSchema,
  BrowserAiRecommendedPolicyInputSchema,
  BrowserAiRequestedTaskSchema,
  BrowserAiUncertaintyReasonSchema,
  BrowserBenefitSignalSchema,
  BrowserContentCategorySchema,
  BrowserContentModifierSchema,
  BrowserEvidenceBackedContentRefSchema,
  BrowserKnowledgeGraphRefSchema,
  BrowserParentRuleRefSchema,
  BrowserPolicyVersionRefSchema,
  BrowserRiskSignalSchema,
  BrowserScheduleContextRefSchema,
  BrowserUrlAiAnalysisIdSchema,
  BrowserUrlAiAnalysisRequestIdSchema,
  BrowserVideoKindSchema,
} from './browser-ai-analysis-values';

export {
  BrowserAiConfidenceSchema,
  BrowserAiDegradedStateSchema,
  BrowserAiModelRuntimePreferenceSchema,
  BrowserAiModelRuntimeRefSchema,
  BrowserAiPromptTemplateIdSchema,
  BrowserAiRecommendedPolicyInputSchema,
  BrowserAiRequestedTaskSchema,
  BrowserAiUncertaintyReasonSchema,
  BrowserBenefitSignalSchema,
  BrowserContentCategorySchema,
  BrowserContentModifierSchema,
  BrowserParentRuleRefSchema,
  BrowserPolicyVersionRefSchema,
  BrowserRiskSignalSchema,
  BrowserUrlAiAnalysisIdSchema,
  BrowserUrlAiAnalysisRequestIdSchema,
} from './browser-ai-analysis-values';
export type { BrowserContentCategory, BrowserContentModifier } from './browser-ai-analysis-values';

export const BrowserAiAnalysisSchemaVersion = 1;
const OptionalBrowserAiTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const OptionalBrowserAiUrlSchema = Schema.Union(BrowserUrlSchema, Schema.Null);
const OptionalBrowserAiDomainSchema = Schema.Union(BrowserDomainSchema, Schema.Null);
const OptionalBrowserAiTitleSchema = Schema.Union(BrowserPageTitleSchema, Schema.Null);

const NonEmptyBrowserAiTextArraySchema = Schema.Array(NonEmptyStringSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser AI text ref')
);
const SourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser AI source evidence id')
);
export const BrowserBenefitSignalsSchema = Schema.Array(BrowserBenefitSignalSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser benefit signal')
);
export const BrowserRiskSignalsSchema = Schema.Array(BrowserRiskSignalSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser risk signal')
);

const BrowserAiPromptTemplateBaseSchema = Schema.Struct({
  promptTemplateId: BrowserAiPromptTemplateIdSchema,
  promptTemplateVersion: BrowserAiPromptTemplateVersionSchema,
  requestedTask: BrowserAiRequestedTaskSchema,
  allowedInputFieldRefs: NonEmptyBrowserAiTextArraySchema,
  rawPromptTextIncluded: Schema.Boolean,
  capturesRawPageBody: Schema.Boolean,
  capturesTranscriptText: Schema.Boolean,
});
export const BrowserAiPromptTemplateSchema = withParser(
  BrowserAiPromptTemplateBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiPromptTemplateIsConsistent(value) ||
        'Expected browser AI prompt template to be versioned without raw prompt or content capture'
    )
  )
);

const BrowserUrlAiAnalysisInputBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiAnalysisSchemaVersion),
  requestId: BrowserUrlAiAnalysisRequestIdSchema,
  requestedAt: ActivityTimestampSchema,
  childProfileRef: ActivitySubjectIdSchema,
  deviceId: ActivityDeviceIdSchema,
  policyVersionRef: BrowserPolicyVersionRefSchema,
  sourceEvidenceIds: SourceEvidenceIdsSchema,
  urlShapeClassificationId: BrowserUrlShapeClassificationIdSchema,
  metadataEvidenceIds: Schema.Array(BrowserUrlMetadataEvidenceIdSchema),
  memoryHitIds: Schema.Array(BrowserUrlIntelligenceMemoryHitIdSchema),
  graphRefs: Schema.Array(BrowserKnowledgeGraphRefSchema),
  parentRuleRefs: Schema.Array(BrowserParentRuleRefSchema),
  scheduleContextRefs: Schema.Array(BrowserScheduleContextRefSchema),
  normalizedUrl: OptionalBrowserAiUrlSchema,
  normalizedDomain: OptionalBrowserAiDomainSchema,
  platform: BrowserUrlShapePlatformSchema,
  platformIds: BrowserUrlShapePlatformIdsSchema,
  title: OptionalBrowserAiTitleSchema,
  description: OptionalBrowserAiTextSchema,
  transcriptRefs: Schema.Array(BrowserEvidenceBackedContentRefSchema),
  thumbnailRefs: Schema.Array(BrowserEvidenceBackedContentRefSchema),
  screenEvidenceRefs: Schema.Array(ActivityEvidenceIdSchema),
  requestedTask: BrowserAiRequestedTaskSchema,
  modelRuntimePreference: BrowserAiModelRuntimePreferenceSchema,
  promptTemplate: BrowserAiPromptTemplateSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  rawBrowserStateIncluded: Schema.Boolean,
  devToolsPayloadIncluded: Schema.Boolean,
  sqlitePathIncluded: Schema.Boolean,
  journalPathIncluded: Schema.Boolean,
  osStateIncluded: Schema.Boolean,
});
export const BrowserUrlAiAnalysisInputSchema = withParser(
  BrowserUrlAiAnalysisInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserUrlAiAnalysisInputIsConsistent(value) ||
        'Expected browser URL AI analysis input to use structured evidence refs without raw browser state'
    )
  )
);
export const decodeBrowserUrlAiAnalysisInput = Schema.decodeUnknownSync(BrowserUrlAiAnalysisInputSchema);

const BrowserUrlAiAnalysisResultBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiAnalysisSchemaVersion),
  analysisId: BrowserUrlAiAnalysisIdSchema,
  requestId: BrowserUrlAiAnalysisRequestIdSchema,
  analyzedAt: ActivityTimestampSchema,
  expiresAt: ActivityTimestampSchema,
  sourceEvidenceIds: SourceEvidenceIdsSchema,
  metadataEvidenceIds: Schema.Array(BrowserUrlMetadataEvidenceIdSchema),
  memoryHitIds: Schema.Array(BrowserUrlIntelligenceMemoryHitIdSchema),
  graphRefs: Schema.Array(BrowserKnowledgeGraphRefSchema),
  parentRuleRefs: Schema.Array(BrowserParentRuleRefSchema),
  contentKind: BrowserUrlShapeTargetKindSchema,
  videoKind: BrowserVideoKindSchema,
  contentCategory: BrowserContentCategorySchema,
  contentModifiers: Schema.Array(BrowserContentModifierSchema),
  benefitSignals: BrowserBenefitSignalsSchema,
  riskSignals: BrowserRiskSignalsSchema,
  recommendedPolicyInput: BrowserAiRecommendedPolicyInputSchema,
  confidence: BrowserAiConfidenceSchema,
  uncertaintyReasons: Schema.Array(BrowserAiUncertaintyReasonSchema),
  parentSummary: NonEmptyStringSchema,
  childSafeSummary: OptionalBrowserAiTextSchema,
  modelRuntimeRef: BrowserAiModelRuntimeRefSchema,
  promptTemplate: BrowserAiPromptTemplateSchema,
  degradedState: BrowserAiDegradedStateSchema,
  finalPolicyActionClaimed: Schema.Boolean,
  enforcementActionClaimed: Schema.Boolean,
  rawContentStored: Schema.Boolean,
});
export const BrowserUrlAiAnalysisResultSchema = withParser(
  BrowserUrlAiAnalysisResultBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserUrlAiAnalysisResultIsConsistent(value) ||
        'Expected browser URL AI analysis result to be candidate-only, evidence-backed, and degradation-aware'
    )
  )
);
export const decodeBrowserUrlAiAnalysisResult = Schema.decodeUnknownSync(BrowserUrlAiAnalysisResultSchema);

export type BrowserBenefitSignals = Infer<typeof BrowserBenefitSignalsSchema>;
export type BrowserRiskSignals = Infer<typeof BrowserRiskSignalsSchema>;
export type BrowserAiPromptTemplate = Infer<typeof BrowserAiPromptTemplateSchema>;
export type BrowserUrlAiAnalysisInput = Infer<typeof BrowserUrlAiAnalysisInputSchema>;
export type BrowserUrlAiAnalysisResult = Infer<typeof BrowserUrlAiAnalysisResultSchema>;

function browserAiPromptTemplateIsConsistent(value: Infer<typeof BrowserAiPromptTemplateBaseSchema>) {
  return !value.rawPromptTextIncluded && !value.capturesRawPageBody && !value.capturesTranscriptText;
}

function browserUrlAiAnalysisInputIsConsistent(value: Infer<typeof BrowserUrlAiAnalysisInputBaseSchema>) {
  if (browserUrlAiAnalysisInputHasForbiddenRawState(value)) {
    return false;
  }
  if (value.normalizedUrl !== null && value.normalizedDomain === null) {
    return false;
  }
  return value.promptTemplate.requestedTask === value.requestedTask;
}

function browserUrlAiAnalysisInputHasForbiddenRawState(value: Infer<typeof BrowserUrlAiAnalysisInputBaseSchema>) {
  return (
    value.rawBrowserStateIncluded ||
    value.devToolsPayloadIncluded ||
    value.sqlitePathIncluded ||
    value.journalPathIncluded ||
    value.osStateIncluded
  );
}

function browserUrlAiAnalysisResultIsConsistent(value: Infer<typeof BrowserUrlAiAnalysisResultBaseSchema>) {
  if (value.finalPolicyActionClaimed || value.enforcementActionClaimed || value.rawContentStored) {
    return false;
  }
  if (!browserAiVideoKindMatchesContentKind(value.contentKind, value.videoKind)) {
    return false;
  }
  if (value.degradedState === 'none') {
    return value.uncertaintyReasons.length === 0 && value.confidence !== 'unknown';
  }
  return value.uncertaintyReasons.length > 0 && value.confidence !== 'high';
}

function browserAiVideoKindMatchesContentKind(contentKind: string, videoKind: string) {
  if (contentKind === 'video' || contentKind === 'short-video' || contentKind === 'channel') {
    return videoKind !== 'non-video';
  }
  if (contentKind === 'playlist') {
    return videoKind === 'playlist' || videoKind === 'unknown';
  }
  return videoKind === 'non-video' || videoKind === 'unknown';
}

