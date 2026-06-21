import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const wp07ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const wp32ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultDir = join(repoRoot, 'test-results', 'tracking-retention-settings-writer-boundary-proof');
const companionDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  'tracking-retention-settings-writer-boundary-proof'
);
const timestamp = '2026-06-06T13:17:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await rm(companionDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(wp07ProofDir, { recursive: true });
await mkdir(wp32ProofDir, { recursive: true });
await mkdir(wp33ProofDir, { recursive: true });
await mkdir(companionDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-retention-settings-writer-boundary-proof',
  'tracking-retention-settings-read-model-proof',
]);

const retentionSettingsProof = await importDist('tracking-retention-settings-writer-boundary-proof.js');
const proofModel = retentionSettingsProof.buildTrackingRetentionSettingsWriterBoundaryProof(timestamp);
const proof = {
  proofMode: 'tracking-retention-settings-writer-boundary-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.rows),
  proofClaims: proofModel.proofClaims,
  productClaims: proofModel.productClaims,
  proofPaths: {
    source: 'packages/schema-domain/src/tracking-retention-settings-writer-boundary-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-retention-settings-writer-boundary-proof.test.ts',
    harness: 'scripts/test/tracking-retention-settings-writer-boundary-proof.mjs',
    evidence: 'test-results/tracking-retention-settings-writer-boundary-proof/proof.json',
    retentionProofPack:
      'output/tracking-plan-proof/07-retention-and-custody-model/19-retention-settings-writer-boundary-proof.json',
    readModelProofPack:
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-settings-writer-boundary-proof.json',
    proofGatePack:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/25-retention-settings-writer-boundary-proof.json',
    companionProofPack: 'output/tracking-plan-proof/tracking-retention-settings-writer-boundary-proof/proof.json',
  },
  rows: proofModel.rows,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'retention-settings-writer-boundary.json'), proofModel.rows);
await writeJson(join(wp07ProofDir, '19-retention-settings-writer-boundary-proof.json'), proof);
await writeJson(join(wp32ProofDir, '25-retention-settings-writer-boundary-proof.json'), proof);
await writeJson(join(wp33ProofDir, '25-retention-settings-writer-boundary-proof.json'), proof);
await writeCompanionPack(companionDir, proof);

console.log('tracking-retention-settings-writer-boundary-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-retention-settings-writer-boundary-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function summarize(rows) {
  return {
    rowCount: rows.length,
    settingsKinds: countBy(rows.map((row) => row.settingsKind)),
    writeActions: countBy(rows.map((row) => row.writeAction)),
    writerStates: countBy(rows.map((row) => row.writerState)),
    retentionWindowRows: rows.filter((row) => row.requestedRetentionWindowHours !== null).length,
    deleteAfterAlertRows: rows.filter((row) => row.requestedDeleteAfterAlertResolved).length,
    parentExportRows: rows.filter((row) => row.requestedParentExport).length,
    requestedRemoteSyncEnabledRows: rows.filter((row) => row.requestedRemoteSyncEnabled).length,
    requestedRemoteAiEnabledRows: rows.filter((row) => row.requestedRemoteAiEnabled).length,
    sourceReadModelProofRefRows: rows.filter((row) => row.sourceReadModelProofRefs.length > 0).length,
    retentionProofRefRows: rows.filter((row) => row.retentionProofRefs.length > 0).length,
    readModelProofRefRows: rows.filter((row) => row.readModelProofRefs.length > 0).length,
    evidenceReferenceRows: rows.filter((row) => row.evidenceReferences.length > 0).length,
    auditRefRows: rows.filter((row) => row.auditRefs.length > 0).length,
    localValidationClaimedRows: rows.filter((row) => row.localValidationClaimed).length,
    writerBoundaryClaimedRows: rows.filter((row) => row.writerBoundaryClaimed).length,
    serviceMutationPreflightClaimedRows: rows.filter((row) => row.serviceMutationPreflightClaimed).length,
    serviceMutationExecutedRows: rows.filter((row) => row.serviceMutationExecuted).length,
    portalUiClaimedRows: rows.filter((row) => row.portalUiClaimed).length,
    platformRuntimeClaimedRows: rows.filter((row) => row.platformRuntimeClaimed).length,
    childDeviceDeliveryClaimedRows: rows.filter((row) => row.childDeviceDeliveryClaimed).length,
    providerDeliveryClaimedRows: rows.filter((row) => row.providerDeliveryClaimed).length,
    notificationReceiptClaimedRows: rows.filter((row) => row.notificationReceiptClaimed).length,
    physicalDeviceClaimedRows: rows.filter((row) => row.physicalDeviceClaimed).length,
    authorityClaimedRows: rows.filter((row) => row.authorityClaimed).length,
    productClaimReadyRows: rows.filter((row) => row.productClaimReady).length,
  };
}

function assertProof(proof) {
  const summary = proof.summary;
  if (
    summary.rowCount !== 5 ||
    summary.settingsKinds['retention-window-setting'] !== 1 ||
    summary.settingsKinds['delete-after-alert-setting'] !== 1 ||
    summary.settingsKinds['parent-export-setting'] !== 1 ||
    summary.settingsKinds['remote-sync-disabled-setting'] !== 1 ||
    summary.settingsKinds['remote-ai-disabled-setting'] !== 1 ||
    summary.writeActions['set-retention-window'] !== 1 ||
    summary.writeActions['enable-delete-after-alert'] !== 1 ||
    summary.writeActions['prepare-parent-export'] !== 1 ||
    summary.writeActions['keep-remote-sync-disabled'] !== 1 ||
    summary.writeActions['keep-remote-ai-disabled'] !== 1 ||
    summary.writerStates['writer-preflight-ready'] !== 5 ||
    summary.retentionWindowRows !== 1 ||
    summary.deleteAfterAlertRows !== 1 ||
    summary.parentExportRows !== 1 ||
    summary.requestedRemoteSyncEnabledRows !== 0 ||
    summary.requestedRemoteAiEnabledRows !== 0 ||
    summary.sourceReadModelProofRefRows !== 5 ||
    summary.retentionProofRefRows !== 5 ||
    summary.readModelProofRefRows !== 5 ||
    summary.evidenceReferenceRows !== 5 ||
    summary.auditRefRows !== 5 ||
    summary.localValidationClaimedRows !== 5 ||
    summary.writerBoundaryClaimedRows !== 5 ||
    summary.serviceMutationPreflightClaimedRows !== 5
  ) {
    throw new Error(`Unexpected tracking retention writer summary: ${JSON.stringify(summary)}`);
  }

  if (
    summary.serviceMutationExecutedRows !== 0 ||
    summary.portalUiClaimedRows !== 0 ||
    summary.platformRuntimeClaimedRows !== 0 ||
    summary.childDeviceDeliveryClaimedRows !== 0 ||
    summary.providerDeliveryClaimedRows !== 0 ||
    summary.notificationReceiptClaimedRows !== 0 ||
    summary.physicalDeviceClaimedRows !== 0 ||
    summary.authorityClaimedRows !== 0 ||
    summary.productClaimReadyRows !== 0 ||
    Object.values(proof.productClaims).some((value) => value !== false)
  ) {
    throw new Error(`Tracking retention writer proof overclaimed product behavior: ${JSON.stringify(summary)}`);
  }
}

async function writeCompanionPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Retention Settings Writer Boundary Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: schema-domain retention settings writer intent/preflight rows consuming existing read-model proof refs.',
      '- Source inspected: location/geofence feature doc, WP07, WP32, and retention settings read-model proof.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/schema-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tracking-retention-settings-writer-boundary-proof tracking-retention-settings-read-model-proof: PASS',
      '- Writer rows parse through schema-domain schemas for retention window, delete-after-alert, parent export, remote sync disabled, and remote AI disabled settings.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Writer rows claim local validation and writer-boundary preflight only.',
      '- Writer rows do not claim executed service mutation, portal UI completion, platform runtime, child-device delivery, provider delivery, notification receipts, authority enrollment, physical-device behavior, or product completion.',
      '- Remote sync and remote AI remain disabled in every requested writer row.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeJson(join(path, 'proof.json'), proof);
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

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
