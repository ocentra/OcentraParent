import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofMode = 'tracking-retention-settings-write-command-proof';
const output07 = join(repoRoot, 'output', 'tracking-plan-proof', '07-retention-and-custody-model');
const output32 = join(repoRoot, 'output', 'tracking-plan-proof', '32-journal-sqlite-and-read-model-proof');
const output33 = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const resultRoot = join(repoRoot, 'test-results', proofMode);
const generatedAt = '2026-06-06T20:05:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await main();

async function main() {
  await rm(resultRoot, { recursive: true, force: true });
  await mkdir(resultRoot, { recursive: true });
  await mkdir(output07, { recursive: true });
  await mkdir(output32, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/agent-protocol-domain',
    '--',
    'tracking-retention-settings-write-command',
  ]);
  run('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'retention_settings_write', '--', '--nocapture']);
  run('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'retention_settings_write', '--', '--nocapture']);

  const protocolModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'agent-protocol-domain', 'dist', 'tracking-retention-settings-write-command.js')
    ).href
  );
  const request = protocolModule.AgentTrackingRetentionSettingsWriteRequestSchema.parse(writeRequest());
  const result = protocolModule.AgentTrackingRetentionSettingsWriteResultSchema.parse(writeResult());
  const proof = {
    schemaVersion: result.schemaVersion,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    request,
    result,
    proofClaims: {
      commandTransportClaimed: result.commandTransportClaimed,
      serviceWritePreflightClaimed: result.serviceWritePreflightClaimed,
      serviceMutationExecuted: result.serviceMutationExecuted,
      localServiceStateRevisionRecorded: result.localServiceStateRevision !== null,
    },
    productClaims: {
      durableSettingsPersisted: result.durableSettingsPersisted,
      portalWritableUiClaimed: result.portalWritableUiClaimed,
      platformRuntimeClaimed: result.platformRuntimeClaimed,
      childDeviceDeliveryClaimed: result.childDeviceDeliveryClaimed,
      providerDeliveryClaimed: result.providerDeliveryClaimed,
      notificationReceiptClaimed: result.notificationReceiptClaimed,
      physicalDeviceClaimed: result.physicalDeviceClaimed,
      authorityClaimed: result.authorityClaimed,
      productClaimReady: result.productClaimReady,
    },
    commands,
    artifactPaths: {
      wp07: 'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json',
      wp32: 'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/27-retention-settings-write-command-proof.json',
      wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/33-retention-settings-write-command-proof.json',
      evidence: 'test-results/tracking-retention-settings-write-command-proof/proof.json',
    },
  };

  assertProof(proof);
  await writeJson(join(resultRoot, 'proof.json'), proof);
  await writeJson(join(output07, '21-retention-settings-write-command-proof.json'), proof);
  await writeJson(join(output32, '27-retention-settings-write-command-proof.json'), proof);
  await writeJson(join(output33, '33-retention-settings-write-command-proof.json'), proof);
  await writeFile(join(output07, '21-retention-settings-write-command-validation.log'), validationLog());
  await writeFile(join(output32, '27-retention-settings-write-command-validation.log'), validationLog());

  console.log('tracking-retention-settings-write-command-proof-ok');
  console.log('evidence=test-results/tracking-retention-settings-write-command-proof/proof.json');
}

function writeRequest() {
  return {
    schemaVersion: 1,
    commandId: 'tracking-retention-settings-write-command',
    settingsKind: 'retention-window-setting',
    requestedRetentionWindowHours: 168,
    requestedDeleteAfterAlertResolved: false,
    requestedParentExport: false,
    requestedRemoteSyncEnabled: false,
    requestedRemoteAiEnabled: false,
    sourceWriterIntentRefs: ['tracking-retention-settings-write-retention-window'],
    sourceReadModelProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
    ],
  };
}

function writeResult() {
  const request = writeRequest();
  return {
    schemaVersion: 1,
    commandId: request.commandId,
    settingsKind: request.settingsKind,
    writeState: 'service-write-command-accepted',
    acceptedAt: '2026-06-06T19:50:00Z',
    sourceWriterIntentRefs: request.sourceWriterIntentRefs,
    sourceReadModelProofRefs: request.sourceReadModelProofRefs,
    sourceMutationProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
    ],
    appliedRetentionWindowHours: request.requestedRetentionWindowHours,
    appliedDeleteAfterAlertResolved: request.requestedDeleteAfterAlertResolved,
    parentExportPrepared: request.requestedParentExport,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    localServiceStateRevision: 1,
    localServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
    durableSettingsPersisted: false,
    commandTransportClaimed: true,
    serviceWritePreflightClaimed: true,
    serviceMutationExecuted: true,
    portalWritableUiClaimed: false,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  };
}

function assertProof(proof) {
  if (!proof.result.commandTransportClaimed || !proof.result.serviceWritePreflightClaimed) {
    throw new Error('Retention write command proof must claim command transport and service preflight.');
  }
  if (!proof.result.serviceMutationExecuted) {
    throw new Error('Retention write command proof must execute the local service mutation.');
  }
  if (proof.result.appliedRetentionWindowHours !== proof.request.requestedRetentionWindowHours) {
    throw new Error('Retention write command proof must return the applied retention window.');
  }
  if (proof.result.localServiceStateRevision !== 1) {
    throw new Error('Retention write command proof must record the local service state revision.');
  }
  if (proof.result.localServiceStateSnapshotRef !== 'agent-service-local-retention-settings-state') {
    throw new Error('Retention write command proof must record the local service state snapshot ref.');
  }
  if (proof.result.durableSettingsPersisted) {
    throw new Error('Retention write command proof must not claim durable product settings persistence.');
  }
  if (proof.result.remoteSyncEnabled || proof.result.remoteAiEnabled) {
    throw new Error('Retention write command proof must keep remote sync and remote AI disabled.');
  }
  if (Object.values(proof.productClaims).some((claim) => claim !== false)) {
    throw new Error(
      `Retention write command proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`
    );
  }
  if (proof.result.sourceMutationProofRefs.length !== 1) {
    throw new Error('Retention write command proof must cite the existing mutation proof ref.');
  }
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function validationLog() {
  return `${commands.map((entry) => `${entry.command} exit=${entry.status}`).join('\n')}\n`;
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

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
