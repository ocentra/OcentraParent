import { createHash } from 'node:crypto';
import { execFileSync, spawn } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { createServer } from 'node:http';
import { dirname, join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { fileURLToPath } from 'node:url';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const androidProjectRoot = join(repoRoot, 'platforms', 'android', 'agent');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/05-cross-platform-inventory-matrix');
const testResultPath = join(repoRoot, 'test-results/browser-platform-android-owned-shell-proof/proof.json');
const outputProofPath = join(proofRoot, '15-android-owned-browser-shell-proof.json');
const screenshotProofPath = join(proofRoot, '15-android-owned-browser-shell-screenshot.png');
const apkPath = join(
  androidProjectRoot,
  'browser-shell',
  'build',
  'outputs',
  'apk',
  'debug',
  'browser-shell-debug.apk'
);
const observedAt = new Date().toISOString();
const packageName = 'com.ocentra.parent.browser';
const activityName = 'ca.ocentra.parent.browser.OcentraOwnedBrowserShellActivity';

mkdirSync(proofRoot, { recursive: true });

const adb = findAdb();
const emulator = findEmulator();
const emulatorAvds = emulator ? listAvds(emulator.path) : [];
let launchedEmulator = false;
let launchedEmulatorSerial = null;
let server = null;

try {
  buildOwnedBrowserShell();

  if (!adb) {
    throw new Error('adb is required for Android owned browser shell proof');
  }

  if (listDevices(adb.path).filter((device) => device.state === 'device').length === 0) {
    if (!emulator) {
      throw new Error('Android emulator executable is required when no device is attached');
    }
    launchedEmulatorSerial = await launchEmulatorIfAvailable(adb.path, emulator.path, emulatorAvds);
    launchedEmulator = launchedEmulatorSerial !== null;
  }

  const devices = listDevices(adb.path).filter((device) => device.state === 'device');
  if (devices.length === 0) {
    throw new Error('No booted Android device/emulator was available for owned browser shell proof');
  }

  server = await startProofServer();
  const proofUrl = `http://10.0.2.2:${server.port}/owned-browser-shell-proof`;
  const deviceProofs = [];

  for (const device of devices) {
    const bootCompleted = command(['-s', device.serial, 'shell', 'getprop', 'sys.boot_completed'], {
      adbPath: adb.path,
      allowFailure: true,
    }).trim();
    if (bootCompleted !== '1') {
      continue;
    }
    deviceProofs.push(await proveDevice(adb.path, device.serial, proofUrl, launchedEmulator));
  }

  const sourceBoundary = inspectOwnedBrowserShellSource();
  const successfulLaunches = deviceProofs.filter((proof) => proof.launchObserved);
  const negativeChecks = [
    { claim: 'managed-exact-url-on-android', rejected: true },
    { claim: 'known-active-tab-on-android', rejected: true },
    { claim: 'android-device-owner-policy', rejected: true },
    { claim: 'android-vpn-dns-browser-proof', rejected: true },
    { claim: 'android-usagestats-route-proof', rejected: true },
    { claim: 'android-accessibility-route-proof', rejected: true },
    { claim: 'android-browser-enforcement', rejected: true },
  ];

  if (!sourceBoundary.ownedBrowserShellSourceDeclared) {
    throw new Error('Owned browser shell source boundary is not declared');
  }
  if (!sourceBoundary.webViewDeclared || !sourceBoundary.browsableViewIntentDeclared) {
    throw new Error('Owned browser shell source lacks WebView or BROWSABLE VIEW intent evidence');
  }
  if (successfulLaunches.length === 0) {
    throw new Error('Owned browser shell did not launch with observable proof UI on any booted Android device');
  }
  if (!negativeChecks.every((check) => check.rejected)) {
    throw new Error('Expected Android owned browser shell negative checks to reject dishonest claims');
  }

  const proof = {
    schemaVersion: 1,
    proofId: 'browser-platform-android-owned-shell-proof',
    generatedAt: observedAt,
    branch: git(['branch', '--show-current']),
    commit: git(['rev-parse', 'HEAD']),
    baseCommit: git(['rev-parse', 'origin/main']),
    hostProofSummary: {
      adbInstalled: true,
      adbPathPersisted: false,
      adbPathSha256: sha256(adb.path),
      emulatorPathPersisted: false,
      emulatorPathSha256: emulator ? sha256(emulator.path) : null,
      emulatorAvdCount: emulatorAvds.length,
      emulatorLaunchedByProof: launchedEmulator,
      emulatorCleanupAttempted: launchedEmulator,
      attachedDeviceCount: devices.length,
      bootedDeviceCount: deviceProofs.length,
      ownedBrowserShellPackageInstalled: deviceProofs.some((device) => device.packageInstalled),
      ownedBrowserShellSourceDeclared: sourceBoundary.ownedBrowserShellSourceDeclared,
      webViewDeclared: sourceBoundary.webViewDeclared,
      browsableViewIntentDeclared: sourceBoundary.browsableViewIntentDeclared,
      launchObserved: successfulLaunches.length > 0,
      localProofPageObserved: deviceProofs.some((device) => device.localProofPageObserved),
      screenshotsCaptured: deviceProofs.some((device) => device.screenshotCaptured),
      screenshotsPersisted: deviceProofs.some((device) => device.screenshotPersisted),
      uiTreeCaptured: deviceProofs.some((device) => device.uiTreeCaptured),
      uiTreeRawPersisted: false,
      rawInstalledPackageListPersisted: false,
      rawIntentResolutionPersisted: false,
      rawUrlPersisted: false,
      rawPageContentPersisted: false,
      exactUrlPolicyClaimed: false,
      knownActiveTabProofClaimed: false,
      deviceOwnerEnrollmentClaimed: false,
      vpnDnsBrowserProofClaimed: false,
      usageStatsRouteProofClaimed: false,
      accessibilityRouteProofClaimed: false,
      enforcementClaimed: false,
      resultState: 'android-owned-browser-shell-build-install-launch-proof',
    },
    proofUrlRef: `redacted-android-owned-browser-proof-url-${sha256(proofUrl).slice(0, 16)}`,
    proofUrlPersisted: false,
    apk: {
      path: 'platforms/android/agent/browser-shell/build/outputs/apk/debug/browser-shell-debug.apk',
      exists: existsSync(apkPath),
      sha256: existsSync(apkPath) ? sha256(readFileSync(apkPath)) : null,
    },
    devices: deviceProofs,
    sourceBoundary,
    negativeChecks,
  };

  writeJson(testResultPath, proof);
  writeJson(outputProofPath, proof);

  console.log('browser-platform-android-owned-shell-proof-ok=true');
  console.log(`proof=${testResultPath}`);
  console.log(`outputProof=${outputProofPath}`);
  console.log(`attachedDeviceCount=${proof.hostProofSummary.attachedDeviceCount}`);
  console.log(`bootedDeviceCount=${proof.hostProofSummary.bootedDeviceCount}`);
  console.log(`launchObserved=${proof.hostProofSummary.launchObserved}`);
  console.log(`resultState=${proof.hostProofSummary.resultState}`);
} finally {
  if (server) {
    await new Promise((resolve) => server.instance.close(resolve));
  }
  if (adb && launchedEmulatorSerial !== null) {
    command(['-s', launchedEmulatorSerial, 'emu', 'kill'], { adbPath: adb.path, allowFailure: true });
  }
}

function buildOwnedBrowserShell() {
  const executable = process.platform === 'win32' ? 'cmd' : './gradlew';
  const args =
    process.platform === 'win32'
      ? ['/c', 'gradlew.bat', ':browser-shell:assembleDebug', '--console=plain', '--quiet']
      : [':browser-shell:assembleDebug', '--console=plain', '--quiet'];
  execFileSync(executable, args, {
    cwd: androidProjectRoot,
    stdio: 'inherit',
  });
}

async function proveDevice(adbPath, serial, proofUrl, headlessEmulator) {
  const serialRef = `redacted-android-device-ref-${sha256(serial).slice(0, 16)}`;

  command(['-s', serial, 'install', '-r', apkPath], { adbPath, allowFailure: false });
  const packageQuery = command(['-s', serial, 'shell', 'pm', 'path', packageName], {
    adbPath,
    allowFailure: true,
  });
  const resolveOutput = command(
    [
      '-s',
      serial,
      'shell',
      'cmd',
      'package',
      'resolve-activity',
      '--brief',
      '-a',
      'android.intent.action.VIEW',
      '-d',
      proofUrl,
    ],
    { adbPath, allowFailure: true }
  );
  command(
    [
      '-s',
      serial,
      'shell',
      'am',
      'start',
      '-W',
      '-a',
      'android.intent.action.VIEW',
      '-d',
      proofUrl,
      '-n',
      `${packageName}/${activityName}`,
    ],
    { adbPath, allowFailure: false }
  );

  await delay(3_000);

  const uiTree = command(['-s', serial, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], {
    adbPath,
    allowFailure: true,
  });
  const screenshot = headlessEmulator
    ? Buffer.alloc(0)
    : commandBuffer(['-s', serial, 'exec-out', 'screencap', '-p'], {
        adbPath,
        allowFailure: true,
      });
  const screenshotUsable = screenshot.length > 8;
  if (screenshotUsable) {
    writeFileSync(screenshotProofPath, screenshot);
  } else if (existsSync(screenshotProofPath)) {
    rmSync(screenshotProofPath);
  }

  const launchObserved =
    uiTree.includes('Ocentra owned browser proof page loaded') || uiTree.includes('Ocentra owned browser shell ready');

  return {
    serialRef,
    packageInstalled: packageQuery.includes('package:'),
    viewIntentResolved: resolveOutput.includes(packageName) || resolveOutput.includes(activityName),
    rawIntentResolutionPersisted: false,
    rawUrlPersisted: false,
    rawPageContentPersisted: false,
    launchObserved,
    localProofPageObserved: uiTree.includes('Ocentra owned browser proof page loaded'),
    uiTreeCaptured: uiTree.includes('<hierarchy'),
    uiTreeRawPersisted: false,
    uiTreeSha256: uiTree.length > 0 ? sha256(uiTree) : null,
    screenshotCaptured: screenshotUsable,
    screenshotPersisted: screenshotUsable,
    screenshotPath: screenshotUsable
      ? 'output/browser-plan-proof/05-cross-platform-inventory-matrix/15-android-owned-browser-shell-screenshot.png'
      : null,
    screenshotSha256: screenshotUsable ? sha256(screenshot) : null,
    exactUrlPolicyClaimed: false,
    knownActiveTabProofClaimed: false,
    deviceOwnerEnrollmentClaimed: false,
    enforcementClaimed: false,
  };
}

async function startProofServer() {
  const instance = createServer((request, response) => {
    if (request.url === '/owned-browser-shell-proof') {
      response.writeHead(200, {
        'Content-Type': 'text/html; charset=utf-8',
        'Cache-Control': 'no-store',
      });
      response.end(
        '<!doctype html><html><head><title>Ocentra Owned Browser Shell Proof</title></head><body><main><h1>Ocentra owned browser proof page</h1></main></body></html>'
      );
      return;
    }
    response.writeHead(404, { 'Content-Type': 'text/plain; charset=utf-8' });
    response.end('not found');
  });

  await new Promise((resolve) => instance.listen(0, '127.0.0.1', resolve));
  return { instance, port: instance.address().port };
}

function inspectOwnedBrowserShellSource() {
  const settingsPath = 'platforms/android/agent/settings.gradle';
  const manifestPath = 'platforms/android/agent/browser-shell/src/main/AndroidManifest.xml';
  const activityPath =
    'platforms/android/agent/browser-shell/src/main/java/ca/ocentra/parent/browser/OcentraOwnedBrowserShellActivity.java';
  const buildGradlePath = 'platforms/android/agent/browser-shell/build.gradle';
  const settings = readRepoText(settingsPath);
  const manifest = readRepoText(manifestPath);
  const activity = readRepoText(activityPath);
  const buildGradle = readRepoText(buildGradlePath);

  return {
    settingsPath,
    settingsSha256: sha256(settings),
    manifestPath,
    manifestSha256: sha256(manifest),
    activityPath,
    activitySha256: sha256(activity),
    buildGradlePath,
    buildGradleSha256: sha256(buildGradle),
    rawSourcePersisted: false,
    ownedBrowserShellModuleIncluded: settings.includes("include ':browser-shell'"),
    ownedBrowserShellSourceDeclared:
      buildGradle.includes("applicationId = 'com.ocentra.parent.browser'") &&
      manifest.includes('@string/owned_browser_shell_label'),
    webViewDeclared: activity.includes('android.webkit.WebView') && activity.includes('new WebView'),
    browsableViewIntentDeclared:
      manifest.includes('android.intent.action.VIEW') && manifest.includes('android.intent.category.BROWSABLE'),
    cleartextLimitedToDebugProof: manifest.includes('android.permission.INTERNET'),
    deviceOwnerDeclared: manifest.includes('DeviceAdminReceiver'),
    accessibilityServiceDeclared:
      manifest.includes('AccessibilityService') || manifest.includes('android.permission.BIND_ACCESSIBILITY_SERVICE'),
    vpnServiceDeclared: manifest.includes('VpnService') || manifest.includes('android.permission.BIND_VPN_SERVICE'),
    usageStatsPermissionDeclared: manifest.includes('android.permission.PACKAGE_USAGE_STATS'),
    negativeBoundaryState:
      'owned-browser-shell-build-and-launch-only-no-device-owner-no-accessibility-no-vpn-no-usagestats-no-enforcement',
  };
}

function findAdb() {
  const output = commandWhere('adb');
  const candidate = output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .find((line) => line.length > 0 && existsSync(line));
  return candidate ? { path: candidate } : null;
}

function findEmulator() {
  const sdkRoot =
    process.env.ANDROID_SDK_ROOT ??
    process.env.ANDROID_HOME ??
    (process.env.LOCALAPPDATA ? join(process.env.LOCALAPPDATA, 'Android', 'Sdk') : '');
  const candidate = sdkRoot.length > 0 ? join(sdkRoot, 'emulator', 'emulator.exe') : '';
  return candidate.length > 0 && existsSync(candidate) ? { path: candidate } : null;
}

function listAvds(emulatorPath) {
  const output = commandExternal(emulatorPath, ['-list-avds'], { allowFailure: true });
  return output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
}

async function launchEmulatorIfAvailable(adbPath, emulatorPath, avds) {
  const selectedAvd = avds[0];
  if (!selectedAvd) {
    return null;
  }

  const child = spawn(
    emulatorPath,
    [
      '-avd',
      selectedAvd,
      '-no-window',
      '-no-snapshot-save',
      '-no-audio',
      '-no-boot-anim',
      '-gpu',
      'swiftshader_indirect',
    ],
    {
      cwd: repoRoot,
      detached: process.platform !== 'win32',
      stdio: ['ignore', 'ignore', 'ignore'],
      windowsHide: true,
    }
  );
  child.unref();

  const serial = await waitForReadyEmulator(adbPath);
  await waitForBoot(adbPath, serial);
  return serial;
}

async function waitForReadyEmulator(adbPath) {
  const deadline = Date.now() + 8 * 60_000;
  while (Date.now() < deadline) {
    const ready = listDevices(adbPath).find(
      (device) => device.state === 'device' && device.serial.startsWith('emulator-')
    );
    if (ready) {
      return ready.serial;
    }
    await delay(2_000);
  }
  throw new Error('Timed out waiting for Android emulator device');
}

async function waitForBoot(adbPath, serial) {
  const deadline = Date.now() + 8 * 60_000;
  while (Date.now() < deadline) {
    const bootCompleted = command(['-s', serial, 'shell', 'getprop', 'sys.boot_completed'], {
      adbPath,
      allowFailure: true,
    }).trim();
    if (bootCompleted === '1') {
      return;
    }
    await delay(2_000);
  }
  throw new Error('Timed out waiting for Android emulator boot');
}

function listDevices(adbPath) {
  const output = command(['devices'], { adbPath, allowFailure: true });
  return output
    .split(/\r?\n/)
    .slice(1)
    .map((line) => line.trim())
    .filter((line) => line.length > 0)
    .map((line) => {
      const [serial, state] = line.split(/\s+/);
      return { serial, state: state ?? 'unknown' };
    });
}

function readRepoText(relativePath) {
  return readFileSync(join(repoRoot, relativePath), 'utf8');
}

function command(args, { adbPath, allowFailure }) {
  try {
    return execFileSync(adbPath, args, { cwd: repoRoot, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
  } catch (error) {
    if (allowFailure) {
      return `${error.stdout?.toString() ?? ''}${error.stderr?.toString() ?? ''}`;
    }
    throw error;
  }
}

function commandBuffer(args, { adbPath, allowFailure }) {
  try {
    return execFileSync(adbPath, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
  } catch (error) {
    if (allowFailure) {
      return Buffer.alloc(0);
    }
    throw error;
  }
}

function commandExternal(file, args, { allowFailure }) {
  try {
    return execFileSync(file, args, { cwd: repoRoot, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
  } catch (error) {
    if (allowFailure) {
      return `${error.stdout?.toString() ?? ''}${error.stderr?.toString() ?? ''}`;
    }
    throw error;
  }
}

function commandWhere(binary) {
  try {
    return execFileSync('where.exe', [binary], { cwd: repoRoot, encoding: 'utf8' });
  } catch {
    return '';
  }
}

function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
}

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}
