import { type Infer, Schema, withParser } from './effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentContractSchemaVersionSchema,
  ParentDeviceIdSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import { PolicyCompilerCapabilityStateSchema } from './policy-compiler';
import {
  BrowserGamePolicyActionCandidateSchema,
  BrowserGamePolicyAnalysisRefsSchema,
  BrowserGamePolicyCompileRequestIdSchema,
  BrowserGamePolicyCompilerModeSchema,
  BrowserGamePolicyConfidenceSchema,
  BrowserGamePolicyDecisionCandidateIdSchema,
  BrowserGamePolicyEvidenceRefsSchema,
  BrowserGamePolicyMobileCapabilityRefsSchema,
  BrowserGamePolicyParentRuleRefsSchema,
  BrowserGamePolicyReasonCodesSchema,
  BrowserGamePolicyScheduleRefsSchema,
  BrowserGamePolicyTargetKindSchema,
} from './browser-game-policy-compiler-values';
import type { BrowserGamePolicyReasonCodeSchema } from './browser-game-policy-compiler-values';

const BrowserGamePolicyCompilerInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  compileRequestId: BrowserGamePolicyCompileRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  requestedAt: ParentTimestampSchema,
  policyVersionRef: ParentPolicyVersionSchema,
  targetKind: BrowserGamePolicyTargetKindSchema,
  sourceEvidenceRefs: BrowserGamePolicyEvidenceRefsSchema,
  analysisRefs: BrowserGamePolicyAnalysisRefsSchema,
  mobileCapabilityRefs: BrowserGamePolicyMobileCapabilityRefsSchema,
  parentRuleRefs: BrowserGamePolicyParentRuleRefsSchema,
  scheduleContextRefs: BrowserGamePolicyScheduleRefsSchema,
  compilerMode: BrowserGamePolicyCompilerModeSchema,
  rawGamePayloadIncluded: Schema.Boolean,
  rawModelTextIncluded: Schema.Boolean,
  activityDomainObjectIncluded: Schema.Boolean,
  finalDecisionClaimedByInput: Schema.Boolean,
  runtimeGateClaimedByInput: Schema.Boolean,
  uiClaimedByInput: Schema.Boolean,
  enforcementClaimedByInput: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
});

export const BrowserGamePolicyCompilerInputSchema = withParser(
  BrowserGamePolicyCompilerInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserGamePolicyCompilerInputIsConsistent(value) ||
        'Expected browser game policy compiler input to use parent-owned refs without runtime, UI, native-game, cloud-frame, or enforcement authority'
    )
  )
);

const BrowserGamePolicyDecisionCandidateBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  decisionCandidateId: BrowserGamePolicyDecisionCandidateIdSchema,
  compileRequestId: BrowserGamePolicyCompileRequestIdSchema,
  decidedAt: ParentTimestampSchema,
  expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  policyVersionRef: ParentPolicyVersionSchema,
  targetKind: BrowserGamePolicyTargetKindSchema,
  sourceEvidenceRefs: BrowserGamePolicyEvidenceRefsSchema,
  analysisRefs: BrowserGamePolicyAnalysisRefsSchema,
  mobileCapabilityRefs: BrowserGamePolicyMobileCapabilityRefsSchema,
  parentRuleRefs: BrowserGamePolicyParentRuleRefsSchema,
  scheduleContextRefs: BrowserGamePolicyScheduleRefsSchema,
  actionCandidate: BrowserGamePolicyActionCandidateSchema,
  reasonCodes: BrowserGamePolicyReasonCodesSchema,
  confidence: BrowserGamePolicyConfidenceSchema,
  compilerMode: BrowserGamePolicyCompilerModeSchema,
  compilerCapabilityState: PolicyCompilerCapabilityStateSchema,
  fallbackUsed: Schema.Boolean,
  parentApprovalRequired: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  rawGamePayloadStored: Schema.Boolean,
  rawModelTextUsed: Schema.Boolean,
});

export const BrowserGamePolicyDecisionCandidateSchema = withParser(
  BrowserGamePolicyDecisionCandidateBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserGamePolicyDecisionCandidateIsConsistent(value) ||
        'Expected browser game policy decision candidate to remain non-final and non-enforcing'
    )
  )
);

export const decodeBrowserGamePolicyCompilerInput = Schema.decodeUnknownSync(BrowserGamePolicyCompilerInputSchema);
export const decodeBrowserGamePolicyDecisionCandidate = Schema.decodeUnknownSync(
  BrowserGamePolicyDecisionCandidateSchema
);

export type BrowserGamePolicyCompilerInput = Infer<typeof BrowserGamePolicyCompilerInputSchema>;
export type BrowserGamePolicyDecisionCandidate = Infer<typeof BrowserGamePolicyDecisionCandidateSchema>;

function browserGamePolicyCompilerInputIsConsistent(value: Infer<typeof BrowserGamePolicyCompilerInputBaseSchema>) {
  if (browserGamePolicyCompilerInputClaimsAuthority(value)) {
    return false;
  }
  if (value.compilerMode === 'contract-only') {
    return value.analysisRefs.length > 0 && value.parentRuleRefs.length > 0 && value.targetKind !== 'manual-required';
  }
  return value.analysisRefs.length === 0 || value.targetKind === 'manual-required';
}

function browserGamePolicyDecisionCandidateIsConsistent(
  value: Infer<typeof BrowserGamePolicyDecisionCandidateBaseSchema>
) {
  if (browserGamePolicyDecisionCandidateClaimsAuthority(value)) {
    return false;
  }
  return BrowserGamePolicyActionValidators[value.actionCandidate](value);
}

type BrowserGamePolicyDecisionCandidateInput = Infer<typeof BrowserGamePolicyDecisionCandidateBaseSchema>;
type BrowserGamePolicyActionValidator = (value: BrowserGamePolicyDecisionCandidateInput) => boolean;

const BrowserGamePolicyActionValidators = {
  'unknown-candidate': unknownBrowserGamePolicyActionIsConsistent,
  'manual-review-candidate': manualReviewBrowserGamePolicyActionIsConsistent,
  'parent-review-candidate': askParentBrowserGamePolicyActionIsConsistent,
  'allow-candidate': allowBrowserGamePolicyActionIsConsistent,
  'time-limit-candidate': timeLimitBrowserGamePolicyActionIsConsistent,
  'warn-candidate': contractOnlyBrowserGamePolicyActionIsConsistent,
  'block-candidate': contractOnlyBrowserGamePolicyActionIsConsistent,
} satisfies Record<BrowserGamePolicyDecisionCandidateInput['actionCandidate'], BrowserGamePolicyActionValidator>;

function unknownBrowserGamePolicyActionIsConsistent(value: BrowserGamePolicyDecisionCandidateInput): boolean {
  return value.fallbackUsed && reasonCodesIncludeUnknownFallback(value.reasonCodes);
}

function manualReviewBrowserGamePolicyActionIsConsistent(value: BrowserGamePolicyDecisionCandidateInput): boolean {
  return value.fallbackUsed && value.reasonCodes.includes('manual-required');
}

function askParentBrowserGamePolicyActionIsConsistent(value: BrowserGamePolicyDecisionCandidateInput): boolean {
  return value.parentApprovalRequired && value.reasonCodes.includes('parent-rule-match');
}

function allowBrowserGamePolicyActionIsConsistent(value: BrowserGamePolicyDecisionCandidateInput): boolean {
  return BrowserGamePolicyAllowReasons.some((reasonCode) => value.reasonCodes.includes(reasonCode));
}

function timeLimitBrowserGamePolicyActionIsConsistent(value: BrowserGamePolicyDecisionCandidateInput): boolean {
  return value.reasonCodes.includes('schedule-context') && value.reasonCodes.includes('parent-rule-match');
}

function contractOnlyBrowserGamePolicyActionIsConsistent(value: BrowserGamePolicyDecisionCandidateInput): boolean {
  return value.compilerMode === 'contract-only' && value.analysisRefs.length > 0;
}

function browserGamePolicyCompilerInputClaimsAuthority(value: Infer<typeof BrowserGamePolicyCompilerInputBaseSchema>) {
  return (
    value.rawGamePayloadIncluded ||
    value.rawModelTextIncluded ||
    value.activityDomainObjectIncluded ||
    value.finalDecisionClaimedByInput ||
    value.runtimeGateClaimedByInput ||
    value.uiClaimedByInput ||
    value.enforcementClaimedByInput ||
    value.nativeGameControlClaimed ||
    value.cloudFrameAnalysisClaimed
  );
}

function browserGamePolicyDecisionCandidateClaimsAuthority(
  value: Infer<typeof BrowserGamePolicyDecisionCandidateBaseSchema>
) {
  return (
    value.finalPolicyDecisionClaimed ||
    value.runtimeGateExecutedClaimed ||
    value.uiRenderedClaimed ||
    value.enforcementClaimed ||
    value.nativeGameControlClaimed ||
    value.cloudFrameAnalysisClaimed ||
    value.rawGamePayloadStored ||
    value.rawModelTextUsed
  );
}

function reasonCodesIncludeUnknownFallback(value: ReadonlyArray<Infer<typeof BrowserGamePolicyReasonCodeSchema>>) {
  return BrowserGamePolicyUnknownFallbackReasons.some((reasonCode) => value.includes(reasonCode));
}

const BrowserGamePolicyAllowReasons = [
  'educational-benefit-present',
  'parent-rule-match',
] as const satisfies ReadonlyArray<Infer<typeof BrowserGamePolicyReasonCodeSchema>>;

const BrowserGamePolicyUnknownFallbackReasons = [
  'missing-game-evidence',
  'degraded-analysis',
  'low-confidence',
  'unknown-evidence',
] as const satisfies ReadonlyArray<Infer<typeof BrowserGamePolicyReasonCodeSchema>>;
