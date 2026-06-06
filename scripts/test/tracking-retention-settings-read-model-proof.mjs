import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const wp07ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const wp32ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const resultDir = join(repoRoot, 'test-results', 'tracking-retention-settings-read-model-proof');
const companionDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-retention-settings-read-model-proof');
const timestamp = '2026-06-06T05:24:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await rm(companionDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(wp07ProofDir, { recursive: true });
await mkdir(wp32ProofDir, { recursive: true });
await mkdir(companionDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-retention-settings-read-model-proof',
  'tracking-location-policy',
]);

const retentionSettingsProof = await importDist('tracking-retention-settings-read-model-proof.js');
const proofModel = retentionSettingsProof.buildTrackingRetentionSettingsReadModelProof(timestamp);
const proof = {
  proofMode: 'tracking-retention-settings-read-model-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.rows),
  productClaims: proofModel.productClaims,
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-retention-settings-read-model-proof.ts',
    test: 'packages/parent-domain/tests/tracking-retention-settings-read-model-proof.test.ts',
    harness: 'scripts/test/tracking-retention-settings-read-model-proof.mjs',
    evidence: 'test-results/tracking-retention-settings-read-model-proof/proof.json',
    retentionProofPack:
      'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
    readModelProofPack:
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
    companionProofPack: 'output/tracking-plan-proof/tracking-retention-settings-read-model-proof/proof.json',
  },
  rows: proofModel.rows,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'retention-settings-read-model.json'), proofModel.rows);
await writeJson(join(wp07ProofDir, '18-retention-settings-read-model-proof.json'), proof);
await writeJson(join(wp32ProofDir, '24-retention-settings-read-model-proof.json'), proof);
await writeCompanionPack(companionDir, proof);

console.log('tracking-retention-settings-read-model-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-retention-settings-read-model-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function summarize(rows) {
  return {
    rowCount: rows.length,
    settingsKinds: countBy(rows.map((row) => row.settingsKind)),
    settingsStates: countBy(rows.map((row) => row.settingsState)),
    proofTiers: countBy(rows.map((row) => row.currentProofTier)),
    custodyScopes: countBy(rows.map((row) => row.custodyScope)),
    retentionWindowRows: rows.filter((row) => row.retentionWindowHours !== null).length,
    deleteAfterAlertRows: rows.filter((row) => row.deleteAfterAlertResolved).length,
    parentExportReadyRows: rows.filter((row) => row.parentExportReady).length,
    remoteSyncEnabledRows: rows.filter((row) => row.remoteSyncEnabled).length,
    remoteAiEnabledRows: rows.filter((row) => row.remoteAiEnabled).length,
    sourceProofRefRows: rows.filter((row) => row.sourceProofRefs.length > 0).length,
    retentionProofRefRows: rows.filter((row) => row.retentionProofRefs.length > 0).length,
    readModelProofRefRows: rows.filter((row) => row.readModelProofRefs.length > 0).length,
    evidenceReferenceRows: rows.filter((row) => row.evidenceReferences.length > 0).length,
    portalUiClaimedRows: rows.filter((row) => row.portalUiClaimed).length,
    serviceMutationClaimedRows: rows.filter((row) => row.serviceMutationClaimed).length,
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
    summary.settingsStates['settings-read-model-ready'] !== 5 ||
    summary.retentionWindowRows !== 1 ||
    summary.deleteAfterAlertRows !== 1 ||
    summary.parentExportReadyRows !== 1 ||
    summary.remoteSyncEnabledRows !== 0 ||
    summary.remoteAiEnabledRows !== 0 ||
    summary.sourceProofRefRows !== 5 ||
    summary.retentionProofRefRows !== 5 ||
    summary.readModelProofRefRows !== 5 ||
    summary.evidenceReferenceRows !== 5
  ) {
    throw new Error(`Unexpected tracking retention settings summary: ${JSON.stringify(summary)}`);
  }

  if (
    summary.portalUiClaimedRows !== 0 ||
    summary.serviceMutationClaimedRows !== 0 ||
    summary.platformRuntimeClaimedRows !== 0 ||
    summary.childDeviceDeliveryClaimedRows !== 0 ||
    summary.providerDeliveryClaimedRows !== 0 ||
    summary.notificationReceiptClaimedRows !== 0 ||
    summary.physicalDeviceClaimedRows !== 0 ||
    summary.authorityClaimedRows !== 0 ||
    summary.productClaimReadyRows !== 0 ||
    Object.values(proof.productClaims).some((value) => value !== false)
  ) {
    throw new Error(`Tracking retention settings proof overclaimed product behavior: ${JSON.stringify(summary)}`);
  }
}

async function writeCompanionPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Retention Settings Read-Model Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain retention settings read-model rows for existing WP07 retention/delete/export and WP32 service read-model proof refs.',
      '- Source inspected: feature list, location/geofence feature doc, location/geofence expectations, platform expectations, parent-domain README, WP07, and WP32.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-retention-settings-read-model-proof tracking-location-policy: PASS',
      '- Retention window, delete-after-alert, parent export, remote sync disabled, and remote AI disabled rows parse through parent-domain schemas.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Retention settings rows do not claim portal UI completion, service mutation, platform runtime, child-device delivery, provider delivery, notification receipts, authority enrollment, physical-device behavior, or product completion.',
      '- Remote sync and remote AI stay disabled in every row.',
      '- Parent export readiness is parent-owned and does not claim Ocentra-hosted custody.',
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
