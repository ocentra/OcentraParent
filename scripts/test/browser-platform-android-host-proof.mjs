import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(scriptDir, '..', '..');
const proofRoot = join(repoRoot, 'output/browser-plan-proof/05-cross-platform-inventory-matrix');
const testResultPath = join(repoRoot, 'test-results/browser-platform-android-host-proof/proof.json');
const outputProofPath = join(proofRoot, '11-android-host-device-proof.json');
const observedAt = new Date().toISOString();

const targetPackages = [
  { targetId: 'android-chrome', packageName: 'com.android.chrome' },
  { targetId: 'firefox-android', packageName: 'org.mozilla.firefox' },
  { targetId: 'edge-android', packageName: 'com.microsoft.emmx' },
  { targetId: 'samsung-internet', packageName: 'com.sec.android.app.sbrowser' },
  { targetId: 'ocentra-owned-browser-shell', packageName: 'com.ocentra.parent.browser' },
];

mkdirSync(proofRoot, { recursive: true });

const adb = findAdb();
const adbVersion = adb ? command(['version'], { allowFailure: true, adbPath: adb.path }) : null;
const devices = adb ? listDevices(adb.path) : [];
const attachedDevices = devices.filter((device) => device.state === 'device');
const packageVisibility = [];
const defaultViewHandlers = [];

for (const device of attachedDevices) {
  const bootCompleted = command(['-s', device.serial, 'shell', 'getprop', 'sys.boot_completed'], {
    adbPath: adb.path,
    allowFailure: true,
  }).trim();
  device.bootCompleted = bootCompleted === '1';

  for (const target of targetPackages) {
    packageVisibility.push(queryPackage(adb.path, device.serial, target));
  }

  defaultViewHandlers.push(resolveDefaultViewHandler(adb.path, device.serial));
}

const ownedShellVisible = packageVisibility.some(
  (entry) => entry.targetId === 'ocentra-owned-browser-shell' && entry.installed
);
const browserPackageVisible = packageVisibility.some(
  (entry) => entry.targetId !== 'ocentra-owned-browser-shell' && entry.installed
);
const bootedDeviceCount = attachedDevices.filter((device) => device.bootCompleted).length;
const negativeChecks = [
  {
    claim: 'owned-browser-shell-custody',
    rejected: !ownedShellVisible,
  },
  {
    claim: 'managed-exact-url-on-android',
    rejected: true,
  },
  {
    claim: 'known-active-tab-on-android',
    rejected: true,
  },
  {
    claim: 'android-browser-enforcement',
    rejected: true,
  },
];

if (!negativeChecks.every((check) => check.rejected)) {
  throw new Error('Expected Android browser host proof negative checks to reject dishonest claims');
}

const proof = {
  schemaVersion: 1,
  proofId: 'browser-platform-android-host-proof',
  generatedAt: observedAt,
  branch: git(['branch', '--show-current']),
  commit: git(['rev-parse', 'HEAD']),
  baseCommit: git(['rev-parse', 'origin/main']),
  hostProofSummary: {
    adbInstalled: adb !== null,
    adbPathPersisted: false,
    adbPathSha256: adb ? sha256(adb.path) : null,
    adbVersionSha256: adbVersion ? sha256(adbVersion) : null,
    attachedDeviceCount: attachedDevices.length,
    bootedDeviceCount,
    realDeviceOrEmulatorInspected: bootedDeviceCount > 0,
    knownBrowserPackageIdsQueriedOnly: true,
    browserPackageVisible,
    ownedBrowserShellVisible: ownedShellVisible,
    defaultViewHandlerQueried: bootedDeviceCount > 0,
    rawInstalledPackageListPersisted: false,
    screenshotsCaptured: false,
    uiTreeCaptured: false,
    logcatCaptured: false,
    exactUrlProofClaimed: false,
    knownActiveTabProofClaimed: false,
    ownedShellCustodyClaimed: false,
    managedProfileClaimed: false,
    deviceOwnerEnrollmentClaimed: false,
    vpnDnsBrowserProofClaimed: false,
    usageStatsRouteProofClaimed: false,
    accessibilityRouteProofClaimed: false,
    enforcementClaimed: false,
    resultState:
      bootedDeviceCount > 0 ? 'android-browser-package-visibility-proof' : 'manual-android-device-proof-required',
  },
  devices: attachedDevices.map((device) => ({
    serialRef: `redacted-android-device-ref-${sha256(device.serial).slice(0, 16)}`,
    state: device.state,
    bootCompleted: device.bootCompleted,
    rawSerialPersisted: false,
  })),
  packageVisibility,
  defaultViewHandlers,
  negativeChecks,
};

writeJson(testResultPath, proof);
writeJson(outputProofPath, proof);

console.log('browser-platform-android-host-proof-ok=true');
console.log(`proof=${testResultPath}`);
console.log(`outputProof=${outputProofPath}`);
console.log(`adbInstalled=${adb !== null}`);
console.log(`attachedDeviceCount=${attachedDevices.length}`);
console.log(`bootedDeviceCount=${bootedDeviceCount}`);
console.log(`resultState=${proof.hostProofSummary.resultState}`);

function findAdb() {
  const output = commandWhere('adb');
  const candidate = output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .find((line) => line.length > 0 && existsSync(line));
  return candidate ? { path: candidate } : null;
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

function queryPackage(adbPath, serial, target) {
  const output = command(['-s', serial, 'shell', 'pm', 'path', target.packageName], {
    adbPath,
    allowFailure: true,
  });
  return {
    serialRef: `redacted-android-device-ref-${sha256(serial).slice(0, 16)}`,
    targetId: target.targetId,
    targetPackageId: target.packageName,
    installed: output.includes('package:'),
    rawInstalledPackageListPersisted: false,
    exactUrlProofClaimed: false,
    knownActiveTabProofClaimed: false,
    ownedShellCustodyClaimed: false,
    enforcementClaimed: false,
  };
}

function resolveDefaultViewHandler(adbPath, serial) {
  const output = command(
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
      'https://example.com',
    ],
    {
      adbPath,
      allowFailure: true,
    }
  );
  const lines = output
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  const component = lines.at(-1) ?? 'unresolved';
  return {
    serialRef: `redacted-android-device-ref-${sha256(serial).slice(0, 16)}`,
    componentRef:
      component === 'unresolved' ? 'unresolved' : `redacted-android-view-handler-${sha256(component).slice(0, 16)}`,
    resolved: component !== 'unresolved' && !component.includes('No activity found'),
    rawComponentPersisted: false,
    urlPersisted: false,
    urlContentCaptured: false,
    exactUrlProofClaimed: false,
  };
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
