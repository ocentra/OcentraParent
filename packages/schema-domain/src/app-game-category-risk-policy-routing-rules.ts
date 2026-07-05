import type { PolicyAction } from './policy';
import {
  generatedAppGameCategoryRiskPolicyRouteActionMatchesCandidate,
  generatedAppGameCategoryRiskPolicyRouteKeepsSoftBoundary,
  generatedAppGameCategoryRiskPolicyRouteLocalAiRequiresDigest,
  generatedAppGameCategoryRiskPolicyRouteManualReviewRequiresManualState,
  generatedAppGameCategoryRiskPolicyRouteTargetMatchesFamily,
  generatedAppGameCategoryRiskPolicyRouteUsesCategoryProof,
} from './generated-policy-control-helpers';
import type {
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

export const appGameCategoryRiskPolicyRouteTargetMatchesFamily = (route: CategoryRiskPolicyRouteLike) =>
  generatedAppGameCategoryRiskPolicyRouteTargetMatchesFamily(route);

export const appGameCategoryRiskPolicyRouteUsesCategoryProof = (route: CategoryRiskPolicyRouteLike) =>
  generatedAppGameCategoryRiskPolicyRouteUsesCategoryProof(route);

export const appGameCategoryRiskPolicyRouteActionMatchesCandidate = (route: CategoryRiskPolicyRouteLike) =>
  generatedAppGameCategoryRiskPolicyRouteActionMatchesCandidate(route);

export const appGameCategoryRiskPolicyRouteKeepsSoftBoundary = (route: CategoryRiskPolicyRouteLike) =>
  generatedAppGameCategoryRiskPolicyRouteKeepsSoftBoundary(route);

export const appGameCategoryRiskPolicyRouteManualReviewRequiresManualState = (route: CategoryRiskPolicyRouteLike) =>
  generatedAppGameCategoryRiskPolicyRouteManualReviewRequiresManualState(route);

export const appGameCategoryRiskPolicyRouteLocalAiRequiresDigest = (route: CategoryRiskPolicyRouteLike) =>
  generatedAppGameCategoryRiskPolicyRouteLocalAiRequiresDigest(route);
