import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'parent-mobile-controller-observer-handoff-proof');
const proofPath = join(outputDir, 'proof.json');
const serviceBridgeProofPath = join(repoRoot, 'test-results', 'parent-mobile-service-bridge-proof', 'proof.json');
const productionMobileProofPath = join(
  repoRoot,
  'test-results',
  'v0-9-production-lan-mobile-controller-proof',
  'proof.json'
);
const discoveryRuntimeProofPath = join(
  repoRoot,
  'test-results',
  'v0-9-mobile-controller-discovery-runtime-proof',
  'proof.json'
);
const providerSelectionProofPath = join(
  repoRoot,
  'test-results',
  'v0-9-prod-discovery-provider-selection-proof',
  'proof.json'
);
const checkpointPath = join(
  repoRoot,
  'docs',
  'checkpoints',
  'parent-mobile-controller-observer-handoff-proof-2026-05-30.md'
);
const proofCommand = 'node scripts/test/parent-mobile-controller-observer-handoff-proof.mjs';
const commands = [];
const proofLabels = [];

await main();
process.exit(0);

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runNpm(['run', 'build:contracts']);
  await runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/parent-mobile-controller-observer-handoff-runtime.test.ts',
  ]);
  await ensureProofArtifact(serviceBridgeProofPath, 'parent-mobile-service-bridge-proof', [
    'cmd',
    '/c',
    'node',
    'scripts/test/parent-mobile-service-bridge-proof.mjs',
  ]);
  await ensureProofArtifact(providerSelectionProofPath, 'v0-9-prod-discovery-provider-selection-proof', [
    'cmd',
    '/c',
    'node',
    'scripts/test/v0-9-prod-discovery-provider-selection-proof.mjs',
  ]);

  const serviceBridgeProof = await readJson(serviceBridgeProofPath);
  const productionMobileProof = await readJson(productionMobileProofPath);
  const discoveryRuntimeProof = await readJson(discoveryRuntimeProofPath);
  const providerSelectionProof = await readJson(providerSelectionProofPath);
  const runtimeReadModel = await parseRuntimeReadModel(
    buildRuntimeReadModel(serviceBridgeProof, providerSelectionProof, discoveryRuntimeProof)
  );

  assertServiceBridgeProof(serviceBridgeProof);
  assertProductionMobileProof(productionMobileProof);
  assertDiscoveryRuntimeProof(discoveryRuntimeProof);
  assertProviderSelectionProof(providerSelectionProof);
  assertRuntimeReadModel(runtimeReadModel);
  const scriptWiring = await assertScriptWiring();

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'parent-mobile-controller-observer-handoff-proof',
    commands,
    proofLabels,
    evidence: {
      serviceBridge: relativePath(serviceBridgeProofPath),
      productionMobileController: relativePath(productionMobileProofPath),
      discoveryRuntime: relativePath(discoveryRuntimeProofPath),
      providerSelection: relativePath(providerSelectionProofPath),
      contract: 'packages/parent-domain/src/parent-mobile-controller-observer-handoff-runtime.ts',
      contractTest: 'packages/parent-domain/tests/parent-mobile-controller-observer-handoff-runtime.test.ts',
      checkpoint: relativePath(checkpointPath),
      output: relativePath(proofPath),
    },
    runtimeReadModel,
    scriptWiring,
    claimsProved: [
      'Parent mobile observer handoff exposes controller lease visibility without granting mobile write authority.',
      'Controller takeover remains denied or manual-required until real mobile package and device authority proof exists.',
      'Selected route and provider handoff states stay degraded, unavailable, or manual-required instead of claiming provider readiness.',
      'Parent cache and parent-owned storage handoff states stay stale/offline instead of silently replacing LAN or relay routes.',
      'LAN AI handoff stays degraded or unavailable and never runs a phone-local model by default.',
      'Cloud relay, mobile parity, child mobile agent behavior, Android device-owner, iOS Family Controls, signing, stores, and entitlements remain explicit non-claims.',
    ],
    claimsNotProved: [
      'real Android or iOS parent mobile active-controller authority',
      'remote control through a mobile shell',
      'physical household LAN or provider readiness on two devices',
      'cloud relay routing, authentication, storage, or fallback behavior',
      'parent-owned storage sync or cache freshness',
      'phone-local model execution',
      'C-owned UI rendering or vendor visual changes',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`parent-mobile-controller-observer-handoff-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function buildRuntimeReadModel(serviceBridgeProof, providerSelectionProof, discoveryRuntimeProof) {
  const androidBridge = findBridgeModel(serviceBridgeProof, 'android');
  const iosBridge = findBridgeModel(serviceBridgeProof, 'ios');
  const providerReadModel = providerSelectionProof.providerSelectionReadModel;
  return {
    schemaVersion: 'parent-mobile-controller-observer-handoff-proof',
    proofHarness: {
      sourceProofs: [
        proofInput(
          'parent-mobile-service-bridge-proof',
          'test-results/parent-mobile-service-bridge-proof/proof.json',
          'node scripts/test/parent-mobile-service-bridge-proof.mjs'
        ),
        proofInput(
          'v0-9-production-lan-mobile-controller-proof',
          'test-results/v0-9-production-lan-mobile-controller-proof/proof.json',
          'node scripts/test/v0-9-production-lan-mobile-controller-proof.mjs'
        ),
        proofInput(
          'v0-9-mobile-controller-discovery-runtime-proof',
          'test-results/v0-9-mobile-controller-discovery-runtime-proof/proof.json',
          'node scripts/test/v0-9-mobile-controller-discovery-runtime-proof.mjs'
        ),
        proofInput(
          'v0-9-prod-discovery-provider-selection-proof',
          'test-results/v0-9-prod-discovery-provider-selection-proof/proof.json',
          'node scripts/test/v0-9-prod-discovery-provider-selection-proof.mjs'
        ),
      ],
      outputProofPath: 'test-results/parent-mobile-controller-observer-handoff-proof/proof.json',
      checkpointPath: 'docs/checkpoints/parent-mobile-controller-observer-handoff-proof-2026-05-30.md',
    },
    handoffReadModels: [
      buildHandoffReadModel('android', androidBridge, providerReadModel),
      buildHandoffReadModel('ios', iosBridge, providerReadModel),
    ],
    claimBoundaries: {
      parentMobileWriteAuthority: serviceBridgeProof.runtimeReadModel.claimBoundaries.parentMobileWriteAuthority,
      mobileParity: 'not claimed by controller-observer handoff proof',
      childMobileAgentBehavior: discoveryRuntimeProof.runtimeReadModel.claimBoundaries.mobileChildAgentBehavior,
      androidChildAgentBehavior: serviceBridgeProof.runtimeReadModel.claimBoundaries.androidChildAgent,
      iosChildAgentBehavior: serviceBridgeProof.runtimeReadModel.claimBoundaries.iosChildAgent,
      androidDeviceOwner: 'not claimed; Android device-owner behavior belongs to child-agent platform proof',
      iosFamilyControls: 'not claimed; iOS Family Controls requires entitlements and platform proof',
      signingStoresEntitlements: discoveryRuntimeProof.runtimeReadModel.claimBoundaries.storesSigningEntitlements,
      cloudRelay: serviceBridgeProof.runtimeReadModel.claimBoundaries.cloudRelay,
      cUiOwnership: 'C UI can render this contract later; this proof does not touch UI or vendor paths',
    },
    updatedAt: providerSelectionProof.checkedAt,
  };
}

function buildHandoffReadModel(platform, bridgeModel, providerReadModel) {
  const route = routeCandidate(platform, providerReadModel);
  return {
    platform,
    parentDeviceId: `parent-mobile-${platform}-handoff`,
    role: bridgeModel.role,
    leaseSnapshot: leaseSnapshot(platform, bridgeModel),
    routeSnapshot: routeSnapshot(platform, bridgeModel, route),
    lanAiHandoff: lanAiHandoff(bridgeModel),
    handoffSteps: handoffSteps(bridgeModel.aiSubmission.jobState),
  };
}

function leaseSnapshot(platform, bridgeModel) {
  if (platform === 'android') {
    return {
      leaseState: 'visible-read-only',
      controllerState: bridgeModel.controllerState,
      commandAuthorityState: bridgeModel.commandAuthorityState,
      controllerLeaseVisible: true,
      controllerDeviceId: 'parent-desktop-controller',
      handoffRequirement: 'lease visibility is read-only until mobile controller package authority is proven',
    };
  }

  return {
    leaseState: 'manual-required',
    controllerState: bridgeModel.controllerState,
    commandAuthorityState: bridgeModel.commandAuthorityState,
    controllerLeaseVisible: false,
    controllerDeviceId: null,
    handoffRequirement: 'iOS controller handoff requires signed package entitlement and device proof',
  };
}

function routeSnapshot(platform, bridgeModel, route) {
  return {
    routeState: platform === 'android' ? 'selected-route-degraded' : 'provider-unavailable',
    selectedRouteId: platform === 'android' ? route.routeId : null,
    discoveryState: route.discoveryState,
    reachability: route.reachability,
    providerLifecycleState: route.lifecycleState,
    providerPolicyDecision: route.policyDecision,
    providerId: platform === 'android' ? route.providerPeerId : null,
    localServiceState: connectionState(bridgeModel, 'local-service'),
    lanServiceState: connectionState(bridgeModel, 'lan-service'),
    cloudRelayState: connectionState(bridgeModel, 'cloud-relay'),
    parentCacheState: connectionState(bridgeModel, 'parent-cache'),
    parentOwnedStorageState: connectionState(bridgeModel, 'parent-owned-storage'),
    routeRequirement: `${route.lifecycleState} provider route must stay explicit and must not silently fall back to cloud relay`,
  };
}

function lanAiHandoff(bridgeModel) {
  return {
    jobState: bridgeModel.aiSubmission.jobState,
    routeState: bridgeModel.aiSubmission.jobState === 'degraded' ? 'selected-route-degraded' : 'provider-unavailable',
    providerId: null,
    unavailableReason: bridgeModel.aiSubmission.unavailableReason,
    localModelExecutionState: bridgeModel.aiSubmission.localModelExecutionState,
    localModelExecutionAllowed: false,
    evidenceReferenceIds: bridgeModel.aiSubmission.evidenceReferenceIds,
  };
}

function handoffSteps(aiState) {
  return [
    step('observe-controller-lease', 'completed', 'observed-read-only', 'parent-mobile-shell', 'observer-read-only'),
    step('observe-selected-route', 'completed', 'observed-read-only', 'parent-mobile-shell', 'observer-read-only'),
    step(
      'request-controller-takeover',
      'rejected',
      'manual-required',
      'manual-proof',
      'controller-takeover-manual-required',
      'takeover-denied',
      'mobile-controller-takeover-device-proof-required'
    ),
    step('deny-controller-takeover', 'rejected', 'denied', 'agent-service', 'observer-read-only', 'takeover-denied'),
    step(
      'degrade-controller-session',
      'degraded',
      'degraded',
      'agent-service',
      'observer-read-only',
      'lan-ai-provider-unavailable',
      'lan-ai-provider-degraded'
    ),
    step('release-controller-lease', 'completed', 'released', 'agent-service', 'observer-read-only'),
    step(
      'handoff-lan-ai-provider',
      aiState,
      aiState,
      'lan-ai-provider',
      'observer-read-only',
      'lan-ai-provider-unavailable',
      aiState === 'degraded' ? 'lan-ai-provider-degraded' : 'mobile-provider-unavailable'
    ),
    step(
      'disable-phone-local-model',
      'rejected',
      'disabled-by-default',
      'parent-mobile-shell',
      'observer-read-only',
      null,
      'phone-local-model-disabled-by-default'
    ),
    step(
      'refuse-cloud-relay',
      'not-implemented',
      'not-implemented',
      'cloud-relay-not-implemented',
      'observer-read-only',
      null,
      'cloud-relay-not-implemented'
    ),
  ];
}

function step(
  phase,
  responseState,
  handoffState,
  runtimeOwner,
  commandAuthorityState,
  rejectionReason = null,
  unavailableReason = null
) {
  return {
    phase,
    responseState,
    handoffState,
    runtimeOwner,
    commandAuthorityState,
    rejectionReason,
    unavailableReason,
    proofLabel: `parent-mobile-controller-observer-handoff:${phase}`,
    proofRequirement: `${phase} must not upgrade parent mobile beyond current package and LAN evidence`,
  };
}

function routeCandidate(platform, providerReadModel) {
  if (platform === 'android') {
    return providerReadModel.candidates.find((candidate) => candidate.lifecycleState === 'candidate-degraded');
  }

  return providerReadModel.candidates.find((candidate) => candidate.lifecycleState === 'manual-required');
}

function findBridgeModel(serviceBridgeProof, platform) {
  const model = serviceBridgeProof.runtimeReadModel.mobileBridgeReadModels.find((entry) => entry.platform === platform);
  if (!model) {
    throw new Error(`Missing ${platform} service bridge read model.`);
  }
  return model;
}

function cloudRelayConnection(bridgeModel) {
  return bridgeConnection(bridgeModel, 'cloud-relay');
}

function connectionState(bridgeModel, connectionKind) {
  return bridgeConnection(bridgeModel, connectionKind).state;
}

function bridgeConnection(bridgeModel, connectionKind) {
  const connection = bridgeModel.connections.find((entry) => entry.connectionKind === connectionKind);
  if (!connection) {
    throw new Error(`Missing ${connectionKind} connection.`);
  }
  return connection;
}

function proofInput(source, path, command) {
  return { source, path, command };
}

async function parseRuntimeReadModel(readModel) {
  const module = await import('@ocentra-parent/parent-domain/parent-mobile-controller-observer-handoff-runtime');
  proofLabels.push('parent-domain.parent-mobile-controller-observer-handoff-runtime-parse');
  return module.ParentMobileControllerObserverHandoffRuntimeReadModelSchema.parse(readModel);
}

function assertServiceBridgeProof(proof) {
  assertEqual(proof.proofMode, 'parent-mobile-service-bridge-proof', 'service bridge proof mode');
  for (const model of proof.runtimeReadModel.mobileBridgeReadModels) {
    assertEqual(
      model.commandAuthorityState === 'active-controller-backend-proof',
      false,
      `${model.platform} authority`
    );
    assertEqual(model.aiSubmission.localModelExecutionAllowed, false, `${model.platform} local model`);
    assertEqual(cloudRelayConnection(model).state, 'not-implemented', `${model.platform} cloud relay`);
  }
  proofLabels.push('service-bridge.observer-authority-and-ai-boundaries');
}

function assertProductionMobileProof(proof) {
  assertEqual(proof.proofMode, 'v0-9-production-lan-mobile-controller-proof', 'production mobile proof mode');
  assertEqual(proof.manualProofGates.parentMobileControllerObserver.state, 'manual-required', 'mobile controller gate');
  assertEqual(proof.manualProofGates.cloudRelay.state, 'not-implemented', 'cloud relay gate');
  proofLabels.push('production-mobile.manual-handoff-gates');
}

function assertDiscoveryRuntimeProof(proof) {
  assertEqual(proof.proofMode, 'v0-9-mobile-controller-discovery-runtime-proof', 'discovery runtime proof mode');
  assertEqual(
    proof.runtimeReadModel.claimBoundaries.cloudRelay,
    'not implemented and not counted as LAN proof',
    'discovery cloud relay boundary'
  );
  proofLabels.push('discovery-runtime.route-boundaries');
}

function assertProviderSelectionProof(proof) {
  assertEqual(proof.proofMode, 'v0-9-prod-discovery-provider-selection-proof', 'provider selection proof mode');
  const lifecycleStates = proof.providerSelectionSummary.lifecycleStates;
  assertArrayIncludes(lifecycleStates, 'candidate-degraded', 'provider lifecycle states');
  assertArrayIncludes(lifecycleStates, 'manual-required', 'provider lifecycle states');
  assertEqual(
    proof.providerSelectionReadModel.cloudRelayImplementationState,
    'not-implemented',
    'provider cloud relay'
  );
  proofLabels.push('provider-selection.degraded-and-manual-handoff-states');
}

function assertRuntimeReadModel(readModel) {
  assertEqual(readModel.handoffReadModels.length, 2, 'handoff read-model count');
  for (const model of readModel.handoffReadModels) {
    const phases = model.handoffSteps.map((handoffStep) => handoffStep.phase);
    assertArrayIncludes(phases, 'request-controller-takeover', `${model.platform} takeover phase`);
    assertArrayIncludes(phases, 'handoff-lan-ai-provider', `${model.platform} LAN AI phase`);
    assertArrayIncludes(phases, 'disable-phone-local-model', `${model.platform} phone-local model phase`);
    assertEqual(model.lanAiHandoff.localModelExecutionAllowed, false, `${model.platform} local model`);
    assertEqual(model.routeSnapshot.parentCacheState, 'stale', `${model.platform} parent cache state`);
    assertEqual(model.routeSnapshot.parentOwnedStorageState, 'offline', `${model.platform} parent storage state`);
  }
  proofLabels.push('handoff-runtime.read-model-boundaries');
}

async function assertScriptWiring() {
  const packageJson = JSON.parse(await readFile(join(repoRoot, 'package.json'), 'utf8'));
  const parentDomainPackage = JSON.parse(
    await readFile(join(repoRoot, 'packages', 'parent-domain', 'package.json'), 'utf8')
  );
  assertEqual(
    packageJson.scripts['test:parent-mobile-controller-observer-handoff'],
    proofCommand,
    'root parent mobile handoff script'
  );
  if (!parentDomainPackage.exports['./parent-mobile-controller-observer-handoff-runtime']) {
    throw new Error('Missing parent-domain parent-mobile-controller-observer-handoff-runtime export.');
  }
  proofLabels.push('package-scripts.parent-mobile-controller-observer-handoff');
  return {
    rootScript: 'test:parent-mobile-controller-observer-handoff',
    parentDomainExport: './parent-mobile-controller-observer-handoff-runtime',
  };
}

async function runNodeScript(scriptPath) {
  await runCommand('cmd', ['/c', 'node', scriptPath]);
}

async function runNpm(args) {
  await runCommand(...npmCommand([...args]));
}

async function runCommand(commandName, args) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}`))
    );
    child.once('error', reject);
  });
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function ensureProofArtifact(path, expectedMode, commandSpec) {
  if (await proofArtifactMatches(path, expectedMode)) {
    commands.push(`reuse-proof ${relativePath(path)}`);
    return;
  }
  const [commandName, ...args] = commandSpec;
  await runCommand(commandName, args);
}

async function proofArtifactMatches(path, expectedMode) {
  try {
    const proof = await readJson(path);
    return proof.proofMode === expectedMode;
  } catch {
    return false;
  }
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
