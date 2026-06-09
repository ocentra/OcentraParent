import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-authority-preflight-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '192-app-game-android-authority-preflight');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });

  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-android-authority-preflight',
    'app-game-android-physical-device-proof',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  const androidProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-android-physical-device-proof', 'proof.json')
  );
  const authorityModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-authority-preflight.js')).href
  );
  const authorityPreflight = authorityModule.createAppGameAndroidAuthorityPreflightReadModel({
    androidProof: androidProof.readModel,
    generatedAt: '2026-06-08T17:55:00.000Z',
  });
  const authoritySummary = authorityModule.summarizeAppGameAndroidAuthorityPreflightReadModel(authorityPreflight);

  assertEqual(authoritySummary.authorityState, 'authority-not-enrolled', 'Android authority state');
  assertEqual(authoritySummary.dispatchableActionCount, 0, 'Android dispatchable action count');
  assertPositive(authoritySummary.blockedActionCount, 'Android blocked action count');
  assertIncludes(authorityPreflight.openBlockers, 'android-device-owner-not-proved', 'Device Owner blocker');
  assertIncludes(authorityPreflight.openBlockers, 'android-profile-owner-not-proved', 'Profile Owner blocker');

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    authorityPreflight,
    authoritySummary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-android-authority-preflight.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-authority-preflight.test.ts',
      androidPhysicalProof: 'test-results/app-game-android-physical-device-proof/proof.json',
    },
    claimsProved: [
      'Android package policy actions are represented as machine-readable authority preflight rows',
      'The current physical Android device remains blocked before adapter dispatch because Device Owner/Profile Owner proof is absent',
      'not-proved policy states are not treated as owner proof',
    ],
    claimsNotProved: [
      'Android Device Owner or Profile Owner enrollment',
      'Android hide, suspend, uninstall block, lock task, or managed configuration execution',
      'Android adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
      'Raw package names or raw device serial custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);

  console.log('app-game-android-authority-preflight-proof-ok');
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

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
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

function assertPositive(actual, label) {
  if (actual <= 0) {
    throw new Error(`${label}: expected positive count, received ${actual}`);
  }
}

function assertIncludes(values, expected, label) {
  if (!values.includes(expected)) {
    throw new Error(`${label}: expected ${expected}`);
  }
}
