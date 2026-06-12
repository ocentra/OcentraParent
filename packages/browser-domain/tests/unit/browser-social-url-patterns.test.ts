import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../../src/browser-url-intelligence';
import {
  buildBrowserSocialRouteEvidenceFromUrlPattern,
  matchBrowserSocialUrlPattern,
} from '../../src/browser-social-url-patterns';

describe('browser social URL pattern library', () => {
  it('builds managed browser signup route evidence from exact social URL shape', buildsSignupRouteEvidence);
  it('maps social URL pattern targets into social route kinds', mapsTargetKindsToRouteKinds);
  it('matches social domains beyond the first URL-shape parser platform set', matchesAdditionalSocialDomains);
  it('rejects unmanaged browser and unknown-domain classifications', rejectsUnsupportedClassifications);
  it('preserves privacy, AI, policy, connector, native, and enforcement no-claim boundaries', preservesNoClaimBoundary);
});

function buildsSignupRouteEvidence() {
  const evidence = buildEvidence('https://www.instagram.com/accounts/emailsignup/');

  expect(evidence.sourceKind).toBe('managed-browser-url-shape');
  expect(evidence.platform).toBe('instagram');
  expect(evidence.routeKind).toBe('account-signup');
  expect(evidence.urlShapeClassificationId).toBe(
    evidenceInput('https://www.instagram.com/accounts/emailsignup/').classification.classificationId
  );
  expect(evidence.exactManagedBrowserRouteEvidence).toBe(true);
}

function mapsTargetKindsToRouteKinds() {
  const cases = [
    ['https://www.instagram.com/direct/inbox/', 'messaging-route'],
    ['https://www.tiktok.com/upload', 'upload-post'],
    ['https://www.twitch.tv/directory/following', 'feed'],
    ['https://www.youtube.com/shorts/short-video-id', 'video'],
    ['https://discord.com/channels/123/456', 'messaging-route'],
    ['https://x.com/home', 'feed'],
  ] as const;

  for (const [url, routeKind] of cases) {
    expect(buildEvidence(url).routeKind).toBe(routeKind);
  }
}

function matchesAdditionalSocialDomains() {
  const snapchat = buildEvidence('https://accounts.snapchat.com/accounts/signup');
  const pinterest = buildEvidence('https://www.pinterest.com/login/');

  expect(snapchat.platform).toBe('snapchat');
  expect(snapchat.routeKind).toBe('account-signup');
  expect(snapchat.urlShapeTargetKind).toBe('unknown');
  expect(pinterest.platform).toBe('pinterest');
  expect(pinterest.routeKind).toBe('login');
}

function rejectsUnsupportedClassifications() {
  const unmanaged = parseBrowserUrlShape({
    classificationId: 'social-pattern-unmanaged',
    classifiedAt: '2026-06-03T05:33:00.000Z',
    sourceEvidenceIds: ['browser-evidence-unmanaged-social'],
    sourceKind: 'unmanaged-browser-process',
    url: 'https://www.instagram.com/accounts/emailsignup/',
    title: 'Ignored unmanaged signup',
  });
  const fakeDomain = parseManagedUrl('https://instagram.example.test/accounts/emailsignup/');

  expect(matchBrowserSocialUrlPattern(unmanaged)).toBeNull();
  expect(matchBrowserSocialUrlPattern(fakeDomain)).toBeNull();
  expect(() => buildEvidenceFromClassification(unmanaged)).toThrow();
  expect(() => buildEvidenceFromClassification(fakeDomain)).toThrow();
}

function preservesNoClaimBoundary() {
  const evidence = buildEvidence('https://www.facebook.com/reel/123456');

  expect(evidence.platform).toBe('facebook');
  expect(evidence.routeKind).toBe('video');
  expect(evidence.platformAccountRef).toBeNull();
  expect(evidence.parentApprovalRequestRef).toBeNull();
  expect(evidence.accountIdentityClaimed).toBe(false);
  expect(evidence.messageContentClaimed).toBe(false);
  expect(evidence.feedContentSemanticsClaimed).toBe(false);
  expect(evidence.aiDecisionClaimed).toBe(false);
  expect(evidence.policyDecisionClaimed).toBe(false);
  expect(evidence.enforcementClaimed).toBe(false);
  expect(evidence.nativeAppControlClaimed).toBe(false);
  expect(evidence.platformConnectorClaimed).toBe(false);
}

function buildEvidence(url: string) {
  return buildBrowserSocialRouteEvidenceFromUrlPattern(evidenceInput(url));
}

function buildEvidenceFromClassification(classification: ReturnType<typeof parseBrowserUrlShape>) {
  return buildBrowserSocialRouteEvidenceFromUrlPattern({
    ...baseEvidenceInput(),
    classification,
  });
}

function evidenceInput(url: string) {
  return {
    ...baseEvidenceInput(),
    socialRouteEvidenceId: `social-route-url-pattern-${url.length}`,
    sourceEvidenceIds: [`browser-evidence-social-url-${url.length}`],
    classification: parseManagedUrl(url),
  };
}

function baseEvidenceInput() {
  return {
    socialRouteEvidenceId: 'social-route-url-pattern',
    observedAt: '2026-06-03T05:33:00.000Z',
    sourceEvidenceIds: ['browser-evidence-social-url'],
    classification: parseManagedUrl('https://www.instagram.com/'),
  };
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `social-url-shape-${url.length}`,
    classifiedAt: '2026-06-03T05:33:00.000Z',
    sourceEvidenceIds: ['browser-evidence-social-url-shape'],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Social URL shape evidence',
  });
}
