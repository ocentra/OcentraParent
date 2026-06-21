import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '75-source-freshness-preview-gate';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-source-freshness-preview-gate-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-05T23:45:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
  await mkdir(path, { recursive: true });
}

runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/app-game-domain',
  '--',
  '--run',
  'tests/unit/app-game-source-freshness-preview-gate.test.ts',
]);
runNpm(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']);

const schemaGate = await import('@ocentra-parent/schema-domain/app-game-source-freshness-preview-gate');
commands.push('node import @ocentra-parent/schema-domain/app-game-source-freshness-preview-gate');
if (!('AppGameSourceFreshnessPreviewGateReadModelSchema' in schemaGate)) {
  throw new Error('Missing AppGameSourceFreshnessPreviewGateReadModelSchema export from schema-domain');
}

const gate = await importAppGameDist('app-game-source-freshness-preview-gate.js');
const sourceData = await importAppGameDist('app-game-source-freshness-policy-consumption-data.js');
const compilerRules = await importSchemaDist('app-game-policy-target-compiler-rules.js');
const policy = await importSchemaDist('policy.js');
const refs = await importSchemaDist('family-reference-primitives.js');

const [readyAppSource, readyGameSource, manualGameSource] =
  sourceData.AppGameSourceFreshnessPolicyConsumptionMatrix.readiness;
const readModel = gate.buildAppGameSourceFreshnessPreviewGateReadModel(gateOptions(refs), [
  {
    rowId: 'source-gate-row-ready-app',
    sourceReadiness: readyAppSource,
    compiledDecision: compiledDecision({
      compilerRules,
      policy,
      refs,
      compiledDecisionId: 'compiled-source-gate-native-app',
      compileRequestId: 'compile-source-gate-native-app',
      policyDecisionId: 'policy-decision-source-gate-native-app',
      policyTargetId: 'policy-target-source-gate-native-app',
      targetKind: compilerRules.AppGamePolicyTargetKind.SpecificApp,
      targetRef: 'app-target-parental-controls-helper',
      requestedAction: compilerRules.AppGamePolicyCompilerRequestedAction.TimeLimit,
      policyAction: policy.PolicyAction.TimeLimit,
      capabilityState: compilerRules.AppGamePolicyCompilerCapabilityState.Supported,
      authorityState: compilerRules.AppGamePolicyCompilerAuthorityState.Proved,
      outcomeState: compilerRules.AppGamePolicyCompilerOutcomeState.DryRunReady,
      rejectionReason: compilerRules.AppGamePolicyCompilerRejectionReason.None,
      ruleId: 'rule-source-gate-native-app-time-limit',
      reasonCode: 'source-gate-native-app-time-limit',
      evidenceReferenceId: 'evidence-source-gate-native-app',
      capabilityRef: 'capability-source-gate-native-app',
      authorityRef: 'authority-source-gate-native-app',
      auditRef: 'audit-source-gate-native-app',
    }),
  },
  {
    rowId: 'source-gate-row-manual-game',
    sourceReadiness: manualGameSource,
    compiledDecision: null,
  },
  {
    rowId: 'source-gate-row-compiler-manual-game',
    sourceReadiness: readyGameSource,
    compiledDecision: compiledDecision({
      compilerRules,
      policy,
      refs,
      compiledDecisionId: 'compiled-source-gate-native-game-manual',
      compileRequestId: 'compile-source-gate-native-game-manual',
      policyDecisionId: 'policy-decision-source-gate-native-game-manual',
      policyTargetId: 'policy-target-source-gate-native-game-manual',
      targetKind: compilerRules.AppGamePolicyTargetKind.SpecificGame,
      targetRef: 'game-target-launcher-child-game',
      requestedAction: compilerRules.AppGamePolicyCompilerRequestedAction.BlockLaunch,
      policyAction: policy.PolicyAction.Block,
      capabilityState: compilerRules.AppGamePolicyCompilerCapabilityState.ManualRequired,
      authorityState: compilerRules.AppGamePolicyCompilerAuthorityState.ManualRequired,
      outcomeState: compilerRules.AppGamePolicyCompilerOutcomeState.ManualRequired,
      rejectionReason: compilerRules.AppGamePolicyCompilerRejectionReason.BlockLaunchManualRequired,
      ruleId: 'rule-source-gate-native-game-block-launch',
      reasonCode: 'source-gate-native-game-block-launch',
      evidenceReferenceId: 'evidence-source-gate-native-game',
      capabilityRef: 'capability-source-gate-native-game-manual',
      authorityRef: 'authority-source-gate-native-game-manual',
      auditRef: 'audit-source-gate-native-game-manual',
    }),
  },
]);

const proof = {
  proofMode: 'app-game-source-freshness-preview-gate',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    requiredBranch: 'origin/main',
    reason: 'WP74 source freshness policy-consumption contracts have landed on main; WP75 targets current main.',
  },
  summary: summarize(readModel),
  nonClaims: {
    policyEvaluatorRuntimeClaimed: readModel.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: readModel.timerRuntimeClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    childDeliveryClaimed: readModel.childDeliveryClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
  },
  proofPaths: {
    schemaSource: 'packages/schema-domain/src/app-game-source-freshness-preview-gate.ts',
    schemaRules: 'packages/schema-domain/src/app-game-source-freshness-preview-gate-rules.ts',
    consumerSource: 'packages/app-game-domain/src/app-game-source-freshness-preview-gate.ts',
    consumerTest: 'packages/app-game-domain/tests/unit/app-game-source-freshness-preview-gate.test.ts',
    harness: 'scripts/test/app-game-source-freshness-preview-gate-proof.mjs',
    evidence: 'test-results/app-game-source-freshness-preview-gate-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'source-freshness-preview-gate-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP75');
await writeProofPack(appProofDir, proof, 'app WP75');

console.log('app-game-source-freshness-preview-gate-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-source-freshness-preview-gate-proof', 'proof.json')}`);

function importAppGameDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'app-game-domain', 'dist', name)).href);
}

function importSchemaDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', name)).href);
}

function gateOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    gateId: 'app-game-source-freshness-preview-gate-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-freshness-policy-consumption',
      'app-game-policy-preview-handoff',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
    policyPreviewOptions: {
      schemaVersion: refs.ParentContractSchemaVersion.V0_6,
      handoffId: 'app-game-source-freshness-preview-gate-handoff',
      generatedAt: timestamp,
      sourceContractRefs: ['app-game-policy-target-compiler', 'app-game-policy-preview-handoff'],
    },
  };
}

function childDevice(refs) {
  return {
    deviceId: 'device-windows-source-gate',
    childProfileId: 'child-source-gate',
    label: 'Study PC',
    platform: refs.ParentPlatform.Windows,
  };
}

function evidenceRef(refs, evidenceReferenceId) {
  return {
    evidenceReferenceId,
    kind: refs.ParentEvidenceReferenceKind.ActivityEvent,
    observedAt: timestamp,
  };
}

function compiledDecision(input) {
  const evidenceReference = evidenceRef(input.refs, input.evidenceReferenceId);
  const request = {
    schemaVersion: input.refs.ParentContractSchemaVersion.V0_6,
    compileRequestId: input.compileRequestId,
    policyVersion: 'app-game-source-gate-policy-version',
    ruleId: input.ruleId,
    device: childDevice(input.refs),
    localUserRef: 'windows-local-user-source-gate',
    target: {
      targetKind: input.targetKind,
      targetRef: input.targetRef,
    },
    requestedAction: input.requestedAction,
    policyAction: input.policyAction,
    scheduleRef: null,
    evidence: [
      {
        evidenceReference,
        proofKind: input.compilerRules.AppGamePolicyCompilerProofKind.Identity,
        evidenceState: input.compilerRules.AppGamePolicyCompilerEvidenceState.Active,
        device: childDevice(input.refs),
        localUserRef: 'windows-local-user-source-gate',
        observedAt: timestamp,
      },
    ],
    capabilityRefs: [
      {
        capabilityRef: input.capabilityRef,
        capabilityState: input.capabilityState,
        evidenceReferences: [evidenceReference],
      },
    ],
    authorityRefs: [
      {
        authorityRef: input.authorityRef,
        authorityState: input.authorityState,
        evidenceReferences: [evidenceReference],
      },
    ],
    requestedAt: timestamp,
  };

  return {
    schemaVersion: input.refs.ParentContractSchemaVersion.V0_6,
    compiledDecisionId: input.compiledDecisionId,
    request,
    policyTarget: {
      targetId: input.policyTargetId,
      targetType: 'app',
      targetValue: input.targetRef,
    },
    policyDecision: {
      schemaVersion: input.refs.ParentContractSchemaVersion.V0_6,
      decisionId: input.policyDecisionId,
      action: input.policyAction,
      reasonCodes: [input.reasonCode],
      evidenceReferences: [evidenceReference],
      ruleIds: [input.ruleId],
      localAiResultId: null,
      dryRun: true,
      enforcementHandoffState: input.policy.PolicyDecisionHandoffState.Disabled,
      expiresAt: null,
    },
    outcomeState: input.outcomeState,
    rejectionReason: input.rejectionReason,
    capabilityRefs: [input.capabilityRef],
    authorityRefs: [input.authorityRef],
    auditRefs: [input.auditRef],
    compiledAt: timestamp,
  };
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    previewReadyCount: readModel.previewReadyCount,
    manualRequiredCount: readModel.manualRequiredCount,
    sourceManualRequiredCount: readModel.sourceManualRequiredCount,
    compilerManualRequiredCount: readModel.compilerManualRequiredCount,
    gateStates: countBy(readModel.rows.map((row) => row.gateState)),
    statuses: countBy(readModel.rows.map((row) => row.previewStatus)),
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.nativeAppRowCount !== 1 ||
    proof.summary.nativeGameRowCount !== 2 ||
    proof.summary.previewReadyCount !== 1 ||
    proof.summary.manualRequiredCount !== 2 ||
    proof.summary.sourceManualRequiredCount !== 1 ||
    proof.summary.compilerManualRequiredCount !== 1
  ) {
    throw new Error(`Unexpected source freshness preview gate summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Source freshness preview gate overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
}

async function writeProofPack(proofDir, proof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: app/game source freshness readiness gating before read-only policy preview handoff.',
      '- Stack note: schema-domain owns the preview-gate contract surface; app-game-domain consumes it for local gate construction.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/schema-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/app-game-domain -- --run tests/unit/app-game-source-freshness-preview-gate.test.ts: PASS',
      '- cmd /c npm run build --workspace @ocentra-parent/app-game-domain: PASS',
      '- Policy-ready source freshness rows can build read-only policy preview rows.',
      '- Manual-required source freshness rows block preview before a compiled decision is accepted.',
      '- Source/compiled app-game target domain mismatches are rejected.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust protocol proof not applicable: this workpack validates schema-domain contract ownership plus the app-game-domain consumer only.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof.summary);
  await writeJson(join(proofDir, '04-journal-sqlite-proof.json'), {
    schemaVersion: 1,
    journalSqliteChanged: false,
    sourceFreshnessRowsFromActivityReadModel: true,
    previewGateArtifact:
      'test-results/app-game-source-freshness-preview-gate-proof/source-freshness-preview-gate-read-model.json',
  });
  await writeJson(join(proofDir, '05-policy-action-proof.json'), {
    schemaVersion: 1,
    previewReadyRows: proof.summary.previewReadyCount,
    manualRequiredRows: proof.summary.manualRequiredCount,
    sourceManualRequiredRows: proof.summary.sourceManualRequiredCount,
    compilerManualRequiredRows: proof.summary.compilerManualRequiredCount,
    policyEvaluatorRuntimeClaimed: false,
    timerRuntimeClaimed: false,
    adapterDispatchClaimed: false,
    childDeliveryClaimed: false,
    platformEnforcementClaimed: false,
  });
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo portal, parent notification, child-facing, or adapter UI source changed in this workpack.\n',
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
      '- Manual-required source freshness rows produce no preview row and accept no compiled decision.',
      '- Policy-ready source freshness rows must match compiled decision native app/native game domain.',
      '- Preview rows remain dry-run-only through the existing WP70 handoff contract.',
      '- Policy evaluator runtime, timers, child delivery, adapter dispatch, and platform enforcement remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNo live platform authority tier is raised. The gate does not execute adapters, block launches, start timers, or deliver child notifications.\n',
    'utf8'
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nThe gate consumes source readiness and compiled policy preview rows only; it does not upgrade authority, capability, or platform support.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-rollback-proof.md'),
    '# Rollback Proof\n\nNo runtime state, timer, provider send, child delivery, block, suspend, shield, or adapter dispatch is created. Rollback is deleting generated proof artifacts.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, 'README.md'),
    `# ${label} Source Freshness Preview Gate Proof\n\nThis proof pack records source freshness readiness gating before read-only app/game policy preview rows.\n`,
    'utf8'
  );
  await writeJson(join(proofDir, 'proof.json'), proof);
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

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
