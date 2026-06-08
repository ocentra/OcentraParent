import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { access, mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-cross-platform-runtime-capability-proof';
const generatedAt = new Date().toISOString();
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const proofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output31 = path.join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '31-platform-extension-checklists-and-proof-routing'
);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commandResults = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });
  await mkdir(output31, { recursive: true });
  await mkdir(output33, { recursive: true });

  runRequired('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  runRequired('cmd', ['/c', 'npm', 'run', 'test', '--workspace', '@ocentra-parent/parent-domain', '--', proofMode]);

  const proofModule = await importDist('tracking-cross-platform-runtime-capability-proof.js');
  const probes = await collectProbes();
  const sources = await collectSourceProofs();
  const readModel = proofModule.buildTrackingCrossPlatformRuntimeCapabilityProof(
    generatedAt,
    rowsFrom(probes, sources)
  );
  const proof = buildProof(readModel, probes, sources);
  assertProof(proof);
  await writeArtifacts(proof, probes);

  console.log('tracking-cross-platform-runtime-capability-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function collectProbes() {
  const dockerPath = await firstExistingPath([
    'docker',
    'C:\\Program Files\\Docker\\Docker\\resources\\bin\\docker.exe',
  ]);
  return {
    windows: run('windows-host-toolchain', 'cmd', [
      '/c',
      'ver & where node & node --version & where npm & npm --version & where cargo & cargo --version',
    ]),
    wsl: run('wsl-linux-toolchain', 'wsl.exe', [
      'sh',
      '-lc',
      'uname -a && node --version && npm --version && cargo --version && git --version',
    ]),
    wslDistros: run('wsl-distros', 'wsl.exe', ['-l', '-v']),
    dockerPath,
    dockerVersion:
      dockerPath === null
        ? unavailable('docker-version', 'Docker CLI not found')
        : run('docker-version', dockerPath, ['--version']),
    dockerDaemon:
      dockerPath === null
        ? unavailable('docker-daemon', 'Docker CLI not found')
        : run('docker-daemon', dockerPath, ['version', '--format', '{{.Server.Version}}']),
    android: run('android-toolchain', 'cmd', ['/c', 'where adb & adb version & where java & java -version']),
  };
}

async function collectSourceProofs() {
  return {
    android: await readOptionalJson('test-results/tracking-plan-android-emulator-proof/proof.json'),
    androidInventory: await readOptionalJson(
      'test-results/tracking-android-emulator-artifact-inventory-proof/proof.json'
    ),
    androidPhysical: await readOptionalJson('test-results/tracking-android-physical-device-runtime-proof/proof.json'),
    wsl: await readOptionalJson('test-results/tracking-plan-wsl-local-proof/proof.json'),
    iosSimulator: await readOptionalJson('test-results/tracking-ios-simulator-artifact-inventory-proof/proof.json'),
    iosRoute: await readOptionalJson('test-results/tracking-plan-ios-simulator-proof/proof.json'),
  };
}

function rowsFrom(probes, sources) {
  const dockerObserved = probes.dockerVersion.exitCode === 0 && probes.dockerDaemon.exitCode === 0;
  const androidArtifacts = sources.androidPhysical?.summary?.presentArtifactCount ?? 0;
  return [
    {
      area: 'windows-host-toolchain',
      status: statusFrom(probes.windows),
      proofRef: 'test-results/tracking-cross-platform-runtime-capability-proof/windows-host-toolchain.json',
      sourceRefs: [],
      currentProofTier: 'P3_LOCAL_WINDOWS_HOST',
      requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
      observedTooling: lines(probes.windows.stdout),
      passedAssertions: probes.windows.exitCode === 0 ? ['Windows host shell, Node, npm, and Cargo are reachable'] : [],
      remainingBlockers: ['Windows host proof is not approved-manual desktop precise location proof'],
      artifactCount: lines(probes.windows.stdout).length,
      ciRunnable: true,
      localRuntimeClaimed: probes.windows.exitCode === 0,
    },
    {
      area: 'wsl-linux-replay',
      status: statusFrom(probes.wsl),
      proofRef: 'test-results/tracking-plan-wsl-local-proof/proof.json',
      sourceRefs: ['output/tracking-plan-proof/wsl-local-replay/proof.json'],
      currentProofTier: sources.wsl?.currentProofTier ?? 'P3_LOCAL_DEV_MACHINE',
      requiredProofTier: sources.wsl?.requiredProofTier ?? 'P3_LOCAL_DEV_MACHINE',
      observedTooling: lines(`${probes.wslDistros.stdout}\n${probes.wsl.stdout}`),
      passedAssertions: probes.wsl.exitCode === 0 ? ['WSL2 Ubuntu Linux toolchain is reachable'] : [],
      remainingBlockers: ['WSL replay does not prove mobile physical-device or production runtime behavior'],
      artifactCount: sources.wsl?.commands?.length ?? 0,
      ciRunnable: true,
      localRuntimeClaimed: probes.wsl.exitCode === 0,
    },
    {
      area: 'docker-container-runtime',
      status: dockerObserved ? 'local-proof-passed' : 'host-tool-unavailable',
      proofRef: 'test-results/tracking-cross-platform-runtime-capability-proof/docker-container-runtime.json',
      sourceRefs: [],
      currentProofTier: dockerObserved ? 'P3_LOCAL_CONTAINER_RUNTIME' : 'P2_HOST_TOOL_DISCOVERED',
      requiredProofTier: 'P3_LOCAL_CONTAINER_RUNTIME',
      observedTooling: lines(`${probes.dockerPath ?? 'Docker CLI not found'}\n${probes.dockerVersion.stdout}`),
      passedAssertions: dockerObserved ? ['Docker CLI and daemon are reachable'] : [],
      remainingBlockers: dockerObserved
        ? ['Docker smoke is infrastructure proof, not tracking product runtime proof']
        : ['Docker CLI or daemon is not currently reachable from this shell'],
      artifactCount: dockerObserved ? 2 : 1,
      ciRunnable: true,
      localRuntimeClaimed: dockerObserved,
    },
    {
      area: 'android-emulator-runtime',
      status: sources.android === null ? 'host-tool-unavailable' : 'local-proof-passed',
      proofRef: 'test-results/tracking-plan-android-emulator-proof/proof.json',
      sourceRefs: ['test-results/tracking-android-emulator-artifact-inventory-proof/proof.json'],
      currentProofTier: sources.android?.currentProofTier ?? 'P3_LOCAL_ANDROID_EMULATOR',
      requiredProofTier: sources.android?.requiredProofTier ?? 'P4_PHYSICAL_DEVICE',
      observedTooling: lines(probes.android.stdout),
      passedAssertions: sources.android === null ? [] : ['Android emulator tracking proof artifacts are present'],
      remainingBlockers: ['Android system geofence/dwell and physical behavior remain separately gated'],
      artifactCount: sources.androidInventory?.summary?.presentArtifactCount ?? 0,
      ciRunnable: false,
      localRuntimeClaimed: sources.android !== null,
    },
    {
      area: 'android-physical-device-status',
      status: sources.androidPhysical === null ? 'host-tool-unavailable' : 'local-proof-passed',
      proofRef: 'test-results/tracking-android-physical-device-runtime-proof/proof.json',
      sourceRefs: [],
      currentProofTier: 'P4_PHYSICAL_DEVICE_STATUS_ONLY',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      observedTooling: ['Samsung S9 adb package/service/status evidence'],
      passedAssertions:
        sources.androidPhysical === null ? [] : ['Android physical package/service/status artifacts are present'],
      remainingBlockers: ['Physical location/geofence behavior and Android system geofence delivery remain unclaimed'],
      artifactCount: androidArtifacts,
      ciRunnable: false,
      localRuntimeClaimed: sources.androidPhysical !== null,
    },
    {
      area: 'macos-ios-ci-manual-routing',
      status: 'ci-manual-required',
      proofRef: 'test-results/tracking-ios-simulator-artifact-inventory-proof/proof.json',
      sourceRefs: ['test-results/tracking-plan-ios-simulator-proof/proof.json'],
      currentProofTier: 'P2_CI_OR_MANUAL_REQUIRED',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      observedTooling: ['macOS package-preview job routing', 'iOS simulator proof routing'],
      passedAssertions: ['macOS/iOS work is routed to CI/manual proof because this host is Windows'],
      remainingBlockers: ['macOS and iOS runtime proof cannot execute on this Windows host'],
      artifactCount: sources.iosSimulator?.summary?.requiredArtifactCount ?? 0,
      ciRunnable: true,
      localRuntimeClaimed: false,
    },
  ];
}

function buildProof(readModel, probes, sources) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    workpackIds: [
      '08-android-foreground-location-adapter',
      '09-android-background-location-and-geofence-adapter',
      '10-android-battery-connectivity-and-status-adapter',
      '11-ios-foreground-location-adapter',
      '12-ios-background-region-adapter',
      '13-desktop-presence-signals',
      '31-platform-extension-checklists-and-proof-routing',
      '33-proof-gates-fixtures-rollout-and-pr-gate',
    ],
    readModel,
    summary: readModel.summary,
    productClaims: readModel.productClaims,
    probes,
    sourceProofsPresent: Object.fromEntries(Object.entries(sources).map(([key, value]) => [key, value !== null])),
    nonClaims: [
      'This proof does not claim Android physical location or system geofence behavior.',
      'This proof does not claim iOS Core Location, macOS precise location, authority enrollment, provider delivery, production workers, or product readiness.',
      'Docker availability is recorded only when the CLI and daemon are reachable.',
    ],
    commands: commandResults,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 6);
  assert.equal(proof.productClaims.windowsHostToolchainObserved, true);
  assert.equal(proof.productClaims.wslLinuxReplayObserved, true);
  assert.equal(proof.productClaims.androidEmulatorRuntimeObserved, true);
  assert.equal(proof.productClaims.androidPhysicalStatusObserved, true);
  assert.equal(proof.productClaims.physicalDeviceBehaviorClaimed, false);
  assert.equal(proof.productClaims.productClaimReady, false);
}

async function writeArtifacts(proof, probes) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(resultDir, 'windows-host-toolchain.json'), probes.windows);
  await writeJson(path.join(resultDir, 'docker-container-runtime.json'), {
    path: probes.dockerPath,
    version: probes.dockerVersion,
    daemon: probes.dockerDaemon,
  });
  await writeJson(path.join(proofDir, 'proof.json'), proof);
  await writeJson(path.join(proofDir, 'read-model.json'), proof.readModel);
  await writeFile(path.join(proofDir, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(path.join(proofDir, '16-validation-commands.log'), commandLog(), 'utf8');
  await writeJson(path.join(output31, '24-cross-platform-runtime-capability-proof.json'), proof);
  await writeJson(path.join(output33, '75-cross-platform-runtime-capability-proof.json'), proof);
  await writeFile(path.join(output33, '75-cross-platform-runtime-capability-validation.log'), commandLog(), 'utf8');
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Cross-Platform Runtime Capability Proof',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.commit}`,
    `- localProofPassedRows: ${proof.summary.localProofPassedRows}`,
    `- hostToolUnavailableRows: ${proof.summary.hostToolUnavailableRows}`,
    `- ciManualRequiredRows: ${proof.summary.ciManualRequiredRows}`,
    '- Windows, WSL/Linux, Android emulator, and Android physical status are host-verifiable here.',
    '- macOS/iOS remain CI/manual-routed on this Windows host.',
    '- Docker is only claimed when CLI and daemon are both reachable.',
    '',
  ].join('\n');
}

function runRequired(command, args) {
  const result = run(`required-${command}-${args.at(-1) ?? 'command'}`, command, args, { inherit: true });
  if (result.exitCode !== 0) throw new Error(`Command failed: ${command} ${args.join(' ')}`);
}

function run(id, command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    stdio: options.inherit ? 'inherit' : 'pipe',
  });
  const entry = {
    id,
    command: [command, ...args].join(' '),
    exitCode: result.status ?? 1,
    stdout: trim(result.stdout ?? ''),
    stderr: trim(result.stderr ?? ''),
  };
  commandResults.push(entry);
  return entry;
}

function unavailable(id, message) {
  const entry = { id, command: 'unavailable', exitCode: 127, stdout: '', stderr: message };
  commandResults.push(entry);
  return entry;
}

async function firstExistingPath(candidates) {
  for (const candidate of candidates) {
    if (candidate === 'docker') {
      const result = run('docker-path-probe', 'cmd', ['/c', 'where docker']);
      if (result.exitCode === 0) return result.stdout.split(/\r?\n/u)[0];
      continue;
    }
    try {
      await access(candidate);
      return candidate;
    } catch {
      continue;
    }
  }
  return null;
}

function statusFrom(result) {
  return result.exitCode === 0 ? 'local-proof-passed' : 'host-tool-unavailable';
}

function lines(value) {
  return value
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter((line) => line.length > 0)
    .slice(0, 12);
}

async function readOptionalJson(relativeFile) {
  try {
    return JSON.parse(await readFile(path.join(repoRoot, relativeFile), 'utf8'));
  } catch {
    return null;
  }
}

function importDist(name) {
  return import(pathToFileURL(path.join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(filePath, value) {
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function commandLog() {
  return `${commandResults
    .map((entry) => [`$ ${entry.command}`, `exitCode=${entry.exitCode}`, entry.stdout, entry.stderr].join('\n'))
    .join('\n\n')}\n`;
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}

function trim(value) {
  return value.length > 4000 ? `${value.slice(0, 4000)}\n...[truncated]` : value;
}
