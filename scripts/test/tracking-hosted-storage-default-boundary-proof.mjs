import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const timestamp = '2026-06-07T06:12:00.000Z';
const resultDir = join(repoRoot, 'test-results', 'tracking-hosted-storage-default-boundary-proof');
const wp32Dir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const wp33Dir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(wp32Dir, { recursive: true });
await mkdir(wp33Dir, { recursive: true });

runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/tracking-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-hosted-storage-default-boundary-proof.test.ts',
  'tracking-retention-settings-read-model-proof.test.ts',
  'tracking-ai-stored-ref-consumer-proof.test.ts',
]);

const storageBoundary = await importDist('tracking-hosted-storage-default-boundary-proof.js');
const proofModel = storageBoundary.buildTrackingHostedStorageDefaultBoundaryProof(timestamp);
const proof = {
  proofMode: 'tracking-hosted-storage-default-boundary-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.rows),
  productClaims: proofModel.productClaims,
  proofPaths: {
    source: 'packages/tracking-domain/src/tracking-hosted-storage-default-boundary-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-hosted-storage-default-boundary-proof.test.ts',
    harness: 'scripts/test/tracking-hosted-storage-default-boundary-proof.mjs',
    evidence: 'test-results/tracking-hosted-storage-default-boundary-proof/proof.json',
    wp32ProofPack:
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/31-hosted-storage-default-boundary-proof.json',
    wp33ProofPack:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/38-hosted-storage-default-boundary-proof.json',
  },
  rows: proofModel.rows,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'hosted-storage-default-boundary-read-model.json'), proofModel.rows);
await writeJson(join(wp32Dir, '31-hosted-storage-default-boundary-proof.json'), proof);
await writeJson(join(wp33Dir, '38-hosted-storage-default-boundary-proof.json'), proof);

console.log('tracking-hosted-storage-default-boundary-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-hosted-storage-default-boundary-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'tracking-domain', 'dist', name)).href);
}

function summarize(rows) {
  return {
    rowCount: rows.length,
    boundaryKinds: countBy(rows.map((row) => row.boundaryKind)),
    boundaryStates: countBy(rows.map((row) => row.boundaryState)),
    proofTiers: countBy(rows.map((row) => row.currentProofTier)),
    defaultCustody: countBy(rows.map((row) => row.defaultCustody)),
    sourceProofRefRows: rows.filter((row) => row.sourceProofRefs.length > 0).length,
    journalProofRefRows: rows.filter((row) => row.journalProofRefs.length > 0).length,
    readModelProofRefRows: rows.filter((row) => row.readModelProofRefs.length > 0).length,
    retentionProofRefRows: rows.filter((row) => row.retentionProofRefs.length > 0).length,
    aiConsumerProofRefRows: rows.filter((row) => row.aiConsumerProofRefs.length > 0).length,
    evidenceReferenceRows: rows.filter((row) => row.evidenceReferences.length > 0).length,
    parentOwnedExportRequiredRows: rows.filter((row) => row.parentOwnedExportRequired).length,
    storedRefConsumerRequiredRows: rows.filter((row) => row.storedRefConsumerRequired).length,
    ocentraHostedStorageDefaultRows: rows.filter((row) => row.ocentraHostedStorageDefault).length,
    rawLocationRemoteUploadEnabledRows: rows.filter((row) => row.rawLocationRemoteUploadEnabled).length,
    sqliteSnapshotRemoteUploadEnabledRows: rows.filter((row) => row.sqliteSnapshotRemoteUploadEnabled).length,
    remoteSyncEnabledRows: rows.filter((row) => row.remoteSyncEnabled).length,
    remoteAiEnabledRows: rows.filter((row) => row.remoteAiEnabled).length,
    portalUiClaimedRows: rows.filter((row) => row.portalUiClaimed).length,
    serviceMutationClaimedRows: rows.filter((row) => row.serviceMutationClaimed).length,
    platformRuntimeClaimedRows: rows.filter((row) => row.platformRuntimeClaimed).length,
    childDeviceDeliveryClaimedRows: rows.filter((row) => row.childDeviceDeliveryClaimed).length,
    providerDeliveryClaimedRows: rows.filter((row) => row.providerDeliveryClaimed).length,
    notificationReceiptClaimedRows: rows.filter((row) => row.notificationReceiptClaimed).length,
    physicalDeviceClaimedRows: rows.filter((row) => row.physicalDeviceClaimed).length,
    authorityClaimedRows: rows.filter((row) => row.authorityClaimed).length,
    productionBehaviorClaimedRows: rows.filter((row) => row.productionBehaviorClaimed).length,
    productClaimReadyRows: rows.filter((row) => row.productClaimReady).length,
  };
}

function assertProof(proof) {
  const summary = proof.summary;
  if (
    summary.rowCount !== 5 ||
    summary.boundaryKinds['journal-local-default'] !== 1 ||
    summary.boundaryKinds['sqlite-read-model-local-default'] !== 1 ||
    summary.boundaryKinds['parent-export-local-default'] !== 1 ||
    summary.boundaryKinds['ai-context-stored-ref-local-default'] !== 1 ||
    summary.boundaryKinds['remote-sync-disabled-default'] !== 1 ||
    summary.boundaryStates['hosted-storage-not-default'] !== 5 ||
    summary.sourceProofRefRows !== 5 ||
    summary.journalProofRefRows !== 5 ||
    summary.readModelProofRefRows !== 5 ||
    summary.retentionProofRefRows !== 5 ||
    summary.aiConsumerProofRefRows !== 1 ||
    summary.evidenceReferenceRows !== 5 ||
    summary.parentOwnedExportRequiredRows !== 1 ||
    summary.storedRefConsumerRequiredRows !== 1 ||
    summary.defaultCustody['remote-disabled'] !== 1
  ) {
    throw new Error(`Unexpected tracking hosted storage default summary: ${JSON.stringify(summary)}`);
  }

  if (
    summary.ocentraHostedStorageDefaultRows !== 0 ||
    summary.rawLocationRemoteUploadEnabledRows !== 0 ||
    summary.sqliteSnapshotRemoteUploadEnabledRows !== 0 ||
    summary.remoteSyncEnabledRows !== 0 ||
    summary.remoteAiEnabledRows !== 0 ||
    summary.portalUiClaimedRows !== 0 ||
    summary.serviceMutationClaimedRows !== 0 ||
    summary.platformRuntimeClaimedRows !== 0 ||
    summary.childDeviceDeliveryClaimedRows !== 0 ||
    summary.providerDeliveryClaimedRows !== 0 ||
    summary.notificationReceiptClaimedRows !== 0 ||
    summary.physicalDeviceClaimedRows !== 0 ||
    summary.authorityClaimedRows !== 0 ||
    summary.productionBehaviorClaimedRows !== 0 ||
    summary.productClaimReadyRows !== 0 ||
    Object.values(proof.productClaims).some((value) => value !== false)
  ) {
    throw new Error(`Tracking hosted storage default proof overclaimed behavior: ${JSON.stringify(summary)}`);
  }
}

function run(command, args) {
  commands.push([command, ...args].join(' '));
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
