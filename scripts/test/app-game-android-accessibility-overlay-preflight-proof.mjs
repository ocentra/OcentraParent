import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-android-accessibility-overlay-preflight-proof';
const adbTarget = process.env.OCENTRA_ANDROID_PHYSICAL_SERIAL ?? '192.168.2.45:5555';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '194-app-game-android-accessibility-overlay-preflight'
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
    'app-game-android-accessibility-overlay-preflight',
    'app-game-android-physical-device-proof',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  runAdb(['connect', adbTarget]);
  assertPhysicalDevice(runAdb(['devices', '-l']).stdout);
  const accessibilitySettings = accessibilitySettingsSample();
  const androidProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-android-physical-device-proof', 'proof.json')
  );
  const preflightModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-accessibility-overlay-preflight.js')
    ).href
  );
  const preflight = preflightModule.createAppGameAndroidAccessibilityOverlayPreflightReadModel({
    androidProof: androidProof.readModel,
    accessibilitySettings,
    generatedAt: '2026-06-08T18:20:00.000Z',
  });
  const summary = preflightModule.summarizeAppGameAndroidAccessibilityOverlayPreflightReadModel(preflight);

  assertEqual(summary.dispatchableActionCount, 0, 'Android Accessibility dispatchable action count');
  assertEqual(summary.blockedActionCount, 4, 'Android Accessibility blocked action count');
  assertEqual(preflight.rawAccessibilityServiceNamesClaimed, false, 'raw Accessibility service name claim');
  assertEqual(preflight.rawOverlayContentClaimed, false, 'raw overlay content claim');
  assertIncludes(preflight.openBlockers, 'android-overlay-runtime-not-proved', 'overlay runtime blocker');
  assertIncludes(
    preflight.openBlockers,
    'android-adapter-dispatch-blocked-before-accessibility',
    'adapter dispatch blocker'
  );

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands: commands.map(redactCommandOutput),
    accessibilitySettings,
    preflight,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-android-accessibility-overlay-preflight.ts',
      contractTest: 'packages/parent-domain/tests/app-game-android-accessibility-overlay-preflight.test.ts',
      physicalDeviceProof: 'test-results/app-game-android-physical-device-proof/proof.json',
      settingsProof:
        'adb shell settings get secure accessibility_enabled and enabled_accessibility_services were sampled; service/component names are redacted and only the enabled count is stored.',
    },
    claimsProved: [
      'Android Accessibility overlay actions have explicit preflight rows for warning, block, request, and usage-context overlays',
      'The physical Android target exposes redacted Accessibility settings evidence without raw service/component name custody',
      'Overlay actions remain blocked before adapter dispatch until an enabled service, overlay runtime proof, and child delivery proof exist',
    ],
    claimsNotProved: [
      'Android Accessibility service implementation or enablement for Ocentra',
      'Android warning, blocking, request, or usage-context overlay execution',
      'Adapter dispatch, platform enforcement, provider delivery, or child-device delivery',
      'Raw Accessibility service/component names or raw overlay content custody',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${proof.commands.join('\n\n')}\n`);

  console.log('app-game-android-accessibility-overlay-preflight-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push({ rendered: commandLine, status: 'pending', stdout: '', stderr: '' });
  await new Promise((resolve, reject) => {
    const child = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
    commands[commands.length - 1] = {
      rendered: commandLine,
      status: child.status,
      stdout: child.stdout,
      stderr: child.stderr,
    };
    if (child.status === 0) {
      process.stdout.write(child.stdout);
      process.stderr.write(child.stderr);
      resolve();
      return;
    }
    reject(new Error(`${commandLine} exited with ${child.status}`));
  });
}

function runAdb(args) {
  const result = spawnSync('adb', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push({
    rendered: ['adb', ...args].join(' '),
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
  });
  if (result.status !== 0) {
    throw new Error(`adb ${args.join(' ')} exited with ${result.status}`);
  }
  return result;
}

function assertPhysicalDevice(devicesOutput) {
  const line = devicesOutput
    .split(/\r?\n/)
    .find((candidate) => candidate.includes(adbTarget) && candidate.includes(' device '));
  if (!line || line.includes('emulator') || !line.includes('product:star2qltecs') || !line.includes('model:SM_G965W')) {
    throw new Error(`Physical Android target ${adbTarget} was not listed as Samsung Galaxy S9.`);
  }
}

function accessibilitySettingsSample() {
  const enabledValue = runAdb([
    '-s',
    adbTarget,
    'shell',
    'settings',
    'get',
    'secure',
    'accessibility_enabled',
  ]).stdout.trim();
  const servicesValue = runAdb([
    '-s',
    adbTarget,
    'shell',
    'settings',
    'get',
    'secure',
    'enabled_accessibility_services',
  ]).stdout.trim();
  const enabledServices = servicesValue === 'null' || servicesValue.length === 0 ? [] : servicesValue.split(':');

  return {
    accessibilityEnabled: enabledValue === '1',
    enabledServiceCount: enabledServices.filter((entry) => entry.trim().length > 0).length,
    serviceNamesRedacted: true,
    settingsReadable: enabledValue === '0' || enabledValue === '1',
  };
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function gitHead() {
  const result = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error('git rev-parse HEAD failed');
  }
  return result.stdout.trim();
}

function redactCommandOutput(record) {
  return [
    record.rendered.replaceAll(adbTarget, 'android-physical-adb-device-ref'),
    `exit=${record.status}`,
    normalizeOutput(record.stdout),
    normalizeOutput(record.stderr),
  ]
    .filter(Boolean)
    .join('\n');
}

function normalizeOutput(output) {
  return output
    .replaceAll(adbTarget, 'android-physical-adb-device-ref')
    .replace(/enabled_accessibility_services[^\r\n]*/giu, 'enabled_accessibility_services=<redacted>')
    .replace(/([A-Za-z0-9_.]+\/[A-Za-z0-9_.$]+)/gu, '<android-component-redacted>');
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertIncludes(values, expected, label) {
  if (!values.includes(expected)) {
    throw new Error(`${label}: expected ${expected}`);
  }
}
