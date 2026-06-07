import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const resultDir = join(repoRoot, 'test-results', 'app-game-adapter-capability-status-handoff-proof');
const appGameOutputDir = join(repoRoot, 'output', 'app-game-plan-proof', '107-adapter-capability-status-handoff');
const appPlanOutputDir = join(repoRoot, 'output', 'app-plan-proof', '107-adapter-capability-status-handoff');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(appGameOutputDir, { recursive: true });
  await mkdir(appPlanOutputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-adapter-capability-status-handoff',
    'v0-8-supported-adapter-runtime-proof',
    'v0-8-cross-platform-enforcement-capability-proof',
  ]);

  const { AppGameAdapterCapabilityStatusReadModel } =
    await import('../../packages/parent-domain/dist/app-game-adapter-capability-status-handoff.js');
  const summary = summarize(AppGameAdapterCapabilityStatusReadModel);
  assertSummary(AppGameAdapterCapabilityStatusReadModel, summary);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-adapter-capability-status-handoff',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-adapter-capability-status-handoff.ts',
      test: 'packages/parent-domain/tests/app-game-adapter-capability-status-handoff.test.ts',
      harness: 'scripts/test/app-game-adapter-capability-status-handoff-proof.mjs',
      featureDoc: 'docs/features/app-game-control.md',
      appGameWorkpack: 'docs/plans/app-game-plan/workpacks/107-adapter-capability-status-handoff.md',
      appGameProofDir: 'output/app-game-plan-proof/107-adapter-capability-status-handoff',
      appPlanProofDir: 'output/app-plan-proof/107-adapter-capability-status-handoff',
    },
    counts: summary,
    claimsProved: [
      'Adapter capability status handoff projects native app and native game rows for Windows, macOS, Linux, Android, and iOS',
      'Windows rows expose owned-process time-limit runtime-boundary readiness while broad installed-app blocking stays manual-required',
      'macOS rows are scaffold/manual-required, Linux rows are unavailable, and Android/iOS rows are manual-required',
      'Every row keeps adapter dispatch, broad blocking, platform enforcement, and child delivery claims false',
    ],
    claimsNotProved: [
      'broad installed-app blocking',
      'macOS adapter execution',
      'Linux adapter execution',
      'Android privileged child-device adapter execution',
      'iOS Family Controls or DeviceActivity execution',
      'portal runtime rendering',
      'child-device delivery',
    ],
  };

  const proofText = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(join(resultDir, 'proof.json'), proofText);
  await writeFile(join(appGameOutputDir, 'proof.json'), proofText);
  await writeFile(join(appPlanOutputDir, 'proof.json'), proofText);
  await writeFile(join(appGameOutputDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);
  await writeFile(join(appPlanOutputDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);
  await writeFile(
    join(appGameOutputDir, '00-source-snapshot.md'),
    [
      '# Source Snapshot',
      '',
      '- Source branch: codex/app-game-adapter-capability-status-handoff',
      '- Base: origin/main at PR490 merge b491e2e38',
      '- Existing source truth: V0.8 supported adapter runtime proof and cross-platform capability proof.',
      '- New proof: parent-domain projection only; no platform adapter execution or portal rendering.',
      '',
    ].join('\n')
  );
  await writeFile(
    join(appPlanOutputDir, '00-source-snapshot.md'),
    [
      '# Source Snapshot',
      '',
      '- Cross-recorded from shared app/game WP107.',
      '- Native app and native game product meanings stay separate over the shared adapter evidence spine.',
      '- Product checklist status is unchanged because runtime UI, broad blocking, and platform adapters remain gaps.',
      '',
    ].join('\n')
  );

  console.log(`app-game-adapter-capability-status-handoff-ok:${summary.rows}`);
  console.log(`evidence=${relative(repoRoot, join(resultDir, 'proof.json'))}`);
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    byPlatform: countBy(readModel.rows.map((row) => row.platform)),
    byTarget: countBy(readModel.rows.map((row) => row.productTarget)),
    byAdapterStatus: countBy(readModel.rows.map((row) => row.adapterStatus)),
    byBroadBlockingStatus: countBy(readModel.rows.map((row) => row.broadBlockingStatus)),
    adapterDispatchClaimed: readModel.rows.filter((row) => row.adapterDispatchClaimed).length,
    broadBlockingClaimed: readModel.rows.filter((row) => row.broadBlockingClaimed).length,
    platformEnforcementClaimed: readModel.rows.filter((row) => row.platformEnforcementClaimed).length,
    childDeliveryClaimed: readModel.rows.filter((row) => row.childDeliveryClaimed).length,
  };
}

function assertSummary(readModel, summary) {
  assertEqual(readModel.readModelId, 'app-game-adapter-capability-status-handoff', 'read model id');
  assertEqual(summary.rows, 10, 'row count');
  assertEqual(summary.byTarget['native-app'], 5, 'native app row count');
  assertEqual(summary.byTarget['native-game'], 5, 'native game row count');
  assertEqual(summary.byAdapterStatus['runtime-boundary-ready'], 2, 'runtime-boundary-ready count');
  assertEqual(summary.byAdapterStatus['scaffold-only'], 2, 'scaffold-only count');
  assertEqual(summary.byAdapterStatus.unavailable, 2, 'unavailable count');
  assertEqual(summary.byAdapterStatus['manual-required'], 4, 'manual-required count');
  assertEqual(summary.adapterDispatchClaimed, 0, 'adapter dispatch claim count');
  assertEqual(summary.broadBlockingClaimed, 0, 'broad blocking claim count');
  assertEqual(summary.platformEnforcementClaimed, 0, 'platform enforcement claim count');
  assertEqual(summary.childDeliveryClaimed, 0, 'child delivery claim count');
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`))));
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
