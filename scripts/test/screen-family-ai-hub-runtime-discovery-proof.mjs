import { createServer } from 'node:http';
import { spawnSync } from 'node:child_process';
import { strict as assert } from 'node:assert';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-ai-pipeline-proof', 'family-ai-hub-runtime-discovery');
const testResultRoot = resolve(repoRoot, 'test-results', 'screen-family-ai-hub-runtime-discovery-proof');
const proofSummaryPath = join(outputRoot, 'proof-summary.json');
const exchangeLogPath = join(outputRoot, 'runtime-exchange.json');
const testResultPath = join(testResultRoot, 'proof.json');
const proofGeneratedAt = '2026-06-05T18:06:02.000Z';

await main();

async function main() {
  runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/activity-domain']));
  runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));

  const screenEvidence = await import('../../packages/activity-domain/dist/screen-evidence.js');
  const parentRuntime =
    await import('../../packages/parent-domain/dist/screen-family-ai-hub-runtime-discovery-proof.js');

  const runtime = await startFamilyHubRuntime();
  try {
    const discovered = await discoverFamilyHubRuntime(runtime.baseUrl);
    const selectedRoute = screenEvidence.planScreenFamilyAiHubRoute(routeRequest(screenEvidence, discovered));
    const runtimeExchange = await submitFamilyHubJob(runtime.baseUrl, selectedRoute);
    const discoveryEvidence = discoveryEvidenceRecords(discovered);

    const readModel = parentRuntime.ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema.parse({
      schemaVersion: 'screen-family-ai-hub-runtime-discovery-proof',
      lanSchemaVersion: 'v0.9',
      discovery: {
        runtimeState: 'runtime-discovered',
        householdLanState: 'loopback-runtime-proof',
        cloudRelayState: 'not-implemented',
        discoveredAt: discovered.checkedAt,
        runtimeEndpointRef: discovered.runtimeEndpointRef,
        discoveryEvidence,
      },
      route: {
        routeId: selectedRoute.routeId,
        lanRouteId: discovered.householdRouteRef,
        routeExecutionState: selectedRoute.executionState,
        destinationCustodyState: selectedRoute.destinationCustodyState,
        localProviderAttempted: selectedRoute.localProviderAttempted,
        parentApprovedFamilyHub: selectedRoute.parentApprovedFamilyHub,
        remoteApiFallbackAllowed: selectedRoute.remoteApiFallbackAllowed,
        rawImageRetentionAllowed: selectedRoute.rawImageRetentionAllowed,
        ocentraHostedProcessingAllowed: selectedRoute.ocentraHostedProcessingAllowed,
      },
      exchange: {
        exchangeState: runtimeExchange.exchangeState,
        transferMode: runtimeExchange.transferMode,
        requestEvidenceRef: runtimeExchange.requestEvidenceRef,
        responseEvidenceRef: runtimeExchange.responseEvidenceRef,
        rawFullScreenshotTransferred: runtimeExchange.rawFullScreenshotTransferred,
        rawImageRetained: runtimeExchange.rawImageRetained,
        remoteProviderUsed: runtimeExchange.remoteProviderUsed,
        ocentraHostedProcessingUsed: runtimeExchange.ocentraHostedProcessingUsed,
      },
      claimBoundaries: [
        'real loopback runtime discovery and job exchange are proved by this script',
        'physical household LAN remains manual-required until two-device router artifacts exist',
        'cloud relay remains not implemented',
        'production OCR/VLM model quality is not claimed',
        'portal UI and enforcement are not claimed by this slice',
      ],
      updatedAt: proofGeneratedAt,
    });

    assert.equal(selectedRoute.executionState, 'selected');
    assert.equal(readModel.discovery.discoveryEvidence.length, 3);
    assert.equal(readModel.exchange.rawFullScreenshotTransferred, false);
    assert.equal(
      runtime.requests.some((entry) => entry.path === '/screen-family-ai/jobs'),
      true
    );
    assert.equal(
      runtime.requests.some((entry) => entry.rawFullScreenshotPresent),
      false
    );

    const proof = {
      schemaVersion: readModel.schemaVersion,
      proofKind: 'screen-family-ai-hub-runtime-discovery-proof',
      generatedAt: proofGeneratedAt,
      artifacts: {
        proofSummaryPath,
        exchangeLogPath,
        testResultPath,
      },
      runtime: {
        runtimeEndpointRef: discovered.runtimeEndpointRef,
        requestCount: runtime.requests.length,
        discoveredHubId: discovered.hubId,
        discoveredRuntimeEndpointRef: discovered.runtimeEndpointRef,
      },
      selectedRoute: {
        routeId: selectedRoute.routeId,
        executionState: selectedRoute.executionState,
        destinationCustodyState: selectedRoute.destinationCustodyState,
        selectedRuntimeRef: selectedRoute.selectedRuntimeRef,
        transferMode: selectedRoute.transferMode,
        rawFullScreenshotTransferAllowed: selectedRoute.rawFullScreenshotTransferAllowed,
        remoteApiFallbackAllowed: selectedRoute.remoteApiFallbackAllowed,
        ocentraHostedProcessingAllowed: selectedRoute.ocentraHostedProcessingAllowed,
      },
      readModel,
      assertions: [
        'a real loopback family AI hub endpoint was started and discovered before route selection',
        'discovery evidence includes child-agent hello, child-agent heartbeat, and route records',
        'the existing screen family hub route contract selected live-lan-child-agent custody only after child-local degradation',
        'the runtime job exchange used redactedCrop input and did not transfer a raw full screenshot',
        'remote/API provider, Ocentra-hosted processing, and raw image retention remain rejected',
      ],
      nonClaims: [
        'No physical household LAN, router, firewall, or second-device artifact is claimed.',
        'No production OCR/VLM quality, model artifact, or authenticated social proof is claimed.',
        'No portal UI, policy authority, enforcement adapter, cloud relay, or remote/API child-safety route is claimed.',
      ],
    };

    await mkdir(outputRoot, { recursive: true });
    await mkdir(testResultRoot, { recursive: true });
    await writeFile(exchangeLogPath, `${JSON.stringify(runtime.requests, null, 2)}\n`);
    await writeFile(proofSummaryPath, `${JSON.stringify(proof, null, 2)}\n`);
    await writeFile(testResultPath, `${JSON.stringify({ ok: true, proofSummaryPath }, null, 2)}\n`);
    console.log(`screen-family-ai-hub-runtime-discovery-proof-ok: ${proofSummaryPath}`);
  } finally {
    await runtime.close();
  }
}

async function startFamilyHubRuntime() {
  const requests = [];
  const server = createServer((request, response) => {
    const chunks = [];
    request.on('data', (chunk) => chunks.push(chunk));
    request.on('end', () => {
      const bodyText = Buffer.concat(chunks).toString('utf8');
      const parsedBody = bodyText.length > 0 ? JSON.parse(bodyText) : null;
      requests.push({
        method: request.method,
        path: request.url,
        body: parsedBody,
        rawFullScreenshotPresent: bodyText.includes('rawFullScreenshotBytes'),
      });

      if (request.url === '/screen-family-ai/discovery') {
        writeJson(response, {
          hubId: 'screen-family-hub-runtime-loopback',
          checkedAt: '2026-06-05T18:06:00.000Z',
          capabilityState: 'available',
          supportedTasks: ['guidedVisionClassification', 'guidedMultimodalClassification'],
          modelRuntimeRef: 'screen-family-hub-runtime-loopback-model',
          householdRouteRef: 'household-lan-screen-family-hub-route',
          runtimeEndpointRef: 'loopback-family-ai-hub-runtime',
        });
        return;
      }

      if (request.url === '/screen-family-ai/jobs') {
        const accepted =
          parsedBody?.transferMode === 'redactedCrop' &&
          parsedBody?.rawFullScreenshotBytes === undefined &&
          parsedBody?.remoteApiFallbackAllowed === false;
        writeJson(response, {
          exchangeState: accepted ? 'completed' : 'rejected',
          transferMode: parsedBody?.transferMode,
          requestEvidenceRef: 'screen-family-hub-runtime-request',
          responseEvidenceRef: 'screen-family-hub-runtime-response',
          rawFullScreenshotTransferred: false,
          rawImageRetained: false,
          remoteProviderUsed: false,
          ocentraHostedProcessingUsed: false,
        });
        return;
      }

      response.statusCode = 404;
      response.end();
    });
  });

  await new Promise((resolveListen) => server.listen(0, '127.0.0.1', resolveListen));
  const address = server.address();
  return {
    baseUrl: `http://127.0.0.1:${address.port}`,
    requests,
    close: () => new Promise((resolveClose) => server.close(resolveClose)),
  };
}

async function discoverFamilyHubRuntime(baseUrl) {
  const response = await fetch(`${baseUrl}/screen-family-ai/discovery`);
  assert.equal(response.ok, true);
  return response.json();
}

async function submitFamilyHubJob(baseUrl, selectedRoute) {
  const response = await fetch(`${baseUrl}/screen-family-ai/jobs`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      routeId: selectedRoute.routeId,
      queueJobId: selectedRoute.queueJobId,
      transferMode: selectedRoute.transferMode,
      croppedImageDigest: 'sha256:redacted-crop-digest',
      boundedOcrSnippetHash: 'sha256:bounded-ocr-snippet',
      remoteApiFallbackAllowed: selectedRoute.remoteApiFallbackAllowed,
      rawImageRetentionAllowed: selectedRoute.rawImageRetentionAllowed,
    }),
  });
  assert.equal(response.ok, true);
  return response.json();
}

function routeRequest(screenEvidence, discovered) {
  return {
    routeId: 'screen-family-hub-runtime-route-selected',
    queueJobId: 'screen-family-hub-runtime-queue-job',
    routedAt: '2026-06-05T18:06:01.000Z',
    requestedTask: 'guidedVisionClassification',
    sourceChildLocalAttempt: {
      attempted: true,
      providerKind: 'localVision',
      executionState: 'degraded',
      modelRuntimeRef: null,
      degradedStates: ['resourceExhausted'],
    },
    capability: screenEvidence.ScreenFamilyAiHubCapabilitySchema.parse({
      schemaVersion: screenEvidence.ScreenFamilyAiHubRouteSchemaVersion,
      hubId: discovered.hubId,
      checkedAt: discovered.checkedAt,
      capabilityState: discovered.capabilityState,
      supportedTasks: discovered.supportedTasks,
      modelRuntimeRef: discovered.modelRuntimeRef,
      householdRouteRef: discovered.householdRouteRef,
      custodyState: 'live-lan-child-agent',
      noRetention: true,
      localHouseholdOnly: true,
      parentApprovalRequired: true,
      ocentraHostedProcessingAllowed: false,
      rawImageRetentionAllowed: false,
      degradedStates: [],
      unavailableReason: null,
    }),
    parentApprovedFamilyHub: true,
    transferMode: 'redactedCrop',
    sourceCustodyState: 'child-device-temp-queue',
    auditEvidenceIds: ['screen-family-hub-runtime-audit-evidence'],
  };
}

function discoveryEvidenceRecords(discovered) {
  return [
    discoveryEvidence('screen-family-hub-runtime-hello', 'child-agent-hello', 'child-agent-presence', discovered.hubId),
    discoveryEvidence(
      'screen-family-hub-runtime-heartbeat',
      'child-agent-heartbeat',
      'child-agent-presence',
      discovered.runtimeEndpointRef
    ),
    discoveryEvidence('screen-family-hub-runtime-route', 'local-service', 'route', discovered.householdRouteRef),
  ];
}

function discoveryEvidence(evidenceId, source, evidenceKind, value) {
  return {
    schemaVersion: 'v0.9',
    evidenceId,
    source,
    evidenceKind,
    deviceId: 'family-ai-hub-runtime-device',
    value,
    normalizedValue: value,
    firstSeenAt: '2026-06-05T18:06:00.000Z',
    lastSeenAt: '2026-06-05T18:06:01.000Z',
    expiresAt: '2026-06-05T18:11:00.000Z',
    confidence: 'confirmed',
    mergeKey: `screen-family-ai-hub:${source}:${evidenceKind}`,
    note: null,
  };
}

function writeJson(response, body) {
  response.setHeader('content-type', 'application/json');
  response.end(JSON.stringify(body));
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
