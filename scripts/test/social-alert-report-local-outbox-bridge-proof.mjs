import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const generatedAt = '2026-06-07T07:04:00Z';
const outputDirectory = join(repoRoot, 'output', 'browser-plan-proof', 'social-alert-report-local-outbox-bridge-proof');
const resultDirectory = join(repoRoot, 'test-results', 'social-alert-report-local-outbox-bridge-proof');
const commands = [];

await main();

async function main() {
  await mkdir(outputDirectory, { recursive: true });
  await mkdir(resultDirectory, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'social-alert-report-local-outbox-bridge',
    'social-alert-report-intent',
    'notification-local-outbox-adapter-proof',
  ]);

  const bridge = await importDist('social-alert-report-local-outbox-bridge.js');
  const readModel = bridge.buildSocialAlertReportLocalOutboxBridgeReadModel(bridgeOptions(), proofIntents());
  const jsonl = bridge.serializeSocialAlertReportLocalOutboxJsonl(readModel);
  const rereadRecords = bridge.parseSocialAlertReportLocalOutboxJsonl(jsonl);
  const source = await readText('packages/parent-domain/src/social-alert-report-local-outbox-bridge.ts');
  const test = await readText('packages/parent-domain/tests/social-alert-report-local-outbox-bridge.test.ts');
  const socialFeature = await readText('docs/features/social-video-control.md');
  const socialExpectation = await readText('docs/expectations/social-video-control.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/readme.md');

  await writeFile(join(resultDirectory, 'local-outbox-records.jsonl'), jsonl, 'utf8');

  const checks = [
    checkFile('packages/parent-domain/src/social-alert-report-local-outbox-bridge.ts'),
    checkFile('packages/parent-domain/tests/social-alert-report-local-outbox-bridge.test.ts'),
    checkFile('scripts/test/social-alert-report-local-outbox-bridge-proof.mjs'),
    checkIncludes(source, 'providerDeliveryRuntimeClaimed: Schema.Literal(false)', 'provider delivery non-claim guard'),
    checkIncludes(source, 'finalPolicyExecutionClaimed: Schema.Literal(false)', 'final policy non-claim guard'),
    checkIncludes(source, 'enforcementClaimed: Schema.Literal(false)', 'enforcement non-claim guard'),
    checkIncludes(test, 'providerDeliveryRuntimeClaimed: true', 'provider delivery rejection test'),
    checkIncludes(test, 'providerDeliveryAttempted: true', 'unsafe JSONL rejection test'),
    checkIncludes(socialFeature, 'social alert/report local outbox bridge', 'social feature bridge note'),
    checkIncludes(socialExpectation, 'Social alert/report local outbox bridge', 'social expectation bridge note'),
    checkIncludes(workpackReadme, 'social-alert-report-local-outbox-bridge-proof', 'social README bridge proof note'),
  ];
  const failures = checks.filter((check) => !check.pass).map((check) => check.label);

  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-local-outbox-bridge-proof',
    generatedAt,
    branch: await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: await gitOutput(['rev-parse', 'HEAD']),
    commands,
    checks,
    summary: {
      linkedRecordCount: readModel.linkedRecordCount,
      manualRequiredCount: readModel.manualRequiredCount,
      unavailableCount: readModel.unavailableCount,
      jsonlRecordCount: rereadRecords.length,
      providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
      providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
      schedulerRuntimeClaimed: readModel.schedulerRuntimeClaimed,
      cloudRoutingClaimed: readModel.cloudRoutingClaimed,
      parentNotificationUiClaimed: readModel.parentNotificationUiClaimed,
      reportDeliveryExecutionClaimed: readModel.reportDeliveryExecutionClaimed,
      finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
      enforcementClaimed: readModel.enforcementClaimed,
    },
    claimsProved: [
      'Local-outbox-eligible social alert/report intents parse through the parent-domain intent contract before outbox linking',
      'Eligible intents become existing NotificationLocalOutboxRecord rows with minimal payload fields and parent-owned local path refs',
      'JSONL output is reread through the existing local outbox record parser',
      'Manual-required and unavailable intents are visible in the bridge read model but do not produce queued JSONL records',
      'Provider delivery, receipts, scheduler runtime, cloud routing, parent notification UI, report delivery execution, final policy execution, and enforcement remain unclaimed',
    ],
    claimsNotProved: [
      'provider adapter delivery, credentials, or receipt ingestion',
      'production quiet-hours scheduler or retry worker runtime',
      'parent notification UI or notification history UI',
      'external report delivery execution',
      'connector or native-app runtime',
      'final policy evaluator execution',
      'enforcement',
      'product checklist completion',
    ],
    evidence: {
      source: 'packages/parent-domain/src/social-alert-report-local-outbox-bridge.ts',
      test: 'packages/parent-domain/tests/social-alert-report-local-outbox-bridge.test.ts',
      existingIntentContract: 'packages/parent-domain/src/social-alert-report-intent.ts',
      existingOutboxContract: 'packages/parent-domain/src/notification-local-outbox-adapter-proof.ts',
      harness: 'scripts/test/social-alert-report-local-outbox-bridge-proof.mjs',
      jsonl: 'test-results/social-alert-report-local-outbox-bridge-proof/local-outbox-records.jsonl',
      proof: 'test-results/social-alert-report-local-outbox-bridge-proof/proof.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-local-outbox-bridge-proof/01-social-alert-report-local-outbox-bridge-proof.md',
    },
    readModel,
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report local outbox bridge proof failed:\n${failures.join('\n')}`);
  }

  await writeJson(join(resultDirectory, 'proof.json'), proof);
  await writeJson(join(resultDirectory, 'local-outbox-bridge-read-model.json'), readModel);
  await writeFile(join(outputDirectory, '00-source-snapshot.md'), await sourceSnapshot(proof), 'utf8');
  await writeFile(
    join(outputDirectory, '01-social-alert-report-local-outbox-bridge-proof.md'),
    markdownFor(proof),
    'utf8'
  );
  await writeFile(join(outputDirectory, '08-security-negative-proof.md'), securityProofFor(proof), 'utf8');
  await writeFile(join(outputDirectory, '10-validation-commands.log'), validationLogFor(proof), 'utf8');
  await writeFile(
    join(outputDirectory, 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nThis proof adds a parent-domain bridge from social alert/report intents to the existing parent-owned local notification outbox record schema. It does not change rendered portal UI.\n',
    'utf8'
  );

  console.log('social-alert-report-local-outbox-bridge-proof-ok=true');
  console.log(`proof=${relativePath(join(resultDirectory, 'proof.json'))}`);
  console.log(`manifest=${relativePath(join(outputDirectory, '01-social-alert-report-local-outbox-bridge-proof.md'))}`);
}

function bridgeOptions() {
  return {
    family: { familyId: 'family-social-alert-report-outbox-bridge-proof' },
    parentAction: {
      actionReferenceId: 'parent-action-social-alert-report-outbox-bridge-proof',
      actor: { actorId: 'parent-social-alert-report-outbox-bridge-proof', role: 'parent' },
      policyVersion: 'policy-social-alert-report-outbox-proof-v1',
      createdAt: generatedAt,
    },
    generatedAt,
    bridgeId: 'social-alert-report-local-outbox-bridge-proof',
    outboxRootRef: 'parent-owned-social-alert-report-local-outbox-root',
    outboxFileRef: 'parent-owned-social-alert-report-local-outbox-jsonl-ref',
    localDataPathRef: 'parent-owned-social-alert-report-local-outbox-data-path-ref',
  };
}

function proofIntents() {
  const base = {
    schemaVersion: 'v0.6',
    alertReportIntentId: 'social-alert-report-high-risk-outbox-proof',
    intentKind: 'high-risk-signal',
    intentStatus: 'local-outbox-eligible',
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-alert-report-outbox-proof',
      childProfileId: 'child-social-alert-report-outbox-proof',
      label: 'Study Phone',
      platform: 'android',
    },
    notificationReasonCode: 'social-high-risk-signal',
    providerChannelPreference: 'in-app',
    parentTitleToken: 'social.alert.highRisk.title',
    parentBodyToken: 'social.alert.highRisk.body',
    parentActionToken: 'social.alert.action.openParentReview',
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-outbox-proof',
    explanationEventRefs: ['social-explanation-event-outbox-proof'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-alert-report-outbox-proof',
        kind: 'policy-decision',
        observedAt: generatedAt,
      },
    ],
    policyRefs: ['policy-ref-social-alert-report-outbox-proof'],
    auditRefs: ['audit-ref-social-alert-report-outbox-proof'],
    parentReportRef: null,
    parentActionRef: null,
    localOutboxRecordRef: 'local-outbox-record-social-alert-report-high-risk-proof',
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
      'explanation-ref',
      'parent-action-link-ref',
    ],
    deliveryClaimState: 'local-outbox-only',
    rawAccountDataIncluded: false,
    rawVideoContentIncluded: false,
    rawMessageContentIncluded: false,
    screenshotIncluded: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    reportDeliveryClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    adapterDispatchState: 'not-dispatched',
    adapterActionClaimed: false,
    createdAt: generatedAt,
  };

  return [
    base,
    {
      ...base,
      alertReportIntentId: 'social-alert-report-account-approval-outbox-proof',
      intentKind: 'account-approval-needed',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-account-approval-needed',
      providerChannelPreference: 'email',
      parentTitleToken: 'social.alert.accountApproval.title',
      parentBodyToken: 'social.alert.accountApproval.body',
      parentActionRef: bridgeOptions().parentAction,
      localOutboxRecordRef: 'local-outbox-record-social-alert-report-account-approval-proof',
    },
    {
      ...base,
      alertReportIntentId: 'social-alert-report-manual-required-outbox-proof',
      intentKind: 'manual-required',
      intentStatus: 'manual-required',
      priority: 'attention',
      severity: 'warning',
      notificationReasonCode: 'social-manual-review-required',
      parentTitleToken: 'social.alert.manualRequired.title',
      parentBodyToken: 'social.alert.manualRequired.body',
      parentActionToken: 'social.alert.action.reviewManually',
      localOutboxRecordRef: null,
      manualProofRequirements: ['provider preference setup before social alert/report can be queued'],
      deliveryClaimState: 'manual-required',
    },
    {
      ...base,
      alertReportIntentId: 'social-alert-report-unavailable-outbox-proof',
      intentKind: 'capability-unavailable',
      intentStatus: 'unavailable',
      priority: 'info',
      severity: 'info',
      notificationReasonCode: 'social-capability-unavailable',
      parentTitleToken: 'social.alert.unavailable.title',
      parentBodyToken: 'social.alert.unavailable.body',
      parentActionToken: 'social.alert.action.reviewManually',
      localOutboxRecordRef: null,
      manualProofRequirements: [
        'local evidence and policy readiness before unavailable social alert/report can be queued',
      ],
      deliveryClaimState: 'manual-required',
    },
  ];
}

async function sourceSnapshot(proof) {
  return [
    '# Social Alert Report Local Outbox Bridge Source Snapshot',
    '',
    `- Branch: ${proof.branch}`,
    `- Commit: ${proof.commit}`,
    '- Scope: social alert/report intent bridge to existing parent-owned notification local outbox record schema.',
    '- Source inspected: social alert/report intent contract, notification local outbox adapter proof, social/video feature doc, social/video expectation doc, and browser-plan social rollout gates.',
    '- Package export was intentionally not changed because another lane currently owns packages/parent-domain/package.json.',
    '',
  ].join('\n');
}

function markdownFor(proof) {
  return [
    '# Social Alert Report Local Outbox Bridge Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Linked local outbox rows: ${proof.summary.linkedRecordCount}`,
    `Manual-required rows: ${proof.summary.manualRequiredCount}`,
    `Unavailable rows: ${proof.summary.unavailableCount}`,
    `JSONL records reread: ${proof.summary.jsonlRecordCount}`,
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'This proof consumes parsed social alert/report intents and links only',
    'local-outbox-eligible rows to the existing parent-owned',
    '`NotificationLocalOutboxRecord` schema. The generated JSONL is reread',
    'through the same parser. Manual-required and unavailable rows remain in',
    'the bridge read model but do not produce queued local outbox records.',
    '',
    'It does not claim provider delivery execution, receipt ingestion, provider',
    'credentials, scheduler runtime, cloud routing, parent notification UI',
    'delivery, report delivery execution, final policy execution, connector or',
    'native runtime, or enforcement.',
    '',
  ].join('\n');
}

function securityProofFor(proof) {
  return [
    '# Social Alert Report Local Outbox Bridge Security Negative Proof',
    '',
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Provider receipt ingestion claimed: ${proof.summary.providerReceiptIngestionClaimed}`,
    `Parent notification UI claimed: ${proof.summary.parentNotificationUiClaimed}`,
    `Report delivery execution claimed: ${proof.summary.reportDeliveryExecutionClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
    '',
    'The unit test rejects provider-delivery overclaims and rejects JSONL rows',
    'that attempt to set provider delivery flags before a provider adapter exists.',
    '',
  ].join('\n');
}

function validationLogFor(proof) {
  return proof.commands.map((command) => `${command.exitCode === 0 ? 'PASS' : 'FAIL'} ${command.command}`).join('\n');
}

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function checkFile(path) {
  return {
    label: `${path} exists`,
    pass: existsSync(join(repoRoot, path)),
  };
}

function checkIncludes(text, expected, label) {
  return {
    label,
    pass: text.includes(expected),
  };
}

async function readText(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  const result = await new Promise((resolve) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
    child.on('close', (exitCode) => resolve({ exitCode: exitCode ?? 1 }));
  });
  commands.push({ command: commandLine, exitCode: result.exitCode });
  if (result.exitCode !== 0) {
    throw new Error(`${commandLine} exited with ${result.exitCode}`);
  }
}

async function gitOutput(args) {
  const result = await new Promise((resolve) => {
    const child = spawn('git', args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], shell: false });
    let stdout = '';
    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });
    child.on('close', (exitCode) => resolve({ exitCode: exitCode ?? 1, stdout }));
  });
  if (result.exitCode !== 0) {
    return 'unknown';
  }
  return result.stdout.trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
