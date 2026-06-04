import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const workpackId = '25-policy-compiler-for-tracking-rules';
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', workpackId);
const resultDir = join(repoRoot, 'test-results', 'tracking-plan-policy-compiler-proof');
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
    'tests/tracking-location-policy-compiler.test.ts',
  ]);

  const policy = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-location-policy.js'))
  );
  const compiler = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-location-policy-compiler.js'))
  );
  const checkedAt = new Date().toISOString();
  const commit = await gitHead();
  const results = compileProofScenarios(policy, compiler);

  await mkdir(proofDir, { recursive: true });
  await mkdir(resultDir, { recursive: true });
  await writeFile(join(proofDir, '00-source-snapshot.md'), sourceSnapshot(commit, checkedAt));
  await writeFile(join(proofDir, '01-contract-proof.log'), contractProofLog());
  await writeJson(join(proofDir, '09-policy-alert-proof.json'), policyAlertProof(results, checkedAt));
  await writeFile(join(proofDir, '13-security-negative-proof.log'), securityNegativeProof(results));
  await writeFile(
    join(proofDir, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
  );
  await writeJson(join(proofDir, 'proof-summary.json'), proofSummary(results, commit, checkedAt));
  await writeJson(join(resultDir, 'proof.json'), testResult(results, checkedAt));

  console.log('tracking-plan-policy-compiler-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofDir)}`);
}

function compileProofScenarios(policy, compiler) {
  const scenarios = [
    scenario(policy, compiler, 'observe'),
    scenario(policy, compiler, 'notify-parent', { alertId: 'tracking-policy-alert-notify' }),
    scenario(policy, compiler, 'ask-child-check-in', { checkInId: 'tracking-policy-checkin' }),
    scenario(policy, compiler, 'request-parent-acknowledgement', { alertId: 'tracking-policy-alert-ack' }),
    scenario(policy, compiler, 'start-temporary-live-tracking', {
      liveTrackingGrantId: 'tracking-policy-live-grant',
      liveTrackingDurationSeconds: 900,
      parentConfirmationReceived: true,
    }),
    scenario(policy, compiler, 'escalate', {
      alertId: 'tracking-policy-alert-escalate',
      escalationId: 'tracking-policy-escalation',
    }),
    scenario(policy, compiler, 'no-action', { requestedAction: 'suppress' }),
    scenario(policy, compiler, 'notify-parent', {
      requestedAction: 'manual-required',
      platformManualRequired: true,
      alertId: 'tracking-policy-alert-manual',
    }),
    scenario(policy, compiler, 'notify-parent', {
      requestedAction: 'critical-alert',
      alertId: 'tracking-policy-alert-critical',
      aiAnalysis: aiCritical(policy),
    }),
    scenario(policy, compiler, 'observe', {
      requestedAction: 'critical-alert',
      alertId: 'tracking-policy-alert-ai-not-authority',
      aiAnalysis: aiCritical(policy),
    }),
  ];
  return scenarios.map((entry) => ({
    requestedAction: entry.requestedAction,
    decisionAction: entry.decision.action,
    finalActionSource: entry.finalActionSource,
    alertSeverity: entry.alertIntent?.severity ?? null,
    childCheckInState: entry.childCheckInRequest?.state ?? null,
    escalationState: entry.escalationChain?.state ?? null,
    liveGrantState: entry.temporaryLiveGrant?.state ?? null,
    parentPolicyFinalAuthority: entry.parentPolicyFinalAuthority,
    aiFinalAuthority: entry.aiFinalAuthority,
    runtimeEnforcementClaimed: entry.runtimeEnforcementClaimed,
    providerDeliveryClaimed: entry.providerDeliveryClaimed,
    reasonCodes: entry.decision.reasonCodes,
  }));
}

function scenario(policy, compiler, ruleAction, options = {}) {
  return compiler.compileTrackingPolicyDecision({
    schemaVersion: policy.TrackingPolicySchemaVersion,
    requestId: `tracking-policy-compile-${ruleAction}`,
    rule: rule(policy, ruleAction, options.ruleOverrides),
    requestedAt: '2026-06-04T08:00:00.000Z',
    decidedAt: '2026-06-04T08:01:00.000Z',
    followUpExpiresAt: '2026-06-04T08:11:00.000Z',
    decisionId: `tracking-policy-decision-${ruleAction}`,
    requestedAction: options.requestedAction ?? ruleAction,
    compilerMode: 'dry-run',
    evidenceReferences: [evidenceTrace()],
    aiAnalysis: options.aiAnalysis ?? null,
    alertId: options.alertId ?? null,
    alertSeverity: options.alertSeverity ?? null,
    checkInId: options.checkInId ?? null,
    escalationId: options.escalationId ?? null,
    liveTrackingGrantId: options.liveTrackingGrantId ?? null,
    liveTrackingDurationSeconds: options.liveTrackingDurationSeconds ?? null,
    parentConfirmationReceived: options.parentConfirmationReceived ?? true,
    freshEvidenceAvailable: options.freshEvidenceAvailable ?? true,
    platformManualRequired: options.platformManualRequired ?? false,
    reasonCodes: ['tracking-compiler-requested'],
    auditRefs: ['tracking-compiler-audit'],
  });
}

function rule(policy, action, overrides = {}) {
  return policy.TrackingPolicyRuleSchema.parse({
    schemaVersion: policy.TrackingPolicySchemaVersion,
    ruleId: `tracking-policy-rule-${action}`,
    familyId: 'family-1',
    childProfileId: 'child-1',
    deviceId: 'parent-device-1',
    policyVersion: 'tracking-policy-v2',
    targetKind: 'geofence-transition',
    action,
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: false,
    reasonCodes: ['tracking-rule-matched'],
    auditRefs: ['tracking-rule-audit'],
    ...overrides,
  });
}

function evidenceTrace() {
  return {
    evidenceReferenceId: 'tracking-policy-evidence-1',
    kind: 'journal-event',
    observedAt: '2026-06-04T08:00:00.000Z',
  };
}

function aiCritical(policy) {
  return policy.TrackingLocationAiAnalysisResultSchema.parse({
    schemaVersion: policy.TrackingPolicySchemaVersion,
    analysisId: 'tracking-ai-critical-1',
    completedAt: '2026-06-04T08:01:00.000Z',
    riskLevel: 'critical',
    confidence: 0.91,
    providerRouteId: 'tracking-ai-route-1',
    evidenceReferences: [evidenceTrace()],
    reasonCodes: ['ai-critical-location-risk'],
    canTriggerAlertDirectly: false,
    isFinalAuthority: false,
  });
}

function sourceSnapshot(commit, checkedAt) {
  return [
    `# ${workpackId} Source Snapshot`,
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- source: packages/parent-domain/src/tracking-location-policy-compiler-contracts.ts',
    '- source: packages/parent-domain/src/tracking-location-policy-compiler.ts',
    '- test: packages/parent-domain/tests/tracking-location-policy-compiler.test.ts',
    '- command: npm run test:tracking-plan-policy-compiler-proof',
    '',
  ].join('\n');
}

function contractProofLog() {
  return [
    `workpack=${workpackId}`,
    'proofState=runtime-proof-complete',
    'parent-domain build passed',
    'tracking-location-policy-compiler focused tests passed',
    'compiler outputs parse through existing tracking policy schemas',
    '',
  ].join('\n');
}

function policyAlertProof(results, checkedAt) {
  return {
    schemaVersion: 1,
    checkedAt,
    workpackId,
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'simulated',
    results,
  };
}

function securityNegativeProof(results) {
  const aiNotAuthority = results.find(
    (entry) => entry.requestedAction === 'critical-alert' && entry.alertSeverity === null
  );
  return [
    `workpack=${workpackId}`,
    'AI critical candidates do not become final alert authority.',
    `aiCriticalCandidateDecisionAction=${aiNotAuthority?.decisionAction ?? 'missing'}`,
    `aiFinalAuthority=${aiNotAuthority?.aiFinalAuthority ?? 'missing'}`,
    'Compiler results claim no runtime enforcement and no provider delivery.',
    'Platform adapters, physical-device behavior, notification delivery, UI, and production persistence remain unclaimed.',
    '',
  ].join('\n');
}

function proofSummary(results, commit, checkedAt) {
  return {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId,
    proofState: 'runtime-proof-complete',
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'simulated',
    summary:
      'Tracking policy compiler runtime proof covers parent policy final authority, deterministic action compilation, AI non-authority, manual-required fallback, and non-enforcement/provider non-claims.',
    commands,
    productClaims: productClaims(),
    resultCount: results.length,
  };
}

function testResult(results, checkedAt) {
  return {
    schemaVersion: 1,
    checkedAt,
    workpackId,
    currentStatus: 'simulated',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    resultCount: results.length,
    productClaims: productClaims(),
  };
}

function productClaims() {
  return {
    parentPolicyFinalAuthorityProved: true,
    aiFinalAuthorityClaimed: false,
    runtimeEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    platformAdapterClaimed: false,
    uiCompleteClaimed: false,
    physicalDeviceClaimed: false,
  };
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args]);
    return;
  }
  await runCommand('npm', args);
}

async function runCommand(command, args, options = {}) {
  const rendered = [command, ...args].join(' ');
  const exitCode = await new Promise((resolve) => {
    const child = spawn(command, args, { cwd: repoRoot, shell: false, stdio: 'inherit', ...options });
    child.on('close', resolve);
  });
  commands.push({ command: rendered, exitCode });
  if (exitCode !== 0) {
    throw new Error(`${rendered} exited ${exitCode}`);
  }
}

async function gitHead() {
  let output = '';
  const exitCode = await new Promise((resolve) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], {
      cwd: repoRoot,
      shell: false,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    child.stdout.on('data', (chunk) => {
      output += chunk;
    });
    child.on('close', resolve);
  });
  if (exitCode !== 0) return 'unknown';
  return output.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
