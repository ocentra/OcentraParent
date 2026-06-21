import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../../src/browser-url-intelligence';
import { buildVideoMetadataEvidence } from '@ocentra-parent/schema-domain/browser-video-metadata';

describe('browser Vimeo and generic video parser metadata adapter', () => {
  it('parses Vimeo page and player URLs', parsesVimeoUrls);
  it('builds Vimeo metadata evidence from managed exact video shape', buildsVimeoMetadata);
  it('builds generic schema.org video metadata without URL semantic authority', buildsGenericVideoMetadata);
  it('rejects generic OpenGraph-only or unmanaged classifications', rejectsUnsupportedGenericMetadata);
  it('keeps partial Vimeo metadata usable with degraded reasons', acceptsPartialVimeoMetadata);
});

function parsesVimeoUrls() {
  expect(parseManagedUrl('https://vimeo.com/123456').platformIds.videoId).toBe('123456');
  expect(parseManagedUrl('https://player.vimeo.com/video/789012').platformIds.videoId).toBe('789012');
}

function buildsVimeoMetadata() {
  const parsed = buildVideoMetadataEvidence(vimeoMetadataInput('https://player.vimeo.com/video/789012'));

  expect(parsed.metadataState).toBe('available');
  expect(parsed.fields.platformVideoId).toBe('789012');
  expect(parsed.fields.schemaOrgType).toBe('video-object');
  expect(parsed.pageBodyCaptured).toBe(false);
  expect(parsed.transcriptTextCaptured).toBe(false);
}

function buildsGenericVideoMetadata() {
  const parsed = buildVideoMetadataEvidence({
    ...genericVideoMetadataInput(),
    platformVideoIdOverride: 'generic-video-object-1',
  });

  expect(parsed.fields.platformVideoId).toBe('generic-video-object-1');
  expect(parsed.fields.schemaOrgType).toBe('video-object');
  expect(parsed.contentSemanticsClaimed).toBe(false);
  expect(parsed.canDriveAiInput).toBe(true);
}

function rejectsUnsupportedGenericMetadata() {
  expect(() =>
    buildVideoMetadataEvidence({
      ...genericVideoMetadataInput(),
      sourceKind: 'open-graph',
    })
  ).toThrow();

  expect(() =>
    buildVideoMetadataEvidence({
      ...vimeoMetadataInput('https://vimeo.com/123456'),
      classification: parseBrowserUrlShape({
        classificationId: 'url-shape-unmanaged-vimeo',
        classifiedAt: '2026-06-03T04:46:00.000Z',
        sourceEvidenceIds: ['browser-evidence-unmanaged'],
        sourceKind: 'unmanaged-browser-process',
        url: 'https://vimeo.com/123456',
        title: 'Ignored unmanaged title',
      }),
    })
  ).toThrow();
}

function acceptsPartialVimeoMetadata() {
  const parsed = buildVideoMetadataEvidence({
    ...vimeoMetadataInput('https://vimeo.com/123456'),
    transcriptAvailable: false,
    degradedReasons: ['platform-restricted'],
  });

  expect(parsed.metadataState).toBe('partial');
  expect(parsed.degradedReasons).toContain('platform-restricted');
  expect(parsed.policyAuthorityClaimed).toBe(false);
}

function vimeoMetadataInput(url: string) {
  return {
    metadataEvidenceId: `metadata-evidence-vimeo-${url.length}`,
    collectedAt: '2026-06-03T04:46:00.000Z',
    sourceEvidenceIds: ['browser-evidence-vimeo-video'],
    classification: parseManagedUrl(url),
    sourceKind: 'platform-page-metadata',
    sourceRef: 'vimeo-platform-page-metadata',
    browserTitle: 'Vimeo science demo',
    openGraphTitle: 'Vimeo science demo',
    openGraphDescription: 'A science explainer hosted on Vimeo.',
    platformVideoIdOverride: null,
    channelName: 'Ocentra Learning',
    thumbnailUrl: 'https://i.vimeocdn.com/video/789012.jpg',
    thumbnailHashRef: 'thumbnail-hash-vimeo-789012',
    durationSeconds: 360,
    publishDate: '2026-05-21T00:00:00.000Z',
    captionsAvailable: true,
    transcriptAvailable: true,
    platformCategory: 'Education',
    platformRating: null,
    restrictedSignal: null,
    degradedReasons: [],
  };
}

function genericVideoMetadataInput() {
  return {
    metadataEvidenceId: 'metadata-evidence-generic-video',
    collectedAt: '2026-06-03T04:46:00.000Z',
    sourceEvidenceIds: ['browser-evidence-generic-video'],
    classification: parseManagedUrl('https://example.test/lesson/fractions-video'),
    sourceKind: 'schema-org-video-object',
    sourceRef: 'schema-org-video-object',
    browserTitle: 'Fractions video lesson',
    openGraphTitle: 'Fractions video lesson',
    openGraphDescription: 'A generic web page exposing VideoObject metadata.',
    platformVideoIdOverride: null,
    channelName: null,
    thumbnailUrl: 'https://example.test/static/fractions-video.jpg',
    thumbnailHashRef: 'thumbnail-hash-generic-video',
    durationSeconds: 300,
    publishDate: '2026-05-22T00:00:00.000Z',
    captionsAvailable: null,
    transcriptAvailable: null,
    platformCategory: null,
    platformRating: null,
    restrictedSignal: null,
    degradedReasons: [],
  };
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `url-shape-video-${url.length}`,
    classifiedAt: '2026-06-03T04:46:00.000Z',
    sourceEvidenceIds: ['browser-evidence-video'],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Video evidence title',
  });
}
