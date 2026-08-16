import {
  AppGamePolicyCompilerAuthorityStateGenerated,
  AppGamePolicyCompilerCapabilityStateGenerated,
  AppGamePolicyCompilerEvidenceStateGenerated,
  AppGamePolicyCompilerOutcomeStateGenerated,
  AppGamePolicyCompilerProofKindGenerated,
  AppGamePolicyCompilerRejectionReasonGenerated,
  AppGamePolicyCompilerRequestedActionGenerated,
  AppGamePolicyTargetKindGenerated,
  appGamePolicyBlockLaunchWithoutProofIsManualRequiredGenerated,
  appGamePolicyCapabilityRefsKeepNonReadyStatesExplicitGenerated,
  appGamePolicyCompiledDecisionCarriesProofRefsGenerated,
  appGamePolicyHardActionProofIsCompleteGenerated,
  appGamePolicyRequestHasCapabilityRefGenerated,
  appGamePolicyRequestHasFreshLocalEvidenceGenerated,
  appGamePolicyRequestHasProofKindGenerated,
  appGamePolicyRequestHasScheduleProofGenerated,
  appGamePolicyRequestHasSupportedAuthorityGenerated,
  appGamePolicyRequestHasSupportedCapabilityGenerated,
  appGamePolicyRequestedActionIsHardGenerated,
  appGamePolicyTargetProofIsCompleteGenerated,
  appGamePolicyTargetRequiresCategoryGenerated,
  appGamePolicyTargetRequiresIdentityGenerated,
  appGamePolicyTargetRequiresUnknownStateGenerated,
} from './generated-app-game-policy-target-compiler-rules';
import type { PolicyCompilerCapabilityState } from './policy-compiler';

export const AppGamePolicyTargetKind = AppGamePolicyTargetKindGenerated;
export const AppGamePolicyCompilerProofKind = AppGamePolicyCompilerProofKindGenerated;
export const AppGamePolicyCompilerEvidenceState = AppGamePolicyCompilerEvidenceStateGenerated;
export const AppGamePolicyCompilerCapabilityState = AppGamePolicyCompilerCapabilityStateGenerated;
export const AppGamePolicyCompilerAuthorityState = AppGamePolicyCompilerAuthorityStateGenerated;
export const AppGamePolicyCompilerRequestedAction = AppGamePolicyCompilerRequestedActionGenerated;
export const AppGamePolicyCompilerOutcomeState = AppGamePolicyCompilerOutcomeStateGenerated;
export const AppGamePolicyCompilerRejectionReason = AppGamePolicyCompilerRejectionReasonGenerated;

type TargetKindValue = (typeof AppGamePolicyTargetKind)[keyof typeof AppGamePolicyTargetKind];
type ProofKindValue = (typeof AppGamePolicyCompilerProofKind)[keyof typeof AppGamePolicyCompilerProofKind];
type EvidenceStateValue = (typeof AppGamePolicyCompilerEvidenceState)[keyof typeof AppGamePolicyCompilerEvidenceState];
type CapabilityStateValue = (typeof PolicyCompilerCapabilityState)[keyof typeof PolicyCompilerCapabilityState];
type AuthorityStateValue =
  (typeof AppGamePolicyCompilerAuthorityState)[keyof typeof AppGamePolicyCompilerAuthorityState];
type RequestedActionValue =
  (typeof AppGamePolicyCompilerRequestedAction)[keyof typeof AppGamePolicyCompilerRequestedAction];
type OutcomeStateValue = (typeof AppGamePolicyCompilerOutcomeState)[keyof typeof AppGamePolicyCompilerOutcomeState];
type RejectionReasonValue =
  (typeof AppGamePolicyCompilerRejectionReason)[keyof typeof AppGamePolicyCompilerRejectionReason];

type TargetLike = {
  readonly targetKind: TargetKindValue;
};

type EvidenceLike = {
  readonly proofKind: ProofKindValue;
  readonly evidenceState: EvidenceStateValue;
  readonly device: { readonly deviceId: unknown };
  readonly localUserRef: unknown;
};

type CapabilityLike = {
  readonly capabilityState: CapabilityStateValue;
};

type AuthorityLike = {
  readonly authorityState: AuthorityStateValue;
};

type CompileRequestLike = {
  readonly device: { readonly deviceId: unknown };
  readonly localUserRef: unknown;
  readonly target: TargetLike;
  readonly requestedAction: RequestedActionValue;
  readonly scheduleRef: unknown;
  readonly evidence: ReadonlyArray<EvidenceLike>;
  readonly capabilityRefs: ReadonlyArray<CapabilityLike>;
  readonly authorityRefs: ReadonlyArray<AuthorityLike>;
};

type CompiledDecisionLike = {
  readonly request: CompileRequestLike;
  readonly outcomeState: OutcomeStateValue;
  readonly rejectionReason: RejectionReasonValue;
  readonly policyDecision: {
    readonly dryRun: boolean;
    readonly enforcementHandoffState: unknown;
    readonly evidenceReferences: ReadonlyArray<unknown>;
    readonly ruleIds: ReadonlyArray<unknown>;
  };
  readonly capabilityRefs: ReadonlyArray<unknown>;
};

export const appGamePolicyTargetRequiresIdentity = (target: TargetLike) =>
  appGamePolicyTargetRequiresIdentityGenerated(target);

export const appGamePolicyTargetRequiresUnknownState = (target: TargetLike) =>
  appGamePolicyTargetRequiresUnknownStateGenerated(target);

export const appGamePolicyTargetRequiresCategory = (target: TargetLike) =>
  appGamePolicyTargetRequiresCategoryGenerated(target);

export const appGamePolicyRequestHasFreshLocalEvidence = (request: CompileRequestLike) =>
  appGamePolicyRequestHasFreshLocalEvidenceGenerated(request);

export const appGamePolicyRequestHasProofKind = (request: CompileRequestLike, proofKind: ProofKindValue) =>
  appGamePolicyRequestHasProofKindGenerated(request, proofKind);

export const appGamePolicyTargetProofIsComplete = (request: CompileRequestLike) =>
  appGamePolicyTargetProofIsCompleteGenerated(request);

export const appGamePolicyRequestHasScheduleProof = (request: CompileRequestLike) =>
  appGamePolicyRequestHasScheduleProofGenerated(request);

export const appGamePolicyRequestHasCapabilityRef = (request: CompileRequestLike) =>
  appGamePolicyRequestHasCapabilityRefGenerated(request);

export const appGamePolicyRequestHasSupportedAuthority = (request: CompileRequestLike) =>
  appGamePolicyRequestHasSupportedAuthorityGenerated(request);

export const appGamePolicyRequestHasSupportedCapability = (request: CompileRequestLike) =>
  appGamePolicyRequestHasSupportedCapabilityGenerated(request);

export const appGamePolicyCapabilityRefsKeepNonReadyStatesExplicit = (decision: CompiledDecisionLike) =>
  appGamePolicyCapabilityRefsKeepNonReadyStatesExplicitGenerated(decision);

export const appGamePolicyRequestedActionIsHard = (request: CompileRequestLike) =>
  appGamePolicyRequestedActionIsHardGenerated(request);

export const appGamePolicyHardActionProofIsComplete = (request: CompileRequestLike) =>
  appGamePolicyHardActionProofIsCompleteGenerated(request);

export const appGamePolicyBlockLaunchWithoutProofIsManualRequired = (decision: CompiledDecisionLike) =>
  appGamePolicyBlockLaunchWithoutProofIsManualRequiredGenerated(decision);

export const appGamePolicyCompiledDecisionCarriesProofRefs = (decision: CompiledDecisionLike) =>
  appGamePolicyCompiledDecisionCarriesProofRefsGenerated(decision);
