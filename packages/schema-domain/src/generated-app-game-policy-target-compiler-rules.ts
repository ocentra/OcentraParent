/* generated from crates/app-game-core/src/app_game_policy_target_compiler.rs */

import { PolicyCompilerCapabilityState } from './policy-compiler';

export const AppGamePolicyTargetKindGenerated = {
  SpecificApp: 'specific-app',
  PackageId: 'package-id',
  BundleId: 'bundle-id',
  AppUserModelId: 'app-user-model-id',
  DesktopEntryId: 'desktop-entry-id',
  ExecutableHash: 'executable-hash',
  Publisher: 'publisher',
  AppCategory: 'app-category',
  UnknownApp: 'unknown-app',
  NewApp: 'new-app',
  PortableApp: 'portable-app',
  RiskApp: 'risk-app',
  AllNonSystemApps: 'all-non-system-apps',
  SpecificGame: 'specific-game',
  LauncherGameId: 'launcher-game-id',
  StoreGameId: 'store-game-id',
  GameCategory: 'game-category',
  UnknownGame: 'unknown-game',
  NewGame: 'new-game',
  LauncherGameCandidate: 'launcher-game-candidate',
  MultiplayerGame: 'multiplayer-game',
  UgcGame: 'ugc-game',
  PurchaseCapableGame: 'purchase-capable-game',
  MatureGame: 'mature-game',
  AllGames: 'all-games',
} as const;

export const AppGamePolicyCompilerProofKindGenerated = {
  Identity: 'identity-proof',
  Category: 'category-proof',
  UnknownState: 'unknown-state-proof',
  Schedule: 'schedule-proof',
  Approval: 'approval-proof',
  Authority: 'authority-proof',
  Capability: 'capability-proof',
  SessionSummary: 'session-summary-proof',
  CurrentProcess: 'current-process-proof',
} as const;

export const AppGamePolicyCompilerEvidenceStateGenerated = {
  Active: 'active',
  Stale: 'stale',
  WrongDevice: 'wrong-device',
  WrongLocalUser: 'wrong-local-user',
} as const;

export const AppGamePolicyCompilerCapabilityStateGenerated = PolicyCompilerCapabilityState;

export const AppGamePolicyCompilerAuthorityStateGenerated = {
  Proved: 'proved',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
  Unproved: 'unproved',
} as const;

export const AppGamePolicyCompilerRequestedActionGenerated = {
  Observe: 'observe',
  Warn: 'warn',
  AskParent: 'ask-parent',
  TimeLimit: 'time-limit',
  TerminateRunning: 'terminate-running',
  BlockLaunch: 'block-launch',
  HideApp: 'hide-app',
  SuspendApp: 'suspend-app',
  ShieldApp: 'shield-app',
  ManualRequired: 'manual-required',
} as const;

export const AppGamePolicyCompilerOutcomeStateGenerated = {
  DryRunReady: 'dry-run-ready',
  ManualRequired: 'manual-required',
  Rejected: 'rejected',
} as const;

export const AppGamePolicyCompilerRejectionReasonGenerated = {
  None: 'none',
  MissingIdentity: 'missing-identity-proof',
  MissingUnknownState: 'missing-unknown-state-proof',
  MissingCategory: 'missing-category-proof',
  MissingSchedule: 'missing-schedule-proof',
  MissingEvidence: 'missing-evidence',
  MissingCapability: 'missing-capability-proof',
  MissingAuthority: 'missing-authority-proof',
  BlockLaunchManualRequired: 'block-launch-manual-required',
} as const;

type TargetKindValueGenerated =
  (typeof AppGamePolicyTargetKindGenerated)[keyof typeof AppGamePolicyTargetKindGenerated];
type ProofKindValueGenerated =
  (typeof AppGamePolicyCompilerProofKindGenerated)[keyof typeof AppGamePolicyCompilerProofKindGenerated];
type EvidenceStateValueGenerated =
  (typeof AppGamePolicyCompilerEvidenceStateGenerated)[keyof typeof AppGamePolicyCompilerEvidenceStateGenerated];
type CapabilityStateValueGenerated =
  (typeof AppGamePolicyCompilerCapabilityStateGenerated)[keyof typeof AppGamePolicyCompilerCapabilityStateGenerated];
type AuthorityStateValueGenerated =
  (typeof AppGamePolicyCompilerAuthorityStateGenerated)[keyof typeof AppGamePolicyCompilerAuthorityStateGenerated];
type RequestedActionValueGenerated =
  (typeof AppGamePolicyCompilerRequestedActionGenerated)[keyof typeof AppGamePolicyCompilerRequestedActionGenerated];
type OutcomeStateValueGenerated =
  (typeof AppGamePolicyCompilerOutcomeStateGenerated)[keyof typeof AppGamePolicyCompilerOutcomeStateGenerated];
type RejectionReasonValueGenerated =
  (typeof AppGamePolicyCompilerRejectionReasonGenerated)[keyof typeof AppGamePolicyCompilerRejectionReasonGenerated];

type TargetLikeGenerated = {
  readonly targetKind: TargetKindValueGenerated;
};

type EvidenceLikeGenerated = {
  readonly proofKind: ProofKindValueGenerated;
  readonly evidenceState: EvidenceStateValueGenerated;
  readonly device: { readonly deviceId: unknown };
  readonly localUserRef: unknown;
};

type CapabilityLikeGenerated = {
  readonly capabilityState: CapabilityStateValueGenerated;
};

type AuthorityLikeGenerated = {
  readonly authorityState: AuthorityStateValueGenerated;
};

type CompileRequestLikeGenerated = {
  readonly device: { readonly deviceId: unknown };
  readonly localUserRef: unknown;
  readonly target: TargetLikeGenerated;
  readonly requestedAction: RequestedActionValueGenerated;
  readonly scheduleRef: unknown;
  readonly evidence: ReadonlyArray<EvidenceLikeGenerated>;
  readonly capabilityRefs: ReadonlyArray<CapabilityLikeGenerated>;
  readonly authorityRefs: ReadonlyArray<AuthorityLikeGenerated>;
};

type CompiledDecisionLikeGenerated = {
  readonly request: CompileRequestLikeGenerated;
  readonly outcomeState: OutcomeStateValueGenerated;
  readonly rejectionReason: RejectionReasonValueGenerated;
  readonly policyDecision: {
    readonly dryRun: boolean;
    readonly enforcementHandoffState: unknown;
    readonly evidenceReferences: ReadonlyArray<unknown>;
    readonly ruleIds: ReadonlyArray<unknown>;
  };
  readonly capabilityRefs: ReadonlyArray<unknown>;
};

const identityTargetsGenerated = [
  AppGamePolicyTargetKindGenerated.SpecificApp,
  AppGamePolicyTargetKindGenerated.PackageId,
  AppGamePolicyTargetKindGenerated.BundleId,
  AppGamePolicyTargetKindGenerated.AppUserModelId,
  AppGamePolicyTargetKindGenerated.DesktopEntryId,
  AppGamePolicyTargetKindGenerated.ExecutableHash,
  AppGamePolicyTargetKindGenerated.Publisher,
  AppGamePolicyTargetKindGenerated.SpecificGame,
  AppGamePolicyTargetKindGenerated.LauncherGameId,
  AppGamePolicyTargetKindGenerated.StoreGameId,
] as const;

const unknownTargetsGenerated = [
  AppGamePolicyTargetKindGenerated.UnknownApp,
  AppGamePolicyTargetKindGenerated.UnknownGame,
] as const;

const categoryTargetsGenerated = [
  AppGamePolicyTargetKindGenerated.AppCategory,
  AppGamePolicyTargetKindGenerated.RiskApp,
  AppGamePolicyTargetKindGenerated.GameCategory,
  AppGamePolicyTargetKindGenerated.MultiplayerGame,
  AppGamePolicyTargetKindGenerated.UgcGame,
  AppGamePolicyTargetKindGenerated.PurchaseCapableGame,
  AppGamePolicyTargetKindGenerated.MatureGame,
] as const;

const hardActionsGenerated = [
  AppGamePolicyCompilerRequestedActionGenerated.TerminateRunning,
  AppGamePolicyCompilerRequestedActionGenerated.BlockLaunch,
  AppGamePolicyCompilerRequestedActionGenerated.HideApp,
  AppGamePolicyCompilerRequestedActionGenerated.SuspendApp,
  AppGamePolicyCompilerRequestedActionGenerated.ShieldApp,
] as const;

const targetKindIsInGenerated = (
  targetKind: TargetKindValueGenerated,
  candidates: ReadonlyArray<TargetKindValueGenerated>
) => candidates.some((candidate) => candidate === targetKind);

export const appGamePolicyTargetRequiresIdentityGenerated = (target: TargetLikeGenerated) =>
  targetKindIsInGenerated(target.targetKind, identityTargetsGenerated);

export const appGamePolicyTargetRequiresUnknownStateGenerated = (target: TargetLikeGenerated) =>
  targetKindIsInGenerated(target.targetKind, unknownTargetsGenerated);

export const appGamePolicyTargetRequiresCategoryGenerated = (target: TargetLikeGenerated) =>
  targetKindIsInGenerated(target.targetKind, categoryTargetsGenerated);

export const appGamePolicyRequestHasFreshLocalEvidenceGenerated = (request: CompileRequestLikeGenerated) =>
  request.evidence.length > 0 &&
  request.evidence.every(
    (evidence) =>
      evidence.evidenceState === AppGamePolicyCompilerEvidenceStateGenerated.Active &&
      evidence.device.deviceId === request.device.deviceId &&
      evidence.localUserRef === request.localUserRef
  );

export const appGamePolicyRequestHasProofKindGenerated = (
  request: CompileRequestLikeGenerated,
  proofKind: ProofKindValueGenerated
) => request.evidence.some((evidence) => evidence.proofKind === proofKind);

export const appGamePolicyTargetProofIsCompleteGenerated = (request: CompileRequestLikeGenerated) =>
  (!appGamePolicyTargetRequiresIdentityGenerated(request.target) ||
    appGamePolicyRequestHasProofKindGenerated(request, AppGamePolicyCompilerProofKindGenerated.Identity)) &&
  (!appGamePolicyTargetRequiresUnknownStateGenerated(request.target) ||
    appGamePolicyRequestHasProofKindGenerated(request, AppGamePolicyCompilerProofKindGenerated.UnknownState)) &&
  (!appGamePolicyTargetRequiresCategoryGenerated(request.target) ||
    appGamePolicyRequestHasProofKindGenerated(request, AppGamePolicyCompilerProofKindGenerated.Category));

export const appGamePolicyRequestHasScheduleProofGenerated = (request: CompileRequestLikeGenerated) =>
  request.scheduleRef === null ||
  appGamePolicyRequestHasProofKindGenerated(request, AppGamePolicyCompilerProofKindGenerated.Schedule);

export const appGamePolicyRequestHasCapabilityRefGenerated = (request: CompileRequestLikeGenerated) =>
  request.capabilityRefs.length > 0;

export const appGamePolicyRequestHasSupportedAuthorityGenerated = (request: CompileRequestLikeGenerated) =>
  request.authorityRefs.some((ref) => ref.authorityState === AppGamePolicyCompilerAuthorityStateGenerated.Proved);

export const appGamePolicyRequestHasSupportedCapabilityGenerated = (request: CompileRequestLikeGenerated) =>
  request.capabilityRefs.length > 0 &&
  request.capabilityRefs.every(
    (ref) => ref.capabilityState === AppGamePolicyCompilerCapabilityStateGenerated.supported
  );

export const appGamePolicyCapabilityRefsKeepNonReadyStatesExplicitGenerated = (
  decision: CompiledDecisionLikeGenerated
) =>
  appGamePolicyRequestHasSupportedCapabilityGenerated(decision.request) ||
  decision.outcomeState !== AppGamePolicyCompilerOutcomeStateGenerated.DryRunReady;

export const appGamePolicyRequestedActionIsHardGenerated = (request: CompileRequestLikeGenerated) =>
  hardActionsGenerated.some((action) => action === request.requestedAction);

export const appGamePolicyHardActionProofIsCompleteGenerated = (request: CompileRequestLikeGenerated) =>
  !appGamePolicyRequestedActionIsHardGenerated(request) ||
  (appGamePolicyRequestHasSupportedAuthorityGenerated(request) &&
    appGamePolicyRequestHasSupportedCapabilityGenerated(request));

export const appGamePolicyBlockLaunchWithoutProofIsManualRequiredGenerated = (
  decision: CompiledDecisionLikeGenerated
) =>
  decision.request.requestedAction !== AppGamePolicyCompilerRequestedActionGenerated.BlockLaunch ||
  appGamePolicyHardActionProofIsCompleteGenerated(decision.request) ||
  (decision.outcomeState === AppGamePolicyCompilerOutcomeStateGenerated.ManualRequired &&
    decision.rejectionReason === AppGamePolicyCompilerRejectionReasonGenerated.BlockLaunchManualRequired &&
    decision.policyDecision.enforcementHandoffState === 'disabled');

export const appGamePolicyCompiledDecisionCarriesProofRefsGenerated = (decision: CompiledDecisionLikeGenerated) =>
  decision.policyDecision.dryRun &&
  decision.policyDecision.evidenceReferences.length > 0 &&
  decision.policyDecision.ruleIds.length > 0 &&
  decision.capabilityRefs.length > 0;
