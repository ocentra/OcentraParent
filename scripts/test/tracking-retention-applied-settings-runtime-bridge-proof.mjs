import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-retention-applied-settings-runtime-bridge-proof';
const resultRoot = join(repoRoot, 'test-results', proofMode);
const focusedProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output07 = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const sourceWritableExecutionProofRef =
  'output/tracking-plan-proof/tracking-retention-product-settings-writable-execution-proof/proof.json';
const sourceWritableExecutionProofPath = join(repoRoot, sourceWritableExecutionProofRef);
const generatedAt = '2026-06-08T20:10:00.000Z';
const commands = [];

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(focusedProofRoot, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-retention-product-settings-writable-execution-proof.mjs']);
  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-retention-applied-settings-runtime-bridge-proof.test.ts',
  ]);

  const writableExecutionProof = JSON.parse(await readFile(sourceWritableExecutionProofPath, 'utf8'));
  const proofModule = await tsImport(
    pathToFileURL(
      join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-retention-applied-settings-runtime-bridge-proof.ts')
    ).href,
    import.meta.url
  );
  const proof = {
    ...proofModule.buildTrackingRetentionAppliedSettingsRuntimeBridgeProof(
      generatedAt,
      sourceWritableExecutionProofRef,
      writableExecutionProof
    ),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    commands,
    artifactPaths: {
      focused: 'output/tracking-plan-proof/tracking-retention-applied-settings-runtime-bridge-proof/proof.json',
      wp07: 'output/tracking-plan-proof/07-retention-and-custody-model/28-retention-applied-settings-runtime-bridge-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/76-retention-applied-settings-runtime-bridge-proof.json',
      evidence: 'test-results/tracking-retention-applied-settings-runtime-bridge-proof/proof.json',
      sourceWritableExecutionProof: sourceWritableExecutionProofRef,
    },
  };

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-retention-applied-settings-runtime-bridge-proof-ok');
  console.log('evidence=test-results/tracking-retention-applied-settings-runtime-bridge-proof/proof.json');
}

function assertProof(proof) {
  if (proof.rows.length !== 1) {
    throw new Error(`Expected one applied settings bridge row, got ${proof.rows.length}`);
  }
  const [row] = proof.rows;
  if (!row.localAppliedSettingsObserved || !row.writableExecutionArtifactPresent) {
    throw new Error('Applied settings bridge must preserve local writable execution evidence.');
  }
  if (row.platformRuntimeRetentionEnforcementPresent || row.platformRuntimeRetentionEnforcementClaimed) {
    throw new Error('Applied settings bridge must keep platform runtime enforcement missing and unclaimed.');
  }
  if (
    !proof.runtimeArtifactInventory.presentArtifacts.includes(
      'tracking-retention/product-settings-writable-execution.json'
    )
  ) {
    throw new Error('Applied settings bridge must mark the local writable execution artifact present.');
  }
  if (
    !proof.runtimeArtifactInventory.missingArtifacts.includes(
      'tracking-retention/platform-runtime-retention-enforcement.json'
    )
  ) {
    throw new Error('Applied settings bridge must mark the platform runtime enforcement artifact missing.');
  }
  if (Object.values(proof.productClaims).some((claim) => claim !== false)) {
    throw new Error(`Applied settings bridge overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
  }
}

async function writeArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(focusedProofRoot, 'proof.json'), proof);
  await writeJson(join(output07, '28-retention-applied-settings-runtime-bridge-proof.json'), proof);
  await writeJson(join(output33, '76-retention-applied-settings-runtime-bridge-proof.json'), proof);
  await writeFile(
    join(focusedProofRoot, '00-source-snapshot.md'),
    [
      '# Tracking Retention Applied Settings Runtime Bridge Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.baseCommitAtGeneration}`,
      '- source writable execution proof is consumed before this bridge is emitted',
      '- local writable execution artifact is present',
      '- platform runtime retention enforcement artifact remains missing',
      '- production, authority, provider, physical-device, child-runtime, and product-ready claims remain false',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(output07, '28-retention-applied-settings-runtime-bridge-validation.log'), validationLog());
  await writeFile(join(output33, '76-retention-applied-settings-runtime-bridge-validation.log'), validationLog());
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
