import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-retention-durable-settings-proof';
const output07 = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output32 = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultRoot = join(repoRoot, 'test-results', proofMode);
const sourceLocalServiceStateProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json';
const localServiceStateProofPath = join(
  repoRoot,
  'test-results',
  'tracking-retention-local-service-state-proof',
  'proof.json'
);
const generatedAt = '2026-06-07T10:20:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output32, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-retention-local-service-state-proof.mjs']);
  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-retention-durable-settings-proof.test.ts',
  ]);

  const localServiceStateProof = JSON.parse(await readFile(localServiceStateProofPath, 'utf8'));
  const proofModule = await tsImport(
    pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'src', 'tracking-retention-durable-settings-proof.ts'))
      .href,
    import.meta.url
  );
  const proof = {
    ...proofModule.buildTrackingRetentionDurableSettingsProof(
      generatedAt,
      sourceLocalServiceStateProofRef,
      localServiceStateProof
    ),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp07: 'output/tracking-plan-proof/07-retention-and-custody-model/23-retention-durable-settings-proof.json',
      wp32: 'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/34-retention-durable-settings-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/41-retention-durable-settings-proof.json',
      evidence: 'test-results/tracking-retention-durable-settings-proof/proof.json',
      sourceLocalServiceStateProof: sourceLocalServiceStateProofRef,
    },
  };

  assertProof(proof);
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(output07, '23-retention-durable-settings-proof.json'), proof);
  await writeJson(join(output32, '34-retention-durable-settings-proof.json'), proof);
  await writeJson(join(output33, '41-retention-durable-settings-proof.json'), proof);
  await writeFile(join(output07, '23-retention-durable-settings-validation.log'), validationLog());
  await writeFile(join(output32, '34-retention-durable-settings-validation.log'), validationLog());

  console.log('tracking-retention-durable-settings-proof-ok');
  console.log('evidence=test-results/tracking-retention-durable-settings-proof/proof.json');
}

function assertProof(proof) {
  if (proof.rows.length !== 1) {
    throw new Error(`Expected 1 retention durable settings row, got ${proof.rows.length}`);
  }
  const [row] = proof.rows;
  if (
    row.durableSettingsPersisted !== true ||
    row.durableSettingsStoreRef !== 'agent-service-local-retention-settings-durable-json' ||
    row.durablePersistenceRequired !== true ||
    row.durabilityFailureVisible !== false
  ) {
    throw new Error(`Unexpected durable settings state: ${JSON.stringify(row)}`);
  }
  const { durableSettingsPersisted, ...remainingProductClaims } = proof.productClaims;
  if (!durableSettingsPersisted || Object.values(remainingProductClaims).some((claim) => claim !== false)) {
    throw new Error(
      `Retention durable settings proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`
    );
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
