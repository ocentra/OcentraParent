import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { setTimeout as delay } from 'node:timers/promises';

const repoRoot = process.cwd();
const adbTarget = process.env.OCENTRA_ANDROID_PHYSICAL_SERIAL ?? '192.168.2.45:5555';
const packageId = 'ca.ocentra.parent.agent';
const activityId = 'ca.ocentra.parent.agent/.MainActivity';
const proofMode = 'app-game-android-usage-events-granted-sample-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '218-app-game-android-usage-events-granted-sample-proof'
);
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

  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'app-game-android-usage-events-granted-sample-proof',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(...npmCommand(['run', 'release:package:android']));
  assertFileExists(apkPath, 'Android debug APK');

  await adb(['connect', adbTarget]);
  const devices = await adb(['devices', '-l']);
  assertIncludes(devices.stdout, 'product:star2qltecs', 'physical Android product');
  assertIncludes(devices.stdout, 'model:SM_G965W', 'physical Android model');

  await adb(['-s', adbTarget, 'install', '-r', apkPath]);
  await adb(['-s', adbTarget, 'shell', 'appops', 'set', packageId, 'GET_USAGE_STATS', 'allow'], {
    allowFailure: true,
  });

  const sourceState = await observeGrantedSampleState();
  if (sourceState.permissionCheckState !== 'usage-stats-granted') {
    throw new Error(`Expected UsageStats grant, received ${sourceState.permissionCheckState}`);
  }
  if (sourceState.sampleState !== 'sample-observed' || sourceState.sampleEventCount < 1) {
    throw new Error(`Expected count-only UsageEvents sample, received ${JSON.stringify(sourceState)}`);
  }

  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-usage-events-granted-sample-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidUsageEventsGrantedSampleProof({
    sampleEventCount: sourceState.sampleEventCount,
    foregroundEventCount: sourceState.foregroundEventCount,
    checkedAt: '2026-06-08T21:40:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidUsageEventsGrantedSampleProof(readModel);

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
      contract: 'packages/parent-domain/src/app-game-android-usage-events-granted-sample-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-usage-events-granted-sample-proof.test.ts',
      androidRuntime:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      packageInstall: 'adb install -r target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      appOpsGrant: 'adb shell appops set ca.ocentra.parent.agent GET_USAGE_STATS allow',
      packageLaunch: `adb shell am start -n ${activityId}`,
      uiDump: 'uiautomator dump observed granted/sample-count state; raw UI XML is redacted in command log',
    },
    claimsProved: [
      'Android UsageStats AppOps can be granted for the debug child-agent package on the physical Samsung Galaxy S9 proof target',
      'The package-local runtime preflight observes a count-only UsageEvents sample after launch',
      'Parent-domain records only permission state, sample state, and counts without raw UsageEvents rows, package names, or activity rows',
    ],
    claimsNotProved: [
      'Device Owner or Profile Owner authority',
      'Play policy or store distribution proof',
      'Provider delivery, child-device delivery outside the package, adapter dispatch, or platform enforcement',
      'Raw UsageEvents, package name, activity, UI XML, or device serial custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(
    join(appGameProofDir, '10-validation-commands.log'),
    `${commands.map(redactCommandRecord).join('\n\n')}\n`
  );

  console.log('app-game-android-usage-events-granted-sample-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function observeGrantedSampleState() {
  let lastState = null;
  for (let attempt = 0; attempt < 3; attempt += 1) {
    await adb(['-s', adbTarget, 'shell', 'am', 'start', '-S', '-n', activityId]);
    await delay(2500);
    const appOps = await adb(['-s', adbTarget, 'shell', 'appops', 'get', packageId, 'GET_USAGE_STATS'], {
      allowFailure: true,
    });
    const uiDump = await adb(['-s', adbTarget, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], {
      allowFailure: true,
    });
    lastState = parseRuntimeState({
      appOpsText: `${appOps.stdout}${appOps.stderr}`,
      uiText: uiDump.stdout,
    });
    if (lastState.permissionCheckState === 'usage-stats-granted' && lastState.sampleState === 'sample-observed') {
      return lastState;
    }
    await delay(1500);
  }
  return lastState;
}

function parseRuntimeState({ appOpsText, uiText }) {
  const permissionCheckState = parsePermissionState(appOpsText, uiText);
  const sampleState = parseSampleState(uiText, permissionCheckState);
  return {
    appOpsGrantObserved: /\ballow\b/i.test(appOpsText) || uiText.includes('usage-stats-granted'),
    uiStateObserved: uiText.includes('Ocentra Parent Agent') && uiText.includes(permissionCheckState),
    permissionCheckState,
    sampleState,
    sampleEventCount: parseCount(uiText, 'sampleEventCount'),
    foregroundEventCount: parseCount(uiText, 'foregroundEventCount'),
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

function parseCount(uiText, key) {
  const match = uiText.match(new RegExp(`${key}=([0-9]+)`));
  return match ? Number.parseInt(match[1], 10) : 0;
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
    '# WP218 Android UsageEvents granted sample proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    '- ADB target ref: `android-physical-adb-device-ref`',
    '- AppOps grant command: `GET_USAGE_STATS allow`',
    `- UI state observed: \`${sourceState.uiStateObserved}\``,
    `- Permission state: \`${sourceState.permissionCheckState}\``,
    `- Sample state: \`${sourceState.sampleState}\``,
    `- Sample event count: \`${sourceState.sampleEventCount}\``,
    `- Foreground event count: \`${sourceState.foregroundEventCount}\``,
    '',
  ].join('\n');
}

function redactCommandRecord(record) {
  const rendered = record.commandLine
    .replace(adbTarget, 'android-physical-adb-device-ref')
    .replace(apkPath, 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk');
  return [rendered, `exit=${record.status}`, redactOutput(record.stdout), redactOutput(record.stderr)]
    .filter(Boolean)
    .join('\n');
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
    match.includes('sample-unavailable') ||
    match.includes('sampleEventCount=') ||
    match.includes('foregroundEventCount=')
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
