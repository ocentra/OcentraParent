import {
  type AppRiskDetectionCandidate,
  AppRiskDetectionCandidateSchema,
  type AppRiskDetectionConfidenceBand,
  AppRiskDetectionMatrixSchema,
  type AppRiskDetectionPolicyCandidateAction,
  type AppRiskDetectionPublisherTrustState,
  type AppRiskDetectionRiskSignal,
  type AppRiskDetectionSourceKind,
  AppRiskDetectionAskParentRouting,
  AppRiskDetectionCandidateState,
  AppRiskDetectionNoContentClaimState,
  AppRiskDetectionPolicyTargetKind,
  AppRiskDetectionSourceKind as SourceKind,
  AppRiskDetectionSurfaceState,
} from '@ocentra-parent/schema-domain/app-game';
import { ParentContractSchemaVersion, ParentEvidenceReferenceKind, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

type RiskSeed = {
  readonly candidateId: string;
  readonly riskSignal: AppRiskDetectionRiskSignal;
  readonly sourceKind: AppRiskDetectionSourceKind;
  readonly publisherTrustState: AppRiskDetectionPublisherTrustState;
  readonly confidence: number;
  readonly confidenceBand: AppRiskDetectionConfidenceBand;
  readonly policyCandidateAction: AppRiskDetectionPolicyCandidateAction;
  readonly inventoryEntryRef?: string | null;
  readonly identityRef?: string | null;
  readonly sourceRefs?: readonly string[];
  readonly localAiDigestRef?: string | null;
  readonly parentDisplayLabel?: string;
};

const GeneratedAt = '2026-06-03T10:55:00.000Z';

export const AppRiskDetectionMatrix = AppRiskDetectionMatrixSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  matrixId: 'app-riskdetection-proof-matrix',
  generatedAt: GeneratedAt,
  candidates: [
    knownCatalog('known-vpn-proxy-risk', 'vpnProxy', 0.94, 'warn'),
    knownCatalog('known-remote-desktop-risk', 'remoteDesktop', 0.92, 'askParent'),
    knownCatalog('known-download-torrent-risk', 'downloadTorrent', 0.9, 'warn'),
    knownCatalog('known-ai-chatbot-risk', 'aiChatbot', 0.86, 'manualReview'),
    heuristicCandidate('unknown-vpn-name-candidate', 'vpnProxy', SourceKind.ExecutableName, 0.42),
    heuristicCandidate('unknown-publisher-hash-candidate', 'unknownRisk', SourceKind.ExecutableHash, 0.38),
    aiDigestCandidate(),
    parentOverrideCandidate(),
  ],
});

function knownCatalog(
  candidateId: string,
  riskSignal: AppRiskDetectionRiskSignal,
  confidence: number,
  policyCandidateAction: AppRiskDetectionPolicyCandidateAction
): AppRiskDetectionCandidate {
  return riskCandidate({
    candidateId,
    riskSignal,
    sourceKind: SourceKind.KnownCatalog,
    publisherTrustState: 'knownPublisher',
    confidence,
    confidenceBand: 'high',
    policyCandidateAction,
    inventoryEntryRef: `inventory-${candidateId}`,
    identityRef: `identity-${candidateId}`,
  });
}

function heuristicCandidate(
  candidateId: string,
  riskSignal: AppRiskDetectionRiskSignal,
  sourceKind: AppRiskDetectionSourceKind,
  confidence: number
): AppRiskDetectionCandidate {
  return riskCandidate({
    candidateId,
    riskSignal,
    sourceKind,
    publisherTrustState: 'unknownPublisher',
    confidence,
    confidenceBand: 'review',
    policyCandidateAction: 'manualReview',
    sourceRefs: [`source-${candidateId}`],
  });
}

function aiDigestCandidate(): AppRiskDetectionCandidate {
  return riskCandidate({
    candidateId: 'local-ai-social-video-messaging-risk',
    riskSignal: 'socialVideoMessaging',
    sourceKind: SourceKind.LocalAiDigest,
    publisherTrustState: 'knownPublisher',
    confidence: 0.73,
    confidenceBand: 'medium',
    policyCandidateAction: 'askParent',
    localAiDigestRef: 'local-ai-digest-social-video-messaging',
  });
}

function parentOverrideCandidate(): AppRiskDetectionCandidate {
  return riskCandidate({
    candidateId: 'parent-display-override-ai-tool',
    riskSignal: 'aiChatbot',
    sourceKind: SourceKind.ParentOverride,
    publisherTrustState: 'parentTrusted',
    confidence: 0.8,
    confidenceBand: 'medium',
    policyCandidateAction: 'observe',
    parentDisplayLabel: 'Homework AI tool',
  });
}

function riskCandidate(seed: RiskSeed): AppRiskDetectionCandidate {
  const evidenceReferenceId = `evidence-${seed.candidateId}`;
  const candidateState =
    seed.sourceKind === SourceKind.KnownCatalog
      ? AppRiskDetectionCandidateState.CatalogMatch
      : seed.sourceKind === SourceKind.LocalAiDigest
        ? AppRiskDetectionCandidateState.AiCandidate
        : seed.sourceKind === SourceKind.ParentOverride
          ? AppRiskDetectionCandidateState.ParentDisplayOverride
          : AppRiskDetectionCandidateState.HeuristicCandidate;

  return AppRiskDetectionCandidateSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    candidateId: seed.candidateId,
    platform: ParentPlatform.Windows,
    inventoryEntryRef: seed.inventoryEntryRef ?? null,
    identityRef: seed.identityRef ?? null,
    riskSignal: seed.riskSignal,
    sourceKind: seed.sourceKind,
    candidateState,
    publisherTrustState: seed.publisherTrustState,
    confidence: seed.confidence,
    confidenceBand: seed.confidenceBand,
    evidenceReferences: [
      {
        evidenceReferenceId,
        kind: ParentEvidenceReferenceKind.ActivityEvent,
        observedAt: GeneratedAt,
      },
    ],
    sourceRefs: seed.sourceRefs ?? [`source-${seed.candidateId}`],
    localAiDigestRef: seed.localAiDigestRef ?? null,
    parentOverride:
      seed.sourceKind === SourceKind.ParentOverride
        ? {
            parentDisplayLabel: seed.parentDisplayLabel ?? 'Parent label',
            policyCandidateAction: seed.policyCandidateAction,
            rawIdentityChanged: false,
          }
        : null,
    policyCandidateAction: seed.policyCandidateAction,
    policyTargetKind: AppRiskDetectionPolicyTargetKind.RiskApp,
    askParentRouting:
      seed.policyCandidateAction === 'askParent'
        ? AppRiskDetectionAskParentRouting.Available
        : seed.policyCandidateAction === 'manualReview'
          ? AppRiskDetectionAskParentRouting.ManualReview
          : AppRiskDetectionAskParentRouting.NotRouted,
    notDirectEnforcement: true,
    noContentClaim: true,
    surfaceDisclosure: {
      surfaceState: AppRiskDetectionSurfaceState.RiskDisclosureReady,
      confidencePercent: Math.round(seed.confidence * 100),
      sourceEvidenceCount: 1,
      noContentClaimState: AppRiskDetectionNoContentClaimState.NoContentCaptured,
    },
    lastCheckedAt: GeneratedAt,
  });
}
