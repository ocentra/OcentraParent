import { describe, expect, it } from 'vitest';
import { BrowserAiAnalysisSchemaVersion } from '@ocentra-parent/schema-domain/browser-ai-analysis-schemas';
import {
  BrowserAiProviderCapabilitySchema,
  BrowserAiProviderRouteSchema,
  BrowserAiProviderRouteSchemaVersion,
  planBrowserAiLocalProviderRoute,
} from '@ocentra-parent/schema-domain/browser-ai-provider-routing-schemas';

describe('browser AI local provider capability contract', () => {
  it('accepts a no-retention child-device local provider capability', () => {
    const parsed = BrowserAiProviderCapabilitySchema.safeParse(localProviderCapability());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.localOnly).toBe(true);
      expect(parsed.data.parentApprovedRemoteEnabled).toBe(false);
      expect(parsed.data.noRetention).toBe(true);
    }
  });

  it('rejects a local provider that enables remote behavior or retention', () => {
    const parsed = BrowserAiProviderCapabilitySchema.safeParse({
      ...localProviderCapability(),
      localOnly: false,
      parentApprovedRemoteEnabled: true,
      noRetention: false,
    });

    expect(parsed.success).toBe(false);
  });

  it('accepts unavailable local provider capability only with explicit degraded state and reason', () => {
    const parsed = BrowserAiProviderCapabilitySchema.safeParse(unavailableLocalProviderCapability());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.capabilityState).toBe('provider-unavailable');
      expect(parsed.data.modelRuntimeRef).toBeNull();
    }
  });
});

describe('browser AI local provider route planner', () => {
  it('selects local runtime for local-preferred requests when capability supports the task', () => {
    const route = planBrowserAiLocalProviderRoute(routeRequest());

    expect(route.executionState).toBe('selected');
    expect(route.providerKind).toBe('child-device-local-ai');
    expect(route.selectedRuntimeRef).toBe('local-model-runtime-ref-browser-ai');
    expect(route.remoteDefaultForBlocking).toBe(false);
  });

  it('returns manual-required without selecting remote when the local model is missing', () => {
    const route = planBrowserAiLocalProviderRoute({
      ...routeRequest(),
      capability: {
        ...unavailableLocalProviderCapability(),
        capabilityState: 'model-missing',
        degradedStates: ['model-missing'],
        unavailableReason: 'local-browser-ai-model-missing',
      },
    });

    expect(route.executionState).toBe('manual-required');
    expect(route.selectedRuntimeRef).toBeNull();
    expect(route.degradedStates).toEqual(['model-missing']);
    expect(route.parentExplicitRemoteApproval).toBe(false);
  });

  it('keeps unsupported tasks manual-required instead of routing remotely', () => {
    const route = planBrowserAiLocalProviderRoute({
      ...routeRequest(),
      input: {
        ...aiAnalysisInput(),
        requestedTask: 'parent-summary',
        promptTemplate: {
          ...aiAnalysisInput().promptTemplate,
          requestedTask: 'parent-summary',
        },
      },
    });

    expect(route.executionState).toBe('manual-required');
    expect(route.degradedStates).toEqual(['unsupported-task']);
    expect(route.selectedRuntimeRef).toBeNull();
  });

  it('rejects routes that hide custody/retention/provider visibility or claim unsafe remote authority', () => {
    const route = planBrowserAiLocalProviderRoute(routeRequest());
    const parsed = BrowserAiProviderRouteSchema.safeParse({
      ...route,
      dataScopeVisible: false,
      remoteDefaultForBlocking: true,
      remoteCanOverrideStricterLocalRules: true,
      remoteOutageDisablesLocalSafety: true,
    });

    expect(parsed.success).toBe(false);
  });
});

function routeRequest() {
  return {
    routeId: 'browser-ai-local-provider-route-youtube-video',
    routedAt: '2026-06-03T00:10:01.000Z',
    input: aiAnalysisInput(),
    capability: localProviderCapability(),
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  };
}

function localProviderCapability() {
  return {
    schemaVersion: BrowserAiProviderRouteSchemaVersion,
    providerId: 'child-device-local-browser-ai',
    checkedAt: '2026-06-03T00:10:00.000Z',
    providerKind: 'child-device-local-ai',
    capabilityState: 'available',
    supportedTasks: ['video-safety', 'url-safety', 'educational-relevance'],
    modelRuntimeRef: 'local-model-runtime-ref-browser-ai',
    custodyLabel: 'child-device-local',
    noRetention: true,
    localOnly: true,
    parentApprovedRemoteEnabled: false,
    canRunOnChildDevice: true,
    degradedStates: [],
    unavailableReason: null,
  };
}

function unavailableLocalProviderCapability() {
  return {
    ...localProviderCapability(),
    capabilityState: 'provider-unavailable',
    modelRuntimeRef: null,
    degradedStates: ['provider-unavailable'],
    unavailableReason: 'local-browser-ai-provider-unavailable',
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
    modelRuntimePreference: 'local-preferred',
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
