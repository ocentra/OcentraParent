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
  if (hiddenAnalysisLoadedStateIsReady(result.state)) {
    return loadedHiddenAnalysisResultIsHonest(result);
  }
  if (hiddenAnalysisPendingStateIsReady(result.state)) {
    return pendingHiddenAnalysisResultIsHonest(result);
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

function hiddenAnalysisLoadedStateIsReady(state: BrowserGameHiddenAnalysisLoaderResultCandidate['state']): boolean {
  return state === 'metadata-only' || state === 'analysis-ready';
}

function hiddenAnalysisPendingStateIsReady(state: BrowserGameHiddenAnalysisLoaderResultCandidate['state']): boolean {
  return state === 'queued' || state === 'loading';
}

function loadedHiddenAnalysisResultIsHonest(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return (
    result.loadedByHiddenAdapter &&
    result.loaderProofRef !== null &&
    result.summaryRef !== null &&
    result.reasonCodes.includes('loader-proof-required') &&
    result.confidence !== 'unknown'
  );
}

function pendingHiddenAnalysisResultIsHonest(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return (
    !result.loadedByHiddenAdapter &&
    result.loaderProofRef === null &&
    result.summaryRef === null &&
    result.confidence !== 'high'
  );
}

function profileSafetyIsInvalid(safety: BrowserGameHiddenAnalysisProfileSafetyFlags): boolean {
  return (
    RequiredProfileSafetyFields.some((field) => safety[field] !== true) ||
    UnsafeProfileSafetyFields.some((field) => safety[field] === true)
  );
}

function profileDesignClaimsUnsafeAuthority(design: BrowserGameHiddenAnalysisProfileDesignCandidate): boolean {
  return ProfileDesignUnsafeAuthorityFields.some((field) => design[field] === true);
}

function loaderResultClaimsUnsafeAuthority(result: BrowserGameHiddenAnalysisLoaderResultCandidate): boolean {
  return LoaderResultUnsafeAuthorityFields.some((field) => result[field] === true);
}

const RequiredProfileSafetyFields = [
  'ocentraOwnedProfile',
  'separateFromChildVisibleProfile',
  'boundedRetention',
] as const satisfies ReadonlyArray<keyof BrowserGameHiddenAnalysisProfileSafetyFlags>;

const UnsafeProfileSafetyFields = [
  'usesChildCookies',
  'usesChildSessionTokens',
  'sharesStorageWithChildProfile',
  'allowsAutoplayAudio',
  'allowsDownloads',
  'allowsFormSubmit',
  'claimsCaptchaAutomation',
  'claimsLoginBypass',
  'retainsRawPageBody',
  'retainsRawGamePayload',
  'retainsRawScreenFrame',
] as const satisfies ReadonlyArray<keyof BrowserGameHiddenAnalysisProfileSafetyFlags>;

const ProfileDesignUnsafeAuthorityFields = [
  'rawUrlStored',
  'rawPageBodyStored',
  'rawGamePayloadStored',
  'rawScreenFrameStored',
  'childCookiesOrSessionUsed',
  'browserInstrumentationClaimed',
  'hiddenNativeControlClaimed',
  'aiClassificationClaimed',
  'finalPolicyDecisionClaimed',
  'uiRenderedClaimed',
  'cloudFrameAnalysisClaimed',
  'nativeGameControlClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameHiddenAnalysisProfileDesignCandidate>;

const LoaderResultUnsafeAuthorityFields = [
  'rawUrlStored',
  'rawPageBodyCaptured',
  'rawGamePayloadCaptured',
  'rawScreenFrameCaptured',
  'childCookiesOrSessionUsed',
  'browserInstrumentationClaimed',
  'hiddenNativeControlClaimed',
  'aiClassificationClaimed',
  'finalPolicyDecisionClaimed',
  'uiRenderedClaimed',
  'cloudFrameAnalysisClaimed',
  'nativeGameControlClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameHiddenAnalysisLoaderResultCandidate>;

const degradedReasons = new Set<BrowserGameHiddenAnalysisReasonCode>([
  'policy-disabled',
  'profile-proof-missing',
  'manual-required',
  'timeout',
  'platform-restricted',
  'unsupported-content',
  'unavailable',
]);
