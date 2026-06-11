import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-android-system-geofence-blocker-proof';
const output09 = join(repoRoot, 'output', 'tracking-plan-proof', '09-android-background-location-and-geofence-adapter');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultRoot = join(repoRoot, 'test-results', proofMode);
const sourceAndroidEmulatorProofRef = 'test-results/tracking-plan-android-emulator-proof/proof.json';
const androidEmulatorProofPath = join(repoRoot, sourceAndroidEmulatorProofRef);
const generatedAt = '2026-06-07T15:45:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output09, { recursive: true });
  await mkdir(output33, { recursive: true });

  runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-android-system-geofence-blocker-proof',
  ]);

  const androidEmulatorProof = JSON.parse(await readFile(androidEmulatorProofPath, 'utf8'));
  const proof = await buildProof(androidEmulatorProof);
  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-android-system-geofence-blocker-proof-ok');
  console.log('evidence=test-results/tracking-android-system-geofence-blocker-proof/proof.json');
}

async function buildProof(androidEmulatorProof) {
  const proofModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-android-system-geofence-blocker-proof.js')
    ).href
  );
  return {
    ...proofModule.buildTrackingAndroidSystemGeofenceBlockerProof(
      generatedAt,
      sourceAndroidEmulatorProofRef,
      androidEmulatorProof
    ),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp09: 'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/26-android-system-geofence-blocker-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/44-android-system-geofence-blocker-proof.json',
      evidence: 'test-results/tracking-android-system-geofence-blocker-proof/proof.json',
      sourceAndroidEmulatorProof: sourceAndroidEmulatorProofRef,
    },
  };
}

function assertProof(proof) {
  const [row] = proof.rows;
  if (row.systemProximityTransitionCount !== 0 || row.androidSystemGeofenceDeliveryClaimed) {
    throw new Error(`Android system geofence delivery was overclaimed: ${JSON.stringify(row)}`);
  }
  if (row.localListenerGeofenceTransitionCount <= 0 || !row.systemProximityRegistered) {
    throw new Error(`Android emulator geofence prerequisite evidence is missing: ${JSON.stringify(row)}`);
  }
  if (row.localEvidenceArtifactRefs.length === 0) {
    throw new Error(`Android system geofence blocker proof has no local evidence refs: ${JSON.stringify(row)}`);
  }
  if (row.requiredRuntimeArtifactRefs.length === 0 || row.presentRuntimeArtifactRefs.length !== 0) {
    throw new Error(`Android system geofence runtime artifacts were overclaimed: ${JSON.stringify(row)}`);
  }
  if (row.missingRuntimeArtifactRefs.length !== row.requiredRuntimeArtifactRefs.length) {
    throw new Error(`Android system geofence runtime artifact accounting is incomplete: ${JSON.stringify(row)}`);
  }
  if (row.runtimeArtifactSetComplete) {
    throw new Error(`Android system geofence runtime artifact set was marked complete: ${JSON.stringify(row)}`);
  }
}

async function writeProofArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'android-system-geofence-blocker-read-model.json'), proof.rows);
  await writeJson(join(output09, '26-android-system-geofence-blocker-proof.json'), proof);
  await writeJson(join(output33, '44-android-system-geofence-blocker-proof.json'), proof);
  await writeFile(join(output09, '26-android-system-geofence-blocker-validation.log'), validationLog());
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
