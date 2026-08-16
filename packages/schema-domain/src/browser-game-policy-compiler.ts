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
import {
  browserGamePolicyCompilerInputIsConsistentGenerated,
  browserGamePolicyDecisionCandidateIsConsistentGenerated,
} from './generated-browser-policy-control-catalog-helpers';

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
  return browserGamePolicyCompilerInputIsConsistentGenerated(value);
}

function browserGamePolicyDecisionCandidateIsConsistent(
  value: Infer<typeof BrowserGamePolicyDecisionCandidateBaseSchema>
) {
  return browserGamePolicyDecisionCandidateIsConsistentGenerated(value);
}
