import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const resultDir = join(repoRoot, 'test-results', 'tracking-report-policy-consumer-proof');
const companionDir = join(repoRoot, 'output', 'tracking-plan-proof', 'report-policy-consumer-proof');
const timestamp = '2026-06-05T20:25:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await rm(companionDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(proofDir, { recursive: true });
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
  'tracking-report-policy-consumer-proof',
  'tracking-location-policy',
]);

const consumerProof = await importDist('tracking-report-policy-consumer-proof.js');
const proofModel = consumerProof.buildTrackingReportPolicyConsumerProof(timestamp);
const proof = {
  proofMode: 'tracking-report-policy-consumer-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.rows),
  productClaims: proofModel.productClaims,
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-report-policy-consumer-proof.ts',
    test: 'packages/parent-domain/tests/tracking-report-policy-consumer-proof.test.ts',
    harness: 'scripts/test/tracking-report-policy-consumer-proof.mjs',
    evidence: 'test-results/tracking-report-policy-consumer-proof/proof.json',
    trackingProofPack:
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
    companionProofPack: 'output/tracking-plan-proof/report-policy-consumer-proof/proof.json',
  },
  rows: proofModel.rows,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'report-policy-consumer-read-model.json'), proofModel.rows);
await writeJson(join(proofDir, '22-report-policy-consumer-proof.json'), proof);
await writeCompanionPack(companionDir, proof);

console.log('tracking-report-policy-consumer-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-report-policy-consumer-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function summarize(rows) {
  return {
    rowCount: rows.length,
    consumerKinds: countBy(rows.map((row) => row.consumerKind)),
    readinessStates: countBy(rows.map((row) => row.readinessState)),
    proofTiers: countBy(rows.map((row) => row.currentProofTier)),
    policyDecisionRows: rows.filter((row) => row.policyDecision !== null).length,
    sourceProofRefRows: rows.filter((row) => row.sourceProofRefs.length > 0).length,
    productSurfaceSummaryRefRows: rows.filter((row) => row.productSurfaceSummaryRefs.length > 0).length,
    evidenceReferenceRows: rows.filter((row) => row.evidenceReferences.length > 0).length,
    storedJournalRefRows: rows.filter((row) => row.storedJournalRefs.length > 0).length,
    storedReadModelRowRefRows: rows.filter((row) => row.storedReadModelRowRefs.length > 0).length,
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
    summary.consumerKinds['parent-report-summary'] !== 1 ||
    summary.consumerKinds['policy-evidence-drill-in'] !== 1 ||
    summary.consumerKinds['retention-audit-export'] !== 1 ||
    summary.readinessStates['consumer-ready'] !== 3 ||
    summary.policyDecisionRows !== 1 ||
    summary.sourceProofRefRows !== 3 ||
    summary.productSurfaceSummaryRefRows !== 3 ||
    summary.evidenceReferenceRows !== 3 ||
    summary.storedJournalRefRows !== 3 ||
    summary.storedReadModelRowRefRows !== 3
  ) {
    throw new Error(`Unexpected tracking report policy consumer summary: ${JSON.stringify(summary)}`);
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
    throw new Error(`Tracking report policy consumer overclaimed product behavior: ${JSON.stringify(summary)}`);
  }
}

async function writeCompanionPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Report Policy Consumer Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain report and policy consumer readiness rows for the existing tracking service read model.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-report-policy-consumer-proof tracking-location-policy: PASS',
      '- Report, policy drill-in, and retention export consumer rows parse through parent-domain schemas.',
      '- Every consumer row carries stored journal refs and stored read-model row refs before report/policy use.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Consumer-readiness rows do not claim portal UI completion, child-device delivery, provider delivery, notification receipts, authority enrollment, physical-device behavior, or product completion.',
      '- The policy drill-in row requires a parsed policy decision and cited evidence references.',
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
