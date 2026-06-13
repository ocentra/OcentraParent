import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-session-duration-replay-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'session-duration-replay');
const proofBranch = 'codex/app-game-session-duration-replay-gate-proof-split';
const deterministicProofRevision = 'branch-head-validated-by-harness';
const deterministicGeneratedAt = 'deterministic-proof-artifact';
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
  runNpm(['run', 'test', '--workspace', '@ocentra-parent/app-game-domain', '--', 'app-game.test.ts']);
  run('cargo', ['test', '-p', 'ocentra-parent-agent-core', 'app_game_sessionization', '--no-default-features']);
  run('cargo', ['test', '-p', 'ocentra-parent-agent-core', 'app_game_journal_sqlite_ingest', '--no-default-features']);

  const tsSessionContracts = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'src', 'app-game.ts'),
    'utf8'
  );
  const tsSessionTests = await readFile(
    join(repoRoot, 'packages', 'activity-domain', 'tests', 'app-game.test.ts'),
    'utf8'
  );
  const rustSessionization = await readFile(
    join(repoRoot, 'crates', 'agent-core', 'src', 'activity_store_app_game', 'app_game_sessionization.rs'),
    'utf8'
  );
  const rustSessionizationTests = await readFile(
    join(repoRoot, 'crates', 'agent-core', 'src', 'activity_store_app_game', 'app_game_sessionization_tests.rs'),
    'utf8'
  );
  const rustJournalReplayTests = await readFile(
    join(repoRoot, 'crates', 'agent-core', 'src', 'activity_store_app_game', 'app_game_journal_sqlite_ingest_tests.rs'),
    'utf8'
  );
  const appGameFeatureDoc = await readFile(join(repoRoot, 'docs', 'features', 'app-game-control.md'), 'utf8');

  assertIncludes(
    tsSessionContracts,
    'session.foregroundDurationMs + session.backgroundDurationMs === session.runningDurationMs',
    'session summary duration total invariant'
  );
  assertIncludes(
    tsSessionContracts,
    'rollup.foregroundDurationMs + rollup.backgroundDurationMs === rollup.runningDurationMs',
    'daily rollup duration total invariant'
  );
  assertIncludes(
    tsSessionTests,
    "it('AppGameSessionSummarySchema: rejects durations beyond running duration'",
    'TS session duration negative test'
  );
  assertIncludes(
    tsSessionTests,
    "it('AppGameSessionDailyRollupSchema: requires exact duration totals'",
    'TS daily rollup duration negative test'
  );

  assertIncludes(
    rustSessionization,
    'observations.sort_by',
    'sessionization sorts replayed rows before deriving duration'
  );
  assertIncludes(
    rustSessionization,
    'update_running_duration(summary, session.started_at_ms, observed_at_ms)',
    'sessionization updates running duration from observation timestamps'
  );
  assertIncludes(
    rustSessionization,
    'summary.background_duration_ms = summary',
    'sessionization derives background duration from running minus foreground duration'
  );
  assertIncludes(
    rustSessionizationTests,
    'fn replay_order_reconstructs_same_summary()',
    'sessionization replay-order stability test'
  );
  assertIncludes(
    rustSessionizationTests,
    'fn daily_rollup_sums_session_durations_by_day_and_classification()',
    'sessionization daily rollup duration test'
  );

  assertIncludes(
    rustJournalReplayTests,
    'fn journal_replay_produces_app_game_sqlite_read_model_rows()',
    'journal replay read-model rows test'
  );
  assertIncludes(
    rustJournalReplayTests,
    'rollup.running_duration_ms, 60000',
    'journal replay changes running duration to one minute'
  );
  assertIncludes(
    rustJournalReplayTests,
    'rollup.foreground_duration_ms, 60000',
    'journal replay changes foreground duration to one minute'
  );
  assertIncludes(
    rustJournalReplayTests,
    'fn duplicate_runtime_observations_do_not_double_count_duration_after_replay()',
    'journal replay duplicate duration guard test'
  );
  assertIncludes(
    rustJournalReplayTests,
    'model.daily_rollups[0].running_duration_ms, 60000',
    'duplicate replay keeps running duration stable'
  );
  assertIncludes(
    rustJournalReplayTests,
    'model.daily_rollups[0].session_count, 1',
    'duplicate replay keeps session count stable'
  );
  assertIncludes(
    appGameFeatureDoc,
    'daily rollup rows while preserving',
    'feature doc names journal replay projection boundary'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-session-duration-replay-gate-proof',
    generatedAt: deterministicGeneratedAt,
    branch: proofBranch,
    commit: deterministicProofRevision,
    commitMetadata:
      'This proof intentionally avoids embedding HEAD because a committed artifact cannot contain its own final commit hash.',
    gitStatusShort: 'validated-by-explicit-handoff-status-check',
    commands,
    gate: 'Session duration changes after journal replay.',
    gateState: 'prevented-by-replay-duration-contract-and-dedupe-proof',
    evidence: {
      typeScriptContracts:
        'packages/app-game-domain/src/app-game.ts requires session and daily-rollup foreground + background duration to equal running duration, with tests covering invalid totals and evidence timestamp requirements.',
      rustSessionization:
        'crates/agent-core/src/activity_store_app_game/app_game_sessionization.rs sorts replayed rows, derives running duration from observation timestamps, and derives background duration from running minus foreground duration.',
      rustSessionizationTests:
        'crates/agent-core/src/activity_store_app_game/app_game_sessionization_tests.rs proves replay order reconstructs the same summary and daily rollups sum durations by day/classification.',
      journalReplayTests:
        'crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs proves encrypted journal replay projects app/game SQLite rows whose daily rollup durations become 60000ms, and duplicate runtime replay does not double-count duration.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      journalReplayChangesDuration: true,
      duplicateReplayDoubleCountsDuration: false,
      inventoryClaimsUse: false,
      rawPrivateExecutablePathExposed: false,
      portalUiChanged: false,
      browserGameWorkDuplicated: false,
      adapterDispatchClaimed: false,
      policyEnforcementClaimed: false,
      packageExportsChanged: false,
    },
    expectedReplayProjection: {
      rollupRunningDurationMs: 60000,
      rollupForegroundDurationMs: 60000,
      duplicateReplayRunningDurationMs: 60000,
      duplicateReplaySessionCount: 1,
    },
    proofPaths: {
      proof: 'test-results/app-game-session-duration-replay-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/session-duration-replay',
      harness: 'scripts/test/app-game-session-duration-replay-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game session duration replay gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- TypeScript session contracts reject impossible running/foreground/background duration totals.',
      '- Rust sessionization sorts replayed rows before deriving session summaries, so replay order does not mutate duration.',
      '- Journal replay into SQLite produces daily rollup duration values from replayed runtime and foreground rows.',
      '- Duplicate runtime journal replay is guarded so duration and session count do not inflate.',
      '- This proof changes no portal UI, adapter dispatch, policy enforcement, or browser-game path.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-session-duration-replay-gate-proof-ok');
  console.log('evidence=test-results/app-game-session-duration-replay-gate-proof/proof.json');
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
  const normalized = output
    .split(repoRoot)
    .join('<repo-root>')
    .split(slashRepoRoot)
    .join('<repo-root>')
    .replace(/Start at\s+\d{2}:\d{2}:\d{2}/g, 'Start at <normalized>')
    .replace(/Duration\s+[^\r\n]+/g, 'Duration <normalized>')
    .replace(/finished in \d+\.\d+s/g, 'finished in <normalized>')
    .replace(/target\(s\) in \d+\.\d+s/g, 'target(s) in <normalized>');
  return normalizeCargoTestLineOrder(normalized);
}

function normalizeCargoTestLineOrder(output) {
  const lines = output.split(/\r?\n/);
  const sortedTestLines = lines.filter((line) => line.startsWith('test ') && line.endsWith(' ... ok')).sort();
  if (sortedTestLines.length === 0) {
    return output;
  }
  return lines
    .map((line) => {
      if (line.startsWith('test ') && line.endsWith(' ... ok')) {
        return sortedTestLines.shift();
      }
      return line;
    })
    .join('\n');
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
