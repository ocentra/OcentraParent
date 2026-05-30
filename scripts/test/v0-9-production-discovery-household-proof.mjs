import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-production-discovery-household-proof');
const proofPath = join(outputDir, 'proof.json');
const productionDiscoveryProofPath = join(repoRoot, 'test-results', 'v0-9-production-discovery-proof', 'proof.json');
const householdProductProofPath = join(repoRoot, 'test-results', 'v0-9-household-lan-product-proof', 'proof.json');
const householdProductionDiscoveryProofPath = join(
  repoRoot,
  'test-results',
  'v0-9-household-lan-production-discovery-proof',
  'proof.json'
);
const productionLanProofPath = join(
  repoRoot,
  'test-results',
  'v0-9-production-lan-multidevice-hardening',
  'proof.json'
);
const matrixPath = join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json');
const command = 'node scripts/test/v0-9-production-discovery-household-proof.mjs';
const claimId = 'v0-9-production-discovery-household-proof';
const commands = [];
const proofLabels = [];
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
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-production-discovery-proof.mjs']);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-household-lan-product-proof.mjs']);

  const productionDiscoveryProof = await readJson(productionDiscoveryProofPath);
  const householdProductProof = await readJson(householdProductProofPath);
  const householdProductionDiscoveryProof = await readJson(householdProductionDiscoveryProofPath);
  const productionLanProof = await readJson(productionLanProofPath);
  const matrix = await readJson(matrixPath);

  const productionDiscoveryState = assertProductionDiscoveryState(
    productionDiscoveryProof,
    householdProductionDiscoveryProof
  );
  const routeCheckSummary = assertHouseholdRouteChecks(householdProductProof);
  const providerPolicySummary = assertProviderPolicy(householdProductProof);
  const manualStates = assertManualStates(productionDiscoveryProof, householdProductProof, productionLanProof);
  const matrixRegistration = assertProofMatrix(matrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-production-discovery-household-proof',
    commands,
    evidence: {
      productionDiscovery: relativePath(productionDiscoveryProofPath),
      householdProduct: relativePath(householdProductProofPath),
      householdProductionDiscovery: relativePath(householdProductionDiscoveryProofPath),
      productionLanMultidevice: relativePath(productionLanProofPath),
      proofMatrix: relativePath(matrixPath),
    },
    proofLabels,
    productionDiscoveryState,
    routeCheckSummary,
    providerPolicySummary,
    manualStates,
    cloudRelayDecision: householdProductProof.cloudRelayDecision,
    matrixRegistration,
    claimsProved: [
      'production discovery states remain explicit for local real-service proof without claiming household router discovery',
      'paired, failed-unpaired, wrong-origin, wrong-device, replay, revocation, stale, offline, unavailable, and manual-required household route checks are machine-checked in the product read model',
      'selected provider policy read-model evidence keeps authorized, unsupported, busy, degraded, unavailable, stale, offline, wrong-origin, wrong-device, replay, and revoked states explicit',
      'cloud relay is explicitly not implemented and requires a separate manual decision plus authenticated relay proof before any relay claim',
    ],
    claimsNotProved: [
      'physical household LAN product readiness across two real devices',
      'router discovery, firewall prompt handling, NAT behavior, or OS local-network permission behavior',
      'real Android or iOS parent mobile controller write authority or background LAN behavior',
      'cloud relay routing, storage, authentication, or runtime implementation',
    ],
  };

  assertNoSensitiveEvidenceMarkers(proof);
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-9-production-discovery-household-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function assertProductionDiscoveryState(productionDiscoveryProof, householdProductionDiscoveryProof) {
  for (const expected of [
    'v0.9.selected-route.trust-state-explicit',
    'v0.9.production-discovery.local-two-service-proof',
    'v0.9.production-discovery.household-non-claims-preserved',
  ]) {
    assertArrayIncludes(productionDiscoveryProof.proofLabels, expected, 'production discovery proof labels');
  }
  for (const expected of ['discovered', 'pending', 'paired', 'revoked', 'stale', 'offline', 'unavailable']) {
    assertObjectHasKey(
      householdProductionDiscoveryProof.localServiceBoundary.productionDiscoveryStates,
      expected,
      'household production discovery states'
    );
  }
  proofLabels.push('v0.9.production-discovery.states-explicit');
  return {
    proofState: productionDiscoveryProof.localMultiServiceProof.state,
    productionDiscoveryStates: householdProductionDiscoveryProof.localServiceBoundary.productionDiscoveryStates,
    selectedRouteTrust: productionDiscoveryProof.selectedRouteTrust.state,
  };
}

function assertHouseholdRouteChecks(householdProductProof) {
  const readModel = householdProductProof.householdProductReadModel;
  assertEqual(readModel.productReadinessDecision, 'not-ready-for-product-ready-household-lan-claim', 'readiness');
  const checks = readModel.routeCheckOutcomes.map((entry) => entry.check);
  for (const expected of [
    'paired-route-accepted',
    'failed-unpaired-rejected',
    'wrong-origin-rejected',
    'wrong-device-rejected',
    'replay-rejected',
    'revocation-rejected',
    'stale-selected-device-rejected',
    'offline-selected-device-rejected',
    'unavailable-route-rejected',
    'manual-required-physical-household-lan',
  ]) {
    assertArrayIncludes(checks, expected, 'household route check outcomes');
  }
  assertArrayIncludes(
    readModel.routeCheckOutcomes.map((entry) => entry.proofState),
    'manual-required',
    'manual-required route check proof state'
  );
  proofLabels.push('v0.9.household-route-checks.machine-checked');
  return {
    checkCount: checks.length,
    checks,
  };
}

function assertProviderPolicy(householdProductProof) {
  const readModel = householdProductProof.householdProductReadModel;
  for (const expected of ['authorized-result', 'unsupported-capability', 'busy', 'degraded', 'unavailable']) {
    assertArrayIncludes(
      readModel.selectedProviderPolicyEvidence.map((entry) => entry.routingState),
      expected,
      'selected provider routing states'
    );
  }
  for (const expected of ['wrong-origin', 'wrong-device', 'replayed', 'revoked', 'stale', 'offline']) {
    assertArrayIncludes(
      readModel.selectedProviderPolicyEvidence.map((entry) => entry.rejectionReason),
      expected,
      'selected provider rejection states'
    );
  }
  proofLabels.push('v0.9.selected-provider-policy.read-model-evidence');
  return {
    evidenceCount: readModel.selectedProviderPolicyEvidence.length,
    routingStates: Array.from(new Set(readModel.selectedProviderPolicyEvidence.map((entry) => entry.routingState))),
  };
}

function assertManualStates(productionDiscoveryProof, householdProductProof, productionLanProof) {
  assertEqual(householdProductProof.physicalHouseholdLanProofState, 'manual-required', 'physical household LAN state');
  assertEqual(householdProductProof.parentMobileControllerProofState, 'manual-required', 'mobile controller state');
  assertEqual(householdProductProof.cloudRelayState, 'not-implemented', 'cloud relay state');
  assertEqual(householdProductProof.cloudRelayDecision.implementationState, 'not-implemented', 'cloud implementation');
  assertEqual(householdProductProof.cloudRelayDecision.decisionState, 'manual-decision-required', 'cloud decision');
  assertEqual(
    productionDiscoveryProof.householdNonClaim.physicalHouseholdLan,
    'manual-required',
    'household non-claim'
  );
  assertEqual(productionDiscoveryProof.householdNonClaim.cloudRelay, 'not-implemented', 'cloud non-claim');
  assertEqual(productionLanProof.cloudRelayDecision.state, 'not-implemented', 'production LAN cloud relay');
  proofLabels.push('v0.9.household-manual-and-cloud-nonclaims-preserved');
  return {
    physicalHouseholdLan: householdProductProof.physicalHouseholdLanProofState,
    parentMobileController: householdProductProof.parentMobileControllerProofState,
    cloudRelayImplementation: householdProductProof.cloudRelayDecision.implementationState,
    cloudRelayDecision: householdProductProof.cloudRelayDecision.decisionState,
  };
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
  assertEqual(claim.runtimeSurfaceCoverage.productionDiscovery.state, 'ci-mechanical-proof', 'production discovery');
  assertEqual(claim.runtimeSurfaceCoverage.routeChecks.state, 'implemented', 'route checks');
  assertEqual(claim.runtimeSurfaceCoverage.physicalHouseholdLan.state, 'manual-required', 'physical household');
  assertEqual(
    claim.runtimeSurfaceCoverage.cloudRelay.implementationState,
    'not-implemented',
    'cloud relay implementation'
  );
  assertEqual(
    claim.runtimeSurfaceCoverage.cloudRelay.decisionState,
    'manual-decision-required',
    'cloud relay decision'
  );
  proofLabels.push('proof-matrix.v0-9-production-discovery-household-proof');
  return {
    scenario: scenario.id,
    claim: claim.id,
  };
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
      throw new Error(`Production discovery household proof includes sensitive marker ${marker}.`);
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

function assertObjectHasKey(value, expected, label) {
  if (!value || typeof value !== 'object' || !Object.prototype.hasOwnProperty.call(value, expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}
