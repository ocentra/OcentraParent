import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-household-lan-product-proof');
const proofPath = join(outputDir, 'proof.json');
const readinessProofPath = join(repoRoot, 'test-results', 'v0-9-household-lan-proof-readiness', 'proof.json');
const productionProofPath = join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json');
const discoveryProofPath = join(repoRoot, 'test-results', 'v0-9-lan-discovery-challenge-mvp', 'proof.json');
const pairingProofPath = join(repoRoot, 'test-results', 'v0-9-lan-pairing-control-mvp', 'proof.json');
const providerProofPath = join(repoRoot, 'test-results', 'platform-roles-lan-ai-provider-pool', 'proof.json');

const commands = [];
const proofLabels = [];
const schemaVersion = 'v0.9';
const routeId = 'route-v0-9-household-lan-product-proof';
const sensitiveEvidenceMarkers = [
  'activity.sqlite',
  'activity.ndjson',
  'decryptedEvidence',
  'journalPath',
  'rawEvidence',
  'rawProofSecret',
  'rawToken',
  'registryPath',
  'sqlitePath',
];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-household-lan-proof-readiness.mjs']);

  const readinessProof = await readJson(readinessProofPath);
  const productionProof = await readJson(productionProofPath);
  const discoveryProof = await readJson(discoveryProofPath);
  const pairingProof = await readJson(pairingProofPath);
  const providerProof = await readJson(providerProofPath);

  assertReadinessProof(readinessProof);
  assertRouteAssertions(discoveryProof, pairingProof, productionProof);
  assertProviderAssertions(providerProof);

  const readModel = {
    schemaVersion,
    checkedAt: new Date().toISOString(),
    productReadinessDecision: 'not-ready-for-product-ready-household-lan-claim',
    localMultiServiceProofState: 'ci-mechanical-proof',
    physicalHouseholdLanProofState: 'manual-required',
    parentMobileControllerProofState: 'manual-required',
    cloudRelayState: 'not-implemented',
    selectedRouteEvidence: selectedRouteEvidence(),
    selectedProviderPolicyEvidence: selectedProviderPolicyEvidence(),
    manualProofGates: manualProofGates(),
  };

  assertReadModel(readModel);
  await validateBuiltContractWhenAvailable(readModel);

  const proof = {
    schemaVersion: 1,
    checkedAt: readModel.checkedAt,
    commit: await gitHead(),
    proofMode: 'household-lan-product-proof',
    commands,
    proofLabels,
    evidence: {
      householdReadiness: relativePath(readinessProofPath),
      productionLanMultidevice: relativePath(productionProofPath),
      discoveryChallenge: relativePath(discoveryProofPath),
      pairingControl: relativePath(pairingProofPath),
      lanAiProviderPool: relativePath(providerProofPath),
    },
    productReadinessDecision: readModel.productReadinessDecision,
    localMultiServiceProofState: readModel.localMultiServiceProofState,
    physicalHouseholdLanProofState: readModel.physicalHouseholdLanProofState,
    parentMobileControllerProofState: readModel.parentMobileControllerProofState,
    cloudRelayState: readModel.cloudRelayState,
    householdProductReadModel: readModel,
    claimsProvedByThisProof: [
      'local direct WebSocket proof artifacts are converted into a product read model without claiming physical household LAN readiness',
      'failed unpaired, wrong-origin, wrong-device, replayed, revoked, stale, and offline selected-route paths are present in product evidence',
      'LAN AI provider routing evidence distinguishes authorized result, unsupported capability, busy, degraded, unavailable, and route-blocked states',
      'manual physical household LAN, real mobile package, real provider host, and cloud relay gates remain explicit product blockers',
    ],
    claimsNotProvedByThisProof: [
      'two physical household LAN devices were exercised',
      'real Android or iOS parent mobile controller packages were exercised',
      'cloud relay routing or authentication exists',
      'router, firewall, NAT, or local network permission behavior outside local CI services is product-ready',
    ],
  };

  assertNoSensitiveEvidenceMarkers(proof);
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-9-household-lan-product-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function selectedRouteEvidence() {
  return [
    routeEvidence('paired', 'paired', 'online', null, 'first-child-agent:route-selected'),
    routeEvidence('unavailable', 'unpaired', 'online', 'anonymous', 'first-child-agent:anonymous-rejected'),
    routeEvidence(
      'unavailable',
      'unpaired',
      'online',
      'unselected-device',
      'first-child-agent:unselected-control-rejected'
    ),
    routeEvidence('unavailable', 'paired', 'online', 'wrong-origin', 'wrong-origin-websocket-rejected-before-upgrade'),
    routeEvidence('unavailable', 'paired', 'online', 'wrong-device', 'wrong-agent-port-rejected-as-wrong-device'),
    routeEvidence('unavailable', 'paired', 'online', 'replayed', 'first-child-agent:replay-rejected'),
    routeEvidence('revoked', 'revoked', 'online', 'revoked', 'first-child-agent:revoked-control-rejected'),
    routeEvidence('stale', 'paired', 'stale', 'stale', 'rust-service:selected-device-stale-control-rejected'),
    routeEvidence('offline', 'paired', 'offline', 'offline', 'rust-service:selected-device-offline-control-rejected'),
  ];
}

function selectedProviderPolicyEvidence() {
  return [
    providerEvidence(
      'authorized-result',
      'paired',
      'online',
      null,
      'parent-desktop-controller-ai-provider:controller-job-completed-observer-job-rejected'
    ),
    providerEvidence(
      'unsupported-capability',
      'paired',
      'online',
      'lan-ai-job-unauthorized',
      'parent-desktop-controller-ai-provider:unsupported-capability-rejected'
    ),
    providerEvidence('busy', 'paired', 'online', null, 'parent-desktop-busy-ai-provider:busy-job-degraded'),
    providerEvidence('degraded', 'paired', 'online', null, 'parent-desktop-degraded-ai-provider:degraded-job-degraded'),
    providerEvidence('unavailable', 'unpaired', 'online', 'anonymous', 'first-child-agent:anonymous-rejected'),
    providerEvidence(
      'unavailable',
      'paired',
      'online',
      'wrong-origin',
      'wrong-origin-websocket-rejected-before-upgrade'
    ),
    providerEvidence('unavailable', 'paired', 'online', 'wrong-device', 'wrong-agent-port-rejected-as-wrong-device'),
    providerEvidence('unavailable', 'paired', 'online', 'replayed', 'first-child-agent:replay-rejected'),
    providerEvidence('unavailable', 'revoked', 'online', 'revoked', 'first-child-agent:revoked-control-rejected'),
    providerEvidence('unavailable', 'paired', 'stale', 'stale', 'rust-service:selected-device-stale-control-rejected'),
    providerEvidence(
      'unavailable',
      'paired',
      'offline',
      'offline',
      'rust-service:selected-device-offline-control-rejected'
    ),
  ];
}

function manualProofGates() {
  return [
    manualGate(
      'two-physical-hosts',
      'two distinct parent and child host names or IP addresses on the same household LAN'
    ),
    manualGate('household-router-reachability', 'router or network note proving child service port reachability'),
    manualGate('os-firewall-or-local-network-permission', 'OS firewall or local network permission artifact'),
    manualGate(
      'allowed-origin-on-physical-controller',
      'allowed origin recorded from the physical parent controller host'
    ),
    manualGate(
      'physical-route-selection-and-takeover',
      'physical route selection and takeover results across two devices'
    ),
    manualGate('physical-revocation-and-rejection', 'physical revocation before rejected follow-up control command'),
    manualGate(
      'physical-stale-offline-selected-device',
      'selected child service stopped or paused before rejected command'
    ),
    manualGate('real-mobile-controller-package', 'real Android or iOS parent mobile package controller proof'),
    manualGate('real-mobile-observer-package', 'real Android or iOS parent mobile package observer proof'),
    manualGate('real-lan-ai-provider-host', 'real opted-in LAN AI provider host proof'),
    manualGate(
      'cloud-relay-separate-proof',
      'separate authenticated cloud relay proof before any relay claim',
      'not-implemented'
    ),
  ];
}

function routeEvidence(discoveryState, trustState, reachability, rejectionReason, evidenceLabel) {
  return {
    schemaVersion,
    routeId,
    discoveryState,
    trustState,
    reachability,
    rejectionReason,
    proofState: 'ci-mechanical-proof',
    evidenceLabel,
  };
}

function providerEvidence(
  routingState,
  selectedRouteTrustState,
  selectedDeviceReachability,
  rejectionReason,
  evidenceLabel
) {
  return {
    schemaVersion,
    routeId,
    routingState,
    selectedRouteTrustState,
    selectedDeviceReachability,
    rejectionReason,
    proofState: 'ci-mechanical-proof',
    evidenceLabel,
  };
}

function manualGate(gate, requiredArtifactSummary, state = 'manual-required') {
  return {
    schemaVersion,
    gate,
    state,
    requiredArtifactSummary,
  };
}

function assertReadinessProof(proof) {
  assertEqual(
    proof.productReadinessDecision,
    'not-ready-for-product-ready-household-lan-claim',
    'household readiness decision'
  );
  assertEqual(proof.readinessGate.physicalHouseholdLan.state, 'manual-required', 'physical household LAN state');
  assertEqual(proof.readinessGate.parentMobileControllerObserver.state, 'manual-required', 'mobile package state');
  assertEqual(proof.readinessGate.cloudRelay.state, 'not-implemented', 'cloud relay state');
  proofLabels.push('v0.9.household-product-readiness-stays-blocked');
}

function assertRouteAssertions(discoveryProof, pairingProof, productionProof) {
  for (const expected of [
    'wrong-origin-websocket-rejected-before-upgrade',
    'wrong-agent-port-challenge-rejected-as-wrong-device',
  ]) {
    assertArrayIncludes(discoveryProof.assertions, expected, 'discovery product route evidence');
  }
  for (const expected of [
    'first-child-agent:anonymous-rejected',
    'first-child-agent:unselected-control-rejected',
    'first-child-agent:replay-rejected',
    'first-child-agent:revoked-control-rejected',
    'wrong-agent-port-rejected-as-wrong-device',
  ]) {
    assertArrayIncludes(pairingProof.assertions, expected, 'pairing product route evidence');
  }
  assertArrayIncludes(
    requiredAssertionsFor(productionProof, 'rust-selected-device-state'),
    'rust-service:selected-device-stale-control-rejected',
    'stale selected-device product evidence'
  );
  assertArrayIncludes(
    requiredAssertionsFor(productionProof, 'rust-selected-device-state'),
    'rust-service:selected-device-offline-control-rejected',
    'offline selected-device product evidence'
  );
  proofLabels.push('v0.9.household-product-route-rejections-covered');
}

function assertProviderAssertions(proof) {
  for (const expected of [
    'parent-desktop-controller-ai-provider:controller-job-completed-observer-job-rejected',
    'parent-desktop-controller-ai-provider:unsupported-capability-rejected',
    'parent-mobile-observer-scaffold:provider-unavailable',
    'parent-desktop-busy-ai-provider:busy-job-degraded',
    'parent-desktop-degraded-ai-provider:provider-degraded',
    'parent-desktop-degraded-ai-provider:degraded-job-degraded',
  ]) {
    assertArrayIncludes(proof.assertions, expected, 'provider product policy evidence');
  }
  proofLabels.push('v0.9.household-product-provider-policy-covered');
}

function assertReadModel(readModel) {
  assertArrayIncludes(
    readModel.selectedRouteEvidence.map((entry) => entry.rejectionReason),
    'wrong-origin',
    'selected route wrong-origin evidence'
  );
  assertArrayIncludes(
    readModel.selectedRouteEvidence.map((entry) => entry.rejectionReason),
    'wrong-device',
    'selected route wrong-device evidence'
  );
  assertArrayIncludes(
    readModel.selectedRouteEvidence.map((entry) => entry.rejectionReason),
    'replayed',
    'selected route replay evidence'
  );
  assertArrayIncludes(
    readModel.selectedRouteEvidence.map((entry) => entry.rejectionReason),
    'revoked',
    'selected route revoked evidence'
  );
  assertArrayIncludes(
    readModel.selectedProviderPolicyEvidence.map((entry) => entry.routingState),
    'degraded',
    'provider degraded routing evidence'
  );
  assertArrayIncludes(
    readModel.selectedProviderPolicyEvidence.map((entry) => entry.selectedDeviceReachability),
    'offline',
    'provider offline selected-device evidence'
  );
  assertArrayIncludes(
    readModel.manualProofGates.map((entry) => entry.gate),
    'two-physical-hosts',
    'manual two-host gate'
  );
  assertArrayIncludes(
    readModel.manualProofGates.map((entry) => entry.gate),
    'physical-stale-offline-selected-device',
    'manual stale/offline gate'
  );
  proofLabels.push('v0.9.household-product-read-model-complete');
}

async function validateBuiltContractWhenAvailable(readModel) {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'lan-pairing.js');
  if (!existsSync(modulePath)) {
    return;
  }
  const module = await import(`file:///${modulePath.replaceAll('\\', '/')}`);
  module.LanHouseholdProductProofReadModelSchema.parse(readModel);
  proofLabels.push('v0.9.household-product-contract-parse');
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

function assertNoSensitiveEvidenceMarkers(value) {
  const serialized = JSON.stringify(value);
  for (const marker of sensitiveEvidenceMarkers) {
    if (serialized.includes(marker)) {
      throw new Error(`Product proof includes sensitive marker ${marker}.`);
    }
  }
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
