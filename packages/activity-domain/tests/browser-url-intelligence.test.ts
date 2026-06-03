import { describe, expect, it } from 'vitest';
import {
  BrowserUrlIntelligenceMemoryHitSchema,
  BrowserUrlIntelligenceMemorySchemaVersion,
  BrowserUrlShapeClassificationResultSchema,
  BrowserUrlShapeSchemaVersion,
  parseBrowserUrlShape,
} from '../src/browser';

describe('browser URL intelligence shape contract', () => {
  it('accepts managed exact YouTube video URL shape without AI, policy, or content claims', () => {
    const parsed = BrowserUrlShapeClassificationResultSchema.safeParse(urlShape());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.targetKind).toBe('video');
      expect(parsed.data.platform).toBe('youtube');
      expect(parsed.data.platformIds.videoId).toBe('video-abc123');
      expect(parsed.data.exactUrlEvidence).toBe(true);
      expect(parsed.data.contentSemanticsClaimed).toBe(false);
    }
  });

  it('rejects URL shape results that claim content semantics from URL parsing', () => {
    const parsed = BrowserUrlShapeClassificationResultSchema.safeParse({
      ...urlShape(),
      contentSemanticsClaimed: true,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects URL shape results that claim AI or policy authority', () => {
    const aiClaim = BrowserUrlShapeClassificationResultSchema.safeParse({
      ...urlShape(),
      aiDecisionClaimed: true,
    });
    const policyClaim = BrowserUrlShapeClassificationResultSchema.safeParse({
      ...urlShape(),
      policyDecisionClaimed: true,
    });

    expect(aiClaim.success).toBe(false);
    expect(policyClaim.success).toBe(false);
  });

  it('rejects unmanaged browser process rows promoted to exact video shape', () => {
    const parsed = BrowserUrlShapeClassificationResultSchema.safeParse({
      ...urlShape(),
      sourceKind: 'unmanaged-browser-process',
      exactUrlEvidence: false,
      reasonCodes: ['unmanaged-process-only', 'content-not-inferred'],
    });

    expect(parsed.success).toBe(false);
  });

  it('accepts unmanaged browser process rows only as unknown non-exact evidence', () => {
    const parsed = BrowserUrlShapeClassificationResultSchema.safeParse({
      ...urlShape(),
      sourceKind: 'unmanaged-browser-process',
      url: null,
      domain: null,
      title: null,
      targetKind: 'unknown',
      platform: 'unknown',
      platformIds: emptyPlatformIds(),
      confidence: 'low',
      reasonCodes: ['unmanaged-process-only', 'content-not-inferred'],
      exactUrlEvidence: false,
    });

    expect(parsed.success).toBe(true);
  });

  it('rejects video shape rows without a platform video id', () => {
    const parsed = BrowserUrlShapeClassificationResultSchema.safeParse({
      ...urlShape(),
      platformIds: emptyPlatformIds(),
    });

    expect(parsed.success).toBe(false);
  });
});

describe('browser URL intelligence parser stable platform shapes', () => {
  it('parses YouTube video, shorts, channel, playlist, and search URL shapes', () => {
    expect(parseManagedUrl('https://www.youtube.com/watch?v=video-abc123').platformIds.videoId).toBe('video-abc123');
    expect(parseManagedUrl('https://www.youtube.com/shorts/short-abc123').targetKind).toBe('short-video');
    expect(parseManagedUrl('https://www.youtube.com/@OcentraLearning').platformIds.channelId).toBe('@OcentraLearning');
    expect(parseManagedUrl('https://www.youtube.com/playlist?list=list-abc123').platformIds.playlistId).toBe(
      'list-abc123'
    );
    expect(parseManagedUrl('https://www.youtube.com/results?search_query=fractions').platformIds.query).toBe(
      'fractions'
    );
  });

  it('parses Vimeo, TikTok, and generic web URL shapes without content semantics', () => {
    const vimeo = parseManagedUrl('https://vimeo.com/123456');
    const tiktok = parseManagedUrl('https://www.tiktok.com/@ocentra/video/123456');
    const generic = parseManagedUrl('https://example.test/learn');

    expect(vimeo.platform).toBe('vimeo');
    expect(vimeo.platformIds.videoId).toBe('123456');
    expect(tiktok.platform).toBe('tiktok');
    expect(tiktok.platformIds.videoId).toBe('123456');
    expect(generic.platform).toBe('generic-web');
    expect(generic.contentSemanticsClaimed).toBe(false);
  });
});

describe('browser URL intelligence parser dynamic social route shapes', () => {
  it('parses dynamic social feeds as route evidence without content semantics', () => {
    const instagramFeed = parseManagedUrl('https://www.instagram.com/explore/');
    const tiktokFeed = parseManagedUrl('https://www.tiktok.com/foryou');
    const xFeed = parseManagedUrl('https://x.com/home');

    expect(instagramFeed.targetKind).toBe('social-feed');
    expect(instagramFeed.platform).toBe('instagram');
    expect(instagramFeed.reasonCodes).toContain('dynamic-feed');
    expect(tiktokFeed.targetKind).toBe('social-feed');
    expect(tiktokFeed.platform).toBe('tiktok');
    expect(xFeed.targetKind).toBe('social-feed');
    expect(xFeed.contentSemanticsClaimed).toBe(false);
    expect(xFeed.aiDecisionClaimed).toBe(false);
    expect(xFeed.policyDecisionClaimed).toBe(false);
  });

  it('parses social post, messaging, upload, livestream, and search routes from exact URLs only', () => {
    const instagramReel = parseManagedUrl('https://www.instagram.com/reel/reel-abc123/');
    const xPost = parseManagedUrl('https://x.com/ocentra/status/post-abc123');
    const instagramMessages = parseManagedUrl('https://www.instagram.com/direct/inbox/');
    const instagramUpload = parseManagedUrl('https://www.instagram.com/create/select/');
    const tiktokLive = parseManagedUrl('https://www.tiktok.com/@ocentra/live');
    const xSearch = parseManagedUrl('https://x.com/search?q=fractions');

    expect(instagramReel.targetKind).toBe('short-video');
    expect(instagramReel.platformIds.postId).toBe('reel-abc123');
    expect(xPost.targetKind).toBe('social-post');
    expect(xPost.platformIds.postId).toBe('post-abc123');
    expect(instagramMessages.targetKind).toBe('social-messaging');
    expect(instagramUpload.targetKind).toBe('social-upload-post');
    expect(tiktokLive.targetKind).toBe('social-livestream');
    expect(xSearch.targetKind).toBe('search');
    expect(xSearch.platformIds.query).toBe('fractions');
  });
});

describe('browser URL intelligence parser rejection boundaries', () => {
  it('keeps unsupported schemes and credential-bearing URLs as rejected parser inputs', () => {
    expect(() => parseManagedUrl('file:///C:/Users/child/video.html')).toThrow();
    expect(() => parseManagedUrl('https://child:secret@example.test/learn')).toThrow();
  });

  it('parses non-exact sources as unknown only', () => {
    const parsed = parseBrowserUrlShape({
      classificationId: 'url-shape-unmanaged',
      classifiedAt: '2026-06-03T00:00:00.000Z',
      sourceEvidenceIds: ['browser-evidence-unmanaged'],
      sourceKind: 'unmanaged-browser-process',
      url: 'https://www.youtube.com/watch?v=video-abc123',
      title: 'Ignored unmanaged title',
    });

    expect(parsed.targetKind).toBe('unknown');
    expect(parsed.platform).toBe('unknown');
    expect(parsed.url).toBeNull();
    expect(parsed.reasonCodes).toContain('unmanaged-process-only');
  });

  it('keeps unmanaged social URLs unknown and non-exact', () => {
    const parsed = parseBrowserUrlShape({
      classificationId: 'url-shape-unmanaged-social-feed',
      classifiedAt: '2026-06-03T00:00:00.000Z',
      sourceEvidenceIds: ['browser-evidence-unmanaged-social'],
      sourceKind: 'unmanaged-browser-process',
      url: 'https://www.instagram.com/explore/',
      title: 'Ignored social title',
    });

    expect(parsed.targetKind).toBe('unknown');
    expect(parsed.platform).toBe('unknown');
    expect(parsed.exactUrlEvidence).toBe(false);
  });
});

describe('browser URL intelligence memory hit contract', () => {
  it('accepts fresh evidence-backed memory hits that can only drive policy input', () => {
    const parsed = BrowserUrlIntelligenceMemoryHitSchema.safeParse(memoryHit());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.hitState).toBe('hit');
      expect(parsed.data.canDrivePolicyInput).toBe(true);
      expect(parsed.data.canDirectlyEnforce).toBe(false);
    }
  });

  it('rejects fresh memory hits without evidence, policy version, or expiry refs', () => {
    const parsed = BrowserUrlIntelligenceMemoryHitSchema.safeParse({
      ...memoryHit(),
      sourceEvidenceIds: [],
      policyVersionRef: null,
      expiresAt: null,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects blocked or denied memory hits without a parent action ref', () => {
    const parsed = BrowserUrlIntelligenceMemoryHitSchema.safeParse({
      ...memoryHit(),
      decisionKind: 'known-blocked',
      parentActionRef: null,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects any memory row that claims direct enforcement authority', () => {
    const parsed = BrowserUrlIntelligenceMemoryHitSchema.safeParse({
      ...memoryHit(),
      canDirectlyEnforce: true,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects social post shape rows without a post id', () => {
    const parsed = BrowserUrlShapeClassificationResultSchema.safeParse({
      ...urlShape(),
      targetKind: 'social-post',
      platform: 'x-twitter',
      platformIds: emptyPlatformIds(),
      reasonCodes: ['parsed-url', 'parsed-post-id', 'parsed-social-route', 'content-not-inferred'],
    });

    expect(parsed.success).toBe(false);
  });
});

describe('browser URL intelligence memory non-hit contract', () => {
  it('accepts stale memory hits only when they cannot drive policy input', () => {
    const parsed = BrowserUrlIntelligenceMemoryHitSchema.safeParse({
      ...memoryHit(),
      hitState: 'stale',
      staleReason: 'policy-changed',
      canDrivePolicyInput: false,
    });

    expect(parsed.success).toBe(true);
  });

  it('keeps dynamic feed TTL stale rows from driving policy input', () => {
    const parsed = BrowserUrlIntelligenceMemoryHitSchema.safeParse({
      ...memoryHit(),
      hitState: 'stale',
      staleReason: 'dynamic-feed-ttl',
      canDrivePolicyInput: false,
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.canDrivePolicyInput).toBe(false);
    }
  });

  it('accepts miss rows only as no-hit rows without refs or policy input authority', () => {
    const parsed = BrowserUrlIntelligenceMemoryHitSchema.safeParse({
      ...memoryHit(),
      hitState: 'miss',
      decisionKind: 'no-hit',
      sourceEvidenceIds: [],
      analysisRef: null,
      parentActionRef: null,
      policyVersionRef: null,
      expiresAt: null,
      staleReason: null,
      canDrivePolicyInput: false,
    });

    expect(parsed.success).toBe(true);
  });
});

function urlShape() {
  return {
    schemaVersion: BrowserUrlShapeSchemaVersion,
    classificationId: 'url-shape-2026-06-03-youtube-video',
    classifiedAt: '2026-06-03T00:00:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    sourceKind: 'managed-browser-exact-url',
    url: 'https://www.youtube.com/watch?v=video-abc123',
    domain: 'www.youtube.com',
    title: 'Example math lesson',
    targetKind: 'video',
    platform: 'youtube',
    platformIds: {
      ...emptyPlatformIds(),
      videoId: 'video-abc123',
    },
    confidence: 'high',
    reasonCodes: ['parsed-url', 'parsed-youtube-video-id', 'content-not-inferred'],
    exactUrlEvidence: true,
    contentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
  };
}

function memoryHit() {
  return {
    schemaVersion: BrowserUrlIntelligenceMemorySchemaVersion,
    memoryHitId: 'memory-hit-youtube-video',
    lookedUpAt: '2026-06-03T00:05:00.000Z',
    key: {
      keyKind: 'platform-video-id',
      keyValue: 'youtube:video-abc123',
    },
    hitState: 'hit',
    decisionKind: 'known-allowed',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    analysisRef: 'ai-analysis-youtube-video',
    parentActionRef: null,
    policyVersionRef: 'policy-version-browser-v1',
    expiresAt: '2026-06-04T00:05:00.000Z',
    staleReason: null,
    canDrivePolicyInput: true,
    canDirectlyEnforce: false,
  };
}

function emptyPlatformIds() {
  return {
    videoId: null,
    channelId: null,
    playlistId: null,
    postId: null,
    query: null,
  };
}

function parseManagedUrl(url: string) {
  return parseBrowserUrlShape({
    classificationId: `url-shape-${url.length}`,
    classifiedAt: '2026-06-03T00:00:00.000Z',
    sourceEvidenceIds: ['browser-evidence-managed'],
    sourceKind: 'managed-browser-exact-url',
    url,
    title: 'Evidence title',
  });
}
