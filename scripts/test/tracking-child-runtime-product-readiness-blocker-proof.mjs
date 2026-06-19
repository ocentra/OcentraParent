import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-child-runtime-product-readiness-blocker-proof';
const output30 = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const namedProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const resultRoot = join(repoRoot, 'test-results', proofMode);
const sourceSnapshotRequirementsProofRef =
  'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/28-child-runtime-snapshot-requirements-proof.json';
const sourceAndroidEmulatorBridgeProofRef =
  'output/tracking-plan-proof/tracking-child-runtime-android-emulator-readiness-bridge-proof/proof.json';
const sourceParentChildLocalRuntimeBridgeProofRef =
  'output/tracking-plan-proof/tracking-parent-child-local-runtime-bridge-proof/proof.json';
const sourceSnapshotRequirementsProofPath = join(repoRoot, sourceSnapshotRequirementsProofRef);
const sourceAndroidEmulatorBridgeProofPath = join(repoRoot, sourceAndroidEmulatorBridgeProofRef);
const sourceParentChildLocalRuntimeBridgeProofPath = join(repoRoot, sourceParentChildLocalRuntimeBridgeProofRef);
const generatedAt = '2026-06-07T16:05:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output30, { recursive: true });
  await mkdir(output33, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });

  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-child-runtime-product-readiness-blocker-proof.test.ts',
  ]);

  const sourceSnapshotRequirementsProof = JSON.parse(await readFile(sourceSnapshotRequirementsProofPath, 'utf8'));
  const sourceAndroidEmulatorBridgeProof = JSON.parse(await readFile(sourceAndroidEmulatorBridgeProofPath, 'utf8'));
  const sourceParentChildLocalRuntimeBridgeProof = JSON.parse(
    await readFile(sourceParentChildLocalRuntimeBridgeProofPath, 'utf8')
  );
  const proof = await buildProof(
    sourceSnapshotRequirementsProof,
    sourceAndroidEmulatorBridgeProof,
    sourceParentChildLocalRuntimeBridgeProof
  );
  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-child-runtime-product-readiness-blocker-proof-ok');
  console.log('evidence=test-results/tracking-child-runtime-product-readiness-blocker-proof/proof.json');
}

async function buildProof(
  sourceSnapshotRequirementsProof,
  sourceAndroidEmulatorBridgeProof,
  sourceParentChildLocalRuntimeBridgeProof
) {
  const proofModule = await tsImport(
    pathToFileURL(
      join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-child-runtime-product-readiness-blocker-proof.ts')
    ).href,
    import.meta.url
  );
  return {
    ...proofModule.buildTrackingChildRuntimeProductReadinessBlockerProof(
      generatedAt,
      sourceSnapshotRequirementsProofRef,
      sourceSnapshotRequirementsProof,
      sourceAndroidEmulatorBridgeProofRef,
      sourceAndroidEmulatorBridgeProof,
      sourceParentChildLocalRuntimeBridgeProofRef,
      sourceParentChildLocalRuntimeBridgeProof
    ),
    androidEmulatorBridgeAccounting: androidEmulatorBridgeAccountingFrom(sourceAndroidEmulatorBridgeProof),
    parentChildLocalRuntimeAccounting: parentChildLocalRuntimeAccountingFrom(sourceParentChildLocalRuntimeBridgeProof),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp30: 'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/29-child-runtime-product-readiness-blocker-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/45-child-runtime-product-readiness-blocker-proof.json',
      evidence: 'test-results/tracking-child-runtime-product-readiness-blocker-proof/proof.json',
      sourceSnapshotRequirementsProof: sourceSnapshotRequirementsProofRef,
      sourceAndroidEmulatorBridgeProof: sourceAndroidEmulatorBridgeProofRef,
      sourceParentChildLocalRuntimeBridgeProof: sourceParentChildLocalRuntimeBridgeProofRef,
    },
  };
}

function assertProof(proof) {
  if (proof.rows.length === 0 || !proof.proofClaims.productReadinessBlocked) {
    throw new Error(`Child runtime product-readiness blocker proof is empty: ${JSON.stringify(proof)}`);
  }
  for (const row of proof.rows) {
    if (row.childDeviceDeliveryRuntimeClaimed || row.childDeviceExecutionRuntimeClaimed || row.productReadyClaimed) {
      throw new Error(`Child runtime product readiness was overclaimed: ${JSON.stringify(row)}`);
    }
    if (
      row.executionResultRequirementRefCount <= 0 ||
      row.visibleSnapshotRequirementRefCount <= 0 ||
      row.parentReceiptRequirementRefCount <= 0 ||
      row.runtimeObservationRequirementRefCount <= 0 ||
      row.androidLocalGeofenceTransitionCount <= 0 ||
      row.childRuntimeRequiredArtifactCount <= 0 ||
      row.childRuntimeMissingArtifactCount <= 0 ||
      row.childRuntimeRequiredArtifactCount !==
        row.childRuntimePresentArtifactCount + row.childRuntimeMissingArtifactCount ||
      row.androidEmulatorChildRuntimeMissingArtifactCount <= 0
    ) {
      throw new Error(`Child runtime requirement refs are incomplete: ${JSON.stringify(row)}`);
    }
    if (!row.androidEmulatorPrerequisitesObserved || !row.androidPackageLaunchObserved) {
      throw new Error(`Android emulator bridge prerequisites are missing: ${JSON.stringify(row)}`);
    }
  }
  if (
    !proof.androidEmulatorBridgeAccounting.androidEmulatorPrerequisitesObserved ||
    proof.androidEmulatorBridgeAccounting.childRuntimeMissingArtifactCount <= 0
  ) {
    throw new Error(
      `Android emulator bridge accounting is incomplete: ${JSON.stringify(proof.androidEmulatorBridgeAccounting)}`
    );
  }
  if (
    !proof.parentChildLocalRuntimeAccounting.localParentChildRuntimeObserved ||
    !proof.parentChildLocalRuntimeAccounting.typedLocalServiceTransportObserved ||
    !proof.parentChildLocalRuntimeAccounting.parentReadModelProjectionObserved ||
    proof.parentChildLocalRuntimeAccounting.deadLetterCount !== 0 ||
    proof.parentChildLocalRuntimeAccounting.storedEventCount < 9 ||
    proof.parentChildLocalRuntimeAccounting.childAgentPhaseCount < 4
  ) {
    throw new Error(
      `Parent-child local runtime bridge accounting is incomplete: ${JSON.stringify(
        proof.parentChildLocalRuntimeAccounting
      )}`
    );
  }
}

async function writeProofArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'child-runtime-product-readiness-blocker-read-model.json'), proof.rows);
  await writeJson(join(output30, '29-child-runtime-product-readiness-blocker-proof.json'), proof);
  await writeJson(join(output33, '45-child-runtime-product-readiness-blocker-proof.json'), proof);
  await writeJson(join(namedProofRoot, 'proof.json'), proof);
  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeFile(join(namedProofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(join(namedProofRoot, '16-validation-commands.log'), validationLog());
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Child Runtime Product Readiness Blocker Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- requiredProofTier: P2_HOSTED_CI',
    '- currentProofTier: P2_HOSTED_CI',
    '- status: proved',
    `- consumes: ${sourceSnapshotRequirementsProofRef}`,
    `- consumes: ${sourceAndroidEmulatorBridgeProofRef}`,
    `- consumes: ${sourceParentChildLocalRuntimeBridgeProofRef}`,
    '- proves child runtime requirement coverage is still product-readiness blocked even with Android emulator prerequisites and local parent-child runtime observed',
    `- androidEmulatorPrerequisitesObserved: ${proof.androidEmulatorBridgeAccounting.androidEmulatorPrerequisitesObserved}`,
    `- androidLocalGeofenceTransitionCount: ${proof.androidEmulatorBridgeAccounting.localGeofenceTransitionCount}`,
    `- localParentChildRuntimeObserved: ${proof.parentChildLocalRuntimeAccounting.localParentChildRuntimeObserved}`,
    `- parentChildLocalRuntimeStoredEventCount: ${proof.parentChildLocalRuntimeAccounting.storedEventCount}`,
    `- parentChildLocalRuntimeDeadLetterCount: ${proof.parentChildLocalRuntimeAccounting.deadLetterCount}`,
    `- parentChildLocalRuntimeChildAgentPhaseCount: ${proof.parentChildLocalRuntimeAccounting.childAgentPhaseCount}`,
    `- childRuntimeRequiredArtifactCount: ${proof.childRuntimeRequiredArtifactCount}`,
    `- childRuntimePresentArtifactCount: ${proof.childRuntimePresentArtifactCount}`,
    `- childRuntimeMissingArtifactCount: ${proof.childRuntimeMissingArtifactCount}`,
    `- androidBridgeChildRuntimeMissingArtifactCount: ${proof.androidEmulatorBridgeAccounting.childRuntimeMissingArtifactCount}`,
    '- proof module: packages/tracking-domain/src/tracking-child-runtime-product-readiness-blocker-proof.ts',
    '- proof tests: packages/tracking-domain/tests/contract/tracking-child-runtime-product-readiness-blocker-proof.test.ts',
    '- proof harness: scripts/test/tracking-child-runtime-product-readiness-blocker-proof.mjs',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=30-parent-and-child-ui-ux-surfaces',
    'Child runtime product-readiness blocker rows consume snapshot requirement rows and preserve explicit non-claims.',
    'Rows also consume the Android emulator readiness bridge so package launch, foreground service, permissions, and local emulator geofence evidence are accounted for in the child-runtime blocker.',
    'Rows consume the parent-child local runtime bridge so typed local transport, stored event, child-agent phase, and parent read-model projection evidence are accounted for before physical runtime handoff.',
    'Rows prove requirement coverage only and do not claim actual child-device delivery, execution, or rendered child UI.',
    'Provider delivery, notification receipt ingestion, live location runtime, physical-device proof, authority proof, production workers, and product-ready behavior are explicit non-claims.',
    '',
  ].join('\n');
}

function androidEmulatorBridgeAccountingFrom(sourceAndroidEmulatorBridgeProof) {
  const [row] = sourceAndroidEmulatorBridgeProof.readModel?.rows ?? sourceAndroidEmulatorBridgeProof.rows ?? [];
  if (row === undefined) {
    throw new Error('Android emulator bridge proof has no rows');
  }
  return {
    sourceAndroidEmulatorBridgeProofRef,
    bridgeRowId: row.rowId,
    bridgeStatus: row.status,
    androidEmulatorPrerequisitesObserved: row.emulatorPrerequisitesObserved,
    packageLaunchObserved: row.packageLaunchObserved,
    foregroundServiceObserved: row.foregroundServiceObserved,
    foregroundPermissionGranted: row.foregroundPermissionGranted,
    backgroundPermissionGranted: row.backgroundPermissionGranted,
    localGeofenceTransitionCount: row.localGeofenceTransitionCount,
    childRuntimeRequiredArtifactCount: row.childRuntimeRequiredArtifacts.length,
    childRuntimePresentArtifactCount: row.childRuntimePresentArtifacts.length,
    childRuntimeMissingArtifactCount: row.childRuntimeMissingArtifacts.length,
  };
}

function parentChildLocalRuntimeAccountingFrom(sourceParentChildLocalRuntimeBridgeProof) {
  const [row] =
    sourceParentChildLocalRuntimeBridgeProof.readModel?.rows ?? sourceParentChildLocalRuntimeBridgeProof.rows ?? [];
  if (row === undefined) {
    throw new Error('Parent-child local runtime bridge proof has no rows');
  }
  return {
    sourceParentChildLocalRuntimeBridgeProofRef,
    bridgeRowId: row.rowId,
    bridgeStatus: row.status,
    localParentChildRuntimeObserved: row.localParentChildRuntimeObserved,
    typedLocalServiceTransportObserved: row.typedLocalServiceTransportObserved,
    parentReadModelProjectionObserved: row.parentReadModelProjectionObserved,
    publishReportCount: row.publishReportCount,
    storedEventCount: row.storedEventCount,
    deadLetterCount: row.deadLetterCount,
    childAgentPhaseCount: row.childAgentPhaseCount,
  };
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
