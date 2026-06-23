import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-android-emulator-artifact-inventory-proof';
const resultRoot = join(repoRoot, 'test-results', proofMode);
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp08Root = join(repoRoot, 'output', 'tracking-plan-proof', '08-android-foreground-location-adapter');
const wp09Root = join(repoRoot, 'output', 'tracking-plan-proof', '09-android-background-location-and-geofence-adapter');
const wp10Root = join(repoRoot, 'output', 'tracking-plan-proof', '10-android-battery-connectivity-and-status-adapter');
const wp33Root = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const generatedAt = '2026-06-08T10:00:00.000Z';
const sourceAndroidEmulatorProofRef = 'test-results/tracking-plan-android-emulator-proof/proof.json';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await rm(proofRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(proofRoot, { recursive: true });
  await mkdir(wp08Root, { recursive: true });
  await mkdir(wp09Root, { recursive: true });
  await mkdir(wp10Root, { recursive: true });
  await mkdir(wp33Root, { recursive: true });

  run('cmd', [
    '/c',
    'npm',
    'exec',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'vitest',
    'run',
    'tests/contract/tracking-android-emulator-artifact-inventory-proof.test.ts',
  ]);

  const proof = await buildProof();
  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-android-emulator-artifact-inventory-proof-ok');
  console.log('evidence=test-results/tracking-android-emulator-artifact-inventory-proof/proof.json');
}

async function buildProof() {
  const proofModule = await importSource(
    'packages/schema-domain/src/tracking-android-emulator-artifact-inventory-proof.ts'
  );
  const androidProof = await readJson(sourceAndroidEmulatorProofRef);
  const readModel = proofModule.buildTrackingAndroidEmulatorArtifactInventoryProof(generatedAt, {
    sourceAndroidEmulatorProofRef,
    androidSdkRoot: androidProof.androidSdkRoot,
    androidProofStatus: androidProof.currentStatus,
    packageName: androidProof.package.packageName,
    activityName: androidProof.package.expectedActivity,
    deviceSerial: androidProof.device.serial,
    androidRelease: androidProof.device.androidRelease,
    androidSdk: androidProof.device.androidSdk,
    productModel: androidProof.device.productModel,
    abi: androidProof.device.abi,
    foregroundPermissionGranted: androidProof.permissionState.foregroundLocationPermissionGranted,
    backgroundPermissionGranted: androidProof.permissionState.backgroundLocationPermissionGranted,
    foregroundPermissionUxObserved: androidProof.foregroundPermissionUx.observed,
    backgroundSettingsPageObserved: androidProof.backgroundSettingsPage.observed,
    packageLaunchObserved: androidProof.runtime.activity.packageFocused,
    foregroundServiceObserved: androidProof.runtime.service.isForeground,
    localGeofenceTransitionCount: androidProof.runtime.geofenceTransitions.transitionCount,
    localGeofenceDwellCount: androidProof.runtime.geofenceTransitions.dwellCount,
    systemProximityRegistered: androidProof.runtime.geofenceTransitions.systemProximityRegistered,
    systemProximityTransitionCount: androidProof.runtime.geofenceTransitions.systemProximityTransitionCount,
    artifactRows: await artifactRows(proofModule.RequiredTrackingAndroidEmulatorArtifactRefs),
  });

  return {
    ...readModel,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    sourceAndroidEmulatorProof: {
      proofRef: sourceAndroidEmulatorProofRef,
      currentStatus: androidProof.currentStatus,
      currentProofTier: androidProof.currentProofTier,
      packageName: androidProof.package.packageName,
      versionName: androidProof.package.versionName,
      versionCode: androidProof.package.versionCode,
      deviceSerial: androidProof.device.serial,
      androidRelease: androidProof.device.androidRelease,
      androidSdk: androidProof.device.androidSdk,
      abi: androidProof.device.abi,
    },
    artifactPaths: {
      wp08: 'output/tracking-plan-proof/08-android-foreground-location-adapter/18-android-emulator-artifact-inventory-proof.json',
      wp09: 'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/18-android-emulator-artifact-inventory-proof.json',
      wp10: 'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/18-android-emulator-artifact-inventory-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/68-android-emulator-artifact-inventory-proof.json',
      evidence: 'test-results/tracking-android-emulator-artifact-inventory-proof/proof.json',
      namedProofRoot: 'output/tracking-plan-proof/tracking-android-emulator-artifact-inventory-proof/proof.json',
    },
  };
}

async function artifactRows(requiredArtifactRefs) {
  const rows = [];
  for (const artifactRef of requiredArtifactRefs) {
    const artifactStat = await stat(join(repoRoot, artifactRef)).catch(() => undefined);
    rows.push({
      artifactRef,
      category: categoryFor(artifactRef),
      required: true,
      present: artifactStat !== undefined && artifactStat.isFile(),
      byteSize: artifactStat?.size ?? 0,
    });
  }
  return rows;
}

function assertProof(proof) {
  if (proof.summary.missingArtifactCount !== 0 || !proof.summary.emulatorArtifactInventoryComplete) {
    throw new Error(`Android emulator artifact inventory has missing artifacts: ${JSON.stringify(proof.summary)}`);
  }
  if (proof.summary.permissionUiArtifactCount < 3 || proof.summary.runtimeArtifactCount < 6) {
    throw new Error(
      `Android emulator artifact inventory lost expected UI/runtime rows: ${JSON.stringify(proof.summary)}`
    );
  }
  if (proof.summary.localGeofenceTransitionCount < 1) {
    throw new Error(
      `Android emulator artifact inventory lost local geofence transition evidence: ${JSON.stringify(proof.summary)}`
    );
  }
  if (proof.summary.localGeofenceDwellCount < 1) {
    throw new Error(
      `Android emulator artifact inventory lost local geofence dwell evidence: ${JSON.stringify(proof.summary)}`
    );
  }
  if (
    proof.productClaims.androidSystemGeofenceDeliveryClaimed ||
    proof.productClaims.physicalDeviceProofClaimed ||
    proof.productClaims.productClaimReady
  ) {
    throw new Error(
      `Android emulator artifact inventory overclaimed product readiness: ${JSON.stringify(proof.productClaims)}`
    );
  }
}

async function writeArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'artifact-inventory-read-model.json'), proof.rows);
  await writeJson(join(proofRoot, 'proof.json'), proof);
  await writeJson(join(wp08Root, '18-android-emulator-artifact-inventory-proof.json'), proof);
  await writeJson(join(wp09Root, '18-android-emulator-artifact-inventory-proof.json'), proof);
  await writeJson(join(wp10Root, '18-android-emulator-artifact-inventory-proof.json'), proof);
  await writeJson(join(wp33Root, '68-android-emulator-artifact-inventory-proof.json'), proof);
  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeFile(join(proofRoot, '16-validation-commands.log'), validationLog());
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Android Emulator Artifact Inventory Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- requiredProofTier: P4_PHYSICAL_DEVICE',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    '- status: android-emulator-local-artifacts-present-physical-device-required',
    `- sourceAndroidEmulatorProofRef: ${sourceAndroidEmulatorProofRef}`,
    `- requiredArtifactCount: ${proof.summary.requiredArtifactCount}`,
    `- presentArtifactCount: ${proof.summary.presentArtifactCount}`,
    `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
    `- permissionUiArtifactCount: ${proof.summary.permissionUiArtifactCount}`,
    `- runtimeArtifactCount: ${proof.summary.runtimeArtifactCount}`,
    `- localGeofenceTransitionCount: ${proof.summary.localGeofenceTransitionCount}`,
    `- localGeofenceDwellCount: ${proof.summary.localGeofenceDwellCount}`,
    `- systemProximityTransitionCount: ${proof.summary.systemProximityTransitionCount}`,
    '- does not prove Android physical-device background behavior, Android system geofence delivery, authority enrollment, production runtime, or product readiness',
    '- proof module: packages/schema-domain/src/tracking-android-emulator-artifact-inventory-proof.ts',
    '- proof tests: packages/tracking-domain/tests/contract/tracking-android-emulator-artifact-inventory-proof.test.ts',
    '- proof harness: scripts/test/tracking-android-emulator-artifact-inventory-proof.mjs',
    '',
  ].join('\n');
}

function categoryFor(artifactRef) {
  if (artifactRef.includes('permission') || artifactRef.includes('settings-page')) return 'permission-ui';
  if (artifactRef.includes('location-evidence')) return 'location-runtime';
  if (artifactRef.includes('geofence-transition')) return 'geofence-runtime';
  if (artifactRef.includes('device-status')) return 'device-status';
  if (artifactRef.includes('validation-commands')) return 'validation-log';
  return 'adb-runtime-output';
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

async function importSource(relativePath) {
  return tsImport(pathToFileURL(join(repoRoot, relativePath)).href, import.meta.url);
}

async function readJson(path) {
  return JSON.parse(await readFile(join(repoRoot, path), 'utf8'));
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function gitOutput(args) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) return '';
  return result.stdout.trim();
}
