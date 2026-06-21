import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-game-adapter-execution-readiness-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      '--run',
      'tests/unit/app-game-adapter-execution-readiness.test.ts',
    ])
  );

  const { AppGameAdapterExecutionReadinessReadModel, summarizeAppGameAdapterExecutionReadiness } =
    await import('../../packages/app-game-domain/dist/app-game-adapter-execution-readiness.js');
  const summary = summarizeAppGameAdapterExecutionReadiness(AppGameAdapterExecutionReadinessReadModel);

  assertEqual(AppGameAdapterExecutionReadinessReadModel.readModelId, 'app-game-adapter-execution-readiness', 'id');
  assertEqual(summary.rows, 8, 'row count');
  assertEqual(summary.executionAllowed, 1, 'execution allowed count');
  assertEqual(summary.blockedBeforeExecution, 7, 'blocked count');
  assertEqual(summary.adapterExecutionClaimed, 1, 'adapter execution claim count');
  assertEqual(summary.broadInstalledAppBlockingClaimed, 0, 'broad app claim count');
  assertEqual(summary.childDeviceDeliveryClaimed, 0, 'child delivery claim count');
  assertEqual(summary.platformEnforcementClaimed, 0, 'platform enforcement claim count');
  assertEqual(summary.providerDeliveryClaimed, 0, 'provider delivery claim count');
  assertEqual(summary.privateDiagnosticsClaimed, 0, 'private diagnostics claim count');

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-adapter-execution-readiness-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    sourceReadModel: 'v0-8-supported-adapter-runtime-proof',
    evidence: {
      schemaContract: 'packages/schema-domain/src/app-game-adapter-execution-readiness.ts',
      consumerReadModel: 'packages/app-game-domain/src/app-game-adapter-execution-readiness.ts',
      consumerTest: 'packages/app-game-domain/tests/unit/app-game-adapter-execution-readiness.test.ts',
      proofHarness: 'scripts/test/app-game-adapter-execution-readiness-proof.mjs',
      sourceRuntimeProof: 'packages/schema-domain/src/v0-8-supported-adapter-runtime-proof.ts',
    },
    summary,
    claimsProved: [
      'App/game adapter execution readiness consumes the central schema-domain contract through the app-game-domain read-model consumer',
      'App/game adapter execution readiness is derived from existing V0.8 supported adapter runtime proof',
      'Windows owned-process time-limit is the only execution-allowed app/game adapter row',
      'Broad installed-app blocking remains blocked before execution',
      'Linux, macOS, Android, and iOS rows remain unavailable, unsupported, or manual-required before execution',
      'No broad blocking, child delivery, platform enforcement, provider delivery, or private diagnostics claim is upgraded',
    ],
    claimsNotProved: [
      'runtime service command exposure for this read model',
      'app-game-domain package export for this read model',
      'broad installed-app blocking execution',
      'platform enforcement outside the scoped Windows owned-process boundary',
      'provider delivery or child-device delivery',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('app-game-adapter-execution-readiness-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
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
