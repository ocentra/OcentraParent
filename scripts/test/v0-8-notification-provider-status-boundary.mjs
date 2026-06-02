import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-8-notification-provider-status-boundary');
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build:contracts']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'v0-8-notification-provider-status-boundary',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'v0-8-enforcement-integrity-runtime-audit',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/agent-protocol-domain',
    '--',
    'enforcement-supported-adapter-runtime-proof-adapter',
  ]);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'notification_provider_status_boundary']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'enforcement_integrity_runtime_audit']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'notification_provider_status_boundary']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'enforcement_integrity_runtime_audit']);

  const { V08NotificationProviderStatusBoundaryReadModel } =
    await import('../../packages/parent-domain/dist/v0-8-notification-provider-status-boundary.js');
  const { V08EnforcementIntegrityRuntimeAuditReadModel } =
    await import('../../packages/parent-domain/dist/v0-8-supported-adapter-runtime-proof.js');
  const summary = summarizeReadModel(V08NotificationProviderStatusBoundaryReadModel);

  assertReadModel(
    V08NotificationProviderStatusBoundaryReadModel,
    V08EnforcementIntegrityRuntimeAuditReadModel,
    summary
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'v0-8-notification-provider-status-boundary',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      tsContract: 'packages/parent-domain/src/v0-8-notification-provider-status-boundary.ts',
      tsContractTest: 'packages/parent-domain/tests/v0-8-notification-provider-status-boundary.test.ts',
      tsNestedAuditContract: 'packages/parent-domain/src/v0-8-enforcement-integrity-runtime-audit.ts',
      tsProtocolAdapter: 'packages/agent-protocol-domain/src/enforcement-supported-adapter-runtime-proof-adapter.ts',
      rustProtocol: 'crates/agent-protocol/src/notification_provider_status_boundary.rs',
      rustProtocolTest: 'crates/agent-protocol/src/notification_provider_status_boundary_tests.rs',
      rustServiceReadModel:
        'crates/agent-service/src/enforcement_api/notification_provider_status_boundary_read_model.rs',
      rustServiceTest:
        'crates/agent-service/src/enforcement_api/notification_provider_status_boundary_read_model_tests.rs',
      rustServiceEventPayload:
        'agent.enforcement.supported-adapter-runtime-proof.reported:enforcementIntegrityRuntimeAuditReadModel.notificationProviderStatusBoundary',
      proofHarness: 'scripts/test/v0-8-notification-provider-status-boundary.mjs',
    },
    counts: summary,
    claimsProved: [
      'Queued, delivered, failed, unavailable, and manual-required provider status contract states are represented',
      'Delivered remains a receipt-required contract state, not observed delivery',
      'Quiet-hours and escalation readiness states are represented with refs',
      'The existing supported-adapter runtime proof event exposes the provider status boundary through the nested integrity audit read model payload',
      'Sensitive provider payloads and provider child-evidence storage remain unclaimed',
    ],
    claimsNotProved: [
      'notification provider delivery',
      'provider adapter implementation',
      'provider retry execution',
      'provider webhook receipt ingestion',
      'parent notification UI',
      'third-party provider credential readiness',
      'Ocentra-hosted storage of child activity or reports',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-8-notification-provider-status-boundary-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function summarizeReadModel(readModel) {
  return {
    entries: readModel.entries.length,
    byProviderStatus: countBy(readModel.entries.map((entry) => entry.providerStatus)),
    byQuietHoursReadiness: countBy(readModel.entries.map((entry) => entry.quietHoursReadiness)),
    byEscalationReadiness: countBy(readModel.entries.map((entry) => entry.escalationReadiness)),
    providerDeliveryImplemented: readModel.entries.filter((entry) => entry.providerDeliveryImplemented).length,
    providerDeliveryObserved: readModel.entries.filter((entry) => entry.providerDeliveryObserved).length,
    deliveredNotificationClaimed: readModel.entries.filter((entry) => entry.deliveredNotificationClaimed).length,
    sensitiveProviderPayloadClaimed: readModel.entries.filter((entry) => entry.sensitiveProviderPayloadClaimed).length,
    providerStoresChildEvidenceClaimed: readModel.entries.filter((entry) => entry.providerStoresChildEvidenceClaimed)
      .length,
  };
}

function assertReadModel(readModel, auditReadModel, summary) {
  assertEqual(readModel.readModelId, 'v0-8-notification-provider-status-boundary', 'read model id');
  assertEqual(summary.entries, 5, 'entry count');
  assertEqual(summary.byProviderStatus.queued, 1, 'queued count');
  assertEqual(summary.byProviderStatus.delivered, 1, 'delivered count');
  assertEqual(summary.byProviderStatus.failed, 1, 'failed count');
  assertEqual(summary.byProviderStatus.unavailable, 1, 'unavailable count');
  assertEqual(summary.byProviderStatus['manual-required'], 1, 'manual-required count');
  assertEqual(summary.byQuietHoursReadiness.ready, 2, 'quiet-hours ready count');
  assertEqual(summary.byQuietHoursReadiness['defer-noncritical'], 1, 'quiet-hours defer count');
  assertEqual(summary.byEscalationReadiness['manual-required'], 2, 'escalation manual-required count');
  assertEqual(summary.providerDeliveryImplemented, 0, 'provider implementation claim count');
  assertEqual(summary.providerDeliveryObserved, 0, 'provider observed claim count');
  assertEqual(summary.deliveredNotificationClaimed, 0, 'delivered notification claim count');
  assertEqual(summary.sensitiveProviderPayloadClaimed, 0, 'sensitive payload claim count');
  assertEqual(summary.providerStoresChildEvidenceClaimed, 0, 'child evidence storage claim count');
  assertEqual(
    auditReadModel.notificationProviderStatusBoundary.readModelId,
    'v0-8-notification-provider-status-boundary',
    'nested audit read model id'
  );

  const delivered = readModel.entries.find(
    (entry) => entry.statusEntryId === 'notification-provider-delivered-receipt-required'
  );
  if (delivered === undefined) {
    throw new Error('missing delivered provider status entry');
  }
  assertEqual(delivered.statusProofState, 'delivery-receipt-required', 'delivered proof state');
  assertEqual(delivered.deliveryClaimState, 'receipt-required', 'delivered claim state');
  if (delivered.providerReceiptRefs.length === 0 || delivered.manualProofRequirements.length === 0) {
    throw new Error('delivered provider status is missing receipt/manual proof requirements');
  }

  proofLabels.push('v0.8.notification-provider-status-boundary.contract-states');
  proofLabels.push('v0.8.notification-provider-status-boundary.quiet-hours-escalation-readiness');
  proofLabels.push('v0.8.notification-provider-status-boundary.service-read-model');
  proofLabels.push('v0.8.notification-provider-status-boundary.supported-adapter-event-nesting');
  proofLabels.push('v0.8.notification-provider-status-boundary.no-provider-delivery-claim');
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`))));
    child.once('error', reject);
  });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}
