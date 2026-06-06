import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'production-support-status-backend-redaction-manifest-proof';
const outputDir = join(repoRoot, 'output', proofMode);
const testResultsDir = join(repoRoot, 'test-results', proofMode);
const proofSummaryPath = join(outputDir, 'proof-summary.json');
const proofPath = join(testResultsDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(testResultsDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/logging-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/logging-domain',
    '--',
    'tests/support-bundle-redaction.test.ts',
  ]);

  const commit = await gitHead();
  const readModel = await parseReadModel();
  assertReadModel(readModel);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    packageExport: '@ocentra-parent/logging-domain/support-bundle-redaction',
    commands,
    evidence: {
      contract: 'packages/logging-domain/src/support-bundle-redaction.ts',
      readModel: 'packages/logging-domain/src/support-bundle-redaction-read-model.ts',
      contractTest: 'packages/logging-domain/tests/support-bundle-redaction.test.ts',
      proofSummary: relative(repoRoot, proofSummaryPath),
      proof: relative(repoRoot, proofPath),
      featureDoc: 'docs/features/production-distribution-support.md',
      expectations: ['docs/expectations/release-installer.md', 'docs/expectations/data-custody.md'],
    },
    claimsProved: [
      'Status backend redaction manifest rows are parent-approved support-safe metadata only.',
      'Status backend redaction manifest rows link queue and audit persistence refs without carrying status backend payloads.',
      'Manual-required status backend redaction rows keep durable payload storage, deletion, retry worker, and audit persistence execution unclaimed.',
      'Support output excludes status backend payloads, public runtime payloads, tokens, child activity, raw URLs, screenshots, journals, SQLite snapshots, private paths, command lines, keystrokes, clipboard data, message contents, and provider secrets.',
    ],
    claimsNotProved: [
      'real status backend execution',
      'durable status backend payload storage',
      'payload deletion execution',
      'retry worker execution',
      'audit persistence execution',
      'public runtime execution',
      'support backend upload execution',
      'provider execution',
      'account lookup execution',
      'billing provider contact',
      'remote support sessions',
      'production SLA',
      'child activity custody',
    ],
    readModel,
  };

  await writeFile(proofSummaryPath, `${JSON.stringify(summarizeProof(proof), null, 2)}\n`);
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`${proofMode}-ok:${relative(repoRoot, proofPath)}`);
}

async function parseReadModel() {
  const modulePath = join(repoRoot, 'packages', 'logging-domain', 'dist', 'support-bundle-redaction.js');
  const readModelPath = join(repoRoot, 'packages', 'logging-domain', 'dist', 'support-bundle-redaction-read-model.js');
  const module = await import(pathToFileURL(modulePath).href);
  const readModelModule = await import(pathToFileURL(readModelPath).href);
  return module.SupportBundleRedactionReadModelSchema.parse(readModelModule.SupportBundleRedactionReadModel);
}

function assertReadModel(readModel) {
  assert.equal(readModel.readModelId, 'support-bundle-redaction-proof');
  const ready = readModel.entries.find(
    (entry) => entry.incidentId === 'support-incident-status-backend-redaction-ready'
  );
  const manual = readModel.entries.find(
    (entry) => entry.incidentId === 'support-incident-status-backend-redaction-manual-required'
  );

  assert.equal(ready.parentConsentState, 'parent-approved');
  assert.deepEqual(ready.statusBackendRefs, [
    'status-backend-execution-queue-ref',
    'status-backend-queue-audit-persistence-ref',
    'status-backend-redaction-manifest-ref',
  ]);
  assert.equal(ready.containsStatusBackendPayload, false);
  assert.equal(ready.publicRuntimePayloadIncluded, false);
  assert.equal(ready.statusBackendExecutionClaimed, false);
  assert.equal(manual.containsStatusBackendPayload, false);
  assert.equal(manual.statusBackendExecutionClaimed, false);
  assert.equal(manual.manualProofRequirements.length, 1);
}

function summarizeProof(proof) {
  return {
    schemaVersion: proof.schemaVersion,
    checkedAt: proof.checkedAt,
    commit: proof.commit,
    proofMode: proof.proofMode,
    packageExport: proof.packageExport,
    commands: proof.commands,
    evidence: proof.evidence,
    claimsProved: proof.claimsProved,
    claimsNotProved: proof.claimsNotProved,
    entryCount: proof.readModel.entries.length,
  };
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
