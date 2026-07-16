/* generated from crates/browser-core/src/browser_generated_social_ts.rs */

import type {
  BrowserUrlShapeTargetKind,
  BrowserUrlShapeSourceKind,
  BrowserUrlIntelligenceMemoryHitState,
  BrowserUrlIntelligenceMemoryDecisionKind,
} from './generated-browser-url-intelligence-schemas';

export type BrowserUrlShapeClassificationCandidate = {
  readonly sourceKind: BrowserUrlShapeSourceKind;
  readonly url: string | null;
  readonly domain: string | null;
  readonly targetKind: BrowserUrlShapeTargetKind;
  readonly platform: string;
  readonly platformIds: {
    readonly videoId: string | null;
    readonly channelId: string | null;
    readonly playlistId: string | null;
    readonly postId: string | null;
    readonly query: string | null;
  };
  readonly confidence: string;
  readonly reasonCodes: readonly string[];
  readonly exactUrlEvidence: boolean;
  readonly contentSemanticsClaimed: boolean;
  readonly aiDecisionClaimed: boolean;
  readonly policyDecisionClaimed: boolean;
};

export type BrowserUrlIntelligenceMemoryHitCandidate = {
  readonly hitState: BrowserUrlIntelligenceMemoryHitState;
  readonly decisionKind: BrowserUrlIntelligenceMemoryDecisionKind;
  readonly sourceEvidenceIds: readonly string[];
  readonly analysisRef: string | null;
  readonly parentActionRef: string | null;
  readonly policyVersionRef: string | null;
  readonly expiresAt: string | null;
  readonly staleReason: string | null;
  readonly canDrivePolicyInput: boolean;
  readonly canDirectlyEnforce: boolean;
};

type MemoryHitValidator = (value: BrowserUrlIntelligenceMemoryHitCandidate) => boolean;

const MemoryHitValidators = {
  hit: activeMemoryHitIsConsistent,
  miss: missedMemoryHitIsConsistent,
  stale: staleMemoryHitIsConsistent,
  'manual-required': manualRequiredMemoryHitIsConsistent,
} as const satisfies Record<BrowserUrlIntelligenceMemoryHitState, MemoryHitValidator>;

const VideoPlatforms = new Set(['youtube', 'youtube-shorts', 'vimeo']);
const VideoKinds = new Set<BrowserUrlShapeTargetKind>(['video', 'short-video']);

export function browserUrlShapeClassificationResultIsConsistent(value: BrowserUrlShapeClassificationCandidate) {
  return (
    !claimsSemanticsOrPolicy(value) &&
    value.exactUrlEvidence === (value.sourceKind === 'managed-browser-exact-url') &&
    (value.sourceKind === 'managed-browser-exact-url'
      ? managedExactUrlShapeIsConsistent(value)
      : nonExactUrlShapeIsConsistent(value))
  );
}

export function browserUrlIntelligenceMemoryHitIsConsistent(value: BrowserUrlIntelligenceMemoryHitCandidate) {
  return !value.canDirectlyEnforce && MemoryHitValidators[value.hitState](value);
}

function claimsSemanticsOrPolicy(value: BrowserUrlShapeClassificationCandidate) {
  return value.contentSemanticsClaimed || value.aiDecisionClaimed || value.policyDecisionClaimed;
}

function managedExactUrlShapeIsConsistent(value: BrowserUrlShapeClassificationCandidate) {
  return (
    value.url !== null &&
    value.domain !== null &&
    (value.targetKind !== 'unknown' || value.confidence !== 'high') &&
    targetSpecificIdsAreConsistent(value)
  );
}

function nonExactUrlShapeIsConsistent(value: BrowserUrlShapeClassificationCandidate) {
  return (
    value.targetKind === 'unknown' &&
    value.platform === 'unknown' &&
    value.confidence !== 'high' &&
    allPlatformIdsMissing(value) &&
    value.reasonCodes.includes(nonExactEvidenceReason(value.sourceKind))
  );
}

function allPlatformIdsMissing(value: BrowserUrlShapeClassificationCandidate) {
  return (
    value.platformIds.videoId === null &&
    value.platformIds.channelId === null &&
    value.platformIds.playlistId === null &&
    value.platformIds.postId === null &&
    value.platformIds.query === null
  );
}

function targetSpecificIdsAreConsistent(value: BrowserUrlShapeClassificationCandidate) {
  if (VideoPlatforms.has(value.platform) && VideoKinds.has(value.targetKind)) {
    return value.platformIds.videoId !== null;
  }
  if (value.targetKind === 'channel') {
    return value.platformIds.channelId !== null;
  }
  if (value.targetKind === 'playlist') {
    return value.platformIds.playlistId !== null;
  }
  if (value.targetKind === 'search') {
    return value.platformIds.query !== null;
  }
  if (value.targetKind === 'social-post') {
    return value.platformIds.postId !== null;
  }
  return true;
}

function nonExactEvidenceReason(value: BrowserUrlShapeSourceKind) {
  return value === 'unmanaged-browser-process'
    ? 'unmanaged-process-only'
    : value === 'network-domain'
      ? 'network-domain-only'
      : 'no-exact-evidence';
}

function missedMemoryHitIsConsistent(value: BrowserUrlIntelligenceMemoryHitCandidate) {
  return (
    value.decisionKind === 'no-hit' &&
    value.sourceEvidenceIds.length === 0 &&
    value.analysisRef === null &&
    value.parentActionRef === null &&
    value.policyVersionRef === null &&
    value.expiresAt === null &&
    value.staleReason === null &&
    !value.canDrivePolicyInput
  );
}

function staleMemoryHitIsConsistent(value: BrowserUrlIntelligenceMemoryHitCandidate) {
  return value.sourceEvidenceIds.length > 0 && value.staleReason !== null && !value.canDrivePolicyInput;
}

function manualRequiredMemoryHitIsConsistent(value: BrowserUrlIntelligenceMemoryHitCandidate) {
  return value.decisionKind === 'manual-required' && !value.canDrivePolicyInput;
}

function activeMemoryHitIsConsistent(value: BrowserUrlIntelligenceMemoryHitCandidate) {
  return (
    value.sourceEvidenceIds.length > 0 &&
    value.policyVersionRef !== null &&
    value.expiresAt !== null &&
    value.staleReason === null &&
    value.canDrivePolicyInput &&
    browserUrlIntelligenceMemoryHitHasDecisionSource(value)
  );
}

function browserUrlIntelligenceMemoryHitHasDecisionSource(value: BrowserUrlIntelligenceMemoryHitCandidate) {
  return value.decisionKind === 'known-blocked' || value.decisionKind === 'previously-denied'
    ? value.parentActionRef !== null
    : value.analysisRef !== null || value.parentActionRef !== null;
}
