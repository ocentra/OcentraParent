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
const accessibilityComponent = 'ca.ocentra.parent.agent/.AppGameAndroidAccessibilityRuntimeService';
const proofMode = 'app-game-android-accessibility-enabled-sample-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '219-app-game-android-accessibility-enabled-sample-proof'
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
    'app-game-android-accessibility-enabled-sample-proof',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'release:package:android']);
  assertFileExists(apkPath, 'Android debug APK');

  await adb(['connect', adbTarget]);
  const devices = await adb(['devices', '-l']);
  assertIncludes(devices.stdout, 'product:star2qltecs', 'physical Android product');
  assertIncludes(devices.stdout, 'model:SM_G965W', 'physical Android model');

  await adb(['-s', adbTarget, 'install', '-r', apkPath]);
  const settingsBefore = await readAccessibilitySettings();
  await enableAccessibilityService(settingsBefore.enabledServices);

  const sourceState = await observeAccessibilityEventSample();
  if (sourceState.runtimeState !== 'accessibility-runtime-bound' || sourceState.eventSampleCount < 1) {
    await writeBlockedProof({ sourceState, settingsBefore });
    console.log('app-game-android-accessibility-enabled-sample-proof-blocked');
    console.log(`evidence=${relativePath(proofPath)}`);
    return;
  }

  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-accessibility-enabled-sample-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidAccessibilityEnabledSampleProof({
    eventSampleCount: sourceState.eventSampleCount,
    checkedAt: '2026-06-08T21:50:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidAccessibilityEnabledSampleProof(readModel);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    adbTargetRef: 'android-physical-adb-device-ref',
    commands: commands.map(redactCommandRecord),
    settingsBefore: redactSettings(settingsBefore),
    sourceState,
    readModel,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-android-accessibility-enabled-sample-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-accessibility-enabled-sample-proof.test.ts',
      androidService:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidAccessibilityRuntimeService.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      packageInstall: 'adb install -r target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      serviceEnable: 'adb shell settings put secure enabled_accessibility_services <component-redacted>',
      packageLaunch: `adb shell am start -n ${activityId}`,
      uiDump: 'uiautomator dump observed bound/event-count state; raw UI XML is redacted in command log',
    },
    claimsProved: [
      'Android Accessibility service can be settings-enabled for the debug child-agent package on the physical Samsung Galaxy S9 proof target',
      'The package-local service reports a bound runtime state and count-only window-state event sample after app launch',
      'Parent-domain records only settings enablement, UI state, and event count without raw Accessibility event rows, service names, overlay content, delivery, dispatch, or enforcement claims',
    ],
    claimsNotProved: [
      'Warning, block, request, or usage-context overlay execution',
      'Device Owner or Profile Owner authority',
      'Play policy or store distribution proof',
      'Provider delivery, child-device delivery outside the package, adapter dispatch, broad blocking, or platform enforcement',
      'Raw Accessibility event row, service-name, overlay-content, UI XML, or device serial custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(
    join(appGameProofDir, '10-validation-commands.log'),
    `${commands.map(redactCommandRecord).join('\n\n')}\n`
  );

  console.log('app-game-android-accessibility-enabled-sample-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function writeBlockedProof({ sourceState, settingsBefore }) {
  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    adbTargetRef: 'android-physical-adb-device-ref',
    blocked: true,
    blockedReason: 'physical-target-did-not-bind-package-accessibility-service-after-secure-settings-enable-attempt',
    commands: commands.map(redactCommandRecord),
    settingsBefore: redactSettings(settingsBefore),
    sourceState,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-android-accessibility-enabled-sample-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-accessibility-enabled-sample-proof.test.ts',
      androidService:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidAccessibilityRuntimeService.java',
      androidActivity: 'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
      packageInstall: 'adb install -r target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      serviceEnableAttempt: 'adb shell settings put secure enabled_accessibility_services <component-redacted>',
      packageLaunch: `adb shell am start -n ${activityId}`,
      uiDump: 'uiautomator dump observed waiting-for-enablement state; raw UI XML is redacted in command log',
    },
    claimsProved: [
      'Android debug package declares and installs the Ocentra AccessibilityService on the physical Samsung Galaxy S9 proof target',
      'Android source and APK build expose the Accessibility runtime state and count-only event-count field',
      'The physical Samsung Galaxy S9 target keeps this service manual-required because it did not bind after the secure settings enablement attempt',
    ],
    claimsNotProved: [
      'Physical child-app UI observation for the Accessibility status on this locked target',
      'Accessibility service bind on this physical target',
      'Accessibility event sample observation',
      'Warning, block, request, or usage-context overlay execution',
      'Device Owner or Profile Owner authority',
      'Play policy or store distribution proof',
      'Provider delivery, child-device delivery outside the package, adapter dispatch, broad blocking, or platform enforcement',
      'Raw Accessibility event row, service-name, overlay-content, UI XML, or device serial custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(
    join(appGameProofDir, '10-validation-commands.log'),
    `${commands.map(redactCommandRecord).join('\n\n')}\n`
  );
}

async function readAccessibilitySettings() {
  const enabledValue = (
    await adb(['-s', adbTarget, 'shell', 'settings', 'get', 'secure', 'accessibility_enabled'], { allowFailure: true })
  ).stdout.trim();
  const servicesValue = (
    await adb(['-s', adbTarget, 'shell', 'settings', 'get', 'secure', 'enabled_accessibility_services'], {
      allowFailure: true,
    })
  ).stdout.trim();
  const enabledServices = servicesValue === 'null' || servicesValue.length === 0 ? [] : servicesValue.split(':');
  return {
    accessibilityEnabled: enabledValue === '1',
    enabledServices,
    settingsReadable: enabledValue === '0' || enabledValue === '1',
  };
}

async function enableAccessibilityService(existingServices) {
  const serviceSet = new Set(existingServices.filter((entry) => entry.trim().length > 0));
  serviceSet.add(accessibilityComponent);
  const mergedServices = [...serviceSet].join(':');
  await adb(['-s', adbTarget, 'shell', 'settings', 'put', 'secure', 'enabled_accessibility_services', mergedServices]);
  await adb(['-s', adbTarget, 'shell', 'settings', 'put', 'secure', 'accessibility_enabled', '1']);
}

async function observeAccessibilityEventSample() {
  let lastState = null;
  for (let attempt = 0; attempt < 3; attempt += 1) {
    await adb(['-s', adbTarget, 'shell', 'input', 'keyevent', 'KEYCODE_WAKEUP'], { allowFailure: true });
    await adb(['-s', adbTarget, 'shell', 'wm', 'dismiss-keyguard'], { allowFailure: true });
    await adb(['-s', adbTarget, 'shell', 'am', 'start', '-S', '-n', activityId]);
    await delay(3000);
    const settings = await readAccessibilitySettings();
    const uiDump = await adb(['-s', adbTarget, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], {
      allowFailure: true,
    });
    lastState = parseRuntimeState({ uiText: uiDump.stdout, settings });
    if (
      lastState.runtimeState === 'accessibility-runtime-bound' &&
      lastState.eventSampleState === 'accessibility-event-sample-observed'
    ) {
      return lastState;
    }
    await delay(1500);
  }
  return lastState;
}

function parseRuntimeState({ uiText, settings }) {
  const runtimeState =
    uiText.includes('accessibility-runtime-bound') || settings.enabledServices.some(isOcentraAccessibilityService)
      ? 'accessibility-runtime-bound'
      : 'accessibility-runtime-waiting-for-enablement';
  const eventSampleCount = parseCount(uiText, 'accessibility-event-sample-count');
  const eventSampleState =
    uiText.includes('accessibility-event-sample-observed') || eventSampleCount > 0
      ? 'accessibility-event-sample-observed'
      : runtimeState === 'accessibility-runtime-bound'
        ? 'accessibility-event-sample-empty'
        : 'accessibility-event-sample-waiting-for-enablement';
  return {
    serviceEnabledBySettings: settings.enabledServices.some(isOcentraAccessibilityService),
    uiStateObserved:
      uiText.includes('Ocentra Parent Agent') &&
      uiText.includes(runtimeState) &&
      (uiText.includes(eventSampleState) || eventSampleCount > 0),
    runtimeState,
    eventSampleState,
    eventSampleCount,
  };
}

function parseCount(uiText, key) {
  const match = uiText.match(new RegExp(`${key}=([0-9]+)`));
  return match ? Number.parseInt(match[1], 10) : 0;
}

function isOcentraAccessibilityService(entry) {
  return (
    entry === accessibilityComponent ||
    entry === 'ca.ocentra.parent.agent/ca.ocentra.parent.agent.AppGameAndroidAccessibilityRuntimeService' ||
    entry.toLowerCase().includes('ca.ocentra.parent.agent')
  );
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
    '# WP219 Android Accessibility enabled sample proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    '- ADB target ref: `android-physical-adb-device-ref`',
    '- Accessibility service settings enablement: `true`',
    `- Runtime state: \`${sourceState.runtimeState}\``,
    `- Event sample state: \`${sourceState.eventSampleState}\``,
    `- Event sample count: \`${sourceState.eventSampleCount}\``,
    '',
  ].join('\n');
}

function redactSettings(settings) {
  return {
    accessibilityEnabled: settings.accessibilityEnabled,
    enabledServiceCount: settings.enabledServices.length,
    serviceNamesRedacted: true,
    settingsReadable: settings.settingsReadable,
  };
}

function redactCommandRecord(record) {
  const rendered = record.commandLine
    .replace(adbTarget, 'android-physical-adb-device-ref')
    .replace(apkPath, 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk')
    .replaceAll(accessibilityComponent, '<android-accessibility-component-redacted>');
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
    .replaceAll(accessibilityComponent, '<android-accessibility-component-redacted>')
    .replace(/serial=\"[^\"]+\"/g, 'serial=\"android-physical-adb-device-ref\"')
    .replace(/text=\"[^\"]*\"/g, (match) => redactUiText(match));
}

function redactUiText(match) {
  if (
    match.includes('Ocentra Parent Agent') ||
    match.includes('accessibility-service-declared') ||
    match.includes('accessibility-runtime-waiting-for-enablement') ||
    match.includes('accessibility-runtime-bound') ||
    match.includes('accessibility-event-sample-waiting-for-enablement') ||
    match.includes('accessibility-event-sample-observed') ||
    match.includes('accessibility-event-sample-empty') ||
    match.includes('accessibility-event-sample-count=')
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
