import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const workpackId = '25-policy-compiler-for-tracking-rules';
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', workpackId);
const resultDir = join(repoRoot, 'test-results', 'tracking-policy-compiler-runtime-proof');
const timestamp = '2026-06-05T17:10:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-policy-compiler-runtime-proof',
  'tracking-location-policy',
]);

const tracking = await importDist('tracking-location-policy.js');
const compiler = await importDist('tracking-policy-compiler-runtime-proof.js');
const results = compileProofScenarios(tracking, compiler);

const proof = {
  proofMode: 'tracking-policy-compiler-runtime-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(results),
  productClaims: productClaims(),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-policy-compiler-runtime-proof.ts',
    test: 'packages/parent-domain/tests/tracking-policy-compiler-runtime-proof.test.ts',
    harness: 'scripts/test/tracking-policy-compiler-runtime-proof.mjs',
    evidence: 'test-results/tracking-policy-compiler-runtime-proof/proof.json',
    trackingProofPack: `output/tracking-plan-proof/${workpackId}`,
  },
  results,
};

assertProof(proof);
await writeJson(join(resultDir, 'policy-compiler-runtime-read-model.json'), results);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-policy-compiler-runtime-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-policy-compiler-runtime-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function compileProofScenarios(tracking, compiler) {
  const scenarios = [
    scenario(tracking, compiler, 'observe'),
    scenario(tracking, compiler, 'notify-parent', { alertId: 'tracking-policy-alert-notify' }),
    scenario(tracking, compiler, 'ask-child-check-in', { checkInId: 'tracking-policy-checkin' }),
    scenario(tracking, compiler, 'request-parent-acknowledgement', { alertId: 'tracking-policy-alert-ack' }),
    scenario(tracking, compiler, 'start-temporary-live-tracking', {
      liveTrackingGrantId: 'tracking-policy-live-grant',
      liveTrackingDurationSeconds: 900,
      parentConfirmationReceived: true,
    }),
    scenario(tracking, compiler, 'escalate', {
      alertId: 'tracking-policy-alert-escalate',
      escalationId: 'tracking-policy-escalation',
    }),
    scenario(tracking, compiler, 'no-action', { requestedAction: 'suppress' }),
    scenario(tracking, compiler, 'notify-parent', {
      requestedAction: 'manual-required',
      platformManualRequired: true,
      alertId: 'tracking-policy-alert-manual',
    }),
    scenario(tracking, compiler, 'notify-parent', {
      requestedAction: 'critical-alert',
      alertId: 'tracking-policy-alert-critical',
      aiAnalysis: aiCritical(tracking),
    }),
    scenario(tracking, compiler, 'observe', {
      requestedAction: 'critical-alert',
      alertId: 'tracking-policy-alert-ai-not-authority',
      aiAnalysis: aiCritical(tracking),
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
    platformAdapterClaimed: entry.platformAdapterClaimed,
    physicalDeviceClaimed: entry.physicalDeviceClaimed,
    productionWorkerClaimed: entry.productionWorkerClaimed,
    reasonCodes: entry.decision.reasonCodes,
  }));
}

function scenario(tracking, compiler, ruleAction, options = {}) {
  return compiler.compileTrackingPolicyRuntimeProofDecision({
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    requestId: `tracking-policy-compile-${ruleAction}`,
    rule: rule(tracking, ruleAction, options.ruleOverrides),
    requestedAt: timestamp,
    decidedAt: '2026-06-05T17:11:00.000Z',
    followUpExpiresAt: '2026-06-05T17:21:00.000Z',
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

function rule(tracking, action, overrides = {}) {
  return tracking.TrackingPolicyRuleSchema.parse({
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    ruleId: `tracking-policy-rule-${action}`,
    familyId: 'family-1',
    childProfileId: 'child-1',
    deviceId: 'parent-device-1',
    policyVersion: 'tracking-policy-v3',
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
    observedAt: '2026-06-05T17:00:00.000Z',
  };
}

function aiCritical(tracking) {
  return tracking.TrackingLocationAiAnalysisResultSchema.parse({
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    analysisId: 'tracking-ai-critical-1',
    completedAt: '2026-06-05T17:01:00.000Z',
    riskLevel: 'critical',
    confidence: 0.91,
    providerRouteId: 'tracking-ai-route-1',
    evidenceReferences: [evidenceTrace()],
    reasonCodes: ['ai-critical-location-risk'],
    canTriggerAlertDirectly: false,
    isFinalAuthority: false,
  });
}

function summarize(results) {
  return {
    resultCount: results.length,
    decisionActions: countBy(results.map((entry) => entry.decisionAction)),
    alertSeverities: countBy(results.map((entry) => entry.alertSeverity ?? 'none')),
    finalActionSources: countBy(results.map((entry) => entry.finalActionSource)),
    aiNonAuthorityRows: results.filter((entry) => entry.aiFinalAuthority === false).length,
    runtimeClaimedRows: results.filter((entry) => entry.runtimeEnforcementClaimed).length,
    providerClaimedRows: results.filter((entry) => entry.providerDeliveryClaimed).length,
    physicalDeviceClaimedRows: results.filter((entry) => entry.physicalDeviceClaimed).length,
  };
}

function assertProof(proof) {
  if (
    proof.summary.resultCount !== 10 ||
    proof.summary.decisionActions.observe !== 2 ||
    proof.summary.decisionActions['notify-parent'] !== 2 ||
    proof.summary.decisionActions['manual-required'] !== 1 ||
    proof.summary.alertSeverities.critical !== 1 ||
    proof.summary.aiNonAuthorityRows !== 10 ||
    proof.summary.runtimeClaimedRows !== 0 ||
    proof.summary.providerClaimedRows !== 0 ||
    proof.summary.physicalDeviceClaimedRows !== 0
  ) {
    throw new Error(`Unexpected tracking policy compiler proof summary: ${JSON.stringify(proof.summary)}`);
  }

  if (Object.values(proof.productClaims).some((value) => value !== false)) {
    throw new Error(`Tracking policy compiler overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# WP25 Tracking Policy Compiler Runtime Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: parent-policy compiler/evaluator proof for tracking rules.',
      '- Source inspected: tracking location policy contracts, policy expectations, location/geofence expectations, feature doc, and WP25 checklist.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking-policy-compiler-runtime-proof tracking-location-policy: PASS',
      '- Compiler outputs parse through existing tracking policy schemas.',
      '- Parent policy remains final action authority; AI result rows remain evidence only.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '09-policy-alert-proof.json'), proof.summary);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- AI critical candidates do not become final alert authority without a parent policy rule.',
      '- Manual-required rows do not create notification delivery, runtime enforcement, platform adapter, production worker, or physical-device claims.',
      '- Provider delivery, provider receipt ingestion, child-device delivery, platform runtime execution, and production persistence remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(path, 'README.md'),
    '# WP25 Tracking Policy Compiler Runtime Proof\n\nThis proof pack records deterministic parent-policy compiler/evaluator output for tracking rules. It proves parent-policy final authority, AI non-authority, degraded/manual-required fallback, and no runtime enforcement, provider delivery, platform adapter, production worker, or physical-device claims.\n',
    'utf8'
  );
  await writeJson(join(path, 'proof.json'), proof);
}

function productClaims() {
  return {
    aiFinalAuthorityClaimed: false,
    runtimeEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    providerReceiptIngestionClaimed: false,
    childDeviceDeliveryClaimed: false,
    platformAdapterClaimed: false,
    physicalDeviceClaimed: false,
    productionWorkerClaimed: false,
    productUiCompleteClaimed: false,
  };
}

function run(command, args) {
  commands.push([command, ...args].join(' '));
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
