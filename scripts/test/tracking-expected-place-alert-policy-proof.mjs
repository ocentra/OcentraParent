import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const timestamp = '2026-06-06T17:16:00.000Z';
const testOutputDir = join(repoRoot, 'test-results', 'tracking-expected-place-alert-policy-proof');
const wp16Dir = join(repoRoot, 'output', 'tracking-plan-proof', '16-expected-place-schedule-engine');
const wp33Dir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'tracking-expected-place-alert-policy-proof');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await rm(proofDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp16Dir, { recursive: true });
await mkdir(wp33Dir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpmCommand(run, ['run', 'build', '--workspace', '@ocentra-parent/tracking-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-expected-place-alert-policy-proof.test.ts',
]);

const tracking = await importSchemaDist('tracking-location-policy.js');
const proofModule = await importTrackingDist('tracking-expected-place-alert-policy-proof.js');
const readModel = tracking.TrackingLocationPolicyReadModelSchema.parse(sourceReadModel(tracking));
const expectedPlaceProof = proofModule.buildTrackingExpectedPlaceAlertPolicyProof({
  generatedAt: timestamp,
  sourceReadModelRef: 'tracking-location-policy-read-model-expected-place-alert-policy',
  sourceProofRefs: [
    'output/tracking-plan-proof/16-expected-place-schedule-engine/06-expected-place-proof.json',
    'output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/proof.json',
  ],
  readModel,
});
const proof = {
  proofMode: 'tracking-expected-place-alert-policy-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(expectedPlaceProof),
  proofPaths: proofPaths(),
  nonClaims: nonClaims(expectedPlaceProof),
  expectedPlaceProof,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeJson(join(testOutputDir, 'expected-place-alert-policy-read-model.json'), expectedPlaceProof);
await writeProofPack(proofDir, proof);
await writeJson(join(wp16Dir, '29-expected-place-alert-policy-proof.json'), proof);
await writeJson(join(wp33Dir, '29-expected-place-alert-policy-proof.json'), proof);

console.log('tracking-expected-place-alert-policy-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-expected-place-alert-policy-proof', 'proof.json')}`);

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function importTrackingDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function sourceReadModel(tracking) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    generatedAt: timestamp,
    rules: [
      expectedPlaceRule(tracking, 'school-arrival', 'notify-parent'),
      expectedPlaceRule(tracking, 'late-bus', 'ask-child-check-in'),
      expectedPlaceRule(tracking, 'holiday', 'no-action'),
      expectedPlaceRule(tracking, 'low-accuracy', 'manual-required'),
    ],
    decisions: [
      decision(tracking, 'school', 'school-arrival', 'notify-parent', 'expected-place-alert-school-arrival'),
      decision(tracking, 'late-bus', 'late-bus', 'ask-child-check-in', null),
      decision(tracking, 'holiday', 'holiday', 'no-action', null),
      decision(tracking, 'low-accuracy', 'low-accuracy', 'manual-required', null),
    ],
    acknowledgements: [],
    checkInRequests: [],
    checkInResponses: [],
    aiRoutes: [],
    aiResults: [],
    alerts: [alert(tracking)],
    escalations: [],
    temporaryLiveGrants: [],
    missingDeviceCases: [],
    platformProofRoutes: [],
  };
}

function expectedPlaceRule(tracking, suffix, action) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    ruleId: `expected-place-rule-${suffix}`,
    familyId: 'family-expected-place-alert-policy',
    childProfileId: 'child-expected-place-alert-policy',
    deviceId: 'parent-device-expected-place-alert-policy',
    policyVersion: 'tracking-policy-expected-place-alert-policy',
    targetKind: 'expected-place',
    action,
    enabled: true,
    requiresFreshEvidence: true,
    requiresParentConfirmation: action === 'manual-required',
    reasonCodes: [`expected-place-${suffix}`],
    auditRefs: [`expected-place-rule-audit-${suffix}`],
  };
}

function decision(tracking, label, ruleSuffix, action, alertIntentId) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    decisionId: `expected-place-decision-${label}`,
    decidedAt: timestamp,
    ruleId: `expected-place-rule-${ruleSuffix}`,
    action,
    dryRun: false,
    evidenceReferences: [evidence(label)],
    aiAnalysisId: null,
    alertIntentId,
    reasonCodes: [`expected-place-decision-${label}`],
    auditRefs: [`expected-place-decision-audit-${label}`],
  };
}

function alert(tracking) {
  return {
    schemaVersion: tracking.TrackingPolicySchemaVersion,
    alertId: 'expected-place-alert-school-arrival',
    createdAt: timestamp,
    severity: 'warning',
    policyDecisionId: 'expected-place-decision-school',
    evidenceReferences: [evidence('school')],
    sensitiveDetailMode: 'minimal-provider-body',
    notificationStatusRefs: ['expected-place-notification-intent-school'],
    acknowledgementId: null,
    reasonCodes: ['expected-place-school-arrival-alert'],
  };
}

function evidence(label) {
  return {
    evidenceReferenceId: `expected-place-evidence-${label}`,
    kind: 'journal-event',
    observedAt: timestamp,
  };
}

function summarize(expectedPlaceProof) {
  return {
    rows: expectedPlaceProof.rows.length,
    alertPolicyReadyCount: expectedPlaceProof.alertPolicyReadyCount,
    checkInPolicyReadyCount: expectedPlaceProof.checkInPolicyReadyCount,
    suppressedNoActionCount: expectedPlaceProof.suppressedNoActionCount,
    manualRequiredCount: expectedPlaceProof.manualRequiredCount,
    statuses: countBy(expectedPlaceProof.rows.map((row) => row.status)),
  };
}

function nonClaims(expectedPlaceProof) {
  return {
    renderedParentUiClaimed: expectedPlaceProof.renderedParentUiClaimed,
    alertDeliveryRuntimeClaimed: expectedPlaceProof.alertDeliveryRuntimeClaimed,
    providerDeliveryClaimed: expectedPlaceProof.providerDeliveryClaimed,
    notificationReceiptRuntimeClaimed: expectedPlaceProof.notificationReceiptRuntimeClaimed,
    childDeviceRuntimeClaimed: expectedPlaceProof.childDeviceRuntimeClaimed,
    physicalDeviceProofClaimed: expectedPlaceProof.physicalDeviceProofClaimed,
    authorityProofClaimed: expectedPlaceProof.authorityProofClaimed,
    productionWorkerClaimed: expectedPlaceProof.productionWorkerClaimed,
    adapterDispatchClaimed: expectedPlaceProof.adapterDispatchClaimed,
  };
}

function proofPaths() {
  return {
    source: 'packages/schema-domain/src/tracking-expected-place-alert-policy-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-expected-place-alert-policy-proof.test.ts',
    harness: 'scripts/test/tracking-expected-place-alert-policy-proof.mjs',
    evidence: 'test-results/tracking-expected-place-alert-policy-proof/proof.json',
    focusedProofRoot: 'output/tracking-plan-proof/tracking-expected-place-alert-policy-proof',
    wp16: 'output/tracking-plan-proof/16-expected-place-schedule-engine/29-expected-place-alert-policy-proof.json',
    wp33: 'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/29-expected-place-alert-policy-proof.json',
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 4 ||
    proof.summary.alertPolicyReadyCount !== 1 ||
    proof.summary.checkInPolicyReadyCount !== 1 ||
    proof.summary.suppressedNoActionCount !== 1 ||
    proof.summary.manualRequiredCount !== 1
  ) {
    throw new Error(`Unexpected expected-place alert policy summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Expected-place alert policy proof overclaimed behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Expected-Place Alert Policy Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: expected-place policy decisions mapped into parent alert/check-in/suppression/manual UI-readiness rows.',
      '- Source refs carried by this proof: WP16 expected-place proof, WP25 policy compiler proof, tracking feature doc, implementation checklist, and WP33 proof gate.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/schema-domain: PASS',
      '- cmd /c npm run build --workspace @ocentra-parent/tracking-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tracking-expected-place-alert-policy-proof.test.ts: PASS',
      '- Expected-place policy decisions preserve schedule rule refs, alert refs, evidence refs, reason refs, audit refs, and UI-readiness refs.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Rendered parent UI, alert delivery runtime, provider delivery, notification receipt runtime, child-device runtime, physical-device proof, authority proof, production workers, and adapter dispatch remain false.',
      '- UI-readiness rows do not expose raw child evidence; evidence remains referenced through typed evidence refs.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeJson(join(path, 'proof.json'), proof);
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
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
