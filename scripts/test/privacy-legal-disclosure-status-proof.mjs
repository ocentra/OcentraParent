import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const resultDir = join(repoRoot, 'test-results', 'production-support-privacy-legal-disclosure-status-proof');
const outputDir = join(repoRoot, 'output', 'production-support-privacy-legal-disclosure-status-proof');
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
    'tests/privacy-legal-disclosure-status.test.ts',
  ]);

  const commit = await gitHead();
  const readModel = await parseReadModel();
  assertReadModel(readModel);
  await assertPackageExports();

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode: 'production-support-privacy-legal-disclosure-status-proof',
    commands,
    evidence: {
      contract: 'packages/logging-domain/src/privacy-legal-disclosure-status.ts',
      guards: 'packages/logging-domain/src/privacy-legal-disclosure-status-guards.ts',
      readModel: 'packages/logging-domain/src/privacy-legal-disclosure-status-read-model.ts',
      contractTest: 'packages/logging-domain/tests/privacy-legal-disclosure-status.test.ts',
      proofOutput: relative(repoRoot, proofPath),
      summaryOutput: relative(repoRoot, summaryPath),
      featureDoc: 'docs/features/production-distribution-support.md',
      expectations: ['docs/expectations/data-custody.md', 'docs/expectations/release-installer.md'],
    },
    claimsProved: [
      'Privacy/legal disclosure status rows cover requested, parent-authorized, legal-review queued/running, parent-notification-ready, publication-ready, failed, and manual-required states.',
      'Each row is parent-authorized and carries only support-safe disclosure status, parent consent, privacy policy, legal review, publication, support runbook, audit, and manual proof references.',
      'Failed rows require failure audit refs and manual-required rows require manual proof requirements before any disclosure execution claim can be made.',
      'Package exports expose the privacy/legal disclosure status contract and read model through @ocentra-parent/logging-domain.',
      'Rows reject tokens, raw child activity, raw URLs, screenshots, journals, SQLite snapshots, private paths, command lines, keystrokes, clipboard data, message contents, provider secrets, remote support transcripts, legal disclosure execution, public runtime execution, support backend upload execution, account lookup, billing provider contact, remote support sessions, production SLA, and child activity custody.',
    ],
    claimsNotProved: [
      'legal disclosure execution',
      'public runtime execution',
      'support backend upload execution',
      'account lookup execution',
      'billing provider contact execution',
      'remote support sessions',
      'production SLA',
      'provider secrets',
      'child activity custody',
    ],
    readModel,
  };

  const summary = {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit,
    proofMode: proof.proofMode,
    statesCovered: readModel.entries.map((entry) => entry.disclosureState),
    output: relative(repoRoot, proofPath),
    claimsNotProved: proof.claimsNotProved,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
  console.log(`production-support-privacy-legal-disclosure-status-proof-ok:${relative(repoRoot, proofPath)}`);
}

async function parseReadModel() {
  const modulePath = join(repoRoot, 'packages', 'logging-domain', 'dist', 'privacy-legal-disclosure-status.js');
  const readModelPath = join(
    repoRoot,
    'packages',
    'logging-domain',
    'dist',
    'privacy-legal-disclosure-status-read-model.js'
  );
  const module = await import(pathToFileURL(modulePath).href);
  const readModelModule = await import(pathToFileURL(readModelPath).href);
  return module.PrivacyLegalDisclosureReadModelSchema.parse(readModelModule.PrivacyLegalDisclosureReadModel);
}

function assertReadModel(readModel) {
  assert.equal(readModel.readModelId, 'production-support-privacy-legal-disclosure-status-proof');
  assert.equal(readModel.entries.length, 8);
  const states = new Set(readModel.entries.map((entry) => entry.disclosureState));
  for (const state of [
    'disclosure-requested',
    'parent-authorized',
    'legal-review-queued',
    'legal-review-running',
    'parent-notification-ready',
    'publication-ready',
    'disclosure-failed',
    'manual-required',
  ]) {
    assert.equal(states.has(state), true);
  }

  const failed = entryFor(readModel, 'privacy-legal-disclosure-failed');
  const manual = entryFor(readModel, 'privacy-legal-manual-required');
  assert.equal(failed.failureRefs.length, 1);
  assert.equal(manual.manualProofRequirements.length, 1);
}

async function assertPackageExports() {
  const contract = await import('@ocentra-parent/logging-domain/privacy-legal-disclosure-status');
  const readModel = await import('@ocentra-parent/logging-domain/privacy-legal-disclosure-status-read-model');
  assert.equal(typeof contract.PrivacyLegalDisclosureReadModelSchema.parse, 'function');
  assert.equal(readModel.PrivacyLegalDisclosureReadModel.entries.length, 8);
}

function entryFor(readModel, disclosureId) {
  const entry = readModel.entries.find((candidate) => candidate.disclosureId === disclosureId);
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
