import { describe, expect, it } from 'vitest';
import { BrowserUrlMetadataEvidenceSchema, BrowserUrlMetadataSchemaVersion } from '../../src/browser';

describe('browser URL metadata evidence available contract', () => {
  it('accepts evidence-backed metadata that can drive AI input but not authority claims', () => {
    const parsed = BrowserUrlMetadataEvidenceSchema.safeParse(metadataEvidence());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.metadataState).toBe('available');
      expect(parsed.data.canDriveAiInput).toBe(true);
      expect(parsed.data.policyAuthorityClaimed).toBe(false);
    }
  });

  it('rejects available metadata rows without metadata fields', () => {
    const parsed = BrowserUrlMetadataEvidenceSchema.safeParse({
      ...metadataEvidence(),
      fields: emptyMetadataFields(),
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects metadata rows that capture page body, transcript text, AI, or policy authority', () => {
    const pageBody = BrowserUrlMetadataEvidenceSchema.safeParse({
      ...metadataEvidence(),
      pageBodyCaptured: true,
    });
    const aiClaim = BrowserUrlMetadataEvidenceSchema.safeParse({
      ...metadataEvidence(),
      aiDecisionClaimed: true,
    });

    expect(pageBody.success).toBe(false);
    expect(aiClaim.success).toBe(false);
  });
});

describe('browser URL metadata evidence degraded contract', () => {
  it('rejects hidden managed analysis metadata without separate proof', () => {
    const parsed = BrowserUrlMetadataEvidenceSchema.safeParse({
      ...metadataEvidence(),
      source: {
        sourceKind: 'hidden-managed-analysis',
        sourceRef: 'hidden-analysis-run',
        hiddenAnalysisProofRef: null,
      },
    });

    expect(parsed.success).toBe(false);
  });

  it('accepts degraded metadata only without AI-input authority', () => {
    const parsed = BrowserUrlMetadataEvidenceSchema.safeParse({
      ...metadataEvidence(),
      metadataState: 'degraded',
      degradedReasons: ['platform-restricted'],
      canDriveAiInput: false,
    });

    expect(parsed.success).toBe(true);
  });

  it('rejects platform category or rating metadata when treated as policy authority', () => {
    const parsed = BrowserUrlMetadataEvidenceSchema.safeParse({
      ...metadataEvidence(),
      policyAuthorityClaimed: true,
    });

    expect(parsed.success).toBe(false);
  });
});

function metadataEvidence() {
  return {
    schemaVersion: BrowserUrlMetadataSchemaVersion,
    metadataEvidenceId: 'metadata-evidence-youtube-video',
    collectedAt: '2026-06-03T00:06:00.000Z',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    urlShapeClassificationId: 'url-shape-2026-06-03-youtube-video',
    source: {
      sourceKind: 'open-graph',
      sourceRef: 'managed-browser-metadata-open-graph',
      hiddenAnalysisProofRef: null,
    },
    metadataState: 'available',
    fields: {
      ...emptyMetadataFields(),
      openGraphTitle: 'Example math lesson',
      openGraphDescription: 'A short fractions lesson for middle school.',
      platformVideoId: 'video-abc123',
      platformChannelId: 'channel-abc123',
      channelName: 'Ocentra Learning',
      durationSeconds: 420,
      captionsAvailable: true,
      platformCategory: 'Education',
    },
    degradedReasons: [],
    pageBodyCaptured: false,
    transcriptTextCaptured: false,
    contentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    policyAuthorityClaimed: false,
    canDriveAiInput: true,
  };
}

function emptyMetadataFields() {
  return {
    browserTitle: null,
    openGraphTitle: null,
    openGraphDescription: null,
    schemaOrgType: null,
    platformVideoId: null,
    platformChannelId: null,
    channelName: null,
    thumbnailUrl: null,
    thumbnailHashRef: null,
    durationSeconds: null,
    publishDate: null,
    captionsAvailable: null,
    transcriptAvailable: null,
    platformCategory: null,
    platformRating: null,
    restrictedSignal: null,
  };
}
