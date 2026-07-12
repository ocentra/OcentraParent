/* generated from crates/schema/src/app_risk_detection.rs */

export const AppRiskDetectionContractRuntime = {
  SchemaVersion: 'v0.6',
} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export type GeneratedParentPlatform = 'windows' | 'linux' | 'macos' | 'android' | 'ios';
export type GeneratedParentEvidenceReferenceKind = 'activity-event';

export const GeneratedAppRiskDetectionRiskSignal = {
  VpnProxy: 'vpnProxy',
  RemoteDesktop: 'remoteDesktop',
  DownloadTorrent: 'downloadTorrent',
  InstallerUpdater: 'installerUpdater',
  AiChatbot: 'aiChatbot',
  SocialVideoMessaging: 'socialVideoMessaging',
  UnknownRisk: 'unknownRisk',
} as const;
export type GeneratedAppRiskDetectionRiskSignal =
  (typeof GeneratedAppRiskDetectionRiskSignal)[keyof typeof GeneratedAppRiskDetectionRiskSignal];

export const GeneratedAppRiskDetectionSourceKind = {
  KnownCatalog: 'knownCatalog',
  ExecutableName: 'executableName',
  PublisherMetadata: 'publisherMetadata',
  ExecutableHash: 'executableHash',
  LocalAiDigest: 'localAiDigest',
  ParentOverride: 'parentOverride',
} as const;
export type GeneratedAppRiskDetectionSourceKind =
  (typeof GeneratedAppRiskDetectionSourceKind)[keyof typeof GeneratedAppRiskDetectionSourceKind];

export const GeneratedAppRiskDetectionCandidateState = {
  CatalogMatch: 'catalogMatch',
  HeuristicCandidate: 'heuristicCandidate',
  AiCandidate: 'aiCandidate',
  ParentReviewCandidate: 'parentReviewCandidate',
  ParentDisplayOverride: 'parentDisplayOverride',
} as const;
export type GeneratedAppRiskDetectionCandidateState =
  (typeof GeneratedAppRiskDetectionCandidateState)[keyof typeof GeneratedAppRiskDetectionCandidateState];

export const GeneratedAppRiskDetectionPublisherTrustState = {
  KnownPublisher: 'knownPublisher',
  UnknownPublisher: 'unknownPublisher',
  MissingPublisher: 'missingPublisher',
  UnverifiedPublisher: 'unverifiedPublisher',
  ParentTrusted: 'parentTrusted',
} as const;
export type GeneratedAppRiskDetectionPublisherTrustState =
  (typeof GeneratedAppRiskDetectionPublisherTrustState)[keyof typeof GeneratedAppRiskDetectionPublisherTrustState];

export const GeneratedAppRiskDetectionPolicyCandidateAction = {
  None: 'none',
  Observe: 'observe',
  Warn: 'warn',
  AskParent: 'askParent',
  ManualReview: 'manualReview',
} as const;
export type GeneratedAppRiskDetectionPolicyCandidateAction =
  (typeof GeneratedAppRiskDetectionPolicyCandidateAction)[keyof typeof GeneratedAppRiskDetectionPolicyCandidateAction];

export const GeneratedAppRiskDetectionConfidenceBand = {
  High: 'high',
  Medium: 'medium',
  Low: 'low',
  Review: 'review',
} as const;
export type GeneratedAppRiskDetectionConfidenceBand =
  (typeof GeneratedAppRiskDetectionConfidenceBand)[keyof typeof GeneratedAppRiskDetectionConfidenceBand];

export const GeneratedAppRiskDetectionPolicyTargetKind = {
  RiskApp: 'risk-app',
} as const;
export type GeneratedAppRiskDetectionPolicyTargetKind =
  (typeof GeneratedAppRiskDetectionPolicyTargetKind)[keyof typeof GeneratedAppRiskDetectionPolicyTargetKind];

export const GeneratedAppRiskDetectionAskParentRouting = {
  Available: 'available',
  ManualReview: 'manual-review',
  NotRouted: 'not-routed',
} as const;
export type GeneratedAppRiskDetectionAskParentRouting =
  (typeof GeneratedAppRiskDetectionAskParentRouting)[keyof typeof GeneratedAppRiskDetectionAskParentRouting];

export const GeneratedAppRiskDetectionSurfaceState = {
  RiskDisclosureReady: 'riskdisclosure-ready',
} as const;
export type GeneratedAppRiskDetectionSurfaceState =
  (typeof GeneratedAppRiskDetectionSurfaceState)[keyof typeof GeneratedAppRiskDetectionSurfaceState];

export const GeneratedAppRiskDetectionNoContentClaimState = {
  NoContentCaptured: 'no-content-captured',
} as const;
export type GeneratedAppRiskDetectionNoContentClaimState =
  (typeof GeneratedAppRiskDetectionNoContentClaimState)[keyof typeof GeneratedAppRiskDetectionNoContentClaimState];

export const GeneratedAppRiskDetectionRiskSignalValues = [
  'vpnProxy',
  'remoteDesktop',
  'downloadTorrent',
  'installerUpdater',
  'aiChatbot',
  'socialVideoMessaging',
  'unknownRisk',
] as const satisfies readonly GeneratedAppRiskDetectionRiskSignal[];

export const GeneratedAppRiskDetectionSourceKindValues = [
  'knownCatalog',
  'executableName',
  'publisherMetadata',
  'executableHash',
  'localAiDigest',
  'parentOverride',
] as const satisfies readonly GeneratedAppRiskDetectionSourceKind[];

export const GeneratedAppRiskDetectionCandidateStateValues = [
  'catalogMatch',
  'heuristicCandidate',
  'aiCandidate',
  'parentReviewCandidate',
  'parentDisplayOverride',
] as const satisfies readonly GeneratedAppRiskDetectionCandidateState[];

export const GeneratedAppRiskDetectionPublisherTrustStateValues = [
  'knownPublisher',
  'unknownPublisher',
  'missingPublisher',
  'unverifiedPublisher',
  'parentTrusted',
] as const satisfies readonly GeneratedAppRiskDetectionPublisherTrustState[];

export const GeneratedAppRiskDetectionPolicyCandidateActionValues = [
  'none',
  'observe',
  'warn',
  'askParent',
  'manualReview',
] as const satisfies readonly GeneratedAppRiskDetectionPolicyCandidateAction[];

export const GeneratedAppRiskDetectionConfidenceBandValues = [
  'high',
  'medium',
  'low',
  'review',
] as const satisfies readonly GeneratedAppRiskDetectionConfidenceBand[];

export const GeneratedAppRiskDetectionPolicyTargetKindValues = [
  'risk-app',
] as const satisfies readonly GeneratedAppRiskDetectionPolicyTargetKind[];

export const GeneratedAppRiskDetectionAskParentRoutingValues = [
  'available',
  'manual-review',
  'not-routed',
] as const satisfies readonly GeneratedAppRiskDetectionAskParentRouting[];

export const GeneratedAppRiskDetectionSurfaceStateValues = [
  'riskdisclosure-ready',
] as const satisfies readonly GeneratedAppRiskDetectionSurfaceState[];

export const GeneratedAppRiskDetectionNoContentClaimStateValues = [
  'no-content-captured',
] as const satisfies readonly GeneratedAppRiskDetectionNoContentClaimState[];

export interface GeneratedParentEvidenceReference {
  evidenceReferenceId: string;
  kind: GeneratedParentEvidenceReferenceKind;
  observedAt: string;
}

export interface GeneratedAppRiskDetectionParentOverride {
  parentDisplayLabel: string;
  policyCandidateAction: GeneratedAppRiskDetectionPolicyCandidateAction;
  rawIdentityChanged: boolean;
}

export interface GeneratedAppRiskDetectionSurfaceDisclosure {
  surfaceState: GeneratedAppRiskDetectionSurfaceState;
  confidencePercent: number;
  sourceEvidenceCount: number;
  noContentClaimState: GeneratedAppRiskDetectionNoContentClaimState;
}

export interface GeneratedAppRiskDetectionCandidate {
  schemaVersion: GeneratedParentContractSchemaVersion;
  candidateId: string;
  platform: GeneratedParentPlatform;
  inventoryEntryRef: string | null;
  identityRef: string | null;
  riskSignal: GeneratedAppRiskDetectionRiskSignal;
  sourceKind: GeneratedAppRiskDetectionSourceKind;
  candidateState: GeneratedAppRiskDetectionCandidateState;
  publisherTrustState: GeneratedAppRiskDetectionPublisherTrustState;
  confidence: number;
  confidenceBand: GeneratedAppRiskDetectionConfidenceBand;
  evidenceReferences: readonly GeneratedParentEvidenceReference[];
  sourceRefs: readonly string[];
  localAiDigestRef: string | null;
  parentOverride: GeneratedAppRiskDetectionParentOverride | null;
  policyCandidateAction: GeneratedAppRiskDetectionPolicyCandidateAction;
  policyTargetKind: GeneratedAppRiskDetectionPolicyTargetKind;
  askParentRouting: GeneratedAppRiskDetectionAskParentRouting;
  notDirectEnforcement: boolean;
  noContentClaim: boolean;
  surfaceDisclosure: GeneratedAppRiskDetectionSurfaceDisclosure;
  lastCheckedAt: string;
}

export interface GeneratedAppRiskDetectionMatrix {
  schemaVersion: GeneratedParentContractSchemaVersion;
  matrixId: string;
  generatedAt: string;
  candidates: readonly GeneratedAppRiskDetectionCandidate[];
}

export const GeneratedAppRiskDetectionMatrix = {
  schemaVersion: 'v0.6',
  matrixId: 'app-riskdetection-proof-matrix',
  generatedAt: '2026-06-03T10:55:00.000Z',
  candidates: [
    {
      schemaVersion: 'v0.6',
      candidateId: 'known-vpn-proxy-risk',
      platform: 'windows',
      inventoryEntryRef: 'inventory-known-vpn-proxy-risk',
      identityRef: 'identity-known-vpn-proxy-risk',
      riskSignal: 'vpnProxy',
      sourceKind: 'knownCatalog',
      candidateState: 'catalogMatch',
      publisherTrustState: 'knownPublisher',
      confidence: 0.94,
      confidenceBand: 'high',
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-known-vpn-proxy-risk',
          kind: 'activity-event',
          observedAt: '2026-06-03T10:55:00.000Z',
        },
      ],
      sourceRefs: ['source-known-vpn-proxy-risk'],
      localAiDigestRef: null,
      parentOverride: null,
      policyCandidateAction: 'warn',
      policyTargetKind: 'risk-app',
      askParentRouting: 'not-routed',
      notDirectEnforcement: true,
      noContentClaim: true,
      surfaceDisclosure: {
        surfaceState: 'riskdisclosure-ready',
        confidencePercent: 94,
        sourceEvidenceCount: 1,
        noContentClaimState: 'no-content-captured',
      },
      lastCheckedAt: '2026-06-03T10:55:00.000Z',
    },
    {
      schemaVersion: 'v0.6',
      candidateId: 'known-remote-desktop-risk',
      platform: 'windows',
      inventoryEntryRef: 'inventory-known-remote-desktop-risk',
      identityRef: 'identity-known-remote-desktop-risk',
      riskSignal: 'remoteDesktop',
      sourceKind: 'knownCatalog',
      candidateState: 'catalogMatch',
      publisherTrustState: 'knownPublisher',
      confidence: 0.92,
      confidenceBand: 'high',
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-known-remote-desktop-risk',
          kind: 'activity-event',
          observedAt: '2026-06-03T10:55:00.000Z',
        },
      ],
      sourceRefs: ['source-known-remote-desktop-risk'],
      localAiDigestRef: null,
      parentOverride: null,
      policyCandidateAction: 'askParent',
      policyTargetKind: 'risk-app',
      askParentRouting: 'available',
      notDirectEnforcement: true,
      noContentClaim: true,
      surfaceDisclosure: {
        surfaceState: 'riskdisclosure-ready',
        confidencePercent: 92,
        sourceEvidenceCount: 1,
        noContentClaimState: 'no-content-captured',
      },
      lastCheckedAt: '2026-06-03T10:55:00.000Z',
    },
    {
      schemaVersion: 'v0.6',
      candidateId: 'known-download-torrent-risk',
      platform: 'windows',
      inventoryEntryRef: 'inventory-known-download-torrent-risk',
      identityRef: 'identity-known-download-torrent-risk',
      riskSignal: 'downloadTorrent',
      sourceKind: 'knownCatalog',
      candidateState: 'catalogMatch',
      publisherTrustState: 'knownPublisher',
      confidence: 0.9,
      confidenceBand: 'high',
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-known-download-torrent-risk',
          kind: 'activity-event',
          observedAt: '2026-06-03T10:55:00.000Z',
        },
      ],
      sourceRefs: ['source-known-download-torrent-risk'],
      localAiDigestRef: null,
      parentOverride: null,
      policyCandidateAction: 'warn',
      policyTargetKind: 'risk-app',
      askParentRouting: 'not-routed',
      notDirectEnforcement: true,
      noContentClaim: true,
      surfaceDisclosure: {
        surfaceState: 'riskdisclosure-ready',
        confidencePercent: 90,
        sourceEvidenceCount: 1,
        noContentClaimState: 'no-content-captured',
      },
      lastCheckedAt: '2026-06-03T10:55:00.000Z',
    },
    {
      schemaVersion: 'v0.6',
      candidateId: 'known-ai-chatbot-risk',
      platform: 'windows',
      inventoryEntryRef: 'inventory-known-ai-chatbot-risk',
      identityRef: 'identity-known-ai-chatbot-risk',
      riskSignal: 'aiChatbot',
      sourceKind: 'knownCatalog',
      candidateState: 'catalogMatch',
      publisherTrustState: 'knownPublisher',
      confidence: 0.86,
      confidenceBand: 'high',
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-known-ai-chatbot-risk',
          kind: 'activity-event',
          observedAt: '2026-06-03T10:55:00.000Z',
        },
      ],
      sourceRefs: ['source-known-ai-chatbot-risk'],
      localAiDigestRef: null,
      parentOverride: null,
      policyCandidateAction: 'manualReview',
      policyTargetKind: 'risk-app',
      askParentRouting: 'manual-review',
      notDirectEnforcement: true,
      noContentClaim: true,
      surfaceDisclosure: {
        surfaceState: 'riskdisclosure-ready',
        confidencePercent: 86,
        sourceEvidenceCount: 1,
        noContentClaimState: 'no-content-captured',
      },
      lastCheckedAt: '2026-06-03T10:55:00.000Z',
    },
    {
      schemaVersion: 'v0.6',
      candidateId: 'unknown-vpn-name-candidate',
      platform: 'windows',
      inventoryEntryRef: null,
      identityRef: null,
      riskSignal: 'vpnProxy',
      sourceKind: 'executableName',
      candidateState: 'heuristicCandidate',
      publisherTrustState: 'unknownPublisher',
      confidence: 0.42,
      confidenceBand: 'review',
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-unknown-vpn-name-candidate',
          kind: 'activity-event',
          observedAt: '2026-06-03T10:55:00.000Z',
        },
      ],
      sourceRefs: ['source-unknown-vpn-name-candidate'],
      localAiDigestRef: null,
      parentOverride: null,
      policyCandidateAction: 'manualReview',
      policyTargetKind: 'risk-app',
      askParentRouting: 'manual-review',
      notDirectEnforcement: true,
      noContentClaim: true,
      surfaceDisclosure: {
        surfaceState: 'riskdisclosure-ready',
        confidencePercent: 42,
        sourceEvidenceCount: 1,
        noContentClaimState: 'no-content-captured',
      },
      lastCheckedAt: '2026-06-03T10:55:00.000Z',
    },
    {
      schemaVersion: 'v0.6',
      candidateId: 'unknown-publisher-hash-candidate',
      platform: 'windows',
      inventoryEntryRef: null,
      identityRef: null,
      riskSignal: 'unknownRisk',
      sourceKind: 'executableHash',
      candidateState: 'heuristicCandidate',
      publisherTrustState: 'unknownPublisher',
      confidence: 0.38,
      confidenceBand: 'review',
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-unknown-publisher-hash-candidate',
          kind: 'activity-event',
          observedAt: '2026-06-03T10:55:00.000Z',
        },
      ],
      sourceRefs: ['source-unknown-publisher-hash-candidate'],
      localAiDigestRef: null,
      parentOverride: null,
      policyCandidateAction: 'manualReview',
      policyTargetKind: 'risk-app',
      askParentRouting: 'manual-review',
      notDirectEnforcement: true,
      noContentClaim: true,
      surfaceDisclosure: {
        surfaceState: 'riskdisclosure-ready',
        confidencePercent: 38,
        sourceEvidenceCount: 1,
        noContentClaimState: 'no-content-captured',
      },
      lastCheckedAt: '2026-06-03T10:55:00.000Z',
    },
    {
      schemaVersion: 'v0.6',
      candidateId: 'local-ai-social-video-messaging-risk',
      platform: 'windows',
      inventoryEntryRef: null,
      identityRef: null,
      riskSignal: 'socialVideoMessaging',
      sourceKind: 'localAiDigest',
      candidateState: 'aiCandidate',
      publisherTrustState: 'knownPublisher',
      confidence: 0.73,
      confidenceBand: 'medium',
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-local-ai-social-video-messaging-risk',
          kind: 'activity-event',
          observedAt: '2026-06-03T10:55:00.000Z',
        },
      ],
      sourceRefs: ['source-local-ai-social-video-messaging-risk'],
      localAiDigestRef: 'local-ai-digest-social-video-messaging',
      parentOverride: null,
      policyCandidateAction: 'askParent',
      policyTargetKind: 'risk-app',
      askParentRouting: 'available',
      notDirectEnforcement: true,
      noContentClaim: true,
      surfaceDisclosure: {
        surfaceState: 'riskdisclosure-ready',
        confidencePercent: 73,
        sourceEvidenceCount: 1,
        noContentClaimState: 'no-content-captured',
      },
      lastCheckedAt: '2026-06-03T10:55:00.000Z',
    },
    {
      schemaVersion: 'v0.6',
      candidateId: 'parent-display-override-ai-tool',
      platform: 'windows',
      inventoryEntryRef: null,
      identityRef: null,
      riskSignal: 'aiChatbot',
      sourceKind: 'parentOverride',
      candidateState: 'parentDisplayOverride',
      publisherTrustState: 'parentTrusted',
      confidence: 0.8,
      confidenceBand: 'medium',
      evidenceReferences: [
        {
          evidenceReferenceId: 'evidence-parent-display-override-ai-tool',
          kind: 'activity-event',
          observedAt: '2026-06-03T10:55:00.000Z',
        },
      ],
      sourceRefs: ['source-parent-display-override-ai-tool'],
      localAiDigestRef: null,
      parentOverride: {
        parentDisplayLabel: 'Homework AI tool',
        policyCandidateAction: 'observe',
        rawIdentityChanged: false,
      },
      policyCandidateAction: 'observe',
      policyTargetKind: 'risk-app',
      askParentRouting: 'not-routed',
      notDirectEnforcement: true,
      noContentClaim: true,
      surfaceDisclosure: {
        surfaceState: 'riskdisclosure-ready',
        confidencePercent: 80,
        sourceEvidenceCount: 1,
        noContentClaimState: 'no-content-captured',
      },
      lastCheckedAt: '2026-06-03T10:55:00.000Z',
    },
  ],
} as const satisfies GeneratedAppRiskDetectionMatrix;
