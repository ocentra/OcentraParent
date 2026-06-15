import { describe, expect, it } from 'vitest';
import {
  BrowserGameHiddenAnalysisLoaderRequestSchema,
  BrowserGameHiddenAnalysisLoaderResultSchema,
  BrowserGameHiddenAnalysisProfileDesignSchema,
  planBrowserGameHiddenAnalysisProfileSafety,
} from '../../src/browser-game-hidden-analysis-profile-safety';
import {
  loaderRequest,
  loaderResultFromInvalidClaim,
  manualProfileDesign,
  profileDesign,
  proofBackedProfileDesign,
  proofBackedResult,
  safeProfileFlags,
} from './browser-game-hidden-analysis-profile-safety-fixtures';

describe('browser-game hidden analysis profile safety contracts', () => {
  it('accepts isolated queued profile designs and safe loader requests', acceptsSafeProfileDesigns);
  it('plans safe requests as loading without claiming hidden adapter proof', plansSafeRequests);
  it('accepts disabled, proof-missing, manual-required, and unavailable degraded states', acceptsDegradedStates);
  it('accepts proof-backed metadata-only and analysis-ready result states', acceptsProofBackedResults);
  it(
    'rejects child profile sharing, raw capture, runtime, UI, native, cloud-frame, and enforcement claims',
    rejectsClaims
  );
  it('rejects inconsistent proof upgrades and dishonest capability requests', rejectsInconsistentRows);
});

function acceptsSafeProfileDesigns() {
  const parsedDesign = BrowserGameHiddenAnalysisProfileDesignSchema.safeParse(profileDesign());
  const parsedRequest = BrowserGameHiddenAnalysisLoaderRequestSchema.safeParse(loaderRequest());

  expect(parsedDesign.success).toBe(true);
  expect(parsedRequest.success).toBe(true);
  if (parsedDesign.success) {
    expect(parsedDesign.data.safety.ocentraOwnedProfile).toBe(true);
    expect(parsedDesign.data.safety.usesChildCookies).toBe(false);
    expect(parsedDesign.data.rawPageBodyStored).toBe(false);
  }
}

function plansSafeRequests() {
  const result = planBrowserGameHiddenAnalysisProfileSafety(loaderRequest());

  expect(result.state).toBe('loading');
  expect(result.loadedByHiddenAdapter).toBe(false);
  expect(result.loaderProofRef).toBeNull();
  expect(result.rawGamePayloadCaptured).toBe(false);
}

function acceptsDegradedStates() {
  const disabled = planBrowserGameHiddenAnalysisProfileSafety(
    loaderRequest({ capabilityState: 'disabled-by-policy', policyAllowsHiddenAnalysis: false })
  );
  const proofMissing = planBrowserGameHiddenAnalysisProfileSafety(
    loaderRequest({ capabilityState: 'profile-proof-missing' })
  );
  const unavailable = planBrowserGameHiddenAnalysisProfileSafety(loaderRequest({ capabilityState: 'unavailable' }));

  expect(disabled.state).toBe('disabled-by-policy');
  expect(proofMissing.reasonCodes).toContain('profile-proof-missing');
  expect(unavailable.state).toBe('unavailable');
  expect(BrowserGameHiddenAnalysisProfileDesignSchema.safeParse(manualProfileDesign()).success).toBe(true);
}

function acceptsProofBackedResults() {
  expect(BrowserGameHiddenAnalysisProfileDesignSchema.safeParse(proofBackedProfileDesign()).success).toBe(true);
  expect(BrowserGameHiddenAnalysisLoaderResultSchema.safeParse(proofBackedResult()).success).toBe(true);
  expect(
    BrowserGameHiddenAnalysisLoaderResultSchema.safeParse(
      proofBackedResult({
        state: 'analysis-ready',
        metadataOnly: false,
        summaryRef: 'browser-game-hidden-analysis-summary-full',
      })
    ).success
  ).toBe(true);
}

function rejectsClaims() {
  const invalidSafety = [
    'usesChildCookies',
    'usesChildSessionTokens',
    'sharesStorageWithChildProfile',
    'allowsDownloads',
    'allowsFormSubmit',
    'retainsRawPageBody',
    'retainsRawGamePayload',
    'retainsRawScreenFrame',
  ];
  const invalidProfileClaims = [
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
  ];
  const invalidResultClaims = [
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
  ];

  for (const flag of invalidSafety) {
    expect(
      BrowserGameHiddenAnalysisProfileDesignSchema.safeParse(
        profileDesign({ safety: { ...safeProfileFlags(), [flag]: true } })
      ).success
    ).toBe(false);
  }
  for (const flag of invalidProfileClaims) {
    expect(BrowserGameHiddenAnalysisProfileDesignSchema.safeParse(profileDesign({ [flag]: true })).success).toBe(false);
  }
  for (const flag of invalidResultClaims) {
    expect(
      BrowserGameHiddenAnalysisLoaderResultSchema.safeParse(loaderResultFromInvalidClaim({ [flag]: true })).success
    ).toBe(false);
  }
}

function rejectsInconsistentRows() {
  expect(
    BrowserGameHiddenAnalysisProfileDesignSchema.safeParse(
      profileDesign({
        state: 'metadata-only',
        loaderProofRef: null,
        summaryRef: 'browser-game-hidden-analysis-summary-metadata',
        reasonCodes: ['loader-proof-required'],
      })
    ).success
  ).toBe(false);
  expect(
    BrowserGameHiddenAnalysisLoaderResultSchema.safeParse(
      proofBackedResult({ loaderProofRef: null, state: 'analysis-ready' })
    ).success
  ).toBe(false);
  expect(
    BrowserGameHiddenAnalysisLoaderRequestSchema.safeParse(
      loaderRequest({ capabilityState: 'available', profileDesign: manualProfileDesign() })
    ).success
  ).toBe(false);
}
