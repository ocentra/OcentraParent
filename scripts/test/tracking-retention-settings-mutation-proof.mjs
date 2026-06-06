import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-retention-settings-mutation-proof';
const output07 = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output32 = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultRoot = join(repoRoot, 'test-results', proofMode);
const generatedAt = '2026-06-06T19:40:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output32, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-retention-settings-mutation-proof',
  ]);

  const proofModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-retention-settings-mutation-proof.js'))
      .href
  );
  const proof = {
    ...proofModule.buildTrackingRetentionSettingsMutationProof(generatedAt),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp07: 'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
      wp32: 'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/26-retention-settings-mutation-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/32-retention-settings-mutation-proof.json',
      evidence: 'test-results/tracking-retention-settings-mutation-proof/proof.json',
    },
  };

  assertProof(proof);
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(output07, '20-retention-settings-mutation-proof.json'), proof);
  await writeJson(join(output32, '26-retention-settings-mutation-proof.json'), proof);
  await writeJson(join(output33, '32-retention-settings-mutation-proof.json'), proof);
  await writeFile(join(output07, '20-retention-settings-mutation-validation.log'), validationLog());
  await writeFile(join(output32, '26-retention-settings-mutation-validation.log'), validationLog());

  console.log('tracking-retention-settings-mutation-proof-ok');
  console.log('evidence=test-results/tracking-retention-settings-mutation-proof/proof.json');
}

function assertProof(proof) {
  if (proof.rows.length !== 5) {
    throw new Error(`Expected 5 retention mutation rows, got ${proof.rows.length}`);
  }
  if (proof.rows.some((row) => row.serviceMutationExecuted !== true)) {
    throw new Error('Every retention mutation row must execute local service mutation.');
  }
  if (proof.rows.some((row) => row.remoteSyncEnabled !== false || row.remoteAiEnabled !== false)) {
    throw new Error('Remote sync and remote AI must remain disabled.');
  }
  if (Object.values(proof.productClaims).some((claim) => claim !== false)) {
    throw new Error(`Retention mutation proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
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

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
