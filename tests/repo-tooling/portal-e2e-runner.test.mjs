import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { DatabaseSync } from 'node:sqlite';
import { test } from 'node:test';
import { Worker } from 'node:worker_threads';

import { NetworkEvidenceDrawerProofFixture } from '../../scripts/test/network-evidence-drawer-fixture.mjs';
import {
  PortalNetworkActivitySeed,
  seedPortalNetworkActivityStore,
} from '../../scripts/test/portal-network-activity-seed.mjs';

function readEnforcerProfileProofScript(relativePath) {
  const candidateRoots = [
    process.env.OCENTRA_ENFORCER_HOME,
    path.join(process.cwd(), 'node_modules', 'ocentra-enforcer'),
    path.resolve(process.cwd(), '..', 'ocentra-enforcer'),
    'E:\\ocentra-enforcer',
  ].filter(Boolean);
  for (const root of candidateRoots) {
    const fullPath = path.join(root, 'profiles', 'ocentra-parent', 'legacy-scripts', relativePath);
    try {
      return readFileSync(fullPath, 'utf8');
    } catch {
      // Try the next configured Enforcer install path.
    }
  }
  throw new Error(`Unable to find migrated Enforcer proof script ${relativePath}`);
}

test('portal e2e owns agent and portal cleanup outside Playwright webServer', () => {
  const portalManifest = JSON.parse(readFileSync('apps/portal/package.json', 'utf8'));
  const configSource = readFileSync('apps/portal/playwright.config.ts', 'utf8');
  const runnerSource = readFileSync('scripts/test/portal-playwright-runner.mjs', 'utf8');
  const processSource = readFileSync('scripts/test/agent-service-process.mjs', 'utf8');

  assert.equal(portalManifest.scripts['test:e2e'], 'node ../../scripts/test/portal-playwright-runner.mjs');
  assert.equal(configSource.includes('webServer'), false);
  assert.equal(configSource.includes('OCENTRA_PARENT_PORTAL_PORT'), true);
  assert.equal(runnerSource.includes('stopProcessTree'), true);
  assert.equal(runnerSource.includes('buildPortalE2eRustServices(repoRoot)'), true);
  assert.equal(runnerSource.includes('ensureParentDevBridgeBinaryUnlocked'), true);
  assert.equal(runnerSource.includes('signal !== null'), true);
  assert.equal(processSource.includes('SIGKILL'), true);
  assert.equal(runnerSource.includes('resolveParentDevPort'), true);
  assert.equal(runnerSource.includes('assertAgentNetworkActivityReadModel'), true);
  assert.equal(processSource.includes('child.exitCode !== null || child.signalCode !== null'), true);
  assert.equal(processSource.includes("taskkill', ['/IM', imageName, '/T', '/F']"), true);
  assert.equal(processSource.includes('ocentra-parent-dev-bridge.exe'), true);
  assert.equal(processSource.includes('resolveAgentServiceManifestPath(repoRoot)'), true);
  assert.equal(processSource.includes('resolveParentDevBridgeManifestPath(repoRoot)'), true);
  assert.equal(processSource.includes('CARGO_BUILD_JOBS'), false);
});

test('portal e2e CI preserves platform results and reports Enforcer proof diagnostics', () => {
  const workflowSource = readFileSync('.github/workflows/ci-portal-e2e.yml', 'utf8');

  assert.equal(workflowSource.includes('fail-fast: false'), true);
  assert.equal(workflowSource.includes('id: portal_e2e'), true);
  assert.equal(workflowSource.includes('continue-on-error: true'), true);
  assert.equal(workflowSource.includes('proof last-failure --json'), true);
  assert.equal(
    workflowSource.includes('proof artifact --proof ocentra-parent.portal-local-smoke --artifact raw/stdout.log'),
    true
  );
  assert.equal(workflowSource.includes("steps.portal_e2e.outcome == 'failure'"), true);
  assert.equal(workflowSource.includes('node -e "process.exit(1)"'), true);
});

test('portal local smoke waits for process shutdown before temp cleanup', () => {
  const smokeSource = readEnforcerProfileProofScript('scripts/test/portal-local-smoke.mjs');
  const stopIndex = smokeSource.indexOf('await Promise.all([stopProcess(portal), stopProcess(agent)])');
  const removeIndex = smokeSource.indexOf('await removeDirectoryWithRetry(devLogDir)');

  assert.notEqual(stopIndex, -1);
  assert.notEqual(removeIndex, -1);
  assert.equal(stopIndex < removeIndex, true);
  assert.equal(smokeSource.includes('stopProcessTreeAndWait'), true);
  assert.equal(smokeSource.includes('resolveParentDevPort'), true);
});

test('portal local smoke typed activity timeout is configurable and diagnostic', () => {
  const smokeSource = readEnforcerProfileProofScript('scripts/test/portal-local-smoke.mjs');

  assert.equal(smokeSource.includes('OCENTRA_PARENT_PORTAL_ACTIVITY_SMOKE_TIMEOUT_MS'), true);
  assert.equal(smokeSource.includes('typedActivityAdapterSmokeTimeoutMs'), true);
  assert.equal(smokeSource.includes('describeTypedActivityTimeout(steps, stepIndex)'), true);
  assert.equal(smokeSource.includes('while waiting for ${step.event}'), true);
  assert.equal(smokeSource.includes('from ${step.command}'), true);
  assert.equal(smokeSource.includes("new Error('Typed Activity adapter smoke timed out')"), false);
  assert.equal(smokeSource.includes('), 10000);'), false);
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
        .get(PortalNetworkActivitySeed.EventId);

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

test('portal network activity seed waits for a transient SQLite write lock before persisting evidence', async () => {
  const runRoot = await mkdtemp(path.join(tmpdir(), 'ocentra-parent-network-seed-lock-'));
  const activityDbPath = path.join(runRoot, 'activity.sqlite');
  const lock = holdDatabaseWriteLock(activityDbPath, 150);
  try {
    seedPortalNetworkActivityStore(activityDbPath);
    await lock;
    const database = new DatabaseSync(activityDbPath);
    try {
      const row = database
        .prepare('SELECT evidence_json FROM activity_events WHERE event_id = ?;')
        .get(PortalNetworkActivitySeed.EventId);

      assert.equal(typeof row.evidence_json, 'string');
      assert.equal(row.evidence_json.includes(PortalNetworkActivitySeed.EvidenceId), true);
    } finally {
      database.close();
    }
  } finally {
    await rm(runRoot, { recursive: true, force: true });
  }
});

test('portal network activity service preflight uses the Rust wire decoder and shared command helper', () => {
  const preflightSource = readFileSync('scripts/test/portal-network-activity-service-preflight.mjs', 'utf8');

  assert.equal(preflightSource.includes('createPortalSmokeCommandEnvelope'), true);
  assert.equal(preflightSource.includes('parseAgentEventEnvelope'), true);
  assert.equal(preflightSource.includes("'agent.network.flow.read-model.get'"), true);
  assert.equal(preflightSource.includes("'agent.network.flow.read-model.reported'"), true);
  assert.equal(preflightSource.includes('@ocentra-parent/schema-domain'), false);
  assert.equal(preflightSource.includes('PortalNetworkActivitySeed.EvidenceId'), true);
});

test('network drawer proof ids stay single-sourced across scripts and portal tests', () => {
  const e2eSource = readFileSync('apps/portal/tests/e2e/network-evidence-drawer-proof.spec.ts', 'utf8');
  const unitSource = readFileSync('apps/portal/tests/live-activity/live-activity-network-flow.test.ts', 'utf8');
  const seedSource = readFileSync('scripts/test/portal-network-activity-seed.mjs', 'utf8');
  const proofSource = readEnforcerProfileProofScript('scripts/test/network-parent-ui-evidence-drawer-proof.mjs');

  assert.equal(NetworkEvidenceDrawerProofFixture.eventId, PortalNetworkActivitySeed.EventId);
  assert.equal(NetworkEvidenceDrawerProofFixture.evidenceId, PortalNetworkActivitySeed.EvidenceId);
  assert.equal(NetworkEvidenceDrawerProofFixture.journalEvidenceId, PortalNetworkActivitySeed.JournalEvidenceId);
  assert.equal(e2eSource.includes(NetworkEvidenceDrawerProofFixture.eventId), false);
  assert.equal(e2eSource.includes(NetworkEvidenceDrawerProofFixture.evidenceId), false);
  assert.equal(unitSource.includes(NetworkEvidenceDrawerProofFixture.eventId), false);
  assert.equal(unitSource.includes(NetworkEvidenceDrawerProofFixture.evidenceId), false);
  assert.equal(seedSource.includes(NetworkEvidenceDrawerProofFixture.eventId), false);
  assert.equal(seedSource.includes(NetworkEvidenceDrawerProofFixture.evidenceId), false);
  assert.equal(proofSource.includes(NetworkEvidenceDrawerProofFixture.eventId), false);
  assert.equal(proofSource.includes(NetworkEvidenceDrawerProofFixture.evidenceId), false);
});

function holdDatabaseWriteLock(activityDbPath, holdMs) {
  const state = new Int32Array(new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT));
  const worker = new Worker(
    `
      const { DatabaseSync } = require('node:sqlite');
      const { workerData } = require('node:worker_threads');
      const state = new Int32Array(workerData.state);
      const database = new DatabaseSync(workerData.activityDbPath);
      database.exec('BEGIN IMMEDIATE;');
      Atomics.store(state, 0, 1);
      Atomics.notify(state, 0);
      Atomics.wait(state, 0, 1, workerData.holdMs);
      database.exec('COMMIT;');
      database.close();
    `,
    {
      eval: true,
      workerData: { activityDbPath, holdMs, state: state.buffer },
    }
  );
  const acquired = Atomics.wait(state, 0, 0, 5000);
  assert.notEqual(acquired, 'timed-out');
  return new Promise((resolve, reject) => {
    worker.once('error', reject);
    worker.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`SQLite lock worker exited with code ${code}.`));
    });
  });
}
