import type {
  BrowserGameHiddenAnalysisLoaderRequest,
  BrowserGameHiddenAnalysisLoaderResult,
  BrowserGameHiddenAnalysisProfileDesign,
} from '../src/browser-game-hidden-analysis-profile-safety';

export function profileDesign(overrides = {}): BrowserGameHiddenAnalysisProfileDesign {
  return {
    schemaVersion: 'browser-game-hidden-analysis-profile-safety-contract',
    profileDesignId: 'browser-game-hidden-profile-design-portal',
    designedAt: '2026-06-03T12:03:00.000Z',
    profileKind: 'isolated-managed-profile',
    profileFingerprint: 'browser-game-hidden-profile-fingerprint-portal',
    sourceEvidenceRefs: ['browser-game-hidden-analysis-source-ref'],
    state: 'queued',
    confidence: 'medium',
    loaderProofRef: null,
    summaryRef: null,
    reasonCodes: ['ocentra-owned-profile', 'separate-from-child-profile', 'bounded-retention'],
    retentionTtlSeconds: 1800,
    maxStructuredSummaryBytes: 4096,
    safety: safeProfileFlags(),
    rawUrlStored: false,
    rawPageBodyStored: false,
    rawGamePayloadStored: false,
    rawScreenFrameStored: false,
    childCookiesOrSessionUsed: false,
    browserInstrumentationClaimed: false,
    hiddenNativeControlClaimed: false,
    aiClassificationClaimed: false,
    finalPolicyDecisionClaimed: false,
    uiRenderedClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

export function manualProfileDesign(overrides = {}): BrowserGameHiddenAnalysisProfileDesign {
  return profileDesign({
    profileDesignId: 'browser-game-hidden-profile-design-manual',
    profileKind: 'manual-required',
    profileFingerprint: 'browser-game-hidden-profile-fingerprint-manual',
    state: 'manual-required',
    confidence: 'low',
    reasonCodes: ['manual-required'],
    ...overrides,
  });
}

export function proofBackedProfileDesign(overrides = {}): BrowserGameHiddenAnalysisProfileDesign {
  return profileDesign({
    profileDesignId: 'browser-game-hidden-profile-design-proof-backed',
    profileKind: 'metadata-only-profile',
    profileFingerprint: 'browser-game-hidden-profile-fingerprint-proof-backed',
    state: 'metadata-only',
    confidence: 'medium',
    loaderProofRef: 'browser-game-hidden-loader-proof',
    summaryRef: 'browser-game-hidden-analysis-summary-metadata',
    reasonCodes: ['loader-proof-required'],
    ...overrides,
  });
}

export function loaderRequest(overrides = {}): BrowserGameHiddenAnalysisLoaderRequest {
  return {
    schemaVersion: 'browser-game-hidden-analysis-profile-safety-contract',
    loaderRequestId: 'browser-game-hidden-loader-request-portal',
    requestedAt: '2026-06-03T12:04:00.000Z',
    profileDesign: profileDesign(),
    capabilityState: 'available',
    policyAllowsHiddenAnalysis: true,
    ...overrides,
  };
}

export function proofBackedResult(overrides = {}): BrowserGameHiddenAnalysisLoaderResult {
  return {
    schemaVersion: 'browser-game-hidden-analysis-profile-safety-contract',
    loaderResultId: 'browser-game-hidden-loader-result-proof-backed',
    loaderRequestId: 'browser-game-hidden-loader-request-portal',
    producedAt: '2026-06-03T12:05:00.000Z',
    profileDesignId: 'browser-game-hidden-profile-design-proof-backed',
    sourceEvidenceRefs: ['browser-game-hidden-analysis-source-ref'],
    state: 'metadata-only',
    confidence: 'medium',
    loaderProofRef: 'browser-game-hidden-loader-proof',
    summaryRef: 'browser-game-hidden-analysis-summary-metadata',
    reasonCodes: ['loader-proof-required'],
    loadedByHiddenAdapter: true,
    metadataOnly: true,
    rawUrlStored: false,
    rawPageBodyCaptured: false,
    rawGamePayloadCaptured: false,
    rawScreenFrameCaptured: false,
    childCookiesOrSessionUsed: false,
    browserInstrumentationClaimed: false,
    hiddenNativeControlClaimed: false,
    aiClassificationClaimed: false,
    finalPolicyDecisionClaimed: false,
    uiRenderedClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

export function loaderResultFromInvalidClaim(overrides = {}): BrowserGameHiddenAnalysisLoaderResult {
  return {
    ...proofBackedResult(),
    ...overrides,
  };
}

export function safeProfileFlags() {
  return {
    ocentraOwnedProfile: true,
    separateFromChildVisibleProfile: true,
    usesChildCookies: false,
    usesChildSessionTokens: false,
    sharesStorageWithChildProfile: false,
    allowsAutoplayAudio: false,
    allowsDownloads: false,
    allowsFormSubmit: false,
    claimsCaptchaAutomation: false,
    claimsLoginBypass: false,
    retainsRawPageBody: false,
    retainsRawGamePayload: false,
    retainsRawScreenFrame: false,
    boundedRetention: true,
  };
}
