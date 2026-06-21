import { describe, expect, it } from 'vitest';
import { BrowserAiAnalysisSchemaVersion } from '@ocentra-parent/schema-domain/browser-ai-analysis-schemas';
import {
  BrowserAiRemoteApprovalSchema,
  BrowserAiRemoteBoundarySchemaVersion,
  BrowserAiRemoteCapabilitySchema,
  BrowserAiRemoteRouteSchema,
  planBrowserAiRemoteRoute,
} from '@ocentra-parent/schema-domain/browser-ai-remote-boundary-schemas';

describe('browser AI parent-approved remote approval contract', () => {
  it('accepts parent-owned no-retention structured-scope approval', () => {
    const parsed = BrowserAiRemoteApprovalSchema.safeParse(remoteApproval());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.parentCanRevoke).toBe(true);
      expect(parsed.data.retentionMode).toBe('no-retention');
      expect(parsed.data.rawBrowserStateAllowed).toBe(false);
    }
  });

  it('rejects remote approval for raw browser state, page body, transcript text, screenshots, or retention', () => {
    const parsed = BrowserAiRemoteApprovalSchema.safeParse({
      ...remoteApproval(),
      retentionMode: 'manual-required',
      rawBrowserStateAllowed: true,
      rawPageBodyAllowed: true,
      transcriptTextAllowed: true,
      screenshotAllowed: true,
    });

    expect(parsed.success).toBe(false);
  });
});

describe('browser AI parent-approved remote capability contract', () => {
  it('accepts available remote capability only with approval, visible no-retention, and runtime ref', () => {
    const parsed = BrowserAiRemoteCapabilitySchema.safeParse(remoteCapability());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.approval?.approvalId).toBe('parent-approved-remote-ai-browser-safety');
      expect(parsed.data.noRetentionVisible).toBe(true);
    }
  });

  it('rejects available remote capability without approval visibility or no-retention', () => {
    const parsed = BrowserAiRemoteCapabilitySchema.safeParse({
      ...remoteCapability(),
      approval: null,
      dataScopeVisible: false,
      retentionMode: 'manual-required',
    });

    expect(parsed.success).toBe(false);
  });
});

describe('browser AI parent-approved remote route planner', () => {
  it('selects remote route only with explicit approval, remote preference, and local safety fallback', () => {
    const route = planBrowserAiRemoteRoute(remoteRouteRequest());

    expect(route.executionState).toBe('selected');
    expect(route.selectedRuntimeRef).toBe('remote-runtime-ref-browser-ai');
    expect(route.parentExplicitRemoteApproval).toBe(true);
    expect(route.localSafetyFallbackAvailable).toBe(true);
  });

  it('returns manual-required when parent approval is missing', () => {
    const route = planBrowserAiRemoteRoute({
      ...remoteRouteRequest(),
      parentExplicitRemoteApproval: false,
    });

    expect(route.executionState).toBe('manual-required');
    expect(route.degradedStates).toEqual(['parent-approval-missing']);
    expect(route.selectedRuntimeRef).toBeNull();
  });

  it('rejects routes without local safety fallback or with unsafe remote authority claims', () => {
    const route = planBrowserAiRemoteRoute(remoteRouteRequest());
    const parsed = BrowserAiRemoteRouteSchema.safeParse({
      ...route,
      localSafetyFallbackAvailable: false,
      remoteDefaultForBlocking: true,
      remoteCanOverrideStricterLocalRules: true,
      remoteOutageDisablesLocalSafety: true,
    });

    expect(parsed.success).toBe(false);
  });
});

function remoteRouteRequest() {
  return {
    routeId: 'browser-ai-remote-route-youtube-video',
    routedAt: '2026-06-03T00:12:01.000Z',
    input: aiAnalysisInput(),
    capability: remoteCapability(),
    parentExplicitRemoteApproval: true,
    localSafetyFallbackAvailable: true,
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  };
}

function remoteCapability() {
  return {
    schemaVersion: BrowserAiRemoteBoundarySchemaVersion,
    providerId: 'parent-approved-remote-provider',
    checkedAt: '2026-06-03T00:12:00.000Z',
    capabilityState: 'available',
    supportedTasks: ['video-safety', 'url-safety', 'educational-relevance'],
    modelRuntimeRef: 'remote-runtime-ref-browser-ai',
    approval: remoteApproval(),
    retentionMode: 'no-retention',
    allowedDataScopes: ['url-shape', 'metadata-summary', 'memory-refs', 'parent-rule-refs', 'schedule-refs'],
    dataScopeVisible: true,
    retentionVisible: true,
    providerVisible: true,
    noRetentionVisible: true,
    degradedStates: [],
    unavailableReason: null,
  };
}

function remoteApproval() {
  return {
    schemaVersion: BrowserAiRemoteBoundarySchemaVersion,
    approvalId: 'parent-approved-remote-ai-browser-safety',
    approvedAt: '2026-06-03T00:12:00.000Z',
    approvedByParentRef: 'parent-admin-ref',
    providerId: 'parent-approved-remote-provider',
    allowedTasks: ['video-safety', 'url-safety'],
    allowedDataScopes: ['url-shape', 'metadata-summary', 'memory-refs', 'parent-rule-refs', 'schedule-refs'],
    retentionMode: 'no-retention',
    expiresAt: '2026-06-04T00:12:00.000Z',
    parentCanRevoke: true,
    rawBrowserStateAllowed: false,
    rawPageBodyAllowed: false,
    transcriptTextAllowed: false,
    screenshotAllowed: false,
  };
}

function aiAnalysisInput() {
  return {
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    requestId: 'browser-ai-analysis-request-youtube-video',
    requestedAt: '2026-06-03T00:09:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    policyVersionRef: 'browser-policy-version-2026-06-03',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    urlShapeClassificationId: 'url-shape-2026-06-03-youtube-video',
    metadataEvidenceIds: ['metadata-evidence-youtube-video'],
    memoryHitIds: ['memory-hit-known-education-video'],
    graphRefs: ['knowledge-graph-node-fractions'],
    parentRuleRefs: ['parent-rule-homework-window'],
    scheduleContextRefs: ['schedule-context-school-night'],
    normalizedUrl: 'https://www.youtube.com/watch?v=abc123',
    normalizedDomain: 'youtube.com',
    platform: 'youtube',
    platformIds: {
      videoId: 'abc123',
      channelId: 'channel-abc123',
      playlistId: null,
      postId: null,
      query: null,
    },
    title: 'Example math lesson',
    description: 'A short fractions lesson for middle school.',
    transcriptRefs: ['transcript-summary-ref-abc123'],
    thumbnailRefs: ['thumbnail-hash-ref-abc123'],
    screenEvidenceRefs: ['screen-evidence-youtube-video'],
    requestedTask: 'video-safety',
    modelRuntimePreference: 'parent-approved-remote',
    promptTemplate: {
      promptTemplateId: 'browser-ai-video-safety-template',
      promptTemplateVersion: 'browser-ai-video-safety-template-v1',
      requestedTask: 'video-safety',
      allowedInputFieldRefs: ['url-shape', 'metadata-evidence', 'memory-hit', 'parent-rule', 'schedule-context'],
      rawPromptTextIncluded: false,
      capturesRawPageBody: false,
      capturesTranscriptText: false,
    },
    custodyLabel: 'child-device-local',
    rawBrowserStateIncluded: false,
    devToolsPayloadIncluded: false,
    sqlitePathIncluded: false,
    journalPathIncluded: false,
    osStateIncluded: false,
  };
}
