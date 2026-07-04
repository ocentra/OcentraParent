/* generated from crates/browser-core/src/social_applied_schedule_time_budget_proof.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  socialAppliedScheduleTimeBudgetHasRequiredNonClaims,
  summarizeSocialAppliedScheduleTimeBudgetRows,
} from './social_applied_schedule_time_budget_helpers';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { SocialParentPolicyDecisionCandidateSchema } from './social_policy_compiler_contract';
import {
  PolicyCompilerCapabilityState,
  SocialParentPolicyDecisionCandidateIdSchema,
  SocialParentPolicyScheduleStateSchema,
  SocialParentPolicyTimeBudgetStateSchema,
  SocialPolicyScheduleRefsSchema,
  SocialPolicyTimeBudgetRefsSchema,
} from './social_applied_schedule_time_budget_proof_support';
const SocialAppliedScheduleReferenceSchema = withParser(brandedNonEmptyStringSchema('SocialAppliedScheduleReference'));
const SocialAppliedScheduleNonClaimsSchema = Schema.Array(SocialAppliedScheduleReferenceSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social applied schedule non-claim refs')
);

const RequiredNonClaims = {
  NoRuntimeAppliedSchedule: SocialAppliedScheduleReferenceSchema.parse('no-runtime-applied-schedule'),
  NoRuntimeAppliedTimeBudget: SocialAppliedScheduleReferenceSchema.parse('no-runtime-applied-time-budget'),
  NoBrowserRuntimeGate: SocialAppliedScheduleReferenceSchema.parse('no-browser-runtime-gate'),
  NoFinalPolicyExecution: SocialAppliedScheduleReferenceSchema.parse('no-final-policy-execution'),
  NoEnforcement: SocialAppliedScheduleReferenceSchema.parse('no-enforcement'),
} as const;

export const SocialAppliedScheduleTimeBudgetState = {
  ParentOwnedApplicationEvaluated: 'parent-owned-application-evaluated',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const SocialAppliedScheduleTimeBudgetStateSchema = withParser(
  Schema.Literal(...Object.values(SocialAppliedScheduleTimeBudgetState))
);

export const SocialAppliedScheduleTimeBudgetRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    appliedScheduleTimeBudgetRowId: SocialAppliedScheduleReferenceSchema,
    sourceDecisionCandidateId: SocialParentPolicyDecisionCandidateIdSchema,
    sourceDecisionCandidate: SocialParentPolicyDecisionCandidateSchema,
    evaluatedScheduleState: SocialParentPolicyScheduleStateSchema,
    evaluatedTimeBudgetState: SocialParentPolicyTimeBudgetStateSchema,
    appliedScheduleContextRefs: SocialPolicyScheduleRefsSchema,
    appliedTimeBudgetContextRefs: SocialPolicyTimeBudgetRefsSchema,
    parentOwnedScheduleEvaluationRef: Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null),
    parentOwnedBudgetEvaluationRef: Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null),
    runtimeHandoffRef: Schema.Union(SocialAppliedScheduleReferenceSchema, Schema.Null),
    manualProofRequirements: Schema.Array(SocialAppliedScheduleReferenceSchema),
    applicationState: SocialAppliedScheduleTimeBudgetStateSchema,
    parentOwnedScheduleWindowEvaluated: Schema.Boolean,
    parentOwnedTimeBudgetEvaluated: Schema.Boolean,
    runtimeScheduleAppliedClaimed: Schema.Literal(false),
    runtimeTimeBudgetAppliedClaimed: Schema.Literal(false),
    browserRuntimeGateExecutedClaimed: Schema.Literal(false),
    finalPolicyDecisionClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    createdAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        socialAppliedScheduleTimeBudgetRowIsCoherent(row) ||
        'Expected social applied schedule/time-budget rows to match compiler candidate and runtime non-claims'
    )
  )
);

export const SocialAppliedScheduleTimeBudgetProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofId: SocialAppliedScheduleReferenceSchema,
    sourceScheduleCompilerProofRef: SocialAppliedScheduleReferenceSchema,
    rows: Schema.Array(SocialAppliedScheduleTimeBudgetRowSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected applied schedule/time-budget rows')
    ),
    nonClaims: SocialAppliedScheduleNonClaimsSchema,
    generatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (readModel) =>
        socialAppliedScheduleTimeBudgetReadModelHasRequiredNonClaims(readModel) ||
        'Expected social applied schedule/time-budget proof to preserve runtime and enforcement non-claims'
    )
  )
);

export type SocialAppliedScheduleTimeBudgetRow = Infer<typeof SocialAppliedScheduleTimeBudgetRowSchema>;
export type SocialAppliedScheduleTimeBudgetProofReadModel = Infer<
  typeof SocialAppliedScheduleTimeBudgetProofReadModelSchema
>;
type SocialAppliedScheduleTimeBudgetCandidate = {
  readonly sourceDecisionCandidateId: Infer<typeof SocialParentPolicyDecisionCandidateIdSchema>;
  readonly sourceDecisionCandidate: Infer<typeof SocialParentPolicyDecisionCandidateSchema>;
  readonly evaluatedScheduleState: Infer<typeof SocialParentPolicyScheduleStateSchema>;
  readonly evaluatedTimeBudgetState: Infer<typeof SocialParentPolicyTimeBudgetStateSchema>;
  readonly parentOwnedScheduleEvaluationRef: Infer<typeof ParentEvidenceReferenceIdSchema> | null;
  readonly parentOwnedBudgetEvaluationRef: Infer<typeof ParentEvidenceReferenceIdSchema> | null;
  readonly runtimeHandoffRef: Infer<typeof SocialAppliedScheduleReferenceSchema> | null;
  readonly manualProofRequirements: ReadonlyArray<Infer<typeof SocialAppliedScheduleReferenceSchema>>;
  readonly applicationState: Infer<typeof SocialAppliedScheduleTimeBudgetStateSchema>;
  readonly parentOwnedScheduleWindowEvaluated: boolean;
  readonly parentOwnedTimeBudgetEvaluated: boolean;
};
type SocialAppliedScheduleReference = Infer<typeof SocialAppliedScheduleReferenceSchema>;

const ReadyDecisionCandidate = SocialParentPolicyDecisionCandidateSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionCandidateId: 'social-policy-decision-candidate-schedule-video',
  compileRequestId: 'social-policy-compile-request-schedule-video',
  decidedAt: '2026-06-07T04:45:00Z',
  expiresAt: '2026-06-07T05:15:00Z',
  policyVersionRef: 'policy-version-social-school-night',
  targetKind: 'social-video',
  sourceEvidenceRefs: ['parent-evidence-social-video-route'],
  signalSetRefs: ['social-riskbenefit-signal-set-video'],
  parentRuleRefs: ['parent-rule-social-school-night'],
  scheduleContextRefs: ['schedule-context-social-school-night'],
  timeBudgetContextRefs: ['time-budget-context-social-video-daily'],
  scheduleState: 'outside-allowed-window',
  timeBudgetState: 'budget-low',
  actionCandidate: 'warn-candidate',
  reasonCodes: ['parent-rule-match', 'schedule-context'],
  confidence: 'medium',
  compilerMode: 'contract-only',
  compilerCapabilityState: PolicyCompilerCapabilityState.Supported,
  fallbackUsed: false,
  parentApprovalRequired: false,
  finalPolicyDecisionClaimed: false,
  runtimeGateExecutedClaimed: false,
  uiRenderedClaimed: false,
  enforcementClaimed: false,
  nativeAppControlClaimed: false,
  platformConnectorClaimed: false,
  rawSignalPayloadStored: false,
  rawModelTextUsed: false,
});

const ManualDecisionCandidate = SocialParentPolicyDecisionCandidateSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionCandidateId: 'social-policy-decision-candidate-schedule-manual',
  compileRequestId: 'social-policy-compile-request-schedule-manual',
  decidedAt: '2026-06-07T04:45:00Z',
  expiresAt: null,
  policyVersionRef: 'policy-version-social-manual-required',
  targetKind: 'manual-required',
  sourceEvidenceRefs: ['parent-evidence-social-native-manual-required'],
  signalSetRefs: [],
  parentRuleRefs: ['parent-rule-social-manual-required'],
  scheduleContextRefs: ['schedule-context-social-manual-required'],
  timeBudgetContextRefs: ['time-budget-context-social-manual-required'],
  scheduleState: 'manual-required',
  timeBudgetState: 'manual-required',
  actionCandidate: 'manual-review-candidate',
  reasonCodes: ['manual-required'],
  confidence: 'unknown',
  compilerMode: 'manual-required',
  compilerCapabilityState: PolicyCompilerCapabilityState.ManualRequired,
  fallbackUsed: true,
  parentApprovalRequired: false,
  finalPolicyDecisionClaimed: false,
  runtimeGateExecutedClaimed: false,
  uiRenderedClaimed: false,
  enforcementClaimed: false,
  nativeAppControlClaimed: false,
  platformConnectorClaimed: false,
  rawSignalPayloadStored: false,
  rawModelTextUsed: false,
});

export const SocialAppliedScheduleTimeBudgetProofReadModel = SocialAppliedScheduleTimeBudgetProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  proofId: 'social-applied-schedule-time-budget-proof',
  sourceScheduleCompilerProofRef: 'social-policy-schedule-time-budget-proof',
  rows: [
    {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      appliedScheduleTimeBudgetRowId: 'social-applied-schedule-budget-video-ready',
      sourceDecisionCandidateId: ReadyDecisionCandidate.decisionCandidateId,
      sourceDecisionCandidate: ReadyDecisionCandidate,
      evaluatedScheduleState: ReadyDecisionCandidate.scheduleState,
      evaluatedTimeBudgetState: ReadyDecisionCandidate.timeBudgetState,
      appliedScheduleContextRefs: ReadyDecisionCandidate.scheduleContextRefs,
      appliedTimeBudgetContextRefs: ReadyDecisionCandidate.timeBudgetContextRefs,
      parentOwnedScheduleEvaluationRef: 'parent-evidence-social-schedule-window-evaluation',
      parentOwnedBudgetEvaluationRef: 'parent-evidence-social-time-budget-evaluation',
      runtimeHandoffRef: 'social-runtime-schedule-budget-handoff-ready',
      manualProofRequirements: [],
      applicationState: SocialAppliedScheduleTimeBudgetState.ParentOwnedApplicationEvaluated,
      parentOwnedScheduleWindowEvaluated: true,
      parentOwnedTimeBudgetEvaluated: true,
      runtimeScheduleAppliedClaimed: false,
      runtimeTimeBudgetAppliedClaimed: false,
      browserRuntimeGateExecutedClaimed: false,
      finalPolicyDecisionClaimed: false,
      enforcementClaimed: false,
      createdAt: '2026-06-07T04:45:00Z',
    },
    {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      appliedScheduleTimeBudgetRowId: 'social-applied-schedule-budget-manual-required',
      sourceDecisionCandidateId: ManualDecisionCandidate.decisionCandidateId,
      sourceDecisionCandidate: ManualDecisionCandidate,
      evaluatedScheduleState: ManualDecisionCandidate.scheduleState,
      evaluatedTimeBudgetState: ManualDecisionCandidate.timeBudgetState,
      appliedScheduleContextRefs: ManualDecisionCandidate.scheduleContextRefs,
      appliedTimeBudgetContextRefs: ManualDecisionCandidate.timeBudgetContextRefs,
      parentOwnedScheduleEvaluationRef: null,
      parentOwnedBudgetEvaluationRef: null,
      runtimeHandoffRef: null,
      manualProofRequirements: ['manual-proof-runtime-social-schedule-budget-application-required'],
      applicationState: SocialAppliedScheduleTimeBudgetState.ManualRequired,
      parentOwnedScheduleWindowEvaluated: false,
      parentOwnedTimeBudgetEvaluated: false,
      runtimeScheduleAppliedClaimed: false,
      runtimeTimeBudgetAppliedClaimed: false,
      browserRuntimeGateExecutedClaimed: false,
      finalPolicyDecisionClaimed: false,
      enforcementClaimed: false,
      createdAt: '2026-06-07T04:45:00Z',
    },
  ],
  nonClaims: [
    'no-runtime-applied-schedule',
    'no-runtime-applied-time-budget',
    'no-browser-runtime-gate',
    'no-final-policy-execution',
    'no-enforcement',
  ],
  generatedAt: '2026-06-07T04:45:00Z',
});

export function summarizeSocialAppliedScheduleTimeBudgetProof(
  readModel: SocialAppliedScheduleTimeBudgetProofReadModel
) {
  return summarizeSocialAppliedScheduleTimeBudgetRows(
    readModel.rows,
    SocialAppliedScheduleTimeBudgetState.ParentOwnedApplicationEvaluated,
    SocialAppliedScheduleTimeBudgetState.ManualRequired
  );
}

function socialAppliedScheduleTimeBudgetRowIsCoherent(row: SocialAppliedScheduleTimeBudgetCandidate): boolean {
  if (!socialAppliedScheduleTimeBudgetCandidateMatchesRow(row)) {
    return false;
  }

  if (row.applicationState === SocialAppliedScheduleTimeBudgetState.ParentOwnedApplicationEvaluated) {
    return socialAppliedScheduleTimeBudgetParentOwnedRowIsCoherent(row);
  }

  return socialAppliedScheduleTimeBudgetManualRowIsCoherent(row);
}

function socialAppliedScheduleTimeBudgetCandidateMatchesRow(row: SocialAppliedScheduleTimeBudgetCandidate): boolean {
  return (
    row.sourceDecisionCandidateId === row.sourceDecisionCandidate.decisionCandidateId &&
    row.evaluatedScheduleState === row.sourceDecisionCandidate.scheduleState &&
    row.evaluatedTimeBudgetState === row.sourceDecisionCandidate.timeBudgetState
  );
}

function socialAppliedScheduleTimeBudgetParentOwnedRowIsCoherent(
  row: SocialAppliedScheduleTimeBudgetCandidate
): boolean {
  return (
    row.sourceDecisionCandidate.compilerMode === 'contract-only' &&
    row.evaluatedScheduleState !== 'manual-required' &&
    row.evaluatedScheduleState !== 'unavailable' &&
    row.evaluatedTimeBudgetState !== 'manual-required' &&
    row.evaluatedTimeBudgetState !== 'unavailable' &&
    row.parentOwnedScheduleEvaluationRef !== null &&
    row.parentOwnedBudgetEvaluationRef !== null &&
    row.runtimeHandoffRef !== null &&
    row.parentOwnedScheduleWindowEvaluated &&
    row.parentOwnedTimeBudgetEvaluated &&
    row.manualProofRequirements.length === 0
  );
}

function socialAppliedScheduleTimeBudgetManualRowIsCoherent(row: SocialAppliedScheduleTimeBudgetCandidate): boolean {
  return (
    row.sourceDecisionCandidate.compilerMode !== 'contract-only' &&
    row.parentOwnedScheduleEvaluationRef === null &&
    row.parentOwnedBudgetEvaluationRef === null &&
    row.runtimeHandoffRef === null &&
    !row.parentOwnedScheduleWindowEvaluated &&
    !row.parentOwnedTimeBudgetEvaluated &&
    row.manualProofRequirements.length > 0
  );
}

function socialAppliedScheduleTimeBudgetReadModelHasRequiredNonClaims(readModel: {
  readonly nonClaims: ReadonlyArray<SocialAppliedScheduleReference>;
}): boolean {
  return socialAppliedScheduleTimeBudgetHasRequiredNonClaims(readModel.nonClaims);
}
