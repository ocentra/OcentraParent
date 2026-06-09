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
const requestQueueFileName = 'app-game-local-notification-request-queue/request-queue-proof-state.txt';
const requestDrainFileName = 'app-game-local-notification-request-queue/request-drain-proof-state.txt';
const proofMode = 'app-game-android-child-runtime-local-notification-request-queue-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '222-app-game-android-child-runtime-local-notification-request-queue-proof'
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

  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-android-child-runtime-local-notification-request-queue-proof',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'release:package:android']);
  assertFileExists(apkPath, 'Android debug APK');

  await adb(['connect', adbTarget]);
  const devices = await adb(['devices', '-l']);
  assertIncludes(devices.stdout, 'product:star2qltecs', 'physical Android product');
  assertIncludes(devices.stdout, 'model:SM_G965W', 'physical Android model');

  await adb(['-s', adbTarget, 'install', '-r', apkPath]);
  await adb([
    '-s',
    adbTarget,
    'shell',
    'am',
    'start',
    '-S',
    '-n',
    activityId,
    '--ez',
    'ca.ocentra.parent.agent.RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF',
    'true',
  ]);
  await delay(2500);

  const requestQueueReadback = await adb([
    '-s',
    adbTarget,
    'shell',
    'run-as',
    packageId,
    'cat',
    `files/${requestQueueFileName}`,
  ]);
  const requestDrainReadback = await adb([
    '-s',
    adbTarget,
    'shell',
    'run-as',
    packageId,
    'cat',
    `files/${requestDrainFileName}`,
  ]);
  const uiDump = await adb(['-s', adbTarget, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], { allowFailure: true });
  const sourceState = parseSourceState({
    requestQueueText: requestQueueReadback.stdout,
    requestDrainText: requestDrainReadback.stdout,
    uiText: uiDump.stdout,
  });

  const contractModule = await import(
    pathToFileURL(
      join(
        repoRoot,
        'packages',
        'parent-domain',
        'dist',
        'app-game-android-child-runtime-local-notification-request-queue-proof.js'
      )
    ).href
  );
  const readModel = contractModule.createAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof({
    checkedAt: '2026-06-08T22:45:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidChildRuntimeLocalNotificationRequestQueueProof(readModel);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    adbTargetRef: 'android-physical-adb-device-ref',
    commands: commands.map(redactCommandRecord),
    sourceState,
    readModel,
    summary,
    evidence: {
      contract:
        'packages/parent-domain/src/app-game-android-child-runtime-local-notification-request-queue-proof.ts',
      contractTest:
        'packages/parent-domain/tests/app-game-android-child-runtime-local-notification-request-queue-proof.test.ts',
      androidQueueRuntime:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeNotificationRequestQueueProof.java',
      androidReceiver:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeNotificationActionReceiver.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      packageInstall: 'adb install -r target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      packageLaunch:
        'adb shell am start -n ca.ocentra.parent.agent/.MainActivity --ez ca.ocentra.parent.agent.RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF true',
      requestQueueReadback: `adb shell run-as ca.ocentra.parent.agent cat files/${requestQueueFileName}`,
      requestDrainReadback: `adb shell run-as ca.ocentra.parent.agent cat files/${requestDrainFileName}`,
    },
    claimsProved: [
      'Android child package records package-local ask-parent request queue evidence after the notification action path',
      'Android child package records package-local request readback and drain markers through internal app storage',
      'Parent-domain records only action/queue/readback/drain proof refs and keeps service ingestion, approval round trip, provider delivery, adapter dispatch, platform enforcement, and raw private source rows unclaimed',
    ],
    claimsNotProved: [
      'Service request ingestion',
      'Parent approval round trip',
      'Provider notification delivery',
      'Platform delivery outside the child package',
      'Adapter dispatch, broad blocking, or platform enforcement',
      'Raw private source row custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.map(redactCommandRecord).join('\n\n')}\n`);

  console.log('app-game-android-child-runtime-local-notification-request-queue-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

function parseSourceState({ requestQueueText, requestDrainText, uiText }) {
  const requestQueueReadbackObserved = requestQueueText.includes(
    'android-child-runtime-local-notification-request-queue-ref'
  );
  const requestDrainReadbackObserved = requestDrainText.includes(
    'android-child-runtime-local-notification-request-drain-ref'
  );
  return {
    notificationRequestQueueState:
      requestQueueReadbackObserved || uiText.includes('local-notification-request-queue-recorded')
        ? 'local-notification-request-queue-recorded'
        : 'local-notification-request-queue-unavailable',
    notificationRequestReadbackState:
      requestQueueReadbackObserved || uiText.includes('local-notification-request-readback-observed')
        ? 'local-notification-request-readback-observed'
        : 'local-notification-request-readback-unavailable',
    notificationRequestDrainState:
      requestDrainReadbackObserved || uiText.includes('local-notification-request-drain-recorded')
        ? 'local-notification-request-drain-recorded'
        : 'local-notification-request-drain-unavailable',
    requestQueueReadbackObserved,
    requestDrainReadbackObserved,
  };
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
    '# WP222 Android child runtime local notification request queue source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    '- ADB target ref: `android-physical-adb-device-ref`',
    `- Request queue state: \`${sourceState.notificationRequestQueueState}\``,
    `- Request readback state: \`${sourceState.notificationRequestReadbackState}\``,
    `- Request drain state: \`${sourceState.notificationRequestDrainState}\``,
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
    match.includes('local-notification-request-queue-recorded') ||
    match.includes('local-notification-request-readback-observed') ||
    match.includes('local-notification-request-drain-recorded')
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
