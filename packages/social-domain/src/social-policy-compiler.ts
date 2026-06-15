import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentContractSchemaVersionSchema,
  ParentDeviceIdSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  SocialParentPolicyActionCandidateSchema,
  SocialParentPolicyCompilerModeSchema,
  SocialParentPolicyCompileRequestIdSchema,
  SocialParentPolicyConfidenceSchema,
  SocialParentPolicyDecisionCandidateIdSchema,
  SocialParentPolicyReasonCodesSchema,
  SocialParentPolicyScheduleStateSchema,
  SocialParentPolicyTargetKindSchema,
  SocialParentPolicyTimeBudgetStateSchema,
  SocialPolicyEvidenceRefsSchema,
  SocialPolicyParentRuleRefsSchema,
  SocialPolicyScheduleRefsSchema,
  SocialPolicySignalSetRefsSchema,
  SocialPolicyTimeBudgetRefsSchema,
} from './social-policy-compiler-values';
import type { SocialParentPolicyReasonCodeSchema } from './social-policy-compiler-values';

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
  timeBudgetContextRefs: SocialPolicyTimeBudgetRefsSchema,
  scheduleState: SocialParentPolicyScheduleStateSchema,
  timeBudgetState: SocialParentPolicyTimeBudgetStateSchema,
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
  timeBudgetContextRefs: SocialPolicyTimeBudgetRefsSchema,
  scheduleState: SocialParentPolicyScheduleStateSchema,
  timeBudgetState: SocialParentPolicyTimeBudgetStateSchema,
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
    timeBudgetContextRefs: input.timeBudgetContextRefs,
    scheduleState: input.scheduleState,
    timeBudgetState: input.timeBudgetState,
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
    return socialPolicyCompilerContractOnlyInputIsConsistent(value);
  }

  return socialPolicyCompilerManualInputIsConsistent(value);
}

function socialPolicyCompilerContractOnlyInputIsConsistent(
  value: Infer<typeof SocialParentPolicyCompilerInputBaseSchema>
): boolean {
  return (
    value.signalSetRefs.length > 0 &&
    value.parentRuleRefs.length > 0 &&
    value.scheduleContextRefs.length > 0 &&
    value.timeBudgetContextRefs.length > 0 &&
    value.scheduleState !== 'manual-required' &&
    value.scheduleState !== 'unavailable' &&
    value.timeBudgetState !== 'manual-required' &&
    value.timeBudgetState !== 'unavailable' &&
    value.targetKind !== 'manual-required'
  );
}

function socialPolicyCompilerManualInputIsConsistent(
  value: Infer<typeof SocialParentPolicyCompilerInputBaseSchema>
): boolean {
  return (
    value.signalSetRefs.length === 0 ||
    value.targetKind === 'manual-required' ||
    value.scheduleState === 'manual-required' ||
    value.scheduleState === 'unavailable' ||
    value.timeBudgetState === 'manual-required' ||
    value.timeBudgetState === 'unavailable'
  );
}

function socialPolicyDecisionCandidateIsConsistent(value: Infer<typeof SocialParentPolicyDecisionCandidateBaseSchema>) {
  if (socialPolicyDecisionCandidateClaimsAuthority(value)) {
    return false;
  }
  return SocialPolicyActionValidators[value.actionCandidate](value);
}

type SocialPolicyDecisionCandidateInput = Infer<typeof SocialParentPolicyDecisionCandidateBaseSchema>;
type SocialPolicyActionValidator = (value: SocialPolicyDecisionCandidateInput) => boolean;

const SocialPolicyActionValidators = {
  'unknown-candidate': unknownSocialPolicyActionIsConsistent,
  'manual-review-candidate': manualReviewSocialPolicyActionIsConsistent,
  'parent-review-candidate': askParentSocialPolicyActionIsConsistent,
  'allow-candidate': allowSocialPolicyActionIsConsistent,
  'warn-candidate': contractOnlySocialPolicyActionIsConsistent,
  'block-candidate': contractOnlySocialPolicyActionIsConsistent,
} satisfies Record<SocialPolicyDecisionCandidateInput['actionCandidate'], SocialPolicyActionValidator>;

function unknownSocialPolicyActionIsConsistent(value: SocialPolicyDecisionCandidateInput): boolean {
  return value.fallbackUsed && reasonCodesIncludeUnknownFallback(value.reasonCodes);
}

function manualReviewSocialPolicyActionIsConsistent(value: SocialPolicyDecisionCandidateInput): boolean {
  return value.fallbackUsed && value.reasonCodes.includes('manual-required');
}

function askParentSocialPolicyActionIsConsistent(value: SocialPolicyDecisionCandidateInput): boolean {
  return value.parentApprovalRequired && value.reasonCodes.includes('parent-rule-match');
}

function allowSocialPolicyActionIsConsistent(value: SocialPolicyDecisionCandidateInput): boolean {
  return SocialPolicyAllowReasons.some((reasonCode) => value.reasonCodes.includes(reasonCode));
}

function contractOnlySocialPolicyActionIsConsistent(value: SocialPolicyDecisionCandidateInput): boolean {
  return (
    value.compilerMode === 'contract-only' &&
    value.signalSetRefs.length > 0 &&
    value.scheduleContextRefs.length > 0 &&
    value.timeBudgetContextRefs.length > 0
  );
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
  return SocialPolicyUnknownFallbackReasons.some((reasonCode) => value.includes(reasonCode));
}

const SocialPolicyAllowReasons = ['social-benefit-present', 'parent-rule-match'] as const satisfies ReadonlyArray<
  Infer<typeof SocialParentPolicyReasonCodeSchema>
>;

const SocialPolicyUnknownFallbackReasons = [
  'missing-signal-proof',
  'degraded-analysis',
  'low-confidence',
  'unknown-evidence',
] as const satisfies ReadonlyArray<Infer<typeof SocialParentPolicyReasonCodeSchema>>;
