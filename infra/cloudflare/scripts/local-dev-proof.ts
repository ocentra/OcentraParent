#!/usr/bin/env node

import { randomUUID } from 'node:crypto';
import path from 'node:path';
import { env } from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

import { inspectLocalDevWorkflow, writeLocalDevProofSummary } from './local-dev-workflow.js';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const cloudflareDir = path.resolve(scriptDir, '..');
const repoRoot = path.resolve(cloudflareDir, '..', '..');
const runId = env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID ?? `cloudflare-wp07-${randomUUID()}`;
const proofLogRoot =
  env.OCENTRA_PARENT_LOG_ROOT ??
  path.join(
    repoRoot,
    'output',
    'cloudflare-control-plane-plan-proof',
    '07-local-dev-seeding-and-fixtures',
    'runs',
    runId
  );

export function summarizeProofLogLocation(location: string): string {
  const relativeLocation = path.relative(repoRoot, location).replaceAll('\\', '/');
  return relativeLocation.length > 0 && !relativeLocation.startsWith('../') && !path.isAbsolute(relativeLocation)
    ? relativeLocation
    : 'external-proof-root-redacted';
}

function main(): void {
  env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID = runId;
  env.OCENTRA_PARENT_LOG_ROOT = proofLogRoot;

  const workflow = inspectLocalDevWorkflow();
  const summary = {
    runId,
    proofLogLocation: summarizeProofLogLocation(proofLogRoot),
    startStatus: workflow.start.status,
    seedStatus: workflow.seed.status,
    teardownStatus: workflow.teardown.status,
    noClaim: 'local validation logs are not a tracked WP07 proof bundle or local Worker response proof',
  };
  writeLocalDevProofSummary(summary);
  console.log(JSON.stringify(summary));
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  main();
}
