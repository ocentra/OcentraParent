import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-household-lan-production-discovery-proof');
const proofPath = join(outputDir, 'proof.json');
const productionProofPath = join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json');
const readinessProofPath = join(repoRoot, 'test-results', 'v0-9-household-lan-proof-readiness', 'proof.json');
const discoveryProofPath = join(repoRoot, 'test-results', 'v0-9-lan-discovery-challenge-mvp', 'proof.json');
const pairingProofPath = join(repoRoot, 'test-results', 'v0-9-lan-pairing-control-mvp', 'proof.json');
const matrixPath = join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json');

const command = 'node scripts/test/v0-9-household-lan-production-discovery-proof.mjs';
const claimId = 'v0-9-household-lan-production-discovery-proof';
const commands = [];
const proofLabels = [];

const physicalArtifactRequirements = [
  'two distinct physical devices on the same household LAN with recorded parent and child host names or IP addresses',
  'router or network note showing child service port reachability from the parent host',
  'OS firewall prompt or firewall rule evidence for the child service listener',
  'origin allowlist used by the parent portal or controller host during the run',
  'route selection, takeover, revocation, wrong-origin, wrong-device, replay, stale, and failed-unpaired command results from the physical devices',
  'offline or stale selected-device artifact from stopping or pausing the selected child service before a control command',
  'LAN AI provider advertised/accepted/rejected/degraded result from a real opted-in provider host before claiming household provider routing',
];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand(['run', 'test', '--workspace', '@ocentra-parent/parent-domain', '--', 'tests/lan-pairing.test.ts'])
  );
  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-household-lan-proof-readiness.mjs']);

  const productionProof = await readJson(productionProofPath);
  const readinessProof = await readJson(readinessProofPath);
  const discoveryProof = await readJson(discoveryProofPath);
  const pairingProof = await readJson(pairingProofPath);
  const matrix = await readJson(matrixPath);

  const productionDiscoveryStates = assertProductionDiscoveryStates(productionProof, discoveryProof, pairingProof);
  const routeControlRejections = assertRouteControlRejections(pairingProof);
  const physicalClaimUpgradeVerifier = assertPhysicalClaimUpgradeRefused(readinessProof, productionProof);
  assertReadinessProof(readinessProof);
  assertProofMatrix(matrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'household-lan-production-discovery-boundary',
    productReadinessDecision: readinessProof.productReadinessDecision,
    commands,
    proofLabels,
    evidence: {
      productionLanMultidevice: relative(repoRoot, productionProofPath),
      householdLanReadiness: relative(repoRoot, readinessProofPath),
      discoveryChallenge: relative(repoRoot, discoveryProofPath),
      pairingControl: relative(repoRoot, pairingProofPath),
      proofMatrix: relative(repoRoot, matrixPath),
    },
    localServiceBoundary: {
      proofBoundary: 'local-real-service-processes-not-physical-household-lan',
      productionDiscoveryStates,
      routeControlRejections,
      selectedRouteTrust: productionProof.localTwoServiceProof.selectedRouteTrust,
      selectedDeviceReadinessStates: {
        stale: requiredAssertionsFor(productionProof, 'rust-selected-device-state').filter((assertion) =>
          assertion.includes('stale')
        ),
        offline: requiredAssertionsFor(productionProof, 'rust-selected-device-state').filter((assertion) =>
          assertion.includes('offline')
        ),
        registry: requiredAssertionsFor(productionProof, 'rust-trusted-registry-expiry-and-reachability'),
      },
      cloudRelayDecision: productionProof.cloudRelayDecision,
    },
    physicalClaimUpgradeVerifier,
    claimsProved: [
      'production discovery state labels remain explicit for discovered, pending, paired, revoked, stale, offline, and unavailable/manual-gated states',
      'paired, unpaired, wrong-origin, wrong-device, replay, stale, revoked, observer-read-only, missing-lease, expired-lease, and wrong-controller routes are gathered from real local service proof artifacts',
      'selected-route trust state, selected pairing id, stale time, and offline time are explicit in local service status proof artifacts',
      'selected-device stale and offline readiness states are backed by Rust service and core registry proof assertions',
      'physical household LAN readiness is refused without required two-device, router, firewall, origin, stale/offline, failed-unpaired, and provider artifacts',
    ],
    claimsNotProved: [
      'product-ready household router discovery',
      'two physical household devices on a shared router/firewall path',
      'parent mobile controller write authority or background LAN behavior',
      'cloud relay routing, storage, or authentication',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-9-household-lan-production-discovery-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function assertProductionDiscoveryStates(productionProof, discoveryProof, pairingProof) {
  assertEqual(productionProof.proofMode, 'local-multi-service-production-lan-hardening', 'production proof mode');
  for (const step of [
    'discovery-challenge',
    'pairing-control',
    'lan-ai-provider-pool',
    'rust-selected-device-state',
    'rust-trusted-registry-expiry-and-reachability',
  ]) {
    if (!productionProof.checkedSteps.some((candidate) => candidate.label === step)) {
      throw new Error(`Production proof is missing checked step ${step}.`);
    }
  }

  for (const expected of [
    'wrong-origin-websocket-rejected-before-upgrade',
    'first-discovery-agent:anonymous-control-rejected',
    'first-discovery-agent:challenge-preview-issued',
    'first-discovery-agent:challenge-proof-accepted',
    'first-discovery-agent:challenge-proof-replay-rejected',
    'second-discovery-agent:challenge-preview-issued',
    'wrong-agent-port-challenge-rejected-as-wrong-device',
  ]) {
    assertArrayIncludes(discoveryProof.assertions, expected, 'production discovery assertion');
  }
  for (const expected of [
    'first-child-agent:pairing-proof-accepted-unselected',
    'first-child-agent:route-selected',
    'first-child-agent:selected-route-trust-state-paired',
    'first-child-agent:route-revoked',
    'second-child-agent:restart-restores-selected-route',
    'second-child-agent:restart-restores-selected-route-trust-state',
  ]) {
    assertArrayIncludes(pairingProof.assertions, expected, 'route state assertion');
  }

  proofLabels.push('v0.9.production-discovery.explicit-state-labels');
  proofLabels.push('v0.9.selected-route.trust-state-explicit');
  return {
    discovered: 'parent-domain contract proof: LanPairingDiscoveryDeviceSchema accepts discoveryState=discovered',
    pending: 'first-discovery-agent:challenge-preview-issued',
    paired: ['first-discovery-agent:challenge-proof-accepted', 'first-child-agent:route-selected'],
    revoked: 'first-child-agent:route-revoked',
    stale: requiredAssertionsFor(productionProof, 'rust-selected-device-state').find((assertion) =>
      assertion.includes('stale-status')
    ),
    offline: requiredAssertionsFor(productionProof, 'rust-selected-device-state').find((assertion) =>
      assertion.includes('offline-status')
    ),
    unavailable: 'manual-required physical household LAN gate until real artifacts exist',
  };
}

function assertRouteControlRejections(pairingProof) {
  const required = [
    'first-child-agent:anonymous-rejected',
    'first-child-agent:unselected-control-rejected',
    'first-child-agent:observer-write-rejected',
    'first-child-agent:replay-rejected',
    'first-child-agent:stale-control-rejected',
    'first-child-agent:malformed-control-rejected',
    'first-child-agent:missing-controller-lease-rejected',
    'first-child-agent:expired-controller-lease-rejected',
    'first-child-agent:wrong-controller-rejected',
    'first-child-agent:controller-lease-takeover-denied',
    'first-child-agent:revoked-control-rejected',
    'wrong-agent-port-rejected-as-wrong-device',
  ];
  for (const expected of required) {
    assertArrayIncludes(pairingProof.assertions, expected, 'route control rejection');
  }

  proofLabels.push('v0.9.route-control.failed-unpaired-and-dishonest-states');
  return {
    failedUnpaired: 'first-child-agent:anonymous-rejected',
    unselectedDevice: 'first-child-agent:unselected-control-rejected',
    wrongOrigin: 'wrong-origin-websocket-rejected-before-upgrade',
    wrongDevice: 'wrong-agent-port-rejected-as-wrong-device',
    replay: 'first-child-agent:replay-rejected',
    stale: 'first-child-agent:stale-control-rejected',
    revoked: 'first-child-agent:revoked-control-rejected',
    observerReadOnly: 'first-child-agent:observer-write-rejected',
    leaseRequired: [
      'first-child-agent:missing-controller-lease-rejected',
      'first-child-agent:expired-controller-lease-rejected',
      'first-child-agent:wrong-controller-rejected',
    ],
  };
}

function assertPhysicalClaimUpgradeRefused(readinessProof, productionProof) {
  assertEqual(
    readinessProof.readinessGate.physicalHouseholdLan.state,
    'manual-required',
    'physical household LAN state'
  );
  assertArrayIncludes(
    productionProof.claimsNotProvedLocally,
    'real household router discovery across two physical devices',
    'physical household router non-claim'
  );
  for (const artifact of physicalArtifactRequirements) {
    assertArrayIncludes(readinessProof.readinessGate.physicalHouseholdLan.requiredArtifacts, artifact, 'artifact gate');
  }

  proofLabels.push('v0.9.physical-household-claim-upgrade-refused-without-artifacts');
  return {
    requestedUpgradeState: 'product-ready-household-lan-discovery',
    suppliedPhysicalArtifacts: [],
    currentState: 'manual-required',
    decision: 'rejected',
    rejectionReason:
      'Local multi-service proof cannot upgrade household LAN readiness without two physical devices, router/firewall evidence, origin evidence, stale/offline behavior, failed-unpaired results, and provider artifacts.',
    missingArtifacts: physicalArtifactRequirements,
  };
}

function assertReadinessProof(readinessProof) {
  assertEqual(
    readinessProof.productReadinessDecision,
    'not-ready-for-product-ready-household-lan-claim',
    'readiness decision'
  );
  assertEqual(
    readinessProof.readinessGate.localMultiServiceProof.state,
    'ci-mechanical-proof',
    'local multi-service state'
  );
  assertEqual(readinessProof.readinessGate.cloudRelay.state, 'not-implemented', 'cloud relay state');
  assertArrayIncludes(
    readinessProof.claimsNotProvedByThisGate,
    'product-ready LAN behavior on a household router',
    'product-ready LAN non-claim'
  );
  proofLabels.push('v0.9.household-readiness.manual-gates-preserved');
}

function assertProofMatrix(matrix) {
  assertArrayIncludes(matrix.requiredCompletedClaimIds, claimId, 'required completed claim');
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === claimId);
  if (!scenario) {
    throw new Error(`Proof matrix is missing ${claimId} scenario.`);
  }
  assertArrayIncludes(scenario.ciCommands, command, 'scenario command');
  const claim = matrix.claims.find((candidate) => candidate.id === claimId);
  if (!claim) {
    throw new Error(`Proof matrix is missing ${claimId} claim.`);
  }
  assertArrayIncludes(claim.ciProof.commands, command, 'claim command');
  assertEqual(claim.runtimeSurfaceCoverage.physicalHouseholdLan.state, 'manual-required', 'matrix physical state');
  assertEqual(claim.runtimeSurfaceCoverage.claimUpgradeVerifier.state, 'implemented', 'matrix verifier state');
  proofLabels.push('proof-matrix.household-lan-production-discovery-boundary');
}

function requiredAssertionsFor(proof, label) {
  const step = proof.checkedSteps.find((candidate) => candidate.label === label);
  if (!step) {
    throw new Error(`Missing production proof step ${label}.`);
  }
  return step.requiredAssertions;
}

async function runCommand(commandName, args) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}`));
    });
    child.once('error', reject);
  });
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
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
