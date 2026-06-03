import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../src/browser-url-intelligence';
import {
  BrowserSocialFeedRouteClassificationSchema,
  BrowserSocialFeedSurfaceHint,
  classifyBrowserSocialFeedRoute,
} from '../src/browser-social-feed-route-classification';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '../src/browser-social-url-patterns';

describe('browser social feed route classification contract', () => {
  it('classifies reels and shorts feed routes as route-only short-video surfaces', classifiesShortVideoFeeds);
  it('classifies home/following feeds as route-only dynamic feeds', classifiesDynamicFeeds);
  it('classifies exact Shorts video routes without feed content claims', classifiesSingleShortVideo);
  it('rejects non-feed social routes and mismatched surface hints', rejectsUnsupportedRoutes);
  it(
    'rejects feed content, recommendation, message, AI, policy, connector, native, and enforcement claims',
    rejectsClaims
  );
});

function classifiesShortVideoFeeds() {
  const reels = classifyRoute('https://www.instagram.com/reels/', 'reels-feed');

  expect(reels.platform).toBe('instagram');
  expect(reels.routeKind).toBe('feed');
  expect(reels.surfaceKind).toBe('short-video-surface');
  expect(reels.dynamicFeed).toBe(true);
  expect(reels.shortVideoSurface).toBe(true);
}

function classifiesDynamicFeeds() {
  const home = classifyRoute('https://x.com/home', 'home-feed');
  const following = classifyRoute('https://www.twitch.tv/directory/following', 'following-feed');

  expect(home.surfaceKind).toBe('dynamic-feed');
  expect(following.surfaceKind).toBe('dynamic-feed');
  expect(home.feedContentSemanticsClaimed).toBe(false);
}

function classifiesSingleShortVideo() {
  const short = classifyRoute('https://www.youtube.com/shorts/short-video-id', 'single-short-video');

  expect(short.platform).toBe('youtube-shorts');
  expect(short.routeKind).toBe('video');
  expect(short.surfaceKind).toBe('single-short-video');
  expect(short.dynamicFeed).toBe(false);
  expect(short.shortVideoSurface).toBe(true);
}

function rejectsUnsupportedRoutes() {
  expect(() => classifyRoute('https://www.instagram.com/accounts/emailsignup/', 'home-feed')).toThrow();
  expect(() => classifyRoute('https://x.com/home', 'single-short-video')).toThrow();
}

function rejectsClaims() {
  const valid = classifyRoute('https://www.instagram.com/reels/', 'reels-feed');
  const invalidRows = [
    { ...valid, feedContentSemanticsClaimed: true },
    { ...valid, recommendationSemanticsClaimed: true },
    { ...valid, messageContentClaimed: true },
    { ...valid, aiDecisionClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
    { ...valid, routeOnly: false },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialFeedRouteClassificationSchema.safeParse(invalid).success).toBe(false);
  }
}

function classifyRoute(url: string, surfaceHint: BrowserSocialFeedSurfaceHint) {
  return classifyBrowserSocialFeedRoute({
    feedRouteClassificationId: `social-feed-route-${url.length}-${surfaceHint}`,
    observedAt: '2026-06-03T06:12:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-feed-route-${url.length}`],
    routeEvidence: buildBrowserSocialRouteEvidenceFromUrlPattern({
      socialRouteEvidenceId: `social-route-feed-${url.length}`,
      observedAt: '2026-06-03T06:12:00.000Z',
      sourceEvidenceIds: [`browser-evidence-social-route-feed-${url.length}`],
      classification: parseManagedUrl(url),
    }),
    surfaceHint,
  });
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `social-feed-url-shape-${url.length}`,
    classifiedAt: '2026-06-03T06:12:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-feed-url-shape-${url.length}`],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Social feed route URL evidence',
  });
}
