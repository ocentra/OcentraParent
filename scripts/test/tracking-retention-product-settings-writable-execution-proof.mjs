import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-retention-product-settings-writable-execution-proof';
const resultRoot = join(repoRoot, 'test-results', proofMode);
const focusedProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output07 = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const retentionRuntimeRoot = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-retention');
const sourceLocalServiceStateProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json';
const generatedAt = '2026-06-08T03:45:00.000Z';
const commands = [];

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(focusedProofRoot, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output33, { recursive: true });
  await mkdir(retentionRuntimeRoot, { recursive: true });

  run('node', ['scripts/test/tracking-retention-local-service-state-proof.mjs']);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-retention-product-settings-writable-execution-proof',
  ]);

  const localServiceStateProof = JSON.parse(await readFile(join(repoRoot, sourceLocalServiceStateProofRef), 'utf8'));
  const proofModule = await import(
    pathToFileURL(
      join(
        repoRoot,
        'packages',
        'parent-domain',
        'dist',
        'tracking-retention-product-settings-writable-execution-proof.js'
      )
    ).href
  );
  const proof = {
    ...proofModule.buildTrackingRetentionProductSettingsWritableExecutionProof(
      generatedAt,
      sourceLocalServiceStateProofRef,
      localServiceStateProof
    ),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    requiredRuntimeArtifactRef: proofModule.TrackingRetentionProductSettingsWritableExecutionArtifactRef,
    commands,
    artifactPaths: {
      focused: 'output/tracking-plan-proof/tracking-retention-product-settings-writable-execution-proof/proof.json',
      runtimeArtifact: 'output/tracking-plan-proof/tracking-retention/product-settings-writable-execution.json',
      wp07: 'output/tracking-plan-proof/07-retention-and-custody-model/26-retention-product-settings-writable-execution-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/64-retention-product-settings-writable-execution-proof.json',
      evidence: 'test-results/tracking-retention-product-settings-writable-execution-proof/proof.json',
      sourceLocalServiceStateProof: sourceLocalServiceStateProofRef,
    },
  };

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-retention-product-settings-writable-execution-proof-ok');
  console.log('evidence=test-results/tracking-retention-product-settings-writable-execution-proof/proof.json');
}

function assertProof(proof) {
  if (proof.rows.length !== 1) {
    throw new Error(`Expected one writable execution row, got ${proof.rows.length}`);
  }
  const [row] = proof.rows;
  if (row.outputArtifactRef !== proof.requiredRuntimeArtifactRef) {
    throw new Error(`Writable execution artifact ref mismatch: ${row.outputArtifactRef}`);
  }
  if (!row.writeCommandAccepted || !row.serviceMutationExecuted || !row.localServiceStateReadbackClaimed) {
    throw new Error('Writable execution artifact must come from accepted local service execution.');
  }
  if (!row.durableSettingsPersisted || !row.localProductSettingsWritableExecutionObserved) {
    throw new Error('Writable execution artifact must keep durable local execution evidence.');
  }
  if (row.remoteSyncEnabled || row.remoteAiEnabled) {
    throw new Error('Writable execution artifact must keep remote sync and remote AI disabled.');
  }
  if (Object.values(proof.productClaims).some((claim) => claim !== false)) {
    throw new Error(`Writable execution proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
  }
}

async function writeArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(focusedProofRoot, 'proof.json'), proof);
  await writeJson(join(retentionRuntimeRoot, 'product-settings-writable-execution.json'), proof);
  await writeJson(join(output07, '26-retention-product-settings-writable-execution-proof.json'), proof);
  await writeJson(join(output33, '64-retention-product-settings-writable-execution-proof.json'), proof);
  await writeFile(
    join(focusedProofRoot, '00-source-snapshot.md'),
    [
      '# Tracking Retention Product Settings Writable Execution Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.baseCommitAtGeneration}`,
      `- outputArtifactRef: ${proof.requiredRuntimeArtifactRef}`,
      '- source local service state proof is required before writing this artifact',
      '- artifact proves local typed service write execution and durable readback only',
      '- platform runtime retention enforcement and product-ready claims remain false',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(output07, '26-retention-product-settings-writable-execution-validation.log'), validationLog());
  await writeFile(join(output33, '64-retention-product-settings-writable-execution-validation.log'), validationLog());
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

async function writeJson(filePath, value) {
  await mkdir(dirname(filePath), { recursive: true });
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`);
}
