import { PolicyCompilerCapabilityState } from './policy-compiler';

export const AppGamePolicyTargetKind = {
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

export const AppGamePolicyCompilerProofKind = {
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

export const AppGamePolicyCompilerEvidenceState = {
  Active: 'active',
  Stale: 'stale',
  WrongDevice: 'wrong-device',
  WrongLocalUser: 'wrong-local-user',
} as const;

export const AppGamePolicyCompilerCapabilityState = PolicyCompilerCapabilityState;

export const AppGamePolicyCompilerAuthorityState = {
  Proved: 'proved',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
  Unproved: 'unproved',
} as const;

export const AppGamePolicyCompilerRequestedAction = {
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

export const AppGamePolicyCompilerOutcomeState = {
  DryRunReady: 'dry-run-ready',
  ManualRequired: 'manual-required',
  Rejected: 'rejected',
} as const;

export const AppGamePolicyCompilerRejectionReason = {
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

type TargetKindValue = (typeof AppGamePolicyTargetKind)[keyof typeof AppGamePolicyTargetKind];
type ProofKindValue = (typeof AppGamePolicyCompilerProofKind)[keyof typeof AppGamePolicyCompilerProofKind];
type EvidenceStateValue = (typeof AppGamePolicyCompilerEvidenceState)[keyof typeof AppGamePolicyCompilerEvidenceState];
type CapabilityStateValue =
  (typeof PolicyCompilerCapabilityState)[keyof typeof PolicyCompilerCapabilityState];
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

const identityTargets = [
  AppGamePolicyTargetKind.SpecificApp,
  AppGamePolicyTargetKind.PackageId,
  AppGamePolicyTargetKind.BundleId,
  AppGamePolicyTargetKind.AppUserModelId,
  AppGamePolicyTargetKind.DesktopEntryId,
  AppGamePolicyTargetKind.ExecutableHash,
  AppGamePolicyTargetKind.Publisher,
  AppGamePolicyTargetKind.SpecificGame,
  AppGamePolicyTargetKind.LauncherGameId,
  AppGamePolicyTargetKind.StoreGameId,
] as const;

const unknownTargets = [AppGamePolicyTargetKind.UnknownApp, AppGamePolicyTargetKind.UnknownGame] as const;

const categoryTargets = [
  AppGamePolicyTargetKind.AppCategory,
  AppGamePolicyTargetKind.RiskApp,
  AppGamePolicyTargetKind.GameCategory,
  AppGamePolicyTargetKind.MultiplayerGame,
  AppGamePolicyTargetKind.UgcGame,
  AppGamePolicyTargetKind.PurchaseCapableGame,
  AppGamePolicyTargetKind.MatureGame,
] as const;

const hardActions = [
  AppGamePolicyCompilerRequestedAction.TerminateRunning,
  AppGamePolicyCompilerRequestedAction.BlockLaunch,
  AppGamePolicyCompilerRequestedAction.HideApp,
  AppGamePolicyCompilerRequestedAction.SuspendApp,
  AppGamePolicyCompilerRequestedAction.ShieldApp,
] as const;

const targetKindIsIn = (targetKind: TargetKindValue, candidates: ReadonlyArray<TargetKindValue>) =>
  candidates.some((candidate) => candidate === targetKind);

export const appGamePolicyTargetRequiresIdentity = (target: TargetLike) =>
  targetKindIsIn(target.targetKind, identityTargets);

export const appGamePolicyTargetRequiresUnknownState = (target: TargetLike) =>
  targetKindIsIn(target.targetKind, unknownTargets);

export const appGamePolicyTargetRequiresCategory = (target: TargetLike) =>
  targetKindIsIn(target.targetKind, categoryTargets);

export const appGamePolicyRequestHasFreshLocalEvidence = (request: CompileRequestLike) =>
  request.evidence.length > 0 &&
  request.evidence.every(
    (evidence) =>
      evidence.evidenceState === AppGamePolicyCompilerEvidenceState.Active &&
      evidence.device.deviceId === request.device.deviceId &&
      evidence.localUserRef === request.localUserRef
  );

export const appGamePolicyRequestHasProofKind = (request: CompileRequestLike, proofKind: ProofKindValue) =>
  request.evidence.some((evidence) => evidence.proofKind === proofKind);

export const appGamePolicyTargetProofIsComplete = (request: CompileRequestLike) =>
  (!appGamePolicyTargetRequiresIdentity(request.target) ||
    appGamePolicyRequestHasProofKind(request, AppGamePolicyCompilerProofKind.Identity)) &&
  (!appGamePolicyTargetRequiresUnknownState(request.target) ||
    appGamePolicyRequestHasProofKind(request, AppGamePolicyCompilerProofKind.UnknownState)) &&
  (!appGamePolicyTargetRequiresCategory(request.target) ||
    appGamePolicyRequestHasProofKind(request, AppGamePolicyCompilerProofKind.Category));

export const appGamePolicyRequestHasScheduleProof = (request: CompileRequestLike) =>
  request.scheduleRef === null || appGamePolicyRequestHasProofKind(request, AppGamePolicyCompilerProofKind.Schedule);

export const appGamePolicyRequestHasCapabilityRef = (request: CompileRequestLike) => request.capabilityRefs.length > 0;

export const appGamePolicyRequestHasSupportedAuthority = (request: CompileRequestLike) =>
  request.authorityRefs.some((ref) => ref.authorityState === AppGamePolicyCompilerAuthorityState.Proved);

export const appGamePolicyRequestHasSupportedCapability = (request: CompileRequestLike) =>
  request.capabilityRefs.length > 0 &&
  request.capabilityRefs.every((ref) => ref.capabilityState === PolicyCompilerCapabilityState.Supported);

export const appGamePolicyCapabilityRefsKeepNonReadyStatesExplicit = (decision: CompiledDecisionLike) =>
  appGamePolicyRequestHasSupportedCapability(decision.request) ||
  decision.outcomeState !== AppGamePolicyCompilerOutcomeState.DryRunReady;

export const appGamePolicyRequestedActionIsHard = (request: CompileRequestLike) =>
  hardActions.some((action) => action === request.requestedAction);

export const appGamePolicyHardActionProofIsComplete = (request: CompileRequestLike) =>
  !appGamePolicyRequestedActionIsHard(request) ||
  (appGamePolicyRequestHasSupportedAuthority(request) && appGamePolicyRequestHasSupportedCapability(request));

export const appGamePolicyBlockLaunchWithoutProofIsManualRequired = (decision: CompiledDecisionLike) =>
  decision.request.requestedAction !== AppGamePolicyCompilerRequestedAction.BlockLaunch ||
  appGamePolicyHardActionProofIsComplete(decision.request) ||
  (decision.outcomeState === AppGamePolicyCompilerOutcomeState.ManualRequired &&
    decision.rejectionReason === AppGamePolicyCompilerRejectionReason.BlockLaunchManualRequired &&
    decision.policyDecision.enforcementHandoffState === 'disabled');

export const appGamePolicyCompiledDecisionCarriesProofRefs = (decision: CompiledDecisionLike) =>
  decision.policyDecision.dryRun &&
  decision.policyDecision.evidenceReferences.length > 0 &&
  decision.policyDecision.ruleIds.length > 0 &&
  decision.capabilityRefs.length > 0;
