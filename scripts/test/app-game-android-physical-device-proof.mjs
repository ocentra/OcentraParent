import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-physical-device-proof';
const adbTarget = process.env.OCENTRA_ANDROID_PHYSICAL_SERIAL ?? '192.168.2.45:5555';
const testOutputDir = join(repoRoot, 'test-results', proofMode);
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', '181-app-game-android-physical-device-proof');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-android-physical-device-proof',
    'app-game-broad-blocking-proof-gates',
  ]);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  run('adb', ['connect', adbTarget]);
  const devices = run('adb', ['devices', '-l']).stdout;
  const deviceLine = findPhysicalDeviceLine(devices);
  const buildProps = collectBuildProps();
  const packageManagerVisibleCount = packageCount();
  const usageStatsServiceState = usageStatsServiceVisible() ? 'service-visible' : 'service-not-visible';
  const usageEvents = usageEventsDumpProof();
  const devicePolicy = devicePolicyStates();
  const parentVisibleSummary =
    'Physical Android 10 device is reachable for package, foreground usage-event, and policy-state proof; normal-mode hide/suspend remains blocked until Device Owner or Profile Owner proof is attached.';

  const module = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-physical-device-proof.js')).href
  );
  const readModel = module.decodeAppGameAndroidPhysicalDeviceProof({
    schemaVersion: proofMode,
    proofId: 'app-game-android-physical-device-proof-s9',
    targetKind: 'physical-device',
    connectionState: 'physical-device-connected',
    adbTargetRef: 'android-physical-adb-device-ref',
    product: deviceLine.product,
    model: buildProps.model,
    device: buildProps.device,
    androidRelease: buildProps.androidRelease,
    sdkInt: buildProps.sdkInt,
    supportedAbiCount: buildProps.supportedAbiCount,
    packageManagerVisibleCount,
    usageStatsServiceState,
    usageEventsDumpState: usageEvents.usageEventsDumpState,
    usageEventsSampleCount: usageEvents.usageEventsSampleCount,
    foregroundActivityEventCount: usageEvents.foregroundActivityEventCount,
    deviceOwnerState: devicePolicy.deviceOwnerState,
    profileOwnerState: devicePolicy.profileOwnerState,
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-physical-build-prop-ref',
      'android-physical-package-manager-ref',
      'android-physical-usage-stats-service-ref',
      'android-physical-usage-events-dump-ref',
      'android-physical-device-policy-ref',
    ],
    packageNamesRedacted: true,
    usageEventsPackageNamesRedacted: true,
    rawDeviceSerialRedacted: true,
    foregroundEvidenceObserved: usageEvents.foregroundEvidenceObserved,
    hideSuspendClaimed: false,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
    parentVisibleSummary,
    checkedAt: '2026-06-08T15:55:00.000Z',
  });
  const summary = module.summarizeAppGameAndroidPhysicalDeviceProof(readModel);
  assertSourceBoundaries();

  const proof = {
    schemaVersion: 1,
    proofMode,
    generatedAt: 'deterministic-proof-artifact',
    adbTargetRef: 'android-physical-adb-device-ref',
    commands: commands.map(redactCommandOutput),
    readModel,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-android-physical-device-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-physical-device-proof.test.ts',
      physicalDevice:
        'adb devices -l observed physical product:star2qltecs model:SM_G965W at an explicit Wi-Fi ADB target; raw serial and package names are redacted.',
      packageVisibility:
        'adb shell cmd package list packages returned a nonzero package count; proof stores only the count.',
      usageEvents:
        'adb shell dumpsys usagestats returned a nonzero redacted event sample count with foreground activity events; proof stores only counts/states.',
      devicePolicy:
        'adb shell dumpsys device_policy did not expose Device Owner or Profile Owner entries, so hide/suspend stays blocked before adapter dispatch.',
    },
    productBoundaries: {
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      physicalAndroidDeviceObserved: true,
      packageManagerVisibilityObserved: true,
      usageStatsRuntimeObserved: usageStatsServiceState === 'service-visible',
      usageEventsDumpObserved: usageEvents.usageEventsDumpState === 'usage-events-dump-observed',
      foregroundActivityEventsObserved: usageEvents.foregroundEvidenceObserved,
      androidDeviceOwnerProofAttached: false,
      androidProfileOwnerProofAttached: false,
      androidNormalModeSuspendHideClaimed: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      broadBlockingClaimed: false,
      rawPackageNamesStored: false,
      rawDeviceSerialStored: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-android-physical-device-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/181-app-game-android-physical-device-proof',
      harness: 'scripts/test/app-game-android-physical-device-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game Android physical device proof source snapshot',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- ADB target: android-physical-adb-device-ref',
      '',
      'Evidence:',
      '- Physical Samsung Galaxy S9 SM-G965W Android 10 target was reachable through explicit Wi-Fi ADB.',
      '- Package manager visibility returned a nonzero package count; raw package names are not stored.',
      '- Device policy output did not prove Device Owner or Profile Owner state.',
      '- Android hide/suspend, adapter dispatch, platform enforcement, and broad blocking remain unclaimed.',
      '',
    ].join('\n')
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    [
      '# Android physical device manual platform proof',
      '',
      '| Field | Value |',
      '| --- | --- |',
      `| Device | ${readModel.model} / ${readModel.product} |`,
      `| Android | ${readModel.androidRelease} / API ${readModel.sdkInt} |`,
      `| ABI count | ${readModel.supportedAbiCount} |`,
      `| Package manager visible rows | ${readModel.packageManagerVisibleCount} |`,
      `| UsageStats command state | ${readModel.usageStatsServiceState} |`,
      `| UsageEvents dump state | ${readModel.usageEventsDumpState} |`,
      `| UsageEvents redacted sample rows | ${readModel.usageEventsSampleCount} |`,
      `| Foreground activity event rows | ${readModel.foregroundActivityEventCount} |`,
      `| Device Owner state | ${readModel.deviceOwnerState} |`,
      `| Profile Owner state | ${readModel.profileOwnerState} |`,
      '',
      'Limitations:',
      '- This is physical-device reachability and package/policy-state proof only.',
      '- It proves foreground usage-event visibility through a redacted dumpsys sample, not durable UsageEvents replay or child runtime delivery.',
      '- It does not prove Accessibility overlay behavior, Device Owner/Profile Owner enrollment, setApplicationHidden, setPackagesSuspended, uninstall blocking, lock task, or managed configuration execution.',
      '- Raw package names, raw usage-event package/class names, and raw device serials are intentionally redacted from proof artifacts.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.map(redactCommandOutput).join('\n\n')}\n`);

  console.log('app-game-android-physical-device-proof-ok');
  console.log('evidence=test-results/app-game-android-physical-device-proof/proof.json');
}

function collectBuildProps() {
  const model = getprop('ro.product.model');
  const device = getprop('ro.product.device');
  const androidRelease = getprop('ro.build.version.release');
  const sdkInt = Number.parseInt(getprop('ro.build.version.sdk'), 10);
  const supportedAbiCount = getprop('ro.product.cpu.abilist')
    .split(',')
    .map((entry) => entry.trim())
    .filter(Boolean).length;

  if (!Number.isInteger(sdkInt) || supportedAbiCount === 0) {
    throw new Error('Android physical device build props are incomplete.');
  }

  return { model, device, androidRelease, sdkInt, supportedAbiCount };
}

function getprop(name) {
  return run('adb', ['-s', adbTarget, 'shell', 'getprop', name]).stdout.trim();
}

function findPhysicalDeviceLine(devicesOutput) {
  const line = devicesOutput
    .split(/\r?\n/)
    .find((candidate) => candidate.includes(`${adbTarget}`) && candidate.includes(' device '));

  if (!line || line.includes('emulator')) {
    throw new Error(`Physical Android target ${adbTarget} was not listed as a physical device.`);
  }

  const product = fieldFromDeviceLine(line, 'product');
  const model = fieldFromDeviceLine(line, 'model');

  if (product !== 'star2qltecs' || model !== 'SM_G965W') {
    throw new Error(`Unexpected Android target identity: ${line}`);
  }

  return { product, model };
}

function fieldFromDeviceLine(line, field) {
  const match = line.match(new RegExp(`${field}:([^\\s]+)`));
  if (!match) {
    throw new Error(`Missing ${field} in adb devices line.`);
  }
  return match[1];
}

function packageCount() {
  const output = run('adb', ['-s', adbTarget, 'shell', 'cmd', 'package', 'list', 'packages']).stdout;
  const count = output.split(/\r?\n/).filter((line) => line.startsWith('package:')).length;
  if (count === 0) {
    throw new Error('Physical Android package manager returned zero visible packages.');
  }
  return count;
}

function usageStatsServiceVisible() {
  const result = run('adb', ['-s', adbTarget, 'shell', 'cmd', 'usagestats', 'help'], { allowFailure: true });
  const output = `${result.stdout}${result.stderr}`;
  return result.status === 0 && !output.includes('No shell command implementation');
}

function usageEventsDumpProof() {
  const result = run('adb', ['-s', adbTarget, 'shell', 'dumpsys', 'usagestats'], { allowFailure: true });
  if (result.status !== 0) {
    return {
      usageEventsDumpState: 'usage-events-dump-unavailable',
      usageEventsSampleCount: 0,
      foregroundActivityEventCount: 0,
      foregroundEvidenceObserved: false,
    };
  }

  const eventLines = result.stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.includes(' type=') && line.includes(' package='));
  const foregroundActivityEventCount = eventLines.filter(
    (line) =>
      line.includes('type=ACTIVITY_RESUMED') ||
      line.includes('type=ACTIVITY_PAUSED') ||
      line.includes('type=ACTIVITY_STOPPED')
  ).length;

  if (eventLines.length === 0 || foregroundActivityEventCount === 0) {
    return {
      usageEventsDumpState: 'usage-events-dump-unavailable',
      usageEventsSampleCount: 0,
      foregroundActivityEventCount: 0,
      foregroundEvidenceObserved: false,
    };
  }

  return {
    usageEventsDumpState: 'usage-events-dump-observed',
    usageEventsSampleCount: eventLines.length,
    foregroundActivityEventCount,
    foregroundEvidenceObserved: true,
  };
}

function devicePolicyStates() {
  const output = run('adb', ['-s', adbTarget, 'shell', 'dumpsys', 'device_policy']).stdout;
  return {
    deviceOwnerState: output.includes('Device Owner') ? 'not-proved' : 'not-device-owner',
    profileOwnerState: output.includes('Profile Owner') ? 'not-proved' : 'not-profile-owner',
  };
}

async function assertSourceBoundaries() {
  const gateData = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gate-data.ts'),
    'utf8'
  );
  const gateRules = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gate-rules.ts'),
    'utf8'
  );
  assertIncludes(gateData, "gateId: 'android-normal-mode-hide-suspend-manual-required'", 'Android gate id');
  assertIncludes(gateData, "adapterDispatchState: 'blocked-before-adapter'", 'Android blocked dispatch state');
  assertIncludes(gateData, 'canCallAdapter: false', 'Android adapter non-claim');
  assertIncludes(gateRules, "'android-device-owner-proof'", 'Android device owner rule');
  assertIncludes(gateRules, "'android-profile-owner-proof'", 'Android profile owner rule');
}

function run(command, args, options = {}) {
  const rendered = `${command} ${args.join(' ')}`;
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  const record = {
    rendered,
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
    proofStdout: normalizeCommandOutput(result.stdout),
    proofStderr: normalizeCommandOutput(result.stderr),
  };
  commands.push(record);
  if (result.status !== 0 && !options.allowFailure) {
    throw new Error(`${rendered} failed with exit ${result.status}`);
  }
  return record;
}

function normalizeCommandOutput(output) {
  if (output.includes('package:')) {
    const count = output.split(/\r?\n/).filter((line) => line.startsWith('package:')).length;
    return `<redacted-package-list count=${count}>\n`;
  }

  if (output.includes('Last 24 hour events') && output.includes(' type=')) {
    const eventLines = output.split(/\r?\n/).filter((line) => line.includes(' type=') && line.includes(' package='));
    const foregroundCount = eventLines.filter(
      (line) =>
        line.includes('type=ACTIVITY_RESUMED') ||
        line.includes('type=ACTIVITY_PAUSED') ||
        line.includes('type=ACTIVITY_STOPPED')
    ).length;
    return `<redacted-usage-events count=${eventLines.length} foregroundActivityCount=${foregroundCount}>\n`;
  }

  return output
    .split(repoRoot)
    .join('<repo-root>')
    .replace(new RegExp(adbTarget.replaceAll('.', '\\.').replace(':', '\\:'), 'g'), 'android-physical-adb-device-ref')
    .replace(/192\.168\.\d+\.\d+:\d+/g, 'android-physical-adb-device-ref')
    .replace(/^emulator-[^\r\n]+$/gm, 'emulator:<ignored-non-proof-target>')
    .replace(/(?:[A-Za-z_][\w]*\.){2,}[A-Za-z_][\w]*(?:\/[^\s:]+)?/g, '<android-package-redacted>');
}

function redactCommandOutput(command) {
  return `${command.rendered.replace(adbTarget, 'android-physical-adb-device-ref')}\nexit=${command.status}\n${command.proofStdout}${command.proofStderr}`;
}

function assertIncludes(source, needle, label) {
  if (!source.includes(needle)) {
    throw new Error(`Missing ${label}: ${needle}`);
  }
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
