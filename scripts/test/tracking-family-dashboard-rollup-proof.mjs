import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const resultDir = join(repoRoot, 'test-results', 'tracking-family-dashboard-rollup-proof');
const companionDir = join(repoRoot, 'output', 'tracking-plan-proof', 'family-dashboard-rollup-proof');
const timestamp = '2026-06-05T23:10:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await rm(companionDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(proofDir, { recursive: true });
await mkdir(companionDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-family-dashboard-rollup-proof',
  'tracking-location-policy',
]);

const rollupProof = await importDist('tracking-family-dashboard-rollup-proof.js');
const proofModel = rollupProof.buildTrackingFamilyDashboardRollupProof(timestamp);
const proof = {
  proofMode: 'tracking-family-dashboard-rollup-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.rows),
  productClaims: proofModel.productClaims,
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-family-dashboard-rollup-proof.ts',
    test: 'packages/parent-domain/tests/tracking-family-dashboard-rollup-proof.test.ts',
    harness: 'scripts/test/tracking-family-dashboard-rollup-proof.mjs',
    evidence: 'test-results/tracking-family-dashboard-rollup-proof/proof.json',
    trackingProofPack:
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json',
    companionProofPack: 'output/tracking-plan-proof/family-dashboard-rollup-proof/proof.json',
  },
  rows: proofModel.rows,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'family-dashboard-rollup-read-model.json'), proofModel.rows);
await writeJson(join(proofDir, '23-family-dashboard-rollup-proof.json'), proof);
await writeCompanionPack(companionDir, proof);

console.log('tracking-family-dashboard-rollup-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-family-dashboard-rollup-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function summarize(rows) {
  return {
    rowCount: rows.length,
    rollupKinds: countBy(rows.map((row) => row.rollupKind)),
    rollupStates: countBy(rows.map((row) => row.rollupState)),
    proofTiers: countBy(rows.map((row) => row.currentProofTier)),
    visibleChildCount: sum(rows.map((row) => row.visibleChildCount)),
    attentionItemCount: sum(rows.map((row) => row.attentionItemCount)),
    retainedAuditItemCount: sum(rows.map((row) => row.retainedAuditItemCount)),
    sourceProofRefRows: rows.filter((row) => row.sourceProofRefs.length > 0).length,
    productSurfaceSummaryRefRows: rows.filter((row) => row.productSurfaceSummaryRefs.length > 0).length,
    reportConsumerRefRows: rows.filter((row) => row.reportConsumerRefs.length > 0).length,
    evidenceReferenceRows: rows.filter((row) => row.evidenceReferences.length > 0).length,
    portalUiClaimedRows: rows.filter((row) => row.portalUiClaimed).length,
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
    summary.rowCount !== 3 ||
    summary.rollupKinds['family-active-summary'] !== 1 ||
    summary.rollupKinds['child-attention-summary'] !== 1 ||
    summary.rollupKinds['retention-audit-summary'] !== 1 ||
    summary.rollupStates['rollup-ready'] !== 3 ||
    summary.visibleChildCount !== 3 ||
    summary.attentionItemCount !== 3 ||
    summary.retainedAuditItemCount !== 2 ||
    summary.sourceProofRefRows !== 3 ||
    summary.productSurfaceSummaryRefRows !== 3 ||
    summary.reportConsumerRefRows !== 3 ||
    summary.evidenceReferenceRows !== 3
  ) {
    throw new Error(`Unexpected tracking family dashboard rollup summary: ${JSON.stringify(summary)}`);
  }

  if (
    summary.portalUiClaimedRows !== 0 ||
    summary.childDeviceDeliveryClaimedRows !== 0 ||
    summary.providerDeliveryClaimedRows !== 0 ||
    summary.notificationReceiptClaimedRows !== 0 ||
    summary.physicalDeviceClaimedRows !== 0 ||
    summary.authorityClaimedRows !== 0 ||
    summary.productClaimReadyRows !== 0 ||
    Object.values(proof.productClaims).some((value) => value !== false)
  ) {
    throw new Error(`Tracking family dashboard rollup overclaimed product behavior: ${JSON.stringify(summary)}`);
  }
}

async function writeCompanionPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Family Dashboard Rollup Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain family dashboard rollup rows for existing tracking service read-model/product summary/report consumer proof refs.',
      '- Source inspected: feature list, location/geofence feature doc, location/geofence expectations, platform expectations, parent-domain README, and WP32 checklist rows.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-family-dashboard-rollup-proof tracking-location-policy: PASS',
      '- Family active, child attention, and retention audit rollup rows parse through parent-domain schemas.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Rollup rows do not claim portal UI completion, child-device delivery, provider delivery, notification receipts, authority enrollment, physical-device behavior, or product completion.',
      '- The child attention row must expose a non-zero attention count instead of hiding attention behind a calm summary.',
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

function sum(values) {
  return values.reduce((total, value) => total + value, 0);
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
