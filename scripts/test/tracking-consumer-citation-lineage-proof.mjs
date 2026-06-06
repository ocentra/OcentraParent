import { mkdir, rm, writeFile } from 'node:fs/promises';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const wp32ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultDir = join(repoRoot, 'test-results', 'tracking-consumer-citation-lineage-proof');
const companionDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-consumer-citation-lineage-proof');
const timestamp = '2026-06-06T15:08:00.000Z';
const proofPackName = '26-consumer-citation-lineage-proof.json';
const commands = ['node scripts/test/tracking-consumer-citation-lineage-proof.mjs'];
const initialGitStatusShort = gitOutput(['status', '--short']);

const sourceProofPaths = {
  retentionDelete: 'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/14-retention-delete-proof.json',
  serviceReadModel:
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
  productSurfaceSummary:
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
  reportPolicyConsumer:
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
  familyDashboardRollup:
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json',
  retentionSettingsReadModel:
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
  retentionSettingsWriter:
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-settings-writer-boundary-proof.json',
};

await rm(resultDir, { recursive: true, force: true });
await rm(companionDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(wp32ProofDir, { recursive: true });
await mkdir(wp33ProofDir, { recursive: true });
await mkdir(companionDir, { recursive: true });

const sourceProofs = Object.fromEntries(Object.entries(sourceProofPaths).map(([name, path]) => [name, readJson(path)]));
const proof = buildProof(sourceProofs);
assertProof(proof);

await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(wp32ProofDir, proofPackName), proof);
await writeJson(join(wp33ProofDir, proofPackName), proof);
await writeCompanionPack(companionDir, proof);

console.log('tracking-consumer-citation-lineage-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-consumer-citation-lineage-proof', 'proof.json')}`);

function buildProof(proofs) {
  const reportRows = proofs.reportPolicyConsumer.rows;
  const dashboardRows = proofs.familyDashboardRollup.rows;
  const retentionRows = proofs.retentionSettingsReadModel.rows;
  const writerRows = proofs.retentionSettingsWriter.rows;
  const lineageEdges = [
    ...reportRows.map((row) => rowLineage(row.rowId, 'report-policy-consumer', row.sourceProofRefs)),
    ...dashboardRows.map((row) =>
      rowLineage(row.rowId, 'family-dashboard-rollup', [
        ...row.sourceProofRefs,
        ...row.productSurfaceSummaryRefs,
        ...row.reportConsumerRefs,
      ])
    ),
    ...retentionRows.map((row) =>
      rowLineage(row.rowId, 'retention-settings-read-model', [
        ...row.sourceProofRefs,
        ...row.retentionProofRefs,
        ...row.readModelProofRefs,
      ])
    ),
    ...writerRows.map((row) =>
      rowLineage(row.intentId, 'retention-settings-writer-boundary', [
        ...row.sourceReadModelProofRefs,
        ...row.retentionProofRefs,
        ...row.readModelProofRefs,
      ])
    ),
  ];
  return {
    proofMode: 'tracking-consumer-citation-lineage-proof',
    generatedAt: timestamp,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    summary: summarize(proofs, lineageEdges),
    productClaims: summarizeProductClaims(proofs),
    proofPaths: {
      harness: 'scripts/test/tracking-consumer-citation-lineage-proof.mjs',
      evidence: 'test-results/tracking-consumer-citation-lineage-proof/proof.json',
      readModelProofPack:
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/26-consumer-citation-lineage-proof.json',
      proofGatePack:
        'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/26-consumer-citation-lineage-proof.json',
      companionProofPack: 'output/tracking-plan-proof/tracking-consumer-citation-lineage-proof/proof.json',
    },
    sourceProofPaths,
    lineageEdges,
  };
}

function rowLineage(rowId, consumerKind, refs) {
  const uniqueRefs = Array.from(new Set(refs));
  return {
    rowId,
    consumerKind,
    sourceProofRefs: uniqueRefs,
    existingProofRefs: uniqueRefs.filter((ref) => existsSync(join(repoRoot, ref))),
    missingProofRefs: uniqueRefs.filter((ref) => !existsSync(join(repoRoot, ref))),
  };
}

function summarize(proofs, lineageEdges) {
  return {
    reportPolicyRows: proofs.reportPolicyConsumer.rows.length,
    familyDashboardRows: proofs.familyDashboardRollup.rows.length,
    retentionSettingsReadModelRows: proofs.retentionSettingsReadModel.rows.length,
    retentionSettingsWriterRows: proofs.retentionSettingsWriter.rows.length,
    lineageEdgeCount: lineageEdges.length,
    rowsWithMissingProofRefs: lineageEdges.filter((edge) => edge.missingProofRefs.length > 0).length,
    serviceReadModelRefRows: countEdgesWith(lineageEdges, sourceProofPaths.serviceReadModel),
    productSurfaceSummaryRefRows: countEdgesWith(lineageEdges, sourceProofPaths.productSurfaceSummary),
    reportPolicyConsumerRefRows: countEdgesWith(lineageEdges, sourceProofPaths.reportPolicyConsumer),
    retentionSettingsReadModelRefRows: countEdgesWith(lineageEdges, sourceProofPaths.retentionSettingsReadModel),
    rowsWithoutEvidenceRefs:
      countRowsWithoutEvidenceRefs(proofs.reportPolicyConsumer.rows) +
      countRowsWithoutEvidenceRefs(proofs.familyDashboardRollup.rows) +
      countRowsWithoutEvidenceRefs(proofs.retentionSettingsReadModel.rows) +
      countRowsWithoutEvidenceRefs(proofs.retentionSettingsWriter.rows),
    productReadyClaimedRows: countClaimedRows(proofs, 'productClaimReady'),
    physicalDeviceClaimedRows: countClaimedRows(proofs, 'physicalDeviceClaimed'),
    authorityClaimedRows: countClaimedRows(proofs, 'authorityClaimed'),
    childDeviceDeliveryClaimedRows: countClaimedRows(proofs, 'childDeviceDeliveryClaimed'),
    providerDeliveryClaimedRows: countClaimedRows(proofs, 'providerDeliveryClaimed'),
    notificationReceiptClaimedRows: countClaimedRows(proofs, 'notificationReceiptClaimed'),
    serviceMutationExecutedRows: countClaimedRows(proofs, 'serviceMutationExecuted'),
  };
}

function summarizeProductClaims(proofs) {
  return Object.fromEntries(
    Object.entries(proofs).map(([name, proof]) => [
      name,
      Object.fromEntries(
        Object.entries(proof.productClaims ?? {}).filter(
          ([key]) => key.endsWith('Claimed') || key === 'productClaimReady' || key === 'serviceMutationExecuted'
        )
      ),
    ])
  );
}

function assertProof(proof) {
  const summary = proof.summary;
  if (
    summary.reportPolicyRows !== 3 ||
    summary.familyDashboardRows !== 3 ||
    summary.retentionSettingsReadModelRows !== 5 ||
    summary.retentionSettingsWriterRows !== 5 ||
    summary.lineageEdgeCount !== 16 ||
    summary.rowsWithMissingProofRefs !== 0 ||
    summary.rowsWithoutEvidenceRefs !== 0
  ) {
    throw new Error(`Unexpected consumer citation lineage summary: ${JSON.stringify(summary)}`);
  }
  if (
    summary.serviceReadModelRefRows < 16 ||
    summary.productSurfaceSummaryRefRows < 6 ||
    summary.reportPolicyConsumerRefRows < 3 ||
    summary.retentionSettingsReadModelRefRows < 5
  ) {
    throw new Error(`Consumer lineage lost required read-model refs: ${JSON.stringify(summary)}`);
  }
  if (
    summary.productReadyClaimedRows !== 0 ||
    summary.physicalDeviceClaimedRows !== 0 ||
    summary.authorityClaimedRows !== 0 ||
    summary.childDeviceDeliveryClaimedRows !== 0 ||
    summary.providerDeliveryClaimedRows !== 0 ||
    summary.notificationReceiptClaimedRows !== 0 ||
    summary.serviceMutationExecutedRows !== 0
  ) {
    throw new Error(`Consumer citation lineage overclaimed product behavior: ${JSON.stringify(summary)}`);
  }
}

async function writeCompanionPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Consumer Citation Lineage Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: aggregate WP32 consumer lineage proof over report/policy, family dashboard, retention read-model, and retention writer-boundary rows.',
      '- Boundary: validates stored proof refs and no-product-claim flags only; no physical-device, authority, provider delivery, child runtime, service mutation execution, or production behavior is claimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Every consumer lineage row keeps existing proof refs present on disk.',
      '- No row claims product readiness, physical-device behavior, authority enrollment, child-device delivery, provider delivery, notification receipt, or executed service mutation.',
      '- Remote/provider/device behavior remains represented only by existing manual-required proof rows.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeJson(join(path, 'proof.json'), proof);
}

function countEdgesWith(edges, ref) {
  return edges.filter((edge) => edge.sourceProofRefs.includes(ref)).length;
}

function countRowsWithoutEvidenceRefs(rows) {
  return rows.filter((row) => !Array.isArray(row.evidenceReferences) || row.evidenceReferences.length === 0).length;
}

function countClaimedRows(proofs, field) {
  return Object.values(proofs).reduce((total, proof) => {
    const rows = Array.isArray(proof.rows) ? proof.rows : [];
    return total + rows.filter((row) => row[field] === true).length;
  }, 0);
}

function readJson(path) {
  const fullPath = join(repoRoot, path);
  return JSON.parse(readFileSync(fullPath, 'utf8'));
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
