import { existsSync, mkdirSync, readFileSync, rmSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { delimiter, join, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

const workspaceRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const localRoot = join(workspaceRoot, '.codeql-local');
const databaseRoot = join(localRoot, 'databases');
const resultRoot = join(localRoot, 'results');
const codeqlTargets = [
  {
    language: 'actions',
    pack: 'codeql/actions-queries',
    suite: 'codeql/actions-queries:codeql-suites/actions-security-and-quality.qls',
    resultName: 'actions-security-and-quality.sarif',
  },
  {
    language: 'javascript-typescript',
    pack: 'codeql/javascript-queries',
    suite: 'codeql/javascript-queries:codeql-suites/javascript-security-and-quality.qls',
    resultName: 'javascript-security-and-quality.sarif',
  },
  {
    language: 'rust',
    pack: 'codeql/rust-queries',
    suite: 'codeql/rust-queries:codeql-suites/rust-security-and-quality.qls',
    resultName: 'rust-security-and-quality.sarif',
  },
];

const args = new Set(process.argv.slice(2));
const shouldReuseDatabase = args.has('--reuse-db');
const shouldSkipPackDownload = args.has('--no-pack-download');
const shouldRunAllTargets = args.has('--all');
const shouldFailOnLocalChanges = args.has('--fail-on-local-changes');
const selectedLanguages = new Set(
  (process.env.CODEQL_LANGUAGES ?? '')
    .split(',')
    .map((language) => language.trim())
    .filter(Boolean)
);
const threadCount = process.env.CODEQL_THREADS ?? '0';
const ramMegabytes = process.env.CODEQL_RAM_MB;

function activeTargets() {
  if (shouldRunAllTargets) {
    return codeqlTargets;
  }

  if (selectedLanguages.size > 0) {
    return codeqlTargets.filter((target) => selectedLanguages.has(target.language));
  }

  return codeqlTargets.filter((target) => ['actions', 'javascript-typescript'].includes(target.language));
}

function pathCandidates() {
  const executableNames = process.platform === 'win32' ? ['codeql.exe', 'codeql.cmd', 'codeql'] : ['codeql'];
  const configured = process.env.CODEQL_EXE ? [process.env.CODEQL_EXE] : [];
  const knownWindowsPaths =
    process.platform === 'win32' ? ['E:\\tools\\codeql\\codeql.exe', 'E:\\Tools\\codeql\\codeql.exe'] : [];
  const pathDirs = (process.env.PATH ?? '').split(delimiter).filter(Boolean);
  const pathExecutables = pathDirs.flatMap((dir) => executableNames.map((name) => join(dir, name)));

  return [...configured, ...knownWindowsPaths, ...pathExecutables];
}

function findCodeqlExecutable() {
  const codeqlExecutable = pathCandidates().find((candidate) => existsSync(candidate));

  if (!codeqlExecutable) {
    throw new Error(
      [
        'CodeQL CLI was not found.',
        'Install the CodeQL bundle and either add it to PATH or set CODEQL_EXE.',
        'Example: $env:CODEQL_EXE = "E:\\tools\\codeql\\codeql.exe"',
      ].join('\n')
    );
  }

  return codeqlExecutable;
}

function run(command, commandArgs, options = {}) {
  const result = spawnSync(command, commandArgs, {
    cwd: workspaceRoot,
    env: {
      ...process.env,
      PSExecutionPolicyPreference: 'Bypass',
    },
    stdio: 'inherit',
    windowsHide: true,
    ...options,
  });

  if (result.error) {
    throw result.error;
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function withResourceArgs(commandArgs) {
  const resourceArgs = [...commandArgs, `--threads=${threadCount}`];

  if (ramMegabytes) {
    resourceArgs.push(`--ram=${ramMegabytes}`);
  }

  return resourceArgs;
}

function read(command, commandArgs) {
  const result = spawnSync(command, commandArgs, {
    cwd: workspaceRoot,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  if (result.error) {
    throw result.error;
  }

  if (result.status !== 0) {
    return '';
  }

  return `${result.stdout}\n${result.stderr}`;
}

function ensureQueryPack(codeqlExecutable, packName) {
  const resolvedPacks = read(codeqlExecutable, ['resolve', 'packs']);

  if (resolvedPacks.includes(`${packName}:`)) {
    return;
  }

  if (shouldSkipPackDownload) {
    throw new Error(`${packName} is not installed. Re-run without --no-pack-download.`);
  }

  run(codeqlExecutable, ['pack', 'download', packName]);
}

function prepareOutputFolders(databasePath) {
  mkdirSync(databaseRoot, { recursive: true });
  mkdirSync(resultRoot, { recursive: true });

  if (!shouldReuseDatabase) {
    rmSync(databasePath, { recursive: true, force: true });
  }
}

function summarizeSarif(sarifPath, language) {
  const sarif = JSON.parse(readFileSync(sarifPath, 'utf8'));
  const results = sarif.runs.flatMap((sarifRun) => sarifRun.results ?? []);
  const byRule = new Map();

  for (const result of results) {
    byRule.set(result.ruleId, (byRule.get(result.ruleId) ?? 0) + 1);
  }

  const topRules = [...byRule.entries()]
    .sort((left, right) => right[1] - left[1])
    .slice(0, 8)
    .map(([ruleId, count]) => `${ruleId}:${count}`)
    .join(', ');

  console.log(`CodeQL ${language}: ${results.length} result(s)${topRules ? ` (${topRules})` : ''}`);
}

function readGitLines(gitArgs) {
  const result = spawnSync('git', gitArgs, {
    cwd: workspaceRoot,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  if (result.status !== 0) {
    return [];
  }

  return result.stdout
    .split(/\r?\n/u)
    .map((line) => line.trim().replaceAll('\\', '/'))
    .filter(Boolean);
}

function localChangedFiles() {
  return new Set([
    ...readGitLines(['diff', '--name-only']),
    ...readGitLines(['diff', '--cached', '--name-only']),
    ...readGitLines(['ls-files', '--others', '--exclude-standard']),
  ]);
}

function findingsForFiles(sarifPath, changedFiles) {
  const sarif = JSON.parse(readFileSync(sarifPath, 'utf8'));
  const findings = [];

  for (const sarifRun of sarif.runs ?? []) {
    for (const result of sarifRun.results ?? []) {
      for (const location of result.locations ?? []) {
        const uri = location.physicalLocation?.artifactLocation?.uri?.replaceAll('\\\\', '/').replaceAll('\\', '/');

        if (!uri || !changedFiles.has(uri)) {
          continue;
        }

        findings.push({
          ruleId: result.ruleId,
          uri,
          line: location.physicalLocation?.region?.startLine,
          message: result.message?.text?.split('\n')[0] ?? 'CodeQL finding',
        });
      }
    }
  }

  return findings;
}

function main() {
  const codeqlExecutable = findCodeqlExecutable();
  const changedFiles = shouldFailOnLocalChanges ? localChangedFiles() : new Set();
  const changedFindings = [];

  for (const target of activeTargets()) {
    const databasePath = join(databaseRoot, target.language);
    const sarifPath = join(resultRoot, target.resultName);

    ensureQueryPack(codeqlExecutable, target.pack);
    prepareOutputFolders(databasePath);

    if (!shouldReuseDatabase || !existsSync(databasePath)) {
      run(
        codeqlExecutable,
        withResourceArgs([
          'database',
          'create',
          databasePath,
          `--language=${target.language}`,
          `--source-root=${workspaceRoot}`,
        ])
      );
    }

    run(
      codeqlExecutable,
      withResourceArgs([
        'database',
        'analyze',
        databasePath,
        target.suite,
        '--format=sarif-latest',
        `--output=${sarifPath}`,
      ])
    );

    summarizeSarif(sarifPath, target.language);

    if (shouldFailOnLocalChanges) {
      changedFindings.push(...findingsForFiles(sarifPath, changedFiles));
    }
  }

  console.log(`CodeQL SARIF files written to ${resultRoot}`);
  console.log(`CodeQL databases kept at ${databaseRoot}`);

  if (shouldFailOnLocalChanges && changedFindings.length > 0) {
    console.error('CodeQL found result(s) in locally changed files:');
    for (const finding of changedFindings) {
      console.error(`- ${finding.ruleId} ${finding.uri}:${finding.line ?? '?'} ${finding.message}`);
    }
    process.exit(1);
  }
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
}
