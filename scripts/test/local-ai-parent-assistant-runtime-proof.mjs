import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'local-ai-parent-assistant-runtime-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'local-ai-parent-assistant-runtime-proof',
    ])
  );
  await runCommand('node', ['scripts/test/activity-parent-assistant-runtime-proof.mjs']);

  const proofContractModule = await import('@ocentra-parent/ai-domain/local-ai-parent-assistant-runtime-proof');
  const proofValuesModule =
    await import('@ocentra-parent/ai-domain/local-ai-parent-assistant-runtime-proof-values');
  assertPublicPackageImports(proofContractModule, proofValuesModule);

  const { LocalAiParentAssistantRuntimeProofReadModel } = proofValuesModule;
  const proofSummary = summarizeReadModel(LocalAiParentAssistantRuntimeProofReadModel);
  assertReadModel(LocalAiParentAssistantRuntimeProofReadModel, proofSummary);

  const proof = {
    schemaVersion: 1,
    proofMode: 'local-ai-parent-assistant-runtime-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      tsContract: 'packages/parent-domain/src/local-ai-parent-assistant-runtime-proof.ts',
      tsReadModelValues: 'packages/parent-domain/src/local-ai-parent-assistant-runtime-proof-values.ts',
      tsContractTest: 'packages/parent-domain/tests/local-ai-parent-assistant-runtime-proof.test.ts',
      providerSchedulerProof: 'packages/parent-domain/src/local-ai-runtime-provider-proof.ts',
      parentAssistantContract: 'packages/parent-domain/src/parent-assistant.ts',
      realRuntimeProofHarness: 'scripts/test/activity-parent-assistant-runtime-proof.mjs',
      proofHarness: 'scripts/test/local-ai-parent-assistant-runtime-proof.mjs',
    },
    counts: proofSummary,
    claimsProved: [
      'local parent-assistant answers use the shared local provider scheduler when local execution is allowed',
      'busy local provider state degrades or queues assistant work without a duplicate runtime load',
      'unavailable local provider state is explicit, cited, and does not invent an answer',
      'child-safety work keeps priority over queued parent-assistant work',
      'assistant action preview and confirm require child-agent contract and never directly write policy or enforce',
    ],
    claimsNotProved: [
      'real API or remote AI provider adapter',
      'portal chat/provider-status UI',
      'model quality, safety classifier quality, or deterministic policy accuracy',
      'child-device action validation or enforcement execution',
      'LAN AI provider pool or cross-device AI job routing',
    ],
    runtimeWorkersStillNeedToWire: [
      'portal chat and provider-status surface',
      'parent confirmation to child-agent validation workflow',
      'real local model artifact setup and quality validation',
      'optional parent-authorized API adapter proof',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`local-ai-parent-assistant-runtime-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function summarizeReadModel(readModel) {
  return {
    entries: readModel.entries.length,
    requirements: readModel.entries.map((entry) => entry.requirement).sort(),
    byProofStatus: countBy(readModel.entries.map((entry) => entry.proofStatus)),
    bySchedulerLifecycle: countBy(readModel.entries.map((entry) => entry.schedulerLifecycle)),
    localProviderSelected: readModel.entries.filter((entry) => entry.localProviderSelected).length,
    apiProviderSelected: readModel.entries.filter((entry) => entry.apiProviderSelected).length,
    remoteAiOptionalRows: readModel.entries.filter((entry) => entry.remoteAiOptional).length,
    childSafetyOrEnforcementAllowedRows: readModel.entries.filter((entry) => entry.childSafetyOrEnforcementUseAllowed)
      .length,
    actionRows: readModel.entries.filter(
      (entry) => entry.actionPreviewResult !== null || entry.actionConfirmResult !== null
    ).length,
  };
}

function assertReadModel(readModel, summary) {
  assertEqual(summary.entries, 6, 'runtime proof entry count');
  assertEqual(summary.byProofStatus.proved, 3, 'proved runtime proof count');
  assertEqual(summary.byProofStatus.degraded, 1, 'degraded runtime proof count');
  assertEqual(summary.byProofStatus.unavailable, 1, 'unavailable runtime proof count');
  assertEqual(summary.byProofStatus['not-claimed'], 1, 'not-claimed runtime proof count');
  assertEqual(summary.localProviderSelected, 3, 'local provider selected rows');
  assertEqual(summary.apiProviderSelected, 0, 'api provider selected rows');
  assertEqual(summary.remoteAiOptionalRows, 6, 'remote AI optional rows');
  assertEqual(summary.childSafetyOrEnforcementAllowedRows, 0, 'child-safety or enforcement allowed rows');
  assertEqual(summary.actionRows, 1, 'action contract boundary rows');

  const requirements = new Set(readModel.entries.map((entry) => entry.requirement));
  for (const requirement of [
    'local-provider-answer-uses-shared-runtime',
    'busy-provider-degrades-without-extra-runtime',
    'provider-unavailable-is-explicit-and-cited',
    'child-safety-priority-keeps-assistant-queued',
    'api-provider-remains-optional-parent-authorized-boundary',
    'action-preview-confirm-requires-child-contract',
  ]) {
    assertSetHas(requirements, requirement, 'runtime proof requirement');
  }

  proofLabels.push('local-ai.parent-assistant.shared-provider-runtime');
  proofLabels.push('local-ai.parent-assistant.degraded-queued-unavailable');
  proofLabels.push('local-ai.parent-assistant.api-optional-custody');
  proofLabels.push('local-ai.parent-assistant.action-contract-boundary');
}

function assertPublicPackageImports(proofContractModule, proofValuesModule) {
  assertModuleExport(
    proofContractModule,
    'LocalAiParentAssistantRuntimeProofReadModelSchema',
    'public proof read-model schema export'
  );
  assertModuleExport(
    proofContractModule,
    'decodeLocalAiParentAssistantRuntimeProofReadModel',
    'public proof read-model decoder export'
  );
  assertModuleExport(
    proofValuesModule,
    'LocalAiParentAssistantRuntimeProofReadModel',
    'public proof read-model values export'
  );
  proofLabels.push('local-ai.parent-assistant.public-package-imports');
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${command} ${args.join(' ')} exited with ${code}`))
    );
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

function assertModuleExport(module, exportName, label) {
  if (!(exportName in module)) {
    throw new Error(`${label}: missing ${exportName}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
