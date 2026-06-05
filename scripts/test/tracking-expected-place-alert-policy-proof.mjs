import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const expectedPlaceProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '16-expected-place-schedule-engine');
const alertProofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '26-alert-severity-and-notification-model');
const resultRoot = join(repoRoot, 'test-results', 'tracking-expected-place-alert-policy-proof');
const generatedAt = '2026-06-05T22:00:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultRoot, { recursive: true, force: true });
await mkdir(expectedPlaceProofRoot, { recursive: true });
await mkdir(alertProofRoot, { recursive: true });
await mkdir(resultRoot, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-expected-place-alert-policy-proof',
]);

const proofModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-expected-place-alert-policy-proof.js'))
    .href
);
const readModel = proofModule.buildTrackingExpectedPlaceAlertPolicyProof({
  generatedAt,
  proofId: 'tracking-expected-place-alert-policy-proof',
  sourceExpectedPlaceProofRef:
    'output/tracking-plan-proof/16-expected-place-schedule-engine/06-expected-place-proof.json',
  sourcePolicyCompilerProofRef: 'output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/proof.json',
  sourceNotificationBoundaryRef:
    'output/tracking-plan-proof/26-alert-severity-and-notification-model/proof-summary.json',
});
const proof = {
  proofMode: 'tracking-expected-place-alert-policy-proof',
  generatedAt,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  productClaims: {
    providerDeliveryClaimed: readModel.providerDeliveryClaimed,
    notificationReceiptClaimed: readModel.notificationReceiptClaimed,
    parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    platformAdapterRuntimeClaimed: readModel.platformAdapterRuntimeClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    productionWorkerClaimed: readModel.productionWorkerClaimed,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-expected-place-alert-policy-proof.ts',
    test: 'packages/parent-domain/tests/tracking-expected-place-alert-policy-proof.test.ts',
    harness: 'scripts/test/tracking-expected-place-alert-policy-proof.mjs',
    evidence: 'test-results/tracking-expected-place-alert-policy-proof/proof.json',
    expectedPlaceProof:
      'output/tracking-plan-proof/16-expected-place-schedule-engine/expected-place-alert-policy-proof.json',
    alertProof:
      'output/tracking-plan-proof/26-alert-severity-and-notification-model/expected-place-alert-policy-proof.json',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(resultRoot, 'proof.json'), proof);
await writeJson(join(resultRoot, 'expected-place-alert-policy-read-model.json'), readModel);
await writeJson(join(expectedPlaceProofRoot, 'expected-place-alert-policy-proof.json'), proof);
await writeJson(join(alertProofRoot, 'expected-place-alert-policy-proof.json'), proof);

console.log('tracking-expected-place-alert-policy-proof-ok');
console.log('evidence=test-results/tracking-expected-place-alert-policy-proof/proof.json');

function summarize(readModel) {
  return {
    rowCount: readModel.rows.length,
    alertReadyCount: readModel.alertReadyCount,
    manualRequiredCount: readModel.manualRequiredCount,
    suppressedCount: readModel.suppressedCount,
    noAlertExpectedCount: readModel.noAlertExpectedCount,
    providerDeliveryClaimedRows: readModel.rows.filter((row) => row.providerDeliveryClaimed).length,
    notificationReceiptClaimedRows: readModel.rows.filter((row) => row.notificationReceiptClaimed).length,
    physicalDeviceProofClaimedRows: readModel.rows.filter((row) => row.physicalDeviceProofClaimed).length,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rowCount !== 4 ||
    proof.summary.alertReadyCount !== 1 ||
    proof.summary.manualRequiredCount !== 1 ||
    proof.summary.suppressedCount !== 1 ||
    proof.summary.noAlertExpectedCount !== 1 ||
    proof.summary.providerDeliveryClaimedRows !== 0 ||
    proof.summary.notificationReceiptClaimedRows !== 0 ||
    proof.summary.physicalDeviceProofClaimedRows !== 0
  ) {
    throw new Error(`Unexpected expected-place alert proof summary: ${JSON.stringify(proof.summary)}`);
  }

  if (Object.values(proof.productClaims).some((claim) => claim !== false)) {
    throw new Error(`Expected-place alert proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
  }
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
