import { spawnSync } from 'node:child_process';
import { mkdir, rm, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const proofMode = 'tracking-android-physical-device-runtime-proof';
const packageName = 'ca.ocentra.parent.agent';
const expectedActivity = 'ca.ocentra.parent.agent/.MainActivity';
const serviceName = 'OcentraParentAgentService';
const appLaunchText = 'Ocentra Parent Agent service scaffold is running.';
const defaultPhysicalSerial = '192.168.2.45:5555';
const defaultPhysicalObservationWindowSeconds = 30;
const androidCommandTimeoutMs = 240_000;
const buildCommandTimeoutMs = 300_000;
const ADB_COMMAND = process.platform === 'win32' ? 'adb.exe' : 'adb';
const NODE_COMMAND = 'node';
const npmCliPath = path.join(path.dirname(process.execPath), 'node_modules', 'npm', 'bin', 'npm-cli.js');
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
  await rm(resultDir, { recursive: true, force: true });
  await mkdir(resultDir, { recursive: true });
  await mkdir(output08, { recursive: true });
  await mkdir(output09, { recursive: true });
  await mkdir(output10, { recursive: true });
  await mkdir(output33, { recursive: true });

  runNpm(['run', 'release:package:android']);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-android-physical-device-runtime-proof.test.ts',
  ]);

  const tools = resolveAndroidTools();
  const serial = process.env.OCENTRA_PARENT_ANDROID_SERIAL ?? defaultPhysicalSerial;
  await writeCommandArtifact('01-adb-connect.txt', ADB_COMMAND, ['connect', serial]);
  const devices = runCapture(ADB_COMMAND, ['devices', '-l']);
  await writeText('01-adb-devices.txt', devices);
  if (!new RegExp(`${escapeRegExp(serial)}\\s+device`, 'u').test(devices)) {
    throw new Error(`Physical Android device ${serial} is not connected:\n${devices}`);
  }
  if (serial.startsWith('emulator-')) {
    throw new Error(`Physical proof requires a non-emulator serial, received ${serial}.`);
  }

  await installPhysicalApk(tools, serial);
  await resetPhysicalRuntimeProofState(tools, serial);
  await grantDeclaredLocationPermissions(tools, serial);
  runCapture(ADB_COMMAND, ['-s', serial, 'logcat', '-c']);
  await preparePhysicalDeviceForLaunch(tools, serial);
  await writeCommandArtifact('03-launch-activity.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'am',
    'start',
    '-n',
    expectedActivity,
  ]);
  await writeCommandArtifactAllowFailure('03-start-service.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'am',
    'start-foreground-service',
    '-n',
    'ca.ocentra.parent.agent/.OcentraParentAgentService',
  ]);
  const observationWindowSeconds = physicalObservationWindowSeconds();
  const physicalRouteObservation = await writePhysicalRouteObservation(tools, serial, observationWindowSeconds);
  await delay(observationWindowSeconds * 1_000);

  const device = collectDeviceMetadata(tools, serial);
  await writeJson('00-device.json', device);
  await writeText('12-package-dump.txt', device.packageDump);
  const permissionState = physicalPermissionState(device.packageDump);
  await writeJson('13-permission-state.json', permissionState);
  const serviceDump = await collectTextArtifact('04-service-dump.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'activity',
    'services',
    packageName,
  ]);
  const activityDump = await collectTextArtifact('05-activity-dump.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'activity',
    'activities',
  ]);
  const windowDump = await collectTextArtifact('06-window-dump.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'window',
  ]);
  const batteryDump = await collectTextArtifact('07-battery.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'battery',
  ]);
  const connectivityDump = await collectTextArtifact('08-connectivity.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'connectivity',
  ]);
  const uiDump = await collectUiDump(tools, serial);
  await collectScreenshot(tools, serial);
  const logcat = await collectTextArtifact('11-logcat.txt', ADB_COMMAND, ['-s', serial, 'logcat', '-d', '-t', '500']);
  const backgroundSamplePrefs = await collectRunAsTextArtifact(
    '14-background-location-sample-prefs.xml',
    tools,
    serial,
    ['cat', `shared_prefs/tracking_background_location_sample_proof.xml`]
  );
  const geofenceTransitionPrefs = await collectRunAsTextArtifact('15-geofence-transition-prefs.xml', tools, serial, [
    'cat',
    `shared_prefs/tracking_geofence_transition_proof.xml`,
  ]);
  const locationManagerState = await collectTextArtifact('17-location-manager-state.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'dumpsys',
    'location',
  ]);

  const proof = await buildProof({
    serial,
    device,
    permissionState,
    serviceDump,
    activityDump,
    windowDump,
    batteryDump,
    connectivityDump,
    uiDump,
    logcat,
    backgroundSamplePrefs,
    geofenceTransitionPrefs,
    locationManagerState,
    observationWindowSeconds,
    physicalRouteObservation,
  });
  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-android-physical-device-runtime-proof-ok');
  console.log(`evidence=${path.join('test-results', proofMode, 'proof.json')}`);
}

async function buildProof(runtime) {
  const proofModule = await tsImport(
    pathToFileURL(
      path.join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-android-physical-device-runtime-proof.ts')
    ).href,
    import.meta.url
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
      foregroundServiceObserved: foregroundServiceObserved(runtime.serviceDump),
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
      geofenceRegistrationObserved: parseXmlBoolean(runtime.geofenceTransitionPrefs, 'registered'),
      systemProximityRegistrationObserved: parseXmlBoolean(
        runtime.geofenceTransitionPrefs,
        'systemProximityRegistered'
      ),
      locationSampleObserved:
        parseXmlInt(runtime.backgroundSamplePrefs, 'backgroundLocationSampleCount') > 0 ||
        runtime.uiDump.includes('current-location-sample-observed'),
      backgroundLocationSampleCount: parseXmlInt(runtime.backgroundSamplePrefs, 'backgroundLocationSampleCount'),
      physicalRouteObservationWindowSeconds: runtime.observationWindowSeconds,
      shellLocationInjectionAvailable: runtime.physicalRouteObservation.shellLocationInjectionAvailable,
      localGeofenceTransitionCount: parseXmlInt(runtime.geofenceTransitionPrefs, 'transitionCount'),
      localGeofenceDwellCount: parseXmlInt(runtime.geofenceTransitionPrefs, 'dwellCount'),
      androidSystemGeofenceTransitionCount: parseXmlInt(
        runtime.geofenceTransitionPrefs,
        'systemProximityTransitionCount'
      ),
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
      foregroundServiceObserved: foregroundServiceObserved(runtime.serviceDump),
      uiLaunchTextObserved: runtime.uiDump.includes(appLaunchText),
      batteryDumpObserved: runtime.batteryDump.includes('level:'),
      connectivityDumpObserved: /Network|Active|Connectivity/u.test(runtime.connectivityDump),
      foregroundPermissionGranted: runtime.permissionState.foregroundPermissionGranted,
      backgroundPermissionGranted: runtime.permissionState.backgroundPermissionGranted,
      geofenceRegistrationObserved: parseXmlBoolean(runtime.geofenceTransitionPrefs, 'registered'),
      systemProximityRegistrationObserved: parseXmlBoolean(
        runtime.geofenceTransitionPrefs,
        'systemProximityRegistered'
      ),
      locationSampleObserved:
        parseXmlInt(runtime.backgroundSamplePrefs, 'backgroundLocationSampleCount') > 0 ||
        runtime.uiDump.includes('current-location-sample-observed'),
      backgroundLocationSampleCount: parseXmlInt(runtime.backgroundSamplePrefs, 'backgroundLocationSampleCount'),
      localGeofenceTransitionCount: parseXmlInt(runtime.geofenceTransitionPrefs, 'transitionCount'),
      localGeofenceDwellCount: parseXmlInt(runtime.geofenceTransitionPrefs, 'dwellCount'),
      androidSystemGeofenceTransitionCount: parseXmlInt(
        runtime.geofenceTransitionPrefs,
        'systemProximityTransitionCount'
      ),
      shellLocationInjectionAvailable: runtime.physicalRouteObservation.shellLocationInjectionAvailable,
      physicalRouteObservationWindowSeconds: runtime.observationWindowSeconds,
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
  if (row.backgroundPermissionGranted && !row.geofenceRegistrationObserved) {
    throw new Error(`Physical Android runtime proof did not observe geofence registration: ${JSON.stringify(row)}`);
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
  await writeFile(path.join(output08, '19-android-physical-device-runtime-proof.json'), stringifyJson(proof));
  await writeFile(path.join(output09, '19-android-physical-device-runtime-proof.json'), stringifyJson(proof));
  await writeFile(path.join(output10, '19-android-physical-device-runtime-proof.json'), stringifyJson(proof));
  await writeFile(path.join(output33, '69-android-physical-device-runtime-proof.json'), stringifyJson(proof));
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
    packageDump: runCapture(ADB_COMMAND, ['-s', serial, 'shell', 'dumpsys', 'package', packageName]),
  };
}

function getProp(tools, serial, name) {
  return runCapture(ADB_COMMAND, ['-s', serial, 'shell', 'getprop', name]).trim() || 'unknown';
}

async function collectUiDump(tools, serial) {
  runCapture(ADB_COMMAND, ['-s', serial, 'shell', 'uiautomator', 'dump', '/sdcard/ocentra-tracking-ui.xml']);
  const uiDump = runCapture(ADB_COMMAND, ['-s', serial, 'shell', 'cat', '/sdcard/ocentra-tracking-ui.xml']);
  await writeText('09-ui.xml', uiDump);
  return uiDump;
}

async function collectScreenshot(tools, serial) {
  const result = spawnSync(ADB_COMMAND, ['-s', serial, 'exec-out', 'screencap', '-p'], {
    cwd: repoRoot,
    encoding: null,
    shell: false,
    timeout: androidCommandTimeoutMs,
  });
  commands.push({ command: `${ADB_COMMAND} -s ${serial} exec-out screencap -p`, status: result.status ?? 1 });
  if ((result.status ?? 1) !== 0) {
    throw new Error(`screencap failed: ${String(result.stderr)}`);
  }
  await writeFile(path.join(resultDir, '10-screen.png'), result.stdout);
}

async function collectTextArtifact(name, command, args) {
  const output = redactTextArtifact(runCapture(command, args));
  await writeText(name, output);
  return output;
}

async function installPhysicalApk(tools, serial) {
  const remoteApkPath = '/data/local/tmp/ocentra-parent-agent-debug.apk';
  const pushOutput = runCapture(ADB_COMMAND, ['-s', serial, 'push', apkPath, remoteApkPath]);
  const installOutput = runCapture(ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'pm',
    'install',
    '-r',
    '-t',
    '-g',
    remoteApkPath,
  ]);
  await writeText('02-adb-install.txt', `${pushOutput}\n${installOutput}`);
}

async function collectRunAsTextArtifact(name, tools, serial, args) {
  const result = spawnSync(ADB_COMMAND, ['-s', serial, 'shell', 'run-as', packageName, ...args], {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    timeout: androidCommandTimeoutMs,
  });
  const output = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  commands.push({
    command: `${ADB_COMMAND} -s ${serial} shell run-as ${packageName} ${args.join(' ')}`,
    status: result.status ?? 1,
    output: output.trim(),
  });
  await writeText(name, output.length === 0 ? 'NO_OUTPUT\n' : output);
  return output;
}

async function writeCommandArtifact(name, command, args) {
  const output = runCapture(command, args);
  await writeText(name, output);
  return output;
}

async function writeCommandArtifactAllowFailure(name, command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    timeout: commandTimeoutMs(command),
  });
  const output = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  const errorText = result.error === undefined ? '' : `\n${String(result.error.message)}`;
  commands.push({
    command: `${command} ${args.join(' ')}`,
    status: result.status ?? 1,
    output: `${output}${errorText}`.trim(),
  });
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

function foregroundServiceObserved(serviceDump) {
  return (
    serviceDump.includes(`ServiceRecord`) &&
    serviceDump.includes(serviceName) &&
    (serviceDump.includes('isForeground=true') || serviceDump.includes('foregroundId='))
  );
}

function permissionGranted(packageDump, permissionName) {
  const escaped = escapeRegExp(permissionName);
  return new RegExp(`${escaped}: granted=true`, 'u').test(packageDump);
}

function parseXmlInt(xml, name) {
  const escaped = escapeRegExp(name);
  const intMatch = new RegExp(`<int\\s+name="${escaped}"\\s+value="(?<value>[0-9]+)"\\s*/>`, 'u').exec(xml);
  if (intMatch?.groups?.value !== undefined) return Number.parseInt(intMatch.groups.value, 10);
  const longMatch = new RegExp(`<long\\s+name="${escaped}"\\s+value="(?<value>[0-9]+)"\\s*/>`, 'u').exec(xml);
  return longMatch?.groups?.value === undefined ? 0 : Number.parseInt(longMatch.groups.value, 10);
}

function parseXmlBoolean(xml, name) {
  const escaped = escapeRegExp(name);
  const match = new RegExp(`<boolean\\s+name="${escaped}"\\s+value="(?<value>true|false)"\\s*/>`, 'u').exec(xml);
  return match?.groups?.value === 'true';
}

async function resetPhysicalRuntimeProofState(tools, serial) {
  await writeCommandArtifactAllowFailure('00-reset-runtime-proof-state.txt', ADB_COMMAND, [
    '-s',
    serial,
    'shell',
    'run-as',
    packageName,
    'sh',
    '-c',
    'rm -f shared_prefs/tracking_background_location_sample_proof.xml shared_prefs/tracking_geofence_transition_proof.xml',
  ]);
}

async function grantDeclaredLocationPermissions(tools, serial) {
  const lines = [];
  for (const permission of [
    'android.permission.ACCESS_COARSE_LOCATION',
    'android.permission.ACCESS_FINE_LOCATION',
    'android.permission.ACCESS_BACKGROUND_LOCATION',
  ]) {
    const result = spawnSync(ADB_COMMAND, ['-s', serial, 'shell', 'pm', 'grant', packageName, permission], {
      cwd: repoRoot,
      encoding: 'utf8',
      shell: false,
      timeout: androidCommandTimeoutMs,
    });
    const output = `${result.stdout ?? ''}${result.stderr ?? ''}`.trim();
    commands.push({
      command: `${ADB_COMMAND} -s ${serial} shell pm grant ${packageName} ${permission}`,
      status: result.status ?? 1,
      output,
    });
    lines.push(`${permission} exit=${String(result.status ?? 1)} ${output}`);
  }
  await writeText('13-grant-location-permissions.txt', `${lines.join('\n')}\n`);
}

async function preparePhysicalDeviceForLaunch(tools, serial) {
  const commandsToRun = [
    ['shell', 'input', 'keyevent', 'KEYCODE_WAKEUP'],
    ['shell', 'wm', 'dismiss-keyguard'],
    ['shell', 'input', 'keyevent', '82'],
  ];
  const lines = [];
  for (const args of commandsToRun) {
    const result = spawnSync(ADB_COMMAND, ['-s', serial, ...args], {
      cwd: repoRoot,
      encoding: 'utf8',
      shell: false,
      timeout: androidCommandTimeoutMs,
    });
    const output = `${result.stdout ?? ''}${result.stderr ?? ''}`.trim();
    const command = `${ADB_COMMAND} -s ${serial} ${args.join(' ')}`;
    commands.push({ command, status: result.status ?? 1, output });
    lines.push(`${command} exit=${String(result.status ?? 1)} ${output}`);
  }
  await writeText('03-prepare-device-for-launch.txt', `${lines.join('\n')}\n`);
}

function physicalPermissionState(packageDump) {
  return {
    foregroundPermissionGranted:
      permissionGranted(packageDump, 'android.permission.ACCESS_FINE_LOCATION') ||
      permissionGranted(packageDump, 'android.permission.ACCESS_COARSE_LOCATION'),
    backgroundPermissionGranted: permissionGranted(packageDump, 'android.permission.ACCESS_BACKGROUND_LOCATION'),
    finePermissionGranted: permissionGranted(packageDump, 'android.permission.ACCESS_FINE_LOCATION'),
    coarsePermissionGranted: permissionGranted(packageDump, 'android.permission.ACCESS_COARSE_LOCATION'),
  };
}

async function writePhysicalRouteObservation(tools, serial, observationWindowSeconds) {
  const locationCommand = spawnSync(ADB_COMMAND, ['-s', serial, 'shell', 'cmd', 'location', 'help'], {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    timeout: androidCommandTimeoutMs,
  });
  const output = `${locationCommand.stdout ?? ''}${locationCommand.stderr ?? ''}`.trim();
  commands.push({
    command: `${ADB_COMMAND} -s ${serial} shell cmd location help`,
    status: locationCommand.status ?? 1,
    output,
  });
  const notes = [
    `physicalDeviceSerial=${serial}`,
    `observationWindowSeconds=${String(observationWindowSeconds)}`,
    `shellLocationInjectionAvailable=${String(!output.includes('No shell command implementation.'))}`,
    'physicalRouteExpectation=keep the physical Android device outside/inside the configured proof geofence during the observation window, then rerun this script to capture nonzero location/geofence counters.',
    `shellLocationCommandOutput=${output.length === 0 ? 'NO_OUTPUT' : output}`,
  ];
  await writeText('16-physical-route-observation.txt', `${notes.join('\n')}\n`);
  return {
    shellLocationInjectionAvailable: !output.includes('No shell command implementation.'),
    output,
  };
}

function physicalObservationWindowSeconds() {
  const configured = process.env.OCENTRA_PARENT_ANDROID_PHYSICAL_OBSERVATION_SECONDS;
  if (configured === undefined || configured.length === 0) return defaultPhysicalObservationWindowSeconds;
  const parsed = Number.parseInt(configured, 10);
  return Number.isFinite(parsed) && parsed >= 0 ? parsed : defaultPhysicalObservationWindowSeconds;
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
  if (artifactRef.includes('permission')) return 'permission-state';
  if (artifactRef.includes('sample')) return 'physical-location-runtime';
  if (artifactRef.includes('geofence')) return 'physical-geofence-runtime';
  if (artifactRef.includes('route') || artifactRef.includes('location-manager')) return 'physical-route-observation';
  if (artifactRef.includes('logcat')) return 'validation-log';
  return 'adb-runtime-output';
}

function resolveAndroidTools() {
  const sdkRoot = process.env.ANDROID_SDK_ROOT ?? process.env.ANDROID_HOME;
  if (sdkRoot === undefined || sdkRoot.length === 0) {
    throw new Error('ANDROID_SDK_ROOT or ANDROID_HOME is required for Android physical-device proof.');
  }
  return {};
}

function run(command, args) {
  runCapture(command, args);
}

function runNpm(args) {
  run(NODE_COMMAND, [npmCliPath, ...args]);
}

function runCapture(command, args) {
  const result = spawnKnownCommand(command, args);
  const output = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  const errorText = result.error === undefined ? '' : `\n${String(result.error.message)}`;
  commands.push({
    command: `${command} ${args.join(' ')}`,
    status: result.status ?? 1,
    output: `${output}${errorText}`.trim(),
  });
  if ((result.status ?? 1) !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed\n${output}${errorText}`);
  }
  return output;
}

function spawnKnownCommand(command, args) {
  const options = {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
    timeout: commandTimeoutMs(command),
  };
  if (command === ADB_COMMAND) {
    return spawnSync(ADB_COMMAND, args, options);
  }
  if (command === NODE_COMMAND) {
    return spawnSync(NODE_COMMAND, args, options);
  }
  throw new Error(`Unsupported physical-device proof command: ${command}`);
}

function commandTimeoutMs(command) {
  return command.toLowerCase().includes('adb') ? androidCommandTimeoutMs : buildCommandTimeoutMs;
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
  await writeFile(path.join(resultDir, name), normalizeTextArtifact(value), 'utf8');
}

async function writeJson(name, value) {
  await writeFile(path.join(resultDir, name), stringifyJson(value), 'utf8');
}

function normalizeTextArtifact(value) {
  const normalized = redactTextArtifact(value).replaceAll('\r\n', '\n').replaceAll('\r', '\n');
  return `${normalized
    .split('\n')
    .map((line) => line.trimEnd())
    .join('\n')
    .replace(/\n*$/u, '')}\n`;
}

function stringifyJson(value) {
  return `${JSON.stringify(redactJsonValue(value), null, 2)}\n`;
}

function redactJsonValue(value) {
  if (typeof value === 'string') return redactTextArtifact(value);
  if (Array.isArray(value)) return value.map((item) => redactJsonValue(item));
  if (value !== null && typeof value === 'object') {
    return Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, redactJsonValue(entry)]));
  }
  return value;
}

function redactTextArtifact(value) {
  return value
    .replaceAll(/AIza[0-9A-Za-z_-]+/gu, '[REDACTED_GOOGLE_API_KEY]')
    .replaceAll(/AEdPqrE[0-9A-Za-z_-]+/gu, '[REDACTED_GOOGLE_BACKUP_KEY]')
    .replaceAll(/com\.facebook\.sdk\.ClientToken=[^,}\]\r\n]+/gu, 'com.facebook.sdk.ClientToken=[REDACTED]');
}
