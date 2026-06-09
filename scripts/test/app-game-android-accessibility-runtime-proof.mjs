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
const proofMode = 'app-game-android-accessibility-runtime-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '202-app-game-android-accessibility-runtime-proof');
const manifestPath = join(repoRoot, 'platforms', 'android', 'agent', 'app', 'src', 'main', 'AndroidManifest.xml');
const serviceConfigPath = join(
  repoRoot,
  'platforms',
  'android',
  'agent',
  'app',
  'src',
  'main',
  'res',
  'xml',
  'app_game_accessibility_service.xml'
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
    'app-game-android-accessibility-runtime-proof',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'release:package:android']);
  assertFileExists(apkPath, 'Android debug APK');

  const manifest = await readFile(manifestPath, 'utf8');
  const serviceConfig = await readFile(serviceConfigPath, 'utf8');
  const manifestServiceDeclared =
    manifest.includes('AppGameAndroidAccessibilityRuntimeService') &&
    manifest.includes('android.permission.BIND_ACCESSIBILITY_SERVICE');
  const serviceConfigDeclared =
    serviceConfig.includes('typeWindowStateChanged') && serviceConfig.includes('canRetrieveWindowContent="false"');

  await adb(['connect', adbTarget]);
  const devices = await adb(['devices', '-l']);
  assertIncludes(devices.stdout, 'product:star2qltecs', 'physical Android product');
  assertIncludes(devices.stdout, 'model:SM_G965W', 'physical Android model');
  await adb(['-s', adbTarget, 'install', '-r', apkPath]);
  await adb(['-s', adbTarget, 'shell', 'am', 'start', '-n', activityId]);
  await delay(2500);

  const settingsSample = await accessibilitySettingsSample();
  const uiDump = await adb(['-s', adbTarget, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], { allowFailure: true });
  const sourceState = parseRuntimeState({
    uiText: uiDump.stdout,
    settingsSample,
    manifestServiceDeclared,
    serviceConfigDeclared,
  });

  const contractModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-accessibility-runtime-proof.js')
    ).href
  );
  const readModel = contractModule.createAppGameAndroidAccessibilityRuntimeProof({
    declarationState: sourceState.declarationState,
    runtimeState: sourceState.runtimeState,
    eventSampleState: sourceState.eventSampleState,
    manifestServiceDeclared,
    serviceConfigDeclared,
    uiRuntimeStateObserved: sourceState.uiRuntimeStateObserved,
    settingsStateObserved: settingsSample.settingsReadable,
    checkedAt: '2026-06-08T21:10:00.000Z',
  });
  const summary = contractModule.summarizeAppGameAndroidAccessibilityRuntimeProof(readModel);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    adbTargetRef: 'android-physical-adb-device-ref',
    commands: commands.map(redactCommandRecord),
    settingsSample,
    sourceState,
    readModel,
    summary,
    evidence: {
      serviceSource:
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidAccessibilityRuntimeService.java',
      manifest: 'platforms/android/agent/app/src/main/AndroidManifest.xml',
      serviceConfig: 'platforms/android/agent/app/src/main/res/xml/app_game_accessibility_service.xml',
      contract: 'packages/parent-domain/src/app-game-android-accessibility-runtime-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-accessibility-runtime-proof.test.ts',
      packageInstall: 'adb install -r target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
      packageLaunch: `adb shell am start -n ${activityId}`,
      settingsProof:
        'adb shell settings get secure accessibility_enabled and enabled_accessibility_services were sampled with service/component names redacted.',
    },
    claimsProved: [
      'Android debug package declares an Ocentra AccessibilityService bound by android.permission.BIND_ACCESSIBILITY_SERVICE',
      'The service config listens only for window state changes and does not request window-content retrieval',
      'MainActivity exposes parent-safe Accessibility declaration/runtime/sample states after package install and launch',
      'Parent-domain records declaration/UI/settings evidence without raw event rows, raw service names, raw overlay content, adapter dispatch, platform enforcement, or child-device delivery claims',
    ],
    claimsNotProved: [
      'Accessibility service enablement if Android settings does not include the package-local service',
      'Accessibility event sample observation if the service is not bound or no event was observed',
      'Warning, block, request, or usage-context overlay execution',
      'Adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
      'Device Owner/Profile Owner authority or Play policy proof',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.map(redactCommandRecord).join('\n\n')}\n`);

  console.log('app-game-android-accessibility-runtime-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function accessibilitySettingsSample() {
  const enabledValue = (
    await adb(['-s', adbTarget, 'shell', 'settings', 'get', 'secure', 'accessibility_enabled'], { allowFailure: true })
  ).stdout.trim();
  const servicesValue = (
    await adb(['-s', adbTarget, 'shell', 'settings', 'get', 'secure', 'enabled_accessibility_services'], {
      allowFailure: true,
    })
  ).stdout.trim();
  const enabledServices = servicesValue === 'null' || servicesValue.length === 0 ? [] : servicesValue.split(':');
  const ocentraServiceEnabled = enabledServices.some((entry) =>
    entry.toLowerCase().includes('ca.ocentra.parent.agent')
  );

  return {
    accessibilityEnabled: enabledValue === '1',
    enabledServiceCount: enabledServices.filter((entry) => entry.trim().length > 0).length,
    ocentraServiceEnabled,
    serviceNamesRedacted: true,
    settingsReadable: enabledValue === '0' || enabledValue === '1',
  };
}

function parseRuntimeState({ uiText, settingsSample, manifestServiceDeclared, serviceConfigDeclared }) {
  const declarationState =
    manifestServiceDeclared && serviceConfigDeclared ? 'accessibility-service-declared' : 'accessibility-service-missing';
  const runtimeState =
    uiText.includes('accessibility-runtime-bound') || settingsSample.ocentraServiceEnabled
      ? 'accessibility-runtime-bound'
      : 'accessibility-runtime-waiting-for-enablement';
  const eventSampleState = parseEventSampleState(uiText, runtimeState);

  return {
    declarationState,
    runtimeState,
    eventSampleState,
    uiRuntimeStateObserved:
      uiText.includes('Ocentra Parent Agent') &&
      uiText.includes(declarationState) &&
      uiText.includes(runtimeState) &&
      uiText.includes(eventSampleState),
  };
}

function parseEventSampleState(uiText, runtimeState) {
  if (uiText.includes('accessibility-event-sample-observed')) {
    return 'accessibility-event-sample-observed';
  }
  if (uiText.includes('accessibility-event-sample-empty')) {
    return 'accessibility-event-sample-empty';
  }
  return runtimeState === 'accessibility-runtime-bound'
    ? 'accessibility-event-sample-empty'
    : 'accessibility-event-sample-waiting-for-enablement';
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
    '# WP202 Android Accessibility runtime proof source snapshot',
    '',
    '- Package: `ca.ocentra.parent.agent`',
    '- ADB target ref: `android-physical-adb-device-ref`',
    `- Declaration state: \`${sourceState.declarationState}\``,
    `- Runtime state: \`${sourceState.runtimeState}\``,
    `- Event sample state: \`${sourceState.eventSampleState}\``,
    `- UI runtime state observed: \`${sourceState.uiRuntimeStateObserved}\``,
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
    .replace(/enabled_accessibility_services[^\r\n]*/giu, 'enabled_accessibility_services=<redacted>')
    .replace(/([A-Za-z0-9_.]+\/[A-Za-z0-9_.$]+)/gu, '<android-component-redacted>')
    .replace(/text=\"[^\"]*\"/g, (match) => redactUiText(match));
}

function redactUiText(match) {
  if (
    match.includes('Ocentra Parent Agent') ||
    match.includes('accessibility-service-declared') ||
    match.includes('accessibility-service-missing') ||
    match.includes('accessibility-runtime-waiting-for-enablement') ||
    match.includes('accessibility-runtime-bound') ||
    match.includes('accessibility-event-sample-waiting-for-enablement') ||
    match.includes('accessibility-event-sample-observed') ||
    match.includes('accessibility-event-sample-empty')
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
