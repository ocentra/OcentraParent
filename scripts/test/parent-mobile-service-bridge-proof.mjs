import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'parent-mobile-service-bridge-proof');
const proofPath = join(outputDir, 'proof.json');
const parentMobileProofPath = join(repoRoot, 'test-results', 'parent-mobile-shell-runtime-proof', 'proof.json');
const productionMobileProofPath = join(
  repoRoot,
  'test-results',
  'v0-9-production-lan-mobile-controller-proof',
  'proof.json'
);
const observerProofPath = join(repoRoot, 'test-results', 'v0-9-mobile-controller-observer-runtime-proof', 'proof.json');
const checkpointPath = join(repoRoot, 'docs', 'checkpoints', 'parent-mobile-service-bridge-proof-2026-05-29.md');
const proofCommand = 'node scripts/test/parent-mobile-service-bridge-proof.mjs';
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runNpm(['run', 'build:contracts']);
  await runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/parent-mobile-service-bridge-runtime.test.ts',
  ]);
  await ensureProofArtifact(productionMobileProofPath, 'v0-9-production-lan-mobile-controller-proof', [
    'cmd',
    '/c',
    'node',
    'scripts/test/v0-9-production-lan-mobile-controller-proof.mjs',
  ]);
  await ensureParentMobileProofArtifact();
  await ensureProofArtifact(observerProofPath, 'v0-9-mobile-controller-observer-runtime-proof', [
    'cmd',
    '/c',
    'node',
    'scripts/test/v0-9-mobile-controller-observer-runtime-proof.mjs',
  ]);

  const parentMobileProof = await readJson(parentMobileProofPath);
  const productionMobileProof = await readJson(productionMobileProofPath);
  const observerProof = await readJson(observerProofPath);
  const runtimeReadModel = await parseRuntimeReadModel(
    buildRuntimeReadModel(parentMobileProof, productionMobileProof, observerProof)
  );

  assertParentMobileProof(parentMobileProof);
  assertProductionMobileProof(productionMobileProof);
  assertObserverProof(observerProof);
  assertRuntimeReadModel(runtimeReadModel);
  const scriptWiring = await assertScriptWiring();

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'parent-mobile-service-bridge-proof',
    commands,
    proofLabels,
    evidence: {
      parentMobileRuntime: relative(repoRoot, parentMobileProofPath),
      productionMobileController: relative(repoRoot, productionMobileProofPath),
      observerRuntime: relative(repoRoot, observerProofPath),
      contract: 'packages/parent-domain/src/parent-mobile-service-bridge-runtime.ts',
      contractTest: 'packages/parent-domain/tests/parent-mobile-service-bridge-runtime.test.ts',
      checkpoint: relative(repoRoot, checkpointPath),
      output: relative(repoRoot, proofPath),
    },
    runtimeReadModel,
    scriptWiring,
    proofMatrixRegistration: {
      state: 'deferred-shared-matrix-registration',
      reason:
        'Shared pre-AI proof matrix was not edited in this slice; register after the active matrix owner clears the lock.',
    },
    claimsProved: [
      'Parent mobile local service, LAN service, cloud relay, and mobile package bridge states are explicit and typed.',
      'Parent mobile parent-cache and parent-owned storage route states are explicit stale/offline states, not silent fallback paths.',
      'Observer mobile surfaces remain read-only for policy writes and approval decisions.',
      'Controller takeover remains manual-required for Android and iOS package/device authority.',
      'LAN AI provider submission is degraded or unavailable and never becomes phone-local model execution.',
      'Package and service launch gaps remain explicit for signing, stores, notifications, foreground/background service behavior, and controller authority.',
    ],
    claimsNotProved: [
      'real Android or iOS parent mobile controller write authority',
      'foreground/background mobile service launch on a real device',
      'physical household LAN router or firewall behavior',
      'cloud relay routing, authentication, or storage',
      'parent-owned storage sync or cache freshness',
      'phone-local model execution for parent mobile assistant work',
      'C-owned UI rendering or vendor visual changes',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`parent-mobile-service-bridge-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function buildRuntimeReadModel(parentMobileProof, productionMobileProof, observerProof) {
  return {
    schemaVersion: 'parent-mobile-service-bridge-proof',
    proofHarness: {
      sourceProofs: [
        proofInput(
          'parent-mobile-shell-runtime-proof',
          'test-results/parent-mobile-shell-runtime-proof/proof.json',
          'node scripts/test/parent-mobile-shell-runtime-proof.mjs'
        ),
        proofInput(
          'v0-9-production-lan-mobile-controller-proof',
          'test-results/v0-9-production-lan-mobile-controller-proof/proof.json',
          'node scripts/test/v0-9-production-lan-mobile-controller-proof.mjs'
        ),
        proofInput(
          'v0-9-mobile-controller-observer-runtime-proof',
          'test-results/v0-9-mobile-controller-observer-runtime-proof/proof.json',
          'node scripts/test/v0-9-mobile-controller-observer-runtime-proof.mjs'
        ),
      ],
      outputProofPath: 'test-results/parent-mobile-service-bridge-proof/proof.json',
      checkpointPath: 'docs/checkpoints/parent-mobile-service-bridge-proof-2026-05-29.md',
    },
    mobileBridgeReadModels: [
      buildMobileBridgeReadModel(
        'android',
        parentMobileProof.runtimeProof.androidObserver,
        'parent-mobile-android-service-bridge'
      ),
      buildMobileBridgeReadModel('ios', parentMobileProof.runtimeProof.iosObserver, 'parent-mobile-ios-service-bridge'),
    ],
    claimBoundaries: {
      parentMobileWriteAuthority: observerProof.runtimeReadModel.claimBoundaries.parentMobileWriteAuthority,
      physicalHouseholdLan: productionMobileProof.manualProofGates.physicalHouseholdLan.requiredArtifacts.join('; '),
      cloudRelay: observerProof.runtimeReadModel.claimBoundaries.cloudRelay,
      parentOwnedStorage:
        'parent-owned storage is offline in this proof and does not silently replace LAN service or cloud relay',
      phoneLocalModel: 'disabled by default; parent mobile does not load a phone-local model for assistant work',
      packageServiceLaunch: 'manual-required until foreground/background mobile service launch is proven on device',
      androidParentMobile: 'Android parent mobile observer proof stays separate from Android child-agent authority',
      iosParentMobile: 'iOS parent mobile controller-candidate proof stays separate from iOS child-agent authority',
      androidChildAgent: 'Android child-agent foreground service and Device Owner claims are not proved here',
      iosChildAgent: 'iOS child-agent Family Controls and DeviceActivity claims are not proved here',
      cUiOwnership: 'C UI can render this later, but this proof does not touch UI or vendor paths',
    },
    updatedAt: observerProof.checkedAt,
  };
}

function buildMobileBridgeReadModel(platform, mobileSummary, parentDeviceId) {
  return {
    platform,
    parentDeviceId,
    role: platform === 'android' ? 'observer' : 'controller-candidate',
    controllerState: mobileSummary.controllerState,
    commandAuthorityState: mobileSummary.commandAuthorityState,
    connections: serviceConnections(mobileSummary),
    packageReadiness: packageReadiness(platform, mobileSummary),
    aiSubmission: aiSubmission(mobileSummary),
    capabilities: capabilityStates(mobileSummary.capabilityStates),
    operationProofs: operationProofs(mobileSummary.assistantJobState),
  };
}

function serviceConnections(mobileSummary) {
  return [
    connection('local-service', mobileSummary.localService, 'manual-proof', null),
    connection('lan-service', mobileSummary.lanService, 'lan-ai-provider', 'route-parent-mobile-lan-provider'),
    connection('cloud-relay', mobileSummary.cloudRelay, 'cloud-relay-not-implemented', null),
    connection('parent-cache', mobileSummary.parentCache, 'parent-cache', null),
    connection('parent-owned-storage', mobileSummary.parentOwnedStorage, 'parent-owned-storage', null),
    connection('mobile-package', mobileSummary.packageState, 'parent-mobile-shell', null),
  ];
}

function packageReadiness(platform, mobileSummary) {
  return {
    platform,
    packageState: mobileSummary.packageState,
    serviceLaunchState: 'manual-required',
    launchTarget: platform === 'android' ? 'ca.ocentra.parent.agent/.MainActivity' : 'ca.ocentra.parent.agent',
    signingState: 'manual-required',
    storeDistributionState: 'manual-required',
    missingCapabilityProofs: missingCapabilityProofs(mobileSummary.capabilityStates),
  };
}

function aiSubmission(mobileSummary) {
  return {
    route: mobileSummary.assistantJobRoute,
    jobState: mobileSummary.assistantJobState,
    providerId: null,
    requiredCapabilities: ['chat-completion', 'summarization'],
    evidenceReferenceIds: mobileSummary.assistantJobState === 'degraded' ? ['activity-event-parent-mobile-proof'] : [],
    unavailableReason:
      mobileSummary.assistantJobState === 'degraded'
        ? 'lan-ai-provider-unavailable'
        : 'mobile-package-service-bridge-required',
    localModelExecutionState: 'disabled-by-default',
    localModelExecutionAllowed: false,
  };
}

function operationProofs(aiState) {
  return [
    operationProof(
      'service-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    operationProof(
      'lan-route-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    operationProof(
      'parent-cache-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    operationProof(
      'parent-owned-storage-status-read',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      'observer-read-only'
    ),
    operationProof('capability-refresh', 'completed', 'allowed-read-only', 'parent-mobile-shell', 'observer-read-only'),
    operationProof(
      'package-service-launch',
      'rejected',
      'manual-required-mobile-package',
      'manual-proof',
      'controller-takeover-manual-required',
      'takeover-denied',
      'mobile-package-service-launch-proof-required'
    ),
    operationProof(
      'controller-takeover-request',
      'rejected',
      'manual-required-mobile-package',
      'manual-proof',
      'controller-takeover-manual-required',
      'takeover-denied',
      'mobile-controller-takeover-device-proof-required'
    ),
    operationProof('controller-release', 'completed', 'proved-local-service', 'agent-service', 'observer-read-only'),
    operationProof(
      'write-policy',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'observer-read-only'
    ),
    operationProof(
      'approval-decision',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'observer-read-only'
    ),
    operationProof(
      'submit-lan-ai-job',
      aiState,
      aiState === 'degraded' ? 'degraded-provider' : 'unavailable',
      'lan-ai-provider',
      'observer-read-only',
      'lan-ai-provider-unavailable',
      aiState === 'degraded' ? 'lan-ai-provider-degraded' : 'mobile-package-service-bridge-required'
    ),
    operationProof(
      'submit-cloud-relay-job',
      'not-implemented',
      'not-implemented',
      'cloud-relay-not-implemented',
      'observer-read-only',
      null,
      'cloud-relay-not-implemented'
    ),
    operationProof(
      'submit-phone-local-model-job',
      'rejected',
      'rejected-no-phone-local-model',
      'parent-mobile-shell',
      'observer-read-only',
      null,
      'phone-local-model-disabled-by-default'
    ),
  ];
}

function connection(connectionKind, state, runtimeOwner, selectedRouteId) {
  return {
    connectionKind,
    state,
    runtimeOwner,
    selectedRouteId,
    proofLabel: `parent-mobile-service-bridge:${connectionKind}`,
    proofRequirement: `${connectionKind} state must stay explicit in the parent mobile service bridge`,
  };
}

function operationProof(
  operation,
  responseState,
  operationState,
  runtimeOwner,
  commandAuthorityState,
  rejectionReason = null,
  unavailableReason = null
) {
  return {
    operation,
    responseState,
    operationState,
    runtimeOwner,
    commandAuthorityState,
    rejectionReason,
    unavailableReason,
    proofLabel: `parent-mobile-service-bridge:${operation}`,
    proofRequirement: `${operation} proof must not upgrade parent mobile beyond current bridge evidence`,
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  };
}

function proofInput(source, path, command) {
  return { source, path, command };
}

function missingCapabilityProofs(capabilityStates) {
  return Object.entries(capabilityStates)
    .filter(([, status]) => status !== 'scaffold' && status !== 'implemented' && status !== 'supported')
    .map(([capability]) => capability);
}

function capabilityStates(capabilityStatesByName) {
  return Object.entries(capabilityStatesByName).map(([capability, status]) => ({
    capability,
    status,
    proofRequirement: `${capability} remains ${status} until real mobile artifacts prove otherwise`,
    claimBoundary: `${capability} is not upgraded by parent mobile service bridge proof`,
  }));
}

async function parseRuntimeReadModel(readModel) {
  const module = await import('@ocentra-parent/parent-domain/parent-mobile-service-bridge-runtime');
  proofLabels.push('parent-domain.parent-mobile-service-bridge-runtime-parse');
  return module.ParentMobileServiceBridgeRuntimeReadModelSchema.parse(readModel);
}

function assertParentMobileProof(proof) {
  assertEqual(proof.runtimeProof.androidObserver.commandAuthorityState, 'observer-read-only', 'Android authority');
  assertEqual(
    proof.runtimeProof.iosObserver.commandAuthorityState,
    'controller-takeover-manual-required',
    'iOS authority'
  );
  assertEqual(proof.runtimeProof.androidObserver.cloudRelay, 'not-implemented', 'Android cloud relay');
  assertEqual(proof.runtimeProof.iosObserver.cloudRelay, 'not-implemented', 'iOS cloud relay');
  assertEqual(proof.runtimeProof.androidObserver.parentCache, 'stale', 'Android parent cache');
  assertEqual(proof.runtimeProof.iosObserver.parentOwnedStorage, 'offline', 'iOS parent-owned storage');
  assertEqual(proof.runtimeProof.localModelExecutionDefault, 'disabled-by-default', 'phone-local model default');
  assertEqual(proof.runtimeProof.childAgentBehaviorClaim, 'not-claimed', 'child-agent non-claim');
  proofLabels.push('parent-mobile-shell.boundaries');
}

function assertProductionMobileProof(proof) {
  assertEqual(proof.proofMode, 'v0-9-production-lan-mobile-controller-proof', 'production proof mode');
  assertEqual(proof.manualProofGates.parentMobileControllerObserver.state, 'manual-required', 'mobile authority gate');
  assertEqual(proof.manualProofGates.cloudRelay.state, 'not-implemented', 'cloud relay gate');
  proofLabels.push('production-lan-mobile.manual-gates');
}

function assertObserverProof(proof) {
  assertEqual(proof.proofMode, 'v0-9-mobile-controller-observer-runtime-proof', 'observer proof mode');
  for (const readModel of proof.runtimeReadModel.mobileReadModels) {
    const operations = Object.fromEntries(
      readModel.operationProofs.map((operation) => [operation.operation, operation])
    );
    assertEqual(operations['write-policy'].operationState, 'rejected-observer-read-only', 'observer write rejection');
    assertEqual(
      operations['request-controller-takeover'].operationState,
      'manual-required-mobile-package',
      'mobile takeover gate'
    );
    if (
      operations['submit-lan-ai-job'].operationState !== 'degraded-provider' &&
      operations['submit-lan-ai-job'].operationState !== 'unavailable'
    ) {
      throw new Error(`${readModel.platform} LAN AI state: expected degraded-provider or unavailable`);
    }
  }
  const lanAiStates = new Set(
    proof.runtimeReadModel.mobileReadModels.map(
      (entry) =>
        Object.fromEntries(entry.operationProofs.map((operation) => [operation.operation, operation]))[
          'submit-lan-ai-job'
        ].operationState
    )
  );
  assertArrayIncludes([...lanAiStates], 'degraded-provider', 'observer LAN AI state coverage');
  assertArrayIncludes([...lanAiStates], 'unavailable', 'observer LAN AI state coverage');
  proofLabels.push('observer-runtime.operation-boundaries');
}

function assertRuntimeReadModel(readModel) {
  assertEqual(readModel.mobileBridgeReadModels.length, 2, 'mobile bridge platform count');
  for (const model of readModel.mobileBridgeReadModels) {
    assertEqual(
      model.connections.find((connection) => connection.connectionKind === 'cloud-relay').state,
      'not-implemented',
      `${model.platform} cloud relay`
    );
    assertEqual(model.aiSubmission.localModelExecutionAllowed, false, `${model.platform} local model execution`);
    assertEqual(
      model.connections.find((connection) => connection.connectionKind === 'parent-cache').state,
      'stale',
      `${model.platform} parent cache`
    );
    assertEqual(
      model.connections.find((connection) => connection.connectionKind === 'parent-owned-storage').state,
      'offline',
      `${model.platform} parent-owned storage`
    );
    assertArrayLengthAtLeast(model.packageReadiness.missingCapabilityProofs, 5, `${model.platform} package gaps`);
  }
  proofLabels.push('service-bridge.runtime-read-model');
}

async function assertScriptWiring() {
  const packageJson = JSON.parse(await readFile(join(repoRoot, 'package.json'), 'utf8'));
  const parentDomainPackage = JSON.parse(
    await readFile(join(repoRoot, 'packages', 'parent-domain', 'package.json'), 'utf8')
  );
  assertEqual(
    packageJson.scripts['test:parent-mobile-service-bridge'],
    proofCommand,
    'root parent mobile service bridge script'
  );
  if (!parentDomainPackage.exports['./parent-mobile-service-bridge-runtime']) {
    throw new Error('Missing parent-domain parent-mobile-service-bridge-runtime export.');
  }
  proofLabels.push('package-scripts.parent-mobile-service-bridge');
  return {
    rootScript: 'test:parent-mobile-service-bridge',
    parentDomainExport: './parent-mobile-service-bridge-runtime',
  };
}

async function runNodeScript(scriptPath) {
  await runCommand('cmd', ['/c', 'node', scriptPath]);
}

async function runNpm(args) {
  await runCommand('cmd', ['/c', 'npm', ...args]);
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
    commands.push(`reuse-proof ${relative(repoRoot, path).replaceAll('\\', '/')}`);
    return;
  }
  const [commandName, ...args] = commandSpec;
  await runCommand(commandName, args);
}

async function ensureParentMobileProofArtifact() {
  if (await parentMobileProofArtifactMatches(parentMobileProofPath)) {
    commands.push(`reuse-proof ${relative(repoRoot, parentMobileProofPath).replaceAll('\\', '/')}`);
    return;
  }
  await runCommand('cmd', ['/c', 'node', 'scripts/test/parent-mobile-shell-runtime-proof.mjs']);
}

async function proofArtifactMatches(path, expectedMode) {
  try {
    const proof = await readJson(path);
    return proof.proofMode === expectedMode;
  } catch {
    return false;
  }
}

async function parentMobileProofArtifactMatches(path) {
  try {
    const proof = await readJson(path);
    return proof.runtimeProof?.childAgentBehaviorClaim === 'not-claimed';
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

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertArrayLengthAtLeast(value, minimum, label) {
  if (!Array.isArray(value) || value.length < minimum) {
    throw new Error(`${label}: expected at least ${minimum} entries, received ${value?.length ?? 'non-array'}`);
  }
}
