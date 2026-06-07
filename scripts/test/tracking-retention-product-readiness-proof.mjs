import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-retention-product-readiness-proof';
const output07 = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output32 = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultRoot = join(repoRoot, 'test-results', proofMode);
const sourceDurableSettingsProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/23-retention-durable-settings-proof.json';
const durableSettingsProofPath = join(
  repoRoot,
  'test-results',
  'tracking-retention-durable-settings-proof',
  'proof.json'
);
const generatedAt = '2026-06-07T15:30:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output32, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-retention-durable-settings-proof.mjs']);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-retention-product-readiness-proof',
  ]);

  const durableSettingsProof = JSON.parse(await readFile(durableSettingsProofPath, 'utf8'));
  const proof = await buildProof(durableSettingsProof);
  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-retention-product-readiness-proof-ok');
  console.log('evidence=test-results/tracking-retention-product-readiness-proof/proof.json');
}

async function buildProof(durableSettingsProof) {
  const proofModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-retention-product-readiness-proof.js'))
      .href
  );
  return {
    ...proofModule.buildTrackingRetentionProductReadinessProof(
      generatedAt,
      sourceDurableSettingsProofRef,
      durableSettingsProof
    ),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp07: 'output/tracking-plan-proof/07-retention-and-custody-model/24-retention-product-readiness-proof.json',
      wp32: 'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/35-retention-product-readiness-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/43-retention-product-readiness-proof.json',
      evidence: 'test-results/tracking-retention-product-readiness-proof/proof.json',
      sourceDurableSettingsProof: sourceDurableSettingsProofRef,
    },
  };
}

function assertProof(proof) {
  if (proof.rows.length !== 1) {
    throw new Error(`Expected 1 retention readiness row, got ${proof.rows.length}`);
  }
  const [row] = proof.rows;
  const productClaims = Object.entries(proof.productClaims).filter(([key]) => key !== 'localDurableSettingsReady');
  if (!proof.productClaims.localDurableSettingsReady || productClaims.some(([, value]) => value !== false)) {
    throw new Error(`Retention readiness proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
  }
  if (!row.productReadinessBlockers.includes('production-worker-hardening')) {
    throw new Error(`Retention readiness proof missed production blocker: ${JSON.stringify(row)}`);
  }
}

async function writeProofArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(output07, '24-retention-product-readiness-proof.json'), proof);
  await writeJson(join(output32, '35-retention-product-readiness-proof.json'), proof);
  await writeJson(join(output33, '43-retention-product-readiness-proof.json'), proof);
  await writeFile(
    join(resultRoot, 'retention-product-readiness-read-model.json'),
    `${JSON.stringify(proof.rows, null, 2)}\n`
  );
  await writeFile(join(output07, '24-retention-product-readiness-validation.log'), validationLog());
  await writeFile(join(output32, '35-retention-product-readiness-validation.log'), validationLog());
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
