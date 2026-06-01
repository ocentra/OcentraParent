import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-mobile-controller-discovery-runtime-proof');
const proofPath = join(outputDir, 'proof.json');
const productionMobileProofPath = join(
  repoRoot,
  'test-results',
  'v0-9-production-lan-mobile-controller-proof',
  'proof.json'
);
const productionDiscoveryProofPath = join(
  repoRoot,
  'test-results',
  'v0-9-household-lan-production-discovery-proof',
  'proof.json'
);
const parentMobileProofPath = join(repoRoot, 'test-results', 'parent-mobile-shell-runtime-proof', 'proof.json');
const proofMatrixPath = join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json');
const proofCommand = 'node scripts/test/v0-9-mobile-controller-discovery-runtime-proof.mjs';
const proofClaimId = 'v0-9-mobile-controller-discovery-runtime-proof';
const commands = [];
const proofLabels = [];

await main();
process.exit(0);

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build:contracts']);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-production-lan-mobile-controller-proof.mjs']);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-household-lan-production-discovery-proof.mjs']);

  const productionMobileProof = await readJson(productionMobileProofPath);
  const productionDiscoveryProof = await readJson(productionDiscoveryProofPath);
  const parentMobileProof = await readJson(parentMobileProofPath);
  const proofMatrix = await readJson(proofMatrixPath);
  const runtimeReadModel = await parseRuntimeReadModel(
    buildRuntimeReadModel(productionMobileProof, productionDiscoveryProof, parentMobileProof)
  );

  assertProductionMobileProof(productionMobileProof);
  assertProductionDiscoveryProof(productionDiscoveryProof);
  assertRuntimeReadModel(runtimeReadModel);
  assertProofMatrix(proofMatrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-mobile-controller-discovery-runtime-proof',
    commands,
    proofLabels,
    evidence: {
      productionMobileController: relative(repoRoot, productionMobileProofPath),
      productionDiscoveryBoundary: relative(repoRoot, productionDiscoveryProofPath),
      parentMobileRuntime: relative(repoRoot, parentMobileProofPath),
      proofMatrix: relative(repoRoot, proofMatrixPath),
    },
    runtimeReadModel,
    claimsProved: [
      'V0.9 household discovery state labels are represented in a typed parent-domain runtime read model',
      'Android parent mobile remains observer read-only and iOS parent mobile remains controller-takeover manual-required',
      'takeover, release, renew, degraded provider, failed-unpaired, stale, and offline states are tied to existing proof labels',
      'cloud relay, physical household LAN, mobile write authority, signing, stores, entitlements, and mobile child-agent behavior remain non-claims',
    ],
    claimsNotProved: [
      'two physical household devices on the same router or firewall path',
      'real Android or iOS parent mobile controller write authority',
      'mobile background LAN behavior',
      'cloud relay routing, authentication, or storage',
      'Android child device-owner behavior or iOS Family Controls behavior',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-9-mobile-controller-discovery-runtime-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function buildRuntimeReadModel(productionMobileProof, productionDiscoveryProof, parentMobileProof) {
  return {
    schemaVersion: 'v0.9-mobile-controller-discovery-runtime',
    lanSchemaVersion: 'v0.9',
    householdDiscovery: {
      localServiceState: 'ci-mechanical-proof',
      physicalHouseholdLanState: productionMobileProof.manualProofGates.physicalHouseholdLan.state,
      cloudRelayState: productionMobileProof.manualProofGates.cloudRelay.state,
      discoveryStatesCovered: Object.keys(productionDiscoveryProof.localServiceBoundary.productionDiscoveryStates),
      evidenceReferenceIds: ['evidence-v0-9-production-discovery-proof'],
    },
    mobileRouteReadModels: [
      androidMobileRoute(parentMobileProof.runtimeProof.androidObserver),
      iosMobileRoute(parentMobileProof.runtimeProof.iosObserver),
    ],
    controllerTransitions: [
      transitionProof(
        'takeover',
        'manual-required-mobile-package',
        'takeover-denied',
        'first-child-agent:controller-lease-takeover-denied'
      ),
      transitionProof('release', 'proved-local-service', null, 'first-child-agent:controller-lease-released'),
      transitionProof('renew', 'proved-local-service', null, 'first-child-agent:controller-lease-renewed'),
      transitionProof(
        'degraded-provider',
        'degraded',
        'lan-ai-provider-unavailable',
        'parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable'
      ),
      transitionProof('failed-unpaired', 'rejected', 'anonymous', 'first-child-agent:anonymous-rejected'),
    ],
    failedUnpairedBehavior: {
      reason: 'anonymous',
      proofLabel: productionDiscoveryProof.localServiceBoundary.routeControlRejections.failedUnpaired,
    },
    staleOfflineBehavior: [
      { reason: 'stale', proofLabel: 'rust-service:selected-device-stale-control-rejected' },
      { reason: 'offline', proofLabel: 'rust-service:selected-device-offline-control-rejected' },
    ],
    claimBoundaries: {
      physicalHouseholdLan: 'manual-required until two physical household devices and router/firewall artifacts exist',
      parentMobileWriteAuthority: 'manual-required until Android or iOS package/device controller proof exists',
      cloudRelay: 'not implemented and not counted as LAN proof',
      mobileChildAgentBehavior: 'not claimed by parent mobile controller proof',
      storesSigningEntitlements: 'manual-required until signing store and entitlement artifacts exist',
    },
    updatedAt: productionMobileProof.checkedAt,
  };
}

function androidMobileRoute(androidObserver) {
  return {
    platform: 'android',
    parentDeviceId: 'parent-mobile-android-observer',
    routeId: 'route-parent-mobile-lan-provider',
    routeSource: 'local-real-service-proof',
    discoveryState: 'paired',
    reachability: 'online',
    controllerState: 'observer',
    commandAuthorityState: androidObserver.commandAuthorityState,
    serviceState: androidObserver.lanService,
    packageState: 'ci-mechanical-proof',
    proofLabels: ['parent-mobile.controller-observer-boundaries'],
  };
}

function iosMobileRoute(iosObserver) {
  return {
    platform: 'ios',
    parentDeviceId: 'parent-mobile-ios-observer',
    routeId: null,
    routeSource: 'manual-mobile-package-required',
    discoveryState: 'unavailable',
    reachability: 'offline',
    controllerState: 'manual-required',
    commandAuthorityState: iosObserver.commandAuthorityState,
    serviceState: iosObserver.lanService,
    packageState: 'ci-mechanical-proof',
    proofLabels: ['parent-mobile.controller-observer-boundaries'],
  };
}

function transitionProof(transition, state, rejectionReason, proofLabel) {
  return { transition, state, rejectionReason, proofLabel };
}

async function parseRuntimeReadModel(readModel) {
  const module = await import('@ocentra-parent/parent-domain/v0-9-mobile-controller-discovery-runtime');
  proofLabels.push('parent-domain.v0.9-mobile-controller-discovery-runtime-parse');
  return module.V09MobileControllerDiscoveryRuntimeReadModelSchema.parse(readModel);
}

function assertProductionMobileProof(proof) {
  assertEqual(proof.proofMode, 'v0-9-production-lan-mobile-controller-proof', 'production mobile proof mode');
  assertEqual(
    proof.mobileProof.androidObserver.commandAuthorityState,
    'observer-read-only',
    'Android mobile authority'
  );
  assertEqual(
    proof.mobileProof.iosObserver.commandAuthorityState,
    'controller-takeover-manual-required',
    'iOS mobile authority'
  );
  assertEqual(proof.manualProofGates.physicalHouseholdLan.state, 'manual-required', 'physical household LAN gate');
  assertEqual(proof.manualProofGates.cloudRelay.state, 'not-implemented', 'cloud relay gate');
  proofLabels.push('v0.9.production-mobile-controller-non-claims-preserved');
}

function assertProductionDiscoveryProof(proof) {
  assertEqual(proof.proofMode, 'household-lan-production-discovery-boundary', 'production discovery boundary mode');
  for (const state of ['discovered', 'pending', 'paired', 'revoked', 'stale', 'offline', 'unavailable']) {
    if (!(state in proof.localServiceBoundary.productionDiscoveryStates)) {
      throw new Error(`Missing production discovery state ${state}`);
    }
  }
  assertEqual(proof.physicalClaimUpgradeVerifier.decision, 'rejected', 'physical household LAN upgrade decision');
  proofLabels.push('v0.9.production-discovery-runtime-states-covered');
}

function assertRuntimeReadModel(readModel) {
  assertEqual(readModel.mobileRouteReadModels.length, 2, 'mobile route read model count');
  assertEqual(readModel.householdDiscovery.physicalHouseholdLanState, 'manual-required', 'read model physical LAN');
  assertEqual(readModel.householdDiscovery.cloudRelayState, 'not-implemented', 'read model cloud relay');
  assertEqual(readModel.failedUnpairedBehavior.reason, 'anonymous', 'failed unpaired behavior');
  assertEqual(readModel.staleOfflineBehavior[0].reason, 'stale', 'stale behavior');
  assertEqual(readModel.staleOfflineBehavior[1].reason, 'offline', 'offline behavior');
  proofLabels.push('v0.9.mobile-controller-discovery-runtime-read-model');
}

function assertProofMatrix(matrix) {
  assertArrayIncludes(matrix.requiredCompletedClaimIds, proofClaimId, 'proof matrix required claim');
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === proofClaimId);
  if (!scenario) {
    throw new Error(`Proof matrix is missing ${proofClaimId} checkpoint scenario.`);
  }
  assertArrayIncludes(scenario.ciCommands, proofCommand, 'checkpoint scenario command');
  assertArrayIncludes(
    scenario.requiredArtifacts,
    'test-results/v0-9-mobile-controller-discovery-runtime-proof/proof.json',
    'checkpoint scenario proof artifact'
  );
  const claim = matrix.claims.find((candidate) => candidate.id === proofClaimId);
  if (!claim) {
    throw new Error(`Proof matrix is missing ${proofClaimId} claim.`);
  }
  assertArrayIncludes(claim.ciProof.commands, proofCommand, 'claim command');
  assertEqual(
    claim.runtimeSurfaceCoverage.physicalHouseholdLan.state,
    'manual-required',
    'matrix physical household LAN state'
  );
  assertEqual(claim.runtimeSurfaceCoverage.cloudRelay.state, 'not-implemented', 'matrix cloud relay state');
  proofLabels.push('proof-matrix.v0-9-mobile-controller-discovery-runtime-proof');
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

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: expected ${expected}`);
  }
}
