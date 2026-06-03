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
    'v3-notification-rule-provider-retry-contract',
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
  const { V3NotificationRuleProviderRetryContractReadModel } =
    await import('../../packages/parent-domain/dist/v3-notification-rule-provider-retry-contract.js');
  const { V08EnforcementIntegrityRuntimeAuditReadModel } =
    await import('../../packages/parent-domain/dist/v0-8-supported-adapter-runtime-proof.js');
  const summary = summarizeReadModel(V08NotificationProviderStatusBoundaryReadModel);
  const v3ContractSummary = summarizeV3ContractReadModel(V3NotificationRuleProviderRetryContractReadModel);

  assertReadModel(
    V08NotificationProviderStatusBoundaryReadModel,
    V3NotificationRuleProviderRetryContractReadModel,
    V08EnforcementIntegrityRuntimeAuditReadModel,
    summary,
    v3ContractSummary
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
      tsV3NotificationRuleProviderRetryContract:
        'packages/parent-domain/src/v3-notification-rule-provider-retry-contract.ts',
      tsV3NotificationRuleProviderRetryContractTest:
        'packages/parent-domain/tests/v3-notification-rule-provider-retry-contract.test.ts',
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
    v3ContractCounts: v3ContractSummary,
    claimsProved: [
      'Queued, delivered, failed, unavailable, and manual-required provider status contract states are represented',
      'Delivered remains a receipt-required contract state, not observed delivery',
      'Quiet-hours and escalation readiness states are represented with refs',
      'The existing supported-adapter runtime proof event exposes the provider status boundary through the nested integrity audit read model payload',
      'Sensitive provider payloads and provider child-evidence storage remain unclaimed',
      'V3 notification rule/provider retry contract covers alert reason codes, provider channels, delivery attempt/result states, retry policy states, quiet-hours decisions, escalation decisions, parent preferences, audit refs, and evidence refs',
      'V3 notification contract rows keep provider adapters, delivery execution, provider receipt observation, raw evidence payloads, and provider child-evidence storage unclaimed',
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

function summarizeV3ContractReadModel(readModel) {
  return {
    entries: readModel.entries.length,
    byReasonCode: countBy(readModel.entries.map((entry) => entry.reasonCode)),
    byProviderChannel: countBy(readModel.entries.map((entry) => entry.providerChannel)),
    byDeliveryAttemptState: countBy(readModel.entries.map((entry) => entry.deliveryAttemptState)),
    byDeliveryResultState: countBy(readModel.entries.map((entry) => entry.deliveryResultState)),
    byRetryPolicyState: countBy(readModel.entries.map((entry) => entry.retryPolicyState)),
    byQuietHoursDecision: countBy(readModel.entries.map((entry) => entry.quietHoursDecision)),
    byEscalationDecision: countBy(readModel.entries.map((entry) => entry.escalationDecision)),
    byParentPreferenceState: countBy(readModel.entries.map((entry) => entry.parentPreferenceState)),
    providerAdapterImplemented: readModel.entries.filter((entry) => entry.providerAdapterImplemented).length,
    deliveryAttemptExecuted: readModel.entries.filter((entry) => entry.deliveryAttemptExecuted).length,
    providerReceiptObserved: readModel.entries.filter((entry) => entry.providerReceiptObserved).length,
    rawEvidenceInProviderPayload: readModel.entries.filter((entry) => entry.rawEvidenceInProviderPayload).length,
    providerStoresChildEvidenceClaimed: readModel.entries.filter((entry) => entry.providerStoresChildEvidenceClaimed)
      .length,
  };
}

function assertReadModel(readModel, v3ContractReadModel, auditReadModel, summary, v3ContractSummary) {
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

  assertEqual(
    v3ContractReadModel.readModelId,
    'v3-notification-rule-provider-retry-contract',
    'V3 notification rule provider retry read model id'
  );
  assertEqual(v3ContractSummary.entries, 6, 'V3 contract entry count');
  assertEqual(v3ContractSummary.byReasonCode['policy-violation'], 1, 'V3 policy violation reason count');
  assertEqual(v3ContractSummary.byReasonCode['parent-request'], 1, 'V3 parent request reason count');
  assertEqual(v3ContractSummary.byReasonCode['suspicious-unknown'], 1, 'V3 suspicious unknown reason count');
  assertEqual(v3ContractSummary.byReasonCode['device-offline'], 1, 'V3 device offline reason count');
  assertEqual(v3ContractSummary.byReasonCode['sync-failure'], 1, 'V3 sync failure reason count');
  assertEqual(v3ContractSummary.byReasonCode['provider-failure'], 1, 'V3 provider failure reason count');
  assertEqual(v3ContractSummary.byProviderChannel.push, 1, 'V3 push provider channel count');
  assertEqual(v3ContractSummary.byProviderChannel.email, 1, 'V3 email provider channel count');
  assertEqual(v3ContractSummary.byProviderChannel.sms, 1, 'V3 SMS provider channel count');
  assertEqual(v3ContractSummary.byProviderChannel.whatsapp, 1, 'V3 WhatsApp provider channel count');
  assertEqual(v3ContractSummary.byProviderChannel['in-app'], 2, 'V3 in-app provider channel count');
  assertEqual(v3ContractSummary.byDeliveryResultState['retryable-failure'], 1, 'V3 retryable failure count');
  assertEqual(v3ContractSummary.byDeliveryResultState['permanent-failure'], 1, 'V3 permanent failure count');
  assertEqual(v3ContractSummary.byDeliveryResultState['not-sent'], 1, 'V3 not sent count');
  assertEqual(v3ContractSummary.byRetryPolicyState['exponential-backoff'], 1, 'V3 retry backoff count');
  assertEqual(v3ContractSummary.byRetryPolicyState['quiet-hours-deferred'], 1, 'V3 quiet-hours retry count');
  assertEqual(v3ContractSummary.byParentPreferenceState['channel-disabled'], 1, 'V3 channel disabled count');
  assertEqual(v3ContractSummary.providerAdapterImplemented, 0, 'V3 provider implementation claim count');
  assertEqual(v3ContractSummary.deliveryAttemptExecuted, 0, 'V3 delivery execution claim count');
  assertEqual(v3ContractSummary.providerReceiptObserved, 0, 'V3 provider receipt observed count');
  assertEqual(v3ContractSummary.rawEvidenceInProviderPayload, 0, 'V3 raw evidence payload claim count');
  assertEqual(
    v3ContractSummary.providerStoresChildEvidenceClaimed,
    0,
    'V3 provider child evidence storage claim count'
  );

  proofLabels.push('v0.8.notification-provider-status-boundary.contract-states');
  proofLabels.push('v0.8.notification-provider-status-boundary.quiet-hours-escalation-readiness');
  proofLabels.push('v0.8.notification-provider-status-boundary.service-read-model');
  proofLabels.push('v0.8.notification-provider-status-boundary.supported-adapter-event-nesting');
  proofLabels.push('v0.8.notification-provider-status-boundary.no-provider-delivery-claim');
  proofLabels.push('v3.notification-rule-provider-retry-contract.reason-channel-coverage');
  proofLabels.push('v3.notification-rule-provider-retry-contract.delivery-retry-preference-coverage');
  proofLabels.push('v3.notification-rule-provider-retry-contract.no-provider-runtime-claim');
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
