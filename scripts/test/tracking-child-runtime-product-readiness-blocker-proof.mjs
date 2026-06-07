import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-child-runtime-product-readiness-blocker-proof';
const output30 = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const namedProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const resultRoot = join(repoRoot, 'test-results', proofMode);
const sourceSnapshotRequirementsProofRef =
  'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/28-child-runtime-snapshot-requirements-proof.json';
const sourceSnapshotRequirementsProofPath = join(repoRoot, sourceSnapshotRequirementsProofRef);
const generatedAt = '2026-06-07T16:05:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output30, { recursive: true });
  await mkdir(output33, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-child-runtime-product-readiness-blocker-proof',
  ]);

  const sourceSnapshotRequirementsProof = JSON.parse(await readFile(sourceSnapshotRequirementsProofPath, 'utf8'));
  const proof = await buildProof(sourceSnapshotRequirementsProof);
  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-child-runtime-product-readiness-blocker-proof-ok');
  console.log('evidence=test-results/tracking-child-runtime-product-readiness-blocker-proof/proof.json');
}

async function buildProof(sourceSnapshotRequirementsProof) {
  const proofModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-child-runtime-product-readiness-blocker-proof.js')
    ).href
  );
  return {
    ...proofModule.buildTrackingChildRuntimeProductReadinessBlockerProof(
      generatedAt,
      sourceSnapshotRequirementsProofRef,
      sourceSnapshotRequirementsProof
    ),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp30: 'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/29-child-runtime-product-readiness-blocker-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/45-child-runtime-product-readiness-blocker-proof.json',
      evidence: 'test-results/tracking-child-runtime-product-readiness-blocker-proof/proof.json',
      sourceSnapshotRequirementsProof: sourceSnapshotRequirementsProofRef,
    },
  };
}

function assertProof(proof) {
  if (proof.rows.length === 0 || !proof.proofClaims.productReadinessBlocked) {
    throw new Error(`Child runtime product-readiness blocker proof is empty: ${JSON.stringify(proof)}`);
  }
  for (const row of proof.rows) {
    if (row.childDeviceDeliveryRuntimeClaimed || row.childDeviceExecutionRuntimeClaimed || row.productReadyClaimed) {
      throw new Error(`Child runtime product readiness was overclaimed: ${JSON.stringify(row)}`);
    }
    if (
      row.executionResultRequirementRefCount <= 0 ||
      row.visibleSnapshotRequirementRefCount <= 0 ||
      row.parentReceiptRequirementRefCount <= 0 ||
      row.runtimeObservationRequirementRefCount <= 0
    ) {
      throw new Error(`Child runtime requirement refs are incomplete: ${JSON.stringify(row)}`);
    }
  }
}

async function writeProofArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'child-runtime-product-readiness-blocker-read-model.json'), proof.rows);
  await writeJson(join(output30, '29-child-runtime-product-readiness-blocker-proof.json'), proof);
  await writeJson(join(output33, '45-child-runtime-product-readiness-blocker-proof.json'), proof);
  await writeJson(join(namedProofRoot, 'proof.json'), proof);
  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeFile(join(namedProofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(join(namedProofRoot, '16-validation-commands.log'), validationLog());
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Child Runtime Product Readiness Blocker Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- requiredProofTier: P2_HOSTED_CI',
    '- currentProofTier: P2_HOSTED_CI',
    '- status: proved',
    `- consumes: ${sourceSnapshotRequirementsProofRef}`,
    '- proves child runtime requirement coverage is still product-readiness blocked',
    '- proof module: packages/parent-domain/src/tracking-child-runtime-product-readiness-blocker-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-child-runtime-product-readiness-blocker-proof.test.ts',
    '- proof harness: scripts/test/tracking-child-runtime-product-readiness-blocker-proof.mjs',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=30-parent-and-child-ui-ux-surfaces',
    'Child runtime product-readiness blocker rows consume snapshot requirement rows and preserve explicit non-claims.',
    'Rows prove requirement coverage only and do not claim actual child-device delivery, execution, or rendered child UI.',
    'Provider delivery, notification receipt ingestion, live location runtime, physical-device proof, authority proof, production workers, and product-ready behavior are explicit non-claims.',
    '',
  ].join('\n');
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
