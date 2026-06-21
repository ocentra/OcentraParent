import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const timestamp = '2026-06-07T05:44:00.000Z';
const resultDir = join(repoRoot, 'test-results', 'tracking-ai-stored-ref-consumer-proof');
const wp24Dir = join(repoRoot, 'output', 'tracking-plan-proof', '24-ai-provider-routing');
const wp32Dir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const wp33Dir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(wp24Dir, { recursive: true });
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
  'tracking-ai-stored-ref-consumer-proof.test.ts',
  'tracking-ai-provider-routing-proof.test.ts',
  'tracking-report-policy-consumer-proof.test.ts',
]);

const storedRefConsumer = await importDist('tracking-ai-stored-ref-consumer-proof.js');
const proofModel = storedRefConsumer.buildTrackingAiStoredRefConsumerProof(timestamp);
const proof = {
  proofMode: 'tracking-ai-stored-ref-consumer-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.rows),
  productClaims: proofModel.productClaims,
  proofPaths: {
    source: 'packages/tracking-domain/src/tracking-ai-stored-ref-consumer-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-ai-stored-ref-consumer-proof.test.ts',
    harness: 'scripts/test/tracking-ai-stored-ref-consumer-proof.mjs',
    evidence: 'test-results/tracking-ai-stored-ref-consumer-proof/proof.json',
    wp24ProofPack: 'output/tracking-plan-proof/24-ai-provider-routing/19-ai-stored-ref-consumer-proof.json',
    wp32ProofPack:
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/30-ai-stored-ref-consumer-proof.json',
    wp33ProofPack:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/30-ai-stored-ref-consumer-proof.json',
  },
  rows: proofModel.rows,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'ai-stored-ref-consumer-read-model.json'), proofModel.rows);
await writeJson(join(wp24Dir, '19-ai-stored-ref-consumer-proof.json'), proof);
await writeJson(join(wp32Dir, '30-ai-stored-ref-consumer-proof.json'), proof);
await writeJson(join(wp33Dir, '30-ai-stored-ref-consumer-proof.json'), proof);

console.log('tracking-ai-stored-ref-consumer-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-ai-stored-ref-consumer-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'tracking-domain', 'dist', name)).href);
}

function summarize(rows) {
  return {
    rowCount: rows.length,
    consumerKinds: countBy(rows.map((row) => row.consumerKind)),
    readinessStates: countBy(rows.map((row) => row.readinessState)),
    proofTiers: countBy(rows.map((row) => row.currentProofTier)),
    providerRouteIds: rows.map((row) => row.analysisInput.providerRouteId),
    sourceProofRefRows: rows.filter((row) => row.sourceProofRefs.length > 0).length,
    providerRouteProofRefRows: rows.filter((row) => row.aiProviderRouteProofRefs.length > 0).length,
    reportPolicyConsumerProofRefRows: rows.filter((row) => row.reportPolicyConsumerProofRefs.length > 0).length,
    storedJournalRefRows: rows.filter((row) => row.storedJournalRefs.length > 0).length,
    storedReadModelRowRefRows: rows.filter((row) => row.storedReadModelRowRefs.length > 0).length,
    evidenceReferenceRows: rows.filter((row) => row.evidenceReferences.length > 0).length,
    modelExecutionClaimedRows: rows.filter((row) => row.modelExecutionClaimed).length,
    assistantPolicyWriteClaimedRows: rows.filter((row) => row.assistantPolicyWriteClaimed).length,
    assistantEnforcementClaimedRows: rows.filter((row) => row.assistantEnforcementClaimed).length,
    childDeviceRuntimeClaimedRows: rows.filter((row) => row.childDeviceRuntimeClaimed).length,
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
    summary.rowCount !== 3 ||
    summary.consumerKinds['ai-parent-report-context'] !== 1 ||
    summary.consumerKinds['ai-policy-drill-in-context'] !== 1 ||
    summary.consumerKinds['ai-metadata-fallback-context'] !== 1 ||
    summary.readinessStates['stored-ref-consumer-ready'] !== 3 ||
    summary.sourceProofRefRows !== 3 ||
    summary.providerRouteProofRefRows !== 3 ||
    summary.reportPolicyConsumerProofRefRows !== 3 ||
    summary.storedJournalRefRows !== 3 ||
    summary.storedReadModelRowRefRows !== 3 ||
    summary.evidenceReferenceRows !== 3
  ) {
    throw new Error(`Unexpected tracking AI stored-ref consumer summary: ${JSON.stringify(summary)}`);
  }

  if (
    summary.modelExecutionClaimedRows !== 0 ||
    summary.assistantPolicyWriteClaimedRows !== 0 ||
    summary.assistantEnforcementClaimedRows !== 0 ||
    summary.childDeviceRuntimeClaimedRows !== 0 ||
    summary.providerDeliveryClaimedRows !== 0 ||
    summary.notificationReceiptClaimedRows !== 0 ||
    summary.physicalDeviceClaimedRows !== 0 ||
    summary.authorityClaimedRows !== 0 ||
    summary.productionBehaviorClaimedRows !== 0 ||
    summary.productClaimReadyRows !== 0 ||
    Object.values(proof.productClaims).some((value) => value !== false)
  ) {
    throw new Error(`Tracking AI stored-ref consumer proof overclaimed behavior: ${JSON.stringify(summary)}`);
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
