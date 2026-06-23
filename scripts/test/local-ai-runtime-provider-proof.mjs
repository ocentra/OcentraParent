import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const resultOutputDir = join(repoRoot, 'test-results', 'local-ai-runtime-provider-proof');
const trackedOutputDir = join(repoRoot, 'output', 'ai-plan-proof', 'local-ai-runtime-provider-proof');
const proofPath = join(resultOutputDir, 'proof.json');
const trackedProofPath = join(trackedOutputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(resultOutputDir, { recursive: true });
  await mkdir(trackedOutputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand(['run', 'test', '--workspace', '@ocentra-parent/ai-domain', '--', 'local-ai-runtime-provider-proof'])
  );
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'local_ai_runtime_provider_proof']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'local_ai_provider_scheduler']);
  await runCommand('cargo', [
    'test',
    '-p',
    'ocentra-parent-agent-service',
    'local_ai_runtime_provider_proof_read_model',
  ]);
  await runCommand('cargo', [
    'test',
    '-p',
    'ocentra-parent-agent-service',
    'parent_assistant_busy_provider_degrades_without_running_or_enforcing',
  ]);

  const { LocalAiRuntimeProviderProofReadModel } =
    await import('@ocentra-parent/schema-domain/local-ai-runtime-provider-proof');
  const proofSummary = summarizeReadModel(LocalAiRuntimeProviderProofReadModel);
  assertReadModel(LocalAiRuntimeProviderProofReadModel, proofSummary);

  const proof = {
    schemaVersion: 1,
    proofMode: 'local-ai-runtime-provider-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      tsContract: 'packages/schema-domain/src/local-ai-runtime-provider-proof.ts',
      tsContractTest: 'packages/ai-domain/tests/unit/local-ai-runtime-provider-proof.test.ts',
      rustProtocol: 'crates/agent-protocol/src/local_ai_runtime_provider_proof.rs',
      rustProtocolTest: 'crates/agent-protocol/src/local_ai_runtime_provider_proof_tests.rs',
      rustServiceReadModel: 'crates/agent-service/src/local_ai_runtime_provider_proof_read_model.rs',
      rustServiceReadModelTest: 'crates/agent-service/src/local_ai_runtime_provider_proof_read_model_tests.rs',
      schedulerServiceTest: 'crates/agent-service/src/local_ai_provider_scheduler_tests.rs',
      parentAssistantRuntimeTest: 'crates/agent-service/src/parent_assistant_runtime_tests.rs',
      proofHarness: 'scripts/test/local-ai-runtime-provider-proof.mjs',
    },
    counts: proofSummary,
    claimsProved: [
      'one ai-provider role per physical device',
      'parent and child roles share the same provider/runtime reference on one physical device',
      'one local model runtime lane admits at most one active generation job',
      'child-safety queued jobs preempt queued parent-assistant jobs',
      'queued, degraded, and unavailable provider states are explicit and schema-valid',
      'parent-assistant work uses the local provider scheduler when local execution is allowed',
      'duplicate local model load is rejected by contract and service proof',
    ],
    claimsNotProved: [
      'LAN AI provider pool or cross-device AI job routing',
      'API or remote AI provider authorization',
      'model quality, model safety classification quality, or deterministic safety decision accuracy',
      'Portal UI rendering of provider state',
    ],
    runtimeWorkersStillNeedToWire: [
      'production child-safety classifier requests into the shared provider lane',
      'portal read surface for provider/scheduler proof state',
      'future LAN AI provider pool routing after same-device singleton proof is integrated',
    ],
  };

  await writeProof(proof);
  console.log(`local-ai-runtime-provider-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
  console.log(`trackedEvidence=${relative(repoRoot, trackedProofPath)}`);
}

async function writeProof(proof) {
  const serialized = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(proofPath, serialized);
  await writeFile(trackedProofPath, serialized);
}

function summarizeReadModel(readModel) {
  return {
    entries: readModel.entries.length,
    requirements: readModel.entries.map((entry) => entry.requirement).sort(),
    byProofStatus: countBy(readModel.entries.map((entry) => entry.proofStatus)),
    bySchedulerLifecycle: countBy(readModel.entries.map((entry) => entry.schedulerLifecycle)),
    maxRuntimeAccessLaneCount: Math.max(...readModel.entries.map((entry) => entry.runtimeAccessLaneCount)),
    minRuntimeAccessLaneCount: Math.min(...readModel.entries.map((entry) => entry.runtimeAccessLaneCount)),
    maxRuntimeLoadCount: Math.max(...readModel.entries.map((entry) => entry.runtimeLoadCount)),
    duplicateRuntimeBlocked: readModel.entries.filter((entry) => entry.duplicateRuntimeBlocked).length,
    childSafetyPriorityProved: readModel.entries.filter((entry) => entry.childSafetyPriorityProved).length,
    parentAssistantSubmissionAllowed: readModel.entries.filter((entry) => entry.parentAssistantSubmissionAllowed)
      .length,
    unavailableReasons: readModel.entries.filter((entry) => entry.unavailableReason !== null).length,
  };
}

function assertReadModel(readModel, summary) {
  assertEqual(summary.entries, 8, 'provider proof entry count');
  assertEqual(summary.byProofStatus.proved, 6, 'proved provider proof count');
  assertEqual(summary.byProofStatus.degraded, 1, 'degraded provider proof count');
  assertEqual(summary.byProofStatus.unavailable, 1, 'unavailable provider proof count');
  assertEqual(summary.maxRuntimeAccessLaneCount, 1, 'maximum runtime access lane count');
  assertEqual(summary.minRuntimeAccessLaneCount, 1, 'minimum runtime access lane count');
  assertEqual(summary.maxRuntimeLoadCount, 1, 'maximum runtime load count');
  assertEqual(summary.childSafetyPriorityProved, 2, 'child safety priority proof flags');
  assertEqual(summary.parentAssistantSubmissionAllowed, 3, 'parent assistant allowed proof flags');
  assertEqual(summary.unavailableReasons, 1, 'unavailable reason count');

  const requirements = new Set(readModel.entries.map((entry) => entry.requirement));
  for (const requirement of [
    'one-ai-provider-role-per-physical-device',
    'shared-parent-child-provider',
    'single-local-runtime-lane',
    'child-safety-priority',
    'queued-degraded-unavailable-lifecycle',
    'parent-assistant-submits-when-allowed',
    'no-duplicate-local-model-load',
    'provider-status-contract-hardening',
  ]) {
    assertSetHas(requirements, requirement, 'provider proof requirement');
  }
  proofLabels.push('local-ai.provider-proof.contract-counts');
  proofLabels.push('local-ai.provider-proof.shared-runtime-singleton');
  proofLabels.push('local-ai.provider-proof.one-runtime-access-lane-per-device');
  proofLabels.push('local-ai.provider-proof.child-safety-priority');
  proofLabels.push('local-ai.provider-proof.lifecycle-boundaries');
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
