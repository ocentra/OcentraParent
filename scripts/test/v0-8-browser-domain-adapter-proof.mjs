import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-8-browser-domain-adapter-proof');
const proofPath = join(outputDir, 'proof.json');
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
    'v0-8-browser-domain-adapter-proof',
  ]);
  await runCommand('cargo', [
    'test',
    '-p',
    'ocentra-parent-agent-protocol',
    'enforcement_browser_domain_adapter_proof',
  ]);
  await runCommand('cargo', [
    'test',
    '-p',
    'ocentra-parent-agent-service',
    'enforcement_browser_domain_adapter_proof_read_model',
  ]);
  await runCommand('cargo', [
    'test',
    '-p',
    'ocentra-parent-agent-service',
    'browser_policy_rollback_restores_earlier_persisted_revision',
  ]);

  const { V08BrowserDomainAdapterProofReadModel } =
    await import('../../packages/parent-domain/dist/v0-8-browser-domain-adapter-proof.js');
  const proofMatrix = JSON.parse(await readFile(join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json')));
  const summary = summarizeReadModel(V08BrowserDomainAdapterProofReadModel);

  assertReadModel(V08BrowserDomainAdapterProofReadModel, summary);
  assertProofMatrix(proofMatrix);

  const proof = {
    schemaVersion: 1,
    proofMode: 'v0-8-browser-domain-adapter-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      tsContract: 'packages/parent-domain/src/v0-8-browser-domain-adapter-proof.ts',
      tsContractTest: 'packages/parent-domain/tests/v0-8-browser-domain-adapter-proof.test.ts',
      rustProtocol: 'crates/agent-protocol/src/enforcement_browser_domain_adapter_proof.rs',
      rustProtocolTest: 'crates/agent-protocol/src/enforcement_browser_domain_adapter_proof_tests.rs',
      rustServiceReadModel: 'crates/agent-service/src/enforcement_browser_domain_adapter_proof_read_model.rs',
      rustServiceTest: 'crates/agent-service/src/enforcement_browser_domain_adapter_proof_read_model_tests.rs',
      proofHarness: 'scripts/test/v0-8-browser-domain-adapter-proof.mjs',
      proofMatrix: 'docs/expectations/pre-ai-proof-matrix.json',
      checkpoint: 'docs/checkpoints/v0-8-browser-domain-adapter-proof-2026-05-30.md',
    },
    counts: summary,
    claimsProved: [
      'Windows managed-browser intervention state remains implemented-boundary only for the owned managed-session path',
      'Windows unmanaged browser terminate remains process-scoped pid/name proof only',
      'Windows unmanaged browser warning remains a degraded no-op boundary until notification and browser integration exist',
      'Audit, restart recovery, and browser policy rollback visibility are recorded as service/read-model boundaries',
      'Domain/network blocking, exact managed URL enforcement, unsupported OS, Android, and iOS entries remain manual-required, unavailable, or not-claimed',
    ],
    claimsNotProved: [
      'managed browser exact active-tab URL enforcement',
      'unmanaged browser URL, active tab, title, page, download source, HTTPS content, or intent certainty',
      'host network or domain blocking',
      'broad browser control outside the proved managed-session and process-only boundaries',
      'Linux or macOS browser/domain support',
      'Android VPN/DNS, device-owner, package lifecycle, or managed-profile enforcement',
      'iOS Network Extension, Family Controls, DeviceActivity, signing, TestFlight, or device enforcement',
    ],
    manualProofRequirements: [
      'managed browser active-tab evidence, exact URL apply, rollback, and audit custody artifacts',
      'explicit browser integration for unmanaged active tab, page, title, download, HTTPS content, and intent evidence',
      'host DNS/VPN/filter adapter apply, rollback, and custody evidence before network/domain claims upgrade',
      'Linux and macOS host-specific permission, package, service-manager, adapter apply, rollback, and audit proof',
      'Android VPN/DNS, device-owner or managed-profile, package lifecycle, and real-device proof',
      'iOS Network Extension, Family Controls, DeviceActivity, entitlement, signing, TestFlight, and device proof',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-8-browser-domain-adapter-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function summarizeReadModel(readModel) {
  return {
    entries: readModel.entries.length,
    byPlatform: countBy(readModel.entries.map((entry) => entry.platform)),
    byCapabilityStatus: countBy(readModel.entries.map((entry) => entry.capabilityStatus)),
    byProductClaimState: countBy(readModel.entries.map((entry) => entry.productClaimState)),
    byAdapterExecutionState: countBy(readModel.entries.map((entry) => entry.adapterExecutionState)),
    managedExactUrlClaimed: readModel.entries.filter((entry) => entry.managedExactUrlClaimed).length,
    unmanagedExactUrlClaimed: readModel.entries.filter((entry) => entry.unmanagedExactUrlClaimed).length,
    networkDomainBlockingClaimed: readModel.entries.filter((entry) => entry.networkDomainBlockingClaimed).length,
    broadBrowserControlClaimed: readModel.entries.filter((entry) => entry.broadBrowserControlClaimed).length,
    unsupportedOsClaimed: readModel.entries.filter((entry) => entry.unsupportedOsClaimed).length,
  };
}

function assertReadModel(readModel, summary) {
  assertEqual(readModel.readModelId, 'v0-8-browser-domain-adapter-proof', 'read model id');
  assertEqual(summary.entries, 14, 'entry count');
  assertEqual(summary.byPlatform.windows, 10, 'Windows entry count');
  assertEqual(summary.byPlatform.linux, 1, 'Linux entry count');
  assertEqual(summary.byPlatform.macos, 1, 'macOS entry count');
  assertEqual(summary.byPlatform.android, 1, 'Android entry count');
  assertEqual(summary.byPlatform.ios, 1, 'iOS entry count');
  assertEqual(summary.byProductClaimState['implemented-boundary'], 5, 'implemented-boundary count');
  assertEqual(summary.byProductClaimState['degraded-boundary'], 1, 'degraded-boundary count');
  assertEqual(summary.byProductClaimState['manual-required'], 4, 'manual-required count');
  assertEqual(summary.byProductClaimState.unavailable, 3, 'unavailable count');
  assertEqual(summary.byProductClaimState['not-claimed'], 1, 'not-claimed count');
  assertEqual(summary.byAdapterExecutionState['executes-real-service'], 5, 'real-service execution count');
  assertEqual(summary.byAdapterExecutionState['returns-degraded-noop'], 1, 'degraded no-op count');
  assertEqual(summary.byAdapterExecutionState['returns-manual-required'], 4, 'manual-required execution count');
  assertEqual(summary.byAdapterExecutionState['returns-unavailable'], 3, 'unavailable execution count');
  assertEqual(summary.byAdapterExecutionState['not-invoked'], 1, 'not-invoked execution count');
  assertEqual(summary.managedExactUrlClaimed, 0, 'managed exact URL claim count');
  assertEqual(summary.unmanagedExactUrlClaimed, 0, 'unmanaged exact URL claim count');
  assertEqual(summary.networkDomainBlockingClaimed, 0, 'network/domain claim count');
  assertEqual(summary.broadBrowserControlClaimed, 0, 'broad browser control claim count');
  assertEqual(summary.unsupportedOsClaimed, 0, 'unsupported OS claim count');

  const surfaces = new Set(readModel.entries.map((entry) => entry.surface));
  for (const expectedSurface of [
    'windows-managed-browser-intervention-state',
    'windows-managed-browser-exact-url-manual',
    'windows-unmanaged-browser-terminate-boundary',
    'windows-unmanaged-browser-warn-noop',
    'windows-unmanaged-browser-exact-evidence-not-claimed',
    'windows-network-domain-filter-manual',
    'windows-network-domain-adapter-unavailable',
    'windows-audit-visibility-boundary',
    'windows-restart-recovery-visibility-boundary',
    'windows-browser-policy-rollback-visibility',
    'linux-browser-domain-adapter-unavailable',
    'macos-browser-domain-adapter-unavailable',
    'android-browser-domain-adapter-manual',
    'ios-browser-domain-adapter-manual',
  ]) {
    assertSetHas(surfaces, expectedSurface, 'browser/domain surface coverage');
  }

  proofLabels.push('v0.8.browser-domain-adapter.read-model');
  proofLabels.push('v0.8.browser-domain-adapter.no-claim-upgrade');
  proofLabels.push('v0.8.browser-domain-adapter.manual-unsupported-gates');
}

function assertProofMatrix(matrix) {
  const claim = matrix.claims.find((candidate) => candidate.id === 'v0-8-browser-domain-adapter-proof');
  if (claim === undefined) {
    throw new Error('Proof matrix is missing V0.8 browser/domain adapter proof claim.');
  }

  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === 'v0-8-browser-domain-adapter-proof');
  if (scenario === undefined) {
    throw new Error('Proof matrix is missing V0.8 browser/domain adapter proof checkpoint scenario.');
  }

  assertSetHas(
    new Set(matrix.requiredCompletedClaimIds),
    'v0-8-browser-domain-adapter-proof',
    'browser/domain adapter proof claim is required'
  );
  assertSetHas(
    new Set(claim.ciProof.commands),
    'node scripts/test/v0-8-browser-domain-adapter-proof.mjs',
    'browser/domain adapter proof command is matrix-listed'
  );
  assertEqual(
    claim.runtimeSurfaceCoverage.managedBrowserIntervention.state,
    'implemented-boundary',
    'managed browser intervention state'
  );
  assertEqual(
    claim.runtimeSurfaceCoverage.managedBrowserExactUrl.state,
    'manual-required',
    'managed browser exact URL state'
  );
  assertEqual(
    claim.runtimeSurfaceCoverage.unmanagedBrowserWarn.state,
    'degraded-boundary',
    'unmanaged browser warn state'
  );
  assertEqual(
    claim.runtimeSurfaceCoverage.unmanagedBrowserExactEvidence.state,
    'not-claimed',
    'unmanaged browser exact evidence state'
  );
  assertEqual(
    claim.runtimeSurfaceCoverage.networkDomainBlocking.state,
    'manual-required',
    'network/domain manual-required state'
  );
  assertEqual(
    claim.runtimeSurfaceCoverage.unsupportedTargets.state,
    'manual-required',
    'unsupported target manual-required state'
  );
  proofLabels.push('proof-matrix.v0-8-browser-domain-adapter-proof');
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${command} exited with ${code}`))));
    child.once('error', reject);
  });
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

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertSetHas(set, value, label) {
  if (!set.has(value)) {
    throw new Error(`${label}: missing ${value}`);
  }
}
