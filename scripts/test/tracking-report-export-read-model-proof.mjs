import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const gateDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultDir = join(repoRoot, 'test-results', 'tracking-report-export-read-model-proof');
const companionDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-report-export-read-model-proof');
const timestamp = '2026-06-06T20:50:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await rm(companionDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(proofDir, { recursive: true });
await mkdir(gateDir, { recursive: true });
await mkdir(companionDir, { recursive: true });

runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/tracking-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-report-export-read-model-proof',
  'tracking-report-policy-consumer-proof',
  'tracking-family-dashboard-rollup-proof',
  'tracking-retention-settings-read-model-proof',
]);

const exportProof = await tsImport(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'src', 'tracking-report-export-read-model-proof.ts')).href,
  import.meta.url
);
const proofModel = exportProof.buildTrackingReportExportReadModelProof(timestamp);
const proof = {
  proofMode: 'tracking-report-export-read-model-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(proofModel.packets),
  productClaims: proofModel.productClaims,
  proofPaths: {
    source: 'packages/schema-domain/src/tracking-report-export-read-model-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-report-export-read-model-proof.test.ts',
    harness: 'scripts/test/tracking-report-export-read-model-proof.mjs',
    evidence: 'test-results/tracking-report-export-read-model-proof/proof.json',
    trackingProofPack:
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/28-report-export-read-model-proof.json',
    gateProofPack:
      'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/34-report-export-read-model-proof.json',
    companionProofPack: 'output/tracking-plan-proof/tracking-report-export-read-model-proof/proof.json',
  },
  packets: proofModel.packets,
};

assertProof(proof);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeJson(join(resultDir, 'report-export-read-model-packets.json'), proofModel.packets);
await writeJson(join(proofDir, '28-report-export-read-model-proof.json'), proof);
await writeJson(join(gateDir, '34-report-export-read-model-proof.json'), proof);
await writeCompanionPack(companionDir, proof);

console.log('tracking-report-export-read-model-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-report-export-read-model-proof', 'proof.json')}`);

function summarize(packets) {
  return {
    packetCount: packets.length,
    packetKinds: countBy(packets.map((packet) => packet.packetKind)),
    packetStates: countBy(packets.map((packet) => packet.packetState)),
    proofTiers: countBy(packets.map((packet) => packet.currentProofTier)),
    sourceProofRefPackets: packets.filter((packet) => packet.sourceProofRefs.length > 0).length,
    serviceReadModelRefPackets: packets.filter((packet) => packet.serviceReadModelProofRefs.length > 0).length,
    reportConsumerRefPackets: packets.filter((packet) => packet.reportConsumerProofRefs.length > 0).length,
    dashboardRollupRefPackets: packets.filter((packet) => packet.dashboardRollupProofRefs.length > 0).length,
    retentionSettingsRefPackets: packets.filter((packet) => packet.retentionSettingsProofRefs.length > 0).length,
    evidenceReferencePackets: packets.filter((packet) => packet.evidenceReferences.length > 0).length,
    parentOwnedLocalExportPackets: packets.filter((packet) => packet.custodyScope === 'parent-owned-local-export')
      .length,
    rawLocationPayloadClaimedPackets: packets.filter((packet) => packet.rawLocationPayloadClaimed).length,
    portalUiClaimedPackets: packets.filter((packet) => packet.portalUiClaimed).length,
    serviceMutationClaimedPackets: packets.filter((packet) => packet.serviceMutationClaimed).length,
    platformRuntimeClaimedPackets: packets.filter((packet) => packet.platformRuntimeClaimed).length,
    childDeviceDeliveryClaimedPackets: packets.filter((packet) => packet.childDeviceDeliveryClaimed).length,
    providerDeliveryClaimedPackets: packets.filter((packet) => packet.providerDeliveryClaimed).length,
    notificationReceiptClaimedPackets: packets.filter((packet) => packet.notificationReceiptClaimed).length,
    physicalDeviceClaimedPackets: packets.filter((packet) => packet.physicalDeviceClaimed).length,
    authorityClaimedPackets: packets.filter((packet) => packet.authorityClaimed).length,
    productClaimReadyPackets: packets.filter((packet) => packet.productClaimReady).length,
  };
}

function assertProof(proof) {
  const summary = proof.summary;
  if (
    summary.packetCount !== 4 ||
    summary.packetKinds['redacted-report-export-packet'] !== 1 ||
    summary.packetKinds['retention-audit-export-packet'] !== 1 ||
    summary.packetKinds['family-dashboard-summary-packet'] !== 1 ||
    summary.packetKinds['policy-drill-in-export-packet'] !== 1 ||
    summary.packetStates['export-read-model-ready'] !== 4 ||
    summary.sourceProofRefPackets !== 4 ||
    summary.serviceReadModelRefPackets !== 4 ||
    summary.reportConsumerRefPackets !== 4 ||
    summary.dashboardRollupRefPackets !== 4 ||
    summary.retentionSettingsRefPackets !== 4 ||
    summary.evidenceReferencePackets !== 4 ||
    summary.parentOwnedLocalExportPackets !== 1
  ) {
    throw new Error(`Unexpected tracking report export read-model summary: ${JSON.stringify(summary)}`);
  }

  if (
    summary.rawLocationPayloadClaimedPackets !== 0 ||
    summary.portalUiClaimedPackets !== 0 ||
    summary.serviceMutationClaimedPackets !== 0 ||
    summary.platformRuntimeClaimedPackets !== 0 ||
    summary.childDeviceDeliveryClaimedPackets !== 0 ||
    summary.providerDeliveryClaimedPackets !== 0 ||
    summary.notificationReceiptClaimedPackets !== 0 ||
    summary.physicalDeviceClaimedPackets !== 0 ||
    summary.authorityClaimedPackets !== 0 ||
    summary.productClaimReadyPackets !== 0 ||
    Object.values(proof.productClaims).some((value) => value !== false)
  ) {
    throw new Error(`Tracking report export read model overclaimed product behavior: ${JSON.stringify(summary)}`);
  }
}

async function writeCompanionPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Report Export Read-Model Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: tracking-domain redacted report/export read-model packet readiness rows composed from the existing tracking service read model, report/policy consumer proof, family dashboard rollup proof, and retention settings proof refs.',
      '- Source inspected: location/geofence feature doc, tracking implementation checklist, WP32 workpack, tracking-domain README, and existing tracking-domain proof contracts.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/tracking-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tracking-report-export-read-model-proof tracking-report-policy-consumer-proof tracking-family-dashboard-rollup-proof tracking-retention-settings-read-model-proof: PASS',
      '- Redacted report, retention audit export, family dashboard summary, and policy drill-in export packets parse through tracking-domain schemas.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Report/export packets expose evidence references only; they do not claim raw location payload export.',
      '- Packets keep remote sync and remote AI disabled and do not claim portal UI, service mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device behavior, authority, or product completion.',
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
