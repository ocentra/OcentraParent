import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { DatabaseSync } from 'node:sqlite';
import { test } from 'node:test';

import { PortalNetworkActivitySeed, seedPortalNetworkActivityStore } from './portal-network-activity-seed.mjs';

test('portal e2e owns agent and portal cleanup outside Playwright webServer', () => {
  const portalManifest = JSON.parse(readFileSync('apps/portal/package.json', 'utf8'));
  const configSource = readFileSync('apps/portal/playwright.config.ts', 'utf8');
  const runnerSource = readFileSync('scripts/test/portal-playwright-runner.mjs', 'utf8');

  assert.equal(portalManifest.scripts['test:e2e'], 'node ../../scripts/test/portal-playwright-runner.mjs');
  assert.equal(configSource.includes('webServer'), false);
  assert.equal(configSource.includes('OCENTRA_PARENT_PORTAL_PORT'), true);
  assert.equal(runnerSource.includes('stopProcessTree'), true);
  assert.equal(runnerSource.includes('SIGKILL'), true);
  assert.equal(runnerSource.includes('resolveParentDevPort'), true);
  assert.equal(runnerSource.includes('assertAgentNetworkActivityReadModel'), true);
});

test('portal local smoke waits for process shutdown before temp cleanup', () => {
  const smokeSource = readFileSync('scripts/test/portal-local-smoke.mjs', 'utf8');
  const stopIndex = smokeSource.indexOf('await Promise.all([stopProcess(portal), stopProcess(agent)])');
  const removeIndex = smokeSource.indexOf('await removeDirectoryWithRetry(devLogDir)');

  assert.notEqual(stopIndex, -1);
  assert.notEqual(removeIndex, -1);
  assert.equal(stopIndex < removeIndex, true);
  assert.equal(smokeSource.includes('stopProcessTreeAndWait'), true);
  assert.equal(smokeSource.includes('resolveParentDevPort'), true);
});

test('portal network activity seed persists evidence before Rust service startup', async () => {
  const runRoot = await mkdtemp(path.join(tmpdir(), 'ocentra-parent-network-seed-'));
  const activityDbPath = path.join(runRoot, 'activity.sqlite');
  try {
    seedPortalNetworkActivityStore(activityDbPath);
    const database = new DatabaseSync(activityDbPath);
    try {
      const journalMode = database.prepare('PRAGMA journal_mode;').get();
      const row = database
        .prepare(
          `
SELECT evidence_json
FROM activity_events
WHERE event_id = ?;
`
        )
        .get('network-ui-flow-1');

      assert.equal(String(Object.values(journalMode)[0]), 'delete');
      assert.equal(typeof row.evidence_json, 'string');
      assert.equal(row.evidence_json.includes(PortalNetworkActivitySeed.EvidenceId), true);
      assert.equal(row.evidence_json.includes(PortalNetworkActivitySeed.JournalEvidenceId), true);
    } finally {
      database.close();
    }
  } finally {
    await rm(runRoot, { recursive: true, force: true });
  }
});

test('portal network activity service preflight uses shared protocol command and seed refs', () => {
  const preflightSource = readFileSync('scripts/test/portal-network-activity-service-preflight.mjs', 'utf8');

  assert.equal(preflightSource.includes('AgentCommand.NetworkFlowReadModelGet'), true);
  assert.equal(preflightSource.includes('AgentEvent.NetworkFlowReadModelReported'), true);
  assert.equal(preflightSource.includes('AgentEventEnvelopeSchema.parse'), true);
  assert.equal(preflightSource.includes('PortalNetworkActivitySeed.EvidenceId'), true);
});
