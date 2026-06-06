import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const sourceProofPath = join(repoRoot, 'test-results', 'tracking-android-status-proof', 'proof.json');
const outputDir = join(repoRoot, 'test-results', 'tracking-android-status-gap-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-android-status-gap-proof');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const generatedAt = '2026-06-06T13:55:00.000Z';
const commands = [];

await rm(outputDir, { recursive: true, force: true });
await rm(proofDir, { recursive: true, force: true });
await mkdir(outputDir, { recursive: true });
await mkdir(proofDir, { recursive: true });
await mkdir(wp33ProofDir, { recursive: true });

run('node', ['scripts/test/tracking-android-status-proof.mjs']);

const sourceProof = JSON.parse(await readFile(sourceProofPath, 'utf8'));
const proof = buildProof(sourceProof);

assertProof(proof);
await writeJson(join(outputDir, 'proof.json'), proof);
await writeJson(join(proofDir, 'proof.json'), proof);
await writeFile(join(proofDir, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
await writeFile(join(proofDir, '16-validation-commands.log'), validationLog(), 'utf8');
await writeJson(join(wp33ProofDir, '26-android-status-gap-proof.json'), proof);

console.log('tracking-android-status-gap-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-android-status-gap-proof', 'proof.json')}`);

function buildProof(sourceProof) {
  return {
    proofMode: 'tracking-android-status-gap-proof',
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    commands,
    sourceProofPath: 'test-results/tracking-android-status-proof/proof.json',
    sourceProofSummary: sourceProof.summary,
    sourceNonClaims: sourceProof.nonClaims,
    statusGapRows: sourceProof.readModel.rows.map((row) => ({
      rowId: row.rowId,
      caseKind: row.caseKind,
      claimState: row.claimState,
      parentVisibleStatusToken: row.parentVisibleStatusToken,
      missingProofReasonRefs: row.missingProofReasonRefs,
    })),
    proofPaths: {
      sourceHarness: 'scripts/test/tracking-android-status-proof.mjs',
      companionHarness: 'scripts/test/tracking-android-status-gap-proof.mjs',
      wp10Proof:
        'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/17-status-gap-proof.json',
      wp33Proof:
        'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/26-android-status-gap-proof.json',
      evidence: 'test-results/tracking-android-status-gap-proof/proof.json',
    },
  };
}

function assertProof(proof) {
  const expectedCaseKinds = [
    'low-power-degraded',
    'app-killed-restarted',
    'pending-upload-auditable',
    'manual-required',
  ];
  const actualCaseKinds = proof.statusGapRows.map((row) => row.caseKind);

  if (JSON.stringify(actualCaseKinds) !== JSON.stringify(expectedCaseKinds)) {
    throw new Error(`Unexpected Android status gap rows: ${JSON.stringify(actualCaseKinds)}`);
  }
  if (
    proof.sourceProofSummary.lowPowerDegradedCount !== 1 ||
    proof.sourceProofSummary.appRestartObservedCount !== 1 ||
    proof.sourceProofSummary.pendingUploadAuditableCount !== 1 ||
    proof.sourceProofSummary.manualRequiredCount !== 1
  ) {
    throw new Error(`Unexpected Android status gap summary: ${JSON.stringify(proof.sourceProofSummary)}`);
  }
  if (Object.values(proof.sourceNonClaims).some((value) => value !== false)) {
    throw new Error(`Android status gap proof overclaimed behavior: ${JSON.stringify(proof.sourceNonClaims)}`);
  }
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Android Status Gap Proof Source Snapshot',
    '',
    `- Branch: ${proof.branch}`,
    `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
    '- Source proof: `node scripts/test/tracking-android-status-proof.mjs`.',
    '- Scope: WP10 low-power, killed/restarted, pending-upload, and manual-required Android status gap rows.',
    '- Boundary: companion proof only; foreground/background location runtime, geofence transitions, notification delivery, physical-device behavior, authority, production upload worker, and product-ready Android tracking remain unclaimed.',
    '',
  ].join('\n');
}

function validationLog() {
  return commands
    .map((command) =>
      [`$ ${command.command}`, command.stdout.trim(), command.stderr.trim()]
        .filter((line) => line.length > 0)
        .join('\n')
    )
    .join('\n\n');
}

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push({
    command: [command, ...args].join(' '),
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
  });
  if (result.status !== 0) {
    throw new Error(
      `Command failed: ${[command, ...args].join(' ')}\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
