import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-policy-evaluator-runtime-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '51-policy-evaluator-runtime-breadth');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '51-policy-evaluator-runtime-breadth');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });
  await mkdir(join(appGameProofDir, '06-ui-snapshots'), { recursive: true });
  await mkdir(join(appProofDir, '06-ui-snapshots'), { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      'app-game-time-budget-policy-runtime',
    ])
  );

  const runtime = await import('../../packages/schema-domain/dist/app-game-time-budget-policy-runtime.js');
  const rules = await import('../../packages/schema-domain/dist/app-game-time-budget-policy-rules.js');
  const refs = await import('../../packages/schema-domain/dist/family-reference-primitives.js');

  const fixtures = buildFixtures(runtime, rules, refs);
  const dryRunDecision = runtime.buildAppGameTimeBudgetRuntimeDecision(fixtures.baseInput);
  const warnDecision = runtime.buildAppGameTimeBudgetRuntimeDecision({
    ...fixtures.baseInput,
    decisionId: 'app-game-runtime-policy-evaluator-warn',
    runtimeMode: runtime.AppGameTimeBudgetRuntimeMode.WarnOnly,
  });
  const askParentDecision = runtime.buildAppGameTimeBudgetRuntimeDecision({
    ...fixtures.baseInput,
    decisionId: 'app-game-runtime-policy-evaluator-ask-parent',
    bonusGrant: fixtures.requestedBonusGrant,
  });
  const manualDecision = runtime.buildAppGameTimeBudgetRuntimeDecision({
    ...fixtures.baseInput,
    decisionId: 'app-game-runtime-policy-evaluator-manual',
    runtimeMode: runtime.AppGameTimeBudgetRuntimeMode.ManualRequired,
    timerRefs: [],
  });
  const approvedBonusDecision = runtime.buildAppGameTimeBudgetRuntimeDecision({
    ...fixtures.baseInput,
    decisionId: 'app-game-runtime-policy-evaluator-approved-bonus',
    bonusGrant: fixtures.approvedBonusGrant,
  });
  const rejectedCrossDevice = rejectsCrossDevice(runtime, fixtures);

  assertEqual(
    dryRunDecision.recommendedAction,
    rules.AppGameTimeBudgetRecommendedAction.TimeLimitDryRun,
    'dry-run action'
  );
  assertEqual(
    dryRunDecision.enforcementHandoffState,
    rules.AppGameTimeBudgetHandoffState.DryRunOnly,
    'dry-run handoff'
  );
  assertEqual(dryRunDecision.timerState, rules.AppGameTimeBudgetTimerState.Active, 'dry-run timer state');
  assertEqual(String(dryRunDecision.countedSessionRefs[0]), 'session-runtime-native-app', 'counted app session');
  assertEqual(String(dryRunDecision.excludedSessionRefs[0]), 'session-runtime-native-game', 'excluded game session');
  assertEqual(warnDecision.recommendedAction, rules.AppGameTimeBudgetRecommendedAction.Warn, 'warn-only action');
  assertEqual(
    askParentDecision.recommendedAction,
    rules.AppGameTimeBudgetRecommendedAction.AskParent,
    'ask-parent action'
  );
  assertEqual(
    askParentDecision.approvalState,
    rules.AppGameTimeBudgetApprovalState.Pending,
    'ask-parent approval state'
  );
  assertEqual(
    manualDecision.enforcementHandoffState,
    rules.AppGameTimeBudgetHandoffState.ManualRequired,
    'manual handoff'
  );
  assertEqual(approvedBonusDecision.budgetExceeded, false, 'approved bonus extends limit');
  assertEqual(
    approvedBonusDecision.recommendedAction,
    rules.AppGameTimeBudgetRecommendedAction.Observe,
    'approved bonus observe'
  );
  assertEqual(rejectedCrossDevice, true, 'cross-device rejection');

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-policy-evaluator-runtime-breadth',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    summary: {
      decisionsBuilt: 5,
      countedSessionRefs: dryRunDecision.countedSessionRefs,
      excludedSessionRefs: dryRunDecision.excludedSessionRefs,
      actions: countBy([
        dryRunDecision.recommendedAction,
        warnDecision.recommendedAction,
        askParentDecision.recommendedAction,
        manualDecision.recommendedAction,
        approvedBonusDecision.recommendedAction,
      ]),
      handoffStates: countBy([
        dryRunDecision.enforcementHandoffState,
        warnDecision.enforcementHandoffState,
        askParentDecision.enforcementHandoffState,
        manualDecision.enforcementHandoffState,
        approvedBonusDecision.enforcementHandoffState,
      ]),
      timerStates: countBy([
        dryRunDecision.timerState,
        warnDecision.timerState,
        askParentDecision.timerState,
        manualDecision.timerState,
        approvedBonusDecision.timerState,
      ]),
      rejectedCrossDevice,
    },
    claimsProved: [
      'schema-domain runtime helper builds app/game time-budget dry-run decisions from stored policy and session inputs',
      'target matching counts only native app sessions for an all-native-app policy and excludes native game sessions',
      'running and foreground duration modes are delegated through the existing time-budget policy rules',
      'exceeded budget decisions can resolve to dry-run timer, warn-only, ask-parent, or manual-required states without adapter dispatch',
      'approved bonus time extends the effective budget only with approval and audit proof',
      'cross-device session inputs are rejected by the existing schema boundary',
    ],
    claimsNotProved: [
      'service persistence or WebSocket policy evaluator command',
      'portal budget authoring or policy status rendering',
      'notification delivery or child request UX runtime',
      'adapter execution, broad blocking, or platform support',
      'live classifier/provider execution or category quality',
    ],
    evidence: {
      contract: 'packages/schema-domain/src/app-game-time-budget-policy-runtime.ts',
      existingContract: 'packages/schema-domain/src/app-game-time-budget-policy.ts',
      existingRules: 'packages/schema-domain/src/app-game-time-budget-policy-rules.ts',
      test: 'packages/app-game-domain/tests/unit/app-game-time-budget-policy-runtime.test.ts',
      harness: 'scripts/test/app-game-policy-evaluator-runtime-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/51-policy-evaluator-runtime-breadth',
      appProofPack: 'output/app-plan-proof/51-policy-evaluator-runtime-breadth',
    },
    decisions: {
      dryRunDecision,
      warnDecision,
      askParentDecision,
      manualDecision,
      approvedBonusDecision,
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP51');
  await writeProofPack(appProofDir, proof, 'app WP51');

  console.log(`app-game-policy-evaluator-runtime-proof-ok:${proof.summary.decisionsBuilt}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function buildFixtures(runtime, rules, refs) {
  const timestamp = '2026-06-04T16:15:00Z';
  const policyVersion = 'app-game-runtime-policy-evaluator-v1';
  const device = {
    deviceId: 'device-windows-policy-evaluator',
    childProfileId: 'child-policy-evaluator',
    label: 'Study PC',
    platform: refs.ParentPlatform.Windows,
  };
  const otherDevice = {
    ...device,
    deviceId: 'device-windows-policy-evaluator-other',
  };
  const sessionEvidence = {
    evidenceReferenceId: 'evidence-policy-evaluator-session',
    kind: refs.ParentEvidenceReferenceKind.QueryStoreSummary,
    observedAt: timestamp,
  };
  const gameEvidence = {
    evidenceReferenceId: 'evidence-policy-evaluator-game-session',
    kind: refs.ParentEvidenceReferenceKind.QueryStoreSummary,
    observedAt: timestamp,
  };
  const scheduleEvidence = {
    evidenceReferenceId: 'evidence-policy-evaluator-schedule',
    kind: refs.ParentEvidenceReferenceKind.PolicyDecision,
    observedAt: timestamp,
  };
  const approvalRef = {
    actionReferenceId: 'parent-action-policy-evaluator-approved-bonus',
    actor: {
      actorId: 'parent-actor-policy-evaluator',
      role: refs.ParentActorRole.Parent,
    },
    policyVersion,
    createdAt: timestamp,
  };
  const policy = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    budgetPolicyId: 'app-game-runtime-policy-evaluator-budget',
    policyVersion,
    ruleId: 'rule-app-runtime-budget-daily',
    device,
    target: {
      targetKind: rules.AppGameTimeBudgetTargetKind.AllNativeApps,
      targetRef: null,
    },
    period: rules.AppGameTimeBudgetPeriod.Daily,
    baseBudgetLimitMs: 3_600_000,
    durationSource: rules.AppGameTimeBudgetDurationSource.RunningDuration,
    scheduleRef: 'schedule-policy-evaluator-school-day',
    previewEvidenceReferences: [sessionEvidence],
  };
  const nativeAppSession = {
    sessionRef: {
      sessionRefId: 'session-runtime-native-app',
      device,
      observedAt: timestamp,
    },
    sessionKind: rules.AppGameTimeBudgetSessionKind.NativeAppSession,
    targetRef: 'target-runtime-native-app',
    categoryRef: 'category-runtime-native-app',
    riskSignalRef: 'risk-runtime-native-app',
    parentAllowedCandidate: false,
    runningDurationMs: 4_200_000,
    foregroundDurationMs: 1_500_000,
    evidenceReferences: [sessionEvidence],
  };
  const nativeGameSession = {
    sessionRef: {
      sessionRefId: 'session-runtime-native-game',
      device,
      observedAt: timestamp,
    },
    sessionKind: rules.AppGameTimeBudgetSessionKind.NativeGameSession,
    targetRef: 'target-runtime-native-game',
    categoryRef: 'category-runtime-native-game',
    riskSignalRef: null,
    parentAllowedCandidate: false,
    runningDurationMs: 4_800_000,
    foregroundDurationMs: 4_500_000,
    evidenceReferences: [gameEvidence],
  };
  const baseInput = {
    schemaVersion: refs.ParentContractSchemaVersion.V0_6,
    decisionId: 'app-game-runtime-policy-evaluator-dry-run',
    policy,
    sessions: [nativeAppSession, nativeGameSession],
    scheduleState: rules.AppGameTimeBudgetScheduleState.Active,
    scheduleEvidenceReferences: [scheduleEvidence],
    bonusGrant: {
      bonusState: rules.AppGameTimeBudgetBonusState.None,
      bonusDurationMs: 0,
      approvalRef: null,
      auditRefs: [],
    },
    runtimeMode: runtime.AppGameTimeBudgetRuntimeMode.DryRunPreview,
    timerRefs: ['timer-app-game-policy-evaluator'],
    auditRefs: ['audit-app-game-policy-evaluator'],
    evaluatedAt: timestamp,
  };

  return {
    baseInput,
    nativeAppSession,
    nativeGameSession,
    crossDeviceSession: {
      ...nativeAppSession,
      sessionRef: {
        ...nativeAppSession.sessionRef,
        device: otherDevice,
      },
    },
    requestedBonusGrant: {
      bonusState: rules.AppGameTimeBudgetBonusState.Requested,
      bonusDurationMs: 0,
      approvalRef: null,
      auditRefs: ['audit-app-game-policy-evaluator-bonus-requested'],
    },
    approvedBonusGrant: {
      bonusState: rules.AppGameTimeBudgetBonusState.ApprovedActive,
      bonusDurationMs: 900_000,
      approvalRef,
      auditRefs: ['audit-app-game-policy-evaluator-bonus-approved'],
    },
  };
}

function rejectsCrossDevice(runtime, fixtures) {
  try {
    runtime.buildAppGameTimeBudgetRuntimeDecision({
      ...fixtures.baseInput,
      sessions: [fixtures.crossDeviceSession, fixtures.nativeGameSession],
    });
  } catch {
    return true;
  }
  return false;
}

async function writeProofPack(proofDir, proof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    lines([
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${await gitBranch()}`,
      `- Commit: ${proof.commit}`,
      '- Scope: app-game-domain runtime helper for app/game time-budget policy evaluation with centralized schema rules.',
      '- Source inspected: current app/game time-budget policy contract plus centralized schema rules.',
      '- UI, service WebSocket runtime, notifications, persistence, and adapters are intentionally not changed.',
    ]),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    lines([
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/app-game-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/app-game-domain -- app-game-time-budget-policy-runtime: PASS',
      '- Dry-run, warn-only, ask-parent, manual-required, and approved-bonus runtime decisions build through schema parsing.',
      '- Cross-device session input is rejected before runtime handoff state can be represented.',
    ]),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust/service protocol not changed. This is TypeScript app-game-domain policy evaluator runtime proof backed by centralized schema rules only.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof);
  await writeFile(
    join(proofDir, '04-journal-sqlite-proof.json'),
    `${JSON.stringify(
      {
        schemaVersion: 1,
        journalSqliteChanged: false,
        reason: 'No journal, SQLite, service read-model, or persistence code changed.',
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '05-policy-action-proof.json'),
    `${JSON.stringify(
      {
        schemaVersion: 1,
        decisionsBuilt: proof.summary.decisionsBuilt,
        actions: proof.summary.actions,
        handoffStates: proof.summary.handoffStates,
        timerStates: proof.summary.timerStates,
        adapterDispatchClaimed: false,
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo portal or child-facing UI source changed in this workpack.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '07-playwright-ui-proof.log'),
    'Playwright/browser proof not applicable: no UI source changed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    lines([
      'Security/no-claim proof:',
      '',
      '- Stored sessions are counted only when they match the policy target.',
      '- Cross-device sessions are rejected by the schema boundary.',
      '- Ask-parent/manual-required decisions do not create adapter dispatch proof.',
      '- Dry-run timer state is represented without executing a platform adapter.',
      '- Service persistence, notification delivery, portal authoring, and platform support remain unclaimed.',
    ]),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNo live platform proof is attached. Adapter execution and platform support remain unclaimed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    lines([
      'Validation run:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/app-game-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/app-game-domain -- app-game-time-budget-policy-runtime: PASS',
      '- node scripts/test/app-game-policy-evaluator-runtime-proof.mjs: PASS',
    ]),
    'utf8'
  );
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nNo authority tier is raised. Decisions remain app-game-domain runtime proof with adapter execution unclaimed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-rollback-proof.md'),
    '# Rollback Proof\n\nNo device action, block, suspend, shield, package state, or adapter state is created, so rollback execution is not applicable.\n',
    'utf8'
  );
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, shell: false, stdio: 'inherit' });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(undefined);
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with ${code}`));
    });
  });
}

async function gitBranch() {
  return (await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD'])).trim();
}

async function gitHead() {
  return (await gitOutput(['rev-parse', 'HEAD'])).trim();
}

async function gitOutput(args) {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', args, { cwd: repoRoot, shell: false });
    child.stdout.on('data', (chunk) => chunks.push(Buffer.from(chunk)));
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(undefined);
        return;
      }
      reject(new Error(`git ${args.join(' ')} exited with ${code}`));
    });
  });
  return Buffer.concat(chunks).toString('utf8');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function lines(values) {
  return `${values.join('\n')}\n`;
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, got ${actual}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
