#!/usr/bin/env node

import { randomUUID } from 'node:crypto';
import path from 'node:path';
import { env } from 'node:process';
import { fileURLToPath } from 'node:url';

import { inspectLocalDevWorkflow } from './local-dev-workflow.js';

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

env.OCENTRA_CLOUDFLARE_PROOF_RUN_ID = runId;
env.OCENTRA_PARENT_LOG_ROOT = proofLogRoot;

const workflow = inspectLocalDevWorkflow();

console.log(
  JSON.stringify({
    runId,
    proofLogRoot,
    startStatus: workflow.start.status,
    seedStatus: workflow.seed.status,
    teardownStatus: workflow.teardown.status,
    noClaim: 'local validation logs are not a tracked WP07 proof bundle or local Worker response proof',
  })
);
