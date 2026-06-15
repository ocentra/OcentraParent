import { PolicyAction } from '@ocentra-parent/policy-domain/policy';
import {
  AppGamePolicyCompilerEvidenceState,
  AppGamePolicyCompilerProofKind,
  AppGamePolicyCompilerRequestedAction,
  AppGamePolicyTargetKind,
} from './app-game-policy-target-compiler-rules';

export const AppGameCategoryRiskPolicyRouteFamily = {
  NativeApp: 'nativeApp',
  NativeGame: 'nativeGame',
  RiskCandidate: 'riskCandidate',
  GameContext: 'gameContext',
} as const;

export const AppGameCategoryRiskPolicyRouteSourceKind = {
  Catalog: 'catalog',
  StoreMetadata: 'storeMetadata',
  LauncherManifest: 'launcherManifest',
  ParentLabel: 'parentLabel',
  LocalAi: 'localAi',
  ProcessMetadata: 'processMetadata',
  ExecutableName: 'executableName',
  ManagedDevice: 'managedDevice',
  ManualReview: 'manualReview',
} as const;

export const AppGameCategoryRiskPolicyCandidateAction = {
  Observe: 'observe',
  Warn: 'warn',
  AskParent: 'askParent',
  ManualReview: 'manualReview',
} as const;

export const AppGameCategoryRiskPolicyRoutingState = {
  CompileReady: 'compile-ready',
  ManualRequired: 'manual-required',
} as const;

export const AppGameCategoryRiskPolicyAdapterDispatchState = {
  NotDispatched: 'not-dispatched',
} as const;

type RouteFamilyValue =
  (typeof AppGameCategoryRiskPolicyRouteFamily)[keyof typeof AppGameCategoryRiskPolicyRouteFamily];
type SourceKindValue =
  (typeof AppGameCategoryRiskPolicyRouteSourceKind)[keyof typeof AppGameCategoryRiskPolicyRouteSourceKind];
type CandidateActionValue =
  (typeof AppGameCategoryRiskPolicyCandidateAction)[keyof typeof AppGameCategoryRiskPolicyCandidateAction];
type RoutingStateValue =
  (typeof AppGameCategoryRiskPolicyRoutingState)[keyof typeof AppGameCategoryRiskPolicyRoutingState];
type TargetKindValue = (typeof AppGamePolicyTargetKind)[keyof typeof AppGamePolicyTargetKind];
type RequestedActionValue =
  (typeof AppGamePolicyCompilerRequestedAction)[keyof typeof AppGamePolicyCompilerRequestedAction];

type CategoryProofLike = {
  readonly proofKind: (typeof AppGamePolicyCompilerProofKind)[keyof typeof AppGamePolicyCompilerProofKind];
  readonly evidenceState: (typeof AppGamePolicyCompilerEvidenceState)[keyof typeof AppGamePolicyCompilerEvidenceState];
};

type CategoryRiskPolicyRouteLike = {
  readonly routeFamily: RouteFamilyValue;
  readonly sourceKind: SourceKindValue;
  readonly targetKind: TargetKindValue;
  readonly candidateAction: CandidateActionValue;
  readonly requestedAction: RequestedActionValue;
  readonly policyAction: PolicyAction;
  readonly routingState: RoutingStateValue;
  readonly categoryProof: CategoryProofLike;
  readonly supportingEvidence: ReadonlyArray<unknown>;
  readonly aiDigestRef: unknown;
};

const GameContextTargetKinds = [
  AppGamePolicyTargetKind.MultiplayerGame,
  AppGamePolicyTargetKind.UgcGame,
  AppGamePolicyTargetKind.PurchaseCapableGame,
  AppGamePolicyTargetKind.MatureGame,
] as const;

const RequestedActionByCandidateAction = {
  [AppGameCategoryRiskPolicyCandidateAction.Observe]: AppGamePolicyCompilerRequestedAction.Observe,
  [AppGameCategoryRiskPolicyCandidateAction.Warn]: AppGamePolicyCompilerRequestedAction.Warn,
  [AppGameCategoryRiskPolicyCandidateAction.AskParent]: AppGamePolicyCompilerRequestedAction.AskParent,
  [AppGameCategoryRiskPolicyCandidateAction.ManualReview]: AppGamePolicyCompilerRequestedAction.ManualRequired,
} as const;

const PolicyActionByCandidateAction = {
  [AppGameCategoryRiskPolicyCandidateAction.Observe]: PolicyAction.Unknown,
  [AppGameCategoryRiskPolicyCandidateAction.Warn]: PolicyAction.Warn,
  [AppGameCategoryRiskPolicyCandidateAction.AskParent]: PolicyAction.AskParent,
  [AppGameCategoryRiskPolicyCandidateAction.ManualReview]: PolicyAction.AskParent,
} as const;

export const appGameCategoryRiskPolicyRouteTargetMatchesFamily = (route: CategoryRiskPolicyRouteLike) => {
  switch (route.routeFamily) {
    case AppGameCategoryRiskPolicyRouteFamily.NativeApp:
      return route.targetKind === AppGamePolicyTargetKind.AppCategory;
    case AppGameCategoryRiskPolicyRouteFamily.RiskCandidate:
      return route.targetKind === AppGamePolicyTargetKind.RiskApp;
    case AppGameCategoryRiskPolicyRouteFamily.NativeGame:
      return route.targetKind === AppGamePolicyTargetKind.GameCategory;
    case AppGameCategoryRiskPolicyRouteFamily.GameContext:
      return GameContextTargetKinds.some((targetKind) => targetKind === route.targetKind);
  }
};

export const appGameCategoryRiskPolicyRouteUsesCategoryProof = (route: CategoryRiskPolicyRouteLike) =>
  route.categoryProof.proofKind === AppGamePolicyCompilerProofKind.Category &&
  route.categoryProof.evidenceState === AppGamePolicyCompilerEvidenceState.Active &&
  route.supportingEvidence.length > 0;

export const appGameCategoryRiskPolicyRouteActionMatchesCandidate = (route: CategoryRiskPolicyRouteLike) =>
  route.requestedAction === RequestedActionByCandidateAction[route.candidateAction] &&
  route.policyAction === PolicyActionByCandidateAction[route.candidateAction];

export const appGameCategoryRiskPolicyRouteKeepsSoftBoundary = (route: CategoryRiskPolicyRouteLike) =>
  route.requestedAction === AppGamePolicyCompilerRequestedAction.Observe ||
  route.requestedAction === AppGamePolicyCompilerRequestedAction.Warn ||
  route.requestedAction === AppGamePolicyCompilerRequestedAction.AskParent ||
  route.requestedAction === AppGamePolicyCompilerRequestedAction.ManualRequired;

export const appGameCategoryRiskPolicyRouteManualReviewRequiresManualState = (route: CategoryRiskPolicyRouteLike) =>
  route.candidateAction !== AppGameCategoryRiskPolicyCandidateAction.ManualReview ||
  route.routingState === AppGameCategoryRiskPolicyRoutingState.ManualRequired;

export const appGameCategoryRiskPolicyRouteLocalAiRequiresDigest = (route: CategoryRiskPolicyRouteLike) =>
  route.sourceKind !== AppGameCategoryRiskPolicyRouteSourceKind.LocalAi || route.aiDigestRef !== null;
