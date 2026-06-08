import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-product-readiness-closure-proof';
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const namedProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const resultRoot = join(repoRoot, 'test-results', proofMode);
const generatedAt = '2026-06-07T16:30:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

const sourceProofs = [
  sourceProof('pre-device-gate', 'output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json'),
  sourceProof('android-emulator-proof', 'test-results/tracking-plan-android-emulator-proof/proof.json'),
  sourceProof('ios-simulator-proof', 'test-results/tracking-plan-ios-simulator-proof/proof.json'),
  sourceProof(
    'ios-privacy-disclosure-release-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/47-ios-privacy-disclosure-release-proof.json'
  ),
  sourceProof('wsl-local-replay', 'output/tracking-plan-proof/wsl-local-replay/proof.json'),
  sourceProof('hosted-ui-artifact-inventory', 'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json'),
  sourceProof(
    'android-emulator-artifact-inventory',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/68-android-emulator-artifact-inventory-proof.json'
  ),
  sourceProof(
    'ios-simulator-artifact-inventory',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/69-ios-simulator-artifact-inventory-proof.json'
  ),
  sourceProof(
    'android-system-geofence-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/44-android-system-geofence-blocker-proof.json'
  ),
  sourceProof(
    'notification-receipt-boundary',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/22-notification-receipt-boundary-proof.json'
  ),
  sourceProof(
    'notification-preference-preflight',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/24-notification-preference-preflight-proof.json'
  ),
  sourceProof(
    'notification-preference-status-handoff',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/54-notification-preference-status-handoff-proof.json'
  ),
  sourceProof(
    'notification-local-outbox-readiness',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/42-notification-local-outbox-readiness-proof.json'
  ),
  sourceProof(
    'authority-enrollment-manual-required',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/48-authority-enrollment-manual-required-proof.json'
  ),
  sourceProof(
    'authority-runtime-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/55-authority-runtime-readiness-blocker-proof.json'
  ),
  sourceProof(
    'authority-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/61-authority-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'child-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/50-child-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'child-runtime-android-emulator-readiness-bridge',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/67-child-runtime-android-emulator-readiness-bridge-proof.json'
  ),
  sourceProof(
    'parent-child-local-runtime-bridge',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/68-parent-child-local-runtime-bridge-proof.json'
  ),
  sourceProof(
    'physical-device-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/49-physical-device-artifact-gate-proof.json'
  ),
  sourceProof(
    'physical-device-evidence-review',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/73-physical-device-evidence-review-proof.json'
  ),
  sourceProof(
    'provider-delivery-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/51-provider-delivery-artifact-gate-proof.json'
  ),
  sourceProof(
    'provider-runtime-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/52-provider-runtime-readiness-blocker-proof.json'
  ),
  sourceProof(
    'escalation-runtime-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/53-escalation-runtime-readiness-blocker-proof.json'
  ),
  sourceProof(
    'escalation-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/62-escalation-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'child-runtime-product-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/45-child-runtime-product-readiness-blocker-proof.json'
  ),
  sourceProof(
    'full-product-ui-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/56-full-product-ui-readiness-blocker-proof.json'
  ),
  sourceProof(
    'full-product-ui-local-runtime-artifact-capture',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/66-full-product-ui-local-runtime-artifact-capture-proof.json'
  ),
  sourceProof(
    'full-product-ui-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/59-full-product-ui-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'full-product-ui-runtime-preflight',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/71-full-product-ui-runtime-preflight-proof.json'
  ),
  sourceProof(
    'cross-platform-runtime-capability',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/75-cross-platform-runtime-capability-proof.json'
  ),
  sourceProof(
    'production-durable-workers-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/57-production-durable-workers-readiness-blocker-proof.json'
  ),
  sourceProof(
    'production-worker-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/58-production-worker-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'production-worker-runtime-preflight',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/72-production-worker-runtime-preflight-proof.json'
  ),
  sourceProof(
    'retention-product-readiness-blocker',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/43-retention-product-readiness-proof.json'
  ),
  sourceProof(
    'retention-runtime-artifact-gate',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/60-retention-runtime-artifact-gate-proof.json'
  ),
  sourceProof(
    'retention-platform-enforcement-preflight',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/70-retention-platform-enforcement-preflight-proof.json'
  ),
  sourceProof(
    'tracking-claim-audit',
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/65-claim-audit-proof.json'
  ),
];

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output33, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-product-readiness-closure-proof',
  ]);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-full-product-ui-local-runtime-artifact-capture-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-full-product-ui-runtime-preflight-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-android-emulator-artifact-inventory-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-ios-simulator-artifact-inventory-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-parent-child-local-runtime-bridge-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-cross-platform-runtime-capability-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-physical-device-artifact-gate-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-physical-device-evidence-review-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-retention-runtime-artifact-gate-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-retention-platform-enforcement-preflight-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-production-worker-runtime-preflight-proof.mjs']);
  run('cmd', ['/c', 'node', 'scripts/test/tracking-claim-audit-proof.mjs']);

  await assertSourceProofsExist();
  const proof = await buildProof();
  assertProof(proof);
  await writeProofArtifacts(proof);

  console.log('tracking-product-readiness-closure-proof-ok');
  console.log('evidence=test-results/tracking-product-readiness-closure-proof/proof.json');
}

async function buildProof() {
  const proofModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-product-readiness-closure-proof.js'))
      .href
  );
  return {
    ...proofModule.buildTrackingProductReadinessClosureProof(generatedAt, sourceProofs, await aggregateEvidence()),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    artifactPaths: {
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/46-product-readiness-closure-proof.json',
      evidence: 'test-results/tracking-product-readiness-closure-proof/proof.json',
      namedProofRoot: 'output/tracking-plan-proof/tracking-product-readiness-closure-proof/proof.json',
    },
  };
}

async function aggregateEvidence() {
  const fullProductUiProof = await readJson(
    'test-results/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json'
  );
  const fullProductUiRuntimePreflightProof = await readJson(
    'test-results/tracking-full-product-ui-runtime-preflight-proof/proof.json'
  );
  const androidEmulatorArtifactInventoryProof = await readJson(
    'test-results/tracking-android-emulator-artifact-inventory-proof/proof.json'
  );
  const iosSimulatorArtifactInventoryProof = await readJson(
    'test-results/tracking-ios-simulator-artifact-inventory-proof/proof.json'
  );
  const authorityRuntimeArtifactGateProof = await readJson(
    'test-results/tracking-authority-runtime-artifact-gate-proof/proof.json'
  );
  const authorityRuntimeReadinessProof = await readJson(
    'test-results/tracking-authority-runtime-readiness-blocker-proof/proof.json'
  );
  const retentionRuntimeProof = await readJson(
    'test-results/tracking-retention-runtime-artifact-gate-proof/proof.json'
  );
  const retentionPlatformPreflightProof = await readJson(
    'test-results/tracking-retention-platform-enforcement-preflight-proof/proof.json'
  );
  const childRuntimeArtifactGateProof = await readJson(
    'test-results/tracking-child-runtime-artifact-gate-proof/proof.json'
  );
  const parentChildLocalRuntimeBridgeProof = await readJson(
    'test-results/tracking-parent-child-local-runtime-bridge-proof/proof.json'
  );
  const crossPlatformRuntimeCapabilityProof = await readJson(
    'test-results/tracking-cross-platform-runtime-capability-proof/proof.json'
  );
  const physicalDeviceEvidenceReviewProof = await readJson(
    'test-results/tracking-physical-device-evidence-review-proof/proof.json'
  );
  const providerRuntimeReadinessProof = await readJson(
    'test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json'
  );
  const escalationRuntimeArtifactGateProof = await readJson(
    'test-results/tracking-escalation-runtime-artifact-gate-proof/proof.json'
  );
  const escalationRuntimeReadinessProof = await readJson(
    'test-results/tracking-escalation-runtime-readiness-blocker-proof/proof.json'
  );
  const productionWorkerRuntimeArtifactGateProof = await readJson(
    'test-results/tracking-production-worker-runtime-artifact-gate-proof/proof.json'
  );
  const productionWorkerRuntimePreflightProof = await readJson(
    'test-results/tracking-production-worker-runtime-preflight-proof/proof.json'
  );
  const claimAuditProof = await readJson('test-results/tracking-claim-audit-proof/proof.json');
  const childRuntimeArtifactSummary = artifactSummaryFromRows(childRuntimeArtifactGateProof);
  const productionWorkerArtifactSummary = artifactSummaryFromRows(productionWorkerRuntimeArtifactGateProof);
  return {
    fullProductUiLocalArtifactCount: fullProductUiProof.readModel.localArtifactCount,
    fullProductUiClosureRetentionWritableExecutionRowCount:
      fullProductUiProof.readModel.closureEvidence.retentionWritableExecutionRowCount,
    fullProductUiClosureRetentionWritableExecutionDerivationCount:
      fullProductUiProof.readModel.closureEvidence.retentionWritableExecutionDerivationCount,
    fullProductUiClosureChildRuntimeMissingArtifactCount:
      fullProductUiProof.readModel.closureEvidence.childRuntimeMissingArtifactCount,
    fullProductUiRuntimePreflightRowCount: fullProductUiRuntimePreflightProof.summary.rowCount,
    fullProductUiRuntimePreflightManualRequiredRowCount:
      fullProductUiRuntimePreflightProof.summary.manualRequiredRowCount,
    fullProductUiRuntimePreflightRequiredArtifactCount:
      fullProductUiRuntimePreflightProof.summary.requiredArtifactCount,
    fullProductUiRuntimePreflightPresentArtifactCount: fullProductUiRuntimePreflightProof.summary.presentArtifactCount,
    fullProductUiRuntimePreflightMissingArtifactCount: fullProductUiRuntimePreflightProof.summary.missingArtifactCount,
    fullProductUiRuntimePreflightProductReadyRowCount: fullProductUiRuntimePreflightProof.summary.productReadyRowCount,
    androidEmulatorRequiredArtifactCount: androidEmulatorArtifactInventoryProof.summary.requiredArtifactCount,
    androidEmulatorPresentArtifactCount: androidEmulatorArtifactInventoryProof.summary.presentArtifactCount,
    androidEmulatorMissingArtifactCount: androidEmulatorArtifactInventoryProof.summary.missingArtifactCount,
    androidEmulatorPermissionUiArtifactCount: androidEmulatorArtifactInventoryProof.summary.permissionUiArtifactCount,
    androidEmulatorRuntimeArtifactCount: androidEmulatorArtifactInventoryProof.summary.runtimeArtifactCount,
    androidEmulatorLocalGeofenceTransitionCount:
      androidEmulatorArtifactInventoryProof.summary.localGeofenceTransitionCount,
    iosSimulatorRequiredArtifactCount: iosSimulatorArtifactInventoryProof.summary.requiredArtifactCount,
    iosSimulatorPresentArtifactCount: iosSimulatorArtifactInventoryProof.summary.presentArtifactCount,
    iosSimulatorMissingArtifactCount: iosSimulatorArtifactInventoryProof.summary.missingArtifactCount,
    iosSimulatorPackageArtifactCount: iosSimulatorArtifactInventoryProof.summary.simulatorPackageArtifactCount,
    iosSimulatorLocationManualRequiredArtifactCount:
      iosSimulatorArtifactInventoryProof.summary.locationManualRequiredArtifactCount,
    iosSimulatorPrivacyDisclosureArtifactCount:
      iosSimulatorArtifactInventoryProof.summary.privacyDisclosureArtifactCount,
    iosSimulatorManualRequiredRowCount: iosSimulatorArtifactInventoryProof.summary.iosManualRequiredRowCount,
    iosSimulatorMissingRuntimeArtifactCount: iosSimulatorArtifactInventoryProof.summary.iosMissingRuntimeArtifactCount,
    authorityRuntimeRequiredArtifactCount: authorityRuntimeArtifactGateProof.summary.requiredArtifactCount,
    authorityRuntimePresentArtifactCount:
      authorityRuntimeArtifactGateProof.summary.requiredArtifactCount -
      authorityRuntimeArtifactGateProof.summary.missingArtifactCount,
    authorityRuntimeMissingArtifactCount: authorityRuntimeArtifactGateProof.summary.missingArtifactCount,
    authorityRuntimeBlockerCount: authorityRuntimeReadinessProof.summary.blockerCount,
    childRuntimeRequiredArtifactCount: childRuntimeArtifactSummary.requiredArtifactCount,
    childRuntimePresentArtifactCount: childRuntimeArtifactSummary.presentArtifactCount,
    childRuntimeMissingArtifactCount: childRuntimeArtifactSummary.missingArtifactCount,
    parentChildLocalRuntimeStoredEventCount: parentChildLocalRuntimeBridgeProof.summary.storedEventCount,
    parentChildLocalRuntimeDeadLetterCount: parentChildLocalRuntimeBridgeProof.summary.deadLetterCount,
    parentChildLocalRuntimeChildAgentPhaseCount: parentChildLocalRuntimeBridgeProof.summary.childAgentPhaseCount,
    parentChildLocalRuntimeProductReadyRowCount: parentChildLocalRuntimeBridgeProof.summary.productReadyRows,
    crossPlatformCapabilityRowCount: crossPlatformRuntimeCapabilityProof.summary.rowCount,
    crossPlatformLocalProofPassedRowCount: crossPlatformRuntimeCapabilityProof.summary.localProofPassedRows,
    crossPlatformCiRunnableRowCount: crossPlatformRuntimeCapabilityProof.summary.ciRunnableRows,
    crossPlatformCiManualRequiredRowCount: crossPlatformRuntimeCapabilityProof.summary.ciManualRequiredRows,
    crossPlatformHostToolUnavailableRowCount: crossPlatformRuntimeCapabilityProof.summary.hostToolUnavailableRows,
    crossPlatformAndroidSdkToolchainObservedRows: Number(
      crossPlatformRuntimeCapabilityProof.productClaims.androidSdkToolchainObserved
    ),
    crossPlatformAndroidGradleBuildObservedRows: Number(
      crossPlatformRuntimeCapabilityProof.productClaims.androidGradleProjectBuildObserved
    ),
    crossPlatformProductReadyRowCount: crossPlatformRuntimeCapabilityProof.summary.productReadyRows,
    physicalDeviceEvidenceReviewRowCount: physicalDeviceEvidenceReviewProof.summary.rowCount,
    physicalDeviceEvidenceReviewArtifactMissingRowCount: physicalDeviceEvidenceReviewProof.summary.artifactMissingRows,
    physicalDeviceEvidenceReviewContentReviewRequiredRowCount:
      physicalDeviceEvidenceReviewProof.summary.contentReviewRequiredRows,
    physicalDeviceEvidenceReviewContentAcceptedRowCount: physicalDeviceEvidenceReviewProof.summary.contentAcceptedRows,
    physicalDeviceEvidenceReviewProductReadyRowCount: physicalDeviceEvidenceReviewProof.summary.productReadyRows,
    physicalDeviceEvidenceReviewStatusObservedRowCount:
      physicalDeviceEvidenceReviewProof.summary.physicalDeviceStatusObservedRows,
    physicalDeviceEvidenceReviewSupportingStatusArtifactCount:
      physicalDeviceEvidenceReviewProof.summary.supportingStatusArtifactCount,
    providerRuntimeRequiredArtifactCount: providerRuntimeReadinessProof.summary.requiredProviderRuntimeArtifactCount,
    providerRuntimePresentArtifactCount: providerRuntimeReadinessProof.summary.presentProviderRuntimeArtifactCount,
    providerRuntimeMissingArtifactCount: providerRuntimeReadinessProof.summary.missingProviderRuntimeArtifactCount,
    providerRuntimeBlockerCount: providerRuntimeReadinessProof.summary.blockerCount,
    escalationRuntimeRequiredArtifactCount: escalationRuntimeArtifactGateProof.summary.requiredArtifactCount,
    escalationRuntimePresentArtifactCount:
      escalationRuntimeArtifactGateProof.summary.requiredArtifactCount -
      escalationRuntimeArtifactGateProof.summary.missingArtifactCount,
    escalationRuntimeMissingArtifactCount: escalationRuntimeArtifactGateProof.summary.missingArtifactCount,
    escalationRuntimeBlockerCount: escalationRuntimeReadinessProof.summary.blockerCount,
    retentionRuntimeRequiredArtifactCount: retentionRuntimeProof.summary.requiredArtifactCount,
    retentionRuntimePresentArtifactCount:
      retentionRuntimeProof.summary.requiredArtifactCount - retentionRuntimeProof.summary.missingArtifactCount,
    retentionRuntimeMissingArtifactCount: retentionRuntimeProof.summary.missingArtifactCount,
    retentionRuntimeManualRequiredRowCount: retentionRuntimeProof.summary.manualRequiredRows,
    retentionRuntimeArtifactSetPresentRowCount: retentionRuntimeProof.summary.completeRows,
    retentionPlatformPreflightRowCount: retentionPlatformPreflightProof.summary.rowCount,
    retentionPlatformPreflightManualRequiredRowCount: retentionPlatformPreflightProof.summary.manualRequiredRowCount,
    retentionPlatformPreflightRequiredArtifactCount: retentionPlatformPreflightProof.summary.requiredArtifactCount,
    retentionPlatformPreflightPresentArtifactCount: retentionPlatformPreflightProof.summary.presentArtifactCount,
    retentionPlatformPreflightMissingArtifactCount: retentionPlatformPreflightProof.summary.missingArtifactCount,
    retentionPlatformPreflightProductReadyRowCount: retentionPlatformPreflightProof.summary.productReadyRowCount,
    productionWorkerRequiredArtifactCount: productionWorkerArtifactSummary.requiredArtifactCount,
    productionWorkerPresentArtifactCount: productionWorkerArtifactSummary.presentArtifactCount,
    productionWorkerMissingArtifactCount: productionWorkerArtifactSummary.missingArtifactCount,
    productionWorkerPreflightRowCount: productionWorkerRuntimePreflightProof.summary.rowCount,
    productionWorkerPreflightManualRequiredRowCount:
      productionWorkerRuntimePreflightProof.summary.manualRequiredRowCount,
    productionWorkerPreflightRequiredArtifactCount: productionWorkerRuntimePreflightProof.summary.requiredArtifactCount,
    productionWorkerPreflightPresentArtifactCount: productionWorkerRuntimePreflightProof.summary.presentArtifactCount,
    productionWorkerPreflightMissingArtifactCount: productionWorkerRuntimePreflightProof.summary.missingArtifactCount,
    productionWorkerPreflightProductReadyRowCount: productionWorkerRuntimePreflightProof.summary.productReadyRowCount,
    claimAuditPresentArtifactCount: claimAuditProof.summary.presentArtifactCount,
    claimAuditMissingArtifactCount: claimAuditProof.summary.missingArtifactCount,
    claimAuditManualRequiredRowCount: claimAuditProof.summary.manualRequiredRowCount,
    claimAuditPhysicalDeviceRequiredRowCount: claimAuditProof.summary.physicalDeviceRequiredRowCount,
    claimAuditApprovedManualRequiredRowCount: claimAuditProof.summary.approvedManualRequiredRowCount,
    claimAuditManualProviderRuntimeRequiredRowCount: claimAuditProof.summary.manualProviderRuntimeRequiredRowCount,
    claimAuditProductionRuntimeRequiredRowCount: claimAuditProof.summary.productionRuntimeRequiredRowCount,
    claimAuditProductReadyRowCount: claimAuditProof.summary.productReadyRowCount,
    productClaimReady: false,
  };
}

function assertProof(proof) {
  const [row] = proof.rows;
  if (!row || !proof.proofClaims.remainingProductBlockersEnumerated) {
    throw new Error(`Tracking product readiness closure proof is empty: ${JSON.stringify(proof)}`);
  }
  if (
    row.physicalAndroidBackgroundClaimed ||
    row.physicalIosBackgroundClaimed ||
    row.authorityClaimed ||
    row.productionWorkersClaimed ||
    row.productReadyClaimed
  ) {
    throw new Error(`Tracking product readiness closure overclaimed product readiness: ${JSON.stringify(row)}`);
  }
  if (
    proof.aggregateEvidence.fullProductUiRuntimePreflightRequiredArtifactCount !==
      proof.aggregateEvidence.fullProductUiRuntimePreflightPresentArtifactCount +
        proof.aggregateEvidence.fullProductUiRuntimePreflightMissingArtifactCount ||
    proof.aggregateEvidence.fullProductUiRuntimePreflightManualRequiredRowCount !==
      proof.aggregateEvidence.fullProductUiRuntimePreflightRowCount ||
    proof.aggregateEvidence.fullProductUiRuntimePreflightProductReadyRowCount !== 0
  ) {
    throw new Error(
      `Tracking product readiness closure lost full product UI preflight evidence: ${JSON.stringify(
        proof.aggregateEvidence
      )}`
    );
  }
  if (
    proof.aggregateEvidence.productionWorkerPreflightRequiredArtifactCount !==
      proof.aggregateEvidence.productionWorkerPreflightPresentArtifactCount +
        proof.aggregateEvidence.productionWorkerPreflightMissingArtifactCount ||
    proof.aggregateEvidence.productionWorkerPreflightManualRequiredRowCount !==
      proof.aggregateEvidence.productionWorkerPreflightRowCount ||
    proof.aggregateEvidence.productionWorkerPreflightProductReadyRowCount !== 0
  ) {
    throw new Error(
      `Tracking product readiness closure lost production worker preflight evidence: ${JSON.stringify(
        proof.aggregateEvidence
      )}`
    );
  }
  if (
    proof.aggregateEvidence.physicalDeviceEvidenceReviewRowCount !==
      proof.aggregateEvidence.physicalDeviceEvidenceReviewArtifactMissingRowCount +
        proof.aggregateEvidence.physicalDeviceEvidenceReviewContentReviewRequiredRowCount ||
    proof.aggregateEvidence.physicalDeviceEvidenceReviewContentAcceptedRowCount !== 0 ||
    proof.aggregateEvidence.physicalDeviceEvidenceReviewProductReadyRowCount !== 0
  ) {
    throw new Error(
      `Tracking product readiness closure overclaimed physical evidence review: ${JSON.stringify(
        proof.aggregateEvidence
      )}`
    );
  }
  if (
    proof.aggregateEvidence.parentChildLocalRuntimeStoredEventCount < 9 ||
    proof.aggregateEvidence.parentChildLocalRuntimeDeadLetterCount !== 0 ||
    proof.aggregateEvidence.parentChildLocalRuntimeChildAgentPhaseCount < 4 ||
    proof.aggregateEvidence.parentChildLocalRuntimeProductReadyRowCount !== 0
  ) {
    throw new Error(
      `Tracking product readiness closure lost parent-child local runtime bridge evidence: ${JSON.stringify(
        proof.aggregateEvidence
      )}`
    );
  }
  if (
    proof.aggregateEvidence.crossPlatformCapabilityRowCount < 8 ||
    proof.aggregateEvidence.crossPlatformLocalProofPassedRowCount < 6 ||
    proof.aggregateEvidence.crossPlatformAndroidSdkToolchainObservedRows < 1 ||
    proof.aggregateEvidence.crossPlatformAndroidGradleBuildObservedRows < 1 ||
    proof.aggregateEvidence.crossPlatformProductReadyRowCount !== 0
  ) {
    throw new Error(
      `Tracking product readiness closure lost cross-platform capability accounting: ${JSON.stringify(
        proof.aggregateEvidence
      )}`
    );
  }
  if (
    proof.aggregateEvidence.physicalDeviceEvidenceReviewStatusObservedRowCount < 0 ||
    proof.aggregateEvidence.physicalDeviceEvidenceReviewStatusObservedRowCount >
      proof.aggregateEvidence.physicalDeviceEvidenceReviewRowCount ||
    proof.aggregateEvidence.physicalDeviceEvidenceReviewSupportingStatusArtifactCount <
      proof.aggregateEvidence.physicalDeviceEvidenceReviewStatusObservedRowCount
  ) {
    throw new Error(
      `Tracking product readiness closure has inconsistent physical status support counts: ${JSON.stringify(
        proof.aggregateEvidence
      )}`
    );
  }
  if (
    proof.aggregateEvidence.authorityRuntimeRequiredArtifactCount !==
      proof.aggregateEvidence.authorityRuntimePresentArtifactCount +
        proof.aggregateEvidence.authorityRuntimeMissingArtifactCount ||
    proof.aggregateEvidence.authorityRuntimePresentArtifactCount !== 0 ||
    proof.aggregateEvidence.authorityRuntimeBlockerCount === 0
  ) {
    throw new Error(
      `Tracking product readiness closure lost authority runtime evidence: ${JSON.stringify(proof.aggregateEvidence)}`
    );
  }
  if (
    proof.aggregateEvidence.providerRuntimeRequiredArtifactCount !==
      proof.aggregateEvidence.providerRuntimePresentArtifactCount +
        proof.aggregateEvidence.providerRuntimeMissingArtifactCount ||
    proof.aggregateEvidence.providerRuntimePresentArtifactCount !== 0 ||
    proof.aggregateEvidence.providerRuntimeBlockerCount === 0
  ) {
    throw new Error(
      `Tracking product readiness closure lost provider runtime evidence: ${JSON.stringify(proof.aggregateEvidence)}`
    );
  }
  if (
    proof.aggregateEvidence.escalationRuntimeRequiredArtifactCount !==
      proof.aggregateEvidence.escalationRuntimePresentArtifactCount +
        proof.aggregateEvidence.escalationRuntimeMissingArtifactCount ||
    proof.aggregateEvidence.escalationRuntimePresentArtifactCount !== 0 ||
    proof.aggregateEvidence.escalationRuntimeBlockerCount === 0
  ) {
    throw new Error(
      `Tracking product readiness closure lost escalation runtime evidence: ${JSON.stringify(proof.aggregateEvidence)}`
    );
  }
}

async function writeProofArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(resultRoot, 'product-readiness-closure-read-model.json'), proof.rows);
  await writeJson(join(output33, '46-product-readiness-closure-proof.json'), proof);
  await writeJson(join(namedProofRoot, 'proof.json'), proof);
  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeFile(join(namedProofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(join(namedProofRoot, '16-validation-commands.log'), validationLog());
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Product Readiness Closure Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- requiredProofTier: P3_LOCAL_DEV_MACHINE',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    '- status: proved',
    '- proves local/CI proof accounting is closed for current tracking continuation scope',
    `- fullProductUiRuntimePreflightRowCount: ${proof.aggregateEvidence.fullProductUiRuntimePreflightRowCount}`,
    `- fullProductUiRuntimePreflightRequiredArtifactCount: ${proof.aggregateEvidence.fullProductUiRuntimePreflightRequiredArtifactCount}`,
    `- fullProductUiRuntimePreflightMissingArtifactCount: ${proof.aggregateEvidence.fullProductUiRuntimePreflightMissingArtifactCount}`,
    `- androidEmulatorRequiredArtifactCount: ${proof.aggregateEvidence.androidEmulatorRequiredArtifactCount}`,
    `- androidEmulatorPresentArtifactCount: ${proof.aggregateEvidence.androidEmulatorPresentArtifactCount}`,
    `- androidEmulatorMissingArtifactCount: ${proof.aggregateEvidence.androidEmulatorMissingArtifactCount}`,
    `- androidEmulatorPermissionUiArtifactCount: ${proof.aggregateEvidence.androidEmulatorPermissionUiArtifactCount}`,
    `- androidEmulatorRuntimeArtifactCount: ${proof.aggregateEvidence.androidEmulatorRuntimeArtifactCount}`,
    `- androidEmulatorLocalGeofenceTransitionCount: ${proof.aggregateEvidence.androidEmulatorLocalGeofenceTransitionCount}`,
    `- iosSimulatorRequiredArtifactCount: ${proof.aggregateEvidence.iosSimulatorRequiredArtifactCount}`,
    `- iosSimulatorPresentArtifactCount: ${proof.aggregateEvidence.iosSimulatorPresentArtifactCount}`,
    `- iosSimulatorMissingArtifactCount: ${proof.aggregateEvidence.iosSimulatorMissingArtifactCount}`,
    `- iosSimulatorPackageArtifactCount: ${proof.aggregateEvidence.iosSimulatorPackageArtifactCount}`,
    `- iosSimulatorLocationManualRequiredArtifactCount: ${proof.aggregateEvidence.iosSimulatorLocationManualRequiredArtifactCount}`,
    `- iosSimulatorPrivacyDisclosureArtifactCount: ${proof.aggregateEvidence.iosSimulatorPrivacyDisclosureArtifactCount}`,
    `- iosSimulatorManualRequiredRowCount: ${proof.aggregateEvidence.iosSimulatorManualRequiredRowCount}`,
    `- iosSimulatorMissingRuntimeArtifactCount: ${proof.aggregateEvidence.iosSimulatorMissingRuntimeArtifactCount}`,
    `- authorityRuntimeRequiredArtifactCount: ${proof.aggregateEvidence.authorityRuntimeRequiredArtifactCount}`,
    `- authorityRuntimePresentArtifactCount: ${proof.aggregateEvidence.authorityRuntimePresentArtifactCount}`,
    `- authorityRuntimeMissingArtifactCount: ${proof.aggregateEvidence.authorityRuntimeMissingArtifactCount}`,
    `- authorityRuntimeBlockerCount: ${proof.aggregateEvidence.authorityRuntimeBlockerCount}`,
    `- childRuntimeRequiredArtifactCount: ${proof.aggregateEvidence.childRuntimeRequiredArtifactCount}`,
    `- childRuntimePresentArtifactCount: ${proof.aggregateEvidence.childRuntimePresentArtifactCount}`,
    `- childRuntimeMissingArtifactCount: ${proof.aggregateEvidence.childRuntimeMissingArtifactCount}`,
    `- parentChildLocalRuntimeStoredEventCount: ${proof.aggregateEvidence.parentChildLocalRuntimeStoredEventCount}`,
    `- parentChildLocalRuntimeDeadLetterCount: ${proof.aggregateEvidence.parentChildLocalRuntimeDeadLetterCount}`,
    `- parentChildLocalRuntimeChildAgentPhaseCount: ${proof.aggregateEvidence.parentChildLocalRuntimeChildAgentPhaseCount}`,
    `- crossPlatformCapabilityRowCount: ${proof.aggregateEvidence.crossPlatformCapabilityRowCount}`,
    `- crossPlatformLocalProofPassedRowCount: ${proof.aggregateEvidence.crossPlatformLocalProofPassedRowCount}`,
    `- crossPlatformCiRunnableRowCount: ${proof.aggregateEvidence.crossPlatformCiRunnableRowCount}`,
    `- crossPlatformCiManualRequiredRowCount: ${proof.aggregateEvidence.crossPlatformCiManualRequiredRowCount}`,
    `- crossPlatformHostToolUnavailableRowCount: ${proof.aggregateEvidence.crossPlatformHostToolUnavailableRowCount}`,
    `- crossPlatformAndroidSdkToolchainObservedRows: ${proof.aggregateEvidence.crossPlatformAndroidSdkToolchainObservedRows}`,
    `- crossPlatformAndroidGradleBuildObservedRows: ${proof.aggregateEvidence.crossPlatformAndroidGradleBuildObservedRows}`,
    `- physicalDeviceEvidenceReviewRowCount: ${proof.aggregateEvidence.physicalDeviceEvidenceReviewRowCount}`,
    `- physicalDeviceEvidenceReviewArtifactMissingRowCount: ${proof.aggregateEvidence.physicalDeviceEvidenceReviewArtifactMissingRowCount}`,
    `- physicalDeviceEvidenceReviewContentReviewRequiredRowCount: ${proof.aggregateEvidence.physicalDeviceEvidenceReviewContentReviewRequiredRowCount}`,
    `- physicalDeviceEvidenceReviewContentAcceptedRowCount: ${proof.aggregateEvidence.physicalDeviceEvidenceReviewContentAcceptedRowCount}`,
    `- physicalDeviceEvidenceReviewStatusObservedRowCount: ${proof.aggregateEvidence.physicalDeviceEvidenceReviewStatusObservedRowCount}`,
    `- physicalDeviceEvidenceReviewSupportingStatusArtifactCount: ${proof.aggregateEvidence.physicalDeviceEvidenceReviewSupportingStatusArtifactCount}`,
    `- providerRuntimeRequiredArtifactCount: ${proof.aggregateEvidence.providerRuntimeRequiredArtifactCount}`,
    `- providerRuntimePresentArtifactCount: ${proof.aggregateEvidence.providerRuntimePresentArtifactCount}`,
    `- providerRuntimeMissingArtifactCount: ${proof.aggregateEvidence.providerRuntimeMissingArtifactCount}`,
    `- providerRuntimeBlockerCount: ${proof.aggregateEvidence.providerRuntimeBlockerCount}`,
    `- escalationRuntimeRequiredArtifactCount: ${proof.aggregateEvidence.escalationRuntimeRequiredArtifactCount}`,
    `- escalationRuntimePresentArtifactCount: ${proof.aggregateEvidence.escalationRuntimePresentArtifactCount}`,
    `- escalationRuntimeMissingArtifactCount: ${proof.aggregateEvidence.escalationRuntimeMissingArtifactCount}`,
    `- escalationRuntimeBlockerCount: ${proof.aggregateEvidence.escalationRuntimeBlockerCount}`,
    `- retentionRuntimeRequiredArtifactCount: ${proof.aggregateEvidence.retentionRuntimeRequiredArtifactCount}`,
    `- retentionRuntimePresentArtifactCount: ${proof.aggregateEvidence.retentionRuntimePresentArtifactCount}`,
    `- retentionRuntimeMissingArtifactCount: ${proof.aggregateEvidence.retentionRuntimeMissingArtifactCount}`,
    `- retentionRuntimeManualRequiredRowCount: ${proof.aggregateEvidence.retentionRuntimeManualRequiredRowCount}`,
    `- retentionPlatformPreflightRowCount: ${proof.aggregateEvidence.retentionPlatformPreflightRowCount}`,
    `- retentionPlatformPreflightManualRequiredRowCount: ${proof.aggregateEvidence.retentionPlatformPreflightManualRequiredRowCount}`,
    `- retentionPlatformPreflightRequiredArtifactCount: ${proof.aggregateEvidence.retentionPlatformPreflightRequiredArtifactCount}`,
    `- retentionPlatformPreflightPresentArtifactCount: ${proof.aggregateEvidence.retentionPlatformPreflightPresentArtifactCount}`,
    `- retentionPlatformPreflightMissingArtifactCount: ${proof.aggregateEvidence.retentionPlatformPreflightMissingArtifactCount}`,
    `- productionWorkerRequiredArtifactCount: ${proof.aggregateEvidence.productionWorkerRequiredArtifactCount}`,
    `- productionWorkerPresentArtifactCount: ${proof.aggregateEvidence.productionWorkerPresentArtifactCount}`,
    `- productionWorkerMissingArtifactCount: ${proof.aggregateEvidence.productionWorkerMissingArtifactCount}`,
    `- productionWorkerPreflightRowCount: ${proof.aggregateEvidence.productionWorkerPreflightRowCount}`,
    `- productionWorkerPreflightRequiredArtifactCount: ${proof.aggregateEvidence.productionWorkerPreflightRequiredArtifactCount}`,
    `- productionWorkerPreflightMissingArtifactCount: ${proof.aggregateEvidence.productionWorkerPreflightMissingArtifactCount}`,
    `- claimAuditPhysicalDeviceRequiredRowCount: ${proof.aggregateEvidence.claimAuditPhysicalDeviceRequiredRowCount}`,
    `- claimAuditApprovedManualRequiredRowCount: ${proof.aggregateEvidence.claimAuditApprovedManualRequiredRowCount}`,
    `- claimAuditManualProviderRuntimeRequiredRowCount: ${proof.aggregateEvidence.claimAuditManualProviderRuntimeRequiredRowCount}`,
    `- claimAuditProductionRuntimeRequiredRowCount: ${proof.aggregateEvidence.claimAuditProductionRuntimeRequiredRowCount}`,
    '- does not prove retention product settings, physical-device, authority, provider-delivery, production, or product-ready tracking behavior',
    '- proof module: packages/parent-domain/src/tracking-product-readiness-closure-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-product-readiness-closure-proof.test.ts',
    '- proof harness: scripts/test/tracking-product-readiness-closure-proof.mjs',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=33-proof-gates-fixtures-rollout-and-pr-gate',
    'Closure rows cite existing local/CI proof refs and enumerate remaining product blockers.',
    'Android emulator artifact inventory records local adb, permission UI, location runtime, geofence runtime, device-status, and validation-log artifacts while keeping Android physical-device/system-delivery blockers open.',
    'iOS simulator artifact inventory records simulator package, manual-required Core Location, privacy disclosure, platform proof, and validation-log artifacts while keeping Core Location runtime and physical-device blockers open.',
    'Parent-child local runtime bridge records typed local transport handoff and child-agent phase coverage while keeping physical child-device runtime and product claims false.',
    'Retention runtime closure accounting records the local writable settings artifact as present and the platform runtime retention enforcement artifact as missing.',
    'Retention platform enforcement preflight closure accounting records Android, iOS, and desktop manual-required acceptance rows while keeping product-ready retention false.',
    'Production worker runtime preflight closure accounting records eight manual-required production worker acceptance rows while keeping production and product-ready claims false.',
    'Rows do not claim writable retention product settings, platform retention enforcement, Android/iOS physical background behavior, authority enrollment, provider delivery/receipt runtime, production workers, actual child-device runtime, or product readiness.',
    '',
  ].join('\n');
}

async function assertSourceProofsExist() {
  for (const source of sourceProofs) {
    const contents = await readFile(join(repoRoot, source.proofRef), 'utf8');
    const parsed = JSON.parse(contents);
    source.status = statusFrom(parsed);
    source.proofTier = proofTierFrom(parsed, source.proofTier);
  }
}

function artifactSummaryFromRows(sourceProof) {
  const rows = rowsFrom(sourceProof);
  return {
    requiredArtifactCount: rows.reduce((total, row) => total + (row.requiredArtifacts?.length ?? 0), 0),
    presentArtifactCount: rows.reduce((total, row) => total + (row.presentArtifacts?.length ?? 0), 0),
    missingArtifactCount: rows.reduce((total, row) => total + (row.missingArtifacts?.length ?? 0), 0),
  };
}

function rowsFrom(sourceProof) {
  if (Array.isArray(sourceProof.readModel?.rows)) return sourceProof.readModel.rows;
  if (Array.isArray(sourceProof.rows)) return sourceProof.rows;
  throw new Error(`Artifact gate proof has no rows: ${sourceProof.proofMode ?? 'unknown'}`);
}

function sourceProof(coverageTag, proofRef) {
  return {
    coverageTag,
    proofRef,
    status: 'proved',
    proofTier: 'P3_LOCAL_DEV_MACHINE',
  };
}

function statusFrom(parsed) {
  if (typeof parsed.status === 'string' && parsed.status.length > 0) return parsed.status;
  if (parsed.proofClaims || parsed.productClaims || parsed.generatedAt) return 'proved';
  return 'present';
}

function proofTierFrom(parsed, fallback) {
  if (typeof parsed.currentProofTier === 'string' && parsed.currentProofTier.length > 0) {
    return parsed.currentProofTier;
  }
  if (typeof parsed.requiredProofTier === 'string' && parsed.requiredProofTier.length > 0) {
    return parsed.requiredProofTier;
  }
  return fallback;
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

async function readJson(path) {
  return JSON.parse(await readFile(join(repoRoot, path), 'utf8'));
}
