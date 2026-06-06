import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '18-child-check-in-flow');
const gateRoot = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const testResultRoot = join(repoRoot, 'test-results', 'tracking-child-check-in-timeout-escalation-proof');
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
    'tests/tracking-child-check-in-timeout-escalation-proof.test.ts',
    'tests/tracking-location-policy.test.ts',
  ]);

  const policy = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-location-policy.js'))
  );
  const timeoutProof = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-child-check-in-timeout-escalation-proof.js')
    )
  );
  const checkedAt = new Date().toISOString();
  const commit = await gitHead();
  const readModel = timeoutProof.buildTrackingChildCheckInTimeoutReadModel(
    {
      generatedAt: '2026-06-06T18:10:00.000Z',
      readinessId: 'tracking-child-check-in-timeout-escalation-proof',
      sourceContractRefs: [
        'packages/parent-domain/src/tracking-location-policy.ts',
        'packages/parent-domain/src/tracking-location-policy-runtime.ts',
        'docs/plans/tracking-plan/workpacks/18-child-check-in-flow.md',
        'docs/expectations/notifications.md',
        'docs/expectations/location-geofence.md',
      ],
    },
    trackingPolicyReadModelFixture(policy)
  );

  assert.equal(readModel.rows.length, 5, 'expected five child check-in timeout rows');
  assert.equal(readModel.waitingCount, 1, 'waiting rows');
  assert.equal(readModel.resolvedCount, 1, 'resolved rows');
  assert.equal(readModel.escalationReadyCount, 3, 'escalation-ready rows');
  assert.equal(readModel.childDeviceDeliveryRuntimeClaimed, false, 'no child delivery runtime');
  assert.equal(readModel.renderedChildDeviceUiClaimed, false, 'no rendered child UI');
  assert.equal(readModel.providerDeliveryClaimed, false, 'no provider delivery');
  assert.equal(readModel.physicalDeviceProofClaimed, false, 'no physical device proof');
  assert.equal(readModel.productClaimReady, false, 'not product-ready');

  await writeProofArtifacts({ checkedAt, commit, readModel });

  console.log('tracking-child-check-in-timeout-escalation-proof-ok');
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
    workpackId: '18-child-check-in-flow',
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    status: 'proved',
    artifactPath: relative(repoRoot, join(proofRoot, '31-child-check-in-timeout-escalation-proof.json')),
    rolloutGateArtifactPath: relative(repoRoot, join(gateRoot, '31-child-check-in-timeout-escalation-proof.json')),
    sourceSnapshotPath: relative(repoRoot, join(proofRoot, '00-source-snapshot.md')),
    validationLogPath: relative(repoRoot, join(proofRoot, '16-validation-commands.log')),
    readModel,
    proofLabels: [
      'tracking-child-check-in.waiting-row',
      'tracking-child-check-in.safe-response-recorded',
      'tracking-child-check-in.help-response-escalation-ready',
      'tracking-child-check-in.call-parent-response-escalation-ready',
      'tracking-child-check-in.expired-timeout-escalation-ready',
      'tracking-child-check-in.no-child-runtime-or-device-claim',
    ],
    productClaims: {
      childCheckInTimeoutRowsClaimed: true,
      childCheckInResolutionContractClaimed: true,
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceResponseRuntimeClaimed: false,
      renderedChildDeviceUiClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptRuntimeClaimed: false,
      liveLocationSampleRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionTimeoutWorkerClaimed: false,
      adapterDispatchClaimed: false,
      productClaimReady: false,
    },
    missingProofReason:
      'P1 fixture proof covers child check-in timeout and escalation readiness rows over existing tracking check-in contracts and runtime resolver. Child-device delivery/runtime execution, rendered child-device UI, provider delivery, receipt runtime, live location sample runtime, physical-device proof, authority proof, production timeout workers, and adapter dispatch remain unclaimed.',
    commands,
  };

  await writeFile(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot({ checkedAt, commit }));
  await writeFile(
    join(proofRoot, '31-child-check-in-timeout-escalation-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(
    join(gateRoot, '31-child-check-in-timeout-escalation-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(join(proofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(
    join(proofRoot, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
  );
  await writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(
    join(testResultRoot, 'tracking-child-check-in-timeout-read-model.json'),
    `${JSON.stringify(readModel, null, 2)}\n`
  );
}

function sourceSnapshot({ checkedAt, commit }) {
  return [
    '# 18-child-check-in-flow Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- requiredProofTier: P1_FIXTURE_SIMULATION',
    '- currentProofTier: P1_FIXTURE_SIMULATION',
    '- status: proved',
    '- proof module: packages/parent-domain/src/tracking-child-check-in-timeout-escalation-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-child-check-in-timeout-escalation-proof.test.ts',
    '- proof harness: scripts/test/tracking-child-check-in-timeout-escalation-proof.mjs',
    '- source contracts: packages/parent-domain/src/tracking-location-policy.ts',
    '- runtime resolver: packages/parent-domain/src/tracking-location-policy-runtime.ts',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=18-child-check-in-flow',
    'Child check-in rows preserve request, response, alert, evidence, policy, audit, parent action, and timeout refs.',
    'Escalation-ready rows are derived only from help/call-parent/no-response or expired timeout states.',
    'Child-device delivery/runtime execution, rendered child UI, provider delivery, receipt runtime, live location sample runtime, physical-device proof, authority proof, production timeout workers, and adapter dispatch are all non-claims.',
    '',
  ].join('\n');
}

function trackingPolicyReadModelFixture(policy) {
  const alerts = [
    alert(policy, 'tracking-alert-waiting', 'watch', 'tracking-decision-waiting'),
    alert(policy, 'tracking-alert-safe', 'warning', 'tracking-decision-safe'),
    alert(policy, 'tracking-alert-help', 'urgent', 'tracking-decision-help'),
    alert(policy, 'tracking-alert-call-parent', 'urgent', 'tracking-decision-call-parent'),
    alert(policy, 'tracking-alert-expired', 'critical', 'tracking-decision-expired'),
  ];
  const requests = [
    checkInRequest(policy, 'tracking-check-in-waiting', 'tracking-alert-waiting', 'sent', '2026-06-06T18:30:00.000Z'),
    checkInRequest(policy, 'tracking-check-in-safe', 'tracking-alert-safe', 'sent', '2026-06-06T18:30:00.000Z'),
    checkInRequest(policy, 'tracking-check-in-help', 'tracking-alert-help', 'sent', '2026-06-06T18:30:00.000Z'),
    checkInRequest(
      policy,
      'tracking-check-in-call-parent',
      'tracking-alert-call-parent',
      'sent',
      '2026-06-06T18:30:00.000Z'
    ),
    checkInRequest(policy, 'tracking-check-in-expired', 'tracking-alert-expired', 'sent', '2026-06-06T18:05:00.000Z'),
  ];

  return policy.TrackingLocationPolicyReadModelSchema.parse({
    schemaVersion: policy.TrackingPolicySchemaVersion,
    generatedAt: '2026-06-06T18:10:00.000Z',
    rules: alerts.map((entry) => rule(policy, entry.policyDecisionId)),
    decisions: alerts.map((entry) => decision(policy, entry.policyDecisionId, entry.alertId)),
    acknowledgements: [],
    checkInRequests: requests,
    checkInResponses: [
      checkInResponse(policy, 'tracking-check-in-safe', 'safe', 'tracking-child-check-in-safe-location'),
      checkInResponse(policy, 'tracking-check-in-help', 'help', 'tracking-child-check-in-help-location'),
      checkInResponse(policy, 'tracking-check-in-call-parent', 'call-parent', null),
    ],
    aiRoutes: [],
    aiResults: [],
    alerts,
    escalations: [],
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
    action: 'ask-child-check-in',
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: true,
    reasonCodes: ['tracking-child-check-in-policy-rule'],
    auditRefs: [`tracking-rule-audit-${ruleId}`],
  };
}

function decision(policy, decisionId, alertId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-06T18:01:00.000Z',
    ruleId: decisionId,
    action: 'ask-child-check-in',
    dryRun: false,
    evidenceReferences: [evidenceTrace()],
    aiAnalysisId: null,
    alertIntentId: alertId,
    reasonCodes: ['tracking-child-check-in-decision'],
    auditRefs: [`tracking-decision-audit-${alertId}`],
  };
}

function alert(policy, alertId, severity, policyDecisionId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-06T18:01:30.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [evidenceTrace()],
    sensitiveDetailMode: 'authenticated-drill-in-only',
    notificationStatusRefs: [`notification-status-${alertId}`],
    acknowledgementId: null,
    reasonCodes: ['tracking-child-check-in-alert'],
  };
}

function checkInRequest(policy, checkInId, relatedAlertId, state, expiresAt) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    checkInId,
    requestedAt: '2026-06-06T18:02:00.000Z',
    state,
    relatedAlertId,
    includeLocationIfPermitted: true,
    expiresAt,
    evidenceReferences: [evidenceTrace()],
    auditRefs: [`tracking-child-check-in-request-${checkInId}`],
  };
}

function checkInResponse(policy, checkInId, response, locationEvidenceReferenceId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    checkInId,
    respondedAt: '2026-06-06T18:04:00.000Z',
    response,
    locationEvidenceReference:
      locationEvidenceReferenceId === null
        ? null
        : {
            evidenceReferenceId: locationEvidenceReferenceId,
            kind: 'journal-event',
            observedAt: '2026-06-06T18:03:30.000Z',
          },
    auditRefs: [`tracking-child-check-in-response-${checkInId}`],
  };
}

function evidenceTrace() {
  return {
    evidenceReferenceId: 'tracking-child-check-in-evidence-1',
    kind: 'journal-event',
    observedAt: '2026-06-06T18:00:00.000Z',
  };
}

async function gitHead() {
  const result = await runCommand('git', ['rev-parse', '--short=8', 'HEAD']);
  return result.stdout.trim();
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    return runCommand('cmd', ['/c', 'npm', ...args]);
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
