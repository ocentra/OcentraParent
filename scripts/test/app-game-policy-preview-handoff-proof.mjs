import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const testOutputDir = join(repoRoot, 'test-results', 'app-game-policy-preview-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '70-policy-preview-handoff');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '70-policy-preview-handoff');
const timestamp = '2026-06-05T14:55:00Z';
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
  'app-game-policy-preview-handoff',
  'app-game-policy-target-compiler',
]);

const preview = await importDist('app-game-policy-preview-handoff.js');
const packagePreview = await import('@ocentra-parent/parent-domain/app-game-policy-preview-handoff');
const compilerRules = await importDist('app-game-policy-target-compiler-rules.js');
const policy = await importDist('policy.js');
const refs = await importDist('reference-primitives.js');
commands.push('node import @ocentra-parent/parent-domain/app-game-policy-preview-handoff');
if (typeof packagePreview.buildAppGamePolicyPreviewHandoffReadModel !== 'function') {
  throw new Error('Expected package export to expose buildAppGamePolicyPreviewHandoffReadModel');
}

const readModel = preview.buildAppGamePolicyPreviewHandoffReadModel(previewOptions(refs), [
  appPreviewDecision(compilerRules, policy, refs),
  gameManualPreviewDecision(compilerRules, policy, refs),
]);
const proof = {
  proofMode: 'app-game-policy-preview-handoff',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  commit: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel),
  nonClaims: {
    policyEvaluatorRuntimeClaimed: readModel.policyEvaluatorRuntimeClaimed,
    timerRuntimeClaimed: readModel.timerRuntimeClaimed,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    childDeliveryClaimed: readModel.childDeliveryClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
  },
  proofPaths: {
    source: 'packages/parent-domain/src/app-game-policy-preview-handoff.ts',
    rules: 'packages/parent-domain/src/app-game-policy-preview-handoff-rules.ts',
    packageExport: 'packages/parent-domain/package.json',
    packageReadme: 'packages/parent-domain/README.md',
    test: 'packages/parent-domain/tests/app-game-policy-preview-handoff.test.ts',
    fixture: 'packages/parent-domain/tests/app-game-policy-preview-handoff-fixtures.ts',
    harness: 'scripts/test/app-game-policy-preview-handoff-proof.mjs',
    evidence: 'test-results/app-game-policy-preview-handoff-proof/proof.json',
    appGameProofPack: 'output/app-game-plan-proof/70-policy-preview-handoff',
    appProofPack: 'output/app-plan-proof/70-policy-preview-handoff',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(testOutputDir, 'policy-preview-handoff-read-model.json'), readModel);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeProofPack(appGameProofDir, proof, 'app-game WP70');
await writeProofPack(appProofDir, proof, 'app WP70');

console.log('app-game-policy-preview-handoff-proof-ok');
console.log(`evidence=${join('test-results', 'app-game-policy-preview-handoff-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function previewOptions(refs) {
  return {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    handoffId: 'app-game-policy-preview-handoff-proof',
    generatedAt: timestamp,
    sourceContractRefs: [
      'app-game-policy-target-compiler',
      'policy-expectation-dry-run-preview',
      'enforcement-expectation-disabled-handoff',
    ],
  };
}

function childDevice(refs) {
  return {
    deviceId: 'device-windows-policy-preview',
    childProfileId: 'child-policy-preview',
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

function compileEvidence(compilerRules, refs, proofKind, evidenceReferenceId) {
  return {
    evidenceReference: evidenceRef(refs, evidenceReferenceId),
    proofKind,
    evidenceState: compilerRules.AppGamePolicyCompilerEvidenceState.Active,
    device: childDevice(refs),
    localUserRef: 'windows-local-user-policy-preview',
    observedAt: timestamp,
  };
}

function compileRequest(input) {
  const evidenceReference = evidenceRef(input.refs, input.evidenceReferenceId);
  return {
    schemaVersion: input.refs.ParentContractSchemaVersion.V0_6,
    compileRequestId: input.compileRequestId,
    policyVersion: 'app-game-policy-preview-version-proof',
    ruleId: input.ruleId,
    device: childDevice(input.refs),
    localUserRef: 'windows-local-user-policy-preview',
    target: {
      targetKind: input.targetKind,
      targetRef: input.targetRef,
    },
    requestedAction: input.requestedAction,
    policyAction: input.policyAction,
    scheduleRef: null,
    evidence: [
      compileEvidence(
        input.compilerRules,
        input.refs,
        input.compilerRules.AppGamePolicyCompilerProofKind.Identity,
        input.evidenceReferenceId
      ),
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
}

function compiledDecision(input) {
  const request = compileRequest(input);
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
      evidenceReferences: [evidenceRef(input.refs, input.evidenceReferenceId)],
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

function appPreviewDecision(compilerRules, policy, refs) {
  return compiledDecision({
    compilerRules,
    policy,
    refs,
    compiledDecisionId: 'compiled-preview-native-app',
    compileRequestId: 'compile-preview-native-app',
    policyDecisionId: 'policy-decision-preview-native-app',
    policyTargetId: 'policy-target-preview-native-app',
    targetKind: compilerRules.AppGamePolicyTargetKind.SpecificApp,
    targetRef: 'process:study-game-launcher.exe',
    requestedAction: compilerRules.AppGamePolicyCompilerRequestedAction.TimeLimit,
    policyAction: policy.PolicyAction.TimeLimit,
    capabilityState: compilerRules.AppGamePolicyCompilerCapabilityState.Supported,
    authorityState: compilerRules.AppGamePolicyCompilerAuthorityState.Proved,
    outcomeState: compilerRules.AppGamePolicyCompilerOutcomeState.DryRunReady,
    rejectionReason: compilerRules.AppGamePolicyCompilerRejectionReason.None,
    ruleId: 'rule-preview-native-app-time-limit',
    reasonCode: 'policy-preview-native-app-time-limit',
    evidenceReferenceId: 'evidence-preview-native-app',
    capabilityRef: 'capability-preview-native-app',
    authorityRef: 'authority-preview-native-app',
    auditRef: 'audit-preview-native-app',
  });
}

function gameManualPreviewDecision(compilerRules, policy, refs) {
  return compiledDecision({
    compilerRules,
    policy,
    refs,
    compiledDecisionId: 'compiled-preview-native-game-manual',
    compileRequestId: 'compile-preview-native-game-manual',
    policyDecisionId: 'policy-decision-preview-native-game-manual',
    policyTargetId: 'policy-target-preview-native-game-manual',
    targetKind: compilerRules.AppGamePolicyTargetKind.SpecificGame,
    targetRef: 'launcher-game:space-miner',
    requestedAction: compilerRules.AppGamePolicyCompilerRequestedAction.BlockLaunch,
    policyAction: policy.PolicyAction.Block,
    capabilityState: compilerRules.AppGamePolicyCompilerCapabilityState.ManualRequired,
    authorityState: compilerRules.AppGamePolicyCompilerAuthorityState.ManualRequired,
    outcomeState: compilerRules.AppGamePolicyCompilerOutcomeState.ManualRequired,
    rejectionReason: compilerRules.AppGamePolicyCompilerRejectionReason.BlockLaunchManualRequired,
    ruleId: 'rule-preview-native-game-block-launch',
    reasonCode: 'policy-preview-native-game-block-launch',
    evidenceReferenceId: 'evidence-preview-native-game',
    capabilityRef: 'capability-preview-native-game-manual',
    authorityRef: 'authority-preview-native-game-manual',
    auditRef: 'audit-preview-native-game-manual',
  });
}

function summarize(readModel) {
  return {
    rows: readModel.rows.length,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    previewReadyCount: readModel.previewReadyCount,
    manualRequiredCount: readModel.manualRequiredCount,
    rejectedCount: readModel.rejectedCount,
    statuses: countBy(readModel.rows.map((row) => row.previewStatus)),
    targetDomains: countBy(readModel.rows.map((row) => row.targetDomain)),
  };
}

function assertProof(proof) {
  if (
    proof.summary.rows !== 2 ||
    proof.summary.nativeAppRowCount !== 1 ||
    proof.summary.nativeGameRowCount !== 1 ||
    proof.summary.previewReadyCount !== 1 ||
    proof.summary.manualRequiredCount !== 1 ||
    proof.summary.rejectedCount !== 0
  ) {
    throw new Error(`Unexpected policy preview handoff summary: ${JSON.stringify(proof.summary)}`);
  }
  if (Object.values(proof.nonClaims).some((value) => value !== false)) {
    throw new Error(`Policy preview handoff overclaimed runtime behavior: ${JSON.stringify(proof.nonClaims)}`);
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
      '- Scope: app/game compiled policy decisions to read-only policy preview handoff rows.',
      '- Source inspected: parent-domain policy primitives, app/game policy target compiler, policy expectations, enforcement expectations, app/game feature doc, and implementation checklists.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-policy-preview-handoff app-game-policy-target-compiler: PASS',
      '- node import @ocentra-parent/parent-domain/app-game-policy-preview-handoff: PASS',
      '- Existing compiled app/game policy decisions become preview-ready or manual-required handoff rows.',
      '- Invalid rows claiming evaluator runtime, timer runtime, adapter dispatch, child delivery, or platform enforcement are rejected.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust protocol proof not applicable: this workpack adds a TypeScript parent-domain handoff boundary and does not add a Rust-crossing shape.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof.summary);
  await writeJson(join(proofDir, '04-journal-sqlite-proof.json'), {
    schemaVersion: 1,
    journalSqliteChanged: false,
    policyPreviewArtifact: 'test-results/app-game-policy-preview-handoff-proof/policy-preview-handoff-read-model.json',
  });
  await writeJson(join(proofDir, '05-policy-action-proof.json'), {
    schemaVersion: 1,
    previewReadyRows: proof.summary.previewReadyCount,
    manualRequiredRows: proof.summary.manualRequiredCount,
    policyEvaluatorRuntimeClaimed: false,
    timerRuntimeClaimed: false,
    adapterDispatchClaimed: false,
    childDeliveryClaimed: false,
    platformEnforcementClaimed: false,
  });
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo parent portal policy preview UI, child-facing UI, notification UI, or adapter UI source changed in this workpack.\n',
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
      '- The handoff accepts only compiled decisions that already parse through AppGamePolicyCompiledDecisionSchema.',
      '- Rows preserve dryRun=true and enforcementHandoffState=disabled.',
      '- Manual-required block-launch decisions remain manual-required and never dispatch an adapter.',
      '- Policy evaluator runtime, timers, child delivery, adapter dispatch, and platform enforcement remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNo live platform authority tier is raised. Policy preview rows do not execute adapters, block launches, start timers, or deliver child notifications.\n',
    'utf8'
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nThe preview handoff preserves authority refs from the compiled decision but does not upgrade manual-required or not-claimed platform authority.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-rollback-proof.md'),
    '# Rollback Proof\n\nNo runtime state, timer, block, suspend, shield, provider send, child delivery, or adapter dispatch is created. Rollback is limited to deleting generated proof artifacts.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, 'README.md'),
    `# ${label} Policy Preview Handoff Proof\n\nThis proof pack records app/game compiled policy decisions mapped into read-only preview handoff rows without policy evaluator runtime, timers, child delivery, adapter dispatch, or platform enforcement claims.\n`,
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
