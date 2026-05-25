#!/usr/bin/env node

import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';

import { WINDOWS_PREVIEW_ARTIFACT_NAME, inspectWindowsPreviewArtifact } from './package-lifecycle-artifacts.mjs';
import { readElevationState, readMsiMetadata, runInstallLifecycle } from './package-lifecycle-host.mjs';
import { buildLifecycleDecision, downloadGitHubArtifact, writeProofJson } from './package-lifecycle-runner.mjs';

const DEFAULT_REPO = 'ocentra/OcentraParent';

async function main() {
  const options = parseArgs(process.argv.slice(2));
  const timestamp = new Date().toISOString().replace(/[:.]/gu, '-');
  const outputDirectory = resolve(
    options.outputDirectory ?? join('test-results', 'windows-package-lifecycle-proof', timestamp)
  );
  mkdirSync(outputDirectory, { recursive: true });

  const proof = {
    generatedAt: new Date().toISOString(),
    host: {
      arch: process.arch,
      node: process.version,
      platform: process.platform,
    },
    lifecycle: {
      decision: { status: 'not-run' },
      install: { attempted: false, status: 'not-run' },
      reboot: { attempted: false, status: 'not-run' },
    },
    outputDirectory,
    schemaVersion: 1,
    status: 'started',
  };

  try {
    const artifactSource = await resolveArtifactDirectory(options, outputDirectory);
    proof.artifactSource = artifactSource.source;
    proof.artifact = inspectWindowsPreviewArtifact(artifactSource.directory);
    proof.msiMetadata = readMsiMetadata(proof.artifact.files.versionedMsi.path);
    proof.elevation = readElevationState();
    const decision = buildLifecycleDecision({
      elevated: proof.elevation.isElevated,
      installRequested: options.install,
    });
    proof.lifecycle = {
      decision,
      install: { attempted: false, status: decision.status },
      reboot: { attempted: false, status: 'not-run' },
    };
    if (decision.status === 'install-requested') {
      proof.lifecycle = {
        decision,
        ...runInstallLifecycle({
          msiPath: proof.artifact.files.versionedMsi.path,
          outputDirectory,
        }),
      };
    }
    proof.status = 'ok';
  } catch (error) {
    proof.status = 'failed';
    proof.error = {
      code: error.code ?? 'unexpected-error',
      message: error.message,
    };
    process.exitCode = 1;
  } finally {
    const proofPath = writeProofJson(outputDirectory, proof);
    console.log(`windows-package-lifecycle-proof=${proofPath}`);
    console.log(`windows-package-lifecycle-status=${proof.status ?? 'failed'}`);
    console.log(`windows-package-lifecycle-decision=${proof.lifecycle.decision.status}`);
  }
}

async function resolveArtifactDirectory(options, outputDirectory) {
  if (options.artifactDirectory) {
    return {
      directory: resolve(options.artifactDirectory),
      source: {
        artifactDirectory: resolve(options.artifactDirectory),
        status: 'provided',
      },
    };
  }
  if (!options.runId) {
    throw new Error('Provide --artifact-dir or --run-id.');
  }
  const downloadDirectory = join(outputDirectory, 'downloaded-artifact');
  const download = downloadGitHubArtifact({
    artifactName: options.artifactName,
    outputDirectory: downloadDirectory,
    repo: options.repo,
    runId: options.runId,
  });
  return {
    directory: downloadDirectory,
    source: download,
  };
}

function parseArgs(args) {
  const parsed = {
    artifactName: WINDOWS_PREVIEW_ARTIFACT_NAME,
    install: false,
    repo: DEFAULT_REPO,
  };
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === '--install') {
      parsed.install = true;
      continue;
    }
    const next = args[index + 1];
    if (next === undefined) {
      throw new Error(`Missing value for ${arg}.`);
    }
    if (arg === '--artifact-dir') {
      parsed.artifactDirectory = next;
    } else if (arg === '--artifact-name') {
      parsed.artifactName = next;
    } else if (arg === '--out-dir') {
      parsed.outputDirectory = next;
    } else if (arg === '--repo') {
      parsed.repo = next;
    } else if (arg === '--run-id') {
      parsed.runId = next;
    } else {
      throw new Error(`Unknown argument: ${arg}`);
    }
    index += 1;
  }
  return parsed;
}

main().catch((error) => {
  console.error(error.message);
  process.exitCode = 1;
});
