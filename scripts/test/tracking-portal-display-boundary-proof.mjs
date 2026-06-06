import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const wp30Dir = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const wp33Dir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultDir = join(repoRoot, 'test-results', 'tracking-portal-display-boundary-proof');
const companionDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-portal-display-boundary-proof');
const timestamp = '2026-06-06T06:38:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await rm(companionDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(wp30Dir, { recursive: true });
await mkdir(wp33Dir, { recursive: true });
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
  'tracking-portal-display-boundary-proof',
  'tracking-location-policy',
]);

const displayProofModule = await importDist('tracking-portal-display-boundary-proof.js');
const proofModel = displayProofModule.buildTrackingPortalDisplayBoundaryProof(timestamp);
const proof = {
  proofMode: 'tracking-portal-display-boundary-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.rows),
  productClaims: proofModel.productClaims,
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-portal-display-boundary-proof.ts',
    test: 'packages/parent-domain/tests/tracking-portal-display-boundary-proof.test.ts',
    harness: 'scripts/test/tracking-portal-display-boundary-proof.mjs',
    evidence: 'test-results/tracking-portal-display-boundary-proof/proof.json',
    wp30ProofPack:
      'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-portal-display-boundary-proof.json',
    wp33ProofPack:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/21-portal-display-boundary-proof.json',
    companionProofPack: 'output/tracking-plan-proof/tracking-portal-display-boundary-proof/proof.json',
  },
  rows: proofModel.rows,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'portal-display-boundary-read-model.json'), proofModel.rows);
await writeJson(join(wp30Dir, '20-portal-display-boundary-proof.json'), proof);
await writeJson(join(wp33Dir, '21-portal-display-boundary-proof.json'), proof);
await writeCompanionPack(companionDir, proof);

console.log('tracking-portal-display-boundary-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-portal-display-boundary-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function summarize(rows) {
  return {
    rowCount: rows.length,
    boundaryKinds: countBy(rows.map((row) => row.boundaryKind)),
    boundaryStates: countBy(rows.map((row) => row.boundaryState)),
    proofTiers: countBy(rows.map((row) => row.currentProofTier)),
    visibleStatusCount: sum(rows.map((row) => row.visibleStatusCount)),
    editableSettingCount: sum(rows.map((row) => row.editableSettingCount)),
    sourceProofRefRows: rows.filter((row) => row.sourceProofRefs.length > 0).length,
    hostedProofRefRows: rows.filter((row) => row.hostedProofRefs.length > 0).length,
    evidenceReferenceRows: rows.filter((row) => row.evidenceReferences.length > 0).length,
    portalDisplayClaimedRows: rows.filter((row) => row.portalDisplayClaimed).length,
    portalAuthoringClaimedRows: rows.filter((row) => row.portalAuthoringClaimed).length,
    portalEvaluatorClaimedRows: rows.filter((row) => row.portalEvaluatorClaimed).length,
    policyEvaluationClaimedRows: rows.filter((row) => row.policyEvaluationClaimed).length,
    serviceMutationClaimedRows: rows.filter((row) => row.serviceMutationClaimed).length,
    platformWriterExecutionClaimedRows: rows.filter((row) => row.platformWriterExecutionClaimed).length,
    childRuntimeExecutionClaimedRows: rows.filter((row) => row.childRuntimeExecutionClaimed).length,
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
    summary.rowCount !== 4 ||
    summary.boundaryKinds['service-read-model-display'] !== 1 ||
    summary.boundaryKinds['retention-settings-display'] !== 1 ||
    summary.boundaryKinds['family-dashboard-rollup-display'] !== 1 ||
    summary.boundaryKinds['unsupported-platform-manual-display'] !== 1 ||
    summary.boundaryStates['display-ready'] !== 2 ||
    summary.boundaryStates['authoring-ready'] !== 1 ||
    summary.boundaryStates['manual-required'] !== 1 ||
    summary.visibleStatusCount !== 16 ||
    summary.editableSettingCount !== 5 ||
    summary.sourceProofRefRows !== 4 ||
    summary.hostedProofRefRows !== 4 ||
    summary.evidenceReferenceRows !== 4 ||
    summary.portalDisplayClaimedRows !== 4 ||
    summary.portalAuthoringClaimedRows !== 1
  ) {
    throw new Error(`Unexpected tracking portal display boundary summary: ${JSON.stringify(summary)}`);
  }

  if (
    summary.portalEvaluatorClaimedRows !== 0 ||
    summary.policyEvaluationClaimedRows !== 0 ||
    summary.serviceMutationClaimedRows !== 0 ||
    summary.platformWriterExecutionClaimedRows !== 0 ||
    summary.childRuntimeExecutionClaimedRows !== 0 ||
    summary.providerDeliveryClaimedRows !== 0 ||
    summary.notificationReceiptClaimedRows !== 0 ||
    summary.physicalDeviceClaimedRows !== 0 ||
    summary.authorityClaimedRows !== 0 ||
    summary.productClaimReadyRows !== 0 ||
    Object.values(proof.productClaims).some((value) => value !== false)
  ) {
    throw new Error(`Tracking portal display boundary overclaimed product behavior: ${JSON.stringify(summary)}`);
  }
}

async function writeCompanionPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Portal Display Boundary Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain rows proving the hosted portal consumes and authors tracking status/read-model state without becoming the evaluator, service mutation writer, provider, child runtime, or product-ready authority.',
      '- Source inspected: feature list, location/geofence feature doc, location/geofence expectations, platform expectations, parent-domain README, WP30, WP33, and tracking checklist rows.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-portal-display-boundary-proof tracking-location-policy: PASS',
      '- Service read-model, retention settings, family dashboard, and unsupported/manual platform display rows parse through parent-domain schemas.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Portal display rows do not claim portal evaluator behavior, policy evaluation authority, service mutation, platform writer execution, child runtime execution, provider delivery, notification receipts, physical-device behavior, authority enrollment, or product readiness.',
      '- Retention settings authoring readiness is separated from live service mutation and platform writer execution.',
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
