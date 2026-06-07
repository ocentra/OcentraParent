import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const gateRoot = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const namedProofRoot = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  'tracking-child-runtime-delivery-boundary-proof'
);
const testResultRoot = join(repoRoot, 'test-results', 'tracking-child-runtime-delivery-boundary-proof');
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
    'tests/tracking-child-runtime-delivery-boundary-proof.test.ts',
    'tests/tracking-child-check-in-timeout-escalation-proof.test.ts',
  ]);

  const policy = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-location-policy.js'))
  );
  const timeoutProof = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-child-check-in-timeout-escalation-proof.js')
    )
  );
  const boundaryProof = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-child-runtime-delivery-boundary-proof.js')
    )
  );
  const checkedAt = new Date().toISOString();
  const commit = await gitHead();
  const sourceTimeoutReadModel = timeoutProof.buildTrackingChildCheckInTimeoutReadModel(
    {
      generatedAt: '2026-06-07T14:05:00.000Z',
      readinessId: 'tracking-child-check-in-timeout-escalation-proof',
      sourceContractRefs: [
        'packages/parent-domain/src/tracking-location-policy.ts',
        'packages/parent-domain/src/tracking-child-check-in-timeout-escalation-proof.ts',
        'docs/plans/tracking-plan/workpacks/18-child-check-in-flow.md',
      ],
    },
    trackingPolicyReadModelFixture(policy)
  );
  const readModel = boundaryProof.buildTrackingChildRuntimeDeliveryBoundaryReadModel(
    {
      generatedAt: '2026-06-07T14:10:00.000Z',
      readinessId: 'tracking-child-runtime-delivery-boundary-proof',
      sourceContractRefs: [
        'packages/parent-domain/src/tracking-child-check-in-timeout-escalation-proof.ts',
        'packages/parent-domain/src/tracking-child-runtime-delivery-boundary-proof.ts',
        'apps/portal/src/tracking-child-check-in-proof.ts',
        'apps/portal/e2e/tracking-hosted-ui-proof.spec.ts',
        'docs/plans/tracking-plan/workpacks/30-parent-and-child-ui-ux-surfaces.md',
      ],
      hostedUiProofRefs: [
        'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/19-child-runtime-ui-proof.json',
        'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-runtime-ui.png',
        'test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json',
      ],
    },
    sourceTimeoutReadModel
  );

  assert.equal(readModel.rows.length, 5, 'expected five child runtime boundary rows');
  assert.equal(readModel.hostedCopyOnlyCount, 5, 'hosted-copy rows');
  assert.equal(readModel.safeResponseDisclosureCount, 1, 'safe response disclosure rows');
  assert.equal(readModel.escalationDisclosureCount, 3, 'escalation disclosure rows');
  assert.equal(readModel.requiredRuntimeProofRefCount, 25, 'runtime proof refs');
  assert.equal(readModel.childDeviceDeliveryRuntimeClaimed, false, 'no child delivery runtime');
  assert.equal(readModel.childDeviceExecutionRuntimeClaimed, false, 'no child execution runtime');
  assert.equal(readModel.renderedChildDeviceUiRuntimeClaimed, false, 'no rendered child runtime UI');
  assert.equal(readModel.physicalDeviceProofClaimed, false, 'no physical-device proof');
  assert.equal(readModel.authorityProofClaimed, false, 'no authority proof');
  assert.equal(readModel.productReadyClaimed, false, 'not product ready');

  await writeProofArtifacts({ checkedAt, commit, readModel });

  console.log('tracking-child-runtime-delivery-boundary-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testResultRoot, 'proof.json'))}`);
}

async function writeProofArtifacts({ checkedAt, commit, readModel }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(gateRoot, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId: '30-parent-and-child-ui-ux-surfaces',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    status: 'proved',
    artifactPath: relative(repoRoot, join(proofRoot, '26-child-runtime-delivery-boundary-proof.json')),
    rolloutGateArtifactPath: relative(repoRoot, join(gateRoot, '38-child-runtime-delivery-boundary-proof.json')),
    sourceSnapshotPath: relative(repoRoot, join(namedProofRoot, '00-source-snapshot.md')),
    validationLogPath: relative(repoRoot, join(namedProofRoot, '16-validation-commands.log')),
    readModel,
    proofLabels: [
      'tracking-child-runtime.hosted-copy-only',
      'tracking-child-runtime.safe-response-disclosure',
      'tracking-child-runtime.help-response-disclosure',
      'tracking-child-runtime.timeout-disclosure',
      'tracking-child-runtime.required-runtime-proof-refs',
      'tracking-child-runtime.no-child-device-delivery-runtime',
      'tracking-child-runtime.no-child-device-execution-runtime',
      'tracking-child-runtime.no-physical-device-or-authority-proof',
    ],
    productClaims: {
      hostedChildRuntimeBoundaryRowsClaimed: true,
      hostedChildRuntimeUiProofRefsClaimed: true,
      childCheckInTimeoutRowsClaimed: true,
      requiredRuntimeProofRefsClaimed: true,
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptIngestionClaimed: false,
      liveLocationRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productReadyClaimed: false,
    },
    missingProofReason:
      'P2 hosted boundary proof links hosted child-runtime UI proof refs to child check-in timeout rows and records required runtime proof refs for each row. It proves hosted copy/disclosure coverage only. Actual child-device delivery, child-device execution, rendered child-device runtime UI, provider delivery, notification receipt ingestion, live location runtime, physical-device proof, authority proof, production workers, and product-ready behavior remain unclaimed.',
    commands,
  };

  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot({ checkedAt, commit }));
  await writeFile(
    join(proofRoot, '26-child-runtime-delivery-boundary-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(
    join(gateRoot, '38-child-runtime-delivery-boundary-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(join(namedProofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(
    join(namedProofRoot, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
  );
  await writeFile(join(namedProofRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(
    join(testResultRoot, 'tracking-child-runtime-delivery-boundary-read-model.json'),
    `${JSON.stringify(readModel, null, 2)}\n`
  );
}

function sourceSnapshot({ checkedAt, commit }) {
  return [
    '# Tracking Child Runtime Delivery Boundary Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- requiredProofTier: P2_HOSTED_CI',
    '- currentProofTier: P2_HOSTED_CI',
    '- status: proved',
    '- proves hosted child-runtime disclosure rows linked to child check-in timeout rows and hosted UI proof refs',
    '- proof module: packages/parent-domain/src/tracking-child-runtime-delivery-boundary-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-child-runtime-delivery-boundary-proof.test.ts',
    '- proof harness: scripts/test/tracking-child-runtime-delivery-boundary-proof.mjs',
    '- hosted UI source: apps/portal/src/tracking-child-check-in-proof.ts',
    '- hosted UI proof: scripts/test/tracking-plan-hosted-ui-proof.mjs',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=30-parent-and-child-ui-ux-surfaces',
    'Child runtime delivery boundary rows preserve hosted UI proof refs, source evidence refs, source audit refs, and required runtime proof refs.',
    'Rows are hosted-copy disclosure rows only and do not claim actual child-device delivery or child-device execution.',
    'Rendered child-device runtime UI, provider delivery, notification receipt ingestion, live location runtime, physical-device proof, authority proof, production workers, and product-ready behavior are explicit non-claims.',
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
    checkInRequest(policy, 'tracking-check-in-waiting', 'tracking-alert-waiting', 'sent', '2026-06-07T14:30:00.000Z'),
    checkInRequest(policy, 'tracking-check-in-safe', 'tracking-alert-safe', 'sent', '2026-06-07T14:30:00.000Z'),
    checkInRequest(policy, 'tracking-check-in-help', 'tracking-alert-help', 'sent', '2026-06-07T14:30:00.000Z'),
    checkInRequest(
      policy,
      'tracking-check-in-call-parent',
      'tracking-alert-call-parent',
      'sent',
      '2026-06-07T14:30:00.000Z'
    ),
    checkInRequest(policy, 'tracking-check-in-expired', 'tracking-alert-expired', 'sent', '2026-06-07T14:05:00.000Z'),
  ];

  return policy.TrackingLocationPolicyReadModelSchema.parse({
    schemaVersion: policy.TrackingPolicySchemaVersion,
    generatedAt: '2026-06-07T14:05:00.000Z',
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
    reasonCodes: ['tracking-child-runtime-policy-rule'],
    auditRefs: [`tracking-rule-audit-${ruleId}`],
  };
}

function decision(policy, decisionId, alertId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    decisionId,
    decidedAt: '2026-06-07T14:01:00.000Z',
    ruleId: decisionId,
    action: 'ask-child-check-in',
    dryRun: false,
    evidenceReferences: [evidenceTrace()],
    aiAnalysisId: null,
    alertIntentId: alertId,
    reasonCodes: ['tracking-child-runtime-decision'],
    auditRefs: [`tracking-decision-audit-${alertId}`],
  };
}

function alert(policy, alertId, severity, policyDecisionId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    alertId,
    createdAt: '2026-06-07T14:01:30.000Z',
    severity,
    policyDecisionId,
    evidenceReferences: [evidenceTrace()],
    sensitiveDetailMode: 'authenticated-drill-in-only',
    notificationStatusRefs: [`notification-status-${alertId}`],
    acknowledgementId: null,
    reasonCodes: ['tracking-child-runtime-alert'],
  };
}

function checkInRequest(policy, checkInId, relatedAlertId, state, expiresAt) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    checkInId,
    requestedAt: '2026-06-07T14:02:00.000Z',
    state,
    relatedAlertId,
    includeLocationIfPermitted: true,
    expiresAt,
    evidenceReferences: [evidenceTrace()],
    auditRefs: [`tracking-child-runtime-request-${checkInId}`],
  };
}

function checkInResponse(policy, checkInId, response, locationEvidenceReferenceId) {
  return {
    schemaVersion: policy.TrackingPolicySchemaVersion,
    checkInId,
    respondedAt: '2026-06-07T14:04:00.000Z',
    response,
    locationEvidenceReference:
      locationEvidenceReferenceId === null
        ? null
        : {
            evidenceReferenceId: locationEvidenceReferenceId,
            kind: 'journal-event',
            observedAt: '2026-06-07T14:03:30.000Z',
          },
    auditRefs: [`tracking-child-runtime-response-${checkInId}`],
  };
}

function evidenceTrace() {
  return {
    evidenceReferenceId: 'tracking-child-runtime-evidence-1',
    kind: 'journal-event',
    observedAt: '2026-06-07T14:00:00.000Z',
  };
}

async function runNpm(args) {
  const command = `npm ${args.join(' ')}`;
  commands.push({ command });
  await run('cmd', ['/c', 'npm', ...args]);
}

async function run(command, args) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, shell: false, stdio: 'inherit' });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited ${code}`));
    });
  });
}

async function gitHead() {
  let stdout = '';
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, shell: false });
    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });
    child.stderr.pipe(process.stderr);
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`git rev-parse HEAD exited ${code}`));
    });
  });
  return stdout.trim();
}
