import { AppGamePolicyTargetKind } from './app-game-policy-target-compiler-rules';

export const AppRiskDetectionRiskSignal = {
  VpnProxy: 'vpnProxy',
  RemoteDesktop: 'remoteDesktop',
  DownloadTorrent: 'downloadTorrent',
  InstallerUpdater: 'installerUpdater',
  AiChatbot: 'aiChatbot',
  SocialVideoMessaging: 'socialVideoMessaging',
  UnknownRisk: 'unknownRisk',
} as const;

export const AppRiskDetectionSourceKind = {
  KnownCatalog: 'knownCatalog',
  ExecutableName: 'executableName',
  PublisherMetadata: 'publisherMetadata',
  ExecutableHash: 'executableHash',
  LocalAiDigest: 'localAiDigest',
  ParentOverride: 'parentOverride',
} as const;

export const AppRiskDetectionCandidateState = {
  CatalogMatch: 'catalogMatch',
  HeuristicCandidate: 'heuristicCandidate',
  AiCandidate: 'aiCandidate',
  ParentReviewCandidate: 'parentReviewCandidate',
  ParentDisplayOverride: 'parentDisplayOverride',
} as const;

export const AppRiskDetectionPublisherTrustState = {
  KnownPublisher: 'knownPublisher',
  UnknownPublisher: 'unknownPublisher',
  MissingPublisher: 'missingPublisher',
  UnverifiedPublisher: 'unverifiedPublisher',
  ParentTrusted: 'parentTrusted',
} as const;

export const AppRiskDetectionPolicyCandidateAction = {
  None: 'none',
  Observe: 'observe',
  Warn: 'warn',
  AskParent: 'askParent',
  ManualReview: 'manualReview',
} as const;

export const AppRiskDetectionConfidenceBand = {
  High: 'high',
  Medium: 'medium',
  Low: 'low',
  Review: 'review',
} as const;

export const AppRiskDetectionPolicyTargetKind = {
  RiskApp: AppGamePolicyTargetKind.RiskApp,
} as const;

export const AppRiskDetectionAskParentRouting = {
  Available: 'available',
  ManualReview: 'manual-review',
  NotRouted: 'not-routed',
} as const;

export const AppRiskDetectionSurfaceState = {
  RiskDisclosureReady: 'riskdisclosure-ready',
} as const;

export const AppRiskDetectionNoContentClaimState = {
  NoContentCaptured: 'no-content-captured',
} as const;

type SourceKindValue = (typeof AppRiskDetectionSourceKind)[keyof typeof AppRiskDetectionSourceKind];
type CandidateStateValue = (typeof AppRiskDetectionCandidateState)[keyof typeof AppRiskDetectionCandidateState];
type PublisherTrustValue =
  (typeof AppRiskDetectionPublisherTrustState)[keyof typeof AppRiskDetectionPublisherTrustState];
type PolicyCandidateActionValue =
  (typeof AppRiskDetectionPolicyCandidateAction)[keyof typeof AppRiskDetectionPolicyCandidateAction];

type AppRiskDetectionCandidateRuleInput = {
  readonly sourceKind: SourceKindValue;
  readonly candidateState: CandidateStateValue;
  readonly publisherTrustState: PublisherTrustValue;
  readonly confidence: number;
  readonly evidenceReferences: readonly unknown[];
  readonly sourceRefs: readonly unknown[];
  readonly localAiDigestRef: unknown;
  readonly parentOverride: null | { readonly rawIdentityChanged: boolean };
  readonly policyCandidateAction: PolicyCandidateActionValue;
  readonly notDirectEnforcement: boolean;
  readonly noContentClaim: boolean;
  readonly surfaceDisclosure: {
    readonly sourceEvidenceCount: number;
    readonly confidencePercent: number;
    readonly noContentClaimState: unknown;
  };
};

export function appRiskDetectionCandidateIsHonest(candidate: AppRiskDetectionCandidateRuleInput): boolean {
  return (
    appRiskDetectionCandidateCitesEvidence(candidate) &&
    appRiskDetectionCandidateStateMatchesSource(candidate) &&
    appRiskDetectionUnknownPublisherLowersConfidence(candidate) &&
    appRiskDetectionAiCandidateCitesDigest(candidate) &&
    appRiskDetectionParentOverrideIsDisplayOnly(candidate) &&
    appRiskDetectionCandidateCannotDirectlyEnforce(candidate) &&
    appRiskDetectionSurfaceDisclosureMatchesEvidence(candidate)
  );
}

function appRiskDetectionCandidateCitesEvidence(candidate: AppRiskDetectionCandidateRuleInput): boolean {
  return candidate.evidenceReferences.length > 0 && candidate.sourceRefs.length > 0;
}

function appRiskDetectionCandidateStateMatchesSource(candidate: AppRiskDetectionCandidateRuleInput): boolean {
  switch (candidate.sourceKind) {
    case AppRiskDetectionSourceKind.KnownCatalog:
      return candidate.candidateState === AppRiskDetectionCandidateState.CatalogMatch && candidate.confidence >= 0.7;
    case AppRiskDetectionSourceKind.ExecutableName:
    case AppRiskDetectionSourceKind.PublisherMetadata:
    case AppRiskDetectionSourceKind.ExecutableHash:
      return (
        candidate.candidateState === AppRiskDetectionCandidateState.HeuristicCandidate ||
        candidate.candidateState === AppRiskDetectionCandidateState.ParentReviewCandidate
      );
    case AppRiskDetectionSourceKind.LocalAiDigest:
      return candidate.candidateState === AppRiskDetectionCandidateState.AiCandidate;
    case AppRiskDetectionSourceKind.ParentOverride:
      return candidate.candidateState === AppRiskDetectionCandidateState.ParentDisplayOverride;
  }
}

function appRiskDetectionUnknownPublisherLowersConfidence(candidate: AppRiskDetectionCandidateRuleInput): boolean {
  if (
    candidate.publisherTrustState === AppRiskDetectionPublisherTrustState.KnownPublisher ||
    candidate.publisherTrustState === AppRiskDetectionPublisherTrustState.ParentTrusted
  ) {
    return true;
  }

  return candidate.confidence <= 0.5;
}

function appRiskDetectionAiCandidateCitesDigest(candidate: AppRiskDetectionCandidateRuleInput): boolean {
  if (candidate.sourceKind !== AppRiskDetectionSourceKind.LocalAiDigest) {
    return true;
  }

  return (
    candidate.localAiDigestRef !== null &&
    candidate.policyCandidateAction !== AppRiskDetectionPolicyCandidateAction.None
  );
}

function appRiskDetectionParentOverrideIsDisplayOnly(candidate: AppRiskDetectionCandidateRuleInput): boolean {
  if (candidate.sourceKind !== AppRiskDetectionSourceKind.ParentOverride) {
    return true;
  }

  return candidate.parentOverride !== null && candidate.parentOverride.rawIdentityChanged === false;
}

function appRiskDetectionCandidateCannotDirectlyEnforce(candidate: AppRiskDetectionCandidateRuleInput): boolean {
  return candidate.notDirectEnforcement && candidate.noContentClaim;
}

function appRiskDetectionSurfaceDisclosureMatchesEvidence(candidate: AppRiskDetectionCandidateRuleInput): boolean {
  return (
    candidate.surfaceDisclosure.sourceEvidenceCount === candidate.evidenceReferences.length &&
    candidate.surfaceDisclosure.confidencePercent === Math.round(candidate.confidence * 100) &&
    candidate.surfaceDisclosure.noContentClaimState === AppRiskDetectionNoContentClaimState.NoContentCaptured
  );
}
