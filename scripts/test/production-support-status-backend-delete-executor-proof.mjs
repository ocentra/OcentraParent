import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const resultDir = join(repoRoot, 'test-results', 'production-support-status-backend-delete-executor-proof');
const outputDir = join(repoRoot, 'output', 'production-support-status-backend-delete-executor-proof');
const proofPath = join(resultDir, 'proof.json');
const summaryPath = join(outputDir, 'proof-summary.json');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/logging-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/logging-domain',
    '--',
    'tests/status-backend-delete-executor.test.ts',
  ]);

  const commit = await gitHead();
  const readModel = await parseReadModel();
  assertReadModel(readModel);
  await assertPackageExports();

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode: 'production-support-status-backend-delete-executor-proof',
    commands,
    evidence: {
      contract: 'packages/logging-domain/src/status-backend-delete-executor.ts',
      guards: 'packages/logging-domain/src/status-backend-delete-executor-guards.ts',
      readModel: 'packages/logging-domain/src/status-backend-delete-executor-read-model.ts',
      contractTest: 'packages/logging-domain/tests/status-backend-delete-executor.test.ts',
      proofOutput: relative(repoRoot, proofPath),
      summaryOutput: relative(repoRoot, summaryPath),
      featureDoc: 'docs/features/production-distribution-support.md',
      expectations: ['docs/expectations/data-custody.md', 'docs/expectations/release-installer.md'],
    },
    claimsProved: [
      'Status backend delete executor rows are parent-consented and redaction-backed before request, authorization, queue, running, manual deletion, failure, audit export, or backend-unavailable states are accepted.',
      'Rows link to status backend target, execution queue, queue audit, custody, delete request, delete executor, redaction, and manual proof references while keeping payloads to support-safe delete status refs only.',
      'Delete executor dispatch, running execution, payload deletion, failure replay, and audit export remain manual-required until real runtime proof exists.',
      'Backend-unavailable rows prove the fallback status remains not-executed and not-requested rather than claiming status backend delete execution.',
      'Package exports expose the status backend delete executor contract and read model through @ocentra-parent/logging-domain.',
      'Rows reject tokens, raw child activity, raw support bundles, provider secrets, account lookup results, billing contact records, backend upload payloads, status backend payloads, public runtime payloads, remote support transcripts, status backend execution, durable payload storage, delete executor execution, payload deletion execution, retry worker execution, audit persistence, public runtime execution, support upload execution, provider execution, account lookup execution, billing provider contact, remote support sessions, production SLA, and default Ocentra-hosted family data.',
    ],
    claimsNotProved: [
      'real status backend execution',
      'durable status backend payload storage',
      'status backend delete executor execution',
      'status backend payload deletion execution',
      'retry worker execution',
      'audit persistence execution',
      'public runtime execution',
      'support backend upload execution',
      'provider execution',
      'account lookup execution',
      'billing provider contact execution',
      'remote support session execution',
      'production SLA',
      'default Ocentra-hosted family data',
      'child activity custody',
    ],
    readModel,
  };

  const summary = {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit,
    proofMode: proof.proofMode,
    statesCovered: readModel.entries.map((entry) => entry.deleteExecutorState),
    output: relative(repoRoot, proofPath),
    claimsNotProved: proof.claimsNotProved,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`production-support-status-backend-delete-executor-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function parseReadModel() {
  const modulePath = join(repoRoot, 'packages', 'logging-domain', 'dist', 'status-backend-delete-executor.js');
  const readModelPath = join(
    repoRoot,
    'packages',
    'logging-domain',
    'dist',
    'status-backend-delete-executor-read-model.js'
  );
  const module = await import(pathToFileURL(modulePath).href);
  const readModelModule = await import(pathToFileURL(readModelPath).href);
  return module.StatusBackendDeleteExecutorReadModelSchema.parse(readModelModule.StatusBackendDeleteExecutorReadModel);
}

function assertReadModel(readModel) {
  assert.equal(readModel.readModelId, 'production-support-status-backend-delete-executor-proof');
  assert.equal(readModel.entries.length, 8);
  const states = new Set(readModel.entries.map((entry) => entry.deleteExecutorState));
  for (const state of [
    'delete-request-recorded',
    'delete-executor-authorized',
    'delete-executor-queued',
    'delete-executor-running',
    'deletion-manual-required',
    'delete-executor-failed',
    'audit-export-ready',
    'backend-unavailable',
  ]) {
    assert.equal(states.has(state), true);
  }

  const unavailable = entryFor(readModel, 'status-backend-delete-executor-backend-unavailable');
  assert.equal(unavailable.executorExecutionState, 'not-executed');
  assert.equal(unavailable.payloadDeletionState, 'not-requested');
  assert.equal(unavailable.realStatusBackendExecution, false);
  assert.equal(unavailable.statusBackendDeleteExecutorExecuted, false);
  assert.equal(unavailable.statusBackendPayloadDeletionExecuted, false);
}

async function assertPackageExports() {
  const contract = await import('@ocentra-parent/logging-domain/status-backend-delete-executor');
  const readModel = await import('@ocentra-parent/logging-domain/status-backend-delete-executor-read-model');
  assert.equal(typeof contract.StatusBackendDeleteExecutorReadModelSchema.parse, 'function');
  assert.equal(readModel.StatusBackendDeleteExecutorReadModel.entries.length, 8);
}

function entryFor(readModel, executorId) {
  const entry = readModel.entries.find((candidate) => candidate.executorId === executorId);
  assert.notEqual(entry, undefined);
  return entry;
}

async function runCommand(commandName, args) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}`))
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
