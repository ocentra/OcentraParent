import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-local-execution-strategy-proof';
const generatedAt = new Date().toISOString();
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const namedOutput = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const commands = [];
const proofRefs = {
  androidEmulator: 'test-results/tracking-plan-android-emulator-proof/proof.json',
  androidPhysical: 'test-results/tracking-android-physical-device-runtime-proof/proof.json',
  androidPhysicalReview: 'test-results/tracking-physical-device-evidence-review-proof/proof.json',
  crossPlatform: 'test-results/tracking-cross-platform-runtime-capability-proof/proof.json',
  hostedUi: 'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json',
  localPlatformBatch: 'test-results/tracking-local-platform-proof-batch/proof.json',
  realRuntimeHandoff: 'test-results/tracking-real-runtime-handoff-proof/proof.json',
  claimAudit: 'test-results/tracking-claim-audit-proof/proof.json',
  iosSimulator: 'test-results/tracking-plan-ios-simulator-proof/proof.json',
  iosInventory: 'test-results/tracking-ios-simulator-artifact-inventory-proof/proof.json',
  wsl: 'test-results/tracking-plan-wsl-local-proof/proof.json',
};

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(output33, { recursive: true });
  await mkdir(namedOutput, { recursive: true });

  runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  runNpmCommand(run, ['run', 'test', '--workspace', '@ocentra-parent/parent-domain', '--', proofMode]);

  const host = await hostEvidence();
  const proofModule = await importDist('tracking-local-execution-strategy-proof.js');
  const readModel = proofModule.buildTrackingLocalExecutionStrategyProof(generatedAt, rows(host));
  const proof = {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    host,
    readModel,
    rows: readModel.rows,
    summary: readModel.summary,
    productClaims: readModel.productClaims,
    sourceProofRefs: readModel.sourceProofRefs,
    commands,
    artifactPaths: {
      evidence: 'test-results/tracking-local-execution-strategy-proof/proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/74-local-execution-strategy-proof.json',
      namedProofRoot: 'output/tracking-plan-proof/tracking-local-execution-strategy-proof/proof.json',
    },
  };

  await assertProof(proof);
  await writeArtifacts(proof);
  console.log('tracking-local-execution-strategy-proof-ok');
  console.log('evidence=test-results/tracking-local-execution-strategy-proof/proof.json');
}

async function hostEvidence() {
  const windows = probeCommand('cmd', ['/c', 'ver']);
  const wsl = probeCommand('wsl.exe', ['--status']);
  const docker = probeCommand('docker', ['--version']);
  const adb = probeCommand('adb', ['version']);
  await writeJson('windows-host.json', hostProbeRow('windows-host', windows));
  await writeJson('wsl-host.json', hostProbeRow('wsl-host', wsl));
  await writeJson('docker-host.json', hostProbeRow('docker-host', docker));
  await writeJson('android-adb-host.json', hostProbeRow('android-adb-host', adb));
  return {
    windows: hostProbeRow('windows-host', windows),
    wsl: hostProbeRow('wsl-host', wsl),
    docker: hostProbeRow('docker-host', docker),
    adb: hostProbeRow('android-adb-host', adb),
  };
}

function rows(host) {
  return [
    windowsHostRow(host),
    wslRow(host),
    dockerRow(host),
    androidEmulatorRow(host),
    androidPhysicalStatusRow(),
    macosIosCiRow(),
    physicalManualRuntimeRow(),
    finalSyncRow(),
  ];
}

function windowsHostRow(host) {
  return {
    area: 'windows-host-local-validation',
    route: 'local-runnable',
    status: host.windows.available ? 'observed' : 'unavailable-here',
    proofRef: 'test-results/tracking-local-execution-strategy-proof/windows-host.json',
    sourceRefs: [proofRefs.localPlatformBatch, proofRefs.hostedUi],
    commandsToRunAfterCodeBatch: [
      'cmd /c npm run format:check',
      'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    ],
    evidenceRefsExpected: ['test-results/tracking-local-execution-strategy-proof/windows-host.json'],
    passedEvidenceRefs: host.windows.available
      ? ['test-results/tracking-local-execution-strategy-proof/windows-host.json']
      : [],
    blockers: host.windows.available ? [] : ['Windows command host was not available'],
    localRunnable: host.windows.available,
    ciRunnable: true,
    requiresPhysicalDevice: false,
    requiresMacHost: false,
    requiresDockerHost: false,
  };
}

function wslRow(host) {
  return {
    area: 'wsl-local-replay',
    route: host.wsl.available ? 'local-runnable' : 'unavailable-here',
    status: host.wsl.available ? 'observed' : 'unavailable-here',
    proofRef: proofRefs.wsl,
    sourceRefs: [proofRefs.localPlatformBatch, 'test-results/tracking-local-execution-strategy-proof/wsl-host.json'],
    commandsToRunAfterCodeBatch: ['cmd /c npm run test:tracking-plan-wsl-local-proof'],
    evidenceRefsExpected: [proofRefs.wsl, 'test-results/tracking-local-execution-strategy-proof/wsl-host.json'],
    passedEvidenceRefs: host.wsl.available
      ? [proofRefs.wsl, 'test-results/tracking-local-execution-strategy-proof/wsl-host.json']
      : [],
    blockers: host.wsl.available ? [] : ['WSL is not available on this host'],
    localRunnable: host.wsl.available,
    ciRunnable: true,
    requiresPhysicalDevice: false,
    requiresMacHost: false,
    requiresDockerHost: false,
  };
}

function dockerRow(host) {
  return {
    area: 'docker-host-availability',
    route: host.docker.available ? 'local-runnable' : 'unavailable-here',
    status: host.docker.available ? 'observed' : 'unavailable-here',
    proofRef: 'test-results/tracking-local-execution-strategy-proof/docker-host.json',
    sourceRefs: [],
    commandsToRunAfterCodeBatch: ['docker --version'],
    evidenceRefsExpected: ['test-results/tracking-local-execution-strategy-proof/docker-host.json'],
    passedEvidenceRefs: host.docker.available
      ? ['test-results/tracking-local-execution-strategy-proof/docker-host.json']
      : [],
    blockers: host.docker.available ? [] : ['Docker CLI is not available on the current Windows PATH'],
    localRunnable: host.docker.available,
    ciRunnable: false,
    requiresPhysicalDevice: false,
    requiresMacHost: false,
    requiresDockerHost: true,
  };
}

function androidEmulatorRow(host) {
  return {
    area: 'android-emulator-runtime',
    route: 'local-runnable',
    status: 'ready',
    proofRef: proofRefs.androidEmulator,
    sourceRefs: [proofRefs.crossPlatform, 'test-results/tracking-local-execution-strategy-proof/android-adb-host.json'],
    commandsToRunAfterCodeBatch: ['cmd /c npm run test:tracking-plan-android-emulator-proof'],
    evidenceRefsExpected: [proofRefs.androidEmulator],
    passedEvidenceRefs: [proofRefs.androidEmulator],
    blockers: host.adb.available
      ? []
      : ['ADB is not on the current Windows PATH; Android proof scripts may resolve SDK-local adb instead'],
    localRunnable: true,
    ciRunnable: false,
    requiresPhysicalDevice: false,
    requiresMacHost: false,
    requiresDockerHost: false,
  };
}

function androidPhysicalStatusRow() {
  return {
    area: 'android-physical-status-runtime',
    route: 'local-runnable',
    status: 'observed',
    proofRef: proofRefs.androidPhysical,
    sourceRefs: [proofRefs.androidPhysicalReview],
    commandsToRunAfterCodeBatch: ['cmd /c node scripts/test/tracking-android-physical-device-runtime-proof.mjs'],
    evidenceRefsExpected: [proofRefs.androidPhysical],
    passedEvidenceRefs: [proofRefs.androidPhysical],
    blockers: [
      'Physical geofence transition/dwell and Android system geofence delivery still require real movement or authority-backed runtime artifacts',
    ],
    localRunnable: true,
    ciRunnable: false,
    requiresPhysicalDevice: true,
    requiresMacHost: false,
    requiresDockerHost: false,
  };
}

function macosIosCiRow() {
  return {
    area: 'macos-ios-ci-route',
    route: 'ci-runnable',
    status: 'manual-required',
    proofRef: proofRefs.iosSimulator,
    sourceRefs: [proofRefs.iosInventory],
    commandsToRunAfterCodeBatch: ['GitHub macOS package-preview job uploads iOS simulator tracking artifacts'],
    evidenceRefsExpected: [proofRefs.iosSimulator, proofRefs.iosInventory],
    passedEvidenceRefs: [proofRefs.iosSimulator, proofRefs.iosInventory],
    blockers: ['Windows cannot run macOS/iOS simulator runtime or iOS physical-device Core Location proof locally'],
    localRunnable: false,
    ciRunnable: true,
    requiresPhysicalDevice: false,
    requiresMacHost: true,
    requiresDockerHost: false,
  };
}

function physicalManualRuntimeRow() {
  return {
    area: 'physical-manual-runtime-route',
    route: 'manual-required',
    status: 'manual-required',
    proofRef: proofRefs.realRuntimeHandoff,
    sourceRefs: [proofRefs.claimAudit],
    commandsToRunAfterCodeBatch: ['cmd /c node scripts/test/tracking-real-runtime-handoff-proof.mjs'],
    evidenceRefsExpected: [proofRefs.realRuntimeHandoff, proofRefs.claimAudit],
    passedEvidenceRefs: [proofRefs.realRuntimeHandoff, proofRefs.claimAudit],
    blockers: [
      'Android physical behavior, iOS physical behavior, child-device runtime, authority, provider, retention platform, production worker, and escalation runtime proof remain required',
    ],
    localRunnable: false,
    ciRunnable: false,
    requiresPhysicalDevice: true,
    requiresMacHost: false,
    requiresDockerHost: false,
  };
}

function finalSyncRow() {
  return {
    area: 'final-sync-validation-gate',
    route: 'final-checkpoint',
    status: 'ready',
    proofRef: proofRefs.localPlatformBatch,
    sourceRefs: [proofRefs.localPlatformBatch],
    commandsToRunAfterCodeBatch: ['git fetch origin main', 'git rebase origin/main', 'cmd /c npm run validate'],
    evidenceRefsExpected: ['test-results/tracking-local-execution-strategy-proof/proof.json'],
    passedEvidenceRefs: ['test-results/tracking-local-execution-strategy-proof/proof.json'],
    blockers: [],
    localRunnable: false,
    ciRunnable: true,
    requiresPhysicalDevice: false,
    requiresMacHost: false,
    requiresDockerHost: false,
  };
}

async function assertProof(proof) {
  assert.equal(proof.rows.length, 8, 'expected eight execution strategy rows');
  assert.equal(proof.summary.productReadyRows, 0, 'product ready must stay false');
  assert.equal(proof.productClaims.finalSyncRequiredBeforePr, true, 'final sync gate must remain required');
  assert.equal(proof.productClaims.physicalBehaviorClaimed, false, 'physical behavior must not be claimed');
  assert.equal(proof.productClaims.iosRuntimeClaimed, false, 'iOS runtime must not be claimed');
  assert.equal(proof.productClaims.childRuntimeClaimed, false, 'child runtime must not be claimed');
  assert.equal(proof.productClaims.productionRuntimeClaimed, false, 'production runtime must not be claimed');
  for (const ref of proof.sourceProofRefs.filter((ref) => !ref.includes(proofMode))) {
    await assertPathExists(ref);
  }
}

async function writeArtifacts(proof) {
  await writeJson('proof.json', proof);
  await writeJson('execution-strategy-read-model.json', proof.rows);
  await writeFile(path.join(output33, '74-local-execution-strategy-proof.json'), stringifyJson(proof));
  await writeFile(path.join(namedOutput, 'proof.json'), stringifyJson(proof));
  await writeFile(path.join(output33, '75-local-execution-strategy-validation.log'), validationLog(), 'utf8');
}

function probeCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    timeout: 30_000,
  });
  const output = sanitizeOutput(`${result.stdout ?? ''}${result.stderr ?? ''}`);
  commands.push({ command: `${command} ${args.join(' ')}`, status: result.status ?? 1, output });
  return {
    available: result.error === undefined && (result.status ?? 1) === 0,
    status: result.status ?? 1,
    output,
    error: result.error?.message ?? '',
  };
}

function hostProbeRow(name, probe) {
  return {
    name,
    available: probe.available,
    status: probe.status,
    output: probe.output.length === 0 ? 'NO_OUTPUT' : probe.output,
    error: probe.error,
  };
}

function sanitizeOutput(output) {
  return output.replaceAll('\u0000', '').trim();
}

async function assertPathExists(relativePath) {
  await stat(path.join(repoRoot, relativePath));
}

async function writeJson(name, value) {
  await writeFile(path.join(resultDir, name), stringifyJson(value));
}

function stringifyJson(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

function validationLog() {
  return `${commands
    .map((entry) => `$ ${entry.command}\nexit=${String(entry.status)}\n${entry.output}`)
    .join('\n\n')}\n`;
}

async function importDist(fileName) {
  return import(pathToFileURL(path.join(repoRoot, 'packages', 'parent-domain', 'dist', fileName)).href);
}

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    timeout: 300_000,
  });
  const output = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  commands.push({ command: `${command} ${args.join(' ')}`, status: result.status ?? 1, output: output.trim() });
  if ((result.status ?? 1) !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}\n${output}`);
  }
  return output;
}

function gitOutput(args) {
  return run('git', args).trim();
}
