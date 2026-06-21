import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../../src/browser-url-intelligence';
import {
  BrowserSocialVideoMetadataEvidenceSchema,
  extractBrowserSocialVideoMetadata,
} from '@ocentra-parent/schema-domain/browser-social-video-metadata';
import { buildBrowserSocialRouteEvidenceFromUrlPattern } from '@ocentra-parent/schema-domain/browser-social-url-patterns';

describe('browser social video metadata extractor contract', () => {
  it('extracts available social video metadata from managed route evidence', extractsAvailableMetadata);
  it('extracts partial metadata without page body or transcript text', extractsPartialMetadata);
  it('rejects unsupported route evidence and manual metadata with refs', rejectsUnsupportedInputs);
  it(
    'rejects page body, transcript, message, feed, AI, policy, connector, native, and enforcement claims',
    rejectsClaims
  );
});

function extractsAvailableMetadata() {
  const evidence = extractMetadata('https://www.tiktok.com/@ocentra/video/1234567890', {
    titleRef: 'metadata-title-ref-tiktok-video',
    thumbnailHashRef: 'thumbnail-hash-ref-tiktok-video',
  });

  expect(evidence.platform).toBe('tiktok');
  expect(evidence.routeKind).toBe('video');
  expect(evidence.metadataState).toBe('available');
  expect(evidence.pageBodyCaptured).toBe(false);
  expect(evidence.transcriptTextCaptured).toBe(false);
}

function extractsPartialMetadata() {
  const evidence = extractMetadata('https://www.instagram.com/reel/short-video-id/', {
    titleRef: null,
    thumbnailHashRef: 'thumbnail-hash-ref-instagram-reel',
  });

  expect(evidence.platform).toBe('instagram');
  expect(evidence.metadataState).toBe('partial');
  expect(evidence.contentSemanticsClaimed).toBe(false);
}

function rejectsUnsupportedInputs() {
  expect(() => extractMetadata('https://www.instagram.com/accounts/emailsignup/', {})).toThrow();
  expect(() =>
    extractBrowserSocialVideoMetadata({
      ...metadataInput('https://www.tiktok.com/@ocentra/video/1234567890', {}),
      sourceKind: 'manual-required',
      titleRef: 'manual-required-title-ref-not-allowed',
    })
  ).toThrow();
}

function rejectsClaims() {
  const valid = extractMetadata('https://www.tiktok.com/@ocentra/video/1234567890', {
    titleRef: 'metadata-title-ref-tiktok-video',
    thumbnailHashRef: 'thumbnail-hash-ref-tiktok-video',
  });
  const invalidRows = [
    { ...valid, pageBodyCaptured: true },
    { ...valid, transcriptTextCaptured: true },
    { ...valid, messageContentCaptured: true },
    { ...valid, feedContentSemanticsClaimed: true },
    { ...valid, contentSemanticsClaimed: true },
    { ...valid, aiDecisionClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialVideoMetadataEvidenceSchema.safeParse(invalid).success).toBe(false);
  }
}

function extractMetadata(
  url: string,
  overrides: { readonly titleRef?: string | null; readonly thumbnailHashRef?: string | null }
) {
  return extractBrowserSocialVideoMetadata(metadataInput(url, overrides));
}

function metadataInput(
  url: string,
  overrides: { readonly titleRef?: string | null; readonly thumbnailHashRef?: string | null }
) {
  return {
    metadataEvidenceId: `social-video-metadata-${url.length}`,
    collectedAt: '2026-06-03T06:18:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-video-metadata-${url.length}`],
    routeEvidence: buildBrowserSocialRouteEvidenceFromUrlPattern({
      socialRouteEvidenceId: `social-route-video-metadata-${url.length}`,
      observedAt: '2026-06-03T06:18:00.000Z',
      sourceEvidenceIds: [`browser-evidence-social-route-video-metadata-${url.length}`],
      classification: parseManagedUrl(url),
    }),
    sourceKind: 'platform-page-metadata',
    titleRef: optionalOverride(overrides.titleRef, 'metadata-title-ref-social-video'),
    descriptionRef: 'metadata-description-ref-social-video',
    authorHashRef: 'author-hash-ref-social-video',
    thumbnailHashRef: optionalOverride(overrides.thumbnailHashRef, 'thumbnail-hash-ref-social-video'),
    durationSeconds: 61,
    publishedAt: '2026-05-27T00:00:00.000Z',
    categoryRef: null,
    restrictionSignalRef: null,
  };
}

function optionalOverride(value: string | null | undefined, fallback: string) {
  return value === undefined ? fallback : value;
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `social-video-metadata-url-shape-${url.length}`,
    classifiedAt: '2026-06-03T06:18:00.000Z',
    sourceEvidenceIds: [`browser-evidence-social-video-url-shape-${url.length}`],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Social video metadata URL evidence',
  });
}
