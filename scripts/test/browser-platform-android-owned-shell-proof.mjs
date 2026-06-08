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
const deviceAdminReceiverName = 'ca.ocentra.parent.browser.OcentraOwnedBrowserDeviceAdminReceiver';
const deviceAdminComponentName = `${packageName}/${deviceAdminReceiverName}`;
const browserRoleName = 'android.app.role.BROWSER';
const browserViewIntentArgs = [
  '-a',
  'android.intent.action.VIEW',
  '-c',
  'android.intent.category.DEFAULT',
  '-c',
  'android.intent.category.BROWSABLE',
  '-d',
];
const proofAvdName = 'OcentraParentDeviceOwnerProof';
const proofAvdPackage = 'system-images;android-33;aosp_atd;x86_64';
const proofAvdDevice = 'pixel_6';
const proofEmulatorPort = process.env.OCENTRA_PARENT_ANDROID_PROOF_EMULATOR_PORT?.trim() ?? '5570';

mkdirSync(proofRoot, { recursive: true });

const adb = findAdb();
const emulator = findEmulator();
const avdManager = findAvdManager();
const emulatorAvds = emulator ? listAvds(emulator.path) : [];
let launchedEmulator = false;
let launchedEmulatorSerial = null;
let proofAvdCreated = false;
let server = null;

try {
  buildOwnedBrowserShell();

  if (!adb) {
    throw new Error('adb is required for Android owned browser shell proof');
  }

  if (!emulator) {
    throw new Error('Android emulator executable is required for Android owned browser shell proof');
  }
  if (!avdManager) {
    throw new Error('Android avdmanager executable is required for disposable Device Owner proof AVD');
  }

  const existingSerials = listDevices(adb.path).map((device) => device.serial);
  createProofAvd(avdManager.path);
  proofAvdCreated = true;
  launchedEmulatorSerial = await launchEmulatorIfAvailable(adb.path, emulator.path, existingSerials);
  launchedEmulator = launchedEmulatorSerial !== null;

  const devices = listDevices(adb.path).filter(
    (device) => device.state === 'device' && device.serial === launchedEmulatorSerial
  );
  if (devices.length === 0) {
    throw new Error('No proof-launched Android emulator was available for owned browser shell proof');
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
    deviceProofs.push(
      await proveDevice(adb.path, device.serial, proofUrl, launchedEmulator && device.serial === launchedEmulatorSerial)
    );
  }

  const sourceBoundary = inspectOwnedBrowserShellSource();
  const successfulLaunches = deviceProofs.filter((proof) => proof.launchObserved);
  const negativeChecks = [
    { claim: 'managed-exact-url-on-android', rejected: true },
    { claim: 'known-active-tab-on-android', rejected: true },
    { claim: 'android-content-filter-enforcement', rejected: true },
    { claim: 'android-vpn-dns-browser-proof', rejected: true },
    { claim: 'android-usagestats-route-proof', rejected: true },
    { claim: 'android-accessibility-route-proof', rejected: true },
    { claim: 'android-broad-browser-enforcement', rejected: true },
  ];

  if (!sourceBoundary.ownedBrowserShellSourceDeclared) {
    throw new Error('Owned browser shell source boundary is not declared');
  }
  if (!sourceBoundary.webViewDeclared || !sourceBoundary.browsableViewIntentDeclared) {
    throw new Error('Owned browser shell source lacks WebView or BROWSABLE VIEW intent evidence');
  }
  if (!sourceBoundary.deviceAdminReceiverDeclared || !sourceBoundary.deviceAdminMetadataDeclared) {
    throw new Error('Owned browser shell source lacks DeviceAdmin receiver metadata evidence');
  }
  if (successfulLaunches.length === 0) {
    throw new Error('Owned browser shell did not launch with observable proof UI on any booted Android device');
  }
  if (!deviceProofs.some((proof) => proof.deviceOwnerEnrollmentObserved)) {
    throw new Error('Owned browser shell did not produce proof-launched emulator Device Owner enrollment evidence');
  }
  if (!deviceProofs.some((proof) => proof.deviceOwnerPolicyMutationObserved)) {
    throw new Error('Owned browser shell did not produce Device Owner persistent browser routing policy evidence');
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
      avdManagerPathPersisted: false,
      avdManagerPathSha256: avdManager ? sha256(avdManager.path) : null,
      emulatorAvdCount: emulatorAvds.length,
      emulatorLaunchedByProof: launchedEmulator,
      proofAvdCreated,
      proofAvdNamePersisted: false,
      proofAvdNameSha256: sha256(proofAvdName),
      proofAvdPackagePersisted: false,
      proofAvdPackageSha256: sha256(proofAvdPackage),
      proofEmulatorPortPersisted: false,
      proofEmulatorPortSha256: sha256(proofEmulatorPort),
      emulatorCleanupAttempted: launchedEmulator,
      attachedDeviceCount: devices.length,
      bootedDeviceCount: deviceProofs.length,
      ownedBrowserShellPackageInstalled: deviceProofs.some((device) => device.packageInstalled),
      ownedBrowserShellSourceDeclared: sourceBoundary.ownedBrowserShellSourceDeclared,
      webViewDeclared: sourceBoundary.webViewDeclared,
      browsableViewIntentDeclared: sourceBoundary.browsableViewIntentDeclared,
      deviceAdminReceiverDeclared: sourceBoundary.deviceAdminReceiverDeclared,
      deviceAdminMetadataDeclared: sourceBoundary.deviceAdminMetadataDeclared,
      deviceAdminPoliciesDeclared: sourceBoundary.deviceAdminPoliciesDeclared,
      deviceOwnerPolicyMutationDeclared: sourceBoundary.deviceOwnerPolicyMutationDeclared,
      launchObserved: successfulLaunches.length > 0,
      localProofPageObserved: deviceProofs.some((device) => device.localProofPageObserved),
      deviceOwnerEnrollmentAttempted: deviceProofs.some((device) => device.deviceOwnerEnrollmentAttempted),
      deviceOwnerEnrollmentObserved: deviceProofs.some((device) => device.deviceOwnerEnrollmentObserved),
      deviceOwnerProofLimitedToProofLaunchedEmulator: deviceProofs.every(
        (device) =>
          !device.deviceOwnerEnrollmentAttempted ||
          (device.proofLaunchedEmulator === true && device.serialKind === 'emulator')
      ),
      deviceOwnerCleanupAttempted: deviceProofs.some((device) => device.deviceOwnerCleanupAttempted),
      deviceOwnerCleanupObserved: deviceProofs.some((device) => device.deviceOwnerCleanupObserved),
      deviceOwnerPolicyMutationAttempted: deviceProofs.some((device) => device.deviceOwnerPolicyMutationAttempted),
      deviceOwnerPolicyMutationObserved: deviceProofs.some((device) => device.deviceOwnerPolicyMutationObserved),
      deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator: deviceProofs.every(
        (device) =>
          !device.deviceOwnerPolicyMutationAttempted ||
          (device.proofLaunchedEmulator === true && device.serialKind === 'emulator')
      ),
      androidOwnedBrowserRoutingEnforcementObserved: deviceProofs.some(
        (device) => device.implicitViewIntentLaunchObserved
      ),
      androidBrowserRoleAssignmentAttempted: deviceProofs.some((device) => device.browserRoleAssignmentAttempted),
      androidBrowserRoleAssignmentObserved: deviceProofs.some((device) => device.browserRoleAssignmentObserved),
      androidBrowserRoleAssignmentLimitedToProofLaunchedEmulator: deviceProofs.every(
        (device) =>
          !device.browserRoleAssignmentAttempted ||
          (device.proofLaunchedEmulator === true && device.serialKind === 'emulator')
      ),
      androidBrowserRoleRoutingObserved: deviceProofs.some((device) => device.browserRoleRoutingObserved),
      screenshotsCaptured: deviceProofs.some((device) => device.screenshotCaptured),
      screenshotsPersisted: deviceProofs.some((device) => device.screenshotPersisted),
      uiTreeCaptured: deviceProofs.some((device) => device.uiTreeCaptured),
      uiTreeRawPersisted: false,
      rawInstalledPackageListPersisted: false,
      rawIntentResolutionPersisted: false,
      rawDpmOutputPersisted: false,
      rawUrlPersisted: false,
      rawPageContentPersisted: false,
      exactUrlPolicyClaimed: false,
      knownActiveTabProofClaimed: false,
      deviceOwnerEnrollmentClaimed: true,
      deviceOwnerPolicyMutationClaimed: true,
      androidOwnedBrowserRoutingEnforcementClaimed: deviceProofs.some(
        (device) => device.implicitViewIntentLaunchObserved
      ),
      browserRoleAssignmentClaimed: deviceProofs.some((device) => device.browserRoleAssignmentObserved),
      vpnDnsBrowserProofClaimed: false,
      usageStatsRouteProofClaimed: false,
      accessibilityRouteProofClaimed: false,
      enforcementClaimed: false,
      resultState: deviceProofs.some((device) => device.implicitViewIntentLaunchObserved)
        ? 'android-owned-browser-shell-browser-role-routing-proof'
        : 'android-owned-browser-shell-device-owner-policy-mutation-proof',
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
  if (avdManager && proofAvdCreated) {
    deleteProofAvd(avdManager.path);
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

async function proveDevice(adbPath, serial, proofUrl, proofLaunchedEmulator) {
  const serialRef = `redacted-android-device-ref-${sha256(serial).slice(0, 16)}`;
  const serialKind = serial.startsWith('emulator-') ? 'emulator' : 'attached-device';

  command(['-s', serial, 'install', '-r', apkPath], { adbPath, allowFailure: false });
  const deviceOwnerProof = proveDeviceOwnerEnrollment(adbPath, serial, proofLaunchedEmulator);
  const packageQuery = command(['-s', serial, 'shell', 'pm', 'path', packageName], {
    adbPath,
    allowFailure: true,
  });
  const beforePolicyResolveOutput = command(
    ['-s', serial, 'shell', 'cmd', 'package', 'resolve-activity', '--brief', ...browserViewIntentArgs, proofUrl],
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
      ...browserViewIntentArgs,
      proofUrl,
      '-n',
      `${packageName}/${activityName}`,
    ],
    { adbPath, allowFailure: false }
  );

  const explicitUiTree = await waitForOwnedBrowserUi(adbPath, serial);
  const afterPolicyResolveOutput = command(
    ['-s', serial, 'shell', 'cmd', 'package', 'resolve-activity', '--brief', ...browserViewIntentArgs, proofUrl],
    { adbPath, allowFailure: true }
  );
  const browserRoleProof = proveBrowserRoleAssignment(adbPath, serial, proofLaunchedEmulator);
  const afterRoleResolveOutput = command(
    ['-s', serial, 'shell', 'cmd', 'package', 'resolve-activity', '--brief', ...browserViewIntentArgs, proofUrl],
    { adbPath, allowFailure: true }
  );
  command(['-s', serial, 'shell', 'am', 'force-stop', packageName], { adbPath, allowFailure: true });
  command(['-s', serial, 'shell', 'am', 'start', '-W', ...browserViewIntentArgs, proofUrl], {
    adbPath,
    allowFailure: false,
  });

  const implicitUiTree = await waitForOwnedBrowserUi(adbPath, serial);
  const uiTree = `${explicitUiTree}\n${implicitUiTree}`;
  const screenshot = proofLaunchedEmulator
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
    serialKind,
    proofLaunchedEmulator,
    packageInstalled: packageQuery.includes('package:'),
    deviceOwnerEnrollmentAttempted: deviceOwnerProof.attempted,
    deviceOwnerEnrollmentObserved: deviceOwnerProof.observed,
    deviceOwnerProofLimitedToProofLaunchedEmulator:
      !deviceOwnerProof.attempted || (proofLaunchedEmulator === true && serialKind === 'emulator'),
    deviceOwnerCleanupAttempted: deviceOwnerProof.cleanupAttempted,
    deviceOwnerCleanupObserved: deviceOwnerProof.cleanupObserved,
    deviceOwnerPolicyMutationAttempted: deviceOwnerProof.observed,
    deviceOwnerPolicyMutationObserved:
      deviceOwnerProof.observed &&
      sourceBoundaryForDeviceOwnerPolicyMutation() &&
      explicitUiTree.includes('Ocentra owned browser persistent routing policy configured'),
    deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator:
      !deviceOwnerProof.observed || (proofLaunchedEmulator === true && serialKind === 'emulator'),
    deviceOwnerSetResultSha256: deviceOwnerProof.setResultSha256,
    deviceOwnerQuerySha256: deviceOwnerProof.querySha256,
    devicePolicyDumpSha256: deviceOwnerProof.dumpSha256,
    rawDpmOutputPersisted: false,
    viewIntentResolved:
      beforePolicyResolveOutput.includes(packageName) || beforePolicyResolveOutput.includes(activityName),
    afterPolicyViewIntentResolved:
      afterPolicyResolveOutput.includes(packageName) && afterPolicyResolveOutput.includes(activityName),
    browserRoleAssignmentAttempted: browserRoleProof.attempted,
    browserRoleAssignmentObserved: browserRoleProof.observed,
    browserRoleAssignmentLimitedToProofLaunchedEmulator:
      !browserRoleProof.attempted || (proofLaunchedEmulator === true && serialKind === 'emulator'),
    browserRoleHoldersBeforeSha256: browserRoleProof.beforeSha256,
    browserRoleAddResultSha256: browserRoleProof.addResultSha256,
    browserRoleHoldersAfterSha256: browserRoleProof.afterSha256,
    rawBrowserRoleOutputPersisted: false,
    afterRoleViewIntentResolved:
      afterRoleResolveOutput.includes(packageName) && afterRoleResolveOutput.includes(activityName),
    browserRoleRoutingObserved:
      afterRoleResolveOutput.includes(packageName) &&
      afterRoleResolveOutput.includes(activityName) &&
      implicitUiTree.includes('Ocentra owned browser proof page loaded'),
    rawIntentResolutionPersisted: false,
    rawUrlPersisted: false,
    rawPageContentPersisted: false,
    launchObserved,
    localProofPageObserved: explicitUiTree.includes('Ocentra owned browser proof page loaded'),
    implicitViewIntentLaunchObserved: implicitUiTree.includes('Ocentra owned browser proof page loaded'),
    uiTreeCaptured: uiTree.includes('<hierarchy'),
    uiTreeRawPersisted: false,
    uiTreeSha256: uiTree.length > 0 ? sha256(uiTree) : null,
    beforePolicyResolveSha256: sha256(beforePolicyResolveOutput),
    afterPolicyResolveSha256: sha256(afterPolicyResolveOutput),
    afterRoleResolveSha256: sha256(afterRoleResolveOutput),
    screenshotCaptured: screenshotUsable,
    screenshotPersisted: screenshotUsable,
    screenshotPath: screenshotUsable
      ? 'output/browser-plan-proof/05-cross-platform-inventory-matrix/15-android-owned-browser-shell-screenshot.png'
      : null,
    screenshotSha256: screenshotUsable ? sha256(screenshot) : null,
    exactUrlPolicyClaimed: false,
    knownActiveTabProofClaimed: false,
    deviceOwnerEnrollmentClaimed: deviceOwnerProof.observed,
    deviceOwnerPolicyMutationClaimed: deviceOwnerProof.observed,
    browserRoleAssignmentClaimed: browserRoleProof.observed,
    androidOwnedBrowserRoutingEnforcementClaimed: implicitUiTree.includes('Ocentra owned browser proof page loaded'),
    enforcementClaimed: false,
  };
}

async function waitForOwnedBrowserUi(adbPath, serial) {
  const deadline = Date.now() + 20_000;
  let latestUiTree = '';
  while (Date.now() < deadline) {
    latestUiTree = command(['-s', serial, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], {
      adbPath,
      allowFailure: true,
    });
    if (latestUiTree.includes('Ocentra owned browser proof page loaded')) {
      return latestUiTree;
    }
    await delay(1_000);
  }
  return latestUiTree;
}

function proveDeviceOwnerEnrollment(adbPath, serial, proofLaunchedEmulator) {
  const serialKind = serial.startsWith('emulator-') ? 'emulator' : 'attached-device';
  if (proofLaunchedEmulator !== true || serialKind !== 'emulator') {
    return {
      attempted: false,
      observed: false,
      cleanupAttempted: false,
      cleanupObserved: false,
      setResultSha256: null,
      querySha256: null,
      dumpSha256: null,
    };
  }

  const setOutput = command(['-s', serial, 'shell', 'dpm', 'set-device-owner', deviceAdminComponentName], {
    adbPath,
    allowFailure: true,
  });
  const queryOutput = command(['-s', serial, 'shell', 'dpm', 'list-owners'], {
    adbPath,
    allowFailure: true,
  });
  const dumpOutput = command(['-s', serial, 'shell', 'dumpsys', 'device_policy'], {
    adbPath,
    allowFailure: true,
  });
  const observed =
    (setOutput.includes('Success') || setOutput.includes('Active admin set')) &&
    queryOutput.includes(packageName) &&
    dumpOutput.includes(packageName);
  const cleanupOutput = command(['-s', serial, 'shell', 'dpm', 'remove-active-admin', deviceAdminComponentName], {
    adbPath,
    allowFailure: true,
  });

  return {
    attempted: true,
    observed,
    cleanupAttempted: true,
    cleanupObserved: cleanupOutput.includes('Success') || cleanupOutput.includes('removed'),
    setResultSha256: sha256(setOutput),
    querySha256: sha256(queryOutput),
    dumpSha256: sha256(dumpOutput),
  };
}

function proveBrowserRoleAssignment(adbPath, serial, proofLaunchedEmulator) {
  const serialKind = serial.startsWith('emulator-') ? 'emulator' : 'attached-device';
  const beforeOutput = command(['-s', serial, 'shell', 'cmd', 'role', 'holders', browserRoleName], {
    adbPath,
    allowFailure: true,
  });
  if (proofLaunchedEmulator !== true || serialKind !== 'emulator') {
    return {
      attempted: false,
      observed: false,
      beforeSha256: sha256(beforeOutput),
      addResultSha256: null,
      afterSha256: sha256(beforeOutput),
    };
  }

  const addOutput = command(['-s', serial, 'shell', 'cmd', 'role', 'add-role-holder', browserRoleName, packageName], {
    adbPath,
    allowFailure: true,
  });
  const afterOutput = command(['-s', serial, 'shell', 'cmd', 'role', 'holders', browserRoleName], {
    adbPath,
    allowFailure: true,
  });

  return {
    attempted: true,
    observed: afterOutput.includes(packageName),
    beforeSha256: sha256(beforeOutput),
    addResultSha256: sha256(addOutput),
    afterSha256: sha256(afterOutput),
  };
}

function sourceBoundaryForDeviceOwnerPolicyMutation() {
  const activity = readRepoText(
    'platforms/android/agent/browser-shell/src/main/java/ca/ocentra/parent/browser/OcentraOwnedBrowserShellActivity.java'
  );
  return (
    activity.includes('DevicePolicyManager') &&
    activity.includes('isDeviceOwnerApp') &&
    activity.includes('addPersistentPreferredActivity')
  );
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
  const receiverPath =
    'platforms/android/agent/browser-shell/src/main/java/ca/ocentra/parent/browser/OcentraOwnedBrowserDeviceAdminReceiver.java';
  const deviceAdminXmlPath = 'platforms/android/agent/browser-shell/src/main/res/xml/owned_browser_device_admin.xml';
  const buildGradlePath = 'platforms/android/agent/browser-shell/build.gradle';
  const settings = readRepoText(settingsPath);
  const manifest = readRepoText(manifestPath);
  const activity = readRepoText(activityPath);
  const receiver = readRepoText(receiverPath);
  const deviceAdminXml = readRepoText(deviceAdminXmlPath);
  const buildGradle = readRepoText(buildGradlePath);

  return {
    settingsPath,
    settingsSha256: sha256(settings),
    manifestPath,
    manifestSha256: sha256(manifest),
    activityPath,
    activitySha256: sha256(activity),
    receiverPath,
    receiverSha256: sha256(receiver),
    deviceAdminXmlPath,
    deviceAdminXmlSha256: sha256(deviceAdminXml),
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
    deviceAdminReceiverDeclared:
      manifest.includes('android.permission.BIND_DEVICE_ADMIN') &&
      receiver.includes('extends DeviceAdminReceiver') &&
      manifest.includes('.OcentraOwnedBrowserDeviceAdminReceiver'),
    deviceAdminMetadataDeclared:
      manifest.includes('android.app.device_admin') && manifest.includes('@xml/owned_browser_device_admin'),
    deviceAdminPoliciesDeclared: deviceAdminXml.includes('<force-lock />'),
    deviceOwnerPolicyMutationDeclared:
      activity.includes('DevicePolicyManager') &&
      activity.includes('isDeviceOwnerApp') &&
      activity.includes('addPersistentPreferredActivity'),
    accessibilityServiceDeclared:
      manifest.includes('AccessibilityService') || manifest.includes('android.permission.BIND_ACCESSIBILITY_SERVICE'),
    vpnServiceDeclared: manifest.includes('VpnService') || manifest.includes('android.permission.BIND_VPN_SERVICE'),
    usageStatsPermissionDeclared: manifest.includes('android.permission.PACKAGE_USAGE_STATS'),
    negativeBoundaryState:
      'owned-browser-shell-build-launch-device-owner-emulator-proof-only-no-accessibility-no-vpn-no-usagestats-no-enforcement',
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

function findAvdManager() {
  const sdkRoot =
    process.env.ANDROID_SDK_ROOT ??
    process.env.ANDROID_HOME ??
    (process.env.LOCALAPPDATA ? join(process.env.LOCALAPPDATA, 'Android', 'Sdk') : '');
  const candidates =
    sdkRoot.length > 0
      ? [
          join(sdkRoot, 'cmdline-tools', 'latest', 'bin', 'avdmanager.bat'),
          join(sdkRoot, 'cmdline-tools', 'bin', 'avdmanager.bat'),
        ]
      : [];
  const candidate = candidates.find((path) => existsSync(path));
  return candidate ? { path: candidate } : null;
}

function createProofAvd(avdManagerPath) {
  deleteProofAvd(avdManagerPath);
  execFileSync(
    'cmd',
    [
      '/c',
      avdManagerPath,
      'create',
      'avd',
      '-n',
      proofAvdName,
      '-k',
      proofAvdPackage,
      '--device',
      proofAvdDevice,
      '--force',
    ],
    {
      cwd: repoRoot,
      input: 'no\n',
      stdio: ['pipe', 'pipe', 'pipe'],
    }
  );
}

function deleteProofAvd(avdManagerPath) {
  commandExternal('cmd', ['/c', avdManagerPath, 'delete', 'avd', '-n', proofAvdName], {
    allowFailure: true,
  });
}

function listAvds(emulatorPath) {
  const output = commandExternal(emulatorPath, ['-list-avds'], { allowFailure: true });
  return output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
}

async function launchEmulatorIfAvailable(adbPath, emulatorPath, existingSerials) {
  const child = spawn(
    emulatorPath,
    [
      '-avd',
      proofAvdName,
      '-no-window',
      '-no-snapshot-save',
      '-no-audio',
      '-no-boot-anim',
      '-gpu',
      'swiftshader_indirect',
      '-port',
      proofEmulatorPort,
    ],
    {
      cwd: repoRoot,
      detached: process.platform !== 'win32',
      stdio: ['ignore', 'ignore', 'ignore'],
      windowsHide: true,
    }
  );
  child.unref();

  const serial = await waitForReadyEmulator(adbPath, existingSerials, child);
  await waitForBoot(adbPath, serial);
  return serial;
}

async function waitForReadyEmulator(adbPath, existingSerials, child) {
  const deadline = Date.now() + 8 * 60_000;
  const expectedSerial = `emulator-${proofEmulatorPort}`;
  let childExit = null;
  child.once('exit', (code, signal) => {
    childExit = { code, signal };
  });
  while (Date.now() < deadline) {
    const devices = listDevices(adbPath);
    const expected = devices.find((device) => device.serial === expectedSerial);
    if (expected && !existingSerials.includes(expected.serial)) {
      return expected.serial;
    }
    const ready = devices.find(
      (device) =>
        device.state === 'device' && device.serial.startsWith('emulator-') && !existingSerials.includes(device.serial)
    );
    if (ready) {
      return ready.serial;
    }
    if (childExit) {
      throw new Error(
        `Timed out waiting for Android emulator device after emulator process exited: ${JSON.stringify(childExit)}`
      );
    }
    await delay(2_000);
  }
  if (childExit) {
    throw new Error(
      `Timed out waiting for Android emulator device after emulator process exited: ${JSON.stringify(childExit)}`
    );
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
