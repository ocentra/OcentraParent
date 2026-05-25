import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { PackageLifecycleProofError } from './package-lifecycle-artifacts.mjs';

export function buildLifecycleDecision({ elevated, installRequested, platform = process.platform }) {
  if (platform !== 'win32') {
    return {
      installAttempted: false,
      reason: 'windows-host-required',
      rebootAttempted: false,
      status: 'unsupported-platform',
    };
  }
  if (!elevated) {
    return {
      installAttempted: false,
      reason: 'requires-elevated-shell',
      rebootAttempted: false,
      status: 'admin-required',
    };
  }
  if (!installRequested) {
    return {
      installAttempted: false,
      reason: 'install-flag-not-set',
      rebootAttempted: false,
      status: 'ready-not-run',
    };
  }
  return {
    installAttempted: true,
    reason: 'install-flag-set',
    rebootAttempted: false,
    status: 'install-requested',
  };
}

export function writeProofJson(outputDirectory, proof) {
  mkdirSync(outputDirectory, { recursive: true });
  const proofPath = join(outputDirectory, 'proof.json');
  writeFileSync(proofPath, `${JSON.stringify(proof, null, 2)}\n`, 'utf8');
  return proofPath;
}

export function downloadGitHubArtifact({ artifactName, outputDirectory, repo, runId }) {
  mkdirSync(outputDirectory, { recursive: true });
  const metadata = readGitHubArtifactMetadata({ artifactName, repo, runId });
  const result = spawnSync(
    'gh',
    ['run', 'download', String(runId), '--repo', repo, '--name', artifactName, '--dir', outputDirectory],
    { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }
  );
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new PackageLifecycleProofError(
      'artifact-download-failed',
      `gh run download failed: ${(result.stderr || result.stdout).trim()}`
    );
  }
  return {
    artifactName,
    createdAt: metadata.created_at,
    digest: metadata.digest,
    expiresAt: metadata.expires_at,
    id: metadata.id,
    outputDirectory,
    repo,
    runId: String(runId),
    sizeInBytes: metadata.size_in_bytes,
    status: 'downloaded',
  };
}

function readGitHubArtifactMetadata({ artifactName, repo, runId }) {
  const result = spawnSync('gh', ['api', `repos/${repo}/actions/runs/${runId}/artifacts`], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new PackageLifecycleProofError(
      'artifact-metadata-read-failed',
      `gh api artifact lookup failed: ${(result.stderr || result.stdout).trim()}`
    );
  }
  const response = JSON.parse(result.stdout);
  const artifact = response.artifacts?.find((entry) => entry.name === artifactName);
  if (!artifact) {
    throw new PackageLifecycleProofError(
      'artifact-metadata-missing',
      `Artifact ${artifactName} was not found in run ${runId}.`
    );
  }
  return artifact;
}
