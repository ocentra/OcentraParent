#!/usr/bin/env node

import { randomUUID } from 'node:crypto';
import { closeSync, mkdirSync, openSync } from 'node:fs';
import path from 'node:path';
import { env } from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

import { getRunNdjsonFilePath } from '@ocentra-parent/logging-domain/test-log/ndjsonPaths';
import { RunType, TestLogScope } from '@ocentra-parent/logging-domain/test-log/types';
import {
  inspectLocalDevWorkflow,
  sanitizeProofRunIdSegment,
  writeLocalDevInspectionFailure,
  writeLocalDevProofSummary,
} from './local-dev-workflow.js';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const cloudflareDir = path.resolve(scriptDir, '..');
const repoRoot = path.resolve(cloudflareDir, '..', '..');
function generatedProofRunId(): string {
  return `cloudflare-wp07-${randomUUID()}`;
}

export function resolveCloudflareProofRunId(providedRunId = env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID): string {
  return sanitizeProofRunIdSegment(providedRunId ?? '') ?? generatedProofRunId();
}

export function buildDefaultProofLogRoot(runId: string): string {
  const safeRunId = sanitizeProofRunIdSegment(runId);
  if (safeRunId === null) {
    throw new Error('Cloudflare proof run ID must contain a safe path segment');
  }

  return path.join(
    repoRoot,
    'output',
    'cloudflare-control-plane-plan-proof',
    '07-local-dev-seeding-and-fixtures',
    'runs',
    safeRunId
  );
}

export interface LocalDevProofRun {
  readonly runId: string;
  readonly proofLogRoot: string;
}

function configuredProofLogRoot(value = env.OCENTRA_PARENT_LOG_ROOT): string | null {
  const trimmed = (value ?? '').trim();
  return trimmed.length > 0 ? trimmed : null;
}

function reserveProofLog(runId: string, proofLogRoot: string): boolean {
  const logFile = getRunNdjsonFilePath(
    TestLogScope.ParentCloudflare,
    RunType.Single,
    runId,
    'integration',
    proofLogRoot
  );
  mkdirSync(path.dirname(logFile), { recursive: true });
  try {
    closeSync(openSync(logFile, 'wx'));
    return true;
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'EEXIST') {
      return false;
    }
    throw error;
  }
}

export function prepareLocalDevProofRun(
  providedRunId = env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID,
  providedLogRoot = env.OCENTRA_PARENT_LOG_ROOT
): LocalDevProofRun {
  const explicitLogRoot = configuredProofLogRoot(providedLogRoot);
  let runId = resolveCloudflareProofRunId(providedRunId);

  for (let attempt = 0; attempt < 8; attempt += 1) {
    const proofLogRoot = explicitLogRoot ?? buildDefaultProofLogRoot(runId);
    if (reserveProofLog(runId, proofLogRoot)) {
      return { runId, proofLogRoot };
    }
    runId = generatedProofRunId();
  }

  throw new Error('Cloudflare local-dev proof could not allocate a fresh run ID');
}

export function summarizeProofLogLocation(location: string): string {
  const relativeLocation = path.relative(repoRoot, location).replaceAll('\\', '/');
  return relativeLocation.length > 0 && !relativeLocation.startsWith('../') && !path.isAbsolute(relativeLocation)
    ? relativeLocation
    : 'external-proof-root-redacted';
}

export function runLocalDevProof(): void {
  const proofRun = prepareLocalDevProofRun();
  env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID = proofRun.runId;
  env.OCENTRA_PARENT_LOG_ROOT = proofRun.proofLogRoot;

  let workflow: ReturnType<typeof inspectLocalDevWorkflow>;
  try {
    workflow = inspectLocalDevWorkflow();
  } catch (error) {
    writeLocalDevInspectionFailure(error);
    process.exitCode = 1;
    return;
  }
  const summary = {
    runId: proofRun.runId,
    proofLogLocation: summarizeProofLogLocation(proofRun.proofLogRoot),
    startStatus: workflow.start.status,
    seedStatus: workflow.seed.status,
    teardownStatus: workflow.teardown.status,
    noClaim: 'local validation logs are not a tracked WP07 proof bundle or local Worker response proof',
  };
  writeLocalDevProofSummary(summary);
  console.log(JSON.stringify(summary));
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  runLocalDevProof();
}
