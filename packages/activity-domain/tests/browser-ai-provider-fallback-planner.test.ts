import { describe, expect, it } from 'vitest';
import { BrowserAiAnalysisSchemaVersion } from '../src/browser-ai-analysis-schemas';
import {
  BrowserAiFamilyHubRouteSchemaVersion,
  planBrowserAiFamilyHubRoute,
} from '../src/browser-ai-family-hub-routing-schemas';
import {
  BrowserAiProviderFallbackDecisionSchema,
  planBrowserAiProviderFallbackDecision,
} from '../src/browser-ai-provider-fallback-schemas';
import {
  BrowserAiProviderRouteSchemaVersion,
  planBrowserAiLocalProviderRoute,
} from '../src/browser-ai-provider-routing-schemas';
import {
  BrowserAiRemoteBoundarySchemaVersion,
  planBrowserAiRemoteRoute,
} from '../src/browser-ai-remote-boundary-schemas';

describe('browser AI provider fallback planner', () => {
  it('derives local, family, remote, metadata-only, and no-AI decisions from real route proofs', () =>
    assertProviderFallbackDecisionPlanning());

  it('rejects remote fallback when local or family route is already selected', () =>
    assertRemoteFallbackConflictRejection());
});

function assertProviderFallbackDecisionPlanning() {
  const localSelected = planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-local-selected',
    decidedAt: '2026-06-05T03:20:00.000Z',
    localProviderRoute: selectedLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: null,
    metadataAvailable: true,
  });
  const familyRoute = selectedFamilyHubRoute();
  const familySelected = planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-family-selected',
    decidedAt: '2026-06-05T03:21:00.000Z',
    localProviderRoute: familyRoute.sourceLocalProviderRoute,
    familyHubRoute: familyRoute,
    remoteRoute: null,
    metadataAvailable: true,
  });
  const remoteSelected = planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-remote-selected',
    decidedAt: '2026-06-05T03:22:00.000Z',
    localProviderRoute: missingLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: selectedRemoteRoute(),
    metadataAvailable: true,
  });
  const metadataOnly = planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-metadata-only',
    decidedAt: '2026-06-05T03:23:00.000Z',
    localProviderRoute: missingLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: null,
    metadataAvailable: true,
  });
  const noAi = planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-no-ai',
    decidedAt: '2026-06-05T03:24:00.000Z',
    localProviderRoute: unavailableLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: null,
    metadataAvailable: false,
  });

  expect(localSelected.selectedProviderKind).toBe('child-device-local-ai');
  expect(localSelected.selectedRuntimeRef).toBe('local-model-runtime-ref-browser-ai');
  expect(familySelected.selectedProviderKind).toBe('family-ai-hub');
  expect(familySelected.fallbackReasons).toEqual(['model-missing', 'family-hub-selected']);
  expect(remoteSelected.selectedProviderKind).toBe('parent-approved-remote-ai');
  expect(remoteSelected.fallbackReasons).toEqual(['model-missing', 'remote-selected']);
  expect(metadataOnly.selectedProviderKind).toBe('metadata-only');
  expect(metadataOnly.selectedRuntimeRef).toBeNull();
  expect(metadataOnly.fallbackAction).toBe('metadata-only-review');
  expect(noAi.selectedProviderKind).toBe('no-ai');
  expect(noAi.selectedRuntimeRef).toBeNull();
  expect(noAi.fallbackAction).toBe('parent-review');
  expect(noAi.fallbackReasons).toEqual(['provider-unavailable', 'parent-review']);
}

function assertRemoteFallbackConflictRejection() {
  const remoteRoute = selectedRemoteRoute();
  const validRemoteDecision = planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-remote-selected',
    decidedAt: '2026-06-05T03:25:00.000Z',
    localProviderRoute: missingLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute,
    metadataAvailable: true,
  });
  const localAndRemoteSelected = {
    ...validRemoteDecision,
    selectedProviderKind: 'parent-approved-remote-ai',
    selectedRuntimeRef: remoteRoute.selectedRuntimeRef,
    fallbackReasons: ['remote-selected'],
    localProviderRoute: selectedLocalProviderRoute(),
  };
  const familyAndRemoteSelected = {
    ...validRemoteDecision,
    fallbackDecisionId: 'fallback-remote-conflict-family',
    selectedProviderKind: 'parent-approved-remote-ai',
    selectedRuntimeRef: remoteRoute.selectedRuntimeRef,
    fallbackReasons: ['remote-selected'],
    familyHubRoute: selectedFamilyHubRoute(),
  };

  expect(BrowserAiProviderFallbackDecisionSchema.safeParse(localAndRemoteSelected).success).toBe(false);
  expect(BrowserAiProviderFallbackDecisionSchema.safeParse(familyAndRemoteSelected).success).toBe(false);
}

function selectedFamilyHubRoute() {
  return planBrowserAiFamilyHubRoute({
    routeId: 'browser-ai-family-hub-route-youtube-video',
    routedAt: '2026-06-05T03:21:00.000Z',
    input: aiAnalysisInput('local-preferred'),
    sourceLocalProviderRoute: missingLocalProviderRoute(),
    capability: familyHubCapability(),
    parentAllowedFamilyHub: true,
    auditEvidenceIds: ['browser-evidence-youtube-video', 'family-hub-route-proof'],
  });
}

function selectedRemoteRoute() {
  return planBrowserAiRemoteRoute({
    routeId: 'browser-ai-remote-route-youtube-video',
    routedAt: '2026-06-05T03:22:00.000Z',
    input: aiAnalysisInput('parent-approved-remote'),
    capability: remoteCapability(),
    parentExplicitRemoteApproval: true,
    localSafetyFallbackAvailable: true,
    auditEvidenceIds: ['browser-evidence-youtube-video', 'remote-approval-proof'],
  });
}

function selectedLocalProviderRoute() {
  return planBrowserAiLocalProviderRoute({
    routeId: 'browser-ai-local-provider-route-selected',
    routedAt: '2026-06-05T03:20:00.000Z',
    input: aiAnalysisInput('local-preferred'),
    capability: localProviderCapability(),
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  });
}

function missingLocalProviderRoute() {
  return planBrowserAiLocalProviderRoute({
    routeId: 'browser-ai-local-provider-route-missing',
    routedAt: '2026-06-05T03:20:00.000Z',
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
    routedAt: '2026-06-05T03:20:00.000Z',
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
    checkedAt: '2026-06-05T03:20:00.000Z',
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
    checkedAt: '2026-06-05T03:21:00.000Z',
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
    checkedAt: '2026-06-05T03:22:00.000Z',
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
    approvedAt: '2026-06-05T03:22:00.000Z',
    approvedByParentRef: 'parent-admin-ref',
    providerId: 'parent-approved-remote-provider',
    allowedTasks: ['video-safety', 'url-safety'],
    allowedDataScopes: ['url-shape', 'metadata-summary', 'memory-refs', 'parent-rule-refs', 'schedule-refs'],
    retentionMode: 'no-retention',
    expiresAt: '2026-06-06T03:22:00.000Z',
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
    requestedAt: '2026-06-05T03:19:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    policyVersionRef: 'browser-policy-version-2026-06-05',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    urlShapeClassificationId: 'url-shape-youtube-video',
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
