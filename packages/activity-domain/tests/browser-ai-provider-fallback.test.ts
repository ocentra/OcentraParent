import { describe, expect, it } from 'vitest';
import { BrowserAiAnalysisSchemaVersion } from '../src/browser-ai-analysis-schemas';
import {
  BrowserAiFamilyHubRouteSchemaVersion,
  planBrowserAiFamilyHubRoute,
} from '../src/browser-ai-family-hub-routing-schemas';
import {
  BrowserAiProviderFallbackDecisionSchema,
  BrowserAiProviderFallbackDecisionSchemaVersion,
} from '../src/browser-ai-provider-fallback-schemas';
import {
  BrowserAiProviderRouteSchemaVersion,
  planBrowserAiLocalProviderRoute,
} from '../src/browser-ai-provider-routing-schemas';
import {
  BrowserAiRemoteBoundarySchemaVersion,
  planBrowserAiRemoteRoute,
} from '../src/browser-ai-remote-boundary-schemas';

describe('browser AI provider degraded fallback contract', () => {
  it('accepts a visible local provider selection without policy authority', expectLocalProviderFallback);
  it('accepts family hub selection only after the local route is degraded', expectFamilyHubFallback);
  it('accepts remote selection only with explicit parent approval and local safety fallback', expectRemoteFallback);
  it('accepts metadata-only and no-AI fallbacks without selecting a runtime', expectNoRuntimeFallbacks);
  it('rejects hidden fallback, authority claims, and unsafe remote fallback claims', expectFallbackAuthorityRejections);
  it('rejects selected provider decisions that do not match the selected route', expectRouteMismatchRejections);
});

function expectLocalProviderFallback() {
  const localRoute = selectedLocalProviderRoute();
  const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'child-device-local-ai',
      selectedRuntimeRef: localRoute.selectedRuntimeRef,
      fallbackAction: 'continue-selected-runtime',
      fallbackReasons: ['local-selected'],
      localProviderRoute: localRoute,
    })
  );

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.selectedProviderKind).toBe('child-device-local-ai');
    expect(parsed.data.analysisResultClaimed).toBe(false);
    expect(parsed.data.policyDecisionClaimed).toBe(false);
  }
}

function expectFamilyHubFallback() {
  const familyRoute = selectedFamilyHubRoute();
  const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'family-ai-hub',
      selectedRuntimeRef: familyRoute.selectedRuntimeRef,
      fallbackAction: 'continue-selected-runtime',
      fallbackReasons: ['model-missing', 'family-hub-selected'],
      localProviderRoute: familyRoute.sourceLocalProviderRoute,
      familyHubRoute: familyRoute,
    })
  );

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.localProviderRoute.executionState).toBe('manual-required');
    expect(parsed.data.familyHubRoute?.executionState).toBe('selected');
  }
}

function expectRemoteFallback() {
  const remoteRoute = selectedRemoteRoute();
  const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'parent-approved-remote-ai',
      selectedRuntimeRef: remoteRoute.selectedRuntimeRef,
      fallbackAction: 'continue-selected-runtime',
      fallbackReasons: ['provider-unavailable', 'remote-selected'],
      localProviderRoute: missingLocalProviderRoute(),
      remoteRoute,
    })
  );

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.remoteRoute?.parentExplicitRemoteApproval).toBe(true);
    expect(parsed.data.remoteRoute?.localSafetyFallbackAvailable).toBe(true);
  }
}

function expectNoRuntimeFallbacks() {
  const metadataOnly = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'metadata-only',
      selectedRuntimeRef: null,
      fallbackAction: 'metadata-only-review',
      fallbackReasons: ['metadata-only', 'metadata-degraded'],
      localProviderRoute: missingLocalProviderRoute(),
    })
  );
  const noAi = BrowserAiProviderFallbackDecisionSchema.safeParse(
    providerFallbackDecision({
      selectedProviderKind: 'no-ai',
      selectedRuntimeRef: null,
      fallbackAction: 'parent-review',
      fallbackReasons: ['provider-unavailable', 'parent-review'],
      localProviderRoute: unavailableLocalProviderRoute(),
    })
  );

  expect(metadataOnly.success).toBe(true);
  expect(noAi.success).toBe(true);
}

function expectFallbackAuthorityRejections() {
  for (const invalid of invalidFallbackAuthorityClaims()) {
    const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(invalid);

    expect(parsed.success).toBe(false);
  }
}

function expectRouteMismatchRejections() {
  const localRoute = selectedLocalProviderRoute();
  const familyRoute = selectedFamilyHubRoute();
  const inconsistentFamily = providerFallbackDecision({
    selectedProviderKind: 'family-ai-hub',
    selectedRuntimeRef: familyRoute.selectedRuntimeRef,
    fallbackAction: 'continue-selected-runtime',
    fallbackReasons: ['family-hub-selected'],
    localProviderRoute: localRoute,
    familyHubRoute: familyRoute,
  });
  const runtimeMismatch = providerFallbackDecision({
    selectedProviderKind: 'child-device-local-ai',
    selectedRuntimeRef: 'different-runtime-ref',
    fallbackAction: 'continue-selected-runtime',
    fallbackReasons: ['local-selected'],
    localProviderRoute: localRoute,
  });

  expect(BrowserAiProviderFallbackDecisionSchema.safeParse(inconsistentFamily).success).toBe(false);
  expect(BrowserAiProviderFallbackDecisionSchema.safeParse(runtimeMismatch).success).toBe(false);
}

function providerFallbackDecision(overrides = {}) {
  return {
    schemaVersion: BrowserAiProviderFallbackDecisionSchemaVersion,
    fallbackDecisionId: 'browser-ai-provider-fallback-decision-youtube-video',
    requestId: 'browser-ai-analysis-request-youtube-video',
    decidedAt: '2026-06-03T00:13:00.000Z',
    localProviderRoute: missingLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: null,
    selectedProviderKind: 'no-ai',
    selectedRuntimeRef: null,
    fallbackAction: 'parent-review',
    fallbackReasons: ['provider-unavailable', 'parent-review'],
    auditEvidenceIds: ['browser-evidence-youtube-video'],
    parentFallbackVisible: true,
    childFallbackVisible: true,
    analysisResultClaimed: false,
    policyDecisionClaimed: false,
    localSafetyPreserved: true,
    remoteDefaultForBlocking: false,
    remoteOutageDisablesLocalSafety: false,
    ...overrides,
  };
}

function invalidFallbackAuthorityClaims() {
  const valid = providerFallbackDecision({
    selectedProviderKind: 'metadata-only',
    selectedRuntimeRef: null,
    fallbackAction: 'metadata-only-review',
    fallbackReasons: ['metadata-only'],
    localProviderRoute: missingLocalProviderRoute(),
  });
  return [
    { ...valid, parentFallbackVisible: false },
    { ...valid, childFallbackVisible: false },
    { ...valid, analysisResultClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, localSafetyPreserved: false },
    { ...valid, remoteDefaultForBlocking: true },
    { ...valid, remoteOutageDisablesLocalSafety: true },
  ];
}

function selectedFamilyHubRoute() {
  return planBrowserAiFamilyHubRoute({
    routeId: 'browser-ai-family-hub-route-youtube-video',
    routedAt: '2026-06-03T00:11:01.000Z',
    input: aiAnalysisInput('local-preferred'),
    sourceLocalProviderRoute: missingLocalProviderRoute(),
    capability: familyHubCapability(),
    parentAllowedFamilyHub: true,
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  });
}

function selectedRemoteRoute() {
  return planBrowserAiRemoteRoute({
    routeId: 'browser-ai-remote-route-youtube-video',
    routedAt: '2026-06-03T00:12:01.000Z',
    input: aiAnalysisInput('parent-approved-remote'),
    capability: remoteCapability(),
    parentExplicitRemoteApproval: true,
    localSafetyFallbackAvailable: true,
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  });
}

function selectedLocalProviderRoute() {
  return planBrowserAiLocalProviderRoute({
    routeId: 'browser-ai-local-provider-route-selected',
    routedAt: '2026-06-03T00:10:01.000Z',
    input: aiAnalysisInput('local-preferred'),
    capability: localProviderCapability(),
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  });
}

function missingLocalProviderRoute() {
  return planBrowserAiLocalProviderRoute({
    routeId: 'browser-ai-local-provider-route-missing',
    routedAt: '2026-06-03T00:10:01.000Z',
    input: aiAnalysisInput('local-preferred'),
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

function unavailableLocalProviderRoute() {
  return planBrowserAiLocalProviderRoute({
    routeId: 'browser-ai-local-provider-route-unavailable',
    routedAt: '2026-06-03T00:10:01.000Z',
    input: aiAnalysisInput('local-preferred'),
    capability: {
      ...localProviderCapability(),
      capabilityState: 'provider-unavailable',
      modelRuntimeRef: null,
      degradedStates: ['provider-unavailable'],
      unavailableReason: 'local-browser-ai-provider-unavailable',
    },
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

function aiAnalysisInput(modelRuntimePreference: 'local-preferred' | 'parent-approved-remote') {
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
    platformIds: { videoId: 'abc123', channelId: 'channel-abc123', playlistId: null, postId: null, query: null },
    title: 'Example math lesson',
    description: 'A short fractions lesson for middle school.',
    transcriptRefs: ['transcript-summary-ref-abc123'],
    thumbnailRefs: ['thumbnail-hash-ref-abc123'],
    screenEvidenceRefs: ['screen-evidence-youtube-video'],
    requestedTask: 'video-safety',
    modelRuntimePreference,
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
