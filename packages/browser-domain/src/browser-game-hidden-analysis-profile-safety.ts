import {
  decodeBrowserGameHiddenAnalysisLoaderRequest,
  decodeBrowserGameHiddenAnalysisLoaderResult,
  type BrowserGameHiddenAnalysisLoaderRequest,
  type BrowserGameHiddenAnalysisLoaderResult,
} from '@ocentra-parent/schema-domain/browser-game-hidden-analysis-profile-safety';
import { type BrowserGameHiddenAnalysisReasonCode } from '@ocentra-parent/schema-domain/browser-game-hidden-analysis-profile-safety-values';

export function planBrowserGameHiddenAnalysisProfileSafety(input: unknown): BrowserGameHiddenAnalysisLoaderResult {
  const request = decodeBrowserGameHiddenAnalysisLoaderRequest(input);
  const result = browserGameHiddenAnalysisResultForRequest(request);

  return decodeBrowserGameHiddenAnalysisLoaderResult(result);
}

function browserGameHiddenAnalysisResultForRequest(request: BrowserGameHiddenAnalysisLoaderRequest): unknown {
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
  state: BrowserGameHiddenAnalysisLoaderResult['state'],
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
  overrides: Pick<BrowserGameHiddenAnalysisLoaderResult, 'state' | 'confidence' | 'reasonCodes'>
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
