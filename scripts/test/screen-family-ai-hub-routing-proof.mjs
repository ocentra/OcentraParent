import { spawnSync } from 'node:child_process';
import { strict as assert } from 'node:assert';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', '37-family-ai-hub-screen-analysis-queue');
const artifactSummaryPath = join(outputRoot, 'proof-summary.json');

await main();

async function main() {
  runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain']);

  const screenEvidence = await import('../../packages/activity-domain/dist/screen-evidence.js');
  const selectedRoute = screenEvidence.planScreenFamilyAiHubRoute(routeRequest(screenEvidence));
  const parentDisabledRoute = screenEvidence.planScreenFamilyAiHubRoute({
    ...routeRequest(screenEvidence),
    routeId: 'screen-family-hub-proof-parent-disabled',
    parentApprovedFamilyHub: false,
  });
  const unavailableRoute = screenEvidence.planScreenFamilyAiHubRoute({
    ...routeRequest(screenEvidence),
    routeId: 'screen-family-hub-proof-unavailable',
    capability: unavailableCapability(screenEvidence),
  });
  const localAlreadySelectedRoute = screenEvidence.planScreenFamilyAiHubRoute({
    ...routeRequest(screenEvidence),
    routeId: 'screen-family-hub-proof-local-already-selected',
    sourceChildLocalAttempt: {
      attempted: true,
      providerKind: 'localVision',
      executionState: 'selected',
      modelRuntimeRef: 'screen-child-local-vision-runtime',
      degradedStates: [],
    },
  });

  const invalidRows = {
    rawFullScreenshotTransferRejected: !screenEvidence.ScreenFamilyAiHubRouteSchema.safeParse({
      ...selectedRoute,
      rawFullScreenshotTransferAllowed: true,
    }).success,
    rawRetentionRejected: !screenEvidence.ScreenFamilyAiHubRouteSchema.safeParse({
      ...selectedRoute,
      rawImageRetentionAllowed: true,
    }).success,
    remoteFallbackRejected: !screenEvidence.ScreenFamilyAiHubRouteSchema.safeParse({
      ...selectedRoute,
      remoteApiFallbackAllowed: true,
    }).success,
    ocentraHostedProcessingRejected: !screenEvidence.ScreenFamilyAiHubRouteSchema.safeParse({
      ...selectedRoute,
      ocentraHostedProcessingAllowed: true,
    }).success,
  };

  assert.equal(selectedRoute.executionState, 'selected');
  assert.equal(selectedRoute.destinationCustodyState, 'live-lan-child-agent');
  assert.equal(selectedRoute.remoteProviderSelected, false);
  assert.equal(parentDisabledRoute.degradedStates[0], 'parentDisabled');
  assert.equal(unavailableRoute.executionState, 'unavailable');
  assert.equal(localAlreadySelectedRoute.degradedStates[0], 'childLocalAlreadySelected');
  assert.equal(Object.values(invalidRows).every(Boolean), true);

  const proof = {
    schemaVersion: screenEvidence.ScreenFamilyAiHubRouteSchemaVersion,
    proofKind: 'screen-family-ai-hub-routing-proof',
    generatedAt: new Date().toISOString(),
    artifact: artifactSummaryPath,
    routes: {
      selected: routeSummary(selectedRoute),
      parentDisabled: routeSummary(parentDisabledRoute),
      unavailable: routeSummary(unavailableRoute),
      childLocalAlreadySelected: routeSummary(localAlreadySelectedRoute),
    },
    invalidRows,
    assertions: [
      'child-device local screen analysis is attempted before a family AI hub route can be selected',
      'selected family hub routing stays inside the local household LAN custody boundary',
      'selected family hub routing requires parent approval, no retention, and redacted/cropped input',
      'unavailable hub routing degrades to explicit unavailable/manual-required state',
      'raw full screenshot transfer, raw retention, remote/API fallback, and Ocentra-hosted processing are rejected',
    ],
    nonClaims: [
      'No real LAN family hub runtime or discovery protocol is implemented in this proof.',
      'No production OCR/VLM model quality is claimed.',
      'No remote/API child-safety route, policy decision, portal UI, or enforcement adapter is claimed.',
    ],
  };

  await mkdir(outputRoot, { recursive: true });
  await writeFile(artifactSummaryPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`screen-family-ai-hub-routing-proof-ok: ${artifactSummaryPath}`);
}

function routeRequest(screenEvidence) {
  return {
    routeId: 'screen-family-hub-proof-selected',
    queueJobId: 'screen-queue-family-hub-proof',
    routedAt: '2026-06-05T02:30:00.000Z',
    requestedTask: 'guidedVisionClassification',
    sourceChildLocalAttempt: {
      attempted: true,
      providerKind: 'localVision',
      executionState: 'degraded',
      modelRuntimeRef: null,
      degradedStates: ['resourceExhausted'],
    },
    capability: availableCapability(screenEvidence),
    parentApprovedFamilyHub: true,
    transferMode: 'redactedCrop',
    sourceCustodyState: 'child-device-temp-queue',
    auditEvidenceIds: ['screen-family-hub-proof-audit-evidence'],
  };
}

function availableCapability(screenEvidence) {
  return screenEvidence.ScreenFamilyAiHubCapabilitySchema.parse({
    schemaVersion: screenEvidence.ScreenFamilyAiHubRouteSchemaVersion,
    hubId: 'screen-family-hub-proof',
    checkedAt: '2026-06-05T02:29:00.000Z',
    capabilityState: 'available',
    supportedTasks: ['guidedVisionClassification', 'guidedMultimodalClassification'],
    modelRuntimeRef: 'screen-family-hub-proof-runtime',
    householdRouteRef: 'household-lan-screen-family-hub-route',
    custodyState: 'live-lan-child-agent',
    noRetention: true,
    localHouseholdOnly: true,
    parentApprovalRequired: true,
    ocentraHostedProcessingAllowed: false,
    rawImageRetentionAllowed: false,
    degradedStates: [],
    unavailableReason: null,
  });
}

function unavailableCapability(screenEvidence) {
  return screenEvidence.ScreenFamilyAiHubCapabilitySchema.parse({
    ...availableCapability(screenEvidence),
    capabilityState: 'hubUnavailable',
    modelRuntimeRef: null,
    householdRouteRef: null,
    degradedStates: ['hubUnavailable'],
    unavailableReason: 'household LAN family hub is offline',
  });
}

function routeSummary(route) {
  return {
    routeId: route.routeId,
    executionState: route.executionState,
    requestedTask: route.requestedTask,
    transferMode: route.transferMode,
    sourceChildLocalAttemptState: route.sourceChildLocalAttempt.executionState,
    destinationCustodyState: route.destinationCustodyState,
    selectedRuntimeRef: route.selectedRuntimeRef,
    degradedStates: route.degradedStates,
    parentApprovedFamilyHub: route.parentApprovedFamilyHub,
    rawFullScreenshotTransferAllowed: route.rawFullScreenshotTransferAllowed,
    remoteApiFallbackAllowed: route.remoteApiFallbackAllowed,
    ocentraHostedProcessingAllowed: route.ocentraHostedProcessingAllowed,
  };
}

function runCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed\n${result.stdout}\n${result.stderr}`);
  }
}
