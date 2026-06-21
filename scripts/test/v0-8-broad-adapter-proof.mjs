import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-8-broad-adapter-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/enforcement-domain',
      '--',
      'v0-8-broad-os-adapter-runtime-proof',
      'v0-8-broad-os-adapter-proof',
    ])
  );
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/browser-domain',
      '--',
      'v0-8-browser-domain-adapter-proof',
    ])
  );
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'enforcement-broad-adapter-proof-adapter',
    ])
  );
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'enforcement_broad_adapter_proof']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'enforcement_broad_adapter_proof']);

  const { V08BroadOsAdapterRuntimeProofReadModel } =
    await import('@ocentra-parent/schema-domain/v0-8-broad-os-adapter-runtime-proof');
  const summary = summarizeReadModel(V08BroadOsAdapterRuntimeProofReadModel);

  assertReadModel(V08BroadOsAdapterRuntimeProofReadModel, summary);

  const proof = {
    schemaVersion: 1,
    proofMode: 'v0-8-broad-adapter-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      tsRuntimeContract: 'packages/schema-domain/src/v0-8-broad-os-adapter-runtime-proof.ts',
      tsRuntimeContractTest: 'packages/enforcement-domain/tests/unit/v0-8-broad-os-adapter-runtime-proof.test.ts',
      tsProtocolAdapter: 'packages/agent-protocol-domain/src/enforcement-broad-adapter-proof-adapter.ts',
      tsProtocolAdapterTest: 'packages/agent-protocol-domain/tests/unit/enforcement-broad-adapter-proof-adapter.test.ts',
      rustProtocol: 'crates/agent-protocol/src/enforcement_broad_adapter_proof.rs',
      rustProtocolTest: 'crates/agent-protocol/src/enforcement_broad_adapter_proof_tests.rs',
      rustServiceReadModel: 'crates/agent-service/src/enforcement_api/enforcement_broad_adapter_proof_read_model.rs',
      rustServiceTest: 'crates/agent-service/src/enforcement_api/enforcement_broad_adapter_proof_read_model_tests.rs',
      rustServiceCommand: 'agent.enforcement.broad-adapter-proof.get',
      rustServiceEvent: 'agent.enforcement.broad-adapter-proof.reported',
      proofHarness: 'scripts/test/v0-8-broad-adapter-proof.mjs',
    },
    counts: summary,
    claimsProved: [
      'Broad adapter runtime proof is contract-backed across TypeScript and Rust protocol structs',
      'Service WebSocket command returns a ten-entry broad adapter proof read model',
      'Owned process and managed browser session are implemented-boundary only',
      'Broad installed app, network/domain, managed exact URL, macOS, Android, and iOS stay manual-required',
      'Linux support is unavailable and unmanaged browser exact evidence is not-claimed',
      'No broad app, network/domain, exact URL, unsupported platform, or mobile privilege claim flag is upgraded',
    ],
    claimsNotProved: [
      'global installed-app blocking',
      'host network or domain blocking',
      'managed active-tab exact URL enforcement',
      'unmanaged browser exact URL evidence',
      'Linux, macOS, Android, or iOS enforcement support',
      'admin hardening, anti-tamper, signing, stores, entitlements, or device-owner policy',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-8-broad-adapter-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function summarizeReadModel(readModel) {
  return {
    entries: readModel.entries.length,
    byPlatform: countBy(readModel.entries.map((entry) => entry.platform)),
    byProductClaimState: countBy(readModel.entries.map((entry) => entry.productClaimState)),
    byEvidenceState: countBy(readModel.entries.map((entry) => entry.evidenceState)),
    broadInstalledAppBlockingClaimed: readModel.entries.filter((entry) => entry.broadInstalledAppBlockingClaimed)
      .length,
    networkDomainBlockingClaimed: readModel.entries.filter((entry) => entry.networkDomainBlockingClaimed).length,
    managedBrowserExactUrlClaimed: readModel.entries.filter((entry) => entry.managedBrowserExactUrlClaimed).length,
    unmanagedBrowserExactEvidenceClaimed: readModel.entries.filter(
      (entry) => entry.unmanagedBrowserExactEvidenceClaimed
    ).length,
    unsupportedPlatformClaimed: readModel.entries.filter((entry) => entry.unsupportedPlatformClaimed).length,
    mobilePrivilegeClaimed: readModel.entries.filter((entry) => entry.mobilePrivilegeClaimed).length,
  };
}

function assertReadModel(readModel, summary) {
  assertEqual(readModel.readModelId, 'v0-8-broad-os-adapter-runtime-proof', 'read model id');
  assertEqual(summary.entries, 10, 'entry count');
  assertEqual(summary.byPlatform.windows, 6, 'Windows entry count');
  assertEqual(summary.byPlatform.linux, 1, 'Linux entry count');
  assertEqual(summary.byPlatform.macos, 1, 'macOS entry count');
  assertEqual(summary.byPlatform.android, 1, 'Android entry count');
  assertEqual(summary.byPlatform.ios, 1, 'iOS entry count');
  assertEqual(summary.byProductClaimState['implemented-boundary'], 2, 'implemented-boundary count');
  assertEqual(summary.byProductClaimState['manual-required'], 6, 'manual-required count');
  assertEqual(summary.byProductClaimState.unavailable, 1, 'unavailable count');
  assertEqual(summary.byProductClaimState['not-claimed'], 1, 'not-claimed count');
  assertEqual(summary.byEvidenceState['composite-runtime-proof'], 2, 'composite-runtime-proof count');
  assertEqual(summary.byEvidenceState['manual-artifact-required'], 6, 'manual-artifact-required count');
  assertEqual(summary.byEvidenceState['target-unavailable'], 1, 'target-unavailable count');
  assertEqual(summary.byEvidenceState['not-implemented'], 1, 'not-implemented count');
  assertEqual(summary.broadInstalledAppBlockingClaimed, 0, 'broad installed app claim count');
  assertEqual(summary.networkDomainBlockingClaimed, 0, 'network/domain claim count');
  assertEqual(summary.managedBrowserExactUrlClaimed, 0, 'managed exact URL claim count');
  assertEqual(summary.unmanagedBrowserExactEvidenceClaimed, 0, 'unmanaged exact evidence claim count');
  assertEqual(summary.unsupportedPlatformClaimed, 0, 'unsupported platform claim count');
  assertEqual(summary.mobilePrivilegeClaimed, 0, 'mobile privilege claim count');

  for (const sourceReadModelId of [
    'v0-8-broad-os-adapter-proof',
    'v0-8-browser-domain-adapter-proof',
    'v0-8-os-adapter-manual-artifact-gates',
    'v0-8-os-adapter-product-proof',
  ]) {
    assertSetHas(new Set(readModel.sourceReadModelIds), sourceReadModelId, 'source read model ids');
  }

  assertEntry(readModel, 'windows-owned-process-and-timer-runtime-boundary', {
    productClaimState: 'implemented-boundary',
    evidenceState: 'composite-runtime-proof',
  });
  assertEntry(readModel, 'windows-network-domain-runtime-gate', {
    productClaimState: 'manual-required',
    evidenceState: 'manual-artifact-required',
  });
  assertEntry(readModel, 'windows-unmanaged-browser-exact-evidence-runtime-gap', {
    productClaimState: 'not-claimed',
    evidenceState: 'not-implemented',
  });
  assertEntry(readModel, 'linux-host-runtime-unavailable', {
    productClaimState: 'unavailable',
    evidenceState: 'target-unavailable',
  });

  proofLabels.push('v0.8.broad-adapter-proof.service-command');
  proofLabels.push('v0.8.broad-adapter-proof.runtime-read-model');
  proofLabels.push('v0.8.broad-adapter-proof.no-claim-upgrade');
  proofLabels.push('v0.8.broad-adapter-proof.platform-boundaries');
}

function assertEntry(readModel, proofEntryId, expected) {
  const entry = readModel.entries.find((candidate) => candidate.proofEntryId === proofEntryId);
  if (entry === undefined) {
    throw new Error(`missing broad adapter proof entry ${proofEntryId}`);
  }
  assertEqual(entry.productClaimState, expected.productClaimState, `${proofEntryId} product claim state`);
  assertEqual(entry.evidenceState, expected.evidenceState, `${proofEntryId} evidence state`);
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
