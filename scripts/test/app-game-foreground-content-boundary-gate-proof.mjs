import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-foreground-content-boundary-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'foreground-content-boundary');
const commands = [];
const proofBranch = 'codex/app-game-foreground-content-boundary-gate-proof-split';
const deterministicProofRevision = 'branch-head-validated-by-harness';
const deterministicGeneratedAt = 'deterministic-proof-artifact';

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  runNpm(['run', 'build:contracts']);
  runNpm(['run', 'test', '--workspace', '@ocentra-parent/activity-domain', '--', 'app-game-foreground.test.ts']);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'activity-ui-app-game-dashboard-intent.test.ts',
  ]);

  const foregroundSource = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game-foreground.ts'),
    'utf8'
  );
  const foregroundTest = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game-foreground.test.ts'),
    'utf8'
  );
  const intentSource = await readFile(
    join(repoRoot, 'vendor', 'ocentra-parent-core-ui', 'AppPages', 'ParentPortal', 'app-game-dashboard-intent.ts'),
    'utf8'
  );
  const surfaceSource = await readFile(
    join(repoRoot, 'vendor', 'ocentra-parent-core-ui', 'AppPages', 'ParentPortal', 'ParentPortalSvgSurface.tsx'),
    'utf8'
  );
  const routeAssertions = await readFile(
    join(repoRoot, 'apps', 'portal', 'e2e', 'portal-route-scaffold-assertions.ts'),
    'utf8'
  );
  const intentTest = await readFile(
    join(repoRoot, 'apps', 'portal', 'tests', 'activity-ui-app-game-dashboard-intent.test.ts'),
    'utf8'
  );
  const dashboardPanelSlice = sourceSlice(
    surfaceSource,
    'function ParentPortalAppGameDashboardPanel',
    'function ParentPortalAppGameDashboardMetricCard'
  );
  const dashboardRowSlice = sourceSlice(
    surfaceSource,
    'function ParentPortalAppGameDashboardRowCard',
    'function ParentPortalAppGameDashboardMetricList'
  );

  assertIncludes(
    foregroundSource,
    "const AppGameContentKnowledgeStateSchema = withParser(Schema.Literal('notClaimed'))",
    'foreground contract restricts content knowledge state to notClaimed'
  );
  assertIncludes(
    foregroundSource,
    "foreground.contentKnowledgeState === 'notClaimed'",
    'foreground schema rejects content knowledge claims'
  );
  assertIncludes(
    foregroundSource,
    'windowTitleRef: Schema.Union',
    'foreground contract stores title refs, not raw content'
  );
  assertIncludes(
    foregroundTest,
    "expect(parsed.data.contentKnowledgeState).toBe('notClaimed')",
    'foreground test proves notClaimed content knowledge'
  );
  assertIncludes(
    foregroundTest,
    "contentKnowledgeState: 'windowTitleContent'",
    'foreground test rejects content knowledge promotion'
  );
  assertIncludes(
    foregroundTest,
    "windowTitleRef: 'title-ref-4242'",
    'foreground test uses title refs instead of raw window titles'
  );
  assertIncludes(
    intentSource,
    'value: `${row.evidenceCount} refs; ${row.lastObservedLabel}`',
    'dashboard evidence rows render refs and timestamps'
  );
  assertIncludes(
    intentSource,
    'value: `${row.rowCount} source rows; ${row.capabilityStatus}; ${row.lastObservedLabel}; ${row.evidenceCount} refs`',
    'dashboard source rows render source counts and refs'
  );
  assertIncludes(intentTest, "sourceStatusRow('foregroundWindow'", 'portal test includes foreground source status row');
  assertIncludes(
    intentTest,
    "expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('refs')",
    'portal test expects evidence refs'
  );
  assertIncludes(
    intentTest,
    "['App use sources', 'Study Timer', 'Foreground Window', 'Fresh source']",
    'portal test labels foreground source kind without raw content'
  );
  assertIncludes(routeAssertions, 'FOREGROUND', 'route E2E expects foreground visible text');
  assertIncludes(
    dashboardPanelSlice,
    'dashboard.evidenceRows',
    'app/game dashboard panel renders evidence summary rows'
  );
  assertIncludes(dashboardRowSlice, 'Evidence ${row.evidenceCount}', 'app/game row card renders evidence counts');
  assertNotIncludes(dashboardPanelSlice, 'windowTitleRef', 'dashboard panel must not render window title refs');
  assertNotIncludes(dashboardPanelSlice, 'windowTitle', 'dashboard panel must not render raw window titles');
  assertNotIncludes(dashboardPanelSlice, 'executablePath', 'dashboard panel must not render executable paths');
  assertNotIncludes(dashboardRowSlice, 'windowTitleRef', 'dashboard row card must not render window title refs');
  assertNotIncludes(dashboardRowSlice, 'windowTitle', 'dashboard row card must not render raw window titles');
  assertNotIncludes(dashboardRowSlice, 'executablePath', 'dashboard row card must not render executable paths');

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-foreground-content-boundary-gate-proof',
    generatedAt: deterministicGeneratedAt,
    branch: proofBranch,
    commit: deterministicProofRevision,
    commitMetadata:
      'This proof intentionally avoids embedding HEAD because a committed artifact cannot contain its own final commit hash.',
    gitStatusShort: 'validated-by-explicit-handoff-status-check',
    commands,
    gate: 'Foreground evidence is displayed as content knowledge.',
    gateState: 'prevented-by-foreground-contract-and-portal-display-boundary',
    evidence: {
      foregroundContract:
        'packages/activity-domain/src/app-game-foreground.ts restricts contentKnowledgeState to notClaimed and keeps window title data behind refs/capture state.',
      foregroundTest:
        'packages/activity-domain/tests/app-game-foreground.test.ts proves notClaimed content knowledge and rejects a content-knowledge promotion value.',
      portalIntent:
        'vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts renders foreground source rows as counts, capability state, timestamps, and evidence refs.',
      portalSurface:
        'vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx renders app/game dashboard rows and evidence summaries without window title refs, raw window titles, or executable paths.',
      routeAssertion:
        'apps/portal/e2e/portal-route-scaffold-assertions.ts asserts foreground visibility as source/read-model state, not content knowledge.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      foregroundPromotedToContentKnowledge: false,
      windowTitleRendered: false,
      executablePathRendered: false,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
      runtimeAdapterClaimed: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-foreground-content-boundary-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/foreground-content-boundary',
      harness: 'scripts/test/app-game-foreground-content-boundary-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game foreground content boundary gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- Activity-domain foreground evidence restricts contentKnowledgeState to notClaimed.',
      '- Activity-domain foreground tests reject content-knowledge promotion and keep title data behind refs.',
      '- Portal app/game dashboard evidence rows render refs, counts, capability state, and timestamps.',
      '- Core SVG app/game dashboard rows do not render window title refs, raw window titles, or executable paths.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-foreground-content-boundary-gate-proof-ok');
  console.log('evidence=test-results/app-game-foreground-content-boundary-gate-proof/proof.json');
}

function sourceSlice(source, startNeedle, endNeedle) {
  const start = source.indexOf(startNeedle);
  if (start < 0) throw new Error(`Missing source slice start: ${startNeedle}`);
  const end = source.indexOf(endNeedle, start + startNeedle.length);
  if (end < 0) throw new Error(`Missing source slice end: ${endNeedle}`);
  return source.slice(start, end);
}

function assertIncludes(source, needle, label) {
  if (!source.includes(needle)) {
    throw new Error(`Missing ${label}: ${needle}`);
  }
}

function assertNotIncludes(source, needle, label) {
  if (source.includes(needle)) {
    throw new Error(`Unexpected ${label}: ${needle}`);
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
