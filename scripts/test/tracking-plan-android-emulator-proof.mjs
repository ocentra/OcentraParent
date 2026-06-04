import { spawn, spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const proofMode = 'tracking-plan-android-emulator-proof';
const packageName = 'ca.ocentra.parent.agent';
const expectedActivity = 'ca.ocentra.parent.agent/.MainActivity';
const serviceName = 'OcentraParentAgentService';
const appLaunchText = 'Ocentra Parent Agent service scaffold is running.';
const output08 = path.join(repoRoot, 'output', 'tracking-plan-proof', '08-android-foreground-location-adapter');
const output09 = path.join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '09-android-background-location-and-geofence-adapter'
);
const output10 = path.join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '10-android-battery-connectivity-and-status-adapter'
);
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const proofPath = path.join(resultDir, 'proof.json');
const apkPath = path.join(
  repoRoot,
  'target',
  'release-packages',
  'android',
  'ocentra-parent-agent-android-debug-latest.apk'
);
const commands = [];
let startedEmulator = false;
let selectedSerial = null;

try {
  await main();
} finally {
  if (startedEmulator && selectedSerial !== null) {
    await shutdownEmulator(resolveAndroidTools(), selectedSerial);
  }
}

async function main() {
  const tools = resolveAndroidTools();
  await mkdirProofRoots();
  await runNpm(['run', 'release:package:android']);
  assertFileExists(apkPath, 'Android debug APK');

  selectedSerial = await ensureDevice(tools);
  await adb(tools, selectedSerial, ['logcat', '-c']);
  await adb(tools, selectedSerial, ['install', '-r', apkPath], {
    artifact: path.join(resultDir, '01-adb-install.txt'),
  });

  const device = await readDeviceMetadata(tools, selectedSerial);
  const packageDump = await adbText(tools, selectedSerial, ['shell', 'dumpsys', 'package', packageName]);
  const resolvedActivity = await adbText(tools, selectedSerial, [
    'shell',
    'cmd',
    'package',
    'resolve-activity',
    '--brief',
    packageName,
  ]);
  await writeText(path.join(resultDir, '02-resolve-activity.txt'), resolvedActivity);
  assertIncludes(resolvedActivity, 'MainActivity', 'resolved launcher activity');

  await adb(tools, selectedSerial, ['shell', 'am', 'start', '-n', expectedActivity], {
    artifact: path.join(resultDir, '03-launch-activity.txt'),
  });
  await delay(3_000);

  const runtime = await collectRuntimeArtifacts(tools, selectedSerial);
  const permissionState = parsePermissionState(packageDump);
  const proof = buildProof({ device, packageDump, permissionState, resolvedActivity, runtime, tools });
  await writeProofFiles(proof);

  console.log('tracking-plan-android-emulator-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

function resolveAndroidTools() {
  const sdkRoot = process.env.ANDROID_SDK_ROOT ?? process.env.ANDROID_HOME;
  if (sdkRoot === undefined || sdkRoot.length === 0) {
    throw new Error('ANDROID_SDK_ROOT or ANDROID_HOME is required for Android emulator proof.');
  }
  const adbPath = path.join(sdkRoot, 'platform-tools', process.platform === 'win32' ? 'adb.exe' : 'adb');
  const emulatorPath = path.join(sdkRoot, 'emulator', process.platform === 'win32' ? 'emulator.exe' : 'emulator');
  assertFileExists(adbPath, 'adb');
  assertFileExists(emulatorPath, 'Android emulator');
  return { adbPath, emulatorPath, sdkRoot };
}

async function mkdirProofRoots() {
  await mkdir(output08, { recursive: true });
  await mkdir(output09, { recursive: true });
  await mkdir(output10, { recursive: true });
  await mkdir(resultDir, { recursive: true });
}

async function ensureDevice(tools) {
  const explicitSerial = process.env.OCENTRA_PARENT_ANDROID_SERIAL;
  if (explicitSerial !== undefined && explicitSerial.length > 0) {
    await waitForBoot(tools, explicitSerial);
    return explicitSerial;
  }

  const existing = await findReadyDevice(tools);
  if (existing !== null) {
    await waitForBoot(tools, existing);
    return existing;
  }

  const avd = process.env.OCENTRA_PARENT_ANDROID_AVD ?? firstAvd(tools);
  const emulator = spawn(tools.emulatorPath, emulatorArgs(avd), {
    cwd: repoRoot,
    detached: process.platform !== 'win32',
    stdio: ['ignore', 'ignore', 'ignore'],
    windowsHide: true,
  });
  startedEmulator = true;
  commands.push({ command: `${tools.emulatorPath} ${emulatorArgs(avd).join(' ')}`, exitCode: 0, artifact: null });
  emulator.unref();

  const serial = await waitForNewEmulatorDevice(tools);
  await waitForBoot(tools, serial);
  return serial;
}

function firstAvd(tools) {
  const result = spawnSync(tools.emulatorPath, ['-list-avds'], { cwd: repoRoot, encoding: 'utf8' });
  if ((result.status ?? 1) !== 0) {
    throw new Error(`emulator -list-avds failed: ${result.stderr}`);
  }
  const avd = result.stdout.split(/\r?\n/u).find((line) => line.trim().length > 0);
  if (avd === undefined) {
    throw new Error('No Android AVD is available for emulator proof.');
  }
  return avd.trim();
}

function emulatorArgs(avd) {
  return ['-avd', avd, '-no-window', '-no-snapshot-save', '-no-audio', '-no-boot-anim', '-gpu', 'swiftshader_indirect'];
}

async function findReadyDevice(tools) {
  const devices = await adbDevices(tools);
  return devices.find((device) => device.state === 'device')?.serial ?? null;
}

async function waitForNewEmulatorDevice(tools) {
  const deadline = Date.now() + 8 * 60_000;
  while (Date.now() < deadline) {
    const devices = await adbDevices(tools);
    const ready = devices.find((device) => device.state === 'device' && device.serial.startsWith('emulator-'));
    if (ready !== undefined) {
      return ready.serial;
    }
    await delay(2_000);
  }
  throw new Error('Timed out waiting for Android emulator device.');
}

async function waitForBoot(tools, serial) {
  const deadline = Date.now() + 8 * 60_000;
  while (Date.now() < deadline) {
    const boot = (await adbText(tools, serial, ['shell', 'getprop', 'sys.boot_completed'])).trim();
    if (boot === '1') {
      return;
    }
    await delay(2_000);
  }
  throw new Error(`Timed out waiting for Android boot completion on ${serial}.`);
}

async function adbDevices(tools) {
  const output = await runCapture(tools.adbPath, ['devices']);
  return output
    .split(/\r?\n/u)
    .map((line) => line.match(/^(?<serial>\S+)\s+(?<state>\S+)$/u)?.groups)
    .filter((entry) => entry !== undefined)
    .map((entry) => ({ serial: entry.serial, state: entry.state }));
}

async function readDeviceMetadata(tools, serial) {
  const props = {
    serial,
    androidRelease: await getProp(tools, serial, 'ro.build.version.release'),
    androidSdk: await getProp(tools, serial, 'ro.build.version.sdk'),
    buildFingerprint: await getProp(tools, serial, 'ro.build.fingerprint'),
    productModel: await getProp(tools, serial, 'ro.product.model'),
    productManufacturer: await getProp(tools, serial, 'ro.product.manufacturer'),
    abi: await getProp(tools, serial, 'ro.product.cpu.abi'),
  };
  await writeJson(path.join(output08, '01-device-metadata.json'), props);
  return props;
}

async function getProp(tools, serial, name) {
  return (await adbText(tools, serial, ['shell', 'getprop', name])).trim();
}

async function collectRuntimeArtifacts(tools, serial) {
  const serviceDump = await adbText(tools, serial, ['shell', 'dumpsys', 'activity', 'services', packageName]);
  const activityDump = await adbText(tools, serial, ['shell', 'dumpsys', 'activity', 'activities']);
  const windowDump = await adbText(tools, serial, ['shell', 'dumpsys', 'window']);
  const packageDump = await adbText(tools, serial, ['shell', 'dumpsys', 'package', packageName]);
  const batteryDump = await adbText(tools, serial, ['shell', 'dumpsys', 'battery']);
  const connectivityDump = await adbText(tools, serial, ['shell', 'dumpsys', 'connectivity']);
  const uiDump = await adbText(tools, serial, ['exec-out', 'uiautomator', 'dump', '/dev/tty']);
  const logcat = await adbText(tools, serial, ['logcat', '-d']);
  const screenshot = await adbBuffer(tools, serial, ['exec-out', 'screencap', '-p']);
  const pid = (await adbText(tools, serial, ['shell', 'pidof', '-s', packageName])).trim();

  await writeText(path.join(resultDir, '03-package-dump.txt'), packageDump);
  await writeText(path.join(resultDir, '04-service-dump.txt'), serviceDump);
  await writeText(path.join(resultDir, '05-activity-dump.txt'), activityDump);
  await writeText(path.join(resultDir, '06-window-dump.txt'), windowDump);
  await writeText(path.join(resultDir, '07-battery.txt'), batteryDump);
  await writeText(path.join(resultDir, '08-connectivity.txt'), connectivityDump);
  await writeText(path.join(resultDir, '09-ui.xml'), uiDump);
  await writeFile(path.join(resultDir, '10-screen.png'), screenshot);
  await writeText(path.join(resultDir, '11-logcat.txt'), logcat);

  return {
    pid,
    packageDump,
    service: parseServiceState(serviceDump),
    activity: parseActivityState(activityDump, windowDump),
    battery: parseKeyValueDump(batteryDump),
    connectivitySummary: summarizeConnectivity(connectivityDump),
    ui: parseUiState(uiDump),
    logcatFindings: parseLogcat(logcat),
    artifacts: runtimeArtifactPaths(),
  };
}

function parsePermissionState(packageDump) {
  const requested = [...packageDump.matchAll(/^\s+(android\.permission\.[A-Z_]+)\s*$/gmu)].map((match) => match[1]);
  const grants = {};
  for (const match of packageDump.matchAll(/^\s+(android\.permission\.[A-Z_]+): granted=(true|false)/gmu)) {
    grants[match[1]] = match[2] === 'true';
  }
  return {
    requested,
    grants,
    foregroundServiceGranted: grants['android.permission.FOREGROUND_SERVICE'] === true,
    notificationGranted: grants['android.permission.POST_NOTIFICATIONS'] === true,
    locationPermissionRequested: requested.some(
      (permission) =>
        permission === 'android.permission.ACCESS_FINE_LOCATION' ||
        permission === 'android.permission.ACCESS_COARSE_LOCATION'
    ),
    backgroundLocationPermissionRequested: requested.includes('android.permission.ACCESS_BACKGROUND_LOCATION'),
  };
}

function parseServiceState(serviceDump) {
  return {
    serviceRecordPresent: serviceDump.includes(serviceName),
    isForeground: serviceDump.includes('isForeground=true'),
    foregroundNotification: /foregroundNoti=([^\n]+)/u.exec(serviceDump)?.[1]?.trim() ?? null,
    startForegroundCount: /startForegroundCount=(\d+)/u.exec(serviceDump)?.[1] ?? null,
  };
}

function parseActivityState(activityDump, windowDump) {
  const activityFocused =
    windowDump.includes(`${packageName}/.MainActivity`) ||
    windowDump.includes(`${packageName}/${packageName}.MainActivity`) ||
    windowDump.includes(`${packageName}/MainActivity`);
  return {
    packageFocused: activityFocused,
    currentFocus: /mCurrentFocus=([^\n]+)/u.exec(windowDump)?.[1]?.trim() ?? null,
    resumedActivity: /ResumedActivity:\s*([^\n]+)/u.exec(activityDump)?.[1]?.trim() ?? null,
  };
}

function parseUiState(uiDump) {
  const text = [...uiDump.matchAll(/text="([^"]*)"/gu)].map((match) => decodeXmlText(match[1])).join('\n');
  return {
    hasLaunchText: text.includes(appLaunchText),
    text,
  };
}

function parseLogcat(logcat) {
  return {
    fatalExceptionCount: countMatches(logcat, /FATAL EXCEPTION/gu),
    androidRuntimeErrorCount: countMatches(logcat, /AndroidRuntime.*FATAL/gu),
    packageLogLines: countMatches(logcat, new RegExp(packageName.replaceAll('.', '\\.'), 'gu')),
    foregroundServiceAllowed: logcat.includes('Background started FGS: Allowed'),
  };
}

function parseKeyValueDump(dump) {
  const values = {};
  for (const line of dump.split(/\r?\n/u)) {
    const match = /^\s*(?<key>[A-Za-z0-9 _-]+):\s*(?<value>.*)$/u.exec(line);
    if (match?.groups !== undefined) {
      values[match.groups.key.trim().replaceAll(' ', '_')] = match.groups.value.trim();
    }
  }
  return values;
}

function summarizeConnectivity(connectivityDump) {
  return connectivityDump
    .split(/\r?\n/u)
    .filter((line) => /Active|NetworkAgentInfo|NetworkRequest|not connected|CONNECTED|DISCONNECTED/u.test(line))
    .slice(0, 25);
}

function runtimeArtifactPaths() {
  return {
    packageDump: relativePath(path.join(resultDir, '03-package-dump.txt')),
    serviceDump: relativePath(path.join(resultDir, '04-service-dump.txt')),
    activityDump: relativePath(path.join(resultDir, '05-activity-dump.txt')),
    windowDump: relativePath(path.join(resultDir, '06-window-dump.txt')),
    battery: relativePath(path.join(resultDir, '07-battery.txt')),
    connectivity: relativePath(path.join(resultDir, '08-connectivity.txt')),
    ui: relativePath(path.join(resultDir, '09-ui.xml')),
    screenshot: relativePath(path.join(resultDir, '10-screen.png')),
    logcat: relativePath(path.join(resultDir, '11-logcat.txt')),
  };
}

function buildProof({ device, packageDump, permissionState, resolvedActivity, runtime, tools }) {
  const checkedAt = new Date().toISOString();
  return {
    schemaVersion: 1,
    checkedAt,
    commit: gitHead(),
    proofMode,
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'emulator_scaffold_observed',
    productClaimReady: false,
    androidSdkRoot: tools.sdkRoot,
    package: {
      packageName,
      expectedActivity,
      resolvedActivity: resolvedActivity.trim(),
      apk: relativePath(apkPath),
      versionName: /versionName=([^\s]+)/u.exec(packageDump)?.[1] ?? null,
      versionCode: /versionCode=(\d+)/u.exec(packageDump)?.[1] ?? null,
    },
    device,
    permissionState,
    runtime,
    workpackProof: workpackProofState(permissionState, runtime),
    commands,
    nonClaims: [
      'This proof does not claim Android foreground location sample capture.',
      'This proof does not claim Android background location behavior.',
      'This proof does not claim Android geofence enter, exit, or dwell transitions.',
      'This proof does not claim notification delivery or alert provider behavior.',
      'This proof does not claim physical Android device behavior.',
      'This proof does not claim child-device enforcement, Device Owner, managed profile, or authority proof.',
    ],
  };
}

function workpackProofState(permissionState, runtime) {
  return {
    '08-android-foreground-location-adapter': {
      status: 'manual_required',
      proofArtifact:
        'output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json',
      reason: permissionState.locationPermissionRequested
        ? 'Package launched on emulator, but no runtime foreground location evidence was emitted.'
        : 'Package launched on emulator, but the current scaffold does not request foreground location permission.',
    },
    '09-android-background-location-and-geofence-adapter': {
      status: 'manual_required',
      proofArtifact:
        'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json',
      reason: permissionState.backgroundLocationPermissionRequested
        ? 'Background permission is declared, but no geofence transition was emitted.'
        : 'No background location/geofence permission or transition adapter is present in the current scaffold.',
    },
    '10-android-battery-connectivity-and-status-adapter': {
      status: 'emulator_scaffold_observed',
      proofArtifact:
        'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/04-device-status-proof.json',
      reason: runtime.service.isForeground
        ? 'Emulator package launch, foreground service state, battery dump, and connectivity dump were collected.'
        : 'Package launched, but foreground service state was not observed.',
    },
  };
}

async function writeProofFiles(proof) {
  await writeJson(proofPath, proof);
  for (const root of [output08, output09, output10]) {
    await writeText(path.join(root, '00-source-snapshot.md'), sourceSnapshotMarkdown(proof));
    await writeJson(path.join(root, '01-device-metadata.json'), proof.device);
  }
  await writeText(
    path.join(output08, '02-platform-permission-proof.md'),
    platformPermissionMarkdown(proof, 'foreground')
  );
  await writeJson(path.join(output08, '03-runtime-location-evidence.json'), foregroundLocationProof(proof));
  await writeText(
    path.join(output08, '15-manual-platform-proof.md'),
    manualProofMarkdown(proof, 'WP08 Android foreground')
  );
  await writeText(path.join(output08, '16-validation-commands.log'), commandLog());
  await writeText(
    path.join(output09, '02-platform-permission-proof.md'),
    platformPermissionMarkdown(proof, 'background')
  );
  await writeJson(path.join(output09, '05-geofence-transition-proof.json'), geofenceProof(proof));
  await writeText(
    path.join(output09, '15-manual-platform-proof.md'),
    manualProofMarkdown(proof, 'WP09 Android background/geofence')
  );
  await writeText(path.join(output09, '16-validation-commands.log'), commandLog());
  await writeJson(path.join(output10, '04-device-status-proof.json'), deviceStatusProof(proof));
  await writeText(
    path.join(output10, '15-manual-platform-proof.md'),
    manualProofMarkdown(proof, 'WP10 Android device status')
  );
  await writeText(path.join(output10, '16-validation-commands.log'), commandLog());
}

function sourceSnapshotMarkdown(proof) {
  return `# Android tracking emulator source snapshot

- Checked at: ${proof.checkedAt}
- Commit: ${proof.commit}
- Branch: ${gitBranch()}
- Proof command: \`npm run test:tracking-plan-android-emulator-proof\`
- Proof script: \`scripts/test/tracking-plan-android-emulator-proof.mjs\`
- APK: \`${proof.package.apk}\`
- Required proof tier: ${proof.requiredProofTier}
- Current proof tier: ${proof.currentProofTier}
- Product claim ready: ${String(proof.productClaimReady)}
`;
}

function foregroundLocationProof(proof) {
  return {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit: proof.commit,
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'manual_required',
    packageLaunchObserved: proof.runtime.activity.packageFocused,
    foregroundServiceObserved: proof.runtime.service.isForeground,
    locationEvidenceCaptured: false,
    foregroundLocationPermissionRequested: proof.permissionState.locationPermissionRequested,
    missingProofReason: proof.workpackProof['08-android-foreground-location-adapter'].reason,
    device: proof.device,
    artifacts: proof.runtime.artifacts,
  };
}

function geofenceProof(proof) {
  return {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit: proof.commit,
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'manual_required',
    packageLaunchObserved: proof.runtime.activity.packageFocused,
    foregroundServiceObserved: proof.runtime.service.isForeground,
    backgroundLocationPermissionRequested: proof.permissionState.backgroundLocationPermissionRequested,
    geofenceTransitionCount: 0,
    missingProofReason: proof.workpackProof['09-android-background-location-and-geofence-adapter'].reason,
    device: proof.device,
    artifacts: proof.runtime.artifacts,
  };
}

function deviceStatusProof(proof) {
  return {
    schemaVersion: 1,
    checkedAt: proof.checkedAt,
    commit: proof.commit,
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: proof.workpackProof['10-android-battery-connectivity-and-status-adapter'].status,
    packageLaunchObserved: proof.runtime.activity.packageFocused,
    foregroundServiceObserved: proof.runtime.service.isForeground,
    foregroundNotification: proof.runtime.service.foregroundNotification,
    battery: proof.runtime.battery,
    connectivitySummary: proof.runtime.connectivitySummary,
    ui: proof.runtime.ui,
    logcatFindings: proof.runtime.logcatFindings,
    device: proof.device,
    artifacts: proof.runtime.artifacts,
    nonClaims: ['No product tracking freshness claim is made from this emulator scaffold status proof.'],
  };
}

function platformPermissionMarkdown(proof, mode) {
  return `# Android ${mode} permission proof

- Checked at: ${proof.checkedAt}
- Commit: ${proof.commit}
- Device: ${proof.device.productModel} / Android ${proof.device.androidRelease} API ${proof.device.androidSdk}
- Package: ${proof.package.packageName}
- Resolved activity: ${proof.package.resolvedActivity}

## Requested permissions

${proof.permissionState.requested.map((permission) => `- ${permission}: granted=${String(proof.permissionState.grants[permission] ?? false)}`).join('\n')}
${proof.permissionState.requested.length === 0 ? '_No requested Android permissions were found in the package dump._' : ''}

## Tracking claim boundary

- Foreground location permission requested: ${String(proof.permissionState.locationPermissionRequested)}
- Background location permission requested: ${String(proof.permissionState.backgroundLocationPermissionRequested)}
- Foreground service observed: ${String(proof.runtime.service.isForeground)}
- Product location/geofence claim ready: false
`;
}

function manualProofMarkdown(proof, label) {
  return `# ${label} manual platform proof

This proof was generated by \`npm run test:tracking-plan-android-emulator-proof\` on an Android emulator.

## Proved

- Debug APK built and installed.
- Launcher activity resolved and launched.
- Package process observed with pid ${proof.runtime.pid}.
- Foreground service observed: ${String(proof.runtime.service.isForeground)}.
- Battery and connectivity dumps collected.
- UI tree collected and contains scaffold/manual-consent text: ${String(proof.runtime.ui.hasLaunchText)}.

## Not claimed

${proof.nonClaims.map((claim) => `- ${claim}`).join('\n')}
`;
}

function commandLog() {
  return `${commands.map((entry) => `${entry.exitCode === 0 ? 'PASS' : 'FAIL'} ${entry.command}`).join('\n')}\n`;
}

async function adb(tools, serial, args, options = {}) {
  const result = await runCommand(tools.adbPath, ['-s', serial, ...args], { capture: true });
  if (options.artifact !== undefined) {
    await writeText(options.artifact, result.output);
  }
  return result.output;
}

async function adbText(tools, serial, args) {
  return adb(tools, serial, args);
}

async function adbBuffer(tools, serial, args) {
  const result = spawnSync(tools.adbPath, ['-s', serial, ...args], {
    cwd: repoRoot,
    encoding: null,
    maxBuffer: 20 * 1024 * 1024,
  });
  const exitCode = result.status ?? 1;
  commands.push({ command: `${tools.adbPath} -s ${serial} ${args.join(' ')}`, exitCode, artifact: null });
  if (exitCode !== 0) {
    throw new Error(`adb ${args.join(' ')} failed: ${String(result.stderr)}`);
  }
  return result.stdout;
}

async function runNpm(args) {
  await runCommand(
    process.platform === 'win32' ? 'cmd' : 'npm',
    process.platform === 'win32' ? ['/c', 'npm', ...args] : args
  );
}

async function runCapture(command, args) {
  return (await runCommand(command, args, { capture: true })).output;
}

async function runCommand(command, args, options = {}) {
  const output = [];
  const child = spawn(command, args, {
    cwd: repoRoot,
    stdio: ['ignore', options.capture ? 'pipe' : 'inherit', options.capture ? 'pipe' : 'inherit'],
    windowsHide: true,
  });
  if (options.capture === true) {
    child.stdout.on('data', (chunk) => output.push(String(chunk)));
    child.stderr.on('data', (chunk) => output.push(String(chunk)));
  }
  const exitCode = await waitForExit(child);
  const commandLine = [command, ...args].join(' ');
  commands.push({ command: commandLine, exitCode, artifact: null });
  if (exitCode !== 0) {
    throw new Error(`${commandLine} exited with ${exitCode}`);
  }
  return { exitCode, output: output.join('') };
}

function waitForExit(child) {
  return new Promise((resolve, reject) => {
    child.once('exit', (code, signal) => resolve(signal === null ? (code ?? 1) : 1));
    child.once('error', reject);
  });
}

async function shutdownEmulator(tools, serial) {
  try {
    await adb(tools, serial, ['emu', 'kill']);
  } catch {
    // The emulator may already be gone after a successful run.
  }
}

function gitHead() {
  const result = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' });
  if ((result.status ?? 1) !== 0) {
    throw new Error('git rev-parse HEAD failed');
  }
  return result.stdout.trim();
}

function gitBranch() {
  const result = spawnSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' });
  if ((result.status ?? 1) !== 0) {
    throw new Error('git branch --show-current failed');
  }
  return result.stdout.trim();
}

async function writeJson(filePath, value) {
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

async function writeText(filePath, value) {
  await writeFile(filePath, value.endsWith('\n') ? value : `${value}\n`, 'utf8');
}

function assertFileExists(filePath, label) {
  if (!existsSync(filePath)) {
    throw new Error(`${label} not found: ${filePath}`);
  }
}

function assertIncludes(value, expected, label) {
  if (!value.includes(expected)) {
    throw new Error(`${label} did not include ${expected}`);
  }
}

function countMatches(value, pattern) {
  return [...value.matchAll(pattern)].length;
}

function decodeXmlText(value) {
  return value.replaceAll('&#10;', '\n').replaceAll('&quot;', '"').replaceAll('&amp;', '&');
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll('\\', '/');
}
