import { describe, expect, it } from 'vitest';
import { parseBrowserUrlShape } from '../../src/browser-url-intelligence';
import { buildYouTubeMetadataEvidence } from '@ocentra-parent/schema-domain/browser-youtube-metadata';

describe('browser YouTube parser and metadata adapter', () => {
  it('builds metadata evidence from managed exact YouTube video shape', buildsVideoMetadata);
  it('parses YouTube Shorts, embed, live, channel, and playlist shapes', parsesYouTubeVariants);
  it('keeps partial metadata usable while exposing degraded reasons', acceptsPartialMetadata);
  it('rejects unmanaged or non-YouTube classifications', rejectsUnsupportedClassifications);
  it('does not capture page body, transcript text, AI, policy, or authority claims', rejectsAuthorityCreep);
});

function buildsVideoMetadata() {
  const parsed = buildYouTubeMetadataEvidence(youtubeMetadataInput('https://www.youtube.com/watch?v=video-abc123'));

  expect(parsed.metadataState).toBe('available');
  expect(parsed.fields.platformVideoId).toBe('video-abc123');
  expect(parsed.fields.transcriptAvailable).toBe(true);
  expect(parsed.pageBodyCaptured).toBe(false);
  expect(parsed.transcriptTextCaptured).toBe(false);
  expect(parsed.canDriveAiInput).toBe(true);
}

function parsesYouTubeVariants() {
  expect(parseManagedUrl('https://www.youtube.com/shorts/short-abc123').targetKind).toBe('short-video');
  expect(parseManagedUrl('https://www.youtube.com/embed/embed-abc123').platformIds.videoId).toBe('embed-abc123');
  expect(parseManagedUrl('https://www.youtube.com/live/live-abc123').platformIds.videoId).toBe('live-abc123');
  expect(parseManagedUrl('https://www.youtube.com/@OcentraLearning').targetKind).toBe('channel');
  expect(parseManagedUrl('https://www.youtube.com/playlist?list=list-abc123').targetKind).toBe('playlist');
}

function acceptsPartialMetadata() {
  const parsed = buildYouTubeMetadataEvidence({
    ...youtubeMetadataInput('https://www.youtube.com/watch?v=video-abc123'),
    transcriptAvailable: false,
    degradedReasons: ['platform-restricted'],
  });

  expect(parsed.metadataState).toBe('partial');
  expect(parsed.degradedReasons).toContain('platform-restricted');
  expect(parsed.canDriveAiInput).toBe(true);
}

function rejectsUnsupportedClassifications() {
  expect(() =>
    buildYouTubeMetadataEvidence({
      ...youtubeMetadataInput('https://www.youtube.com/watch?v=video-abc123'),
      classification: parseBrowserUrlShape({
        classificationId: 'url-shape-unmanaged-youtube',
        classifiedAt: '2026-06-03T04:40:00.000Z',
        sourceEvidenceIds: ['browser-evidence-unmanaged'],
        sourceKind: 'unmanaged-browser-process',
        url: 'https://www.youtube.com/watch?v=video-abc123',
        title: 'Ignored unmanaged title',
      }),
    })
  ).toThrow();

  expect(() =>
    buildYouTubeMetadataEvidence({
      ...youtubeMetadataInput('https://www.youtube.com/watch?v=video-abc123'),
      classification: parseManagedUrl('https://example.test/watch?v=video-abc123'),
    })
  ).toThrow();
}

function rejectsAuthorityCreep() {
  const parsed = buildYouTubeMetadataEvidence(youtubeMetadataInput('https://www.youtube.com/watch?v=video-abc123'));

  expect(parsed.contentSemanticsClaimed).toBe(false);
  expect(parsed.aiDecisionClaimed).toBe(false);
  expect(parsed.policyDecisionClaimed).toBe(false);
  expect(parsed.policyAuthorityClaimed).toBe(false);
}

function youtubeMetadataInput(url: string) {
  return {
    metadataEvidenceId: `metadata-evidence-${url.length}`,
    collectedAt: '2026-06-03T04:40:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    classification: parseManagedUrl(url),
    sourceRef: 'youtube-platform-page-metadata',
    browserTitle: 'Introduction to Fractions - Grade 5 Math',
    openGraphTitle: 'Introduction to Fractions - Grade 5 Math',
    openGraphDescription: 'A short fractions lesson for middle school.',
    channelName: 'Ocentra Learning',
    thumbnailUrl: 'https://i.ytimg.com/vi/video-abc123/hqdefault.jpg',
    thumbnailHashRef: 'thumbnail-hash-video-abc123',
    durationSeconds: 420,
    publishDate: '2026-05-20T00:00:00.000Z',
    captionsAvailable: true,
    transcriptAvailable: true,
    platformCategory: 'Education',
    platformRating: null,
    restrictedSignal: null,
    degradedReasons: [],
  };
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `url-shape-youtube-${url.length}`,
    classifiedAt: '2026-06-03T04:40:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'YouTube evidence title',
  });
}
