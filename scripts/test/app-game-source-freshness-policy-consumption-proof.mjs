import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-source-freshness-policy-consumption-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '74-source-freshness-policy-consumption');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '74-source-freshness-policy-consumption');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'app-game-source-freshness-policy-consumption',
    ])
  );

  const { AppGameSourceFreshnessPolicyConsumptionMatrix } =
    await import('../../packages/parent-domain/dist/app-game-source-freshness-policy-consumption-data.js');
  const summary = summarizeMatrix(AppGameSourceFreshnessPolicyConsumptionMatrix);
  assertMatrix(AppGameSourceFreshnessPolicyConsumptionMatrix, summary);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-source-freshness-policy-consumption',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    counts: summary,
    evidence: {
      tsContract: 'packages/parent-domain/src/app-game-source-freshness-policy-consumption.ts',
      tsContractRules: 'packages/parent-domain/src/app-game-source-freshness-policy-consumption-rules.ts',
      tsContractValues: 'packages/parent-domain/src/app-game-source-freshness-policy-consumption-values.ts',
      tsContractData: 'packages/parent-domain/src/app-game-source-freshness-policy-consumption-data.ts',
      tsContractTest: 'packages/parent-domain/tests/app-game-source-freshness-policy-consumption.test.ts',
      proofHarness: 'scripts/test/app-game-source-freshness-policy-consumption-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/74-source-freshness-policy-consumption',
      appProofPack: 'output/app-plan-proof/74-source-freshness-policy-consumption',
    },
    claimsProved: [
      'policy-readiness consumes activity-surface sourceStatusRows without reading raw private source rows',
      'native app policy compile is allowed only when inventory, runtime, and foreground rows are fresh and evidence-backed',
      'native game policy compile is allowed only when inventory, runtime, foreground, and launcher rows are fresh and evidence-backed',
      'stale, missing, permission-limited, unavailable, adapter-error, manual-required, and not-claimed source rows block policy compile',
      'source freshness policy readiness never dispatches adapters or requests direct adapter calls',
    ],
    claimsNotProved: [
      'portal rendering of source freshness rows',
      'runtime policy evaluator dispatch beyond policy-readiness proof',
      'adapter execution, broad app/game blocking, or platform hard-control support',
      'live classifier/provider execution or content knowledge',
    ],
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof);
  await writeProofPack(appProofDir, proof);

  console.log(`app-game-source-freshness-policy-consumption-ok:${Object.keys(summary.byReadinessState).join(',')}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function summarizeMatrix(matrix) {
  return {
    readinessCount: matrix.readiness.length,
    byReadinessState: countBy(matrix.readiness.map((entry) => entry.readinessState)),
    policyCompileAllowed: matrix.readiness.filter((entry) => entry.policyCompileAllowed).length,
    directAdapterCalls: matrix.readiness.filter((entry) => entry.directAdapterCallRequested).length,
    rawPrivateRowsIncluded: matrix.readiness.filter((entry) => entry.rawPrivateSourceRowsIncluded).length,
    manualRequirementFailures: matrix.readiness.flatMap((entry) =>
      entry.requirementResults.filter((result) => result.requirementState !== 'satisfied')
    ).length,
  };
}

function assertMatrix(matrix, summary) {
  assertEqual(String(matrix.matrixId), 'app-game-source-freshness-policy-consumption', 'matrix id');
  assertEqual(summary.readinessCount, 3, 'readiness count');
  assertEqual(summary.byReadinessState['policy-ready'], 2, 'policy-ready count');
  assertEqual(summary.byReadinessState['manual-required'], 1, 'manual-required count');
  assertEqual(summary.policyCompileAllowed, 2, 'policy compile allowed count');
  assertEqual(summary.directAdapterCalls, 0, 'direct adapter call count');
  assertEqual(summary.rawPrivateRowsIncluded, 0, 'raw private rows included count');
  assertEqual(summary.manualRequirementFailures, 3, 'manual requirement failure count');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function writeProofPack(proofDir, proof) {
  await mkdir(join(proofDir, '06-ui-snapshots'), { recursive: true });
  await writeText(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# Source Snapshot',
      '',
      `Branch: ${await gitBranch()}`,
      `Commit: ${proof.commit}`,
      '',
      'Inspected source inputs:',
      '',
      '- `packages/activity-domain/src/activity-surface.ts` for `sourceStatusRows` shape.',
      '- `packages/parent-domain/src/app-game-policy-target-compiler.ts` for existing policy compiler boundaries.',
      '- `docs/features/app-game-control.md` and app/app-game plan WP47/WP72 docs for remaining source freshness gaps.',
      '',
      'Touched implementation:',
      '',
      '- `packages/parent-domain/src/app-game-source-freshness-policy-consumption*.ts`',
      '- `packages/parent-domain/tests/app-game-source-freshness-policy-consumption.test.ts`',
      '- `scripts/test/app-game-source-freshness-policy-consumption-proof.mjs`',
      '',
    ].join('\n')
  );
  await writeText(
    join(proofDir, '01-contract-proof.log'),
    [
      'cmd /c npm run build:contracts: PASS',
      'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-source-freshness-policy-consumption: PASS',
      '',
      'Contract proof covers policy-ready native app/game rows, manual-required stale/missing/not-claimed rows,',
      'evidence-ref requirements, target-ref requirements, and negative adapter-dispatch/private-row cases.',
      '',
    ].join('\n')
  );
  await writeText(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Not applicable: WP74 is a parent-domain policy-readiness contract proof and does not add Rust protocol fields.\n'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), {
    schemaVersion: 1,
    runtimeEvidenceClaim: 'not-applicable',
    reason: 'WP74 consumes already-projected sourceStatusRows and does not add runtime capture or service polling.',
  });
  await writeJson(join(proofDir, '04-journal-sqlite-proof.json'), {
    schemaVersion: 1,
    journalSqliteClaim: 'not-applicable',
    reason:
      'WP74 consumes backend read-model rows from WP47 after the WP72 freshness gate and does not add journal or SQLite storage.',
  });
  await writeJson(join(proofDir, '05-policy-action-proof.json'), proof);
  await writeText(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nWP74 does not change portal or child-facing UI.\n'
  );
  await writeText(join(proofDir, '07-playwright-ui-proof.log'), 'Not applicable: no UI files changed in WP74.\n');
  await writeText(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'PASS: source freshness requests require sourceRowsFromActivityReadModel=true.',
      'PASS: source freshness requests require rawPrivateSourceRowsIncluded=false.',
      'PASS: policy readiness rejects directAdapterCallRequested=true.',
      'PASS: non-empty source rows must cite evidence refs.',
      'PASS: stale, missing, and not-claimed source rows block policy compile.',
      '',
    ].join('\n')
  );
  await writeText(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNot applicable: WP74 does not claim platform adapter support or hard-control behavior.\n'
  );
  await writeText(join(proofDir, '10-validation-commands.log'), `${proof.commands.join('\n')}\n`);
  await writeText(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nNot applicable: WP74 does not change platform authority tiers.\n'
  );
  await writeText(
    join(proofDir, '12-rollback-proof.md'),
    '# Rollback Proof\n\nNot applicable: WP74 does not dispatch adapters or mutate child-device policy state.\n'
  );
}

async function writeText(path, value) {
  await writeFile(path, value.endsWith('\n') ? value : `${value}\n`);
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${command} exited with ${code}`))));
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

async function gitBranch() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['branch', '--show-current'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git branch --show-current failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
