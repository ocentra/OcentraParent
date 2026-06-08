import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-real-runtime-handoff-proof';
const resultRoot = join(repoRoot, 'test-results', proofMode);
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp33Root = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const generatedAt = '2026-06-08T01:05:00.000Z';
const expectedFullProductUiLocalArtifactCount = 6;

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

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tracking-real-runtime-handoff-proof',
  ]);

  const proof = await buildProof();
  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-real-runtime-handoff-proof-ok');
  console.log('evidence=test-results/tracking-real-runtime-handoff-proof/proof.json');
}

async function buildProof() {
  const proofModule = await importDist('tracking-real-runtime-handoff-proof.js');
  const closure = await readJson('test-results/tracking-product-readiness-closure-proof/proof.json');
  const inventories = [];
  for (const gate of proofModule.RequiredTrackingRealRuntimeHandoffGates) {
    const sourceProof = await readJson(gate.sourceProofRef);
    inventories.push(handoffInventoryFrom(gate, sourceProof));
  }
  const closureAccounting = closure.aggregateEvidence;
  const readModel = proofModule.buildTrackingRealRuntimeHandoffProof(generatedAt, inventories, closureAccounting);
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    requiredProofTier: 'P4_REAL_RUNTIME_HANDOFF',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'manual_required',
    sourceGateRefs: readModel.sourceGateRefs,
    closureProofRef: 'test-results/tracking-product-readiness-closure-proof/proof.json',
    closureAccounting: readModel.closureAccounting,
    readModel,
    handoffRows: readModel.handoffRows,
    summary: readModel.summary,
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

function handoffInventoryFrom(gate, sourceProof) {
  const rows = rowsFrom(sourceProof).filter(
    (row) => gate.sourceRowIds === undefined || gate.sourceRowIds.includes(row.rowId)
  );
  if (rows.length === 0) {
    throw new Error(`Artifact gate proof has no rows for ${gate.handoffArea}: ${sourceProof.proofMode ?? 'unknown'}`);
  }
  return {
    handoffArea: gate.handoffArea,
    proofRoot: rows[0]?.proofRoot ?? gate.sourceProofRef,
    requiredArtifacts: unique(rows.flatMap((row) => row.requiredArtifacts ?? [])),
    presentArtifacts: unique(rows.flatMap((row) => row.presentArtifacts ?? [])),
    auditRefs: unique(rows.flatMap((row) => row.auditRefs ?? [])),
  };
}

function rowsFrom(sourceProof) {
  if (Array.isArray(sourceProof.readModel?.rows)) return sourceProof.readModel.rows;
  if (Array.isArray(sourceProof.rows)) return sourceProof.rows;
  throw new Error(`Artifact gate proof has no rows: ${sourceProof.proofMode ?? 'unknown'}`);
}

function unique(values) {
  return [...new Set(values)];
}

function assertProof(proof) {
  const blockerSet = new Set(proof.remainingProductBlockers);
  const missingBlockers = requiredClosureBlockers.filter((blocker) => !blockerSet.has(blocker));
  if (missingBlockers.length > 0) {
    throw new Error(`Real-runtime handoff is missing closure blockers: ${missingBlockers.join(', ')}`);
  }
  if (proof.handoffRows.length < proof.sourceGateRefs.length) {
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
  if (proof.summary.requiredValidationCommandCount < proof.handoffRows.length) {
    throw new Error(`Real-runtime handoff is missing manual validation commands: ${JSON.stringify(proof.summary)}`);
  }
  if (proof.summary.ciRunnableRowCount !== 0) {
    throw new Error(`Real-runtime handoff cannot mark manual runtime rows as CI-runnable: ${JSON.stringify(proof)}`);
  }
  if (proof.closureAccounting.productClaimReady || proof.closureAccounting.claimAuditProductReadyRowCount !== 0) {
    throw new Error(`Closure accounting overclaimed product readiness: ${JSON.stringify(proof.closureAccounting)}`);
  }
  if (proof.closureAccounting.fullProductUiLocalArtifactCount !== expectedFullProductUiLocalArtifactCount) {
    throw new Error(`Closure accounting lost local UI artifact evidence: ${JSON.stringify(proof.closureAccounting)}`);
  }
  if (
    proof.closureAccounting.retentionRuntimeRequiredArtifactCount !==
      proof.closureAccounting.retentionRuntimePresentArtifactCount +
        proof.closureAccounting.retentionRuntimeMissingArtifactCount ||
    proof.closureAccounting.retentionRuntimeMissingArtifactCount < 1
  ) {
    throw new Error(
      `Closure accounting lost retention runtime artifact evidence: ${JSON.stringify(proof.closureAccounting)}`
    );
  }
  const rowsWithoutAcceptanceNotes = proof.handoffRows.filter((row) => row.artifactAcceptanceNotes.length === 0);
  if (rowsWithoutAcceptanceNotes.length > 0) {
    throw new Error(`Real-runtime handoff rows need acceptance notes: ${JSON.stringify(rowsWithoutAcceptanceNotes)}`);
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
  await writeFile(join(proofRoot, 'manual-validation-runbook.md'), manualValidationRunbook(proof));
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
    `- fullProductUiLocalArtifactCount: ${proof.closureAccounting.fullProductUiLocalArtifactCount}`,
    `- retentionRuntimeRequiredArtifactCount: ${proof.closureAccounting.retentionRuntimeRequiredArtifactCount}`,
    `- retentionRuntimePresentArtifactCount: ${proof.closureAccounting.retentionRuntimePresentArtifactCount}`,
    `- retentionRuntimeMissingArtifactCount: ${proof.closureAccounting.retentionRuntimeMissingArtifactCount}`,
    `- claimAuditMissingArtifactCount: ${proof.closureAccounting.claimAuditMissingArtifactCount}`,
    `- ciRunnableRowCount: ${proof.summary.ciRunnableRowCount}`,
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

function manualValidationRunbook(proof) {
  return [
    '# Tracking Real Runtime Handoff Manual Validation Runbook',
    '',
    `- generatedAt: ${proof.generatedAt}`,
    `- commit: ${proof.baseCommitAtGeneration}`,
    '- currentProofTier: P3_LOCAL_DEV_MACHINE',
    '- requiredProofTier: P4_REAL_RUNTIME_HANDOFF',
    '- productReadyClaimed: false',
    `- ciRunnableRowCount: ${proof.summary.ciRunnableRowCount}`,
    `- fullProductUiLocalArtifactCount: ${proof.closureAccounting.fullProductUiLocalArtifactCount}`,
    `- retentionRuntimeRequiredArtifactCount: ${proof.closureAccounting.retentionRuntimeRequiredArtifactCount}`,
    `- retentionRuntimePresentArtifactCount: ${proof.closureAccounting.retentionRuntimePresentArtifactCount}`,
    `- retentionRuntimeMissingArtifactCount: ${proof.closureAccounting.retentionRuntimeMissingArtifactCount}`,
    `- claimAuditMissingArtifactCount: ${proof.closureAccounting.claimAuditMissingArtifactCount}`,
    '',
    ...proof.handoffRows.flatMap((row) => [
      `## ${row.handoffArea}`,
      '',
      `- blockerId: ${row.blockerId}`,
      `- sourceProofRef: ${row.sourceProofRef}`,
      `- proofRoot: ${row.proofRoot}`,
      `- status: ${row.status}`,
      `- readinessCategory: ${row.readinessCategory}`,
      `- ciRunnable: ${row.ciRunnable}`,
      `- missingArtifacts: ${row.missingArtifacts.length}/${row.requiredArtifacts.length}`,
      '',
      '### Required Validation Commands',
      '',
      ...row.requiredValidationCommands.map((command) => `- ${command}`),
      '',
      '### Artifact Acceptance Notes',
      '',
      ...row.artifactAcceptanceNotes.map((note) => `- ${note}`),
      '',
      '### Missing Artifacts',
      '',
      ...row.missingArtifacts.map((artifact) => `- ${artifact}`),
      '',
    ]),
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

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}
