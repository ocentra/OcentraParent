import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-retention-local-service-state-proof';
const output07 = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output32 = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultRoot = join(repoRoot, 'test-results', proofMode);
const writeProofPath = join(resultRootForWriteProof(), 'proof.json');
const sourceWriteCommandProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json';
const generatedAt = '2026-06-07T09:05:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output32, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-retention-settings-write-command-proof.mjs']);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-retention-local-service-state-proof',
  ]);

  const sourceWriteProof = JSON.parse(await readFile(writeProofPath, 'utf8'));
  const proofModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-retention-local-service-state-proof.js')
    ).href
  );
  const proof = {
    ...proofModule.buildTrackingRetentionLocalServiceStateProof(
      generatedAt,
      sourceWriteCommandProofRef,
      sourceWriteProof.result
    ),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp07: 'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json',
      wp32: 'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/33-retention-local-service-state-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/40-retention-local-service-state-proof.json',
      evidence: 'test-results/tracking-retention-local-service-state-proof/proof.json',
      sourceWriteCommandProof: sourceWriteCommandProofRef,
    },
  };

  assertProof(proof);
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(output07, '22-retention-local-service-state-proof.json'), proof);
  await writeJson(join(output32, '33-retention-local-service-state-proof.json'), proof);
  await writeJson(join(output33, '40-retention-local-service-state-proof.json'), proof);
  await writeFile(join(output07, '22-retention-local-service-state-validation.log'), validationLog());
  await writeFile(join(output32, '33-retention-local-service-state-validation.log'), validationLog());

  console.log('tracking-retention-local-service-state-proof-ok');
  console.log('evidence=test-results/tracking-retention-local-service-state-proof/proof.json');
}

function assertProof(proof) {
  if (proof.rows.length !== 1) {
    throw new Error(`Expected 1 retention local service-state row, got ${proof.rows.length}`);
  }
  const [row] = proof.rows;
  if (row.localServiceStateRevision !== 1) {
    throw new Error(`Expected local service revision 1, got ${row.localServiceStateRevision}`);
  }
  if (row.localServiceStateSnapshotRef !== 'agent-service-local-retention-settings-state') {
    throw new Error(`Unexpected local service snapshot ref: ${row.localServiceStateSnapshotRef}`);
  }
  if (row.durableSettingsStoreRef !== 'agent-service-local-retention-settings-durable-json') {
    throw new Error(`Unexpected durable settings store ref: ${row.durableSettingsStoreRef}`);
  }
  const { durableSettingsPersisted, ...remainingProductClaims } = proof.productClaims;
  if (
    row.durableSettingsPersisted !== true ||
    !durableSettingsPersisted ||
    Object.values(remainingProductClaims).some((claim) => claim !== false)
  ) {
    throw new Error(`Retention local state proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
  }
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

function gitOutput(args) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) return '';
  return result.stdout.trim();
}

function resultRootForWriteProof() {
  return join(repoRoot, 'test-results', 'tracking-retention-settings-write-command-proof');
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
