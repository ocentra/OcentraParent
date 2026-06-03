import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentContractSchemaVersionSchema,
  ParentDeviceIdSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
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
  BrowserGamePolicyReasonCodeSchema,
  BrowserGamePolicyReasonCodesSchema,
  BrowserGamePolicyScheduleRefsSchema,
  BrowserGamePolicyTargetKindSchema,
} from './browser-game-policy-compiler-values';

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

const BrowserGamePolicyCompileRequestSchema = withParser(
  Schema.Struct({
    input: BrowserGamePolicyCompilerInputSchema,
    decisionCandidateId: BrowserGamePolicyDecisionCandidateIdSchema,
    decidedAt: ParentTimestampSchema,
    expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    actionCandidate: BrowserGamePolicyActionCandidateSchema,
    reasonCodes: BrowserGamePolicyReasonCodesSchema,
    confidence: BrowserGamePolicyConfidenceSchema,
    fallbackUsed: Schema.Boolean,
    parentApprovalRequired: Schema.Boolean,
  })
);

export const decodeBrowserGamePolicyCompilerInput = Schema.decodeUnknownSync(BrowserGamePolicyCompilerInputSchema);
export const decodeBrowserGamePolicyDecisionCandidate = Schema.decodeUnknownSync(
  BrowserGamePolicyDecisionCandidateSchema
);

export type BrowserGamePolicyCompilerInput = Infer<typeof BrowserGamePolicyCompilerInputSchema>;
export type BrowserGamePolicyDecisionCandidate = Infer<typeof BrowserGamePolicyDecisionCandidateSchema>;

export function compileBrowserGamePolicyCandidate(
  request: Infer<typeof BrowserGamePolicyCompileRequestSchema>
): BrowserGamePolicyDecisionCandidate {
  const parsed = BrowserGamePolicyCompileRequestSchema.parse(request);
  const input = parsed.input;

  return BrowserGamePolicyDecisionCandidateSchema.parse({
    schemaVersion: input.schemaVersion,
    decisionCandidateId: parsed.decisionCandidateId,
    compileRequestId: input.compileRequestId,
    decidedAt: parsed.decidedAt,
    expiresAt: parsed.expiresAt,
    policyVersionRef: input.policyVersionRef,
    targetKind: input.targetKind,
    sourceEvidenceRefs: input.sourceEvidenceRefs,
    analysisRefs: input.analysisRefs,
    mobileCapabilityRefs: input.mobileCapabilityRefs,
    parentRuleRefs: input.parentRuleRefs,
    scheduleContextRefs: input.scheduleContextRefs,
    actionCandidate: parsed.actionCandidate,
    reasonCodes: parsed.reasonCodes,
    confidence: parsed.confidence,
    compilerMode: input.compilerMode,
    fallbackUsed: parsed.fallbackUsed,
    parentApprovalRequired: parsed.parentApprovalRequired,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    rawGamePayloadStored: false,
    rawModelTextUsed: false,
  });
}

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
  if (value.actionCandidate === 'unknown-candidate') {
    return value.fallbackUsed && reasonCodesIncludeUnknownFallback(value.reasonCodes);
  }
  if (value.actionCandidate === 'manual-review-candidate') {
    return value.fallbackUsed && value.reasonCodes.includes('manual-required');
  }
  if (value.actionCandidate === 'ask-parent-candidate') {
    return value.parentApprovalRequired && value.reasonCodes.includes('parent-rule-match');
  }
  if (value.actionCandidate === 'allow-candidate') {
    return value.reasonCodes.includes('educational-benefit-present') || value.reasonCodes.includes('parent-rule-match');
  }
  if (value.actionCandidate === 'time-limit-candidate') {
    return value.reasonCodes.includes('schedule-context') && value.reasonCodes.includes('parent-rule-match');
  }
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
  return (
    value.includes('missing-game-evidence') ||
    value.includes('degraded-analysis') ||
    value.includes('low-confidence') ||
    value.includes('unknown-evidence')
  );
}
