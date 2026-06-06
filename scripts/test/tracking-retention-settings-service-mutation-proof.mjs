import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const wp07ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const wp32ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultDir = join(repoRoot, 'test-results', 'tracking-retention-settings-service-mutation-proof');
const companionDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  'tracking-retention-settings-service-mutation-proof'
);
const timestamp = '2026-06-06T15:41:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await rm(companionDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(wp07ProofDir, { recursive: true });
await mkdir(wp32ProofDir, { recursive: true });
await mkdir(wp33ProofDir, { recursive: true });
await mkdir(companionDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/agent-protocol-domain',
  '--',
  'tracking-retention-settings-service-mutation',
]);
run('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'tracking_retention_settings_mutation']);
run('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'tracking_retention_settings_mutation']);
run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-retention-settings-service-mutation-proof',
  'tracking-retention-settings-writer-boundary-proof',
]);

const retentionSettingsProof = await importDist('tracking-retention-settings-service-mutation-proof.js');
const proofModel = retentionSettingsProof.buildTrackingRetentionSettingsServiceMutationProof(timestamp);
const proof = {
  proofMode: 'tracking-retention-settings-service-mutation-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.rows),
  proofClaims: proofModel.proofClaims,
  productClaims: proofModel.productClaims,
  proofPaths: {
    protocolDomain: 'packages/agent-protocol-domain/src/tracking-retention-settings-service-mutation.ts',
    protocolRust: 'crates/agent-protocol/src/tracking_retention_settings_service_mutation.rs',
    servicePayload: 'crates/agent-service/src/tracking_retention_settings_service_mutation_payload.rs',
    parentDomain: 'packages/parent-domain/src/tracking-retention-settings-service-mutation-proof.ts',
    test: 'packages/parent-domain/tests/tracking-retention-settings-service-mutation-proof.test.ts',
    harness: 'scripts/test/tracking-retention-settings-service-mutation-proof.mjs',
    evidence: 'test-results/tracking-retention-settings-service-mutation-proof/proof.json',
    retentionProofPack:
      'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-service-mutation-proof.json',
    readModelProofPack:
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/27-retention-settings-service-mutation-proof.json',
    proofGatePack:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/28-retention-settings-service-mutation-proof.json',
    companionProofPack: 'output/tracking-plan-proof/tracking-retention-settings-service-mutation-proof/proof.json',
  },
  rows: proofModel.rows,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'retention-settings-service-mutation.json'), proofModel.rows);
await writeJson(join(wp07ProofDir, '20-retention-settings-service-mutation-proof.json'), proof);
await writeJson(join(wp32ProofDir, '27-retention-settings-service-mutation-proof.json'), proof);
await writeJson(join(wp33ProofDir, '28-retention-settings-service-mutation-proof.json'), proof);
await writeCompanionPack(companionDir, proof);

console.log('tracking-retention-settings-service-mutation-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-retention-settings-service-mutation-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function summarize(rows) {
  return {
    rowCount: rows.length,
    settingsKinds: countBy(rows.map((row) => row.settingsKind)),
    writeActions: countBy(rows.map((row) => row.writeAction)),
    mutationStates: countBy(rows.map((row) => row.mutationState)),
    requestedValues: countBy(rows.map((row) => row.requestedValue)),
    sourceReadModelProofRefRows: rows.filter((row) => row.sourceReadModelProofRefs.length > 0).length,
    writerBoundaryProofRefRows: rows.filter((row) => row.writerBoundaryProofRefs.length > 0).length,
    evidenceReferenceRows: rows.filter((row) => row.evidenceReferences.length > 0).length,
    auditRefRows: rows.filter((row) => row.auditRefs.length > 0).length,
    serviceCommandRegisteredClaimedRows: rows.filter((row) => row.serviceCommandRegisteredClaimed).length,
    serviceMutationExecutedRows: rows.filter((row) => row.serviceMutationExecuted).length,
    durablePersistenceClaimedRows: rows.filter((row) => row.durablePersistenceClaimed).length,
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
    summary.mutationStates.accepted !== 5 ||
    summary.requestedValues['168'] !== 1 ||
    summary.requestedValues.true !== 2 ||
    summary.requestedValues.false !== 2 ||
    summary.sourceReadModelProofRefRows !== 5 ||
    summary.writerBoundaryProofRefRows !== 5 ||
    summary.evidenceReferenceRows !== 5 ||
    summary.auditRefRows !== 5 ||
    summary.serviceCommandRegisteredClaimedRows !== 5 ||
    summary.serviceMutationExecutedRows !== 5
  ) {
    throw new Error(`Unexpected tracking retention service mutation summary: ${JSON.stringify(summary)}`);
  }

  if (
    summary.durablePersistenceClaimedRows !== 0 ||
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
    throw new Error(
      `Tracking retention service mutation proof overclaimed product behavior: ${JSON.stringify(summary)}`
    );
  }
}

async function writeCompanionPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Retention Settings Service Mutation Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: protocol/domain/service command mutation proof for local tracking retention settings.',
      '- Source inspected: location/geofence feature doc, WP07, WP32, agent-protocol-domain, parent-domain, agent-protocol, and agent-service READMEs.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- tracking-retention-settings-service-mutation: PASS',
      '- cargo test -p ocentra-parent-agent-protocol tracking_retention_settings_mutation: PASS',
      '- cargo test -p ocentra-parent-agent-service tracking_retention_settings_mutation: PASS',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-retention-settings-service-mutation-proof tracking-retention-settings-writer-boundary-proof: PASS',
      '- The WebSocket command routes a retention settings mutation payload into a typed reported event.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Service mutation rows claim local command registration and service mutation execution only.',
      '- Rows do not claim durable persistence, portal UI completion, platform runtime, child-device delivery, provider delivery, notification receipts, authority enrollment, physical-device behavior, or product completion.',
      '- Remote sync and remote AI requested values stay false.',
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
