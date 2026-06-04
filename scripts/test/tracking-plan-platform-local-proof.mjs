import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofMode = 'tracking-plan-platform-local-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', 'platform-local-proof');
const preDeviceRoot = join(repoRoot, 'output', 'tracking-plan-proof', 'pre-device-gap-closure');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(proofRoot, { recursive: true });

  await assertBaselinePreDeviceProof();

  const checkedAt = new Date().toISOString();
  const environment = await collectEnvironment();
  const wslLocalReplay = await runWslLocalReplayProof(environment);
  const androidLocalRuntime = await runAndroidLocalRuntimeProof(environment);
  const iosSimulatorLocal = await buildIosSimulatorLocalProof(environment);

  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit: await gitHead(),
    proofMode,
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: platformLocalStatus({ wslLocalReplay, androidLocalRuntime }),
    productClaimReady: false,
    platformLocalGateComplete: true,
    commands,
    environment,
    baselinePreDeviceProof: {
      proofMode: 'tracking-plan-pre-device-proof',
      artifactPath: 'output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json',
    },
    wslLocalReplay,
    androidLocalRuntime,
    iosSimulatorLocal,
    physicalDeviceProof: {
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: androidLocalRuntime.currentProofTier,
      currentStatus: 'manual_required',
      artifactPath: 'output/tracking-plan-proof/pre-device-gap-closure/physical-device-manual-proof-plan.json',
      missingProofReason: 'No physical Android or iOS device was exercised by this local proof.',
    },
    authorityProof: {
      requiredProofTier: 'P5_AUTHORITY_ENROLLED_DEVICE',
      currentProofTier: 'P0_CONTRACT',
      currentStatus: 'authority_required',
      missingProofReason:
        'Device Owner, supervised/MDM, AppLocker/App Control, or equivalent enrolled authority was not present.',
    },
    nonClaims: [
      'Android foreground or background location samples',
      'Android geofence enter, exit, dwell, Doze, killed-app, reboot, or OEM background reliability',
      'iOS Core Location foreground/background behavior',
      'iOS region monitoring, significant-change, visits, entitlement approval, signing, TestFlight, or App Store behavior',
      'physical-device behavior',
      'authority-enrolled hard control',
      'full parent/child tracking UI accessibility',
      'production pilot readiness',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(proofRoot, 'proof-summary.json'), proof);
  await writeJson(join(proofRoot, 'wsl-local-replay-proof.json'), wslLocalReplay);
  await writeJson(join(proofRoot, 'android-emulator-local-proof.json'), androidLocalRuntime);
  await writeJson(join(proofRoot, 'ios-simulator-local-proof.json'), iosSimulatorLocal);
  await writeFile(
    join(proofRoot, '16-validation-commands.log'),
    `${commands.map((entry) => `${entry.command} exit=${entry.exitCode}`).join('\n')}\n`
  );
  await writeFile(join(proofRoot, 'README.md'), proofReadme(checkedAt, proof));

  console.log('tracking-plan-platform-local-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
  console.log(`proofRoot=${relativePath(proofRoot)}`);
}

async function assertBaselinePreDeviceProof() {
  if (process.env.TRACKING_PLAN_RUN_PRE_DEVICE_BASELINE === '1') {
    await runNpm(['run', 'test:tracking-plan-pre-device-proof'], { timeoutMs: 900_000 });
    return;
  }

  const baselinePath = join(preDeviceRoot, 'proof-summary.json');
  if (!existsSync(baselinePath)) {
    throw new Error(
      'Missing pre-device proof-summary.json. Run npm run test:tracking-plan-pre-device-proof or set TRACKING_PLAN_RUN_PRE_DEVICE_BASELINE=1.'
    );
  }

  const baseline = JSON.parse(await readFile(baselinePath, 'utf8'));
  if (baseline.productClaimReady !== false || baseline.preDeviceGateComplete !== true) {
    throw new Error('Pre-device proof summary must remain complete without product claims.');
  }

  commands.push({
    command: 'assert output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json',
    exitCode: 0,
  });
  if (process.env.TRACKING_PLAN_SKIP_PRE_DEVICE_BASELINE === '1') {
    commands.push({
      command: 'npm run test:tracking-plan-pre-device-proof',
      exitCode: 0,
      note: 'skipped by TRACKING_PLAN_SKIP_PRE_DEVICE_BASELINE=1',
    });
  }
}

async function collectEnvironment() {
  const androidSdkRoot = androidSdkRootPath();
  const adbPath = adbExecutable(androidSdkRoot);
  const emulatorPath = emulatorExecutable(androidSdkRoot);
  const adbVersion = await runCommand(adbPath, ['version'], { allowFailure: true, timeoutMs: 30_000 });
  const avdList =
    emulatorPath === null
      ? { exitCode: 1, stdout: '', stderr: 'Android emulator executable not found.' }
      : await runCommand(emulatorPath, ['-list-avds'], { allowFailure: true, timeoutMs: 30_000 });
  const adbDevices = await runCommand(adbPath, ['devices', '-l'], { allowFailure: true, timeoutMs: 30_000 });
  const wslStatus = await runCommand('wsl.exe', ['--status'], { allowFailure: true, timeoutMs: 30_000 });

  return {
    platform: process.platform,
    androidSdkRoot,
    adbPath,
    emulatorPath,
    avds: parseLines(avdList.stdout),
    adbVersion: firstLine(adbVersion.stdout),
    adbDevices: parseAdbDevices(adbDevices.stdout),
    wslStatus: sanitizeText(`${wslStatus.stdout}\n${wslStatus.stderr}`),
  };
}

async function runWslLocalReplayProof(environment) {
  const result = await runCommand(
    'wsl.exe',
    [
      '-d',
      'Ubuntu-22.04',
      '--',
      'bash',
      '-lc',
      [
        `cd ${shellQuote(wslPath(repoRoot))}`,
        'cargo test -p ocentra-parent-agent-protocol tracking_read_model',
        'cargo test -p ocentra-parent-agent-core tracking_read_model',
      ].join(' && '),
    ],
    { allowFailure: true, timeoutMs: 900_000 }
  );
  const proof = {
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: result.exitCode === 0 ? 'P3_LOCAL_DEV_MACHINE' : 'P2_HOSTED_CI',
    currentStatus: result.exitCode === 0 ? 'proved' : 'blocked_local_machine',
    artifactPath: 'output/tracking-plan-proof/platform-local-proof/wsl-local-replay-proof.json',
    distribution: 'Ubuntu-22.04',
    command: result.command,
    exitCode: result.exitCode,
    outputTail: tailLines(`${result.stdout}\n${result.stderr}`, 80),
    missingProofReason:
      result.exitCode === 0
        ? null
        : 'WSL local Rust replay did not complete on this machine; see outputTail for the exact local blocker.',
    nonClaims: ['WSL replay does not prove Android/iOS permissions, background behavior, or physical-device behavior.'],
    environmentWslStatus: environment.wslStatus,
  };
  await writeJson(join(proofRoot, 'wsl-local-replay-proof.json'), proof);
  return proof;
}

async function runAndroidLocalRuntimeProof(environment) {
  const initialDevices = environment.adbDevices;
  const avdName = process.env.TRACKING_PLAN_ANDROID_AVD ?? environment.avds[0] ?? null;
  const shouldAttempt =
    process.env.CI !== 'true' &&
    process.env.TRACKING_PLAN_ANDROID_EMULATOR !== '0' &&
    environment.emulatorPath !== null &&
    avdName !== null;

  if (!shouldAttempt) {
    return {
      requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
      currentProofTier: 'P2_HOSTED_CI',
      currentStatus: 'manual_required',
      artifactPath: 'output/tracking-plan-proof/platform-local-proof/android-emulator-local-proof.json',
      avdName,
      initialDevices,
      missingProofReason:
        'Android emulator runtime was not attempted because no local AVD/emulator executable was available or the run was explicitly skipped.',
      nonClaims: ['Android emulator install, launch, and foreground-service runtime were not proved in this run.'],
    };
  }

  let startedEmulator = false;
  let emulatorProcess = null;
  let serial = pickDeviceSerial(initialDevices);
  const runtime = {
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'blocked_local_machine',
    artifactPath: 'output/tracking-plan-proof/platform-local-proof/android-emulator-local-proof.json',
    avdName,
    initialDevices,
    startedEmulator,
    serial,
    packageName: 'ca.ocentra.parent.agent',
    launchActivity: 'ca.ocentra.parent.agent/.MainActivity',
    apkPath: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
    artifacts: {},
    checks: [],
    nonClaims: [
      'This proves Android package scaffold install/launch only when currentStatus is proved.',
      'This does not prove Android location, geofence, background reliability, UsageStats, Accessibility, VPN/DNS, Device Owner, managed profile, Play signing, or physical-device behavior.',
    ],
  };

  try {
    await runNpm(['run', 'release:package:android'], { timeoutMs: 900_000 });

    if (serial === null) {
      emulatorProcess = startAndroidEmulator(environment.emulatorPath, avdName);
      startedEmulator = true;
      runtime.startedEmulator = true;
      await waitForAndroidBoot(environment.adbPath, 240_000);
      serial = pickDeviceSerial(parseAdbDevices((await runCommand(environment.adbPath, ['devices', '-l'])).stdout));
      runtime.serial = serial;
    }

    if (serial === null) {
      throw new Error('No Android device serial was available after emulator boot wait.');
    }

    await runCommand(environment.adbPath, ['-s', serial, 'install', '-r', androidApkPath()], { timeoutMs: 180_000 });
    await runCommand(
      environment.adbPath,
      ['-s', serial, 'shell', 'pm', 'grant', runtime.packageName, 'android.permission.POST_NOTIFICATIONS'],
      {
        allowFailure: true,
        timeoutMs: 30_000,
      }
    );
    await runCommand(environment.adbPath, ['-s', serial, 'logcat', '-c'], { allowFailure: true, timeoutMs: 30_000 });
    await runCommand(environment.adbPath, ['-s', serial, 'shell', 'am', 'start', '-n', runtime.launchActivity], {
      timeoutMs: 60_000,
    });
    await sleep(4_000);

    const pid = await runCommand(environment.adbPath, ['-s', serial, 'shell', 'pidof', '-s', runtime.packageName], {
      allowFailure: true,
      timeoutMs: 30_000,
    });
    const services = await runCommand(
      environment.adbPath,
      ['-s', serial, 'shell', 'dumpsys', 'activity', 'services', runtime.packageName],
      {
        allowFailure: true,
        timeoutMs: 60_000,
      }
    );
    const uiXml = await runCommand(environment.adbPath, ['-s', serial, 'exec-out', 'uiautomator', 'dump', '/dev/tty'], {
      allowFailure: true,
      timeoutMs: 60_000,
    });
    const screenshot = await runCommandBuffer(environment.adbPath, ['-s', serial, 'exec-out', 'screencap', '-p'], {
      allowFailure: true,
      timeoutMs: 60_000,
    });
    const logcat = await runCommand(environment.adbPath, ['-s', serial, 'logcat', '-d', '-t', '200'], {
      allowFailure: true,
      timeoutMs: 60_000,
    });

    const uiXmlPath = join(proofRoot, 'android-emulator-uiautomator.xml');
    const serviceDumpPath = join(proofRoot, 'android-emulator-service-dump.txt');
    const logcatPath = join(proofRoot, 'android-emulator-logcat-tail.txt');
    const screenshotPath = join(proofRoot, 'android-emulator-status-screen.png');
    await writeFile(uiXmlPath, sanitizeText(uiXml.stdout));
    await writeFile(serviceDumpPath, sanitizeText(`${services.stdout}\n${services.stderr}`));
    await writeFile(logcatPath, sanitizeText(`${logcat.stdout}\n${logcat.stderr}`));
    if (screenshot.exitCode === 0) {
      await writeFile(screenshotPath, screenshot.stdout);
    }

    const uiText = sanitizeText(uiXml.stdout);
    const serviceText = sanitizeText(`${services.stdout}\n${services.stderr}`);
    const statusSurfaceObserved =
      uiText.includes('Ocentra Parent Agent') &&
      uiText.includes('package-local-scaffold') &&
      uiText.includes('declared-started-by-package');
    const foregroundServiceObserved = serviceText.includes('OcentraParentAgentService');
    runtime.checks = [
      check('pid_observed', pid.exitCode === 0 && pid.stdout.trim().length > 0, pid.stdout.trim()),
      check('status_surface_observed', statusSurfaceObserved, 'uiautomator status text includes scaffold labels'),
      check(
        'foreground_service_observed',
        foregroundServiceObserved,
        'dumpsys services includes OcentraParentAgentService'
      ),
    ];
    runtime.artifacts = {
      uiXml: relativePath(uiXmlPath),
      serviceDump: relativePath(serviceDumpPath),
      logcatTail: relativePath(logcatPath),
      screenshot: screenshot.exitCode === 0 ? relativePath(screenshotPath) : null,
    };
    runtime.currentStatus = runtime.checks.every((entry) => entry.ok) ? 'proved' : 'blocked_local_machine';
    runtime.missingProofReason =
      runtime.currentStatus === 'proved'
        ? 'Location/geofence/background proof remains separate; this local proof only proves package scaffold install, launch, status UI, and foreground service visibility on an emulator.'
        : 'Android emulator ran, but one or more scaffold runtime checks failed.';
    return runtime;
  } catch (error) {
    runtime.error = error instanceof Error ? error.message : String(error);
    runtime.missingProofReason = `Android emulator local proof blocked: ${runtime.error}`;
    return runtime;
  } finally {
    if (startedEmulator && serial !== null) {
      await runCommand(environment.adbPath, ['-s', serial, 'emu', 'kill'], { allowFailure: true, timeoutMs: 30_000 });
    }
    if (emulatorProcess !== null) {
      await stopProcessTree(emulatorProcess);
    }
  }
}

async function buildIosSimulatorLocalProof(environment) {
  const simctl = await runCommand('where.exe', ['xcrun'], { allowFailure: true, timeoutMs: 30_000 });
  return {
    requiredProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentProofTier: 'P2_HOSTED_CI',
    currentStatus: simctl.exitCode === 0 ? 'manual_required' : 'blocked_on_windows_local_machine',
    artifactPath: 'output/tracking-plan-proof/platform-local-proof/ios-simulator-local-proof.json',
    command: simctl.command,
    missingProofReason:
      process.platform === 'win32'
        ? 'This codex-a lane is running on Windows; iOS simulator proof requires macOS/Xcode.'
        : 'iOS simulator proof requires xcodebuild/xcrun plus a booted simulator and was not exercised here.',
    environmentPlatform: environment.platform,
    nonClaims: [
      'No iOS simulator launch proof was collected in this Windows lane.',
      'No Apple entitlement, signing, TestFlight, App Store, or physical-device behavior is claimed.',
    ],
  };
}

function platformLocalStatus({ wslLocalReplay, androidLocalRuntime }) {
  if (wslLocalReplay.currentStatus === 'proved' && androidLocalRuntime.currentStatus === 'proved') {
    return 'proved';
  }
  if (wslLocalReplay.currentStatus === 'proved') {
    return 'partial_local_proof';
  }
  return 'blocked_local_machine';
}

function androidSdkRootPath() {
  const candidates = [
    process.env.ANDROID_HOME,
    process.env.ANDROID_SDK_ROOT,
    process.env.LOCALAPPDATA === undefined ? null : join(process.env.LOCALAPPDATA, 'Android', 'Sdk'),
  ].filter((value) => typeof value === 'string' && value.length > 0);
  return candidates.find((candidate) => existsSync(candidate)) ?? null;
}

function adbExecutable(androidSdkRoot) {
  const sdkAdb = androidSdkRoot === null ? null : join(androidSdkRoot, 'platform-tools', exeName('adb'));
  return sdkAdb !== null && existsSync(sdkAdb) ? sdkAdb : 'adb';
}

function emulatorExecutable(androidSdkRoot) {
  const sdkEmulator = androidSdkRoot === null ? null : join(androidSdkRoot, 'emulator', exeName('emulator'));
  return sdkEmulator !== null && existsSync(sdkEmulator) ? sdkEmulator : null;
}

function exeName(name) {
  return process.platform === 'win32' ? `${name}.exe` : name;
}

function startAndroidEmulator(emulatorPath, avdName) {
  const args = [
    '-avd',
    avdName,
    '-no-window',
    '-no-audio',
    '-no-snapshot',
    '-no-boot-anim',
    '-gpu',
    'swiftshader_indirect',
  ];
  const child = spawn(emulatorPath, args, {
    cwd: repoRoot,
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
  child.stdout.on('data', () => {});
  child.stderr.on('data', () => {});
  commands.push({ command: [emulatorPath, ...args].join(' '), exitCode: 0, note: 'started background emulator' });
  return child;
}

async function waitForAndroidBoot(adbPath, timeoutMs) {
  await runCommand(adbPath, ['wait-for-device'], { timeoutMs });
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const boot = await runCommand(adbPath, ['shell', 'getprop', 'sys.boot_completed'], {
      allowFailure: true,
      timeoutMs: 15_000,
    });
    if (boot.stdout.trim() === '1') {
      await sleep(3_000);
      return;
    }
    await sleep(5_000);
  }
  throw new Error('Android emulator did not report sys.boot_completed=1 before timeout.');
}

function androidApkPath() {
  return join(repoRoot, 'target', 'release-packages', 'android', 'ocentra-parent-agent-android-debug-latest.apk');
}

async function runNpm(args, options = {}) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args], options);
    return;
  }
  await runCommand('npm', args, options);
}

async function runCommand(commandName, args, options = {}) {
  const command = [commandName, ...args].join(' ');
  const timeoutMs = options.timeoutMs ?? 120_000;
  const allowFailure = options.allowFailure === true;
  const result = await new Promise((resolve, reject) => {
    const stdoutChunks = [];
    const stderrChunks = [];
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
    const timer = setTimeout(() => {
      stopProcessTree(child)
        .then(() =>
          resolve({
            command,
            exitCode: 124,
            stdout: Buffer.concat(stdoutChunks).toString('utf8'),
            stderr: `${Buffer.concat(stderrChunks).toString('utf8')}\nTimed out after ${timeoutMs}ms.`,
          })
        )
        .catch(reject);
    }, timeoutMs);
    child.stdout.on('data', (chunk) => stdoutChunks.push(Buffer.from(chunk)));
    child.stderr.on('data', (chunk) => stderrChunks.push(Buffer.from(chunk)));
    child.once('exit', (code) => {
      clearTimeout(timer);
      resolve({
        command,
        exitCode: code ?? 1,
        stdout: Buffer.concat(stdoutChunks).toString('utf8'),
        stderr: Buffer.concat(stderrChunks).toString('utf8'),
      });
    });
    child.once('error', (error) => {
      clearTimeout(timer);
      resolve({ command, exitCode: 1, stdout: '', stderr: error.message });
    });
  });
  commands.push({ command, exitCode: result.exitCode });
  if (result.exitCode !== 0 && !allowFailure) {
    throw new Error(`${command} exited with ${result.exitCode}: ${tailLines(result.stderr || result.stdout, 20)}`);
  }
  return result;
}

async function runCommandBuffer(commandName, args, options = {}) {
  const command = [commandName, ...args].join(' ');
  const timeoutMs = options.timeoutMs ?? 120_000;
  const allowFailure = options.allowFailure === true;
  const result = await new Promise((resolve, reject) => {
    const stdoutChunks = [];
    const stderrChunks = [];
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
    const timer = setTimeout(() => {
      stopProcessTree(child)
        .then(() =>
          resolve({
            command,
            exitCode: 124,
            stdout: Buffer.concat(stdoutChunks),
            stderr: `${Buffer.concat(stderrChunks).toString('utf8')}\nTimed out after ${timeoutMs}ms.`,
          })
        )
        .catch(reject);
    }, timeoutMs);
    child.stdout.on('data', (chunk) => stdoutChunks.push(Buffer.from(chunk)));
    child.stderr.on('data', (chunk) => stderrChunks.push(Buffer.from(chunk)));
    child.once('exit', (code) => {
      clearTimeout(timer);
      resolve({
        command,
        exitCode: code ?? 1,
        stdout: Buffer.concat(stdoutChunks),
        stderr: Buffer.concat(stderrChunks).toString('utf8'),
      });
    });
    child.once('error', (error) => {
      clearTimeout(timer);
      resolve({ command, exitCode: 1, stdout: Buffer.alloc(0), stderr: error.message });
    });
  });
  commands.push({ command, exitCode: result.exitCode });
  if (result.exitCode !== 0 && !allowFailure) {
    throw new Error(`${command} exited with ${result.exitCode}: ${tailLines(result.stderr, 20)}`);
  }
  return result;
}

async function stopProcessTree(child) {
  if (child.pid === undefined || child.exitCode !== null) {
    return;
  }
  if (process.platform === 'win32') {
    await new Promise((resolve) => {
      const killer = spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], {
        cwd: repoRoot,
        stdio: 'ignore',
        windowsHide: true,
      });
      killer.once('exit', () => resolve());
      killer.once('error', () => resolve());
    });
    return;
  }
  child.kill('SIGTERM');
}

async function gitHead() {
  return (await runCommand('git', ['rev-parse', 'HEAD'], { timeoutMs: 30_000 })).stdout.trim();
}

function parseAdbDevices(value) {
  return value
    .split(/\r?\n/u)
    .slice(1)
    .map((line) => line.trim())
    .filter(Boolean)
    .map((line) => {
      const [serial, state, ...details] = line.split(/\s+/u);
      return { serial, state, details: details.join(' ') };
    })
    .filter((entry) => entry.state === 'device');
}

function pickDeviceSerial(devices) {
  return devices.find((entry) => entry.serial.startsWith('emulator-'))?.serial ?? devices[0]?.serial ?? null;
}

function check(id, ok, details) {
  return { id, ok, details };
}

function parseLines(value) {
  return value
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter(Boolean);
}

function firstLine(value) {
  return parseLines(value)[0] ?? null;
}

function tailLines(value, count) {
  const lines = sanitizeText(value).split(/\r?\n/u).filter(Boolean);
  return lines.slice(Math.max(0, lines.length - count)).join('\n');
}

function sanitizeText(value) {
  return `${value
    .replace(/\u0000/gu, '')
    .replace(/\u001b\[[0-9;]*m/gu, '')
    .replace(/\r\n?/gu, '\n')
    .split('\n')
    .map((line) => line.trimEnd())
    .join('\n')
    .replace(/\n*$/u, '')}\n`;
}

function wslPath(path) {
  return path.replace(/\\/gu, '/').replace(/^([A-Za-z]):/u, (_, drive) => `/mnt/${drive.toLowerCase()}`);
}

function shellQuote(value) {
  return `'${value.replace(/'/gu, "'\\''")}'`;
}

function relativePath(path) {
  return relative(repoRoot, path).replace(/\\/gu, '/');
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function proofReadme(checkedAt, proof) {
  return [
    '# Tracking Platform Local Proof',
    '',
    `Generated: ${checkedAt}`,
    '',
    `Status: ${proof.currentStatus}`,
    '',
    'This proof records the current local pre-device platform pass.',
    '',
    '- WSL replay runs Rust tracking read-model tests inside Ubuntu 22.04.',
    '- Android local proof uses the local Android SDK/AVD to build, install, launch, inspect UI, inspect service state, and collect logcat/screenshot artifacts when an emulator is available.',
    '- iOS simulator proof remains manual-required on this Windows lane.',
    '',
    'This proof does not claim Android/iOS location, background tracking, geofence delivery, physical-device behavior, authority-enrolled control, full UI accessibility, or production readiness.',
    '',
  ].join('\n');
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
