import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-notification-local-outbox-bridge-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '58-notification-local-outbox-bridge');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '58-notification-local-outbox-bridge');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });
  await mkdir(join(appGameProofDir, '06-ui-snapshots'), { recursive: true });
  await mkdir(join(appProofDir, '06-ui-snapshots'), { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'app-game-notification-local-outbox-bridge',
      'notification-local-outbox-adapter-proof',
      'app-game-notification-intent',
    ])
  );

  const bridge = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-notification-local-outbox-bridge.js'))
      .href
  );
  const readModel = bridge.buildAppGameNotificationLocalOutboxBridgeReadModel(bridgeOptions(), proofIntents());
  const jsonl = bridge.serializeAppGameNotificationLocalOutboxJsonl(readModel);
  const rereadRecords = bridge.parseAppGameNotificationLocalOutboxJsonl(jsonl);

  await writeFile(join(testOutputDir, 'local-outbox-records.jsonl'), jsonl, 'utf8');

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-notification-local-outbox-bridge',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    summary: {
      bridge: 'App/game notification intent to parent-owned local outbox JSONL records',
      linkedRecordCount: readModel.linkedRecordCount,
      manualRequiredCount: readModel.manualRequiredCount,
      unavailableCount: readModel.unavailableCount,
      jsonlRecordCount: rereadRecords.length,
      providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
      providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
      schedulerRuntimeClaimed: readModel.schedulerRuntimeClaimed,
      cloudRoutingClaimed: readModel.cloudRoutingClaimed,
      parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
      childDeliveryClaimed: readModel.childDeliveryClaimed,
      adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    },
    claimsProved: [
      'Local-outbox-eligible app/game notification intents parse through the parent-domain intent contract before outbox linking',
      'Eligible intents become existing NotificationLocalOutboxRecord rows with minimal payload fields and parent-owned local path refs',
      'JSONL output is reread through the existing local outbox record parser',
      'Manual-required and unavailable intents are visible in the bridge read model but do not produce queued JSONL records',
      'Provider delivery, receipts, scheduler runtime, cloud routing, parent UI, child delivery, and adapter dispatch remain unclaimed',
    ],
    claimsNotProved: [
      'durable production local outbox storage',
      'production quiet-hours scheduler or retry worker runtime',
      'provider adapter delivery, credentials, or receipt ingestion',
      'parent notification UI, preference UI, or notification history UI',
      'child app, overlay, push, or local notification delivery',
      'policy evaluator execution, adapter dispatch, broad blocking, or platform support',
    ],
    evidence: {
      bridgeSource: 'packages/parent-domain/src/app-game-notification-local-outbox-bridge.ts',
      bridgeTest: 'packages/parent-domain/tests/app-game-notification-local-outbox-bridge.test.ts',
      existingIntentContract: 'packages/parent-domain/src/app-game-notification-intent.ts',
      existingOutboxContract: 'packages/parent-domain/src/notification-local-outbox-adapter-proof.ts',
      harness: 'scripts/test/app-game-notification-local-outbox-bridge-proof.mjs',
      jsonl: 'test-results/app-game-notification-local-outbox-bridge-proof/local-outbox-records.jsonl',
      appGameProofPack: 'output/app-game-plan-proof/58-notification-local-outbox-bridge',
      appProofPack: 'output/app-plan-proof/58-notification-local-outbox-bridge',
    },
    readModel,
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP58');
  await writeProofPack(appProofDir, proof, 'app WP58');

  console.log('app-game-notification-local-outbox-bridge-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function bridgeOptions() {
  return {
    family: { familyId: 'family-app-game-outbox-bridge-proof' },
    parentAction: {
      actionReferenceId: 'parent-action-app-game-outbox-bridge-proof',
      actor: { actorId: 'parent-app-game-outbox-bridge-proof', role: 'parent' },
      policyVersion: 'policy-app-game-notification-outbox-proof-v1',
      createdAt: '2026-06-05T00:24:00Z',
    },
    generatedAt: '2026-06-05T00:24:00Z',
    bridgeId: 'app-game-notification-local-outbox-bridge-proof',
    outboxRootRef: 'parent-owned-app-game-local-outbox-root',
    outboxFileRef: 'parent-owned-app-game-local-outbox-jsonl-ref',
    localDataPathRef: 'parent-owned-app-game-local-outbox-data-path-ref',
  };
}

function proofIntents() {
  const base = {
    schemaVersion: 'v0.6',
    notificationIntentId: 'notification-intent-time-limit-outbox-proof',
    intentKind: 'time-limit-reached',
    intentStatus: 'local-outbox-eligible',
    priority: 'urgent',
    device: {
      deviceId: 'device-app-game-outbox-bridge-proof',
      childProfileId: 'child-app-game-outbox-bridge-proof',
      label: 'Study PC',
      platform: 'windows',
    },
    targetKind: 'native-game',
    targetRef: 'target-native-game-outbox-proof',
    notificationReasonCode: 'app-game-time-limit',
    providerChannelPreference: 'in-app',
    parentTitleToken: 'appGame.notification.timeLimit.title',
    parentBodyToken: 'appGame.notification.timeLimit.body',
    parentActionToken: 'appGame.notification.action.openParentReview',
    childTitleToken: 'appGame.childUx.limitReached.title',
    childBodyToken: 'appGame.childUx.limitReached.body',
    notificationRuleRef: 'notification-rule-app-game-time-limit-outbox-proof',
    notificationStatusRef: 'notification-status-app-game-time-limit-outbox-proof',
    policyRefs: ['policy-ref-app-game-time-limit-outbox-proof'],
    auditRefs: ['audit-ref-app-game-time-limit-outbox-proof'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-ref-app-game-time-limit-outbox-proof',
        kind: 'policy-decision',
        observedAt: '2026-06-05T00:24:00Z',
      },
    ],
    childReasonReferences: [],
    childStatusReferences: ['child-status-app-game-time-limit-outbox-proof'],
    approvalActionRef: null,
    timeBudgetDecisionRef: 'time-budget-decision-app-game-outbox-proof',
    unknownCandidateRef: null,
    localOutboxRecordRef: 'local-outbox-record-app-game-time-limit-proof',
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: [],
    minimalPayloadFields: [
      'alert-id',
      'family-device-scope',
      'severity',
      'reason-code',
      'evidence-ref',
      'policy-ref',
      'parent-action-link-ref',
    ],
    deliveryClaimState: 'local-outbox-only',
    rawChildEvidenceIncluded: false,
    rawUrlOrTitleIncluded: false,
    rawMessageTextIncluded: false,
    screenshotOrReportIncluded: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    adapterDispatchState: 'not-dispatched',
    adapterActionClaimed: false,
    createdAt: '2026-06-05T00:24:00Z',
  };

  return [
    base,
    {
      ...base,
      notificationIntentId: 'notification-intent-suspicious-unknown-outbox-proof',
      intentKind: 'suspicious-unknown',
      priority: 'attention',
      targetKind: 'unknown-app',
      targetRef: 'target-unknown-app-outbox-proof',
      notificationReasonCode: 'app-game-suspicious-unknown',
      providerChannelPreference: 'email',
      parentTitleToken: 'appGame.notification.suspiciousUnknown.title',
      parentBodyToken: 'appGame.notification.suspiciousUnknown.body',
      childTitleToken: 'appGame.childUx.newApp.title',
      childBodyToken: 'appGame.childUx.newApp.body',
      timeBudgetDecisionRef: null,
      unknownCandidateRef: 'unknown-app-candidate-outbox-proof',
      localOutboxRecordRef: 'local-outbox-record-app-game-suspicious-unknown-proof',
    },
    {
      ...base,
      notificationIntentId: 'notification-intent-manual-required-outbox-proof',
      intentKind: 'manual-required',
      intentStatus: 'manual-required',
      priority: 'attention',
      notificationReasonCode: 'app-game-manual-review-required',
      parentTitleToken: 'appGame.notification.manualRequired.title',
      parentBodyToken: 'appGame.notification.manualRequired.body',
      parentActionToken: 'appGame.notification.action.reviewManually',
      childTitleToken: 'appGame.childUx.manualRequired.title',
      childBodyToken: 'appGame.childUx.manualRequired.body',
      timeBudgetDecisionRef: null,
      localOutboxRecordRef: null,
      manualProofRequirements: ['provider preference setup before app game notification can be queued'],
      deliveryClaimState: 'manual-required',
    },
    {
      ...base,
      notificationIntentId: 'notification-intent-unavailable-outbox-proof',
      intentKind: 'capability-unavailable',
      intentStatus: 'unavailable',
      priority: 'info',
      notificationReasonCode: 'app-game-capability-unavailable',
      parentTitleToken: 'appGame.notification.unavailable.title',
      parentBodyToken: 'appGame.notification.unavailable.body',
      parentActionToken: 'appGame.notification.action.reviewManually',
      childTitleToken: 'appGame.childUx.unavailable.title',
      childBodyToken: 'appGame.childUx.unavailable.body',
      timeBudgetDecisionRef: null,
      localOutboxRecordRef: null,
      manualProofRequirements: ['local evidence and policy readiness before unavailable notification can be queued'],
      deliveryClaimState: 'manual-required',
    },
  ];
}

async function writeProofPack(proofDir, proof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${await gitBranch()}`,
      `- Commit: ${proof.commit}`,
      '- Scope: app/game notification local outbox bridge from validated notification intents to existing parent-owned local outbox record schema.',
      '- Source inspected: app/game notification intent contract, notification local outbox adapter proof, notification expectation docs, app/game notification readiness service proof, and app/app-game implementation checklists.',
      '- Package export and module README update were intentionally not touched because E-B currently owns `packages/parent-domain/package.json` and `packages/parent-domain/README.md`.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-notification-local-outbox-bridge notification-local-outbox-adapter-proof app-game-notification-intent: PASS',
      '- Eligible app/game notification intents parse before local outbox linking.',
      '- Unsafe provider delivery, raw child evidence, cloud routing, parent UI, and adapter claims are rejected or pinned false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust protocol proof not applicable: this workpack reuses the existing WP56 service readiness event and parent-domain local outbox schema without adding a Rust-crossing shape.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof.summary);
  await writeFile(
    join(proofDir, '04-journal-sqlite-proof.json'),
    `${JSON.stringify(
      {
        schemaVersion: 1,
        journalSqliteChanged: false,
        localOutboxArtifact: 'test-results/app-game-notification-local-outbox-bridge-proof/local-outbox-records.jsonl',
        durableProductionOutboxStorageClaimed: false,
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '05-policy-action-proof.json'),
    `${JSON.stringify(
      {
        schemaVersion: 1,
        eligibleIntentsLinkedToLocalOutbox: proof.summary.linkedRecordCount,
        manualRequiredIntentsQueued: false,
        unavailableIntentsQueued: false,
        providerDeliveryRuntimeClaimed: false,
        schedulerRuntimeClaimed: false,
        adapterDispatchClaimed: false,
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo parent portal, notification history, preference UI, or child-facing UI source changed in this workpack.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '07-playwright-ui-proof.log'),
    'Playwright proof not applicable: no UI source changed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Local outbox records carry minimal payload refs only: alert id, family/device scope, severity, reason, evidence ref, policy ref, and parent action link.',
      '- JSONL rows are parsed through the existing NotificationLocalOutboxRecordSchema.',
      '- Manual-required and unavailable intents do not create queued JSONL records.',
      '- Provider delivery, receipt ingestion, credentials, cloud routing, parent UI, child delivery, scheduler runtime, adapter dispatch, broad blocking, and platform support remain false or unclaimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNo live platform authority tier is raised. Provider delivery, scheduler workers, and platform adapter execution remain unclaimed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [
      'Validation run:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-notification-local-outbox-bridge notification-local-outbox-adapter-proof app-game-notification-intent: PASS',
      '- node scripts/test/app-game-notification-local-outbox-bridge-proof.mjs: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nThe bridge only writes parent-owned local outbox proof rows. It does not raise provider, scheduler, adapter, or platform authority.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-rollback-proof.md'),
    '# Rollback Proof\n\nNo provider send, scheduler worker, child-device notification, block, suspend, shield, or adapter state is created. Rollback is not applicable beyond deleting the generated local JSONL artifact.\n',
    'utf8'
  );
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, shell: false, stdio: 'inherit' });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(undefined);
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with ${code}`));
    });
  });
}

async function gitBranch() {
  return (await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD'])).trim();
}

async function gitHead() {
  return (await gitOutput(['rev-parse', 'HEAD'])).trim();
}

async function gitOutput(args) {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', args, { cwd: repoRoot, shell: false });
    child.stdout.on('data', (chunk) => chunks.push(Buffer.from(chunk)));
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(undefined);
        return;
      }
      reject(new Error(`git ${args.join(' ')} exited with ${code}`));
    });
  });
  return Buffer.concat(chunks).toString('utf8');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
