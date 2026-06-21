import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  BrowserGameAiAnalysisRequestIdSchema,
  BrowserGameAiAnalysisResultIdSchema,
  BrowserGameAiAnalysisSchemaVersionSchema,
  BrowserGameAiBenefitSignalsSchema,
  BrowserGameAiConfidenceSchema,
  BrowserGameAiCustodyLabelSchema,
  BrowserGameAiDegradedStateSchema,
  BrowserGameAiEvidenceRefListSchema,
  BrowserGameAiEvidenceRefsSchema,
  BrowserGameAiModelRuntimeRefSchema,
  BrowserGameAiModifiersSchema,
  BrowserGameAiPromptTemplateVersionSchema,
  BrowserGameAiRecommendedPolicyInputSchema,
  BrowserGameAiRiskSignalsSchema,
  BrowserGameAiSummaryRefSchema,
  BrowserGameAiSurfaceKindSchema,
  BrowserGameAiTaskSchema,
  BrowserGameAiUncertaintyReasonsSchema,
} from './browser-game-ai-analysis-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const OptionalBrowserGameAiSummaryRefSchema = Schema.Union(BrowserGameAiSummaryRefSchema, Schema.Null);
const OptionalParentTimestampSchema = Schema.Union(ParentTimestampSchema, Schema.Null);

const BrowserGameAiAnalysisInputBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameAiAnalysisSchemaVersionSchema,
  requestId: BrowserGameAiAnalysisRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  requestedAt: ParentTimestampSchema,
  sourceEvidenceRefs: BrowserGameAiEvidenceRefsSchema,
  browserEvidenceRef: OptionalParentEvidenceRefSchema,
  urlShapeRef: OptionalParentEvidenceRefSchema,
  runtimeSignalRef: OptionalParentEvidenceRefSchema,
  metadataEvidenceRefs: BrowserGameAiEvidenceRefListSchema,
  screenSummaryRefs: BrowserGameAiEvidenceRefListSchema,
  parentRuleRefs: BrowserGameAiEvidenceRefListSchema,
  recentActivityRef: OptionalParentEvidenceRefSchema,
  memoryRefs: BrowserGameAiEvidenceRefListSchema,
  task: BrowserGameAiTaskSchema,
  custodyLabel: BrowserGameAiCustodyLabelSchema,
  rawUrlIncluded: Schema.Boolean,
  rawPageBodyIncluded: Schema.Boolean,
  rawGamePayloadIncluded: Schema.Boolean,
  rawScreenFrameIncluded: Schema.Boolean,
  rawModelTextIncluded: Schema.Boolean,
  accountOrPurchaseExecutionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameAiAnalysisInputCandidate = Infer<typeof BrowserGameAiAnalysisInputBaseSchema>;

const BrowserGameAiAnalysisResultBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameAiAnalysisSchemaVersionSchema,
  analysisId: BrowserGameAiAnalysisResultIdSchema,
  requestId: BrowserGameAiAnalysisRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  analyzedAt: ParentTimestampSchema,
  expiresAt: OptionalParentTimestampSchema,
  sourceEvidenceRefs: BrowserGameAiEvidenceRefsSchema,
  parentRuleRefs: BrowserGameAiEvidenceRefListSchema,
  task: BrowserGameAiTaskSchema,
  isGame: Schema.Boolean,
  gameSurfaceKind: BrowserGameAiSurfaceKindSchema,
  modifiers: BrowserGameAiModifiersSchema,
  benefitSignals: BrowserGameAiBenefitSignalsSchema,
  riskSignals: BrowserGameAiRiskSignalsSchema,
  recommendedPolicyInput: BrowserGameAiRecommendedPolicyInputSchema,
  confidence: BrowserGameAiConfidenceSchema,
  uncertaintyReasons: BrowserGameAiUncertaintyReasonsSchema,
  parentSummaryRef: BrowserGameAiSummaryRefSchema,
  childSafeSummaryRef: OptionalBrowserGameAiSummaryRefSchema,
  modelRuntimeRef: BrowserGameAiModelRuntimeRefSchema,
  promptTemplateVersion: BrowserGameAiPromptTemplateVersionSchema,
  degradedState: BrowserGameAiDegradedStateSchema,
  rawModelTextStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  rawGamePayloadStored: Schema.Boolean,
  rawScreenFrameStored: Schema.Boolean,
  accountOrPurchaseExecutionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameAiAnalysisResultCandidate = Infer<typeof BrowserGameAiAnalysisResultBaseSchema>;

export const BrowserGameAiAnalysisInputSchema = withParser(
  BrowserGameAiAnalysisInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserGameAiAnalysisInputIsHonest(value) ||
        'Expected browser-game AI input to use typed evidence refs without raw payloads or execution claims'
    )
  )
);

export const BrowserGameAiAnalysisResultSchema = withParser(
  BrowserGameAiAnalysisResultBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserGameAiAnalysisResultIsHonest(value) ||
        'Expected browser-game AI result to stay candidate-only and non-enforcing'
    )
  )
);

export const decodeBrowserGameAiAnalysisInput = Schema.decodeUnknownSync(BrowserGameAiAnalysisInputSchema);
export const decodeBrowserGameAiAnalysisResult = Schema.decodeUnknownSync(BrowserGameAiAnalysisResultSchema);

export type BrowserGameAiAnalysisInput = Infer<typeof BrowserGameAiAnalysisInputSchema>;
export type BrowserGameAiAnalysisResult = Infer<typeof BrowserGameAiAnalysisResultSchema>;

function browserGameAiAnalysisInputIsHonest(value: BrowserGameAiAnalysisInputCandidate): boolean {
  if (browserGameAiInputClaimsAuthority(value)) {
    return false;
  }
  if (value.custodyLabel === 'unmanaged-browser-bypass') {
    return value.recentActivityRef !== null && value.browserEvidenceRef === null;
  }
  if (value.custodyLabel === 'manual-required') {
    return value.runtimeSignalRef === null;
  }
  return value.browserEvidenceRef !== null;
}

function browserGameAiAnalysisResultIsHonest(value: BrowserGameAiAnalysisResultCandidate): boolean {
  if (browserGameAiResultClaimsAuthority(value)) {
    return false;
  }
  if (value.degradedState === 'none') {
    return (
      value.confidence !== 'unknown' &&
      value.uncertaintyReasons.length === 0 &&
      value.expiresAt !== null &&
      browserGameAiResultPolicyInputIsSupported(value)
    );
  }
  return (
    value.confidence !== 'high' &&
    value.uncertaintyReasons.length > 0 &&
    value.recommendedPolicyInput !== 'allow-candidate'
  );
}

function browserGameAiResultPolicyInputIsSupported(value: BrowserGameAiAnalysisResultCandidate): boolean {
  if (!value.isGame) {
    return nonGamePolicyInputIsSupported(value);
  }
  return gamePolicyInputIsSupported(value);
}

function nonGamePolicyInputIsSupported(value: BrowserGameAiAnalysisResultCandidate): boolean {
  if (!value.isGame) {
    return (
      value.gameSurfaceKind === 'unknown' &&
      value.modifiers.length === 0 &&
      value.benefitSignals.length === 0 &&
      value.riskSignals.length === 0 &&
      value.recommendedPolicyInput === 'unknown-candidate'
    );
  }
  return false;
}

const BrowserGameAiRiskBackedPolicyInputs: ReadonlyArray<
  BrowserGameAiAnalysisResultCandidate['recommendedPolicyInput']
> = ['parent-review-candidate', 'block-candidate', 'time-limit-candidate', 'warn-candidate'] as const;

function gamePolicyInputIsSupported(value: BrowserGameAiAnalysisResultCandidate): boolean {
  if (value.gameSurfaceKind === 'unknown' || value.recommendedPolicyInput === 'unknown-candidate') {
    return false;
  }
  if (value.recommendedPolicyInput === 'allow-candidate') {
    return value.benefitSignals.length > 0 && !hasHighRisk(value);
  }
  if (BrowserGameAiRiskBackedPolicyInputs.includes(value.recommendedPolicyInput)) {
    return value.riskSignals.length > 0 || value.benefitSignals.length > 0;
  }
  return value.uncertaintyReasons.length > 0 || value.confidence === 'low';
}

const BrowserGameAiHighRiskSignals: ReadonlyArray<BrowserGameAiAnalysisResultCandidate['riskSignals'][number]> = [
  'violence',
  'adult-theme',
  'gambling',
  'purchase-risk',
  'loot-box-risk',
] as const;

function hasHighRisk(value: BrowserGameAiAnalysisResultCandidate): boolean {
  return value.riskSignals.some((riskSignal) => BrowserGameAiHighRiskSignals.includes(riskSignal));
}

function browserGameAiInputClaimsAuthority(value: BrowserGameAiAnalysisInputCandidate): boolean {
  return (
    value.rawUrlIncluded ||
    value.rawPageBodyIncluded ||
    value.rawGamePayloadIncluded ||
    value.rawScreenFrameIncluded ||
    value.rawModelTextIncluded ||
    value.accountOrPurchaseExecutionClaimed ||
    value.nativeGameControlClaimed ||
    value.cloudFrameAnalysisClaimed ||
    value.finalPolicyDecisionClaimed ||
    value.runtimeGateExecutedClaimed ||
    value.enforcementClaimed
  );
}

function browserGameAiResultClaimsAuthority(value: BrowserGameAiAnalysisResultCandidate): boolean {
  return (
    value.rawModelTextStored ||
    value.rawPageBodyStored ||
    value.rawGamePayloadStored ||
    value.rawScreenFrameStored ||
    value.accountOrPurchaseExecutionClaimed ||
    value.nativeGameControlClaimed ||
    value.cloudFrameAnalysisClaimed ||
    value.finalPolicyDecisionClaimed ||
    value.runtimeGateExecutedClaimed ||
    value.uiRenderedClaimed ||
    value.enforcementClaimed
  );
}
