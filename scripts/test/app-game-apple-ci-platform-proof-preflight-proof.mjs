import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-apple-ci-platform-proof-preflight-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '196-app-game-apple-ci-platform-proof-preflight'
);
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });

  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'app-game-apple-ci-platform-proof-preflight',
      'v0-8-os-adapter-manual-artifact-gates',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));

  const module = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-apple-ci-platform-proof-preflight.js'))
      .href
  );
  const readModel = module.createAppGameAppleCiPlatformProofPreflightReadModel({
    generatedAt: '2026-06-08T18:30:00.000Z',
  });
  const summary = module.summarizeAppGameAppleCiPlatformProofPreflightReadModel(readModel);

  assertEqual(summary.macosGateCount, 1, 'macOS manual gate count');
  assertEqual(summary.iosGateCount, 6, 'iOS manual gate count');
  assertEqual(summary.dispatchableRowCount, 0, 'dispatchable row count');
  assertEqual(summary.blockedRowCount, 2, 'blocked row count');

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    readModel,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-apple-ci-platform-proof-preflight.ts',
      contractTest: 'packages/parent-domain/tests/app-game-apple-ci-platform-proof-preflight.test.ts',
      sourceGates: 'packages/parent-domain/src/v0-8-os-adapter-manual-artifact-gates.ts',
      sourceGateTest: 'packages/parent-domain/tests/v0-8-os-adapter-manual-artifact-gates.test.ts',
    },
    claimsProved: [
      'macOS app/game control requires Apple-platform CI runner and macOS-specific permission, MDM/Endpoint, rollback, and audit artifacts before support can upgrade',
      'iOS app/game control requires Apple-platform CI runner, Family Controls, DeviceActivity, Managed Settings, and TestFlight/device artifacts before support can upgrade',
      'Windows-local execution is not counted as macOS or iOS proof',
      'macOS and iOS adapter dispatch remain blocked before CI/device artifacts exist',
    ],
    claimsNotProved: [
      'macOS runtime inventory, foreground, hard block, MDM, Endpoint Security, System Extension, rollback, or audit execution',
      'iOS FamilyControls, DeviceActivity, ManagedSettings, shield UI, MDM, supervised restrictions, TestFlight install, or App Store entitlement execution',
      'Adapter dispatch, broad blocking, platform enforcement, provider delivery, and child-device delivery',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(
    join(appGameProofDir, '00-source-snapshot.md'),
    [
      '# App-game Apple CI platform proof preflight',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- Contract: packages/parent-domain/src/app-game-apple-ci-platform-proof-preflight.ts',
      '- Source gates: packages/parent-domain/src/v0-8-os-adapter-manual-artifact-gates.ts',
      '',
      'Evidence:',
      '- macOS and iOS platform proof is represented as CI-required, not Windows-local proof.',
      '- Adapter dispatch, platform enforcement, and child delivery stay unclaimed.',
      '',
    ].join('\n')
  );
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-apple-ci-platform-proof-preflight-proof-ok');
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

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
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
