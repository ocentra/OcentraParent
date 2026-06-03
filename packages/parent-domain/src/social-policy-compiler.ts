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
  SocialParentPolicyActionCandidateSchema,
  SocialParentPolicyCompilerModeSchema,
  SocialParentPolicyCompileRequestIdSchema,
  SocialParentPolicyConfidenceSchema,
  SocialParentPolicyDecisionCandidateIdSchema,
  SocialParentPolicyReasonCodeSchema,
  SocialParentPolicyReasonCodesSchema,
  SocialParentPolicyTargetKindSchema,
  SocialPolicyEvidenceRefsSchema,
  SocialPolicyParentRuleRefsSchema,
  SocialPolicyScheduleRefsSchema,
  SocialPolicySignalSetRefsSchema,
} from './social-policy-compiler-values';

const SocialParentPolicyCompilerInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  compileRequestId: SocialParentPolicyCompileRequestIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  requestedAt: ParentTimestampSchema,
  policyVersionRef: ParentPolicyVersionSchema,
  targetKind: SocialParentPolicyTargetKindSchema,
  sourceEvidenceRefs: SocialPolicyEvidenceRefsSchema,
  signalSetRefs: SocialPolicySignalSetRefsSchema,
  parentRuleRefs: SocialPolicyParentRuleRefsSchema,
  scheduleContextRefs: SocialPolicyScheduleRefsSchema,
  compilerMode: SocialParentPolicyCompilerModeSchema,
  rawSignalPayloadIncluded: Schema.Boolean,
  rawModelTextIncluded: Schema.Boolean,
  activityDomainObjectIncluded: Schema.Boolean,
  finalDecisionClaimedByInput: Schema.Boolean,
  runtimeGateClaimedByInput: Schema.Boolean,
  uiClaimedByInput: Schema.Boolean,
  enforcementClaimedByInput: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
});
export const SocialParentPolicyCompilerInputSchema = withParser(
  SocialParentPolicyCompilerInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialPolicyCompilerInputIsConsistent(value) ||
        'Expected social policy compiler input to use parent-owned refs without runtime authority'
    )
  )
);

const SocialParentPolicyDecisionCandidateBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  decisionCandidateId: SocialParentPolicyDecisionCandidateIdSchema,
  compileRequestId: SocialParentPolicyCompileRequestIdSchema,
  decidedAt: ParentTimestampSchema,
  expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  policyVersionRef: ParentPolicyVersionSchema,
  targetKind: SocialParentPolicyTargetKindSchema,
  sourceEvidenceRefs: SocialPolicyEvidenceRefsSchema,
  signalSetRefs: SocialPolicySignalSetRefsSchema,
  parentRuleRefs: SocialPolicyParentRuleRefsSchema,
  scheduleContextRefs: SocialPolicyScheduleRefsSchema,
  actionCandidate: SocialParentPolicyActionCandidateSchema,
  reasonCodes: SocialParentPolicyReasonCodesSchema,
  confidence: SocialParentPolicyConfidenceSchema,
  compilerMode: SocialParentPolicyCompilerModeSchema,
  fallbackUsed: Schema.Boolean,
  parentApprovalRequired: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
  rawSignalPayloadStored: Schema.Boolean,
  rawModelTextUsed: Schema.Boolean,
});
export const SocialParentPolicyDecisionCandidateSchema = withParser(
  SocialParentPolicyDecisionCandidateBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialPolicyDecisionCandidateIsConsistent(value) ||
        'Expected social policy decision candidate to remain non-final and non-enforcing'
    )
  )
);

const SocialParentPolicyCompileRequestSchema = withParser(
  Schema.Struct({
    input: SocialParentPolicyCompilerInputSchema,
    decisionCandidateId: SocialParentPolicyDecisionCandidateIdSchema,
    decidedAt: ParentTimestampSchema,
    expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    actionCandidate: SocialParentPolicyActionCandidateSchema,
    reasonCodes: SocialParentPolicyReasonCodesSchema,
    confidence: SocialParentPolicyConfidenceSchema,
    fallbackUsed: Schema.Boolean,
    parentApprovalRequired: Schema.Boolean,
  })
);

export const decodeSocialParentPolicyCompilerInput = Schema.decodeUnknownSync(SocialParentPolicyCompilerInputSchema);
export const decodeSocialParentPolicyDecisionCandidate = Schema.decodeUnknownSync(
  SocialParentPolicyDecisionCandidateSchema
);

export type SocialParentPolicyCompilerInput = Infer<typeof SocialParentPolicyCompilerInputSchema>;
export type SocialParentPolicyDecisionCandidate = Infer<typeof SocialParentPolicyDecisionCandidateSchema>;

export function compileSocialParentPolicyCandidate(
  request: Infer<typeof SocialParentPolicyCompileRequestSchema>
): SocialParentPolicyDecisionCandidate {
  const parsed = SocialParentPolicyCompileRequestSchema.parse(request);
  const input = parsed.input;

  return SocialParentPolicyDecisionCandidateSchema.parse({
    schemaVersion: input.schemaVersion,
    decisionCandidateId: parsed.decisionCandidateId,
    compileRequestId: input.compileRequestId,
    decidedAt: parsed.decidedAt,
    expiresAt: parsed.expiresAt,
    policyVersionRef: input.policyVersionRef,
    targetKind: input.targetKind,
    sourceEvidenceRefs: input.sourceEvidenceRefs,
    signalSetRefs: input.signalSetRefs,
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
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    rawSignalPayloadStored: false,
    rawModelTextUsed: false,
  });
}

function socialPolicyCompilerInputIsConsistent(value: Infer<typeof SocialParentPolicyCompilerInputBaseSchema>) {
  if (socialPolicyCompilerInputClaimsAuthority(value)) {
    return false;
  }
  if (value.compilerMode === 'contract-only') {
    return value.signalSetRefs.length > 0 && value.parentRuleRefs.length > 0 && value.targetKind !== 'manual-required';
  }
  return value.signalSetRefs.length === 0 || value.targetKind === 'manual-required';
}

function socialPolicyDecisionCandidateIsConsistent(value: Infer<typeof SocialParentPolicyDecisionCandidateBaseSchema>) {
  if (socialPolicyDecisionCandidateClaimsAuthority(value)) {
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
    return value.reasonCodes.includes('social-benefit-present') || value.reasonCodes.includes('parent-rule-match');
  }
  return value.compilerMode === 'contract-only' && value.signalSetRefs.length > 0;
}

function socialPolicyCompilerInputClaimsAuthority(value: Infer<typeof SocialParentPolicyCompilerInputBaseSchema>) {
  return (
    value.rawSignalPayloadIncluded ||
    value.rawModelTextIncluded ||
    value.activityDomainObjectIncluded ||
    value.finalDecisionClaimedByInput ||
    value.runtimeGateClaimedByInput ||
    value.uiClaimedByInput ||
    value.enforcementClaimedByInput ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed
  );
}

function socialPolicyDecisionCandidateClaimsAuthority(
  value: Infer<typeof SocialParentPolicyDecisionCandidateBaseSchema>
) {
  return (
    value.finalPolicyDecisionClaimed ||
    value.runtimeGateExecutedClaimed ||
    value.uiRenderedClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed ||
    value.rawSignalPayloadStored ||
    value.rawModelTextUsed
  );
}

function reasonCodesIncludeUnknownFallback(value: ReadonlyArray<Infer<typeof SocialParentPolicyReasonCodeSchema>>) {
  return (
    value.includes('missing-signal-proof') ||
    value.includes('degraded-analysis') ||
    value.includes('low-confidence') ||
    value.includes('unknown-evidence')
  );
}
