import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-household-lan-proof-readiness');
const proofPath = join(outputDir, 'proof.json');
const productionProofPath = join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json');
const discoveryProofPath = join(repoRoot, 'test-results', 'v0-9-lan-discovery-challenge-mvp', 'proof.json');
const pairingProofPath = join(repoRoot, 'test-results', 'v0-9-lan-pairing-control-mvp', 'proof.json');
const providerProofPath = join(repoRoot, 'test-results', 'platform-roles-lan-ai-provider-pool', 'proof.json');
const proofMatrixPath = join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json');

const command = 'node scripts/test/v0-9-household-lan-proof-readiness.mjs';
const claimId = 'v0-9-household-lan-proof-readiness';
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-production-lan-multidevice-hardening.mjs']);

  const productionProof = await readJson(productionProofPath);
  const discoveryProof = await readJson(discoveryProofPath);
  const pairingProof = await readJson(pairingProofPath);
  const providerProof = await readJson(providerProofPath);
  const matrix = await readJson(proofMatrixPath);

  assertProductionProof(productionProof);
  assertDiscoveryProof(discoveryProof);
  assertPairingProof(pairingProof);
  assertProviderProof(providerProof);
  assertProofMatrix(matrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'household-lan-readiness-gate',
    productReadinessDecision: 'not-ready-for-product-ready-household-lan-claim',
    commands,
    proofLabels,
    evidence: {
      productionLanMultidevice: relative(repoRoot, productionProofPath),
      discoveryChallenge: relative(repoRoot, discoveryProofPath),
      pairingControl: relative(repoRoot, pairingProofPath),
      lanAiProviderPool: relative(repoRoot, providerProofPath),
      proofMatrix: relative(repoRoot, proofMatrixPath),
    },
    observedLocalServiceStates: {
      discovery: {
        state: 'ci-mechanical-proof',
        services: serviceSummaries(discoveryProof),
        assertions: pickAssertions(discoveryProof.assertions, [
          'challenge-preview-issued',
          'challenge-proof-accepted',
          'challenge-proof-replay-rejected',
          'wrong-agent-port-challenge-rejected-as-wrong-device',
        ]),
      },
      routeControllerSelectedDevice: {
        state: 'ci-mechanical-proof',
        services: serviceSummaries(pairingProof),
        assertions: pickAssertions(pairingProof.assertions, [
          'route-selected',
          'observer-write-rejected',
          'controller-lease-renewed',
          'controller-lease-released',
          'controller-lease-reacquired',
          'controller-lease-takeover-denied',
          'controller-lease-takeover-accepted',
          'restart-restores-selected-route',
          'restart-recovered-approval-accepted',
          'wrong-agent-port-rejected-as-wrong-device',
        ]),
      },
      rejectionAndRevocation: {
        state: 'ci-mechanical-proof',
        assertions: pickAssertions(pairingProof.assertions, [
          'anonymous-rejected',
          'unselected-control-rejected',
          'replay-rejected',
          'stale-control-rejected',
          'malformed-control-rejected',
          'missing-controller-lease-rejected',
          'expired-controller-lease-rejected',
          'wrong-controller-rejected',
          'route-revoked',
          'revoked-control-rejected',
        ]),
      },
      lanAiProviderRouting: {
        state: 'ci-mechanical-proof',
        services: providerProof.services,
        assertions: pickAssertions(providerProof.assertions, [
          'provider-advertised-available',
          'controller-job-completed-observer-job-rejected',
          'unsupported-capability-rejected',
          'provider-unavailable',
          'controller-job-degraded-with-provider-unavailable',
          'provider-busy',
          'busy-job-degraded',
        ]),
      },
      staleOfflineSelectedDevice: {
        state: 'rust-service-and-core-proof',
        assertions: requiredAssertionsFor(productionProof, 'rust-selected-device-state').concat(
          requiredAssertionsFor(productionProof, 'rust-trusted-registry-expiry-and-reachability')
        ),
      },
    },
    readinessGate: {
      localMultiServiceProof: {
        state: 'ci-mechanical-proof',
        summary:
          'Real local Rust service processes prove direct WebSocket discovery, selected-route control, controller lease behavior, registry restart recovery, rejection paths, and LAN AI provider routing/degraded states.',
      },
      physicalHouseholdLan: {
        state: 'manual-required',
        requiredArtifacts: [
          'two distinct physical devices on the same household LAN with recorded parent and child host names or IP addresses',
          'router or network note showing child service port reachability from the parent host',
          'OS firewall prompt or firewall rule evidence for the child service listener',
          'origin allowlist used by the parent portal or controller host during the run',
          'route selection, takeover, revocation, wrong-origin, wrong-device, replay, stale, and failed-unpaired command results from the physical devices',
          'offline or stale selected-device artifact from stopping or pausing the selected child service before a control command',
          'LAN AI provider advertised/accepted/rejected/degraded result from a real opted-in provider host before claiming household provider routing',
        ],
      },
      parentMobileControllerObserver: {
        state: 'manual-required',
        requiredArtifacts: [
          'Android and iOS parent mobile package/device evidence for observer read-only behavior',
          'controller takeover/release/renewal evidence from a real mobile package before granting mobile write authority',
          'mobile background and notification behavior while LAN reachability changes',
        ],
      },
      cloudRelay: {
        state: 'not-implemented',
        decision:
          'Cloud relay is not implemented or counted as LAN proof; any remote relay path needs a separate authenticated proof slice.',
      },
    },
    claimsProvedByThisGate: [
      'local V0.9 LAN proof is distinguishable from physical household LAN readiness',
      'route/controller/selected-device/provider states are gathered from existing real-service proof artifacts',
      'two-device, router, firewall, mobile, cloud, stale/offline, and failed-unpaired physical checks remain explicit manual gates',
    ],
    claimsNotProvedByThisGate: [
      'product-ready LAN behavior on a household router',
      'mobile controller or observer behavior from a real Android or iOS package',
      'cloud relay routing',
      'firewall, OS prompt, NAT, or router discovery handling outside local service processes',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-9-household-lan-proof-readiness-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function assertProductionProof(proof) {
  assertEqual(proof.proofMode, 'local-multi-service-production-lan-hardening', 'production LAN proof mode');
  for (const step of [
    'discovery-challenge',
    'pairing-control',
    'lan-ai-provider-pool',
    'rust-selected-device-state',
    'rust-trusted-registry-expiry-and-reachability',
  ]) {
    if (!proof.checkedSteps.some((candidate) => candidate.label === step)) {
      throw new Error(`Production LAN proof is missing checked step ${step}.`);
    }
  }
  assertArrayIncludes(
    proof.claimsProvedLocally,
    'trusted registry persists selected route and recovers it after restart',
    'trusted registry restart recovery'
  );
  assertArrayIncludes(
    proof.claimsProvedLocally,
    'selected-device stale and offline read-model states reject control through focused Rust service and core registry proof',
    'selected-device stale/offline proof'
  );
  assertArrayIncludes(
    proof.claimsProvedLocally,
    'LAN AI provider routing covers authorized result, unsupported capability, busy, unavailable, and observer rejection',
    'LAN AI provider proof'
  );
  assertArrayIncludes(
    proof.claimsNotProvedLocally,
    'real household router discovery across two physical devices',
    'physical household router non-claim'
  );
  assertArrayIncludes(
    proof.claimsNotProvedLocally,
    'OS firewall prompts and mobile background behavior on Windows/macOS/Linux/Android/iOS',
    'firewall and mobile non-claim'
  );
  if (!Array.isArray(proof.manualTwoDeviceChecklist) || proof.manualTwoDeviceChecklist.length !== 1) {
    throw new Error('Production LAN proof must carry exactly one manual two-device checklist.');
  }
  proofLabels.push('v0.9.local-production-lan-proof-consumed');
  proofLabels.push('v0.9.household-physical-lan-remains-manual-required');
}

function assertDiscoveryProof(proof) {
  for (const expected of [
    'wrong-origin-websocket-rejected-before-upgrade',
    'first-discovery-agent:challenge-proof-accepted',
    'second-discovery-agent:challenge-proof-accepted',
    'wrong-agent-port-challenge-rejected-as-wrong-device',
  ]) {
    assertArrayIncludes(proof.assertions, expected, 'discovery challenge assertion');
  }
  proofLabels.push('v0.9.discovery-current-local-states-gathered');
}

function assertPairingProof(proof) {
  for (const expected of [
    'first-child-agent:observer-write-rejected',
    'first-child-agent:controller-lease-renewed',
    'first-child-agent:revoked-control-rejected',
    'second-child-agent:controller-lease-takeover-accepted',
    'second-child-agent:restart-restores-selected-route',
    'wrong-agent-port-rejected-as-wrong-device',
  ]) {
    assertArrayIncludes(proof.assertions, expected, 'pairing control assertion');
  }
  proofLabels.push('v0.9.route-controller-selected-device-states-gathered');
}

function assertProviderProof(proof) {
  for (const expected of [
    'parent-desktop-controller-ai-provider:provider-advertised-available',
    'parent-desktop-controller-ai-provider:controller-job-completed-observer-job-rejected',
    'parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable',
    'parent-desktop-busy-ai-provider:busy-job-degraded',
  ]) {
    assertArrayIncludes(proof.assertions, expected, 'LAN AI provider assertion');
  }
  proofLabels.push('v0.9.lan-ai-provider-current-states-gathered');
}

function assertProofMatrix(matrix) {
  assertArrayIncludes(matrix.requiredCompletedClaimIds, claimId, 'proof matrix required claim');
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === claimId);
  if (!scenario) {
    throw new Error(`Proof matrix is missing ${claimId} checkpoint scenario.`);
  }
  assertArrayIncludes(scenario.ciCommands, command, 'proof matrix scenario command');
  const claim = matrix.claims.find((candidate) => candidate.id === claimId);
  if (!claim) {
    throw new Error(`Proof matrix is missing ${claimId} claim.`);
  }
  assertArrayIncludes(claim.ciProof.commands, command, 'proof matrix claim command');
  assertEqual(
    claim.runtimeSurfaceCoverage.physicalHouseholdLan.state,
    'manual-required',
    'physical household LAN matrix state'
  );
  assertEqual(claim.runtimeSurfaceCoverage.cloudRelay.state, 'not-implemented', 'cloud relay matrix state');
  proofLabels.push('proof-matrix.household-lan-readiness-gate');
}

function serviceSummaries(proof) {
  return (proof.services ?? []).map((service) => ({
    label: service.label,
    port: service.port,
    childDeviceId: service.childDeviceId,
    surface: service.surface,
    deviceRoles: service.deviceRoles,
    providerOptIn: service.providerOptIn,
    providerBusy: service.providerBusy,
    providerCapabilities: service.providerCapabilities,
    registryPersistence: service.registryPersistence,
  }));
}

function pickAssertions(assertions, suffixes) {
  return assertions.filter((assertion) => suffixes.some((suffix) => assertion.endsWith(suffix)));
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
