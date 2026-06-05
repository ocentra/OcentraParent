import { mkdirSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));
const proofPath = join(repoRoot, 'test-results/browser-ai-provider-fallback-proof/proof.json');
const outputProofPath = join(
  repoRoot,
  'output/browser-plan-proof/ai-24-provider-degraded-fallback-behavior/11-provider-fallback-proof.json'
);

const sourceFiles = ['packages/activity-domain/src/browser-ai-provider-fallback-schemas.ts'];
const builtFiles = ['packages/activity-domain/dist/browser-ai-provider-fallback-schemas.js'];

assertBuiltContractsAreFresh();

const { BrowserAiAnalysisSchemaVersion } = await import(
  pathToFileURL(join(repoRoot, 'packages/activity-domain/dist/browser-ai-analysis-schemas.js')).href
);
const {
  BrowserAiProviderFallbackDecisionSchema,
  BrowserAiProviderFallbackDecisionSchemaVersion,
  planBrowserAiProviderFallbackDecision,
} = await import(
  pathToFileURL(join(repoRoot, 'packages/activity-domain/dist/browser-ai-provider-fallback-schemas.js')).href
);
const { BrowserAiProviderRouteSchemaVersion, planBrowserAiLocalProviderRoute } = await import(
  pathToFileURL(join(repoRoot, 'packages/activity-domain/dist/browser-ai-provider-routing-schemas.js')).href
);
const { BrowserAiFamilyHubRouteSchemaVersion, planBrowserAiFamilyHubRoute } = await import(
  pathToFileURL(join(repoRoot, 'packages/activity-domain/dist/browser-ai-family-hub-routing-schemas.js')).href
);
const { BrowserAiRemoteBoundarySchemaVersion, planBrowserAiRemoteRoute } = await import(
  pathToFileURL(join(repoRoot, 'packages/activity-domain/dist/browser-ai-remote-boundary-schemas.js')).href
);

const localDecision = buildLocalDecision();
const familyDecision = buildFamilyDecision();
const remoteDecision = buildRemoteDecision();
const metadataDecision = buildMetadataOnlyDecision();
const noAiDecision = buildNoAiDecision();
const decisions = [localDecision, familyDecision, remoteDecision, metadataDecision, noAiDecision];
const negativeChecks = runNegativeChecks({ remoteDecision, familyDecision });

const proof = {
  proofName: 'browser-ai-provider-fallback-proof',
  generatedAt: new Date().toISOString(),
  generatedOrFixturePageUsed: false,
  realProviderCalled: false,
  modelExecuted: false,
  remoteProviderCalled: false,
  policyEvaluated: false,
  uiDelivered: false,
  enforcementExecuted: false,
  routePlannerSource: 'built activity-domain contracts',
  decisionCount: decisions.length,
  decisions: decisions.map(decisionSummary),
  negativeChecks,
  noClaimChecks: {
    aiAnalysisResultClaimed: false,
    policyDecisionClaimed: false,
    runtimeDeliveryClaimed: false,
    remoteDefaultBlockingClaimed: false,
    localSafetyDisabledByRemoteOutage: false,
    rawBrowserStatePersisted: false,
    rawPageBodyPersisted: false,
    transcriptTextPersisted: false,
    screenshotPersisted: false,
    uiClaimed: false,
    enforcementClaimed: false,
  },
};

writeJson(proofPath, proof);
writeJson(outputProofPath, proof);

console.log('browser-ai-provider-fallback-proof-ok=true');
console.log(`proof=${proofPath}`);
console.log(`outputProof=${outputProofPath}`);
console.log(`decisionCount=${proof.decisionCount}`);
console.log(`selectedProviders=${decisions.map((decision) => decision.selectedProviderKind).join(',')}`);

function buildLocalDecision() {
  return planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-local-selected',
    decidedAt: '2026-06-05T03:30:00.000Z',
    localProviderRoute: selectedLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: null,
    metadataAvailable: true,
  });
}

function buildFamilyDecision() {
  const familyHubRoute = selectedFamilyHubRoute();
  return planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-family-selected',
    decidedAt: '2026-06-05T03:31:00.000Z',
    localProviderRoute: familyHubRoute.sourceLocalProviderRoute,
    familyHubRoute,
    remoteRoute: null,
    metadataAvailable: true,
  });
}

function buildRemoteDecision() {
  return planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-remote-selected',
    decidedAt: '2026-06-05T03:32:00.000Z',
    localProviderRoute: missingLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: selectedRemoteRoute(),
    metadataAvailable: true,
  });
}

function buildMetadataOnlyDecision() {
  return planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-metadata-only',
    decidedAt: '2026-06-05T03:33:00.000Z',
    localProviderRoute: missingLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: null,
    metadataAvailable: true,
  });
}

function buildNoAiDecision() {
  return planBrowserAiProviderFallbackDecision({
    fallbackDecisionId: 'fallback-no-ai',
    decidedAt: '2026-06-05T03:34:00.000Z',
    localProviderRoute: unavailableLocalProviderRoute(),
    familyHubRoute: null,
    remoteRoute: null,
    metadataAvailable: false,
  });
}

function runNegativeChecks({ remoteDecision, familyDecision }) {
  assertRejected('hidden-parent-fallback', { ...remoteDecision, parentFallbackVisible: false });
  assertRejected('hidden-child-fallback', { ...remoteDecision, childFallbackVisible: false });
  assertRejected('analysis-result-claim', { ...remoteDecision, analysisResultClaimed: true });
  assertRejected('policy-decision-claim', { ...remoteDecision, policyDecisionClaimed: true });
  assertRejected('local-safety-disabled', { ...remoteDecision, localSafetyPreserved: false });
  assertRejected('remote-default-blocking', { ...remoteDecision, remoteDefaultForBlocking: true });
  assertRejected('remote-outage-disables-local-safety', {
    ...remoteDecision,
    remoteOutageDisablesLocalSafety: true,
  });
  assertRejected('remote-selected-while-local-selected', {
    ...remoteDecision,
    localProviderRoute: selectedLocalProviderRoute(),
  });
  assertRejected('remote-selected-while-family-selected', {
    ...remoteDecision,
    familyHubRoute: familyDecision.familyHubRoute,
  });
  assertRejected('route-request-mismatch', {
    ...remoteDecision,
    remoteRoute: {
      ...remoteDecision.remoteRoute,
      requestId: 'browser-ai-analysis-request-other-video',
    },
  });

  return {
    hiddenParentFallbackRejected: true,
    hiddenChildFallbackRejected: true,
    aiAnalysisResultClaimRejected: true,
    policyDecisionClaimRejected: true,
    localSafetyDisabledRejected: true,
    remoteDefaultBlockingRejected: true,
    remoteOutageDisablesLocalSafetyRejected: true,
    remoteSelectedWhileLocalSelectedRejected: true,
    remoteSelectedWhileFamilySelectedRejected: true,
    routeRequestMismatchRejected: true,
  };
}

function assertRejected(label, value) {
  const parsed = BrowserAiProviderFallbackDecisionSchema.safeParse(value);
  if (parsed.success) {
    throw new Error(`Expected provider fallback negative check to reject ${label}`);
  }
}

function selectedFamilyHubRoute() {
  return planBrowserAiFamilyHubRoute({
    routeId: 'browser-ai-family-hub-route-youtube-video',
    routedAt: '2026-06-05T03:31:00.000Z',
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
    routedAt: '2026-06-05T03:32:00.000Z',
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
    routedAt: '2026-06-05T03:30:00.000Z',
    input: aiAnalysisInput('local-preferred'),
    capability: localProviderCapability(),
    auditEvidenceIds: ['browser-evidence-youtube-video'],
  });
}

function missingLocalProviderRoute() {
  return planBrowserAiLocalProviderRoute({
    routeId: 'browser-ai-local-provider-route-missing',
    routedAt: '2026-06-05T03:30:00.000Z',
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
    routedAt: '2026-06-05T03:30:00.000Z',
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
    checkedAt: '2026-06-05T03:30:00.000Z',
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
    checkedAt: '2026-06-05T03:31:00.000Z',
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
    checkedAt: '2026-06-05T03:32:00.000Z',
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
    approvedAt: '2026-06-05T03:32:00.000Z',
    approvedByParentRef: 'parent-admin-ref',
    providerId: 'parent-approved-remote-provider',
    allowedTasks: ['video-safety', 'url-safety'],
    allowedDataScopes: ['url-shape', 'metadata-summary', 'memory-refs', 'parent-rule-refs', 'schedule-refs'],
    retentionMode: 'no-retention',
    expiresAt: '2026-06-06T03:32:00.000Z',
    parentCanRevoke: true,
    rawBrowserStateAllowed: false,
    rawPageBodyAllowed: false,
    transcriptTextAllowed: false,
    screenshotAllowed: false,
  };
}

function aiAnalysisInput(modelRuntimePreference) {
  return {
    schemaVersion: BrowserAiAnalysisSchemaVersion,
    requestId: 'browser-ai-analysis-request-youtube-video',
    requestedAt: '2026-06-05T03:29:00.000Z',
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

function decisionSummary(decision) {
  return {
    fallbackDecisionId: decision.fallbackDecisionId,
    requestId: decision.requestId,
    selectedProviderKind: decision.selectedProviderKind,
    selectedRuntimeRef: decision.selectedRuntimeRef,
    localExecutionState: decision.localProviderRoute.executionState,
    familyExecutionState: decision.familyHubRoute?.executionState ?? null,
    remoteExecutionState: decision.remoteRoute?.executionState ?? null,
    fallbackAction: decision.fallbackAction,
    fallbackReasons: decision.fallbackReasons,
    auditEvidenceIds: decision.auditEvidenceIds,
    parentFallbackVisible: decision.parentFallbackVisible,
    childFallbackVisible: decision.childFallbackVisible,
    analysisResultClaimed: decision.analysisResultClaimed,
    policyDecisionClaimed: decision.policyDecisionClaimed,
    localSafetyPreserved: decision.localSafetyPreserved,
    remoteDefaultForBlocking: decision.remoteDefaultForBlocking,
    remoteOutageDisablesLocalSafety: decision.remoteOutageDisablesLocalSafety,
  };
}

function assertBuiltContractsAreFresh() {
  const newestSourceMtime = Math.max(...sourceFiles.map((file) => statSync(join(repoRoot, file)).mtimeMs));
  for (const builtFile of builtFiles) {
    const builtPath = join(repoRoot, builtFile);
    const builtMtime = statSync(builtPath).mtimeMs;
    if (builtMtime < newestSourceMtime) {
      throw new Error(`Build output is stale: ${builtFile}. Run cmd /c npm run build:contracts first.`);
    }
  }
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
