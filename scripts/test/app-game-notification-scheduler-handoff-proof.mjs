import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-notification-scheduler-handoff-proof';
const testOutputDir = join(repoRoot, 'test-results', proofMode);
const schedulerDir = join(testOutputDir, 'local-scheduler');
const schedulerPath = join(schedulerDir, 'scheduler.jsonl');
const manifestPath = join(schedulerDir, 'manifest.json');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '55-notification-scheduler-handoff');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '55-notification-scheduler-handoff');
const commands = [];

await main();

async function main() {
  await mkdir(schedulerDir, { recursive: true });
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
    'tests/app-game-notification-scheduler-handoff.test.ts',
  ]);

  const bridge = await loadDistModule('app-game-notification-local-outbox-bridge');
  const handoff = await loadDistModule('app-game-notification-scheduler-handoff');
  const scheduler = await loadDistModule('notification-local-outbox-scheduler-proof');
  const notification = await loadDistModule('app-game-notification-intent');
  const refs = await loadDistModule('reference-primitives');
  const childUx = await loadDistModule('app-game-child-facing-ux-rules');
  await assertPackageExport(handoff);

  const fixtures = buildFixtures(notification, refs, childUx);
  const bridgeProof = bridge.buildAppGameNotificationLocalOutboxBridgeProof(fixtures.bridgeInput);
  const proofReadModel = handoff.buildAppGameNotificationSchedulerHandoffProof({
    generatedAt: fixtures.timestamp,
    schedulerNowAt: fixtures.schedulerNowAt,
    schedulerArtifactRootRef: 'parent-owned-app-game-notification-scheduler-root',
    bridgeProof,
  });
  const stateCounts = handoff.summarizeAppGameNotificationSchedulerHandoffStates(proofReadModel);
  const channelCounts = handoff.summarizeAppGameNotificationSchedulerHandoffChannels(proofReadModel);
  const dueRecords = handoff.dueAppGameNotificationSchedulerHandoffRecords(proofReadModel);
  const schedulerArtifact = await writeAndReadScheduler(scheduler, proofReadModel.records);

  assert.equal(proofReadModel.records.length, 3);
  assert.equal(proofReadModel.scheduledIntentRefs.length, 3);
  assert.equal(proofReadModel.blockedIntentRefs.length, 2);
  assert.equal(dueRecords.length, 3);
  assert.equal(stateCounts['due-local'], 3);
  assert.equal(stateCounts['held-quiet-hours'], 0);
  assert.equal(stateCounts['retry-window-scheduled'], 0);
  assert.equal(channelCounts.push, 1);
  assert.equal(channelCounts.email, 1);
  assert.equal(channelCounts['in-app'], 1);
  assert.equal(proofReadModel.providerDeliveryRuntimeClaimed, false);
  assert.equal(proofReadModel.providerReceiptIngestionClaimed, false);
  assert.equal(proofReadModel.providerCredentialsClaimed, false);
  assert.equal(proofReadModel.parentNotificationUiClaimed, false);
  assert.equal(proofReadModel.retryExecutionRuntimeClaimed, false);
  assert.equal(proofReadModel.quietHoursTimerRuntimeClaimed, false);
  assert.equal(proofReadModel.productionDurableOutboxStorageClaimed, false);
  assert.equal(proofReadModel.durableServicePersistenceClaimed, false);
  assert.equal(proofReadModel.childDeviceDeliveryClaimed, false);
  assert.equal(proofReadModel.adapterDispatchClaimed, false);
  assert.equal(proofReadModel.broadAppBlockingClaimed, false);
  assert.equal(proofReadModel.platformSupportClaimed, false);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    branch: await gitBranch(),
    commands,
    summary: {
      sourceBridgeRecords: bridgeProof.records.length,
      scheduledIntentCount: proofReadModel.scheduledIntentRefs.length,
      blockedIntentCount: proofReadModel.blockedIntentRefs.length,
      schedulerRecordCount: proofReadModel.records.length,
      dueLocalRecordCount: dueRecords.length,
      stateCounts,
      channelCounts,
      blockedReasons: countBy(proofReadModel.blockedIntentRefs.map((row) => row.blockReason)),
      nonClaims: proofReadModel.nonClaims,
    },
    claimsProved: [
      'app/game local outbox bridge rows are transformed into scheduler-ready due-local records',
      'scheduler rows preserve local outbox source entry refs, reason codes, provider channel preferences, severity, and parent-owned artifact refs',
      'manual-required and capability-unavailable app/game notification intents remain blocked and are not scheduled',
      'scheduler JSONL evidence rereads through the shared NotificationLocalOutboxSchedulerRecord schema',
      'provider delivery, provider receipts, credentials, cloud routing, parent notification UI, retry worker execution, quiet-hours timer execution, production durable storage, service persistence, child delivery, adapter dispatch, broad blocking, and platform support remain false',
    ],
    claimsNotProved: [
      'production scheduler timer loop or retry worker execution',
      'quiet-hours runtime enforcement',
      'external push/email/SMS/WhatsApp/in-app provider delivery',
      'provider receipt ingestion or credentials',
      'parent notification UI, history, or preferences',
      'durable service persistence or WebSocket notification read model',
      'child app or overlay delivery',
      'policy evaluator execution, adapter dispatch, broad app blocking, or platform support',
    ],
    evidence: {
      contract: 'packages/parent-domain/src/app-game-notification-scheduler-handoff.ts',
      bridgeContract: 'packages/parent-domain/src/app-game-notification-local-outbox-bridge.ts',
      schedulerContract: 'packages/parent-domain/src/notification-local-outbox-scheduler-proof.ts',
      test: 'packages/parent-domain/tests/app-game-notification-scheduler-handoff.test.ts',
      harness: 'scripts/test/app-game-notification-scheduler-handoff-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/55-notification-scheduler-handoff',
      appProofPack: 'output/app-plan-proof/55-notification-scheduler-handoff',
      schedulerJsonl: relativePath(schedulerPath),
      schedulerManifest: relativePath(manifestPath),
      packageExport: '@ocentra-parent/parent-domain/app-game-notification-scheduler-handoff',
    },
    schedulerArtifact,
    proofReadModel,
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP55');
  await writeProofPack(appProofDir, proof, 'app WP55');

  console.log(`${proofMode}-ok:${proofReadModel.records.length}`);
  console.log(`evidence=${relativePath(join(testOutputDir, 'proof.json'))}`);
}

async function loadDistModule(moduleName) {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', `${moduleName}.js`);
  return import(pathToFileURL(modulePath).href);
}

async function assertPackageExport(handoff) {
  const packageJson = JSON.parse(await readFile(join(repoRoot, 'packages', 'parent-domain', 'package.json'), 'utf8'));
  assert.deepEqual(packageJson.exports['./app-game-notification-scheduler-handoff'], {
    import: './dist/app-game-notification-scheduler-handoff.js',
    types: './dist/app-game-notification-scheduler-handoff.d.ts',
  });

  const exportedModule = await import('@ocentra-parent/parent-domain/app-game-notification-scheduler-handoff');
  assert.equal(
    exportedModule.AppGameNotificationSchedulerHandoffNonClaims.length,
    handoff.AppGameNotificationSchedulerHandoffNonClaims.length
  );
}

async function writeAndReadScheduler(scheduler, records) {
  const serialized = `${records.map((record) => JSON.stringify(record)).join('\n')}\n`;
  assertNoForbiddenDetails(serialized);
  await writeFile(schedulerPath, serialized, 'utf8');
  await writeJson(manifestPath, {
    proofMode,
    schedulerFile: relativePath(schedulerPath),
    recordCount: records.length,
    schedulerArtifactRef: records[0].schedulerArtifactRef,
    localDataPathRef: records[0].localDataPathRef,
    generatedAt: new Date().toISOString(),
  });

  const parsed = (await readFile(schedulerPath, 'utf8'))
    .trim()
    .split('\n')
    .map((line) => JSON.parse(line))
    .map((record) => scheduler.NotificationLocalOutboxSchedulerRecordSchema.parse(record));
  return {
    schedulerFile: relativePath(schedulerPath),
    manifest: relativePath(manifestPath),
    recordsWritten: parsed.length,
    schedulerEntryIds: parsed.map((record) => record.schedulerEntryId),
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
    assert.equal(lowerSerialized.includes(fragment), false, `forbidden scheduler detail leaked: ${fragment}`);
  }
}

function buildFixtures(notification, refs, childUx) {
  const timestamp = '2026-06-04T19:14:00Z';
  const schedulerNowAt = '2026-06-04T19:15:00Z';
  const policyVersion = 'policy-app-game-notification-scheduler-v1';
  const family = {
    familyId: 'family-app-game-notification-scheduler',
  };
  const device = {
    deviceId: 'device-app-game-notification-scheduler',
    childProfileId: 'child-app-game-notification-scheduler',
    label: 'Study PC',
    platform: refs.ParentPlatform.Windows,
  };
  const parentAction = {
    actionReferenceId: 'parent-action-app-game-notification-scheduler',
    actor: { actorId: 'parent-app-game-notification-scheduler', role: refs.ParentActorRole.Parent },
    policyVersion,
    createdAt: timestamp,
  };
  const evidence = {
    evidenceReferenceId: 'evidence-app-game-notification-scheduler-session',
    kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
    observedAt: timestamp,
  };
  const approvalActionRef = {
    actionReferenceId: 'approval-action-app-game-notification-scheduler',
    actor: { actorId: 'child-local-agent', role: refs.ParentActorRole.System },
    policyVersion,
    createdAt: timestamp,
  };
  const base = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    notificationIntentId: 'notification-intent-scheduler-time-limit',
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
    localOutboxRecordRef: 'local-outbox-record-scheduler-game-limit',
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
    notificationIntentId: 'notification-intent-scheduler-approval-request',
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
    localOutboxRecordRef: 'local-outbox-record-scheduler-approval-request',
  };
  const suspicious = {
    ...approval,
    notificationIntentId: 'notification-intent-scheduler-suspicious-unknown',
    intentKind: notification.AppGameNotificationIntentKind.SuspiciousUnknown,
    notificationReasonCode: notification.AppGameNotificationReasonCode.SuspiciousUnknown,
    providerChannelPreference: 'email',
    parentTitleToken: notification.AppGameNotificationParentCopyToken.SuspiciousUnknownTitle,
    parentBodyToken: notification.AppGameNotificationParentCopyToken.SuspiciousUnknownBody,
    approvalActionRef: null,
    localOutboxRecordRef: 'local-outbox-record-scheduler-suspicious-unknown',
  };
  const manual = manualOrUnavailable(notification, childUx, base, false);
  const unavailable = manualOrUnavailable(notification, childUx, base, true);
  return {
    timestamp,
    schedulerNowAt,
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
    notificationIntentId: unavailable
      ? 'notification-intent-scheduler-unavailable'
      : 'notification-intent-scheduler-manual-required',
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
      '- Scope: parent-domain app/game notification scheduler handoff proof.',
      '- Source inspected: WP54 local outbox bridge and shared notification scheduler record contract.',
      '- Product checklist intentionally not edited; remaining delta is reported through hub.',
    ].join('\n') + '\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tests/app-game-notification-scheduler-handoff.test.ts: PASS',
      '- App/game local outbox bridge rows create due-local scheduler rows.',
      '- Manual-required and unavailable app/game notification intents remain blocked from scheduler rows.',
      '- Invalid provider, timer, UI, persistence, broad-blocking, platform, and scheduler-link claims are rejected.',
    ].join('\n') + '\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust/service protocol not changed. This is TypeScript parent-domain scheduler handoff proof only.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof);
  await writeJson(join(proofDir, '04-journal-sqlite-proof.json'), {
    schemaVersion: 1,
    journalSqliteChanged: false,
    reason: 'No journal, SQLite, service read-model, runtime timer, or runtime persistence code changed.',
  });
  await writeJson(join(proofDir, '05-policy-action-proof.json'), {
    schemaVersion: 1,
    stateCounts: proof.summary.stateCounts,
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
      '- Scheduler rows carry refs only and exclude raw child evidence.',
      '- Manual-required and unavailable intents are not scheduled.',
      '- Provider delivery, provider receipts, credentials, cloud routing, parent notification UI, retry execution, quiet-hours timer execution, durable storage, service persistence, child-device delivery, adapter dispatch, broad app blocking, and platform support remain false.',
      '- Scheduler JSONL proof artifact is scanned for forbidden raw-detail fragments.',
    ].join('\n') + '\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\nNo provider, timer, child-device UI, service runtime, or platform proof is attached. Delivery remains unclaimed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [
      'Validation run:',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tests/app-game-notification-scheduler-handoff.test.ts: PASS',
      '- node scripts/test/app-game-notification-scheduler-handoff-proof.mjs: PASS',
    ].join('\n') + '\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\nNo authority tier is raised. The handoff remains parent-domain proof with adapter dispatch disabled.\n',
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
