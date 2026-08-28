import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import type { AddressInfo } from 'node:net';
import { afterEach, describe, expect, it } from 'vitest';
import { RunType, TestLogScope, TestLogSchemaVersion } from '../../src/test-log/types';
import { createBridgeServer } from '../../src/transport/bridgeServer';
import {
  fetchRunInfoFromBridge,
  flushBridgeRun,
  notifyBridgeRunStarted,
  sendToBridge,
} from '../../src/transport/bridgeTransport';
import { getTestLogScopeDir, listNdjsonFiles } from '../../src/test-log/ndjsonPaths';
import { appendTestLogEntries } from '../../src/test-log/ndjsonWriter';

function makeTempDir(): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-bridge-life-'));
}

async function listen(server: ReturnType<typeof createBridgeServer>): Promise<AddressInfo> {
  await new Promise<void>((resolve) => {
    server.listen(0, '127.0.0.1', () => resolve());
  });
  return server.address() as AddressInfo;
}

async function closeServer(server: ReturnType<typeof createBridgeServer>): Promise<void> {
  await new Promise<void>((resolve, reject) => {
    server.close((error) => {
      if (error != null) {
        reject(error);
        return;
      }
      resolve();
    });
  });
}

function makeBridgeEntry(runId: string) {
  return {
    testName: 'bridge lifecycle',
    runId,
    runType: RunType.Single,
    consumer: TestLogScope.ParentTest,
    log: {
      log_timestamp: 1_718_000_000_000,
      level: 'info' as const,
      source: 'portal',
      context: 'bridge',
      message: 'bridge lifecycle write',
      data: null,
      file: 'bridge.ts',
      file_path: 'packages/logging-domain/src/transport/bridgeTransport.ts',
      line: 1,
      column: 1,
      correlation_id: 'cid-1',
      tags: [],
      stack: null,
      suite_type: 'unit' as const,
      origin: 'test' as const,
      environment: 'test',
    },
  };
}

const bridgeLifecycleTempDirs: string[] = [];
const bridgeLifecycleServers: Array<ReturnType<typeof createBridgeServer>> = [];

afterEach(async () => {
  await Promise.all(
    bridgeLifecycleServers.splice(0, bridgeLifecycleServers.length).map(
      (server) =>
        new Promise<void>((resolve, reject) => {
          server.close((error) => {
            if (error != null) {
              reject(error);
              return;
            }
            resolve();
          });
        })
    )
  );

  for (const tempDir of bridgeLifecycleTempDirs.splice(0, bridgeLifecycleTempDirs.length)) {
    fs.rmSync(tempDir, { force: true, recursive: true });
  }
});

describe('bridge run lifecycle', () => {
  it('records run metadata, wipes the selected scope, and rejects stale run ids for that scope', async () => {
    const tempDir = makeTempDir();
    bridgeLifecycleTempDirs.push(tempDir);

    appendTestLogEntries(
      [
        {
          schemaVersion: TestLogSchemaVersion,
          type: 'log',
          scope: TestLogScope.ParentTest,
          runId: 'old-run',
          runType: RunType.Single,
          suiteType: 'unit',
          testName: 'old-run',
          timestamp: 1,
          level: 'info',
          source: 'portal',
          context: 'bridge',
          message: 'old',
          data: null,
          file: 'bridge.ts',
          filePath: 'packages/logging-domain/src/transport/bridgeTransport.ts',
          line: null,
          column: null,
          correlationId: null,
          tags: [],
          stack: null,
          origin: 'test',
          environment: 'test',
        },
      ],
      tempDir
    );

    const server = createBridgeServer({ rootDir: tempDir });
    bridgeLifecycleServers.push(server);
    const address = await listen(server);
    const endpoint = `http://127.0.0.1:${address.port}`;

    const started = await notifyBridgeRunStarted(endpoint, {
      runId: 'run-1',
      runType: RunType.Single,
      suiteType: 'unit',
      scope: TestLogScope.ParentTest,
    });
    expect(started).toBe(true);

    const info = await fetchRunInfoFromBridge(endpoint);
    expect(info?.runId).toBe('run-1');
    expect(info?.scope).toBe(TestLogScope.ParentTest);

    const filesAfterWipe = listNdjsonFiles(getTestLogScopeDir(TestLogScope.ParentTest, tempDir));
    expect(filesAfterWipe).toHaveLength(0);

    await sendToBridge([makeBridgeEntry('run-1')], endpoint);

    const lifecyclePath = path.join(tempDir, '.bridge', 'lifecycle-state.json');
    const lifecycleState = JSON.parse(fs.readFileSync(lifecyclePath, 'utf8')) as {
      runCounters: Array<{ runId: string; stored: number; flushed: number; updatedAt: number }>;
    };
    expect(lifecycleState.runCounters).toHaveLength(1);
    expect(lifecycleState.runCounters[0]?.runId).toBe('run-1');
    expect(lifecycleState.runCounters[0]?.stored).toBe(1);
    expect(lifecycleState.runCounters[0]?.flushed).toBe(0);
    expect(Number.isSafeInteger(lifecycleState.runCounters[0]?.updatedAt)).toBe(true);

    const staleResponse = await fetch(`${endpoint}/__logs__`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify([makeBridgeEntry('old-run')]),
    });
    expect(staleResponse.status).toBe(409);

    const flushed = await flushBridgeRun(endpoint, 'run-1');
    expect(flushed).toBe(true);
  });

  it('warns when replacing stale run metadata and preserves the active run across restart', async () => {
    const tempDir = makeTempDir();
    bridgeLifecycleTempDirs.push(tempDir);

    const server = createBridgeServer({ rootDir: tempDir });
    bridgeLifecycleServers.push(server);
    const address = await listen(server);
    const endpoint = `http://127.0.0.1:${address.port}`;

    const started = await notifyBridgeRunStarted(endpoint, {
      runId: 'stale-run',
      runType: RunType.Single,
      suiteType: 'unit',
      scope: TestLogScope.ParentTest,
    });
    expect(started).toBe(true);

    const lifecyclePath = path.join(tempDir, '.bridge', 'lifecycle-state.json');
    const lifecycleState = JSON.parse(fs.readFileSync(lifecyclePath, 'utf8')) as {
      activeRun: { startedAt: number };
    };
    lifecycleState.activeRun.startedAt = Date.now() - 10 * 60 * 1000;
    fs.writeFileSync(lifecyclePath, `${JSON.stringify(lifecycleState)}\n`, 'utf8');

    const replacementResponse = await fetch(`${endpoint}/__run_started__`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        runId: 'fresh-run',
        runType: RunType.Single,
        suiteType: 'unit',
        scope: TestLogScope.ParentTest,
        filePath: null,
        wipeAll: false,
      }),
    });
    expect(replacementResponse.status).toBe(200);
    expect(await replacementResponse.json()).toEqual({
      ok: true,
      warning: 'previous run info was stale and has been replaced',
    });

    await closeServer(server);
    bridgeLifecycleServers.splice(bridgeLifecycleServers.indexOf(server), 1);

    const restarted = createBridgeServer({ rootDir: tempDir });
    bridgeLifecycleServers.push(restarted);
    const restartedAddress = await listen(restarted);
    const info = await fetchRunInfoFromBridge(`http://127.0.0.1:${restartedAddress.port}`);

    expect(info).toEqual({
      runId: 'fresh-run',
      runType: RunType.Single,
      suiteType: 'unit',
      scope: TestLogScope.ParentTest,
      startedAt: expect.any(Number),
    });
  });

  it('keeps the bridge manual-required after invalid lifecycle state recovery', async () => {
    const tempDir = makeTempDir();
    bridgeLifecycleTempDirs.push(tempDir);
    const bridgeDir = path.join(tempDir, '.bridge');
    fs.mkdirSync(bridgeDir, { recursive: true });
    fs.writeFileSync(path.join(bridgeDir, 'lifecycle-state.json'), '{"schemaVersion":999}\n', 'utf8');

    const server = createBridgeServer({ rootDir: tempDir });
    bridgeLifecycleServers.push(server);
    const address = await listen(server);
    const endpoint = `http://127.0.0.1:${address.port}`;

    const healthResponse = await fetch(`${endpoint}/__health__`);
    expect(healthResponse.status).toBe(200);
    const health = (await healthResponse.json()) as {
      ok: boolean;
      operatorState: { status: string; code: string; recordSha256: string } | null;
    };
    expect(health.ok).toBe(false);
    expect(health.operatorState?.status).toBe('manual-required');
    expect(health.operatorState?.code).toBe('invalid-lifecycle-record');
    expect(health.operatorState?.recordSha256).toMatch(/^[0-9a-f]{64}$/u);

    const startResponse = await fetch(`${endpoint}/__run_started__`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ runId: 'blocked-run' }),
    });
    expect(startResponse.status).toBe(423);
    expect(await startResponse.json()).toEqual({
      ok: false,
      error: 'bridge lifecycle requires operator resolution',
    });

    const logsResponse = await fetch(`${endpoint}/__logs__`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify([makeBridgeEntry('blocked-run')]),
    });
    expect(logsResponse.status).toBe(423);
    expect(await logsResponse.json()).toEqual({
      ok: false,
      error: 'bridge lifecycle requires operator resolution',
    });
  });
});
