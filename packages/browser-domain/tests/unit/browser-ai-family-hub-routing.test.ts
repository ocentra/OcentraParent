import { describe, expect, it } from 'vitest';
import { BrowserAiAnalysisSchemaVersion } from '@ocentra-parent/schema-domain/browser-ai-analysis-schemas';
import {
  BrowserAiFamilyHubCapabilitySchema,
  BrowserAiFamilyHubRouteSchema,
  BrowserAiFamilyHubRouteSchemaVersion,
  planBrowserAiFamilyHubRoute,
} from '@ocentra-parent/schema-domain/browser-ai-family-hub-routing-schemas';
import {
  BrowserAiProviderRouteSchemaVersion,
  planBrowserAiLocalProviderRoute,
} from '@ocentra-parent/schema-domain/browser-ai-provider-routing-schemas';

describe('browser AI family hub capability contract', () => {
  it('accepts a no-retention local-household family hub capability', () => {
    const parsed = BrowserAiFamilyHubCapabilitySchema.safeParse(familyHubCapability());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.localHouseholdOnly).toBe(true);
      expect(parsed.data.parentRemoteApprovalRequired).toBe(false);
      expect(parsed.data.childDeviceCanRunModel).toBe(false);
    }
  });

  it('rejects a family hub that looks like remote AI, child-device execution, or retention', () => {
    const parsed = BrowserAiFamilyHubCapabilitySchema.safeParse({
      ...familyHubCapability(),
      noRetention: false,
      localHouseholdOnly: false,
      parentRemoteApprovalRequired: true,
      childDeviceCanRunModel: true,
    });

    expect(parsed.success).toBe(false);
  });
});

describe('browser AI family hub route planner', () => {
  it('selects family hub only after local provider cannot serve and parent allows household routing', () => {
    const route = planBrowserAiFamilyHubRoute(familyHubRouteRequest());

    expect(route.executionState).toBe('selected');
    expect(route.selectedRuntimeRef).toBe('family-hub-runtime-ref-browser-ai');
    expect(route.sourceLocalProviderRoute.executionState).toBe('manual-required');
    expect(route.remoteProviderSelected).toBe(false);
  });

  it('returns manual-required when local provider already served the request', () => {
    const route = planBrowserAiFamilyHubRoute({
      ...familyHubRouteRequest(),
      sourceLocalProviderRoute: selectedLocalProviderRoute(),
    });

    expect(route.executionState).toBe('manual-required');
    expect(route.degradedStates).toEqual(['local-provider-not-exhausted']);
    expect(route.selectedRuntimeRef).toBeNull();
  });

  it('returns manual-required when parent has not allowed family hub routing', () => {
    const route = planBrowserAiFamilyHubRoute({
      ...familyHubRouteRequest(),
      parentAllowedFamilyHub: false,
    });

    expect(route.executionState).toBe('manual-required');
    expect(route.degradedStates).toEqual(['parent-disabled']);
    expect(route.selectedRuntimeRef).toBeNull();
  });

  it('returns unavailable when household hub proof is missing', () => {
    const route = planBrowserAiFamilyHubRoute({
      ...familyHubRouteRequest(),
      capability: {
        ...familyHubCapability(),
        capabilityState: 'hub-unavailable',
        modelRuntimeRef: null,
        householdRouteRef: null,
        degradedStates: ['hub-unavailable'],
        unavailableReason: 'family-hub-not-discovered',
      },
    });

    expect(route.executionState).toBe('unavailable');
    expect(route.degradedStates).toEqual(['hub-unavailable']);
  });

  it('rejects routes that hide visibility or select remote/default blocking', () => {
    const route = planBrowserAiFamilyHubRoute(familyHubRouteRequest());
    const parsed = BrowserAiFamilyHubRouteSchema.safeParse({
      ...route,
      custodyVisible: false,
      remoteProviderSelected: true,
      remoteDefaultForBlocking: true,
    });

    expect(parsed.success).toBe(false);
  });
});

function familyHubRouteRequest() {
  return {
    routeId: 'browser-ai-family-hub-route-youtube-video',
    routedAt: '2026-06-03T00:11:01.000Z',
    input: aiAnalysisInput(),
    sourceLocalProviderRoute: missingLocalProviderRoute(),
    capability: familyHubCapability(),
    parentAllowedFamilyHub: true,
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  };
}

function familyHubCapability() {
  return {
    schemaVersion: BrowserAiFamilyHubRouteSchemaVersion,
    hubId: 'household-family-ai-hub',
    checkedAt: '2026-06-03T00:11:00.000Z',
    capabilityState: 'available',
    supportedTasks: ['video-safety', 'url-safety', 'educational-relevance'],
    modelRuntimeRef: 'family-hub-runtime-ref-browser-ai',
    householdRouteRef: 'household-lan-family-hub-route-proof',
    custodyLabel: 'local-network-child-agent',
    noRetention: true,
    localHouseholdOnly: true,
    parentRemoteApprovalRequired: false,
    childDeviceCanRunModel: false,
    degradedStates: [],
    unavailableReason: null,
  };
}

function missingLocalProviderRoute() {
  return planBrowserAiLocalProviderRoute({
    routeId: 'browser-ai-local-provider-route-unavailable',
    routedAt: '2026-06-03T00:10:01.000Z',
    input: aiAnalysisInput(),
    capability: {
      ...localProviderCapability(),
      capabilityState: 'model-missing',
      modelRuntimeRef: null,
      degradedStates: ['model-missing'],
      unavailableReason: 'local-browser-ai-model-missing',
    },
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  });
}

function selectedLocalProviderRoute() {
  return planBrowserAiLocalProviderRoute({
    routeId: 'browser-ai-local-provider-route-selected',
    routedAt: '2026-06-03T00:10:01.000Z',
    input: aiAnalysisInput(),
    capability: localProviderCapability(),
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  });
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
