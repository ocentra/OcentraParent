import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-real-runtime-handoff-proof';
const resultRoot = join(repoRoot, 'test-results', proofMode);
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp33Root = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const generatedAt = '2026-06-08T01:05:00.000Z';

const gateProofs = [
  {
    handoffArea: 'physical-device-background-and-geofence',
    proofRef: 'test-results/tracking-physical-device-artifact-gate-proof/proof.json',
    requiredTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'child-device-runtime-execution',
    proofRef: 'test-results/tracking-child-runtime-artifact-gate-proof/proof.json',
    requiredTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'full-product-parent-child-ui-runtime',
    proofRef: 'test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json',
    requiredTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'authority-enrolled-hard-control-runtime',
    proofRef: 'test-results/tracking-authority-runtime-artifact-gate-proof/proof.json',
    requiredTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'provider-delivery-receipt-runtime',
    proofRef: 'test-results/tracking-provider-delivery-artifact-gate-proof/proof.json',
    requiredTier: 'P4_MANUAL_PROVIDER_RUNTIME',
  },
  {
    handoffArea: 'retention-product-runtime-enforcement',
    proofRef: 'test-results/tracking-retention-runtime-artifact-gate-proof/proof.json',
    requiredTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'production-durable-workers-and-storage',
    proofRef: 'test-results/tracking-production-worker-runtime-artifact-gate-proof/proof.json',
    requiredTier: 'P4_PRODUCTION_RUNTIME',
  },
  {
    handoffArea: 'escalation-runtime-workers-and-storage',
    proofRef: 'test-results/tracking-escalation-runtime-artifact-gate-proof/proof.json',
    requiredTier: 'P4_PRODUCTION_RUNTIME',
  },
];

const requiredClosureBlockers = [
  'android-physical-background-proof-required',
  'ios-physical-region-proof-required',
  'retention-writable-product-settings-required',
  'retention-platform-runtime-enforcement-required',
  'actual-child-device-runtime-required',
  'full-product-parent-child-ui-required',
  'authority-enrollment-proof-required',
  'provider-delivery-receipt-runtime-required',
  'production-durable-workers-required',
];

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await rm(proofRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(proofRoot, { recursive: true });
  await mkdir(wp33Root, { recursive: true });

  const proof = await buildProof();
  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-real-runtime-handoff-proof-ok');
  console.log('evidence=test-results/tracking-real-runtime-handoff-proof/proof.json');
}

async function buildProof() {
  const closure = await readJson('test-results/tracking-product-readiness-closure-proof/proof.json');
  const gateRows = [];
  for (const gate of gateProofs) {
    const sourceProof = await readJson(gate.proofRef);
    gateRows.push(...handoffRowsFrom(gate, sourceProof));
  }
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    requiredProofTier: 'P4_REAL_RUNTIME_HANDOFF',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'manual_required',
    sourceGateRefs: gateProofs.map((gate) => gate.proofRef),
    closureProofRef: 'test-results/tracking-product-readiness-closure-proof/proof.json',
    handoffRows: gateRows,
    summary: summarize(gateRows),
    remainingProductBlockers: closure.rows?.[0]?.remainingBlockers ?? [],
    productClaims: {
      physicalDeviceClaimed: false,
      actualChildDeviceRuntimeClaimed: false,
      fullProductUiClaimed: false,
      authorityClaimed: false,
      providerDeliveryReceiptRuntimeClaimed: false,
      retentionProductRuntimeClaimed: false,
      productionWorkersClaimed: false,
      escalationRuntimeClaimed: false,
      productReadyClaimed: false,
    },
    proofPaths: {
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/63-real-runtime-handoff-proof.json',
      evidence: 'test-results/tracking-real-runtime-handoff-proof/proof.json',
      namedProofRoot: 'output/tracking-plan-proof/tracking-real-runtime-handoff-proof/proof.json',
    },
  };
}

function handoffRowsFrom(gate, sourceProof) {
  const rows = rowsFrom(sourceProof);
  return rows.map((row) => ({
    handoffArea: gate.handoffArea,
    sourceProofRef: gate.proofRef,
    rowId: row.rowId ?? `${gate.handoffArea}-row`,
    proofRoot: row.proofRoot ?? '',
    requiredProofTier: row.requiredProofTier ?? gate.requiredTier,
    currentProofTier: row.currentProofTier ?? sourceProof.currentProofTier ?? 'P3_LOCAL_DEV_MACHINE',
    status: row.status ?? sourceProof.status ?? 'manual-required',
    requiredArtifacts: row.requiredArtifacts ?? [],
    presentArtifacts: row.presentArtifacts ?? [],
    missingArtifacts: row.missingArtifacts ?? [],
    auditRefs: row.auditRefs ?? [],
    productClaimReady: row.productClaimReady === true,
  }));
}

function rowsFrom(sourceProof) {
  if (Array.isArray(sourceProof.readModel?.rows)) return sourceProof.readModel.rows;
  if (Array.isArray(sourceProof.rows)) return sourceProof.rows;
  throw new Error(`Artifact gate proof has no rows: ${sourceProof.proofMode ?? 'unknown'}`);
}

function summarize(rows) {
  const requiredArtifactCount = rows.reduce((total, row) => total + row.requiredArtifacts.length, 0);
  const missingArtifactCount = rows.reduce((total, row) => total + row.missingArtifacts.length, 0);
  return {
    handoffRowCount: rows.length,
    requiredArtifactCount,
    presentArtifactCount: rows.reduce((total, row) => total + row.presentArtifacts.length, 0),
    missingArtifactCount,
    manualRequiredRowCount: rows.filter((row) => row.status === 'manual-required').length,
    artifactSetPresentRowCount: rows.filter((row) => row.status === 'artifact-set-present').length,
    productReadyRowCount: rows.filter((row) => row.productClaimReady).length,
  };
}

function assertProof(proof) {
  const blockerSet = new Set(proof.remainingProductBlockers);
  const missingBlockers = requiredClosureBlockers.filter((blocker) => !blockerSet.has(blocker));
  if (missingBlockers.length > 0) {
    throw new Error(`Real-runtime handoff is missing closure blockers: ${missingBlockers.join(', ')}`);
  }
  if (proof.handoffRows.length < gateProofs.length) {
    throw new Error(`Expected at least one handoff row per gate proof: ${proof.handoffRows.length}`);
  }
  const emptyArtifactRows = proof.handoffRows.filter((row) => row.requiredArtifacts.length === 0);
  if (emptyArtifactRows.length > 0) {
    throw new Error(`Real-runtime handoff rows need required artifacts: ${JSON.stringify(emptyArtifactRows)}`);
  }
  if (proof.summary.missingArtifactCount === 0 || proof.summary.manualRequiredRowCount === 0) {
    throw new Error(
      `Real-runtime handoff unexpectedly has no missing/manual artifacts: ${JSON.stringify(proof.summary)}`
    );
  }
  if (Object.values(proof.productClaims).some(Boolean) || proof.summary.productReadyRowCount > 0) {
    throw new Error(`Real-runtime handoff overclaimed product readiness: ${JSON.stringify(proof.productClaims)}`);
  }
}

async function writeArtifacts(proof) {
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(proofRoot, 'proof.json'), proof);
  await writeJson(join(wp33Root, '63-real-runtime-handoff-proof.json'), proof);
  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot(proof));
  await writeFile(join(proofRoot, '16-validation-commands.log'), validationLog());
}

function sourceSnapshot(proof) {
  return [
    '# Tracking Real Runtime Handoff Source Snapshot',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- requiredProofTier: P4_REAL_RUNTIME_HANDOFF',
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    '- status: manual_required',
    '- proves real-runtime handoff artifact requirements are derived from existing gates',
    '- does not prove physical-device, child-device runtime, authority, provider, retention product runtime, escalation, production, or product-ready tracking behavior',
    '',
    '## Handoff Areas',
    '',
    ...proof.handoffRows.map(
      (row) => `- ${row.handoffArea}: ${row.missingArtifacts.length}/${row.requiredArtifacts.length} artifacts missing`
    ),
    '',
  ].join('\n');
}

function validationLog() {
  return [
    'node scripts/test/tracking-product-readiness-closure-proof.mjs exit=0',
    'node scripts/test/tracking-real-runtime-handoff-proof.mjs exit=0',
    '',
  ].join('\n');
}

async function readJson(path) {
  return JSON.parse(await readFile(join(repoRoot, path), 'utf8'));
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
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
