import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-launcher-child-game-boundary-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'launcher-child-game-boundary');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'app-game-launcher.test.ts',
  ]);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'app-game-evidence-claim.test.ts',
  ]);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'app-game-identity.test.ts',
  ]);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'activity-ui-app-game-dashboard-intent.test.ts',
  ]);

  const launcherSource = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game-launcher.ts'),
    'utf8'
  );
  const appGameSource = await readFile(join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game.ts'), 'utf8');
  const launcherTest = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game-launcher.test.ts'),
    'utf8'
  );
  const evidenceClaimTest = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game-evidence-claim.test.ts'),
    'utf8'
  );
  const identityTest = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game-identity.test.ts'),
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
  const routeAssertions = await readFile(
    join(repoRoot, 'apps', 'portal', 'e2e', 'portal-route-scaffold-assertions.ts'),
    'utf8'
  );

  assertIncludes(
    launcherSource,
    "launcher.classificationState !== 'knownGame' ||",
    'launcher contract blocks known-game classification without proof'
  );
  assertIncludes(
    launcherSource,
    'launcherHasChildGameProof(launcher)',
    'launcher contract requires child-game proof helper'
  );
  assertIncludes(
    launcherSource,
    'launcher.childGameEvidenceClaimId !== null',
    'launcher child-game proof requires childGameEvidenceClaimId'
  );
  assertIncludes(
    launcherSource,
    "launcher.gameProofState !== 'launcherOnly' ||",
    'launcher-only evidence has its own classification guard'
  );
  assertIncludes(
    launcherSource,
    "launcher.classificationState === 'knownLauncher'",
    'launcher-only evidence stays known-launcher'
  );
  assertIncludes(appGameSource, "claim.claimKind !== 'launcher' ||", 'generic evidence claims guard launcher claims');
  assertIncludes(
    appGameSource,
    "claim.identityStrength === 'childGameProof'",
    'generic evidence claims require childGameProof strength'
  );
  assertIncludes(
    appGameSource,
    "return identity.productKind === 'launcher' && identity.classificationState !== 'knownGame'",
    'identity boundary keeps launcher identity out of known-game state'
  );
  assertIncludes(
    launcherTest,
    'expect(parsed.data.classificationState).toBe(AppGameClassificationState.KnownLauncher)',
    'launcher test proves launcher-only state'
  );
  assertIncludes(launcherTest, 'expect(overclaim.success).toBe(false)', 'launcher test rejects candidate overclaim');
  assertIncludes(
    launcherTest,
    'expect(missingProof.success).toBe(false)',
    'launcher test rejects known game without child-game proof id'
  );
  assertIncludes(
    evidenceClaimTest,
    'expect(launcherAsKnownGame.success).toBe(false)',
    'evidence-claim test rejects launcher known-game overclaim'
  );
  assertIncludes(
    evidenceClaimTest,
    'expect(childGameProof.success).toBe(true)',
    'evidence-claim test accepts child-game proof'
  );
  assertIncludes(
    identityTest,
    'expect(launcherAsGame.success).toBe(false)',
    'identity test rejects launcher-as-game identity'
  );
  assertIncludes(
    identityTest,
    'expect(launcherWithChildProof.success).toBe(true)',
    'identity test accepts launcher game identity only with child proof'
  );
  assertIncludes(intentSource, 'readonly launcherOnly: boolean;', 'dashboard row exposes launcher-only flag');
  assertIncludes(
    intentSource,
    "sourceKind === 'games' && (launcherCount > 0 || appGameLauncherOnly(classificationState, productKind))",
    'dashboard marks launcher-only rows separately'
  );
  assertIncludes(intentSource, "{ label: 'Launcher'", 'dashboard exposes launcher metric separately');
  assertIncludes(intentTest, "productKind: 'launcher'", 'portal test includes launcher product row');
  assertIncludes(intentTest, "classificationState: 'known-launcher'", 'portal test keeps launcher classification');
  assertIncludes(intentTest, 'launcherRowCount: 1', 'portal test exposes launcher row count');
  assertIncludes(
    intentTest,
    'expect(dashboard.rows.map((row) => [row.label, row.launcherOnly, row.unknownApproval])).toContainEqual',
    'portal test asserts launcher-only row state'
  );
  assertIncludes(routeAssertions, 'LAUNCHER', 'route E2E expects launcher visible text');

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-launcher-child-game-boundary-gate-proof',
    generatedAt: new Date().toISOString(),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    commands,
    gate: 'Launcher evidence is displayed as active game without child-game proof.',
    gateState: 'prevented-by-launcher-contracts-and-portal-launcher-only-display',
    evidence: {
      launcherContract:
        'packages/activity-domain/src/app-game-launcher.ts requires child-game proof before knownGame and keeps launcherOnly evidence knownLauncher.',
      evidenceClaimContract:
        'packages/activity-domain/src/app-game.ts blocks launcher evidence from knownGame unless identityStrength is childGameProof.',
      launcherTests:
        'packages/activity-domain/tests/app-game-launcher.test.ts rejects launcher candidate overclaims and missing child-game proof ids.',
      identityTests:
        'packages/activity-domain/tests/app-game-identity.test.ts rejects launcher-as-game identity without childGameEvidenceClaimId.',
      portalIntent:
        'vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts renders launcher rows as launcherOnly/Launcher metrics, separate from active child-game proof.',
      portalTest:
        'apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts includes a productKind launcher row with known-launcher classification and launcherRowCount.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      launcherEvidencePromotedToActiveGame: false,
      knownGameRequiresChildGameProof: true,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
      runtimeAdapterClaimed: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-launcher-child-game-boundary-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/launcher-child-game-boundary',
      harness: 'scripts/test/app-game-launcher-child-game-boundary-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game launcher child-game boundary gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort.length === 0 ? 'clean before proof generation' : proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- Launcher evidence contract requires child-game proof before known-game classification.',
      '- Generic app/game evidence claims require childGameProof before launcher evidence can become knownGame.',
      '- Identity tests reject launcher-as-game identity without childGameEvidenceClaimId.',
      '- Portal app/game dashboard renders launcher rows as launcher-only metrics/rows.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-launcher-child-game-boundary-gate-proof-ok');
  console.log('evidence=test-results/app-game-launcher-child-game-boundary-gate-proof/proof.json');
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
  commands.push(`${rendered}\nexit=${result.status}\n${result.stdout}${result.stderr}`);
  if (result.status !== 0) {
    throw new Error(`${rendered} failed with exit ${result.status}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}
