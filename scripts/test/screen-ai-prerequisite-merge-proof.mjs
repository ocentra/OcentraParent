import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';

const repoRoot = process.cwd();
const defaults = {
  screenCommit: '47151efa7ad617c1b0e8bd58ad499731fe9921ff',
  aiCommit: 'd85ab7c8ff90bce792b96150e6b7a0b7ade5fa00',
  output: 'output/screen-ai-pipeline-proof/prerequisite-merge',
};
const argumentsByName = readArguments(process.argv.slice(2));
const options = {
  screenCommit: argumentsByName.get('screen-commit') ?? defaults.screenCommit,
  aiCommit: argumentsByName.get('ai-commit') ?? defaults.aiCommit,
  output: argumentsByName.get('output') ?? defaults.output,
};

const requiredRuntimeSurfaces = [
  'crates/screen-core/Cargo.toml',
  'crates/screen-ai-core/Cargo.toml',
  'crates/agent-service/Cargo.toml',
  'crates/ocentra-eventing/Cargo.toml',
  'crates/schema/Cargo.toml',
];

try {
  const head = git(['rev-parse', 'HEAD']);
  const prerequisites = [
    describePrerequisite('screen', options.screenCommit, 574),
    describePrerequisite('ai', options.aiCommit, 455),
  ];
  const missingSurfaces = requiredRuntimeSurfaces.filter((surface) => !existsSync(resolve(repoRoot, surface)));
  if (missingSurfaces.length > 0) {
    fail(`Missing Rust-first prerequisite runtime surface(s): ${missingSurfaces.join(', ')}`);
  }

  const dirtyPaths = git(['status', '--porcelain=v1']).split('\n').filter(Boolean);
  const artifactDirectory = resolve(repoRoot, options.output);
  mkdirSync(artifactDirectory, { recursive: true });
  const proof = {
    schema_version: 1,
    plan: 'screen-ai-pipeline-plan',
    workpack: '01-prerequisite-merge-and-branch-gate',
    scenario_id: 'prerequisite-merge',
    artifact_shape: 'proof-summary-json',
    platform: 'n/a',
    result: 'pass',
    head,
    prerequisites,
    required_runtime_surfaces: requiredRuntimeSurfaces,
    dirty_paths_at_run: dirtyPaths,
    no_claim:
      'This is a prerequisite ancestry and Rust-first surface proof only. It does not prove capture, AI invocation, policy authority, action execution, custody, portal rendering, or product readiness.',
  };
  writeJson(artifactDirectory, '01-prerequisite-commits.json', { schema_version: 1, head, prerequisites });
  writeJson(artifactDirectory, 'proof-summary.json', proof);
  writeFileSync(
    resolve(artifactDirectory, '13-validation-log.txt'),
    `${JSON.stringify({ command: process.argv.join(' '), exit: 0, result: 'pass', head, prerequisites }, null, 2)}\n`,
    'utf8'
  );
  console.log(
    `screen-ai-prerequisite-merge-proof: pass screen=${options.screenCommit} ai=${options.aiCommit} head=${head}`
  );
} catch (error) {
  console.error('screen-ai-prerequisite-merge-proof: fail');
  console.error(error.message);
  process.exit(1);
}

function readArguments(values) {
  const parsed = new Map();
  for (let index = 0; index < values.length; index += 2) {
    const name = values[index];
    const value = values[index + 1];
    if (!name?.startsWith('--') || !value || value.startsWith('--')) {
      fail(`Expected --name value arguments; received ${values.join(' ')}`);
    }
    parsed.set(name.slice(2), value);
  }
  return parsed;
}

function describePrerequisite(kind, commit, pullRequest) {
  git(['cat-file', '-e', `${commit}^{commit}`]);
  const ancestor = execFileSync('git', ['merge-base', '--is-ancestor', commit, 'HEAD'], {
    cwd: repoRoot,
    encoding: 'utf8',
    stdio: 'ignore',
  });
  void ancestor;
  const [subject, committedAt] = git(['show', '-s', '--format=%s%n%cI', commit]).split('\n');
  return { kind, pull_request: pullRequest, commit, subject, committed_at: committedAt, ancestor_of_head: true };
}

function git(args) {
  return execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
}

function writeJson(directory, fileName, value) {
  writeFileSync(resolve(directory, fileName), `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function fail(message) {
  throw new Error(message);
}
