import { describe, expect, it } from 'vitest';
import {
  AppGameCategoryRiskPolicyAdapterDispatchState,
  AppGameCategoryRiskPolicyCandidateAction,
  AppGameCategoryRiskPolicyRouteFamily,
  AppGameCategoryRiskPolicyRouteSchema,
  AppGameCategoryRiskPolicyRouteSourceKind,
  AppGameCategoryRiskPolicyRoutingState,
} from '../../src/app-game-category-risk-policy-routing';
import {
  AppGamePolicyCompilerEvidenceState,
  AppGamePolicyCompilerProofKind,
  AppGamePolicyCompilerRequestedAction,
  AppGamePolicyTargetKind,
} from '../../src/app-game-policy-target-compiler-rules';
import { PolicyAction } from '@ocentra-parent/policy-domain/policy';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-04T13:45:00Z';
const LocalUserRef = 'windows-local-user-category-risk';

const ChildDevice = {
  deviceId: 'device-windows-category-risk',
  childProfileId: 'child-category-risk',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-category-risk-route-1',
  kind: ParentEvidenceReferenceKind.ActivityEvent,
  observedAt: Timestamp,
} as const;

const CategoryProof = {
  evidenceReference: EvidenceReference,
  proofKind: AppGamePolicyCompilerProofKind.Category,
  evidenceState: AppGamePolicyCompilerEvidenceState.Active,
  device: ChildDevice,
  localUserRef: LocalUserRef,
  observedAt: Timestamp,
} as const;

const BaseCategoryRoute = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  routeId: 'category-risk-route-school-warn',
  categoryCandidateRef: 'category-candidate-school',
  routeFamily: AppGameCategoryRiskPolicyRouteFamily.NativeApp,
  sourceKind: AppGameCategoryRiskPolicyRouteSourceKind.Catalog,
  sourceRef: 'category-source-catalog',
  targetKind: AppGamePolicyTargetKind.AppCategory,
  targetRef: 'native-app-category:school',
  confidence: 0.94,
  candidateAction: AppGameCategoryRiskPolicyCandidateAction.Warn,
  requestedAction: AppGamePolicyCompilerRequestedAction.Warn,
  policyAction: PolicyAction.Warn,
  routingState: AppGameCategoryRiskPolicyRoutingState.CompileReady,
  categoryProof: CategoryProof,
  supportingEvidence: [EvidenceReference],
  aiDigestRef: null,
  adapterDispatchState: AppGameCategoryRiskPolicyAdapterDispatchState.NotDispatched,
} as const;

const assertCatalogCategoryRoutesToSoftCompilerTarget = () => {
  const parsed = AppGameCategoryRiskPolicyRouteSchema.safeParse(BaseCategoryRoute);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.targetKind).toBe(AppGamePolicyTargetKind.AppCategory);
    expect(parsed.data.categoryProof.proofKind).toBe(AppGamePolicyCompilerProofKind.Category);
    expect(parsed.data.adapterDispatchState).toBe(AppGameCategoryRiskPolicyAdapterDispatchState.NotDispatched);
  }
};

const assertRiskCandidateCannotRouteHardAction = () => {
  const riskRoute = {
    ...BaseCategoryRoute,
    routeId: 'category-risk-route-vpn',
    categoryCandidateRef: 'category-candidate-vpn',
    routeFamily: AppGameCategoryRiskPolicyRouteFamily.RiskCandidate,
    sourceKind: AppGameCategoryRiskPolicyRouteSourceKind.ExecutableName,
    sourceRef: 'category-source-executable-name',
    targetKind: AppGamePolicyTargetKind.RiskApp,
    targetRef: 'risk-app:vpn-proxy',
    confidence: 0.51,
    candidateAction: AppGameCategoryRiskPolicyCandidateAction.AskParent,
    requestedAction: AppGamePolicyCompilerRequestedAction.AskParent,
    policyAction: PolicyAction.AskParent,
  } as const;
  const hardRoute = {
    ...riskRoute,
    requestedAction: AppGamePolicyCompilerRequestedAction.BlockLaunch,
    policyAction: PolicyAction.Block,
  } as const;

  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(riskRoute).success).toBe(true);
  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(hardRoute).success).toBe(false);
};

const assertLocalAiRoutesRequireDigest = () => {
  const localAiRoute = {
    ...BaseCategoryRoute,
    routeId: 'category-risk-route-local-ai-social',
    categoryCandidateRef: 'category-candidate-local-ai-social',
    sourceKind: AppGameCategoryRiskPolicyRouteSourceKind.LocalAi,
    sourceRef: 'category-source-local-ai',
    aiDigestRef: 'ai-digest-category-risk-social',
  } as const;
  const missingDigest = {
    ...localAiRoute,
    aiDigestRef: null,
  } as const;

  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(localAiRoute).success).toBe(true);
  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(missingDigest).success).toBe(false);
};

const assertGameContextRoutesOnlyToContextTargets = () => {
  const multiplayerRoute = {
    ...BaseCategoryRoute,
    routeId: 'category-risk-route-multiplayer-game',
    categoryCandidateRef: 'category-candidate-multiplayer-game',
    routeFamily: AppGameCategoryRiskPolicyRouteFamily.GameContext,
    sourceKind: AppGameCategoryRiskPolicyRouteSourceKind.LauncherManifest,
    sourceRef: 'category-source-launcher-manifest',
    targetKind: AppGamePolicyTargetKind.MultiplayerGame,
    targetRef: 'game-context:multiplayer',
    candidateAction: AppGameCategoryRiskPolicyCandidateAction.AskParent,
    requestedAction: AppGamePolicyCompilerRequestedAction.AskParent,
    policyAction: PolicyAction.AskParent,
  } as const;
  const wrongTarget = {
    ...multiplayerRoute,
    targetKind: AppGamePolicyTargetKind.AppCategory,
  } as const;

  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(multiplayerRoute).success).toBe(true);
  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(wrongTarget).success).toBe(false);
};

const assertManualReviewAndStaleProofStayOutOfCompileReady = () => {
  const manualReviewRoute = {
    ...BaseCategoryRoute,
    routeId: 'category-risk-route-manual-review',
    categoryCandidateRef: 'category-candidate-manual-review',
    sourceKind: AppGameCategoryRiskPolicyRouteSourceKind.ManualReview,
    sourceRef: 'category-source-manual-review',
    candidateAction: AppGameCategoryRiskPolicyCandidateAction.ManualReview,
    requestedAction: AppGamePolicyCompilerRequestedAction.ManualRequired,
    policyAction: PolicyAction.AskParent,
    routingState: AppGameCategoryRiskPolicyRoutingState.ManualRequired,
  } as const;
  const falselyCompileReady = {
    ...manualReviewRoute,
    routingState: AppGameCategoryRiskPolicyRoutingState.CompileReady,
  } as const;
  const staleProof = {
    ...BaseCategoryRoute,
    routeId: 'category-risk-route-stale-proof',
    categoryProof: {
      ...CategoryProof,
      evidenceState: AppGamePolicyCompilerEvidenceState.Stale,
    },
  } as const;

  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(manualReviewRoute).success).toBe(true);
  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(falselyCompileReady).success).toBe(false);
  expect(AppGameCategoryRiskPolicyRouteSchema.safeParse(staleProof).success).toBe(false);
};

describe('app/game category risk policy routing contracts', () => {
  it('routes catalog categories into soft compiler targets with category proof', () => {
    assertCatalogCategoryRoutesToSoftCompilerTarget();
  });

  it('keeps risk candidates from becoming hard adapter actions', () => {
    assertRiskCandidateCannotRouteHardAction();
  });

  it('requires local-AI category routes to cite digest refs', () => {
    assertLocalAiRoutesRequireDigest();
  });

  it('routes game context signals only to game-context policy targets', () => {
    assertGameContextRoutesOnlyToContextTargets();
  });

  it('keeps manual review and stale category proof out of compile-ready routing', () => {
    assertManualReviewAndStaleProofStayOutOfCompileReady();
  });
});
