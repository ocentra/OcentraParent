import { describe, expect, it } from 'vitest';
import {
  BrowserHiddenAnalysisProfileDesignSchema,
  BrowserHiddenAnalysisSchemaVersion,
} from '../src/browser-hidden-analysis-schemas';
import {
  BrowserHiddenAnalysisLoaderResultSchema,
  BrowserHiddenAnalysisLoaderSchemaVersion,
  planBrowserHiddenAnalysisLoader,
} from '../src/browser-hidden-analysis-loader';

describe('browser hidden analysis profile design contract', () => {
  it('accepts queued Ocentra-owned isolated profile designs', () => {
    const parsed = BrowserHiddenAnalysisProfileDesignSchema.safeParse(hiddenAnalysisProfileDesign());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.state).toBe('queued');
      expect(parsed.data.safety.ocentraOwnedProfile).toBe(true);
      expect(parsed.data.safety.usesChildCookies).toBe(false);
    }
  });

  it('rejects hidden analysis profiles that use child cookies or session tokens', () => {
    const cookies = BrowserHiddenAnalysisProfileDesignSchema.safeParse({
      ...hiddenAnalysisProfileDesign(),
      safety: {
        ...safeHiddenAnalysisProfile(),
        usesChildCookies: true,
      },
    });
    const tokens = BrowserHiddenAnalysisProfileDesignSchema.safeParse({
      ...hiddenAnalysisProfileDesign(),
      safety: {
        ...safeHiddenAnalysisProfile(),
        usesChildSessionTokens: true,
      },
    });

    expect(cookies.success).toBe(false);
    expect(tokens.success).toBe(false);
  });

  it('rejects hidden analysis profiles that are not owned and isolated', () => {
    const parsed = BrowserHiddenAnalysisProfileDesignSchema.safeParse({
      ...hiddenAnalysisProfileDesign(),
      safety: {
        ...safeHiddenAnalysisProfile(),
        ocentraOwnedProfile: false,
        separateFromChildVisibleProfile: false,
      },
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects forbidden loader behaviors in the profile design', () => {
    const parsed = BrowserHiddenAnalysisProfileDesignSchema.safeParse({
      ...hiddenAnalysisProfileDesign(),
      safety: {
        ...safeHiddenAnalysisProfile(),
        allowsDownloads: true,
        allowsFormSubmit: true,
        retainsRawPageBody: true,
      },
    });

    expect(parsed.success).toBe(false);
  });
});

describe('browser hidden analysis loader proof boundary', () => {
  it('rejects metadata-only or analysis-ready states without a loader proof ref', () => {
    const metadataOnly = BrowserHiddenAnalysisProfileDesignSchema.safeParse({
      ...hiddenAnalysisProfileDesign(),
      state: 'metadata-only',
      loaderProofRef: null,
    });
    const analysisReady = BrowserHiddenAnalysisProfileDesignSchema.safeParse({
      ...hiddenAnalysisProfileDesign(),
      state: 'analysis-ready',
      loaderProofRef: null,
    });

    expect(metadataOnly.success).toBe(false);
    expect(analysisReady.success).toBe(false);
  });

  it('accepts manual-required hidden analysis states with explicit reasons', () => {
    const parsed = BrowserHiddenAnalysisProfileDesignSchema.safeParse({
      ...hiddenAnalysisProfileDesign(),
      state: 'manual-required',
      degradedReasons: ['manual-required'],
    });

    expect(parsed.success).toBe(true);
  });
});

describe('browser hidden analysis loader adapter contract', () => {
  it('plans safe queued designs as loading without claiming loader proof', () => {
    const result = planBrowserHiddenAnalysisLoader(hiddenAnalysisLoaderRequest());

    expect(result.state).toBe('loading');
    expect(result.loadedByHiddenAdapter).toBe(false);
    expect(result.loaderProofRef).toBeNull();
  });

  it('returns manual-required when policy disables hidden analysis', () => {
    const result = planBrowserHiddenAnalysisLoader({
      ...hiddenAnalysisLoaderRequest(),
      capabilityState: 'disabled-by-policy',
      policyAllowsHiddenAnalysis: false,
    });

    expect(result.state).toBe('manual-required');
    expect(result.degradedReasons).toContain('disabled-by-policy');
  });

  it('returns manual-required when the loader capability is unavailable', () => {
    const result = planBrowserHiddenAnalysisLoader({
      ...hiddenAnalysisLoaderRequest(),
      capabilityState: 'unavailable',
    });

    expect(result.state).toBe('manual-required');
    expect(result.degradedReasons).toContain('loader-unavailable');
  });

  it('rejects analysis-ready results without a loader proof ref', () => {
    const parsed = BrowserHiddenAnalysisLoaderResultSchema.safeParse({
      ...hiddenAnalysisLoaderResult(),
      state: 'analysis-ready',
      loadedByHiddenAdapter: true,
      loaderProofRef: null,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects loader results that capture page body or transcript text', () => {
    const parsed = BrowserHiddenAnalysisLoaderResultSchema.safeParse({
      ...hiddenAnalysisLoaderResult(),
      pageBodyCaptured: true,
      transcriptTextCaptured: true,
    });

    expect(parsed.success).toBe(false);
  });
});

function hiddenAnalysisProfileDesign() {
  return {
    schemaVersion: BrowserHiddenAnalysisSchemaVersion,
    designId: 'hidden-analysis-profile-design-youtube',
    createdAt: '2026-06-03T00:07:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    urlShapeClassificationId: 'url-shape-2026-06-03-youtube-video',
    hiddenProfileId: 'hidden-analysis-profile-youtube',
    childVisibleProfileRef: null,
    loaderProofRef: null,
    state: 'queued',
    degradedReasons: [],
    timeoutMs: 5000,
    retentionTtlSeconds: 3600,
    maxStructuredSummaryBytes: 4096,
    safety: safeHiddenAnalysisProfile(),
  };
}

function hiddenAnalysisLoaderRequest() {
  return {
    schemaVersion: BrowserHiddenAnalysisLoaderSchemaVersion,
    loaderRequestId: 'hidden-analysis-loader-request-youtube',
    requestedAt: '2026-06-03T00:08:00.000Z',
    profileDesign: hiddenAnalysisProfileDesign(),
    capabilityState: 'available',
    policyAllowsHiddenAnalysis: true,
  };
}

function hiddenAnalysisLoaderResult() {
  return {
    schemaVersion: BrowserHiddenAnalysisLoaderSchemaVersion,
    loaderResultId: 'hidden-analysis-loader-result-youtube',
    loaderRequestId: 'hidden-analysis-loader-request-youtube',
    producedAt: '2026-06-03T00:08:01.000Z',
    profileDesignId: 'hidden-analysis-profile-design-youtube',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    state: 'loading',
    loaderProofRef: null,
    degradedReasons: [],
    loadedByHiddenAdapter: false,
    pageBodyCaptured: false,
    transcriptTextCaptured: false,
  };
}

function safeHiddenAnalysisProfile() {
  return {
    ocentraOwnedProfile: true,
    separateFromChildVisibleProfile: true,
    usesChildCookies: false,
    usesChildSessionTokens: false,
    allowsAutoplayAudio: false,
    allowsDownloads: false,
    allowsFormSubmit: false,
    claimsCaptchaAutomation: false,
    claimsLoginBypass: false,
    retainsRawPageBody: false,
    boundedRetention: true,
  };
}
