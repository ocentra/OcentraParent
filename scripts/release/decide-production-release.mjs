import { spawnSync } from 'node:child_process';
import { appendFileSync, existsSync, readFileSync } from 'node:fs';

import { evaluateReleaseVersionPolicy } from './version-policy.mjs';

const zeroBefore = '0000000000000000000000000000000000000000';
const productionRef = 'refs/heads/production';
const decision = decideRelease();

writeOutput('release_required', decision.releaseRequired ? 'true' : 'false');
writeOutput('version', decision.version);
writeOutput('tag', decision.tag);
writeOutput('reason', decision.reason);

console.log(
  `production-release-decision: release_required=${decision.releaseRequired} version=${decision.version} tag=${decision.tag} reason=${decision.reason}`
);

function decideRelease() {
  const versionResult = evaluateReleaseVersionPolicy(process.cwd());
  if (!versionResult.ok) {
    throw new Error(`Release version policy failed: ${versionResult.findings.join('; ')}`);
  }

  const version = versionResult.version;
  const tag = `v${version}`;
  const event = readGithubEvent();

  if (process.env.GITHUB_REF !== productionRef) {
    return {
      releaseRequired: false,
      reason: 'non-production-ref',
      tag,
      version,
    };
  }

  if (event?.before === zeroBefore) {
    return {
      releaseRequired: false,
      reason: 'production-branch-created',
      tag,
      version,
    };
  }

  if (githubReleaseExists(tag)) {
    return {
      releaseRequired: false,
      reason: 'release-tag-already-exists',
      tag,
      version,
    };
  }

  return {
    releaseRequired: true,
    reason: 'release-tag-missing',
    tag,
    version,
  };
}

function readGithubEvent() {
  const eventPath = process.env['GITHUB_EVENT_PATH'];
  if (!eventPath || !existsSync(eventPath)) {
    return null;
  }
  return JSON.parse(readFileSync(eventPath, 'utf8'));
}

function githubReleaseExists(tag) {
  const result = spawnSync('gh', ['release', 'view', tag], {
    encoding: 'utf8',
    env: process.env,
    stdio: ['ignore', 'pipe', 'pipe'],
  });

  if (result.status === 0) {
    return true;
  }

  const output = `${result.stdout}\n${result.stderr}`;
  if (/not found|HTTP 404/u.test(output)) {
    return false;
  }

  throw new Error(`Unable to inspect GitHub release ${tag}: ${output.trim()}`);
}

function writeOutput(name, value) {
  const outputPath = process.env['GITHUB_OUTPUT'];
  if (!outputPath) {
    return;
  }
  appendFileSync(outputPath, `${name}=${value}\n`, 'utf8');
}
