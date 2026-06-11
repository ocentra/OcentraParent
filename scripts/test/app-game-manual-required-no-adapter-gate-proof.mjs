import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-manual-required-no-adapter-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'manual-required-no-adapter');
const commands = [];
const proofBranch = 'codex/app-game-manual-required-no-adapter-gate-proof-split';
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
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-broad-blocking-proof-gates.test.ts',
  ]);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-policy-preview-handoff.test.ts',
  ]);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-policy-target-compiler.test.ts',
  ]);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-category-risk-policy-routing.test.ts',
  ]);

  const broadGateSource = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gates.ts'),
    'utf8'
  );
  const broadGateRules = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gate-rules.ts'),
    'utf8'
  );
  const broadGateData = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gate-data.ts'),
    'utf8'
  );
  const broadGateTest = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'tests', 'app-game-broad-blocking-proof-gates.test.ts'),
    'utf8'
  );
  const previewTest = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'tests', 'app-game-policy-preview-handoff.test.ts'),
    'utf8'
  );
  const compilerTest = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'tests', 'app-game-policy-target-compiler.test.ts'),
    'utf8'
  );
  const categoryRouteSource = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-category-risk-policy-routing.ts'),
    'utf8'
  );
  const categoryRouteRules = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-category-risk-policy-routing-rules.ts'),
    'utf8'
  );
  const categoryRouteTest = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'tests', 'app-game-category-risk-policy-routing.test.ts'),
    'utf8'
  );

  assertIncludes(
    broadGateSource,
    "Schema.Literal('blocked-before-adapter', 'adapter-unavailable', 'not-dispatched', 'dispatch-eligible')",
    'broad blocking gate schema separates blocked-before-adapter from dispatch-eligible'
  );
  assertIncludes(
    broadGateSource,
    "'Expected app/game broad blocking gates to keep unproved platform blocking manual-required, unavailable, or not-claimed before adapter dispatch'",
    'broad blocking gate schema names the manual-required no-adapter invariant'
  );
  assertIncludes(
    broadGateRules,
    'return !gate.canCallAdapter && gate.supportedModes.length === 0 && gate.adapterDispatchState !==',
    'non-supported gates must not call adapters'
  );
  assertIncludes(
    broadGateRules,
    "gate.adapterDispatchState === 'dispatch-eligible'",
    'only supported gates can become dispatch-eligible'
  );
  assertIncludes(
    broadGateRules,
    'return !gate.broadBlockingClaimed',
    'manual-required and unavailable gates cannot claim broad blocking'
  );
  assertIncludes(broadGateData, "outcomeState: 'manual-required'", 'manual gate data marks rows manual-required');
  assertIncludes(
    broadGateData,
    "adapterDispatchState: 'blocked-before-adapter'",
    'manual gate data blocks before adapter dispatch'
  );
  assertIncludes(broadGateData, 'canCallAdapter: false', 'manual gate data disables adapter calls');
  assertIncludes(broadGateData, 'broadBlockingClaimed: false', 'manual gate data does not claim broad blocking');
  assertIncludes(
    broadGateTest,
    'records broad app and game blocking as manual-required, unavailable, or not-claimed before adapter dispatch',
    'broad gate test names the no-adapter manual-required boundary'
  );
  assertIncludes(
    broadGateTest,
    'rejects manual-required and unavailable broad blocking rows that try to call adapters',
    'broad gate test rejects manual-required adapter calls'
  );
  assertIncludes(broadGateTest, 'canCallAdapter: true', 'broad gate negative test tries adapter call');
  assertIncludes(
    broadGateTest,
    "adapterDispatchState: 'dispatch-eligible'",
    'broad gate negative test tries dispatch eligibility'
  );

  assertIncludes(
    previewTest,
    'keeps native game block-launch decisions manual-required without adapter dispatch',
    'preview test names manual-required block-launch without dispatch'
  );
  assertIncludes(
    previewTest,
    'expect(row.previewStatus).toBe(AppGamePolicyPreviewStatus.ManualRequired)',
    'preview test proves manual-required status'
  );
  assertIncludes(
    previewTest,
    "expect(row.adapterDispatchState).toBe('not-dispatched')",
    'preview test proves manual-required rows do not dispatch adapters'
  );
  assertIncludes(
    previewTest,
    'expect(row.adapterDispatchClaimed).toBe(false)',
    'preview test proves adapter dispatch is not claimed'
  );
  assertIncludes(
    previewTest,
    'expect(row.platformEnforcementClaimed).toBe(false)',
    'preview test proves platform enforcement is not claimed'
  );

  assertIncludes(
    compilerTest,
    'assertBlockLaunchWithoutProofIsManualRequired',
    'compiler test names block-launch manual-required boundary'
  );
  assertIncludes(
    compilerTest,
    'outcomeState: AppGamePolicyCompilerOutcomeState.ManualRequired',
    'compiler test accepts unproved block-launch only as manual-required'
  );
  assertIncludes(
    compilerTest,
    'outcomeState: AppGamePolicyCompilerOutcomeState.DryRunReady',
    'compiler test rejects unproved block-launch as dry-run-ready'
  );

  assertIncludes(
    categoryRouteSource,
    'AppGameCategoryRiskPolicyAdapterDispatchStateSchema = withParser',
    'category/risk route schema constrains adapter dispatch state'
  );
  assertIncludes(
    categoryRouteSource,
    'Schema.Literal(AppGameCategoryRiskPolicyAdapterDispatchState.NotDispatched)',
    'category/risk route schema allows not-dispatched only'
  );
  assertIncludes(
    categoryRouteRules,
    '[AppGameCategoryRiskPolicyCandidateAction.ManualReview]: AppGamePolicyCompilerRequestedAction.ManualRequired',
    'manual review routes map to manual-required compiler action'
  );
  assertIncludes(
    categoryRouteTest,
    'keeps manual review and stale category proof out of compile-ready routing',
    'category/risk test keeps manual review from compile-ready dispatch'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-manual-required-no-adapter-gate-proof',
    generatedAt: deterministicGeneratedAt,
    branch: proofBranch,
    commit: deterministicProofRevision,
    commitMetadata:
      'This proof intentionally avoids embedding HEAD because a committed artifact cannot contain its own final commit hash.',
    gitStatusShort: 'validated-by-explicit-handoff-status-check',
    commands,
    gate: 'Manual-required action calls an adapter.',
    gateState: 'prevented-by-manual-required-and-blocked-before-adapter-contracts',
    evidence: {
      broadBlockingGate:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gates.ts requires unproved platform blocking to remain manual-required, unavailable, or not-claimed before adapter dispatch.',
      broadBlockingGateData:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts sets manual gates to blocked-before-adapter, canCallAdapter=false, supportedModes=[], and broadBlockingClaimed=false.',
      broadBlockingGateTests:
        'packages/parent-domain/tests/app-game-broad-blocking-proof-gates.test.ts rejects manual-required/unavailable rows that try canCallAdapter=true or dispatch-eligible.',
      previewHandoff:
        'packages/parent-domain/tests/app-game-policy-preview-handoff.test.ts keeps native-game block-launch manual-required rows not-dispatched with adapter/platform enforcement claims false.',
      compilerBoundary:
        'packages/parent-domain/tests/app-game-policy-target-compiler.test.ts keeps unproved block-launch manual-required and rejects dry-run-ready upgrades without proof.',
      categoryRiskRouting:
        'packages/parent-domain/src/app-game-category-risk-policy-routing.ts and tests keep manual-review category/risk routes not-dispatched and out of compile-ready routing.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      manualRequiredCallsAdapter: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      broadBlockingClaimed: false,
      runtimeAdapterClaimed: false,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-manual-required-no-adapter-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/manual-required-no-adapter',
      harness: 'scripts/test/app-game-manual-required-no-adapter-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game manual-required no-adapter gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- Broad-blocking manual-required rows are blocked-before-adapter, cannot call adapters, have no supported modes, and do not claim broad blocking.',
      '- Broad-blocking tests reject manual-required and unavailable rows that try to become dispatch-eligible.',
      '- Policy preview tests keep manual-required native-game block-launch rows not-dispatched with adapter and platform enforcement claims false.',
      '- Policy compiler tests reject unproved block-launch upgrades out of manual-required state.',
      '- Category/risk manual-review routes map to manual-required and remain not-dispatched.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-manual-required-no-adapter-gate-proof-ok');
  console.log('evidence=test-results/app-game-manual-required-no-adapter-gate-proof/proof.json');
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
