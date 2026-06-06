import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofSlug = '76-source-gated-policy-preview-read-model';
const testOutputDir = join(repoRoot, 'test-results', 'app-game-source-gated-policy-preview-read-model-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', proofSlug);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', proofSlug);
const timestamp = '2026-06-06T00:05:00Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

for (const path of [testOutputDir, appGameProofDir, appProofDir]) {
  await rm(path, { recursive: true, force: true });
  await mkdir(path, { recursive: true });
}
for (const path of [join(appGameProofDir, '06-ui-snapshots'), join(appProofDir, '06-ui-snapshots')]) {
  await mkdir(path, { recursive: true });
}

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'app-game-source-gated-policy-preview-read-model',
  'app-game-source-freshness-preview-gate',
  'app-game-source-freshness-policy-consumption',
  'app-game-policy-preview-handoff',
]);

const readModelContract = await importDist('app-game-source-gated-policy-preview-read-model.js');
const gate = await importDist('app-game-source-freshness-preview-gate.js');
const sourceData = await importDist('app-game-source-freshness-policy-consumption-data.js');
const compilerRules = await importDist('app-game-policy-target-compiler-rules.js');
const policy = await importDist('policy.js');
const refs = await importDist('reference-primitives.js');

const [readyAppSource, readyGameSource, manualGameSource] =
  sourceData.AppGameSourceFreshnessPolicyConsumptionMatrix.readiness;
const gateReadModel = gate.buildAppGameSourceFreshnessPreviewGateReadModel(gateOptions(refs), [
  {
    rowId: 'source-gate-row-ready-app',
    sourceReadiness: readyAppSource,
    compiledDecision: compiledDecision({
      compilerRules,
      policy,
      refs,
      compiledDecisionId: 'compiled-source-gated-preview-native-app',
      compileRequestId: 'compile-source-gated-preview-native-app',
      policyDecisionId: 'policy-decision-source-gated-preview-native-app',
      policyTargetId: 'policy-target-source-gated-preview-native-app',
      targetKind: compilerRules.AppGamePolicyTargetKind.SpecificApp,
      targetRef: 'app-target-parental-controls-helper',
      requestedAction: compilerRules.AppGamePolicyCompilerRequestedAction.TimeLimit,
      policyAction: policy.PolicyAction.TimeLimit,
      capabilityState: compilerRules.AppGamePolicyCompilerCapabilityState.Supported,
      authorityState: compilerRules.AppGamePolicyCompilerAuthorityState.Proved,
      outcomeState: compilerRules.AppGamePolicyCompilerOutcomeState.DryRunReady,
      rejectionReason: compilerRules.AppGamePolicyCompilerRejectionReason.None,
      ruleId: 'rule-source-gated-preview-native-app-time-limit',
      reasonCode: 'source-gated-preview-native-app-time-limit',
      evidenceReferenceId: 'evidence-source-gated-preview-native-app',
      capabilityRef: 'capability-source-gated-preview-native-app',
      authorityRef: 'authority-source-gated-preview-native-app',
      auditRef: 'audit-source-gated-preview-native-app',
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
      compiledDecisionId: 'compiled-source-gated-preview-native-game-manual',
      compileRequestId: 'compile-source-gated-preview-native-game-manual',
      policyDecisionId: 'policy-decision-source-gated-preview-native-game-manual',
      policyTargetId: 'policy-target-source-gated-preview-native-game-manual',
      targetKind: compilerRules.AppGamePolicyTargetKind.SpecificGame,
      targetRef: 'game-target-launcher-child-game',
      requestedAction: compilerRules.AppGamePolicyCompilerRequestedAction.BlockLaunch,
      policyAction: policy.PolicyAction.Block,
      capabilityState: compilerRules.AppGamePolicyCompilerCapabilityState.ManualRequired,
      authorityState: compilerRules.AppGamePolicyCompilerAuthorityState.ManualRequired,
      outcomeState: compilerRules.AppGamePolicyCompilerOutcomeState.ManualRequired,
      rejectionReason: compilerRules.AppGamePolicyCompilerRejectionReason.BlockLaunchManualRequired,
      ruleId: 'rule-source-gated-preview-native-game-block-launch',
      reasonCode: 'source-gated-preview-native-game-block-launch',
      evidenceReferenceId: 'evidence-source-gated-preview-native-game',
      capabilityRef: 'capability-source-gated-preview-native-game-manual',
      authorityRef: 'authority-source-gated-preview-native-game-manual',
      auditRef: 'audit-source-gated-preview-native-game-manual',
    }),
  },
]);
const readModel = readModelContract.buildAppGameSourceGatedPolicyPreviewReadModel(
  readModelOptions(refs),
  gateReadModel
);
const proof = {
  proofMode: 'app-game-source-gated-policy-preview-read-model',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  stackedOn: {
    wp75Branch: 'codex/app-game-source-freshness-preview-gate',
    requiredBranch: 'codex/app-game-source-freshness-preview-gate',
    reason:
      'WP76 consumes WP75 preview-gate contracts that are not on origin/main yet; WP74 source freshness policy consumption has landed on origin/main.',
  },
  summary: summarize(readModel),
  nonClaims: {
    serviceRuntimeEventClaimed: readModel.serviceRuntimeEventClaimed,
    portalUiRendered: readModel.portalUiRendered,
    policyEvaluatorRuntimeClaimed: readModel.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: readModel.timerRuntimeClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    childDeliveryClaimed: readModel.childDeliveryClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
    rawPrivateSourceRowsIncluded: readModel.rawPrivateSourceRowsIncluded,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-source-gated-policy-preview-read-model.ts',
    rules: 'packages/parent-domain/src/app-game-source-gated-policy-preview-read-model-rules.ts',
    test: 'packages/parent-domain/tests/app-game-source-gated-policy-preview-read-model.test.ts',
    harness: 'scripts/test/app-game-source-gated-policy-preview-read-model-proof.mjs',
    evidence: 'test-results/app-game-source-gated-policy-preview-read-model-proof/proof.json',
    appGameProofPack: `output/app-game-plan-proof/${proofSlug}`,
    appProofPack: `output/app-plan-proof/${proofSlug}`,
  },
  gateSummary: summarizeGate(gateReadModel),
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'source-gated-policy-preview-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'source-freshness-preview-gate-read-model.json'), gateReadModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP76');
await writeProofPack(appProofDir, proof, 'app WP76');

console.log('app-game-source-gated-policy-preview-read-model-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-source-gated-policy-preview-read-model-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
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
      handoffId: 'app-game-source-gated-policy-preview-handoff',
      generatedAt: timestamp,
      sourceContractRefs: ['app-game-policy-target-compiler', 'app-game-policy-preview-handoff'],
    },
  };
}

function readModelOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    readModelId: 'app-game-source-gated-policy-preview-read-model-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-source-freshness-preview-gate',
      'docs/expectations/app-game-evidence.md',
      'docs/expectations/policy.md',
    ],
  };
}

function childDevice(refs) {
  return {
    deviceId: 'device-windows-source-gated-preview',
    childProfileId: 'child-source-gated-preview',
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
    policyVersion: 'app-game-source-gated-preview-policy-version',
    ruleId: input.ruleId,
    device: childDevice(input.refs),
    localUserRef: 'windows-local-user-source-gated-preview',
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
        localUserRef: 'windows-local-user-source-gated-preview',
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
    previewReadyVisibleCount: readModel.previewReadyVisibleCount,
    sourceManualRequiredVisibleCount: readModel.sourceManualRequiredVisibleCount,
    compilerManualRequiredVisibleCount: readModel.compilerManualRequiredVisibleCount,
    projectionStates: countBy(readModel.rows.map((row) => row.projectionState)),
    previewDecisionRefs: readModel.rows.map((row) => row.previewDecisionRef),
  };
}

function summarizeGate(gateReadModel) {
  return {
    rows: gateReadModel.rows.length,
    previewReadyCount: gateReadModel.previewReadyCount,
    sourceManualRequiredCount: gateReadModel.sourceManualRequiredCount,
    compilerManualRequiredCount: gateReadModel.compilerManualRequiredCount,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 3 ||
    proof.summary.nativeAppRowCount !== 1 ||
    proof.summary.nativeGameRowCount !== 2 ||
    proof.summary.previewReadyVisibleCount !== 1 ||
    proof.summary.sourceManualRequiredVisibleCount !== 1 ||
    proof.summary.compilerManualRequiredVisibleCount !== 1
  ) {
    throw new Error(`Unexpected source-gated policy preview summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Source-gated policy preview overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
  }
  if (proof.summary.previewDecisionRefs[1] !== null) {
    throw new Error('Expected source-manual-required rows to omit preview decision refs');
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
      '- Scope: source-gated policy preview read-model contract derived from WP75 gate rows.',
      '- Stack note: this branch depends on WP74/WP75 until those stacked PRs land.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-source-gated-policy-preview-read-model app-game-source-freshness-preview-gate app-game-source-freshness-policy-consumption app-game-policy-preview-handoff: PASS',
      '- Read-model rows derive from the WP75 source freshness preview gate.',
      '- Source-manual rows keep previewDecisionRef null before compiler output.',
      '- Compiler-manual rows remain distinct from source-manual rows.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), {
    runtimeEvidence: 'contract-only',
    sourceGateSummary: proof.gateSummary,
    readModelSummary: proof.summary,
  });
  await writeJson(join(proofDir, '05-policy-action-proof.json'), {
    dryRunOnly: true,
    previewReadyVisibleCount: proof.summary.previewReadyVisibleCount,
    sourceManualRequiredVisibleCount: proof.summary.sourceManualRequiredVisibleCount,
    compilerManualRequiredVisibleCount: proof.summary.compilerManualRequiredVisibleCount,
    nonClaims: proof.nonClaims,
  });
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'README.md'),
    'No UI screenshots: WP76 is a parent-domain read-model contract/proof only.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '07-playwright-ui-proof.log'),
    'N/A: no portal or child UI changed in WP76.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Source rows are exposed as evidence refs only.',
      '- Raw private source rows included: false.',
      '- Service runtime event claimed: false.',
      '- Portal UI rendered: false.',
      '- Policy evaluator, timer, adapter dispatch, child delivery, and platform enforcement claimed: false.',
      '- Manual-required source rows cannot produce preview decision refs.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    [
      '# Manual Platform Proof',
      '',
      'WP76 does not add platform adapters, live OS reads, service runtime events, portal UI, or enforcement.',
      'Platform proof remains manual-required until a future promoted adapter/runtime workpack supplies live evidence.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [...proof.commands, 'node scripts/test/app-game-source-gated-policy-preview-read-model-proof.mjs'].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    [
      '# Authority Tier Proof',
      '',
      '- Authority tier: read-only contract projection.',
      '- Setup state: no service runtime, no adapter, no provider, no portal UI.',
      '- Proof needed to move up: agent-protocol command/event, real service derivation, portal renderer, and runtime validation.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-rollback-proof.md'),
    [
      '# Rollback Proof',
      '',
      'Rollback removes only the WP76 parent-domain contract, tests, proof harness, docs, and generated proof outputs.',
      'No service state, timers, adapters, queues, provider credentials, child delivery, or platform settings are created.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(proofDir, 'proof.json'), proof);
}

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  const rendered = `${command} ${args.join(' ')}`;
  commands.push(`${rendered}\nexit=${result.status}\n${result.stdout}${result.stderr}`.trim());
  if (result.status !== 0) {
    throw new Error(`${rendered} failed with exit ${result.status}\n${result.stdout}\n${result.stderr}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

async function writeJson(path, value) {
  await writeFile(`${path}`, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
