import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-mobile-controller-observer-runtime-proof');
const proofPath = join(outputDir, 'proof.json');
const parentMobileProofPath = join(repoRoot, 'test-results', 'parent-mobile-shell-runtime-proof', 'proof.json');
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
const proofMatrixPath = join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json');
const checkpointPath = join(
  repoRoot,
  'docs',
  'checkpoints',
  'v0-9-mobile-controller-observer-runtime-proof-2026-05-29.md'
);
const proofCommand = 'node scripts/test/v0-9-mobile-controller-observer-runtime-proof.mjs';
const proofClaimId = 'v0-9-mobile-controller-observer-runtime-proof';
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build:contracts']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/v0-9-mobile-controller-observer-runtime.test.ts',
  ]);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-mobile-controller-discovery-runtime-proof.mjs']);

  const parentMobileProof = await readJson(parentMobileProofPath);
  const productionMobileProof = await readJson(productionMobileProofPath);
  const discoveryRuntimeProof = await readJson(discoveryRuntimeProofPath);
  const proofMatrix = await readJson(proofMatrixPath);
  const runtimeReadModel = await parseRuntimeReadModel(
    buildRuntimeReadModel(parentMobileProof, productionMobileProof, discoveryRuntimeProof)
  );

  assertParentMobileProof(parentMobileProof);
  assertProductionMobileProof(productionMobileProof);
  assertDiscoveryRuntimeProof(discoveryRuntimeProof);
  assertRuntimeReadModel(runtimeReadModel);
  assertProofMatrix(proofMatrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: proofClaimId,
    commands,
    proofLabels,
    evidence: {
      parentMobileRuntime: relative(repoRoot, parentMobileProofPath),
      productionMobileController: relative(repoRoot, productionMobileProofPath),
      discoveryRuntime: relative(repoRoot, discoveryRuntimeProofPath),
      proofMatrix: relative(repoRoot, proofMatrixPath),
      checkpoint: relative(repoRoot, checkpointPath),
      output: relative(repoRoot, proofPath),
    },
    runtimeReadModel,
    claimsProved: [
      'Parent mobile observer read-only operations are typed separately from rejected write operations.',
      'Controller takeover remains manual-required for mobile packages while backend release proof remains local-service-owned.',
      'LAN AI job submission from parent mobile is represented as degraded provider state, not phone-local model execution.',
      'Package readiness records manual mobile proof gaps for signing, stores, notifications, background behavior, and controller authority.',
      'The proof harness composes parent mobile shell, production mobile controller, and discovery runtime proof artifacts.',
    ],
    claimsNotProved: [
      'real Android or iOS parent mobile controller write authority',
      'physical household LAN router or firewall behavior',
      'cloud relay routing, authentication, or storage',
      'Android child device-owner behavior or iOS Family Controls behavior',
      'C-owned UI rendering or vendor visual changes',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-9-mobile-controller-observer-runtime-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function buildRuntimeReadModel(parentMobileProof, productionMobileProof, discoveryRuntimeProof) {
  return {
    schemaVersion: 'v0.9-mobile-controller-observer-runtime',
    cloudRelayState: productionMobileProof.manualProofGates.cloudRelay.state,
    mobileReadModels: [
      buildMobileReadModel('android', parentMobileProof.runtimeProof.androidObserver),
      buildMobileReadModel('ios', parentMobileProof.runtimeProof.iosObserver),
    ],
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
          'v0-9-mobile-controller-discovery-runtime-proof',
          'test-results/v0-9-mobile-controller-discovery-runtime-proof/proof.json',
          'node scripts/test/v0-9-mobile-controller-discovery-runtime-proof.mjs'
        ),
      ],
      outputProofPath: 'test-results/v0-9-mobile-controller-observer-runtime-proof/proof.json',
      checkpointPath: 'docs/checkpoints/v0-9-mobile-controller-observer-runtime-proof-2026-05-29.md',
    },
    claimBoundaries: {
      parentMobileWriteAuthority: discoveryRuntimeProof.runtimeReadModel.claimBoundaries.parentMobileWriteAuthority,
      physicalHouseholdLan: discoveryRuntimeProof.runtimeReadModel.claimBoundaries.physicalHouseholdLan,
      cloudRelay: discoveryRuntimeProof.runtimeReadModel.claimBoundaries.cloudRelay,
      childAgentBehavior: discoveryRuntimeProof.runtimeReadModel.claimBoundaries.mobileChildAgentBehavior,
      signingStoresEntitlements: discoveryRuntimeProof.runtimeReadModel.claimBoundaries.storesSigningEntitlements,
      cUiOwnership: 'C UI can render this contract later; this proof does not touch C UI or vendor paths',
    },
    updatedAt: discoveryRuntimeProof.checkedAt,
  };
}

function buildMobileReadModel(platform, mobileSummary) {
  return {
    platform,
    parentDeviceId: platform === 'android' ? 'parent-mobile-android-observer' : 'parent-mobile-ios-observer',
    role: platform === 'android' ? 'observer' : 'controller-candidate',
    controllerState: mobileSummary.controllerState,
    commandAuthorityState: mobileSummary.commandAuthorityState,
    serviceState: mobileSummary.lanService,
    packageReadiness: {
      packageState: mobileSummary.packageState,
      runtimeState: 'ci-mechanical-proof',
      signingState: 'manual-required',
      storeDistributionState: 'manual-required',
      foregroundOrBackgroundState: 'manual-required',
      notificationState: 'manual-required',
      missingCapabilityProofs: missingCapabilityProofs(mobileSummary.capabilityStates),
    },
    capabilities: capabilityStates(mobileSummary.capabilityStates),
    operationProofs: operationProofs(),
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
    claimBoundary: `${capability} is not upgraded by parent mobile observer runtime proof`,
  }));
}

function operationProofs() {
  return [
    operationProof(
      'observe-status',
      'health-query',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      null,
      'parent-mobile:observer-status-read-model',
      'typed parent mobile shell can read status without control authority'
    ),
    operationProof(
      'preview-policy-draft',
      'rule-query',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      null,
      'parent-mobile:observer-policy-preview-read-model',
      'policy preview is read-only and does not write child runtime state'
    ),
    operationProof(
      'refresh-capabilities',
      'lan-ai-provider-status',
      'completed',
      'allowed-read-only',
      'parent-mobile-shell',
      null,
      'parent-mobile:capability-refresh-read-model',
      'capability refresh only updates observer readiness labels'
    ),
    operationProof(
      'request-controller-takeover',
      'controller-lease-takeover',
      'rejected',
      'manual-required-mobile-package',
      'manual-proof',
      'takeover-denied',
      'first-child-agent:controller-lease-takeover-denied',
      'real Android or iOS package/device proof before parent mobile takeover can be accepted'
    ),
    operationProof(
      'release-controller-lease',
      'controller-lease-release',
      'completed',
      'proved-local-service',
      'agent-service',
      null,
      'first-child-agent:controller-lease-released',
      'backend release transition is covered by local real-service proof, not mobile authority proof'
    ),
    operationProof(
      'submit-lan-ai-job',
      'lan-ai-job-submit',
      'degraded',
      'degraded-provider',
      'lan-ai-provider',
      'lan-ai-provider-unavailable',
      'parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable',
      'LAN AI job submission stays degraded or unavailable until a real mobile package bridge exists'
    ),
    operationProof(
      'write-policy',
      'rule-update',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'first-child-agent:observer-policy-write-rejected',
      'observer mobile surface cannot write rules'
    ),
    operationProof(
      'approve-override',
      'approval-decision',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'first-child-agent:observer-approval-rejected',
      'observer mobile surface cannot approve overrides'
    ),
    operationProof(
      'pair-device',
      'configuration-update',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'first-child-agent:observer-pair-device-rejected',
      'observer mobile surface cannot pair devices'
    ),
    operationProof(
      'revoke-device',
      'configuration-update',
      'rejected',
      'rejected-observer-read-only',
      'agent-service',
      'observer-read-only',
      'first-child-agent:observer-revoke-device-rejected',
      'observer mobile surface cannot revoke devices'
    ),
  ];
}

function operationProof(
  operation,
  intentKind,
  responseState,
  operationState,
  runtimeOwner,
  rejectionReason,
  proofLabel,
  proofRequirement
) {
  return {
    operation,
    intentKind,
    responseState,
    operationState,
    runtimeOwner,
    rejectionReason,
    proofLabel,
    proofRequirement,
    evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
  };
}

async function parseRuntimeReadModel(readModel) {
  const module = await import('@ocentra-parent/parent-domain/v0-9-mobile-controller-observer-runtime');
  proofLabels.push('parent-domain.v0.9-mobile-controller-observer-runtime-parse');
  return module.V09MobileControllerObserverRuntimeReadModelSchema.parse(readModel);
}

function assertParentMobileProof(proof) {
  assertEqual(proof.proof?.schemaVersion ?? proof.schemaVersion, 1, 'parent mobile proof schema version');
  assertEqual(
    proof.runtimeProof.androidObserver.commandAuthorityState,
    'observer-read-only',
    'Android observer authority'
  );
  assertEqual(
    proof.runtimeProof.iosObserver.commandAuthorityState,
    'controller-takeover-manual-required',
    'iOS controller authority'
  );
  assertEqual(proof.runtimeProof.childAgentBehaviorClaim, 'not-claimed', 'child-agent non-claim');
  assertEqual(proof.runtimeProof.localModelExecutionDefault, 'disabled-by-default', 'local model default');
  proofLabels.push('parent-mobile.observer-runtime-source-proof');
}

function assertProductionMobileProof(proof) {
  assertEqual(proof.proofMode, 'v0-9-production-lan-mobile-controller-proof', 'production mobile proof mode');
  assertEqual(
    proof.manualProofGates.parentMobileControllerObserver.state,
    'manual-required',
    'parent mobile controller gate'
  );
  assertEqual(proof.manualProofGates.cloudRelay.state, 'not-implemented', 'cloud relay gate');
  proofLabels.push('v0.9.production-mobile-controller-gates');
}

function assertDiscoveryRuntimeProof(proof) {
  assertEqual(proof.proofMode, 'v0-9-mobile-controller-discovery-runtime-proof', 'discovery runtime proof mode');
  assertEqual(
    proof.runtimeReadModel.householdDiscovery.physicalHouseholdLanState,
    'manual-required',
    'discovery runtime physical LAN gate'
  );
  assertEqual(
    proof.runtimeReadModel.mobileRouteReadModels[0].commandAuthorityState,
    'observer-read-only',
    'discovery runtime Android observer authority'
  );
  proofLabels.push('v0.9.discovery-runtime-source-proof');
}

function assertRuntimeReadModel(readModel) {
  assertEqual(readModel.cloudRelayState, 'not-implemented', 'observer runtime cloud relay');
  assertEqual(readModel.mobileReadModels.length, 2, 'observer runtime platform count');
  for (const readModelEntry of readModel.mobileReadModels) {
    const operationStates = Object.fromEntries(
      readModelEntry.operationProofs.map((proof) => [proof.operation, proof.operationState])
    );
    assertEqual(operationStates['write-policy'], 'rejected-observer-read-only', 'observer write policy state');
    assertEqual(
      operationStates['request-controller-takeover'],
      'manual-required-mobile-package',
      'takeover request state'
    );
    assertEqual(operationStates['release-controller-lease'], 'proved-local-service', 'release state');
    assertEqual(operationStates['submit-lan-ai-job'], 'degraded-provider', 'LAN AI job state');
  }
  proofLabels.push('v0.9.mobile-controller-observer-runtime-read-model');
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
    'test-results/v0-9-mobile-controller-observer-runtime-proof/proof.json',
    'checkpoint scenario proof artifact'
  );
  const claim = matrix.claims.find((candidate) => candidate.id === proofClaimId);
  if (!claim) {
    throw new Error(`Proof matrix is missing ${proofClaimId} claim.`);
  }
  assertArrayIncludes(claim.ciProof.commands, proofCommand, 'claim command');
  assertEqual(
    claim.runtimeSurfaceCoverage.parentMobileWriteAuthority.state,
    'manual-required',
    'matrix parent mobile authority state'
  );
  assertEqual(claim.runtimeSurfaceCoverage.cloudRelay.state, 'not-implemented', 'matrix cloud relay state');
  proofLabels.push('proof-matrix.v0-9-mobile-controller-observer-runtime-proof');
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
