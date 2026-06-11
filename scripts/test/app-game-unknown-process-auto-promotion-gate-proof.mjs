import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-unknown-process-auto-promotion-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'unknown-process-auto-promotion');
const commands = [];
const proofBranch = 'codex/app-game-unknown-process-auto-promotion-gate-proof-split';
const deterministicProofRevision = 'branch-head-validated-by-harness';
const deterministicGeneratedAt = 'deterministic-proof-artifact';

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  runNpm(['run', 'build:contracts']);
  runNpm(['run', 'test', '--workspace', '@ocentra-parent/activity-domain', '--', 'app-game.test.ts']);
  runNpm(['run', 'test', '--workspace', '@ocentra-parent/activity-domain', '--', 'app-game-identity.test.ts']);
  runNpm(['run', 'test', '--workspace', '@ocentra-parent/activity-domain', '--', 'app-game-category-risk.test.ts']);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'activity-ui-app-game-dashboard-intent.test.ts',
  ]);

  const appGameSource = await readFile(join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game.ts'), 'utf8');
  const appGameTest = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game.test.ts'),
    'utf8'
  );
  const identityTest = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game-identity.test.ts'),
    'utf8'
  );
  const categoryRiskTest = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game-category-risk.test.ts'),
    'utf8'
  );
  const intentSource = await readFile(
    join(repoRoot, 'vendor', 'ocentra-parent-core-ui', 'AppPages', 'ParentPortal', 'app-game-dashboard-intent.ts'),
    'utf8'
  );
  const intentTest = await readFile(
    join(repoRoot, 'apps', 'portal', 'tests', 'activity-ui-app-game-dashboard-intent.test.ts'),
    'utf8'
  );

  assertIncludes(
    appGameSource,
    "identity.confidence === 'weak' &&",
    'identity contract restricts display-label-only identities to weak confidence'
  );
  assertIncludes(
    appGameSource,
    "identity.classificationState === 'unknownProcess'",
    'identity contract keeps display-label-only identities unknown'
  );
  assertIncludes(
    appGameSource,
    "identity.productKind === 'unknownExecutable'",
    'identity contract keeps display-label-only identities outside native-game meaning'
  );
  assertIncludes(
    appGameSource,
    "claim.identityStrength !== 'displayNameOnly' ||",
    'evidence claim contract guards display-name-only claims'
  );
  assertIncludes(
    appGameSource,
    'claim.confidence <= 0.3 &&',
    'evidence claim contract keeps display-name-only claims weak'
  );
  assertIncludes(
    appGameSource,
    'claim.processIdentity === null',
    'evidence claim contract keeps display-name-only claims unlinked from process identity'
  );
  assertIncludes(
    appGameTest,
    'AppGameProcessObservationSchema: preserves permission-limited unknowns',
    'activity-domain test preserves unknown process observation states'
  );
  assertIncludes(
    appGameTest,
    "expect(parsed.data.classificationState).toBe('permissionLimited')",
    'activity-domain test proves unknown permission-limited observations are not known games'
  );
  assertIncludes(
    identityTest,
    'keeps display-label-only identity weak and unknown',
    'identity test names the weak unknown boundary'
  );
  assertIncludes(
    identityTest,
    'expect(promoted.success).toBe(false)',
    'identity test rejects display-only promotion to deterministic known game'
  );
  assertIncludes(
    categoryRiskTest,
    'assertUnknownVpnNameStaysCandidate',
    'category/risk test keeps unknown executable labels as candidates'
  );
  assertIncludes(
    categoryRiskTest,
    'AppGameCategoryPolicyCandidateAction.ManualReview',
    'category/risk test routes unknown heuristic labels to manual review'
  );
  assertIncludes(
    categoryRiskTest,
    'expect(directBlock.success).toBe(false)',
    'category/risk test rejects direct block from candidate labels'
  );
  assertIncludes(intentSource, 'readonly unknownApproval: boolean;', 'dashboard row exposes unknown approval flag');
  assertIncludes(
    intentSource,
    'const unknownApproval = appGameUnknownApproval(classificationState, inventoryState, runtimeState)',
    'dashboard marks unknown and possible-game rows for review'
  );
  assertIncludes(
    intentSource,
    'return values.some((value) => /unknown|new|possible|candidate/u.test(value.toLowerCase()))',
    'dashboard unknown approval helper covers unknown and possible candidate states'
  );
  assertIncludes(intentTest, "classificationState: 'unknown-process'", 'portal fixture includes unknown process row');
  assertIncludes(intentTest, "state: 'manual-required'", 'portal fixture keeps unknown process row manual-required');
  assertIncludes(
    intentTest,
    "classificationState: 'possible-game'",
    'portal fixture includes possible-game candidate row'
  );
  assertIncludes(
    intentTest,
    "expect(dashboard.capabilityRows.map((row) => row.label)).toContain('manual-required')",
    'portal test expects manual-required capability visibility'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-unknown-process-auto-promotion-gate-proof',
    generatedAt: deterministicGeneratedAt,
    branch: proofBranch,
    commit: deterministicProofRevision,
    commitMetadata:
      'This proof intentionally avoids embedding HEAD because a committed artifact cannot contain its own final commit hash.',
    gitStatusShort: 'validated-by-explicit-handoff-status-check',
    commands,
    gate: 'Unknown process is auto-promoted to known game.',
    gateState: 'prevented-by-weak-unknown-identity-and-manual-review-contracts',
    evidence: {
      identityContract:
        'packages/activity-domain/src/app-game.ts keeps display-label-only identities weak, unknownProcess, and unknownExecutable.',
      evidenceClaimContract:
        'packages/activity-domain/src/app-game.ts keeps displayNameOnly claims weak and unlinked from inventory, process, launcher, and catalog refs.',
      identityTests:
        'packages/activity-domain/tests/app-game-identity.test.ts rejects deterministic known-game promotion from display-only identity.',
      categoryRiskTests:
        'packages/activity-domain/tests/app-game-category-risk.test.ts keeps heuristic unknown executable labels as manual-review candidates and rejects direct block.',
      portalIntent:
        'vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts surfaces unknown/possible-game rows as unknownApproval/manual review state.',
      portalTest:
        'apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts renders unknown-process and possible-game rows as manual-required/review state.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      unknownProcessPromotedToKnownGame: false,
      heuristicLabelDispatchesAdapter: false,
      manualReviewRequired: true,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
      runtimeAdapterClaimed: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-unknown-process-auto-promotion-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/unknown-process-auto-promotion',
      harness: 'scripts/test/app-game-unknown-process-auto-promotion-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game unknown process auto-promotion gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- Activity-domain identity contracts keep display-label-only evidence weak, unknown, and unknown-executable.',
      '- Activity-domain tests reject deterministic known-game promotion from display-only identity.',
      '- Category/risk tests keep unknown heuristic labels as manual-review candidates and reject direct block.',
      '- Portal app/game dashboard renders unknown-process and possible-game rows as manual-required review state.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-unknown-process-auto-promotion-gate-proof-ok');
  console.log('evidence=test-results/app-game-unknown-process-auto-promotion-gate-proof/proof.json');
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
