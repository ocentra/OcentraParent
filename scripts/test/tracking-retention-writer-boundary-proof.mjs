import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'tracking-retention-writer-boundary-proof');
const wp07ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const wp32ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-retention-writer-boundary-proof');
const timestamp = '2026-06-06T05:58:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await rm(proofDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp07ProofDir, { recursive: true });
await mkdir(wp32ProofDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-retention-writer-boundary-proof',
]);

const proofModule = await importDist('tracking-retention-writer-boundary-proof.js');
const readModel = proofModule.buildTrackingRetentionWriterBoundaryReadModel(proofOptions(), retentionRequests());

const proof = {
  proofMode: 'tracking-retention-writer-boundary-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: nonClaims(readModel),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-retention-writer-boundary-proof.ts',
    test: 'packages/parent-domain/tests/tracking-retention-writer-boundary-proof.test.ts',
    harness: 'scripts/test/tracking-retention-writer-boundary-proof.mjs',
    evidence: 'test-results/tracking-retention-writer-boundary-proof/proof.json',
    wp07: 'output/tracking-plan-proof/07-retention-and-custody-model/19-retention-writer-boundary-proof.json',
    wp32: 'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-writer-boundary-proof.json',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'retention-writer-boundary-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeJson(join(wp07ProofDir, '19-retention-writer-boundary-proof.json'), proof);
await writeJson(join(wp32ProofDir, '25-retention-writer-boundary-proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-retention-writer-boundary-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-retention-writer-boundary-proof', 'proof.json')}`);

function proofOptions() {
  return {
    generatedAt: timestamp,
    proofId: 'tracking-retention-writer-boundary-proof',
    familyId: 'family-tracking-retention-writer',
    childProfileId: 'child-profile-aarav',
    deviceId: 'device-aarav-phone',
    deviceLabel: 'Aarav phone',
    platform: 'android',
    sourceFeatureRefs: [
      'location-geofence-device-status',
      'tracking-plan-wp07-retention-and-custody-model',
      'tracking-plan-wp32-journal-sqlite-and-read-model-proof',
    ],
  };
}

function retentionRequests() {
  return [
    retentionRequest('retention-window', 'tracking-retention-value-7d'),
    retentionRequest('delete-after-alert-resolved', 'tracking-retention-value-delete-after-alert'),
    retentionRequest('parent-export', 'tracking-retention-value-parent-export'),
    retentionRequest('remote-sync', 'tracking-retention-value-remote-sync-disabled'),
    retentionRequest('remote-ai', 'tracking-retention-value-remote-ai-disabled'),
  ];
}

function retentionRequest(settingKind, requestedValueRef) {
  return {
    requestId: `tracking-retention-writer-${settingKind}`,
    settingKind,
    requestedValueRef,
    parentActionRef: `tracking-retention-parent-action-${settingKind}`,
    sourceProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
      'output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
    ],
    evidenceRefs: [`tracking-retention-evidence-${settingKind}`, `tracking-retention-read-model-${settingKind}`],
    auditRefs: [`tracking-retention-audit-${settingKind}`],
    requestedAt: timestamp,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    settingKinds: countBy(readModel.rows.map((row) => row.request.settingKind)),
    states: countBy(readModel.rows.map((row) => row.state)),
    acceptedForContractCount: readModel.acceptedForContractCount,
    manualServiceMutationRequiredCount: readModel.manualServiceMutationRequiredCount,
    disabledRemoteRuntimeCount: readModel.disabledRemoteRuntimeCount,
  };
}

function nonClaims(readModel) {
  return {
    serviceMutationClaimed: readModel.serviceMutationClaimed,
    platformRetentionWriterClaimed: readModel.platformRetentionWriterClaimed,
    childDeviceDeliveryClaimed: readModel.childDeviceDeliveryClaimed,
    providerDeliveryClaimed: readModel.providerDeliveryClaimed,
    notificationReceiptClaimed: readModel.notificationReceiptClaimed,
    remoteSyncRuntimeClaimed: readModel.remoteSyncRuntimeClaimed,
    remoteAiRuntimeClaimed: readModel.remoteAiRuntimeClaimed,
    portalSettingsUiClaimed: readModel.portalSettingsUiClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    productReadyClaimed: readModel.productReadyClaimed,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 5 ||
    proof.summary.acceptedForContractCount !== 2 ||
    proof.summary.manualServiceMutationRequiredCount !== 1 ||
    proof.summary.disabledRemoteRuntimeCount !== 2
  ) {
    throw new Error(`Unexpected retention writer summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Retention writer proof overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Retention Writer Boundary Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-domain retention settings writer boundary rows for typed inputs, validation refs, mutation-envelope refs, and read-model update refs.',
      '- Source inspected: location/geofence feature doc, location/geofence expectations, platforms expectations, WP07, WP32, and parent-domain README.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- Parent-domain build and focused Vitest proof passed.',
      '- Rows cover retention-window, delete-after-alert-resolved, parent-export, remote-sync disabled, and remote-AI disabled settings.',
      '- Each row carries source proof refs, evidence refs, audit refs, validation refs, mutation-envelope refs, and read-model update refs.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Live service mutation, platform retention writer, child-device delivery, provider delivery, notification receipt, remote sync runtime, remote AI runtime, portal settings UI, physical-device proof, and product-ready claims remain false.',
      '- Remote sync and remote AI request rows are represented only as disabled states.',
      '- Parent export remains manual-service-mutation-required until a real service mutation/runtime writer exists.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeJson(join(path, 'proof.json'), proof);
}

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
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
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
