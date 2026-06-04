import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-notification-local-outbox-bridge-proof';
const testOutputDir = join(repoRoot, 'test-results', proofMode);
const outboxDir = join(testOutputDir, 'local-outbox');
const outboxPath = join(outboxDir, 'outbox.jsonl');
const manifestPath = join(outboxDir, 'manifest.json');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '54-notification-local-outbox-bridge');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '54-notification-local-outbox-bridge');
const commands = [];

await main();

async function main() {
  await mkdir(outboxDir, { recursive: true });
  await mkdir(join(appGameProofDir, '06-ui-snapshots'), { recursive: true });
  await mkdir(join(appProofDir, '06-ui-snapshots'), { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/app-game-notification-local-outbox-bridge.test.ts',
  ]);

  const bridge = await loadDistModule('app-game-notification-local-outbox-bridge');
  const notification = await loadDistModule('app-game-notification-intent');
  const refs = await loadDistModule('reference-primitives');
  const childUx = await loadDistModule('app-game-child-facing-ux-rules');
  await assertPackageExport(bridge);

  const fixtures = buildFixtures(notification, refs, childUx);
  const proofReadModel = bridge.buildAppGameNotificationLocalOutboxBridgeProof(fixtures.bridgeInput);
  const reasonCounts = bridge.summarizeAppGameNotificationLocalOutboxBridgeReasons(proofReadModel);
  const channelCounts = bridge.summarizeAppGameNotificationLocalOutboxBridgeChannels(proofReadModel);
  const outboxArtifact = await writeAndReadLocalOutbox(proofReadModel.records);

  assert.equal(proofReadModel.records.length, 3);
  assert.equal(proofReadModel.blockedIntentRefs.length, 2);
  assert.equal(reasonCounts['policy-violation'], 1);
  assert.equal(reasonCounts['parent-request'], 1);
  assert.equal(reasonCounts['suspicious-unknown'], 1);
  assert.equal(channelCounts.push, 1);
  assert.equal(channelCounts.email, 1);
  assert.equal(channelCounts['in-app'], 1);
  assert.equal(proofReadModel.providerDeliveryRuntimeClaimed, false);
  assert.equal(proofReadModel.providerReceiptIngestionClaimed, false);
  assert.equal(proofReadModel.parentNotificationUiClaimed, false);
  assert.equal(proofReadModel.adapterDispatchClaimed, false);
  assert.equal(proofReadModel.durableServicePersistenceClaimed, false);
  assert.equal(proofReadModel.broadAppBlockingClaimed, false);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    branch: await gitBranch(),
    commands,
    summary: {
      bridgedIntentCount: proofReadModel.bridgedIntentRefs.length,
      blockedIntentCount: proofReadModel.blockedIntentRefs.length,
      recordCount: proofReadModel.records.length,
      reasonCounts,
      channelCounts,
      blockedReasons: countBy(proofReadModel.blockedIntentRefs.map((row) => row.blockReason)),
      nonClaims: proofReadModel.nonClaims,
    },
    claimsProved: [
      'app/game local-outbox-eligible notification intents bridge into parent-domain NotificationLocalOutboxRecord rows',
      'bridged outbox records remain queued-local with minimal ref-only envelopes',
      'time-limit, approval request, and suspicious unknown app/game reasons map to notification policy-violation, parent-request, and suspicious-unknown reasons',
      'manual-required and capability-unavailable app/game intents are recorded as blocked and do not create local outbox records',
      'provider delivery, provider receipts, cloud routing, parent notification UI, adapter dispatch, durable service persistence, child delivery, broad blocking, and platform support remain false',
    ],
    claimsNotProved: [
      'provider push/email/SMS/WhatsApp/in-app delivery',
      'provider receipt ingestion',
      'parent notification UI, preferences, or history',
      'service persistence, WebSocket notification read model, or durable production outbox storage',
      'child app or overlay delivery',
      'policy evaluator execution, adapter dispatch, broad app blocking, or platform support',
    ],
    evidence: {
      contract: 'packages/parent-domain/src/app-game-notification-local-outbox-bridge.ts',
      intentContract: 'packages/parent-domain/src/app-game-notification-intent.ts',
      outboxContract: 'packages/parent-domain/src/notification-local-outbox-adapter-proof.ts',
      test: 'packages/parent-domain/tests/app-game-notification-local-outbox-bridge.test.ts',
      harness: 'scripts/test/app-game-notification-local-outbox-bridge-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/54-notification-local-outbox-bridge',
      appProofPack: 'output/app-plan-proof/54-notification-local-outbox-bridge',
      localOutboxJsonl: relativePath(outboxPath),
      localOutboxManifest: relativePath(manifestPath),
      packageExport: '@ocentra-parent/parent-domain/app-game-notification-local-outbox-bridge',
    },
    outboxArtifact,
    proofReadModel,
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP54');
  await writeProofPack(appProofDir, proof, 'app WP54');

  console.log(`${proofMode}-ok:${proofReadModel.records.length}`);
  console.log(`evidence=${relativePath(join(testOutputDir, 'proof.json'))}`);
}

async function loadDistModule(moduleName) {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', `${moduleName}.js`);
  return import(pathToFileURL(modulePath).href);
}

async function assertPackageExport(bridge) {
  const packageJson = JSON.parse(await readFile(join(repoRoot, 'packages', 'parent-domain', 'package.json'), 'utf8'));
  assert.deepEqual(packageJson.exports['./app-game-notification-local-outbox-bridge'], {
    import: './dist/app-game-notification-local-outbox-bridge.js',
    types: './dist/app-game-notification-local-outbox-bridge.d.ts',
  });

  const exportedModule = await import('@ocentra-parent/parent-domain/app-game-notification-local-outbox-bridge');
  assert.equal(
    exportedModule.AppGameNotificationLocalOutboxBridgeNonClaims.length,
    bridge.AppGameNotificationLocalOutboxBridgeNonClaims.length
  );
}

async function writeAndReadLocalOutbox(records) {
  const serialized = `${records.map((record) => JSON.stringify(record)).join('\n')}\n`;
  assertNoForbiddenDetails(serialized);
  await writeFile(outboxPath, serialized, 'utf8');
  await writeJson(manifestPath, {
    proofMode,
    outboxFile: relativePath(outboxPath),
    recordCount: records.length,
    outboxFileRef: records[0].outboxFileRef,
    localDataPathRef: records[0].localDataPathRef,
    generatedAt: new Date().toISOString(),
  });

  const parsed = (await readFile(outboxPath, 'utf8'))
    .trim()
    .split('\n')
    .map((line) => JSON.parse(line));
  return {
    outboxFile: relativePath(outboxPath),
    manifest: relativePath(manifestPath),
    recordsWritten: parsed.length,
    recordIds: parsed.map((record) => record.entryId),
  };
}

function assertNoForbiddenDetails(serialized) {
  const lowerSerialized = serialized.toLowerCase();
  for (const fragment of [
    'http://',
    'https://',
    'screenshot-bytes',
    'raw-title-value',
    'raw-message-body',
    'sqlite-private-path',
    'oauth-secret',
    'provider-token',
    'report-body',
  ]) {
    assert.equal(lowerSerialized.includes(fragment), false, `forbidden outbox detail leaked: ${fragment}`);
  }
}

function buildFixtures(notification, refs, childUx) {
  const timestamp = '2026-06-04T19:04:00Z';
  const family = {
    familyId: 'family-app-game-notification-outbox',
  };
  const device = {
    deviceId: 'device-app-game-notification-outbox',
    childProfileId: 'child-app-game-notification-outbox',
    label: 'Study PC',
    platform: refs.ParentPlatform.Windows,
  };
  const parentAction = {
    actionReferenceId: 'parent-action-app-game-notification-outbox',
    actor: { actorId: 'parent-app-game-notification-outbox', role: refs.ParentActorRole.Parent },
    policyVersion: 'policy-app-game-notification-outbox-v1',
    createdAt: timestamp,
  };
  const evidence = {
    evidenceReferenceId: 'evidence-app-game-notification-outbox-session',
    kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
    observedAt: timestamp,
  };
  const approvalActionRef = {
    actionReferenceId: 'approval-action-app-game-notification-outbox',
    actor: { actorId: 'child-local-agent', role: refs.ParentActorRole.System },
    policyVersion: 'policy-app-game-notification-outbox-v1',
    createdAt: timestamp,
  };
  const base = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    notificationIntentId: 'notification-intent-outbox-time-limit',
    intentKind: notification.AppGameNotificationIntentKind.TimeLimitReached,
    intentStatus: notification.AppGameNotificationIntentStatus.LocalOutboxEligible,
    priority: notification.AppGameNotificationPriority.Urgent,
    device,
    targetKind: childUx.AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-claim',
    notificationReasonCode: notification.AppGameNotificationReasonCode.TimeLimit,
    providerChannelPreference: 'push',
    parentTitleToken: notification.AppGameNotificationParentCopyToken.TimeLimitTitle,
    parentBodyToken: notification.AppGameNotificationParentCopyToken.TimeLimitBody,
    parentActionToken: notification.AppGameNotificationParentCopyToken.OpenParentReviewAction,
    childTitleToken: childUx.AppGameChildUxCopyToken.LimitReachedTitle,
    childBodyToken: childUx.AppGameChildUxCopyToken.LimitReachedBody,
    notificationRuleRef: 'notification-rule-app-game-time-limit',
    notificationStatusRef: 'notification-status-app-game-time-limit',
    policyRefs: ['policy-ref-game-limit'],
    auditRefs: ['audit-ref-game-limit-notification'],
    evidenceReferences: [evidence],
    childReasonReferences: [],
    childStatusReferences: ['child-status-time-limit-reached'],
    approvalActionRef: null,
    timeBudgetDecisionRef: 'time-budget-decision-game-limit',
    unknownCandidateRef: null,
    localOutboxRecordRef: 'local-outbox-record-game-limit',
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: [],
    minimalPayloadFields: Object.values(notification.AppGameNotificationPayloadField),
    deliveryClaimState: notification.AppGameNotificationDeliveryClaimState.LocalOutboxOnly,
    rawChildEvidenceIncluded: false,
    rawUrlOrTitleIncluded: false,
    rawMessageTextIncluded: false,
    screenshotOrReportIncluded: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    adapterDispatchState: notification.AppGameNotificationAdapterDispatchState.NotDispatched,
    adapterActionClaimed: false,
    createdAt: timestamp,
  };
  const approval = {
    ...base,
    notificationIntentId: 'notification-intent-approval-request',
    intentKind: notification.AppGameNotificationIntentKind.ApprovalRequested,
    priority: notification.AppGameNotificationPriority.Attention,
    targetKind: childUx.AppGameChildUxTargetKind.UnknownApp,
    targetRef: 'target-unknown-app',
    notificationReasonCode: notification.AppGameNotificationReasonCode.ApprovalRequest,
    providerChannelPreference: 'in-app',
    parentTitleToken: notification.AppGameNotificationParentCopyToken.ApprovalTitle,
    parentBodyToken: notification.AppGameNotificationParentCopyToken.ApprovalBody,
    childTitleToken: childUx.AppGameChildUxCopyToken.NewAppTitle,
    childBodyToken: childUx.AppGameChildUxCopyToken.NewAppBody,
    childReasonReferences: ['child-reason-new-app-request'],
    childStatusReferences: ['child-status-new-app-request'],
    approvalActionRef,
    timeBudgetDecisionRef: null,
    unknownCandidateRef: 'unknown-app-candidate-request',
    localOutboxRecordRef: 'local-outbox-record-approval-request',
  };
  const suspicious = {
    ...approval,
    notificationIntentId: 'notification-intent-suspicious-unknown',
    intentKind: notification.AppGameNotificationIntentKind.SuspiciousUnknown,
    notificationReasonCode: notification.AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: notification.AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: notification.AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    approvalActionRef: null,
    localOutboxRecordRef: 'local-outbox-record-suspicious-unknown',
  };
  const manual = manualOrUnavailable(notification, childUx, base, false);
  const unavailable = manualOrUnavailable(notification, childUx, base, true);
  return {
    bridgeInput: {
      generatedAt: timestamp,
      family,
      parentAction,
      sourceIntentReadModelRef: 'app-game-notification-intent-contract-proof',
      localOutboxReadModelRef: 'notification-local-outbox-adapter-proof',
      outboxRootRef: 'parent-owned-local-app-game-notification-outbox-root',
      outboxFileRef: 'parent-owned-app-game-notification-outbox-jsonl-ref',
      localDataPathRef: 'parent-owned-app-game-notification-local-data-path-ref',
      intents: [base, approval, suspicious, manual, unavailable],
    },
  };
}

function manualOrUnavailable(notification, childUx, base, unavailable) {
  return {
    ...base,
    notificationIntentId: unavailable ? 'notification-intent-unavailable' : 'notification-intent-manual-required',
    intentKind: unavailable
      ? notification.AppGameNotificationIntentKind.CapabilityUnavailable
      : notification.AppGameNotificationIntentKind.ManualRequired,
    intentStatus: unavailable
      ? notification.AppGameNotificationIntentStatus.Unavailable
      : notification.AppGameNotificationIntentStatus.ManualRequired,
    priority: notification.AppGameNotificationPriority.Attention,
    notificationReasonCode: unavailable
      ? notification.AppGameNotificationReasonCode.CapabilityUnavailable
      : notification.AppGameNotificationReasonCode.ManualReviewRequired,
    parentTitleToken: unavailable
      ? notification.AppGameNotificationParentCopyToken.UnavailableTitle
      : notification.AppGameNotificationParentCopyToken.ManualRequiredTitle,
    parentBodyToken: unavailable
      ? notification.AppGameNotificationParentCopyToken.UnavailableBody
      : notification.AppGameNotificationParentCopyToken.ManualRequiredBody,
    parentActionToken: notification.AppGameNotificationParentCopyToken.ReviewManuallyAction,
    childTitleToken: unavailable
      ? childUx.AppGameChildUxCopyToken.UnavailableTitle
      : childUx.AppGameChildUxCopyToken.ManualRequiredTitle,
    childBodyToken: unavailable
      ? childUx.AppGameChildUxCopyToken.UnavailableBody
      : childUx.AppGameChildUxCopyToken.ManualRequiredBody,
    timeBudgetDecisionRef: null,
    localOutboxRecordRef: null,
    manualProofRequirements: [
      unavailable ? 'provider or capability availability proof required' : 'parent manual review required',
    ],
    deliveryClaimState: notification.AppGameNotificationDeliveryClaimState.ManualRequired,
  };
}

async function writeProofPack(proofDir, proof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Scope: parent-domain app/game notification local outbox bridge proof.',
      '- Source inspected: WP53 notification intent contract and notification local outbox adapter proof.',
      '- Product checklist intentionally not edited; remaining delta is reported through hub.',
    ].join('\n') + '\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tests/app-game-notification-local-outbox-bridge.test.ts: PASS',
      '- Local-outbox-eligible app/game notification intents create queued local outbox records.',
      '- Manual-required and unavailable app/game notification intents are blocked from local outbox records.',
      '- Invalid provider, receipt, adapter, UI, persistence, and bridge coherence claims are rejected.',
    ].join('\n') + '\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust/service protocol not changed. This is TypeScript parent-domain bridge proof only.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof);
  await writeJson(join(proofDir, '04-journal-sqlite-proof.json'), {
    schemaVersion: 1,
    journalSqliteChanged: false,
    reason: 'No journal, SQLite, service read-model, or runtime persistence code changed.',
  });
  await writeJson(join(proofDir, '05-policy-action-proof.json'), {
    schemaVersion: 1,
    reasonCounts: proof.summary.reasonCounts,
    channelCounts: proof.summary.channelCounts,
    policyEvaluatorExecuted: false,
    adapterDispatchClaimed: false,
    broadAppBlockingClaimed: false,
  });
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\nNo portal, child app, overlay, or notification UI source changed in this workpack.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '07-playwright-ui-proof.log'),
    'Playwright/browser proof not applicable: no UI source changed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '- Bridged outbox records carry refs only and exclude raw child evidence.',
      '- Manual-required and unavailable intents do not create outbox records.',
      '- Provider delivery, provider receipts, cloud routing, parent notification UI, adapter dispatch, child-device delivery, broad app blocking, and platform support remain false.',
      '- Local outbox JSONL proof artifact is scanned for forbidden raw-detail fragments.',
    ].join('\n') + '\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\nNo provider, child-device UI, service runtime, or platform proof is attached. Delivery remains unclaimed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [
      'Validation run:',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tests/app-game-notification-local-outbox-bridge.test.ts: PASS',
      '- node scripts/test/app-game-notification-local-outbox-bridge-proof.mjs: PASS',
    ].join('\n') + '\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\nNo authority tier is raised. The bridge remains parent-domain proof with adapter dispatch disabled.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-rollback-proof.md'),
    '# Rollback Proof\nNo provider send, device action, timer, block, suspend, shield, or adapter state is created, so rollback is not applicable.\n',
    'utf8'
  );
}

async function runCommand(command, args) {
  const startedAt = new Date().toISOString();
  const child = spawn(command, args, { cwd: repoRoot, shell: false, stdio: 'inherit', windowsHide: true });
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  commands.push({ command: `${command} ${args.join(' ')}`, startedAt, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} exited with ${exitCode}`);
  }
}

async function gitBranch() {
  return (await commandOutput('git', ['rev-parse', '--abbrev-ref', 'HEAD'])).trim();
}

async function gitHead() {
  return (await commandOutput('git', ['rev-parse', 'HEAD'])).trim();
}

async function commandOutput(command, args) {
  const chunks = [];
  const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
  child.stdout.on('data', (chunk) => chunks.push(chunk));
  child.stderr.on('data', (chunk) => chunks.push(chunk));
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  const output = Buffer.concat(chunks).toString('utf8');
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}\n${output}`);
  }
  return output;
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
