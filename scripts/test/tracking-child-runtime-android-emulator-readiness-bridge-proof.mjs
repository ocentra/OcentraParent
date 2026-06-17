import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-child-runtime-android-emulator-readiness-bridge-proof';
const resultRoot = join(repoRoot, 'test-results', proofMode);
const namedProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output08 = join(repoRoot, 'output', 'tracking-plan-proof', '08-android-foreground-location-adapter');
const output30 = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const androidProofRef = 'test-results/tracking-plan-android-emulator-proof/proof.json';
const childRuntimeArtifactGateProofRef =
  'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json';
const generatedAt = '2026-06-08T05:20:00.000Z';
const commands = [];

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });
  await mkdir(output08, { recursive: true });
  await mkdir(output30, { recursive: true });
  await mkdir(output33, { recursive: true });

  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-child-runtime-android-emulator-readiness-bridge-proof.test.ts',
  ]);

  const androidProof = await readJson(androidProofRef);
  const childRuntimeArtifactGateProof = await readJson(childRuntimeArtifactGateProofRef);
  const proofModule = await tsImport(
    pathToFileURL(
      join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-child-runtime-android-emulator-readiness-bridge-proof.ts')
    ).href,
    import.meta.url
  );
  const proof = buildProof({
    proofModule,
    androidProof,
    childRuntimeArtifactGateProof,
  });

  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-child-runtime-android-emulator-readiness-bridge-proof-ok');
  console.log('evidence=test-results/tracking-child-runtime-android-emulator-readiness-bridge-proof/proof.json');
}

function buildProof({ proofModule, androidProof, childRuntimeArtifactGateProof }) {
  const childRuntimeRequiredArtifacts = unique(
    childRuntimeArtifactGateProof.readModel.rows.flatMap((row) => row.requiredArtifacts)
  );
  const childRuntimePresentArtifacts = unique(
    childRuntimeArtifactGateProof.readModel.rows.flatMap((row) => row.presentArtifacts)
  );
  const childRuntimeMissingArtifacts = unique(
    childRuntimeArtifactGateProof.readModel.rows.flatMap((row) => row.missingArtifacts)
  );
  const androidEvidenceRefs = [
    androidProofRef,
    'output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json',
    'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/04-background-geofence-evidence.json',
    'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/02-android-status-proof.json',
  ];

  const readModel = proofModule.buildTrackingChildRuntimeAndroidEmulatorBridgeProof(generatedAt, {
    androidEmulatorProofRef: androidProofRef,
    childRuntimeArtifactGateProofRef,
    androidProofStatus: androidProof.currentStatus,
    packageLaunchObserved: androidProof.runtime.activity.packageFocused,
    foregroundServiceObserved: androidProof.runtime.service.isForeground,
    foregroundPermissionGranted: androidProof.permissionState.foregroundLocationPermissionGranted,
    backgroundPermissionGranted: androidProof.permissionState.backgroundLocationPermissionGranted,
    localGeofenceTransitionCount: androidProof.runtime.geofenceTransitions.transitionCount,
    androidEvidenceRefs,
    childRuntimeRequiredArtifacts,
    childRuntimePresentArtifacts,
    childRuntimeMissingArtifacts,
  });

  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackIds: [
      '08-android-foreground-location-adapter',
      '30-parent-and-child-ui-ux-surfaces',
      '33-proof-gates-fixtures-rollout-and-pr-gate',
    ],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'emulator_prerequisites_observed_manual_runtime_required',
    sourceProofRefs: [androidProofRef, childRuntimeArtifactGateProofRef],
    readModel,
    summary: {
      androidProofStatus: androidProof.currentStatus,
      packageLaunchObserved: androidProof.runtime.activity.packageFocused,
      foregroundServiceObserved: androidProof.runtime.service.isForeground,
      foregroundPermissionGranted: androidProof.permissionState.foregroundLocationPermissionGranted,
      backgroundPermissionGranted: androidProof.permissionState.backgroundLocationPermissionGranted,
      localGeofenceTransitionCount: androidProof.runtime.geofenceTransitions.transitionCount,
      childRuntimeRequiredArtifactCount: childRuntimeRequiredArtifacts.length,
      childRuntimePresentArtifactCount: childRuntimePresentArtifacts.length,
      childRuntimeMissingArtifactCount: childRuntimeMissingArtifacts.length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    productClaims: readModel.productClaims,
    missingProofReason:
      'The Android emulator proof confirms package launch, foreground service, permissions, and local emulator location/geofence evidence, but it is not a child-device runtime execution proof. Child-runtime delivery envelope, execution result, rendered child UI snapshot, parent receipt, runtime observation, consent state, and device log artifacts remain manual-required P4 physical-device evidence.',
    commands,
  };
}

function assertProof(proof) {
  if (!proof.summary.packageLaunchObserved || !proof.summary.foregroundServiceObserved) {
    throw new Error(`Android emulator prerequisite evidence is missing: ${JSON.stringify(proof.summary)}`);
  }
  if (proof.summary.childRuntimeMissingArtifactCount === 0) {
    throw new Error('Child-runtime artifact gate unexpectedly has no missing artifacts');
  }
  if (
    proof.productClaims.childRuntimeArtifactSetComplete ||
    proof.productClaims.childDeviceDeliveryRuntimeClaimed ||
    proof.productClaims.childDeviceExecutionRuntimeClaimed ||
    proof.productClaims.renderedChildDeviceUiRuntimeClaimed ||
    proof.productClaims.productClaimReady
  ) {
    throw new Error(`Bridge proof overclaimed child runtime/product readiness: ${JSON.stringify(proof.productClaims)}`);
  }
}

async function writeProofArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'read-model.json'), proof.readModel);
  await writeJson(join(namedProofRoot, 'proof.json'), proof);
  await writeJson(join(namedProofRoot, 'read-model.json'), proof.readModel);
  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeJson(join(output08, '10-child-runtime-android-emulator-readiness-bridge-proof.json'), proof);
  await writeJson(join(output30, '34-child-runtime-android-emulator-readiness-bridge-proof.json'), proof);
  await writeJson(join(output33, '67-child-runtime-android-emulator-readiness-bridge-proof.json'), proof);
  await writeFile(
    join(output33, '67-child-runtime-android-emulator-readiness-bridge-validation-commands.log'),
    validationLog()
  );
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Child Runtime Android Emulator Readiness Bridge Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.commit}`,
    '- requiredProofTier: P4_PHYSICAL_DEVICE',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    '- status: emulator prerequisites observed, child runtime manual-required',
    '- source proof: test-results/tracking-plan-android-emulator-proof/proof.json',
    '- source proof: output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json',
    '- does not prove child-device delivery/execution runtime, rendered child-device UI, parent receipt runtime, physical-device behavior, authority, provider delivery, production, or product readiness',
    '',
  ].join('\n');
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
}

function unique(values) {
  return [...new Set(values)];
}

async function readJson(relativePath) {
  return JSON.parse(await readFile(join(repoRoot, relativePath), 'utf8'));
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

function gitOutput(args) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) return '';
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
