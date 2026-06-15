import { spawnSync } from 'node:child_process';
import { mkdir, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const proofMode = 'tracking-android-physical-device-runtime-proof';
const packageName = 'ca.ocentra.parent.agent';
const expectedActivity = 'ca.ocentra.parent.agent/.MainActivity';
const serviceName = 'OcentraParentAgentService';
const appLaunchText = 'Ocentra Parent Agent service scaffold is running.';
const defaultPhysicalSerial = '192.168.2.45:5555';
const apkPath = path.join(
  repoRoot,
  'target',
  'release-packages',
  'android',
  'ocentra-parent-agent-android-debug-latest.apk'
);
const resultDir = path.join(repoRoot, 'test-results', proofMode);
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
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
let requiredArtifactRefs = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(output08, { recursive: true });
  await mkdir(output09, { recursive: true });
  await mkdir(output10, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'release:package:android']);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-android-physical-device-runtime-proof',
  ]);

  const tools = resolveAndroidTools();
  const serial = process.env.OCENTRA_PARENT_ANDROID_SERIAL ?? defaultPhysicalSerial;
  await writeCommandArtifact('01-adb-connect.txt', tools.adbPath, ['connect', serial]);
  const devices = runCapture(tools.adbPath, ['devices', '-l']);
  await writeText('01-adb-devices.txt', devices);
  if (!new RegExp(`${escapeRegExp(serial)}\\s+device`, 'u').test(devices)) {
    throw new Error(`Physical Android device ${serial} is not connected:\n${devices}`);
  }
  if (serial.startsWith('emulator-')) {
    throw new Error(`Physical proof requires a non-emulator serial, received ${serial}.`);
  }

  await writeCommandArtifact('02-adb-install.txt', tools.adbPath, ['-s', serial, 'install', '-r', apkPath]);
  runCapture(tools.adbPath, ['-s', serial, 'logcat', '-c']);
  await writeCommandArtifact('03-launch-activity.txt', tools.adbPath, [
    '-s',
    serial,
    'shell',
    'am',
    'start',
    '-n',
    expectedActivity,
  ]);
  await writeCommandArtifactAllowFailure('03-start-service.txt', tools.adbPath, [
    '-s',
    serial,
    'shell',
    'am',
    'start-foreground-service',
    '-n',
    'ca.ocentra.parent.agent/.OcentraParentAgentService',
  ]);
  await delay(4_000);

  const device = collectDeviceMetadata(tools, serial);
  await writeJson('00-device.json', device);
  const serviceDump = await collectTextArtifact('04-service-dump.txt', tools.adbPath, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'activity',
    'services',
    packageName,
  ]);
  const activityDump = await collectTextArtifact('05-activity-dump.txt', tools.adbPath, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'activity',
    'activities',
  ]);
  const windowDump = await collectTextArtifact('06-window-dump.txt', tools.adbPath, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'window',
  ]);
  const batteryDump = await collectTextArtifact('07-battery.txt', tools.adbPath, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'battery',
  ]);
  const connectivityDump = await collectTextArtifact('08-connectivity.txt', tools.adbPath, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'connectivity',
  ]);
  const uiDump = await collectUiDump(tools, serial);
  await collectScreenshot(tools, serial);
  const logcat = await collectTextArtifact('11-logcat.txt', tools.adbPath, ['-s', serial, 'logcat', '-d', '-t', '500']);

  const proof = await buildProof({
    serial,
    device,
    serviceDump,
    activityDump,
    windowDump,
    batteryDump,
    connectivityDump,
    uiDump,
    logcat,
  });
  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-android-physical-device-runtime-proof-ok');
  console.log(`evidence=${path.join('test-results', proofMode, 'proof.json')}`);
}

async function buildProof(runtime) {
  const proofModule = await import(
    pathToFileURL(
      path.join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-android-physical-device-runtime-proof.js')
    ).href
  );
  requiredArtifactRefs = proofModule.RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs;
  return {
    ...proofModule.buildTrackingAndroidPhysicalDeviceRuntimeProof(new Date().toISOString(), {
      physicalDeviceProofRef: 'test-results/tracking-android-physical-device-runtime-proof/proof.json',
      packageName,
      activityName: expectedActivity,
      deviceSerial: runtime.serial,
      androidRelease: runtime.device.androidRelease,
      androidSdk: runtime.device.androidSdk,
      productModel: runtime.device.productModel,
      productName: runtime.device.productName,
      abi: runtime.device.abi,
      packageInstallObserved: true,
      packageLaunchObserved: packageLaunchObserved(
        runtime.activityDump,
        runtime.windowDump,
        runtime.uiDump,
        runtime.serviceDump
      ),
      foregroundServiceObserved: runtime.serviceDump.includes(serviceName),
      uiLaunchTextObserved: runtime.uiDump.includes(appLaunchText),
      batteryDumpObserved: runtime.batteryDump.includes('level:'),
      connectivityDumpObserved: /Network|Active|Connectivity/u.test(runtime.connectivityDump),
      foregroundPermissionGranted: permissionGranted(
        runtime.device.packageDump,
        'android.permission.ACCESS_FINE_LOCATION'
      ),
      backgroundPermissionGranted: permissionGranted(
        runtime.device.packageDump,
        'android.permission.ACCESS_BACKGROUND_LOCATION'
      ),
      locationSampleObserved: runtime.uiDump.includes('current-location-sample-observed'),
      localGeofenceTransitionCount: parseUiNumber(runtime.uiDump, 'backgroundGeofenceTransitionCount'),
      localGeofenceDwellCount: parseUiNumber(runtime.uiDump, 'backgroundGeofenceDwellCount'),
      androidSystemGeofenceTransitionCount: parseUiNumber(runtime.uiDump, 'backgroundSystemGeofenceTransitionCount'),
      artifactRows: await artifactRows(),
    }),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    commands,
    device: runtime.device,
    runtimeObservations: {
      packageLaunchObserved: packageLaunchObserved(
        runtime.activityDump,
        runtime.windowDump,
        runtime.uiDump,
        runtime.serviceDump
      ),
      foregroundServiceObserved: runtime.serviceDump.includes(serviceName),
      uiLaunchTextObserved: runtime.uiDump.includes(appLaunchText),
      batteryDumpObserved: runtime.batteryDump.includes('level:'),
      connectivityDumpObserved: /Network|Active|Connectivity/u.test(runtime.connectivityDump),
      locationSampleObserved: runtime.uiDump.includes('current-location-sample-observed'),
      localGeofenceTransitionCount: parseUiNumber(runtime.uiDump, 'backgroundGeofenceTransitionCount'),
      localGeofenceDwellCount: parseUiNumber(runtime.uiDump, 'backgroundGeofenceDwellCount'),
      androidSystemGeofenceTransitionCount: parseUiNumber(runtime.uiDump, 'backgroundSystemGeofenceTransitionCount'),
    },
    artifactPaths: {
      wp08: 'output/tracking-plan-proof/08-android-foreground-location-adapter/19-android-physical-device-runtime-proof.json',
      wp09: 'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/19-android-physical-device-runtime-proof.json',
      wp10: 'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/19-android-physical-device-runtime-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/69-android-physical-device-runtime-proof.json',
      evidence: 'test-results/tracking-android-physical-device-runtime-proof/proof.json',
    },
  };
}

function assertProof(proof) {
  const [row] = proof.rows;
  if (!row.physicalDeviceRuntimeObserved || !row.packageLaunchObserved || !row.foregroundServiceObserved) {
    throw new Error(`Physical Android runtime proof did not observe package launch/service: ${JSON.stringify(row)}`);
  }
  if (
    proof.productClaims.physicalLocationRuntimeClaimed ||
    proof.productClaims.physicalGeofenceRuntimeClaimed ||
    proof.productClaims.productClaimReady
  ) {
    throw new Error(`Physical Android proof overclaimed tracking readiness: ${JSON.stringify(proof.productClaims)}`);
  }
}

async function writeProofArtifacts(proof) {
  await writeJson('proof.json', proof);
  await writeJson('tracking-android-physical-device-runtime-read-model.json', proof.rows);
  await writeFile(
    path.join(output08, '19-android-physical-device-runtime-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(
    path.join(output09, '19-android-physical-device-runtime-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(
    path.join(output10, '19-android-physical-device-runtime-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(
    path.join(output33, '69-android-physical-device-runtime-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(path.join(output10, '19-android-physical-device-runtime-validation.log'), validationLog(), 'utf8');
}

function collectDeviceMetadata(tools, serial) {
  return {
    serial,
    androidRelease: getProp(tools, serial, 'ro.build.version.release'),
    androidSdk: getProp(tools, serial, 'ro.build.version.sdk'),
    productModel: getProp(tools, serial, 'ro.product.model'),
    productName: getProp(tools, serial, 'ro.product.name'),
    deviceName: getProp(tools, serial, 'ro.product.device'),
    abi: getProp(tools, serial, 'ro.product.cpu.abi'),
    packageDump: runCapture(tools.adbPath, ['-s', serial, 'shell', 'dumpsys', 'package', packageName]),
  };
}

function getProp(tools, serial, name) {
  return runCapture(tools.adbPath, ['-s', serial, 'shell', 'getprop', name]).trim() || 'unknown';
}

async function collectUiDump(tools, serial) {
  runCapture(tools.adbPath, ['-s', serial, 'shell', 'uiautomator', 'dump', '/sdcard/ocentra-tracking-ui.xml']);
  const uiDump = runCapture(tools.adbPath, ['-s', serial, 'shell', 'cat', '/sdcard/ocentra-tracking-ui.xml']);
  await writeText('09-ui.xml', uiDump);
  return uiDump;
}

async function collectScreenshot(tools, serial) {
  const result = spawnSync(tools.adbPath, ['-s', serial, 'exec-out', 'screencap', '-p'], {
    cwd: repoRoot,
    encoding: null,
    shell: false,
  });
  commands.push({ command: `${tools.adbPath} -s ${serial} exec-out screencap -p`, status: result.status ?? 1 });
  if ((result.status ?? 1) !== 0) {
    throw new Error(`screencap failed: ${String(result.stderr)}`);
  }
  await writeFile(path.join(resultDir, '10-screen.png'), result.stdout);
}

async function collectTextArtifact(name, command, args) {
  const output = runCapture(command, args);
  await writeText(name, output);
  return output;
}

async function writeCommandArtifact(name, command, args) {
  const output = runCapture(command, args);
  await writeText(name, output);
  return output;
}

async function writeCommandArtifactAllowFailure(name, command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  const output = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  commands.push({ command: `${command} ${args.join(' ')}`, status: result.status ?? 1, output: output.trim() });
  await writeText(name, output);
  return output;
}

function packageLaunchObserved(activityDump, windowDump, uiDump, serviceDump = '') {
  return (
    activityDump.includes(packageName) ||
    windowDump.includes(packageName) ||
    uiDump.includes(appLaunchText) ||
    serviceDump.includes(packageName)
  );
}

function permissionGranted(packageDump, permissionName) {
  const escaped = escapeRegExp(permissionName);
  return new RegExp(`${escaped}: granted=true`, 'u').test(packageDump);
}

function parseUiNumber(uiDump, label) {
  const match = new RegExp(`${escapeRegExp(label)}[^0-9]*(?<value>[0-9]+)`, 'u').exec(uiDump);
  return match?.groups?.value === undefined ? 0 : Number.parseInt(match.groups.value, 10);
}

async function artifactRows() {
  const rows = [];
  for (const artifactRef of requiredArtifactRefs) {
    const artifactPath = path.join(repoRoot, artifactRef);
    const stats = await stat(artifactPath).catch(() => null);
    rows.push({
      artifactRef,
      category: categoryFor(artifactRef),
      required: true,
      present: stats !== null && stats.size > 0,
      byteSize: stats?.size ?? 0,
    });
  }
  return rows;
}

function categoryFor(artifactRef) {
  if (artifactRef.includes('install') || artifactRef.includes('launch') || artifactRef.includes('activity')) {
    return 'package-runtime';
  }
  if (artifactRef.includes('service')) return 'foreground-service';
  if (artifactRef.includes('battery') || artifactRef.includes('connectivity')) return 'device-status';
  if (artifactRef.includes('ui') || artifactRef.includes('screen')) return 'ui-screenshot';
  if (artifactRef.includes('logcat')) return 'validation-log';
  return 'adb-runtime-output';
}

function resolveAndroidTools() {
  const sdkRoot = process.env.ANDROID_SDK_ROOT ?? process.env.ANDROID_HOME;
  if (sdkRoot === undefined || sdkRoot.length === 0) {
    throw new Error('ANDROID_SDK_ROOT or ANDROID_HOME is required for Android physical-device proof.');
  }
  const adbPath = path.join(sdkRoot, 'platform-tools', process.platform === 'win32' ? 'adb.exe' : 'adb');
  return { adbPath };
}

function run(command, args) {
  runCapture(command, args);
}

function runCapture(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  const output = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  commands.push({ command: `${command} ${args.join(' ')}`, status: result.status ?? 1, output: output.trim() });
  if ((result.status ?? 1) !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed\n${output}`);
  }
  return output;
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if ((result.status ?? 1) !== 0) return '';
  return result.stdout.trim();
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
}

function delay(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

async function writeText(name, value) {
  await writeFile(path.join(resultDir, name), value, 'utf8');
}

async function writeJson(name, value) {
  await writeFile(path.join(resultDir, name), `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
