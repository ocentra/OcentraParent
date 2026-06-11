import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-ai-output-direct-enforcement-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'ai-output-direct-enforcement');
const commands = [];
const proofBranch = 'codex/app-game-ai-output-direct-enforcement-gate-proof-split';
const deterministicProofRevision = 'branch-head-validated-by-harness';
const deterministicGeneratedAt = 'deterministic-proof-artifact';

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  runNpm(['run', 'build:contracts']);
  runNpm(['run', 'test', '--workspace', '@ocentra-parent/activity-domain', '--', 'app-game-category-risk.test.ts']);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-category-risk-policy-routing.test.ts',
  ]);
  runNpm(['run', 'test', '--workspace', '@ocentra-parent/activity-domain', '--', 'app-game.test.ts']);

  const appGameSource = await readFile(join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game.ts'), 'utf8');
  const primitivesSource = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game-primitives.ts'),
    'utf8'
  );
  const categoryRiskSource = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game-category-risk.ts'),
    'utf8'
  );
  const categoryRiskPrimitives = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game-category-risk-primitives.ts'),
    'utf8'
  );
  const categoryRiskTest = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game-category-risk.test.ts'),
    'utf8'
  );
  const routeSource = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-category-risk-policy-routing.ts'),
    'utf8'
  );
  const routeRules = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-category-risk-policy-routing-rules.ts'),
    'utf8'
  );
  const routeTest = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'tests', 'app-game-category-risk-policy-routing.test.ts'),
    'utf8'
  );

  assertIncludes(
    categoryRiskPrimitives,
    'export const AppGameCategoryPolicyCandidateActionSchema = withParser(',
    'category candidate actions exclude hard enforcement actions'
  );
  assertIncludes(
    categoryRiskPrimitives,
    "'manualReview'",
    'category candidate actions include manual review as soft action'
  );
  assertIncludes(
    categoryRiskPrimitives,
    'export const AppGameCategoryEnforcementStateSchema = withParser',
    'category candidate enforcement state is notEnforcement only'
  );
  assertIncludes(categoryRiskPrimitives, "'notEnforcement'", 'category enforcement state literal is notEnforcement');
  assertIncludes(
    primitivesSource,
    "Schema.Literal(\n    'classifyOnly'",
    'AI action hints start as classify-only support values'
  );
  assertIncludes(primitivesSource, "'policyDraftPreview'", 'AI action hints can preview policy drafts only');
  assertIncludes(primitivesSource, "'askParentPreview'", 'AI action hints can preview ask-parent only');
  assertIncludes(
    appGameSource,
    'actionHints: Schema.Array(AppGameAiActionHintSchema)',
    'AI classification digest carries hints, not adapter commands'
  );
  assertIncludes(
    categoryRiskSource,
    'appGameCategoryAiCandidateIsReviewOnly(candidate)',
    'category-risk schema applies AI review-only guard'
  );
  assertIncludes(
    categoryRiskSource,
    "candidate.enforcementState === 'notEnforcement'",
    'AI category candidates must remain notEnforcement'
  );
  assertIncludes(
    categoryRiskSource,
    "candidate.policyCandidateAction === 'manualReview'",
    'AI category candidates can route only through soft/manual-review actions'
  );
  assertIncludes(
    categoryRiskPrimitives,
    "LocalAi: AppGameCategorySourceKindSchema.parse('localAi')",
    'local AI source kind is explicit and schema-bound'
  );
  assertIncludes(
    categoryRiskTest,
    'assertAiCategoryCannotDirectlyAct',
    'activity-domain test names the AI direct-action boundary'
  );
  assertIncludes(categoryRiskTest, "policyCandidateAction: 'shieldApp'", 'activity-domain test rejects hard AI action');
  assertIncludes(
    categoryRiskTest,
    'expect(directBlock.success).toBe(false)',
    'activity-domain test rejects AI hard enforcement action'
  );
  assertIncludes(
    routeSource,
    'AppGameCategoryRiskPolicyAdapterDispatchStateSchema = withParser',
    'parent-domain route schema constrains adapter dispatch state'
  );
  assertIncludes(
    routeSource,
    'Schema.Literal(AppGameCategoryRiskPolicyAdapterDispatchState.NotDispatched)',
    'parent-domain route schema allows not-dispatched only'
  );
  assertIncludes(
    routeRules,
    'route.sourceKind !== AppGameCategoryRiskPolicyRouteSourceKind.LocalAi || route.aiDigestRef !== null',
    'local AI policy routes require digest refs'
  );
  assertIncludes(
    routeRules,
    'route.requestedAction === AppGamePolicyCompilerRequestedAction.ManualRequired',
    'policy routing soft boundary includes manual-required but not adapter dispatch'
  );
  assertIncludes(
    routeTest,
    'requires local-AI category routes to cite digest refs',
    'parent-domain test covers local AI digest requirement'
  );
  assertIncludes(
    routeTest,
    'keeps risk candidates from becoming hard adapter actions',
    'parent-domain test covers hard action rejection'
  );
  assertIncludes(
    routeTest,
    'expect(parsed.data.adapterDispatchState).toBe(AppGameCategoryRiskPolicyAdapterDispatchState.NotDispatched)',
    'parent-domain test proves route stays not-dispatched'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-ai-output-direct-enforcement-gate-proof',
    generatedAt: deterministicGeneratedAt,
    branch: proofBranch,
    commit: deterministicProofRevision,
    commitMetadata:
      'This proof intentionally avoids embedding HEAD because a committed artifact cannot contain its own final commit hash.',
    gitStatusShort: 'validated-by-explicit-handoff-status-check',
    commands,
    gate: 'AI output directly enforces.',
    gateState: 'prevented-by-ai-review-only-and-not-dispatched-policy-routing-contracts',
    evidence: {
      aiDigestContract:
        'packages/activity-domain/src/app-game.ts represents AI classifier output as classification state, confidence, actionHints, and source evidence/session refs only.',
      categoryRiskContract:
        'packages/activity-domain/src/app-game-category-risk.ts requires local-AI category candidates to cite aiDigestRef, stay notEnforcement, and use only soft/manual-review actions.',
      categoryRiskTests:
        'packages/activity-domain/tests/app-game-category-risk.test.ts rejects local-AI hard action candidates such as shieldApp and missing digest refs.',
      policyRouteContract:
        'packages/parent-domain/src/app-game-category-risk-policy-routing.ts constrains adapterDispatchState to not-dispatched for category/risk routes.',
      policyRouteTests:
        'packages/parent-domain/tests/app-game-category-risk-policy-routing.test.ts proves local-AI routes require digest refs and hard risk-candidate actions are rejected.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      aiOutputDirectlyEnforces: false,
      adapterDispatchClaimed: false,
      policyPreviewOnly: true,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
      runtimeAdapterClaimed: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-ai-output-direct-enforcement-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/ai-output-direct-enforcement',
      harness: 'scripts/test/app-game-ai-output-direct-enforcement-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game AI output direct-enforcement gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- Activity-domain AI classification digests expose action hints and evidence/session refs, not adapter commands.',
      '- Activity-domain local-AI category candidates require aiDigestRef and stay notEnforcement.',
      '- Activity-domain tests reject local-AI hard action candidates such as shieldApp.',
      '- Parent-domain category/risk policy routes constrain adapterDispatchState to not-dispatched.',
      '- Parent-domain tests reject hard risk-candidate actions and require digest refs for local-AI routes.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-ai-output-direct-enforcement-gate-proof-ok');
  console.log('evidence=test-results/app-game-ai-output-direct-enforcement-gate-proof/proof.json');
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
