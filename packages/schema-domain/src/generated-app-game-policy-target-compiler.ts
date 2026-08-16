/* generated from crates/app-game-core/src/app_game_policy_target_compiler_generated_ts.rs */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  AppGamePolicyCompilerAuthorityState,
  AppGamePolicyCompilerEvidenceState,
  AppGamePolicyCompilerOutcomeState,
  AppGamePolicyCompilerProofKind,
  AppGamePolicyCompilerRejectionReason,
  AppGamePolicyCompilerRequestedAction,
  AppGamePolicyTargetKind,
  appGamePolicyBlockLaunchWithoutProofIsManualRequired,
  appGamePolicyCapabilityRefsKeepNonReadyStatesExplicit,
  appGamePolicyCompiledDecisionCarriesProofRefs,
  appGamePolicyHardActionProofIsComplete,
  appGamePolicyRequestHasCapabilityRef,
  appGamePolicyRequestHasFreshLocalEvidence,
  appGamePolicyRequestHasScheduleProof,
  appGamePolicyTargetProofIsComplete,
} from './app-game-policy-target-compiler-rules';
import { ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from './family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  PolicyActionSchema,
  PolicyDecisionSchema,
  PolicyRuleIdSchema,
  PolicyScheduleIdSchema,
  PolicyTargetSchema,
} from './policy-contracts';
import { PolicyCompilerCapabilityStateSchema } from './policy-compiler';

export const AppGamePolicyCompileRequestIdSchema = brandedNonEmptyStringSchema('AppGamePolicyCompileRequestId');
export const AppGamePolicyCompiledDecisionIdSchema = brandedNonEmptyStringSchema('AppGamePolicyCompiledDecisionId');
export const AppGamePolicyTargetRefSchema = brandedNonEmptyStringSchema('AppGamePolicyTargetRef');
export const AppGamePolicyLocalUserRefSchema = brandedNonEmptyStringSchema('AppGamePolicyLocalUserRef');
export const AppGamePolicyCapabilityRefSchema = brandedNonEmptyStringSchema('AppGamePolicyCapabilityRef');
export const AppGamePolicyAuthorityRefSchema = brandedNonEmptyStringSchema('AppGamePolicyAuthorityRef');
export const AppGamePolicyAuditRefSchema = brandedNonEmptyStringSchema('AppGamePolicyAuditRef');

export const AppGamePolicyTargetKindSchema = withParser(Schema.Literal(...Object.values(AppGamePolicyTargetKind)));
export const AppGamePolicyCompilerProofKindSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyCompilerProofKind))
);
export const AppGamePolicyCompilerEvidenceStateSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyCompilerEvidenceState))
);
export const AppGamePolicyCompilerCapabilityStateSchema = PolicyCompilerCapabilityStateSchema;
export const AppGamePolicyCompilerAuthorityStateSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyCompilerAuthorityState))
);
export const AppGamePolicyCompilerRequestedActionSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyCompilerRequestedAction))
);
export const AppGamePolicyCompilerOutcomeStateSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyCompilerOutcomeState))
);
export const AppGamePolicyCompilerRejectionReasonSchema = withParser(
  Schema.Literal(...Object.values(AppGamePolicyCompilerRejectionReason))
);

export const AppGamePolicyCompilerTargetSchema = withParser(
  Schema.Struct({
    targetKind: AppGamePolicyTargetKindSchema,
    targetRef: Schema.Union(AppGamePolicyTargetRefSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (target) =>
        target.targetRef !== null ||
        target.targetKind === AppGamePolicyTargetKind.AllGames ||
        target.targetKind === AppGamePolicyTargetKind.AllNonSystemApps ||
        'Expected concrete app/game policy targets to carry a target ref'
    )
  )
);

export const AppGamePolicyCompilerEvidenceSchema = withParser(
  Schema.Struct({
    evidenceReference: ParentEvidenceReferenceSchema,
    proofKind: AppGamePolicyCompilerProofKindSchema,
    evidenceState: AppGamePolicyCompilerEvidenceStateSchema,
    device: ParentDeviceReferenceSchema,
    localUserRef: AppGamePolicyLocalUserRefSchema,
    observedAt: ParentTimestampSchema,
  })
);

export const AppGamePolicyCompilerCapabilityRefSchema = withParser(
  Schema.Struct({
    capabilityRef: AppGamePolicyCapabilityRefSchema,
    capabilityState: AppGamePolicyCompilerCapabilityStateSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  }).pipe(
    Schema.filter(
      (capability) =>
        capability.evidenceReferences.length > 0 || 'Expected app/game compiler capability refs to cite evidence'
    )
  )
);

export const AppGamePolicyCompilerAuthorityRefSchema = withParser(
  Schema.Struct({
    authorityRef: AppGamePolicyAuthorityRefSchema,
    authorityState: AppGamePolicyCompilerAuthorityStateSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  }).pipe(
    Schema.filter(
      (authority) =>
        authority.evidenceReferences.length > 0 || 'Expected app/game compiler authority refs to cite evidence'
    )
  )
);

export const AppGamePolicyCompileRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    compileRequestId: AppGamePolicyCompileRequestIdSchema,
    policyVersion: ParentPolicyVersionSchema,
    ruleId: PolicyRuleIdSchema,
    device: ParentDeviceReferenceSchema,
    localUserRef: AppGamePolicyLocalUserRefSchema,
    target: AppGamePolicyCompilerTargetSchema,
    requestedAction: AppGamePolicyCompilerRequestedActionSchema,
    policyAction: PolicyActionSchema,
    scheduleRef: Schema.Union(PolicyScheduleIdSchema, Schema.Null),
    evidence: Schema.Array(AppGamePolicyCompilerEvidenceSchema),
    capabilityRefs: Schema.Array(AppGamePolicyCompilerCapabilityRefSchema),
    authorityRefs: Schema.Array(AppGamePolicyCompilerAuthorityRefSchema),
    requestedAt: ParentTimestampSchema,
  })
    .pipe(
      Schema.filter(
        (request) =>
          appGamePolicyRequestHasFreshLocalEvidence(request) ||
          'Expected app/game compiler evidence to be fresh and tied to the requested device and local user'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          appGamePolicyTargetProofIsComplete(request) ||
          'Expected app/game compiler targets to carry required identity, unknown-state, or category proof'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          appGamePolicyRequestHasScheduleProof(request) ||
          'Expected scheduled app/game compiler rules to carry schedule proof'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          appGamePolicyRequestHasCapabilityRef(request) ||
          'Expected app/game compiler requests to include capability refs'
      )
    )
);
export const AppGamePolicyCompiledDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    compiledDecisionId: AppGamePolicyCompiledDecisionIdSchema,
    request: AppGamePolicyCompileRequestSchema,
    policyTarget: PolicyTargetSchema,
    policyDecision: PolicyDecisionSchema,
    outcomeState: AppGamePolicyCompilerOutcomeStateSchema,
    rejectionReason: AppGamePolicyCompilerRejectionReasonSchema,
    capabilityRefs: Schema.Array(AppGamePolicyCapabilityRefSchema),
    authorityRefs: Schema.Array(AppGamePolicyAuthorityRefSchema),
    auditRefs: Schema.Array(AppGamePolicyAuditRefSchema),
    compiledAt: ParentTimestampSchema,
  })
    .pipe(
      Schema.filter(
        (decision) =>
          appGamePolicyCompiledDecisionCarriesProofRefs(decision) ||
          'Expected compiled app/game policy decisions to remain dry-run and carry evidence, rule, and capability refs'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          decision.policyDecision.ruleIds.includes(decision.request.ruleId) ||
          'Expected compiled app/game policy decisions to cite the source rule'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          decision.policyDecision.action === decision.request.policyAction ||
          'Expected compiled app/game policy decision action to match the requested policy action'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          appGamePolicyCapabilityRefsKeepNonReadyStatesExplicit(decision) ||
          'Expected manual-required or unsupported app/game capability refs to avoid dry-run-ready output'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          decision.request.requestedAction === AppGamePolicyCompilerRequestedAction.BlockLaunch ||
          appGamePolicyHardActionProofIsComplete(decision.request) ||
          decision.outcomeState !== AppGamePolicyCompilerOutcomeState.DryRunReady ||
          'Expected hard app/game actions to avoid dry-run-ready output without authority and capability proof'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          appGamePolicyBlockLaunchWithoutProofIsManualRequired(decision) ||
          'Expected unproved app/game block-launch actions to compile as manual-required'
      )
    )
);

export type AppGamePolicyTargetKind = Infer<typeof AppGamePolicyTargetKindSchema>;
export type AppGamePolicyCompilerTarget = Infer<typeof AppGamePolicyCompilerTargetSchema>;
export type AppGamePolicyCompilerEvidence = Infer<typeof AppGamePolicyCompilerEvidenceSchema>;
export type AppGamePolicyCompileRequest = Infer<typeof AppGamePolicyCompileRequestSchema>;
export type AppGamePolicyCompiledDecision = Infer<typeof AppGamePolicyCompiledDecisionSchema>;
