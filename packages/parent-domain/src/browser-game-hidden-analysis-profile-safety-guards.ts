import type { BrowserGameHiddenAnalysisReasonCode } from './browser-game-hidden-analysis-profile-safety-values';
import type {
  BrowserGameHiddenAnalysisLoaderRequest,
  BrowserGameHiddenAnalysisLoaderRequestCandidate,
  BrowserGameHiddenAnalysisLoaderResultCandidate,
  BrowserGameHiddenAnalysisProfileDesignCandidate,
  BrowserGameHiddenAnalysisProfileSafetyFlags,
} from './browser-game-hidden-analysis-profile-safety';

export function browserGameHiddenAnalysisProfileDesignIsHonest(
  design: BrowserGameHiddenAnalysisProfileDesignCandidate
): boolean {
  if (profileDesignClaimsUnsafeAuthority(design) || profileSafetyIsInvalid(design.safety)) {
    return false;
  }
  if (design.state === 'metadata-only' || design.state === 'analysis-ready') {
    return (
      design.loaderProofRef !== null &&
      design.summaryRef !== null &&
      design.reasonCodes.includes('loader-proof-required') &&
      design.confidence !== 'unknown'
    );
  }
  if (design.state === 'queued' || design.state === 'loading') {
    return design.loaderProofRef === null && design.summaryRef === null && design.confidence !== 'high';
  }
  return degradedDesignIsHonest(design);
}

export function browserGameHiddenAnalysisLoaderRequestIsHonest(
  request: BrowserGameHiddenAnalysisLoaderRequestCandidate
): boolean {
  if (!request.policyAllowsHiddenAnalysis) {
    return request.capabilityState === 'disabled-by-policy';
  }
  if (request.capabilityState === 'available') {
    return request.profileDesign.state === 'queued' || request.profileDesign.state === 'loading';
  }
  return request.profileDesign.state !== 'analysis-ready';
}

export function browserGameHiddenAnalysisLoaderResultIsHonest(
  result: BrowserGameHiddenAnalysisLoaderResultCandidate
): boolean {
  if (loaderResultClaimsUnsafeAuthority(result)) {
    return false;
  }
  if (result.state === 'metadata-only' || result.state === 'analysis-ready') {
    return (
      result.loadedByHiddenAdapter &&
      result.loaderProofRef !== null &&
      result.summaryRef !== null &&
      result.reasonCodes.includes('loader-proof-required') &&
      result.confidence !== 'unknown'
    );
  }
  if (result.state === 'queued' || result.state === 'loading') {
    return (
      !result.loadedByHiddenAdapter &&
      result.loaderProofRef === null &&
      result.summaryRef === null &&
      result.confidence !== 'high'
    );
  }
  return degradedResultIsHonest(result);
}

export function browserGameHiddenAnalysisResultForRequest(request: BrowserGameHiddenAnalysisLoaderRequest): unknown {
  if (!request.policyAllowsHiddenAnalysis) {
    return manualRequiredHiddenAnalysisResult(request, 'disabled-by-policy', ['policy-disabled']);
  }
  if (request.capabilityState === 'profile-proof-missing') {
    return manualRequiredHiddenAnalysisResult(request, 'profile-proof-missing', ['profile-proof-missing']);
  }
  if (request.capabilityState === 'manual-required') {
    return manualRequiredHiddenAnalysisResult(request, 'manual-required', ['manual-required']);
  }
  if (request.capabilityState === 'unavailable') {
    return manualRequiredHiddenAnalysisResult(request, 'unavailable', ['unavailable']);
  }

  return hiddenAnalysisBaseResult(request, {
    state: 'loading',
    confidence: 'medium',
    reasonCodes: ['ocentra-owned-profile', 'separate-from-child-profile', 'bounded-retention'],
  });
}

function manualRequiredHiddenAnalysisResult(
  request: BrowserGameHiddenAnalysisLoaderRequest,
  state: BrowserGameHiddenAnalysisLoaderResultCandidate['state'],
  reasonCodes: BrowserGameHiddenAnalysisReasonCode[]
) {
  return hiddenAnalysisBaseResult(request, {
    state,
    confidence: 'low',
    reasonCodes,
  });
}

function hiddenAnalysisBaseResult(
  request: BrowserGameHiddenAnalysisLoaderRequest,
  overrides: Pick<BrowserGameHiddenAnalysisLoaderResultCandidate, 'state' | 'confidence' | 'reasonCodes'>
) {
  return {
    schemaVersion: 'browser-game-hidden-analysis-profile-safety-contract',
    loaderResultId: `${request.loaderRequestId}-result`,
    loaderRequestId: request.loaderRequestId,
    producedAt: request.requestedAt,
    profileDesignId: request.profileDesign.profileDesignId,
    sourceEvidenceRefs: request.profileDesign.sourceEvidenceRefs,
    loaderProofRef: null,
    summaryRef: null,
    loadedByHiddenAdapter: false,
    metadataOnly: false,
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

function degradedDesignIsHonest(design: BrowserGameHiddenAnalysisProfileDesignCandidate): boolean {
  return (
    design.confidence !== 'high' &&
    design.loaderProofRef === null &&
    design.summaryRef === null &&
    design.reasonCodes.some((reason) => degradedReasons.has(reason))
  );
}

function degradedResultIsHonest(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return (
    result.confidence !== 'high' &&
    !result.loadedByHiddenAdapter &&
    result.loaderProofRef === null &&
    result.summaryRef === null &&
    result.reasonCodes.some((reason) => degradedReasons.has(reason))
  );
}

function profileSafetyIsInvalid(safety: BrowserGameHiddenAnalysisProfileSafetyFlags): boolean {
  return (
    !safety.ocentraOwnedProfile ||
    !safety.separateFromChildVisibleProfile ||
    safety.usesChildCookies ||
    safety.usesChildSessionTokens ||
    safety.sharesStorageWithChildProfile ||
    safety.allowsAutoplayAudio ||
    safety.allowsDownloads ||
    safety.allowsFormSubmit ||
    safety.claimsCaptchaAutomation ||
    safety.claimsLoginBypass ||
    safety.retainsRawPageBody ||
    safety.retainsRawGamePayload ||
    safety.retainsRawScreenFrame ||
    !safety.boundedRetention
  );
}

function profileDesignClaimsUnsafeAuthority(design: BrowserGameHiddenAnalysisProfileDesignCandidate): boolean {
  return (
    design.rawUrlStored ||
    design.rawPageBodyStored ||
    design.rawGamePayloadStored ||
    design.rawScreenFrameStored ||
    design.childCookiesOrSessionUsed ||
    design.browserInstrumentationClaimed ||
    design.hiddenNativeControlClaimed ||
    design.aiClassificationClaimed ||
    design.finalPolicyDecisionClaimed ||
    design.uiRenderedClaimed ||
    design.cloudFrameAnalysisClaimed ||
    design.nativeGameControlClaimed ||
    design.enforcementClaimed
  );
}

function loaderResultClaimsUnsafeAuthority(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return (
    result.rawUrlStored ||
    result.rawPageBodyCaptured ||
    result.rawGamePayloadCaptured ||
    result.rawScreenFrameCaptured ||
    result.childCookiesOrSessionUsed ||
    result.browserInstrumentationClaimed ||
    result.hiddenNativeControlClaimed ||
    result.aiClassificationClaimed ||
    result.finalPolicyDecisionClaimed ||
    result.uiRenderedClaimed ||
    result.cloudFrameAnalysisClaimed ||
    result.nativeGameControlClaimed ||
    result.enforcementClaimed
  );
}

const degradedReasons = new Set<BrowserGameHiddenAnalysisReasonCode>([
  'policy-disabled',
  'profile-proof-missing',
  'manual-required',
  'timeout',
  'platform-restricted',
  'unsupported-content',
  'unavailable',
]);
