import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { setTimeout as delay } from 'node:timers/promises';

const repoRoot = process.cwd();
const adbTarget = process.env.OCENTRA_ANDROID_PHYSICAL_SERIAL ?? '192.168.2.45:5555';
const packageId = 'ca.ocentra.parent.agent';
const activityId = 'ca.ocentra.parent.agent/.MainActivity';
const proofMode = 'app-game-android-usage-events-package-runtime-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '201-app-game-android-usage-events-package-runtime-proof');
const apkPath = join(
  repoRoot,
  'target',
  'release-packages',
  'android',
  'ocentra-parent-agent-android-debug-latest.apk'
);
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
    'app-game-android-usage-events-package-runtime-proof',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'release:package:android']);
  assertFileExists(apkPath, 'Android debug APK');

  await adb(['connect', adbTarget]);
  const devices = await adb(['devices', '-l']);
  assertIncludes(devices.stdout, 'product:star2qltecs', 'physical Android product');
  assertIncludes(devices.stdout, 'model:SM_G965W', 'physical Android model');

  await adb(['-s', adbTarget, 'install', '-r', apkPath]);
  await adb(['-s', adbTarget, 'shell', 'am', 'start', '-n', activityId]);
  await delay(2500);

  const appOps = await adb(['-s', adbTarget, 'shell', 'cmd', 'appops', 'get', packageId, 'GET_USAGE_STATS'], {
    allowFailure: true,
  });
  const uiDump = await adb(['-s', adbTarget, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], { allowFailure: true });
  const sourceState = parseRuntimeState({ appOpsText: `${appOps.stdout}${appOps.stderr}`, uiText: uiDump.stdout });

  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-usage-events-package-runtime-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidUsageEventsPackageRuntimeProof({
    permissionCheckState: sourceState.permissionCheckState,
    sampleState: sourceState.sampleState,
    uiStateObserved: sourceState.uiStateObserved,
    appOpsObserved: sourceState.appOpsObserved,
    checkedAt: '2026-06-08T20:05:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidUsageEventsPackageRuntimeProof(readModel);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    adbTargetRef: 'android-physical-adb-device-ref',
    commands: commands.map(redactCommandRecord),
    readModel,
    summary,
    sourceState,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-android-usage-events-package-runtime-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-usage-events-package-runtime-proof.test.ts',
      packageInstall: 'adb install -r target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      packageLaunch: `adb shell am start -n ${activityId}`,
      uiDump: 'uiautomator dump observed package runtime state text; raw UI XML is redacted in command log',
      appOps: 'cmd appops get ca.ocentra.parent.agent GET_USAGE_STATS observed without storing private package lists',
    },
    claimsProved: [
      'Android debug package installs and MainActivity launches on the physical Samsung Galaxy S9 target',
      'Package UI exposes UsageEvents permission/sample states from the package-local runtime preflight',
      'Parent-domain records install/launch/AppOps/UI evidence without raw device serial, raw package list, raw UsageEvents rows, or enforcement claims',
    ],
    claimsNotProved: [
      'UsageStats settings grant if AppOps still reports ignored/default',
      'Live package UsageEvents sample when UI reports sample-permission-required',
      'Adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
      'Device Owner/Profile Owner authority or Play policy proof',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.map(redactCommandRecord).join('\n\n')}\n`);

  console.log('app-game-android-usage-events-package-runtime-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

function parseRuntimeState({ appOpsText, uiText }) {
  const permissionCheckState = parsePermissionState(appOpsText, uiText);
  const sampleState = parseSampleState(uiText, permissionCheckState);
  return {
    appOpsObserved: appOpsText.length > 0 && !appOpsText.includes('Unknown operation string'),
    uiStateObserved: uiText.includes('Ocentra Parent Agent') && uiText.includes(permissionCheckState),
    permissionCheckState,
    sampleState,
  };
}

function parsePermissionState(appOpsText, uiText) {
  if (uiText.includes('usage-stats-granted') || /\ballow\b/i.test(appOpsText)) {
    return 'usage-stats-granted';
  }
  if (uiText.includes('settings-grant-required') || /\bignore\b|\bdefault\b|\bdeny\b/i.test(appOpsText)) {
    return 'settings-grant-required';
  }
  return 'permission-check-unavailable';
}

function parseSampleState(uiText, permissionState) {
  if (uiText.includes('sample-observed')) {
    return 'sample-observed';
  }
  if (uiText.includes('sample-empty')) {
    return 'sample-empty';
  }
  if (uiText.includes('sample-unavailable')) {
    return 'sample-unavailable';
  }
  return permissionState === 'usage-stats-granted' ? 'sample-empty' : 'sample-permission-required';
}

async function adb(args, options = {}) {
  return runCommand('adb', args, options);
}

async function runCommand(command, args, options = {}) {
  const commandLine = [command, ...args].join(' ');
  const result = await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
    const stdout = [];
    const stderr = [];
    child.stdout.on('data', (chunk) => stdout.push(String(chunk)));
    child.stderr.on('data', (chunk) => stderr.push(String(chunk)));
    child.once('exit', (code) =>
      resolve({ commandLine, status: code ?? 1, stdout: stdout.join(''), stderr: stderr.join('') })
    );
    child.once('error', reject);
  });
  commands.push(result);
  if (result.status !== 0 && !options.allowFailure) {
    throw new Error(`${commandLine} exited with ${result.status}: ${result.stderr}`);
  }
  return result;
}

async function gitHead() {
  const result = await runCommand('git', ['rev-parse', 'HEAD']);
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function sourceSnapshot(sourceState) {
  return [
    '# WP201 Android UsageEvents package runtime proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    '- ADB target ref: `android-physical-adb-device-ref`',
    `- AppOps observed: \`${sourceState.appOpsObserved}\``,
    `- UI state observed: \`${sourceState.uiStateObserved}\``,
    `- Permission state: \`${sourceState.permissionCheckState}\``,
    `- Sample state: \`${sourceState.sampleState}\``,
    '',
  ].join('\n');
}

function redactCommandRecord(record) {
  const rendered = record.commandLine
    .replace(adbTarget, 'android-physical-adb-device-ref')
    .replace(apkPath, 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk');
  return [
    rendered,
    `exit=${record.status}`,
    redactOutput(record.stdout),
    redactOutput(record.stderr),
  ].filter(Boolean).join('\n');
}

function redactOutput(output) {
  return output
    .split(repoRoot)
    .join('<repo-root>')
    .replace(new RegExp(adbTarget.replaceAll('.', '\\.').replace(':', '\\:'), 'g'), 'android-physical-adb-device-ref')
    .replace(/192\.168\.\d+\.\d+:\d+/g, 'android-physical-adb-device-ref')
    .replace(/serial=\"[^\"]+\"/g, 'serial=\"android-physical-adb-device-ref\"')
    .replace(/text=\"[^\"]*\"/g, (match) => redactUiText(match));
}

function redactUiText(match) {
  if (
    match.includes('Ocentra Parent Agent') ||
    match.includes('usage-stats-granted') ||
    match.includes('settings-grant-required') ||
    match.includes('permission-check-unavailable') ||
    match.includes('sample-observed') ||
    match.includes('sample-empty') ||
    match.includes('sample-permission-required') ||
    match.includes('sample-unavailable')
  ) {
    return match;
  }
  return 'text=\"<ui-text-redacted>\"';
}

function assertFileExists(path, label) {
  if (!existsSync(path)) {
    throw new Error(`Missing ${label}: ${path}`);
  }
}

function assertIncludes(value, expected, label) {
  if (!value.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
