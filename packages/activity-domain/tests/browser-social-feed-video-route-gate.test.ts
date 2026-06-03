import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../src/browser-url-intelligence';
import { classifyBrowserSocialFeedRoute } from '../src/browser-social-feed-route-classification';
import { extractBrowserSocialVideoMetadata } from '../src/browser-social-video-metadata';
import {
  BrowserSocialFeedVideoRouteGatePlanSchema,
  planBrowserSocialFeedVideoRouteGate,
} from '../src/browser-social-feed-video-route-gate';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '../src/browser-social-url-patterns';

describe('browser social feed video route gate contract', () => {
  it('plans a block candidate for managed short-video feed routes without executing it', plansShortVideoBlock);
  it('plans a time-limit candidate for dynamic feeds and keeps route-only boundaries', plansDynamicFeedLimit);
  it('plans a video-route parent review from bounded metadata evidence', plansVideoParentReview);
  it('rejects mismatched route and metadata evidence plus missing action refs', rejectsMismatchedEvidenceAndRefs);
  it(
    'rejects runtime, UI, policy finality, native, connector, content, and recommendation claims',
    rejectsRuntimeClaims
  );
});

function plansShortVideoBlock() {
  const gate = planBrowserSocialFeedVideoRouteGate(
    gateInput('https://www.instagram.com/reels/', {
      routeGateAction: 'block-route-candidate',
      reasons: ['short-video-route', 'policy-block-candidate', 'parent-policy-match'],
    })
  );

  expect(gate.platform).toBe('instagram');
  expect(gate.routeGateTargetKind).toBe('social-short-video-route');
  expect(gate.routeGateState).toBe('planned');
  expect(gate.browserNavigationBlockedClaimed).toBe(false);
  expect(gate.enforcementClaimed).toBe(false);
}

function plansDynamicFeedLimit() {
  const gate = planBrowserSocialFeedVideoRouteGate(
    gateInput('https://x.com/home', {
      routeGateAction: 'limit-route-candidate',
      timeLimitCandidateRef: 'social-route-time-limit-candidate-feed',
      reasons: ['dynamic-feed-route', 'schedule-limit-candidate', 'parent-policy-match'],
    })
  );

  expect(gate.routeGateTargetKind).toBe('social-feed-route');
  expect(gate.surfaceKind).toBe('dynamic-feed');
  expect(gate.timeLimitAppliedClaimed).toBe(false);
  expect(gate.feedContentCaptured).toBe(false);
}

function plansVideoParentReview() {
  const gate = planBrowserSocialFeedVideoRouteGate({
    gatePlanId: 'social-video-route-gate-parent-review',
    plannedAt: '2026-06-03T07:08:00.000Z',
    sourceEvidenceIds: ['browser-evidence-social-video-route-gate'],
    feedRouteClassification: null,
    videoMetadataEvidence: videoMetadata('https://www.tiktok.com/@ocentra/video/1234567890'),
    policyDecisionCandidateRef: null,
    parentApprovalRequestRef: 'social-parent-video-review-request',
    timeLimitCandidateRef: null,
    routeGateAction: 'parent-review-candidate',
    parentApprovalRequired: true,
    reasons: ['single-video-route', 'metadata-available', 'parent-review-required'],
  });

  expect(gate.routeGateTargetKind).toBe('social-video-route');
  expect(gate.metadataState).toBe('available');
  expect(gate.parentApprovalRequired).toBe(true);
  expect(gate.parentUiNotifiedClaimed).toBe(false);
}

function rejectsMismatchedEvidenceAndRefs() {
  const valid = gateInput('https://www.youtube.com/shorts/short-video-id');
  const mismatched = {
    ...valid,
    videoMetadataEvidence: videoMetadata('https://www.tiktok.com/@ocentra/video/1234567890'),
  };
  const missingPolicyRef = gateInput('https://www.instagram.com/reels/', {
    routeGateAction: 'block-route-candidate',
    policyDecisionCandidateRef: null,
    reasons: ['short-video-route', 'policy-block-candidate'],
  });
  const missingLimitRef = gateInput('https://x.com/home', {
    routeGateAction: 'limit-route-candidate',
    timeLimitCandidateRef: null,
    reasons: ['dynamic-feed-route', 'schedule-limit-candidate'],
  });

  expect(() => planBrowserSocialFeedVideoRouteGate(mismatched)).toThrow();
  expect(() => planBrowserSocialFeedVideoRouteGate(missingPolicyRef)).toThrow();
  expect(() => planBrowserSocialFeedVideoRouteGate(missingLimitRef)).toThrow();
}

function rejectsRuntimeClaims() {
  const valid = planBrowserSocialFeedVideoRouteGate(gateInput('https://www.instagram.com/reels/'));
  const invalidRows = [
    { ...valid, browserNavigationBlockedClaimed: true },
    { ...valid, browserRedirectClaimed: true },
    { ...valid, cssDomHiddenClaimed: true },
    { ...valid, tabClosedClaimed: true },
    { ...valid, timeLimitAppliedClaimed: true },
    { ...valid, childUiRenderedClaimed: true },
    { ...valid, parentUiNotifiedClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
    { ...valid, feedContentCaptured: true },
    { ...valid, videoContentCaptured: true },
    { ...valid, recommendationModelClaimed: true },
    { ...valid, routeGateAction: 'manual-review-required', routeGateState: 'planned' },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialFeedVideoRouteGatePlanSchema.safeParse(invalid).success).toBe(false);
  }
}

function gateInput(url: string, overrides = {}) {
  const feedRouteClassification = feedRoute(url);
  return {
    gatePlanId: `social-feed-video-route-gate-${url.length}`,
    plannedAt: '2026-06-03T07:07:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-feed-video-gate-${url.length}`],
    feedRouteClassification,
    videoMetadataEvidence: feedRouteClassification.routeKind === 'video' ? videoMetadata(url) : null,
    policyDecisionCandidateRef: 'social-policy-decision-candidate-feed-video',
    parentApprovalRequestRef: null,
    timeLimitCandidateRef: null,
    routeGateAction: 'warn-route-candidate',
    parentApprovalRequired: false,
    reasons: [feedRouteClassification.shortVideoSurface ? 'short-video-route' : 'dynamic-feed-route'],
    ...overrides,
  };
}

function feedRoute(url: string) {
  return classifyBrowserSocialFeedRoute({
    feedRouteClassificationId: `social-feed-video-gate-route-${url.length}`,
    observedAt: '2026-06-03T07:06:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-feed-video-route-${url.length}`],
    routeEvidence: socialRoute(url),
    surfaceHint: url.includes('/shorts/') ? 'single-short-video' : url.includes('/reels') ? 'reels-feed' : 'home-feed',
  });
}

function videoMetadata(url: string) {
  return extractBrowserSocialVideoMetadata({
    metadataEvidenceId: `social-feed-video-gate-metadata-${url.length}`,
    collectedAt: '2026-06-03T07:06:30.000Z',
    sourceEvidenceIds: [`browser-evidence-social-feed-video-metadata-${url.length}`],
    routeEvidence: socialRoute(url),
    sourceKind: 'platform-page-metadata',
    titleRef: 'social-video-route-gate-title-ref',
    descriptionRef: 'social-video-route-gate-description-ref',
    authorHashRef: 'social-video-route-gate-author-hash-ref',
    thumbnailHashRef: 'social-video-route-gate-thumbnail-hash-ref',
    durationSeconds: 73,
    publishedAt: '2026-05-30T00:00:00.000Z',
    categoryRef: null,
    restrictionSignalRef: null,
  });
}

function socialRoute(url: string) {
  return buildBrowserSocialRouteEvidenceFromUrlPattern({
    socialRouteEvidenceId: `social-route-feed-video-gate-${url.length}`,
    observedAt: '2026-06-03T07:06:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-route-feed-video-gate-${url.length}`],
    classification: parseManagedUrl(url),
  });
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `social-feed-video-gate-url-shape-${url.length}`,
    classifiedAt: '2026-06-03T07:06:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-feed-video-gate-url-${url.length}`],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Social feed video route gate URL evidence',
  });
}
