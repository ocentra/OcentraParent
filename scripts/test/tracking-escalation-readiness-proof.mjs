import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '27-escalation-engine');
const testResultRoot = join(repoRoot, 'test-results', 'tracking-escalation-readiness-proof');
const commands = [];

await main();

async function main() {
  await runNpm(['--workspace', '@ocentra-parent/parent-domain', 'run', 'build']);
  await runNpm([
    'exec',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'vitest',
    'run',
    'tests/tracking-escalation-readiness-proof.test.ts',
    'tests/tracking-location-policy.test.ts',
  ]);

  const policy = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-location-policy.js'))
  );
  const escalationProof = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-escalation-readiness-proof.js'))
  );
  const checkedAt = new Date().toISOString();
  const commit = await gitHead();
  const readModel = escalationProof.buildTrackingEscalationReadinessReadModel(
    {
      generatedAt: '2026-06-05T12:10:00.000Z',
      readinessId: 'tracking-escalation-readiness-proof',
      sourceContractRefs: [
        'packages/parent-domain/src/tracking-location-policy.ts',
        'docs/plans/tracking-plan/workpacks/27-escalation-engine.md',
        'docs/expectations/notifications.md',
        'docs/expectations/policy.md',
      ],
    },
    trackingPolicyReadModelFixture(policy)
  );

  assert.equal(readModel.rows.length, 4, 'expected four escalation readiness rows');
  assert.equal(readModel.resolvedCount, 2, 'resolved rows');
  assert.equal(readModel.manualRequiredCount, 2, 'manual-required rows');
  assert.equal(readModel.emergencyServicesAutoContactClaimed, false, 'no emergency auto-contact');
  assert.equal(readModel.providerDeliveryClaimed, false, 'no provider delivery');
  assert.equal(readModel.physicalDeviceProofClaimed, false, 'no physical device proof');
  assert.equal(readModel.productClaimReady, false, 'not product-ready');

  await writeProofArtifacts({ checkedAt, commit, readModel });

  console.log('tracking-escalation-readiness-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testResultRoot, 'proof.json'))}`);
}

async function writeProofArtifacts({ checkedAt, commit, readModel }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId: '27-escalation-engine',
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    status: 'proved',
    artifactPath: relative(repoRoot, join(proofRoot, '09-policy-alert-proof.json')),
    sourceSnapshotPath: relative(repoRoot, join(proofRoot, '00-source-snapshot.md')),
    validationLogPath: relative(repoRoot, join(proofRoot, '16-validation-commands.log')),
    readModel,
    proofLabels: [
      'tracking-escalation.parent-acknowledgement-cancels-escalation',
      'tracking-escalation.child-check-in-cancels-escalation',
      'tracking-escalation.urgent-second-guardian-manual-required',
      'tracking-escalation.critical-multi-channel-manual-required',
      'tracking-escalation.ai-cannot-schedule-escalation-directly',
      'tracking-escalation.no-emergency-auto-contact',
    ],
    productClaims: {
      parentPolicyAuthorityClaimed: true,
      acknowledgementAwareClaimed: true,
      childCheckInResolutionClaimed: true,
      aiDirectEscalationClaimed: false,
      emergencyServicesAutoContactClaimed: false,
      providerDeliveryClaimed: false,
      providerReceiptIngestionClaimed: false,
      providerCredentialsClaimed: false,
      parentNotificationUiClaimed: false,
      childDeviceDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      productionEscalationWorkerClaimed: false,
      productionQuietHoursTimerClaimed: false,
      productClaimReady: false,
    },
    missingProofReason:
      'P1 fixture proof covers deterministic escalation readiness over existing tracking alerts, acknowledgement, and check-in contracts. Provider delivery, receipt ingestion, credentials, parent notification UI/history/preferences, child-device delivery, Android/iOS physical-device proof, production escalation workers, quiet-hours timers, durable storage, and emergency auto-contact remain unclaimed.',
    commands,
  };

  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot({ checkedAt, commit }));
  await writeFile(join(proofRoot, '09-policy-alert-proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(join(proofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(
    join(proofRoot, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
  );
  await writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(
    join(testResultRoot, 'tracking-escalation-readiness-read-model.json'),
    `${JSON.stringify(readModel, null, 2)}\n`
  );
}

function sourceSnapshot({ checkedAt, commit }) {
  return [
    '# 27-escalation-engine Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- requiredProofTier: P1_FIXTURE_SIMULATION',
    '- currentProofTier: P1_FIXTURE_SIMULATION',
    '- status: proved',
    '- proof module: packages/parent-domain/src/tracking-escalation-readiness-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-escalation-readiness-proof.test.ts',
    '- proof harness: scripts/test/tracking-escalation-readiness-proof.mjs',
    '- source contracts: packages/parent-domain/src/tracking-location-policy.ts',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=27-escalation-engine',
    'AI analysis refs are counted as evidence context only; aiScheduledEscalation is always false.',
    'Emergency services auto-contact is always false for MVP escalation readiness rows.',
    'Provider delivery, receipt ingestion, credentials, cloud routing, child delivery, physical-device proof, and production workers are all non-claims.',
    'Critical alerts become manual-required multi-channel review rows unless resolved by explicit child check-in proof.',
    '',
  ].join('\n');
}

function trackingPolicyReadModelFixture(policy) {
  const alerts = [
    alert(policy, 'tracking-alert-acknowledged', 'warning', 'tracking-decision-acknowledged', 'tracking-ack-1'),
    alert(policy, 'tracking-alert-safe-check-in', 'warning', 'tracking-decision-safe-check-in', null),
    alert(policy, 'tracking-alert-urgent-no-response', 'urgent', 'tracking-decision-urgent', null),
    alert(policy, 'tracking-alert-critical', 'critical', 'tracking-decision-critical', null),
  ];

  return policy.TrackingLocationPolicyReadModelSchema.parse({
    schemaVersion: policy.TrackingPolicySchemaVersion,
    generatedAt: '2026-06-05T12:10:00.000Z',
    rules: alerts.map((entry) => rule(policy, entry.policyDecisionId)),
    decisions: alerts.map((entry) => decision(policy, entry.policyDecisionId, entry.alertId)),
    acknowledgements: [
      {
        schemaVersion: policy.TrackingPolicySchemaVersion,
        acknowledgementId: 'tracking-ack-1',
        alertId: 'tracking-alert-acknowledged',
        state: 'acknowledged-safe',
        acknowledgedAt: '2026-06-05T12:02:00.000Z',
        expiresAt: null,
        stillAlertForCritical: true,
        reasonCodes: ['parent-confirmed-safe'],
        auditRefs: ['tracking-acknowledgement-audit-1'],
      },
    ],
    checkInRequests: [
      checkInRequest(
        policy,
        'tracking-check-in-safe',
        'tracking-alert-safe-check-in',
        'sent',
        '2026-06-05T12:20:00.000Z'
      ),
      checkInRequest(
        policy,
        'tracking-check-in-urgent',
        'tracking-alert-urgent-no-response',
        'sent',
        '2026-06-05T12:05:00.000Z'
      ),
    ],
    checkInResponses: [
      {
        schemaVersion: policy.TrackingPolicySchemaVersion,
        checkInId: 'tracking-check-in-safe',
        respondedAt: '2026-06-05T12:04:00.000Z',
        response: 'safe',
        locationEvidenceReference: evidenceTrace(),
        auditRefs: ['tracking-check-in-safe-response'],
      },
    ],
    aiRoutes: [],
    aiResults: [],
    alerts,
    escalations: [
      escalation(policy, 'tracking-escalation-acknowledged', 'tracking-alert-acknowledged', 'waiting-for-parent'),
      escalation(policy, 'tracking-escalation-safe-check-in', 'tracking-alert-safe-check-in', 'waiting-for-child'),
      escalation(policy, 'tracking-escalation-urgent', 'tracking-alert-urgent-no-response', 'waiting-for-child'),
      escalation(policy, 'tracking-escalation-critical', 'tracking-alert-critical', 'waiting-for-parent'),
    ],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  });
}

function rule(policy, ruleId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    ruleId,
    familyId: 'family-1',
    childProfileId: 'child-1',
    deviceId: 'parent-device-1',
    policyVersion: 'tracking-policy-v1',
    targetKind: 'geofence-transition',
    action: 'escalate',
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: true,
    reasonCodes: ['tracking-escalation-policy-rule'],
    auditRefs: [`tracking-rule-audit-${ruleId}`],
  };
}

function decision(policy, decisionId, alertId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-05T12:01:00.000Z',
    ruleId: decisionId,
    action: 'escalate',
    dryRun: false,
    evidenceReferences: [evidenceTrace()],
    aiAnalysisId: `tracking-ai-analysis-${alertId}`,
    alertIntentId: alertId,
    reasonCodes: ['tracking-parent-policy-authorized-escalation'],
    auditRefs: [`tracking-decision-audit-${alertId}`],
  };
}

function alert(policy, alertId, severity, policyDecisionId, acknowledgementId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-05T12:01:30.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [evidenceTrace()],
    sensitiveDetailMode: 'authenticated-drill-in-only',
    notificationStatusRefs: [`notification-status-${alertId}`],
    acknowledgementId,
    reasonCodes: ['tracking-alert-escalation-candidate'],
  };
}

function checkInRequest(policy, checkInId, relatedAlertId, state, expiresAt) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    checkInId,
    requestedAt: '2026-06-05T12:02:00.000Z',
    state,
    relatedAlertId,
    includeLocationIfPermitted: true,
    expiresAt,
    evidenceReferences: [evidenceTrace()],
    auditRefs: [`tracking-check-in-request-${checkInId}`],
  };
}

function escalation(policy, escalationId, alertId, state) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    escalationId,
    alertId,
    state,
    startedAt: '2026-06-05T12:01:30.000Z',
    nextActionAt: '2026-06-05T12:15:00.000Z',
    steps: ['wait-parent-acknowledgement', 'request-child-check-in', 'guardian-manual-review'],
    auditRefs: [`tracking-escalation-audit-${alertId}`],
  };
}

function evidenceTrace() {
  return {
    evidenceReferenceId: 'tracking-escalation-location-evidence-1',
    kind: 'journal-event',
    observedAt: '2026-06-05T12:00:00.000Z',
  };
}

async function gitHead() {
  const result = await runCommand('git', ['rev-parse', '--short=8', 'HEAD']);
  return result.stdout.trim();
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    return runCommand(...npmCommand([...args]));
  }
  return runCommand('npm', args);
}

function runCommand(command, args) {
  const rendered = [command, ...args].join(' ');
  commands.push({ command: rendered });
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: repoRoot,
      env: process.env,
      shell: false,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => {
      const text = chunk.toString();
      stdout += text;
      process.stdout.write(text);
    });
    child.stderr.on('data', (chunk) => {
      const text = chunk.toString();
      stderr += text;
      process.stderr.write(text);
    });
    child.on('error', reject);
    child.on('close', (code) => {
      if (code === 0) {
        resolve({ stdout, stderr });
        return;
      }
      reject(new Error(`${rendered} failed with exit ${code}\n${stderr}`));
    });
  });
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
