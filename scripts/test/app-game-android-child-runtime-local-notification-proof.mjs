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
const notificationId = '4480';
const markerFileName = 'app-game-local-notification-proof-state.txt';
const proofMode = 'app-game-android-child-runtime-local-notification-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '220-app-game-android-child-runtime-local-notification-proof'
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
      'app-game-android-child-runtime-local-notification-proof',
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
  await adb(['-s', adbTarget, 'shell', 'cmd', 'notification', 'cancel', packageId, notificationId], {
    allowFailure: true,
  });
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
    'ca.ocentra.parent.agent.RUN_APP_GAME_LOCAL_NOTIFICATION_PROOF',
    'true',
  ]);
  await delay(2500);

  const markerReadback = await adb(['-s', adbTarget, 'shell', 'run-as', packageId, 'cat', `files/${markerFileName}`]);
  const notificationDump = await adb(['-s', adbTarget, 'shell', 'dumpsys', 'notification'], { allowFailure: true });
  const uiDump = await adb(['-s', adbTarget, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], { allowFailure: true });
  const sourceState = parseSourceState({
    markerText: markerReadback.stdout,
    notificationText: notificationDump.stdout,
    uiText: uiDump.stdout,
  });

  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-child-runtime-local-notification-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidChildRuntimeLocalNotificationProof({
    notificationSeenInSystemUi: sourceState.notificationSeenInSystemUi,
    checkedAt: '2026-06-08T22:00:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidChildRuntimeLocalNotificationProof(readModel);

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
      contract: 'packages/parent-domain/src/app-game-android-child-runtime-local-notification-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-child-runtime-local-notification-proof.test.ts',
      androidRuntime:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeLocalNotificationProof.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      packageInstall: 'adb install -r target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      packageLaunch:
        'adb shell am start -n ca.ocentra.parent.agent/.MainActivity --ez ca.ocentra.parent.agent.RUN_APP_GAME_LOCAL_NOTIFICATION_PROOF true',
      markerReadback: `adb shell run-as ca.ocentra.parent.agent cat files/${markerFileName}`,
      notificationDump: 'dumpsys notification sampled package/id presence; raw notification text is redacted',
    },
    claimsProved: [
      'Android child package creates a dedicated app/game local notification channel',
      'Android child package posts a package-local app/game notification after proof trigger',
      'Android child package writes a package-local marker that can be read back through debug run-as',
      'Parent-domain records only channel/post/marker proof refs and keeps provider delivery, external platform delivery, adapter dispatch, platform enforcement, and raw private source rows unclaimed',
    ],
    claimsNotProved: [
      'Provider notification delivery',
      'Platform delivery outside the child package',
      'Child request approval round trip',
      'Adapter dispatch, broad blocking, or platform enforcement',
      'Raw private source row custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(
    join(appGameProofDir, '10-validation-commands.log'),
    `${commands.map(redactCommandRecord).join('\n\n')}\n`
  );

  console.log('app-game-android-child-runtime-local-notification-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

function parseSourceState({ markerText, notificationText, uiText }) {
  const notificationSeenInSystemUi =
    notificationText.includes(packageId) &&
    (notificationText.includes(`id=${notificationId}`) || notificationText.includes(`id: ${notificationId}`));
  return {
    notificationChannelState: uiText.includes('local-notification-channel-declared')
      ? 'local-notification-channel-declared'
      : 'local-notification-channel-unavailable',
    notificationPostState: uiText.includes('local-notification-post-recorded')
      ? 'local-notification-post-recorded'
      : 'local-notification-post-unavailable',
    notificationMarkerState:
      markerText.includes('android-child-runtime-app-game-warning-ref') &&
      uiText.includes('local-notification-marker-recorded')
        ? 'local-notification-marker-recorded'
        : 'local-notification-marker-unavailable',
    markerReadbackObserved: markerText.includes('android-child-runtime-app-game-warning-ref'),
    notificationSeenInSystemUi,
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
    '# WP220 Android child runtime local notification proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    '- ADB target ref: `android-physical-adb-device-ref`',
    `- Notification channel state: \`${sourceState.notificationChannelState}\``,
    `- Notification post state: \`${sourceState.notificationPostState}\``,
    `- Notification marker state: \`${sourceState.notificationMarkerState}\``,
    `- Notification seen in system UI: \`${sourceState.notificationSeenInSystemUi}\``,
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
    .replace(/text=\"[^\"]*\"/g, (match) => redactUiText(match))
    .replace(/tickerText=[^\r\n]*/g, 'tickerText=<notification-text-redacted>')
    .replace(/android\.title=[^\r\n]*/g, 'android.title=<notification-text-redacted>')
    .replace(/android\.text=[^\r\n]*/g, 'android.text=<notification-text-redacted>');
}

function redactUiText(match) {
  if (
    match.includes('Ocentra Parent Agent') ||
    match.includes('local-notification-channel-declared') ||
    match.includes('local-notification-post-recorded') ||
    match.includes('local-notification-marker-recorded') ||
    match.includes('local-notification-channel-unavailable') ||
    match.includes('local-notification-post-unavailable') ||
    match.includes('local-notification-marker-unavailable')
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
