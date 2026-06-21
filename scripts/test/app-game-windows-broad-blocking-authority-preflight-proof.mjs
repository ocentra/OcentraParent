import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-windows-broad-blocking-authority-preflight-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '193-app-game-windows-broad-blocking-authority-preflight'
);
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      'app-game-windows-broad-blocking-authority-preflight',
      'app-game-broad-blocking-proof-gates',
      'v0-8-os-adapter-manual-artifact-gates',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));

  const preflightModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'app-game-domain', 'dist', 'app-game-windows-broad-blocking-authority-preflight.js')
    ).href
  );
  const preflight = preflightModule.createAppGameWindowsBroadBlockingAuthorityPreflightReadModel({
    generatedAt: '2026-06-08T18:05:00.000Z',
  });
  const summary = preflightModule.summarizeAppGameWindowsBroadBlockingAuthorityPreflightReadModel(preflight);

  assertEqual(summary.authorityState, 'host-visible-policy-proof-missing', 'Windows authority state');
  assertEqual(summary.windowsHostProbeAttached, true, 'Windows host probe attached');
  assertEqual(summary.dispatchableActionCount, 0, 'Windows dispatchable action count');
  assertEqual(summary.blockedActionCount, 5, 'Windows blocked action count');
  assertIncludes(preflight.openBlockers, 'windows-applocker-enforce-not-proved', 'AppLocker enforce blocker');
  assertIncludes(preflight.openBlockers, 'windows-app-control-not-proved', 'App Control blocker');
  assertIncludes(
    preflight.openBlockers,
    'windows-adapter-dispatch-blocked-before-authority',
    'adapter dispatch blocker'
  );

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    preflight,
    summary,
    evidence: {
      contract: 'packages/app-game-domain/src/app-game-windows-broad-blocking-authority-preflight.ts',
      contractTest: 'packages/app-game-domain/tests/unit/app-game-windows-broad-blocking-authority-preflight.test.ts',
      broadBlockingGateData: 'packages/app-game-domain/src/app-game-broad-blocking-proof-gate-data.ts',
      manualArtifactGates: 'packages/schema-domain/src/v0-8-os-adapter-manual-artifact-gates.ts',
    },
    claimsProved: [
      'Windows broad launch blocking now has an app-game-domain authority preflight over the existing AppLocker/App Control broad-blocking gates',
      'The current Windows host can attach only the parent-safe host probe ref',
      'AppLocker/App Control enforce proof, system-app allowlist proof, rollback proof, and audit custody proof remain required before adapter dispatch',
    ],
    claimsNotProved: [
      'Windows AppLocker enforce policy application',
      'Windows App Control policy application',
      'Windows system-app allowlist execution',
      'Windows broad installed-app launch blocking',
      'Adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
      'Raw executable path custody or raw policy XML custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-windows-broad-blocking-authority-preflight-proof-ok');
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

function assertIncludes(values, expected, label) {
  if (!values.includes(expected)) {
    throw new Error(`${label}: expected ${expected}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
