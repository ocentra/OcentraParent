import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const generatedAt = '2026-06-07T07:24:00Z';
const outputDirectory = join(
  repoRoot,
  'output',
  'browser-plan-proof',
  'social-alert-report-parent-surface-intent-proof'
);
const resultDirectory = join(repoRoot, 'test-results', 'social-alert-report-parent-surface-intent-proof');
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
    'social-alert-report-parent-surface-intent-proof',
    'social-alert-report-provider-status-handoff-proof',
  ]);

  const parentSurface = await importDist('social-alert-report-parent-surface-intent-proof.js');
  const providerStatus = await importDist('social-alert-report-provider-status-handoff-proof.js');
  const providerPreflight = await importDist('social-alert-report-provider-preflight-proof.js');
  const refs = await importDist('reference-primitives.js');
  const readModel = parentSurface.buildSocialAlertReportParentSurfaceIntentReadModel(
    {
      generatedAt,
      intentId: 'social-alert-report-parent-surface-intent-proof',
      sourceContractRefs: [
        'social-alert-report-provider-status-handoff-proof',
        'social-alert-report-local-outbox-bridge-proof',
        'notifications-expectation-parent-surface-boundary',
      ],
    },
    providerStatus.SocialAlertReportProviderStatusHandoffReadModelSchema.parse(
      providerStatusReadModel(providerPreflight, refs)
    )
  );
  const source = await readText('packages/parent-domain/src/social-alert-report-parent-surface-intent-proof.ts');
  const test = await readText('packages/parent-domain/tests/social-alert-report-parent-surface-intent-proof.test.ts');
  const socialFeature = await readText('docs/features/social-video-control.md');
  const socialExpectation = await readText('docs/expectations/social-video-control.md');
  const workpackReadme = await readText('docs/plans/browser-plan/social-platform-account-feed/README.md');

  const checks = [
    checkFile('packages/parent-domain/src/social-alert-report-parent-surface-intent-proof.ts'),
    checkFile('packages/parent-domain/tests/social-alert-report-parent-surface-intent-proof.test.ts'),
    checkFile('scripts/test/social-alert-report-parent-surface-intent-proof.mjs'),
    checkIncludes(source, 'parentNotificationUiRendered: Schema.Literal(false)', 'parent UI rendered non-claim guard'),
    checkIncludes(source, 'providerDeliveryRuntimeClaimed: Schema.Literal(false)', 'provider delivery non-claim guard'),
    checkIncludes(source, 'finalPolicyExecutionClaimed: Schema.Literal(false)', 'final policy non-claim guard'),
    checkIncludes(test, 'parentNotificationUiRendered: true', 'parent UI rejection test'),
    checkIncludes(test, 'finalPolicyExecutionClaimed: true', 'final policy rejection test'),
    checkIncludes(socialFeature, 'social alert/report parent-surface intent', 'social feature parent surface note'),
    checkIncludes(
      socialExpectation,
      'Social alert/report parent-surface intent',
      'social expectation parent surface note'
    ),
    checkIncludes(
      workpackReadme,
      'social-alert-report-parent-surface-intent-proof',
      'social README parent surface proof note'
    ),
  ];
  const failures = checks.filter((check) => !check.pass).map((check) => check.label);
  const proof = {
    schemaVersion: 1,
    proofMode: 'social-alert-report-parent-surface-intent-proof',
    generatedAt,
    branch: await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: await gitOutput(['rev-parse', 'HEAD']),
    commands,
    checks,
    summary: {
      rowCount: readModel.rows.length,
      manualActionRequiredCount: readModel.manualActionRequiredCount,
      unavailableVisibleCount: readModel.unavailableVisibleCount,
      historyVisibleCount: readModel.historyVisibleCount,
      parentNotificationUiRendered: readModel.parentNotificationUiRendered,
      providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
      providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
      reportDeliveryExecutionClaimed: readModel.reportDeliveryExecutionClaimed,
      finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
      connectorNativeRuntimeClaimed: readModel.connectorNativeRuntimeClaimed,
      enforcementClaimed: readModel.enforcementClaimed,
    },
    claimsProved: [
      'Social alert/report provider-status handoff rows can be projected into parent-visible manual/unavailable surface intent rows',
      'Rows keep notification status refs, readiness refs, audit refs, and manual proof requirements for future parent UI drill-in',
      'The read model rejects parent notification UI rendering, provider delivery, provider receipts, report delivery execution, final policy execution, connector/native runtime, and enforcement claims',
    ],
    claimsNotProved: [
      'rendered parent notification UI',
      'provider adapter delivery, credentials, or receipt ingestion',
      'external report delivery execution',
      'connector or native-app runtime',
      'final policy evaluator execution',
      'enforcement',
      'product checklist completion',
    ],
    evidence: {
      source: 'packages/parent-domain/src/social-alert-report-parent-surface-intent-proof.ts',
      test: 'packages/parent-domain/tests/social-alert-report-parent-surface-intent-proof.test.ts',
      providerStatusSource: 'packages/parent-domain/src/social-alert-report-provider-status-handoff-proof.ts',
      harness: 'scripts/test/social-alert-report-parent-surface-intent-proof.mjs',
      proof: 'test-results/social-alert-report-parent-surface-intent-proof/proof.json',
      readModel: 'test-results/social-alert-report-parent-surface-intent-proof/parent-surface-intent-read-model.json',
      manifest:
        'output/browser-plan-proof/social-alert-report-parent-surface-intent-proof/01-social-alert-report-parent-surface-intent-proof.md',
    },
    readModel,
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Social alert/report parent surface intent proof failed:\n${failures.join('\n')}`);
  }

  await writeJson(join(resultDirectory, 'proof.json'), proof);
  await writeJson(join(resultDirectory, 'parent-surface-intent-read-model.json'), readModel);
  await writeFile(join(outputDirectory, '00-source-snapshot.md'), sourceSnapshot(proof), 'utf8');
  await writeFile(
    join(outputDirectory, '01-social-alert-report-parent-surface-intent-proof.md'),
    markdownFor(proof),
    'utf8'
  );
  await writeFile(join(outputDirectory, '08-security-negative-proof.md'), securityProofFor(proof), 'utf8');
  await writeFile(join(outputDirectory, '10-validation-commands.log'), validationLogFor(proof), 'utf8');
  await writeFile(
    join(outputDirectory, 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nThis proof adds a parent-domain parent-surface intent read model only. It does not render portal notification UI.\n',
    'utf8'
  );

  console.log('social-alert-report-parent-surface-intent-proof-ok=true');
  console.log(`proof=${relativePath(join(resultDirectory, 'proof.json'))}`);
  console.log(
    `manifest=${relativePath(join(outputDirectory, '01-social-alert-report-parent-surface-intent-proof.md'))}`
  );
}

function providerStatusReadModel(providerPreflight, refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    handoffId: 'social-provider-status-handoff-parent-surface',
    generatedAt,
    sourceProviderPreflightId: 'social-provider-preflight-parent-surface',
    sourceContractRefs: ['social-alert-report-provider-preflight-proof'],
    providerStatusBoundaryReadModelRef: 'v0-8-notification-provider-status-boundary',
    providerStatusBoundaryCoverageRefs: [
      'notification-provider-queued-contract',
      'notification-provider-delivered-receipt-required',
      'notification-provider-failed-contract',
      'notification-provider-unavailable-contract',
      'notification-provider-manual-required-contract',
    ],
    rows: [
      providerStatusRow(
        providerPreflight,
        refs,
        'high-risk',
        providerPreflight.SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired
      ),
      providerStatusRow(
        providerPreflight,
        refs,
        'manual-required',
        providerPreflight.SocialAlertReportProviderPreflightStatus.ManualRequired
      ),
      providerStatusRow(
        providerPreflight,
        refs,
        'unavailable',
        providerPreflight.SocialAlertReportProviderPreflightStatus.Unavailable
      ),
    ],
    providerStatusManualRequiredCount: 2,
    providerStatusUnavailableCount: 1,
    handoffNonClaims: [
      'no-provider-delivery-execution',
      'no-provider-receipt-ingestion',
      'no-provider-credentials',
      'no-cloud-routing',
      'no-parent-notification-ui-delivery',
      'no-report-delivery-execution',
      'no-final-policy-execution',
      'no-connector-native-runtime',
      'no-enforcement',
    ],
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  };
}

function providerStatusRow(providerPreflight, refs, label, status) {
  const unavailable = status === providerPreflight.SocialAlertReportProviderPreflightStatus.Unavailable;
  return {
    handoffRowId: `social-provider-status-handoff-${label}`,
    sourcePreflightRowId: `social-provider-preflight-${label}`,
    sourceIntentRef: `social-alert-report-intent-${label}`,
    sourcePreflightStatus: status,
    sourceLocalOutboxRecordRef: unavailable ? null : `local-outbox-social-parent-surface-${label}`,
    sourceProviderChannelRef: unavailable ? null : 'social-provider-channel-in-app',
    providerStatusBoundaryEntry: {
      schemaVersion: refs.ParentContractSchemaVersion.V0_6,
      statusEntryId: `social-provider-status-${label}`,
      providerStatus: unavailable ? 'unavailable' : 'manual-required',
      statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
      quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
      escalationReadiness: unavailable ? 'unavailable' : 'manual-required',
      deliveryClaimState: unavailable ? 'not-implemented' : 'not-observed',
      notificationIntentRef: `social-provider-status-intent-${label}`,
      notificationStatusRef: `social-provider-status-ref-${label}`,
      providerAttemptRef: `social-provider-attempt-${label}`,
      auditRefs: [`audit-social-parent-surface-${label}`],
      preferenceRefs: [`social-provider-preference-${label}`],
      readinessRefs: unavailable
        ? ['social-provider-readiness-unavailable']
        : [`provider-adapter-required-${label}`, `provider-credentials-required-${label}`],
      providerReceiptRefs: [],
      manualProofRequirements: [`manual-proof-social-parent-surface-${label}`],
      minimalPayloadBoundary: 'Provider status remains a manual or unavailable setup row without delivery.',
      providerDeliveryImplemented: false,
      providerDeliveryObserved: false,
      deliveredNotificationClaimed: false,
      sensitiveProviderPayloadClaimed: false,
      providerStoresChildEvidenceClaimed: false,
      lastCheckedAt: generatedAt,
    },
    manualProofRequirements: [`manual-proof-social-parent-surface-${label}`],
  };
}

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function checkFile(path) {
  return { label: `${path} exists`, pass: existsSync(join(repoRoot, path)) };
}

function checkIncludes(value, expected, label) {
  return { label, pass: value.includes(expected) };
}

function sourceSnapshot(proof) {
  return [
    '# Source Snapshot',
    '',
    `Branch: ${proof.branch}`,
    `Commit: ${proof.commit}`,
    '',
    'Inspected social alert/report provider status handoff contracts and the app/game parent-surface intent pattern before adding a social-specific parent-surface intent proof.',
  ].join('\n');
}

function markdownFor(proof) {
  return [
    '# Social Alert/Report Parent Surface Intent Proof',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Rows: ${proof.summary.rowCount}`,
    `Manual action required: ${proof.summary.manualActionRequiredCount}`,
    `Unavailable visible: ${proof.summary.unavailableVisibleCount}`,
    '',
    'This proof projects social alert/report provider-status handoff rows into parent-visible manual/unavailable surface intent rows.',
    'It carries notification status refs, readiness refs, audit refs, and manual proof requirements for future authenticated drill-in.',
    'It does not render parent notification UI and does not claim provider delivery, receipts, report delivery execution, final policy execution, connector/native runtime, enforcement, or product completion.',
  ].join('\n');
}

function securityProofFor(proof) {
  return [
    '# Security Negative Proof',
    '',
    `Parent notification UI rendered: ${proof.summary.parentNotificationUiRendered}`,
    `Provider delivery runtime claimed: ${proof.summary.providerDeliveryRuntimeClaimed}`,
    `Provider receipt ingestion claimed: ${proof.summary.providerReceiptIngestionClaimed}`,
    `Report delivery execution claimed: ${proof.summary.reportDeliveryExecutionClaimed}`,
    `Final policy execution claimed: ${proof.summary.finalPolicyExecutionClaimed}`,
    `Enforcement claimed: ${proof.summary.enforcementClaimed}`,
  ].join('\n');
}

function validationLogFor(proof) {
  return proof.commands.map((entry) => `$ ${entry.command}\nexit=${entry.exitCode}`).join('\n\n');
}

async function readText(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
    child.on('close', (exitCode) => {
      commands.push({ command: [command, ...args].join(' '), exitCode });
      exitCode === 0 ? resolve() : reject(new Error(`Command failed: ${command} ${args.join(' ')}`));
    });
  });
}

function gitOutput(args) {
  return runQuiet('git', args);
}

function runQuiet(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], shell: false });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => {
      stdout += chunk;
    });
    child.stderr.on('data', (chunk) => {
      stderr += chunk;
    });
    child.on('close', (exitCode) => {
      exitCode === 0 ? resolve(stdout.trim()) : reject(new Error(stderr.trim()));
    });
  });
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
