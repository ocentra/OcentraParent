import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-ios-privacy-disclosure-release-proof');
const wp12ProofDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '12-ios-background-region-significant-change-adapter'
);
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const timestamp = '2026-06-07T17:30:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp12ProofDir, { recursive: true });
await mkdir(wp33ProofDir, { recursive: true });

run('cmd', [
  '/c',
  'npm',
  'exec',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'vitest',
  'run',
  'tests/contract/tracking-ios-privacy-disclosure-release-proof.test.ts',
]);

const proofModule = await importSource('packages/tracking-domain/src/tracking-ios-privacy-disclosure-release-proof.ts');
const readModel = proofModule.buildTrackingIosPrivacyDisclosureProofReadModel(
  {
    generatedAt: timestamp,
    proofId: 'tracking-ios-privacy-disclosure-release-proof',
    familyId: 'family-tracking-ios-privacy-release',
    childProfileId: 'child-profile-maya',
    deviceId: 'device-maya-ios',
    deviceLabel: 'Maya iOS release gate',
    sourceProofRefs: [
      'docs/plans/tracking-plan/workpacks/12-ios-background-region-significant-change-adapter.md',
      'docs/expectations/platforms.md',
      'docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md',
      'test-results/tracking-ios-location-manual-required-proof/proof.json',
    ],
  },
  disclosureRows()
);

const proof = {
  proofMode: 'tracking-ios-privacy-disclosure-release-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/tracking-domain/src/tracking-ios-privacy-disclosure-release-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-ios-privacy-disclosure-release-proof.test.ts',
    harness: 'scripts/test/tracking-ios-privacy-disclosure-release-proof.mjs',
    evidence: 'test-results/tracking-ios-privacy-disclosure-release-proof/proof.json',
    readModel: 'test-results/tracking-ios-privacy-disclosure-release-proof/read-model.json',
    wp12Proof:
      'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/20-ios-privacy-disclosure-release-proof.json',
    wp33Proof:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/47-ios-privacy-disclosure-release-proof.json',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPacks(proof);

console.log('tracking-ios-privacy-disclosure-release-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-ios-privacy-disclosure-release-proof', 'proof.json')}`);

function importSource(relativePath) {
  return tsImport(pathToFileURL(join(repoRoot, relativePath)).href, import.meta.url);
}

function disclosureRows() {
  return [
    disclosureRow({
      rowId: 'tracking-ios-location-purpose-disclosure',
      releaseGate: 'location-purpose-disclosure',
      disclosureEvidenceRefs: ['tracking-ios-location-purpose-copy-draft'],
      runtimeEvidenceRefs: ['wp12-ios-location-purpose-copy-draft'],
      requiredBeforeReleaseClaimRefs: ['parent-facing-location-purpose-copy', 'apple-privacy-label-location-data'],
    }),
    disclosureRow({
      rowId: 'tracking-ios-background-location-disclosure',
      releaseGate: 'background-location-disclosure',
      requiredBeforeReleaseClaimRefs: [
        'background-location-purpose-copy',
        'apple-background-mode-review-artifact',
        'physical-device-background-delivery-proof',
      ],
    }),
    disclosureRow({
      rowId: 'tracking-ios-region-monitoring-disclosure',
      releaseGate: 'region-monitoring-disclosure',
      requiredBeforeReleaseClaimRefs: [
        'region-monitoring-purpose-copy',
        'apple-region-monitoring-review-artifact',
        'physical-device-region-transition-proof',
      ],
    }),
    disclosureRow({
      rowId: 'tracking-ios-notification-disclosure',
      releaseGate: 'notification-disclosure',
      requiredBeforeReleaseClaimRefs: ['notification-purpose-copy', 'ios-local-notification-delivery-proof'],
    }),
    disclosureRow({
      rowId: 'tracking-ios-data-custody-disclosure',
      releaseGate: 'data-custody-disclosure',
      disclosureEvidenceRefs: ['tracking-ios-data-custody-copy-draft'],
      runtimeEvidenceRefs: ['wp12-ios-data-custody-copy-draft'],
      requiredBeforeReleaseClaimRefs: [
        'parent-owned-location-custody-copy',
        'retention-window-disclosure',
        'apple-privacy-nutrition-label-artifact',
      ],
    }),
    disclosureRow({
      rowId: 'tracking-ios-app-store-review-evidence',
      releaseGate: 'app-store-review-evidence',
      disclosureEvidenceRefs: ['tracking-ios-app-store-review-checklist-draft'],
      appStoreReviewArtifactRefs: ['app-store-review-required-before-release'],
      privacyNutritionArtifactRefs: ['privacy-nutrition-label-required-before-release'],
      runtimeEvidenceRefs: ['wp12-ios-app-store-review-checklist-draft'],
      requiredBeforeReleaseClaimRefs: [
        'apple-app-store-review-approval-artifact',
        'apple-entitlement-approval-artifact',
        'testflight-device-install-proof',
      ],
    }),
  ];
}

function disclosureRow(input) {
  return {
    rowId: input.rowId,
    releaseGate: input.releaseGate,
    observedAt: timestamp,
    disclosureEvidenceRefs: input.disclosureEvidenceRefs ?? [],
    manualProofRefs: input.manualProofRefs ?? ['wp12-ios-release-disclosure-manual-proof-plan'],
    appStoreReviewArtifactRefs: input.appStoreReviewArtifactRefs ?? [],
    privacyNutritionArtifactRefs: input.privacyNutritionArtifactRefs ?? [],
    runtimeEvidenceRefs: input.runtimeEvidenceRefs ?? [],
    requiredBeforeReleaseClaimRefs: input.requiredBeforeReleaseClaimRefs,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.releaseGateRows.length,
    releaseBlockedCount: readModel.releaseBlockedCount,
    manualReviewRequiredCount: readModel.manualReviewRequiredCount,
    runtimeEvidenceRefs: readModel.runtimeEvidenceRefs.length,
    gateStates: countBy(readModel.releaseGateRows.map((row) => row.gateState)),
    releaseGates: countBy(readModel.releaseGateRows.map((row) => row.releaseGate)),
  };
}

function nonClaims(readModel) {
  return {
    releaseClaimAllowed: readModel.releaseClaimAllowed,
    appStoreReviewClaimed: readModel.appStoreReviewClaimed,
    privacyNutritionLabelClaimed: readModel.privacyNutritionLabelClaimed,
    coreLocationRuntimeClaimed: readModel.coreLocationRuntimeClaimed,
    backgroundLocationDeliveryClaimed: readModel.backgroundLocationDeliveryClaimed,
    regionMonitoringClaimed: readModel.regionMonitoringClaimed,
    notificationDeliveryClaimed: readModel.notificationDeliveryClaimed,
    entitlementProofClaimed: readModel.entitlementProofClaimed,
    testflightDeviceInstallClaimed: readModel.testflightDeviceInstallClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
    productReadyIosTrackingClaimed: readModel.productReadyIosTrackingClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 6 ||
    proof.summary.releaseBlockedCount !== 3 ||
    proof.summary.manualReviewRequiredCount !== 3 ||
    proof.summary.runtimeEvidenceRefs !== 3
  ) {
    throw new Error(`Unexpected iOS privacy disclosure proof summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`iOS privacy disclosure proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPacks(proof) {
  await writeFile(join(wp12ProofDir, '20-ios-privacy-disclosure-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(
    join(wp12ProofDir, '20-ios-privacy-disclosure-validation-commands.log'),
    validationLog(proof),
    'utf8'
  );
  await writeJson(join(wp12ProofDir, '20-ios-privacy-disclosure-release-proof.json'), proof);
  await writeJson(join(wp33ProofDir, '47-ios-privacy-disclosure-release-proof.json'), proof);
}

function sourceSnapshot(proof) {
  return [
    '# WP12 iOS Privacy Disclosure Release Proof Source Snapshot',
    '',
    `- Branch: ${proof.branch}`,
    `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
    '- Git status at proof generation:',
    '',
    '```text',
    proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
    '```',
    '',
    '- Scope: parent-domain release gate rows for iOS tracking privacy disclosure, App Store review, and privacy nutrition label evidence requirements.',
    '- Source inspected: WP12 iOS background/region workpack, platform expectations, location platform deep dive, and existing iOS manual-required proof.',
    '- Boundary: this proof blocks release/product-ready iOS tracking claims until disclosure, Apple review, entitlement, TestFlight/device, and runtime Core Location artifacts exist.',
    '',
  ].join('\n');
}

function validationLog(proof) {
  return proof.commands
    .map((command) =>
      [`$ ${command.command}`, command.stdout.trim(), command.stderr.trim()]
        .filter((line) => line.length > 0)
        .join('\n')
    )
    .join('\n\n');
}

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push({
    command: [command, ...args].join(' '),
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
  });
  if (result.status !== 0) {
    throw new Error(
      `Command failed: ${[command, ...args].join(' ')}\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
