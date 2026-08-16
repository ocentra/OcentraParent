import { spawnSync } from 'node:child_process';
import { appendFileSync } from 'node:fs';

const requiredChecks = ['Format, Lint, Types, Rust Check', 'Full Validation Gate', 'Package Preview Gate'];
const repo = requiredEnv('GITHUB_REPOSITORY');
const sha = requiredEnv('GITHUB_SHA');

const associatedPullRequests = ghJson(['api', `repos/${repo}/commits/${sha}/pulls`]);
const mergedMainPullRequest = associatedPullRequests.find(
  (pullRequest) =>
    pullRequest?.merged_at &&
    pullRequest?.base?.ref === 'main' &&
    pullRequest?.merge_commit_sha === sha &&
    pullRequest?.head?.sha
);

if (!mergedMainPullRequest) {
  finish(false, `main push ${sha} is not a verified merge commit from a merged main PR`);
}

const headSha = mergedMainPullRequest.head.sha;
const checkRuns = ghJson(['api', `repos/${repo}/commits/${headSha}/check-runs?per_page=100`]);
const missingOrFailed = requiredChecks.filter((name) => {
  const run = checkRuns.check_runs?.find((candidate) => candidate.name === name);
  return !run || run.status !== 'completed' || run.conclusion !== 'success';
});

if (missingOrFailed.length > 0) {
  finish(
    false,
    `merged PR #${mergedMainPullRequest.number} does not have green required proof on ${headSha}: ${missingOrFailed.join(', ')}`
  );
}

finish(
  true,
  `main push ${sha} reuses green PR #${mergedMainPullRequest.number} proof from ${headSha}: ${requiredChecks.join(', ')}`
);

function ghJson(args) {
  const result = spawnSync('gh', args, {
    encoding: 'utf8',
    env: {
      ...process.env,
      GH_TOKEN: process.env.GH_TOKEN || process.env.GITHUB_TOKEN,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });

  if (result.status !== 0) {
    throw new Error(`gh ${args.join(' ')} failed: ${result.stderr.trim() || result.stdout.trim()}`);
  }

  return JSON.parse(result.stdout);
}

function finish(proofValid, reason) {
  writeOutput('proof_valid', proofValid ? 'true' : 'false');
  writeOutput('full_ci_required', proofValid ? 'false' : 'true');
  writeOutput('reason', reason);
  console.log(`main-pr-proof: proof_valid=${proofValid} full_ci_required=${!proofValid} reason=${reason}`);
  process.exit(0);
}

function writeOutput(name, value) {
  const outputPath = process.env.GITHUB_OUTPUT;
  if (!outputPath) {
    return;
  }
  appendFileSync(outputPath, `${name}=${escapeOutput(value)}\n`, 'utf8');
}

function escapeOutput(value) {
  return String(value).replace(/\r/gu, '%0D').replace(/\n/gu, '%0A');
}

function requiredEnv(name) {
  const value = process.env[name];
  if (!value) {
    throw new Error(`Missing required environment variable ${name}`);
  }
  return value;
}
