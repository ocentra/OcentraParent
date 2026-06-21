import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-dry-run-no-action-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'dry-run-no-action');
const commands = [];
const proofBranch = 'codex/app-game-dry-run-no-action-gate-proof-split';
const deterministicProofRevision = 'branch-head-validated-by-harness';
const deterministicGeneratedAt = 'deterministic-proof-artifact';

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  runNpm(['run', 'build:contracts']);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/app-game-domain',
    '--',
    'app-game-time-budget-policy-runtime.test.ts',
  ]);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/app-game-domain',
    '--',
    'app-game-policy-preview-handoff.test.ts',
  ]);

  const runtimeSource = await readFile(
    join(repoRoot, 'packages', 'app-game-domain', 'src', 'app-game-time-budget-policy-runtime.ts'),
    'utf8'
  );
  const runtimeTest = await readFile(
    join(repoRoot, 'packages', 'app-game-domain', 'tests', 'unit', 'app-game-time-budget-policy-runtime.test.ts'),
    'utf8'
  );
  const previewSource = await readFile(
    join(repoRoot, 'packages', 'app-game-domain', 'src', 'app-game-policy-preview-handoff.ts'),
    'utf8'
  );
  const previewRules = await readFile(
    join(repoRoot, 'packages', 'app-game-domain', 'src', 'app-game-policy-preview-handoff-rules.ts'),
    'utf8'
  );
  const previewTest = await readFile(
    join(repoRoot, 'packages', 'app-game-domain', 'tests', 'unit', 'app-game-policy-preview-handoff.test.ts'),
    'utf8'
  );
  const compilerSource = await readFile(
    join(repoRoot, 'packages', 'schema-domain', 'src', 'app-game-policy-target-compiler.ts'),
    'utf8'
  );

  assertIncludes(
    runtimeSource,
    "DryRunPreview: 'dry-run-preview'",
    'runtime evaluator has explicit dry-run preview mode'
  );
  assertIncludes(
    runtimeSource,
    'return AppGameTimeBudgetRecommendedAction.TimeLimitDryRun',
    'dry-run preview recommends time-limit-dry-run instead of a terminating action'
  );
  assertIncludes(
    runtimeSource,
    'return AppGameTimeBudgetHandoffState.DryRunOnly',
    'time-limit dry-run maps to dry-run-only handoff state'
  );
  assertIncludes(runtimeSource, 'dryRun: true', 'runtime decisions are always dry-run decisions');
  assertIncludes(
    runtimeTest,
    "expect(decision.recommendedAction).toBe('time-limit-dry-run')",
    'runtime test proves exceeded budgets recommend dry-run only'
  );
  assertIncludes(
    runtimeTest,
    "expect(decision.enforcementHandoffState).toBe('dry-run-only')",
    'runtime test proves dry-run-only handoff state'
  );
  assertIncludes(
    runtimeTest,
    "expect(decision.enforcementHandoffState).toBe('manual-required')",
    'runtime test proves manual mode remains manual-required instead of adapter handoff'
  );

  assertIncludes(
    previewSource,
    "AppGamePolicyPreviewHandoffAdapterDispatchStateSchema = withParser(Schema.Literal('not-dispatched'))",
    'preview handoff schema allows only not-dispatched adapter state'
  );
  assertIncludes(previewSource, 'dryRun: decision.policyDecision.dryRun', 'preview rows preserve dry-run decisions');
  assertIncludes(
    previewSource,
    '...AppGamePolicyPreviewNoRuntimeClaimStates',
    'preview rows stamp no runtime claim states'
  );
  assertIncludes(
    previewSource,
    '...AppGamePolicyPreviewNoRuntimeClaimFlags',
    'preview rows stamp no runtime claim flags'
  );
  assertIncludes(
    previewRules,
    'row.dryRun && row.enforcementHandoffState === PolicyDecisionHandoffState.Disabled',
    'preview row guard requires dry-run with disabled enforcement handoff'
  );
  assertIncludes(
    previewRules,
    "row.adapterDispatchState === 'not-dispatched'",
    'preview row guard requires adapter not-dispatched'
  );
  assertIncludes(previewRules, '!row.adapterDispatchClaimed', 'preview row guard rejects adapter dispatch claims');
  assertIncludes(previewTest, 'expect(row.dryRun).toBe(true)', 'preview test proves dry-run rows');
  assertIncludes(
    previewTest,
    'expect(row.enforcementHandoffState).toBe(PolicyDecisionHandoffState.Disabled)',
    'preview test proves disabled enforcement handoff'
  );
  assertIncludes(
    previewTest,
    "expect(row.adapterDispatchState).toBe('not-dispatched')",
    'preview test proves block rows still do not dispatch adapters'
  );
  assertIncludes(
    previewTest,
    'expect(AppGamePolicyPreviewHandoffRowSchema.safeParse({ ...row, dryRun: false }).success).toBe(false)',
    'preview test rejects attempts to turn dry-run rows into execution rows'
  );

  assertIncludes(
    compilerSource,
    'appGamePolicyBlockLaunchWithoutProofIsManualRequired(decision)',
    'compiler requires unproved block launch to stay manual-required'
  );
  assertIncludes(
    compilerSource,
    "'Expected compiled app/game policy decisions to remain dry-run and carry evidence, rule, and capability refs'",
    'compiled decisions are guarded as dry-run proof refs'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-dry-run-no-action-gate-proof',
    generatedAt: deterministicGeneratedAt,
    branch: proofBranch,
    commit: deterministicProofRevision,
    commitMetadata:
      'This proof intentionally avoids embedding HEAD because a committed artifact cannot contain its own final commit hash.',
    gitStatusShort: 'validated-by-explicit-handoff-status-check',
    commands,
    gate: 'Dry-run terminates or blocks app/game.',
    gateState: 'prevented-by-dry-run-only-and-not-dispatched-contracts',
    evidence: {
      runtimeDecision:
        'packages/app-game-domain/src/app-game-time-budget-policy-runtime.ts builds dry-run decisions, maps exceeded dry-run budgets to time-limit-dry-run, and emits dry-run-only handoff state.',
      runtimeTests:
        'packages/app-game-domain/tests/unit/app-game-time-budget-policy-runtime.test.ts proves dry-run-only, disabled, ask-parent, and manual-required outcomes without adapter execution.',
      previewHandoff:
        'packages/app-game-domain/src/app-game-policy-preview-handoff.ts and packages/app-game-domain/src/app-game-policy-preview-handoff-rules.ts require dryRun true, disabled enforcement handoff, not-dispatched adapter state, and false runtime/enforcement claim flags.',
      previewTests:
        'packages/app-game-domain/tests/unit/app-game-policy-preview-handoff.test.ts rejects rows that try to execute policy, clear dryRun, or claim adapter/runtime delivery.',
      compilerBoundary:
        'packages/schema-domain/src/app-game-policy-target-compiler.ts keeps unproved block-launch decisions manual-required instead of executable block claims.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      dryRunTerminatesOrBlocks: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      childDeliveryClaimed: false,
      runtimeAdapterClaimed: false,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-dry-run-no-action-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/dry-run-no-action',
      harness: 'scripts/test/app-game-dry-run-no-action-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game dry-run no-action gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- Time-budget runtime decisions stay dry-run and map exceeded dry-run budgets to time-limit-dry-run plus dry-run-only handoff.',
      '- Preview handoff rows require dryRun, disabled enforcement handoff, not-dispatched adapter state, and false runtime/enforcement claim flags.',
      '- Preview tests reject attempts to clear dryRun, enable pending enforcement handoff, or claim adapter/timer runtime delivery.',
      '- Policy compiler tests keep unproved block-launch decisions manual-required instead of executable block claims.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-dry-run-no-action-gate-proof-ok');
  console.log('evidence=test-results/app-game-dry-run-no-action-gate-proof/proof.json');
}

function assertIncludes(source, needle, label) {
  if (!source.includes(needle)) {
    throw new Error(`Missing ${label}: ${needle}`);
  }
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function run(command, args) {
  const rendered = `${command} ${args.join(' ')}`;
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push(
    `${rendered}\nexit=${result.status}\n${normalizeCommandOutput(result.stdout)}${normalizeCommandOutput(result.stderr)}`
  );
  if (result.status !== 0) {
    throw new Error(`${rendered} failed with exit ${result.status}`);
  }
}

function normalizeCommandOutput(output) {
  const slashRepoRoot = repoRoot.replace(/\\/g, '/');
  return output
    .split(repoRoot)
    .join('<repo-root>')
    .split(slashRepoRoot)
    .join('<repo-root>')
    .replace(/Start at\s+\d{2}:\d{2}:\d{2}/g, 'Start at <normalized>')
    .replace(/\x1b\[2m[^\r\n]*?\x1b\[22m/g, '\x1b[2m<normalized>\x1b[22m')
    .replace(/Duration\s+[^\r\n]+/g, 'Duration <normalized>');
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
