import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '17-parent-acknowledgement-and-exception-model');
const gateRoot = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const testResultRoot = join(repoRoot, 'test-results', 'tracking-parent-acknowledgement-action-readiness-proof');
const commands = [];

await main();

async function main() {
  await runNpm(['--workspace', '@ocentra-parent/schema-domain', 'run', 'build']);
  await runNpm(['--workspace', '@ocentra-parent/tracking-domain', 'run', 'build']);
  await runNpm([
    'exec',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'vitest',
    'run',
    'tests/contract/tracking-parent-acknowledgement-action-readiness-proof.test.ts',
    'tests/contract/tracking-location-policy.test.ts',
  ]);

  const policy = await import(
    pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'tracking-location-policy.js'))
  );
  const actionProof = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'schema-domain', 'dist', 'tracking-parent-acknowledgement-action-readiness-proof.js')
    )
  );
  const checkedAt = new Date().toISOString();
  const commit = await gitHead();
  const readModel = actionProof.buildTrackingParentAcknowledgementActionReadModel(
    {
      generatedAt: '2026-06-06T17:45:00.000Z',
      readinessId: 'tracking-parent-acknowledgement-action-readiness-proof',
      sourceContractRefs: [
        'packages/schema-domain/src/tracking-parent-acknowledgement-action-readiness-proof.ts',
        'packages/schema-domain/src/tracking-location-policy.ts',
        'docs/plans/tracking-plan/workpacks/17-parent-acknowledgement-and-exception-model.md',
        'docs/expectations/notifications.md',
        'docs/expectations/policy.md',
      ],
    },
    trackingPolicyReadModelFixture(policy)
  );

  assert.equal(readModel.rows.length, 5, 'expected five parent action readiness rows');
  assert.equal(readModel.actionReadyCount, 2, 'action ready rows');
  assert.equal(readModel.recordedCount, 3, 'recorded parent action rows');
  assert.equal(readModel.renderedPortalAcknowledgementUiClaimed, false, 'no rendered portal acknowledgement UI');
  assert.equal(readModel.liveServiceMutationClaimed, false, 'no live service mutation');
  assert.equal(readModel.providerDeliveryClaimed, false, 'no provider delivery');
  assert.equal(readModel.childDeviceRuntimeClaimed, false, 'no child runtime');
  assert.equal(readModel.physicalDeviceProofClaimed, false, 'no physical device proof');
  assert.equal(readModel.productClaimReady, false, 'not product-ready');

  await writeProofArtifacts({ checkedAt, commit, readModel });

  console.log('tracking-parent-acknowledgement-action-readiness-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testResultRoot, 'proof.json'))}`);
}

async function writeProofArtifacts({ checkedAt, commit, readModel }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(gateRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId: '17-parent-acknowledgement-and-exception-model',
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    status: 'proved',
    artifactPath: relative(repoRoot, join(proofRoot, '30-parent-acknowledgement-action-readiness-proof.json')),
    rolloutGateArtifactPath: relative(
      repoRoot,
      join(gateRoot, '30-parent-acknowledgement-action-readiness-proof.json')
    ),
    sourceSnapshotPath: relative(repoRoot, join(proofRoot, '00-source-snapshot.md')),
    validationLogPath: relative(repoRoot, join(proofRoot, '16-validation-commands.log')),
    readModel,
    proofLabels: [
      'tracking-parent-action.acknowledge-safe-ready',
      'tracking-parent-action.exception-active',
      'tracking-parent-action.false-alarm-recorded',
      'tracking-parent-action.child-check-in-request-ready',
      'tracking-parent-action.escalation-manual-review-ready',
      'tracking-parent-action.no-rendered-ui-or-runtime-claim',
    ],
    productClaims: {
      parentAcknowledgementActionRowsClaimed: true,
      exceptionExpiryRefsClaimed: true,
      criticalStillAlertInvariantClaimed: true,
      renderedPortalAcknowledgementUiClaimed: false,
      liveServiceMutationClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptRuntimeClaimed: false,
      childDeviceRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      adapterDispatchClaimed: false,
      productClaimReady: false,
    },
    missingProofReason:
      'P1 fixture proof covers parent action readiness rows over existing tracking alert, acknowledgement, exception, false-alarm, child check-in, and escalation contracts. Rendered portal acknowledgement UI, live service mutation, provider delivery, receipt runtime, child-device runtime, physical-device proof, authority proof, production workers, and adapter dispatch remain unclaimed.',
    commands,
  };

  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot({ checkedAt, commit }));
  await writeFile(
    join(proofRoot, '30-parent-acknowledgement-action-readiness-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(
    join(gateRoot, '30-parent-acknowledgement-action-readiness-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(join(proofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(
    join(proofRoot, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
  );
  await writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(
    join(testResultRoot, 'tracking-parent-acknowledgement-action-read-model.json'),
    `${JSON.stringify(readModel, null, 2)}\n`
  );
}

function sourceSnapshot({ checkedAt, commit }) {
  return [
    '# 17-parent-acknowledgement-and-exception-model Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- requiredProofTier: P1_FIXTURE_SIMULATION',
    '- currentProofTier: P1_FIXTURE_SIMULATION',
    '- status: proved',
    '- proof module: packages/schema-domain/src/tracking-parent-acknowledgement-action-readiness-proof.ts',
    '- proof tests: packages/tracking-domain/tests/contract/tracking-parent-acknowledgement-action-readiness-proof.test.ts',
    '- proof harness: scripts/test/tracking-parent-acknowledgement-action-readiness-proof.mjs',
    '- source contracts: packages/schema-domain/src/tracking-location-policy.ts',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=17-parent-acknowledgement-and-exception-model',
    'Parent acknowledgement rows preserve evidence, policy, alert, acknowledgement, escalation, audit, and UI surface refs.',
    'Critical rows keep stillAlertForCritical true instead of suppressing critical alerts generically.',
    'Rendered portal acknowledgement UI, live service mutation, provider delivery, receipt runtime, child-device runtime, physical-device proof, authority proof, production workers, and adapter dispatch are all non-claims.',
    '',
  ].join('\n');
}

function trackingPolicyReadModelFixture(policy) {
  const alerts = [
    alert(policy, 'tracking-alert-safe', 'warning', 'tracking-decision-safe', 'tracking-ack-safe'),
    alert(policy, 'tracking-alert-expected', 'warning', 'tracking-decision-expected', 'tracking-ack-expected'),
    alert(policy, 'tracking-alert-false-alarm', 'warning', 'tracking-decision-false-alarm', 'tracking-ack-false-alarm'),
    alert(policy, 'tracking-alert-check-in', 'watch', 'tracking-decision-check-in', null),
    alert(policy, 'tracking-alert-critical-review', 'critical', 'tracking-decision-critical-review', null),
  ];

  return policy.TrackingLocationPolicyReadModelSchema.parse({
    schemaVersion: policy.TrackingPolicySchemaVersion,
    generatedAt: '2026-06-06T17:45:00.000Z',
    rules: alerts.map((entry) => rule(policy, entry.policyDecisionId, entry.alertId)),
    decisions: [
      decision(policy, 'tracking-decision-safe', 'tracking-alert-safe', 'request-parent-acknowledgement'),
      decision(policy, 'tracking-decision-expected', 'tracking-alert-expected', 'request-parent-acknowledgement'),
      decision(policy, 'tracking-decision-false-alarm', 'tracking-alert-false-alarm', 'request-parent-acknowledgement'),
      decision(policy, 'tracking-decision-check-in', 'tracking-alert-check-in', 'ask-child-check-in'),
      decision(policy, 'tracking-decision-critical-review', 'tracking-alert-critical-review', 'escalate'),
    ],
    acknowledgements: [
      acknowledgement(policy, 'tracking-ack-safe', 'tracking-alert-safe', 'acknowledged-safe', null),
      acknowledgement(
        policy,
        'tracking-ack-expected',
        'tracking-alert-expected',
        'expected',
        '2026-06-06T20:00:00.000Z'
      ),
      acknowledgement(policy, 'tracking-ack-false-alarm', 'tracking-alert-false-alarm', 'false-alarm', null),
    ],
    checkInRequests: [],
    checkInResponses: [],
    aiRoutes: [],
    aiResults: [],
    alerts,
    escalations: [escalation(policy, 'tracking-escalation-critical-review', 'tracking-alert-critical-review')],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  });
}

function rule(policy, ruleId, alertId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    ruleId,
    familyId: 'family-1',
    childProfileId: 'child-1',
    deviceId: 'parent-device-1',
    policyVersion: 'tracking-policy-v1',
    targetKind: 'geofence-transition',
    action: alertId === 'tracking-alert-check-in' ? 'ask-child-check-in' : 'request-parent-acknowledgement',
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: true,
    reasonCodes: ['tracking-parent-action-policy-rule'],
    auditRefs: [`tracking-rule-audit-${ruleId}`],
  };
}

function decision(policy, decisionId, alertId, action) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-06T17:41:00.000Z',
    ruleId: decisionId,
    action,
    dryRun: false,
    evidenceReferences: [evidenceTrace()],
    aiAnalysisId: null,
    alertIntentId: alertId,
    reasonCodes: ['tracking-parent-action-decision'],
    auditRefs: [`tracking-decision-audit-${alertId}`],
  };
}

function alert(policy, alertId, severity, policyDecisionId, acknowledgementId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-06T17:41:30.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [evidenceTrace()],
    sensitiveDetailMode: 'authenticated-drill-in-only',
    notificationStatusRefs: [`notification-status-${alertId}`],
    acknowledgementId,
    reasonCodes: ['tracking-parent-action-alert'],
  };
}

function acknowledgement(policy, acknowledgementId, alertId, state, expiresAt) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    acknowledgementId,
    alertId,
    state,
    acknowledgedAt: '2026-06-06T17:42:00.000Z',
    expiresAt,
    stillAlertForCritical: true,
    reasonCodes: ['tracking-parent-action-acknowledgement'],
    auditRefs: [`tracking-acknowledgement-audit-${alertId}`],
  };
}

function escalation(policy, escalationId, alertId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    escalationId,
    alertId,
    state: 'manual-required',
    startedAt: '2026-06-06T17:42:30.000Z',
    nextActionAt: '2026-06-06T18:00:00.000Z',
    steps: ['parent-manual-review', 'second-guardian-review'],
    auditRefs: [`tracking-escalation-audit-${alertId}`],
  };
}

function evidenceTrace() {
  return {
    evidenceReferenceId: 'tracking-parent-action-evidence-1',
    kind: 'journal-event',
    observedAt: '2026-06-06T17:40:00.000Z',
  };
}

async function gitHead() {
  const result = await runCommand('git', ['rev-parse', '--short=8', 'HEAD']);
  return result.stdout.trim();
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    return runNpmCommand(runCommand, args);
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
