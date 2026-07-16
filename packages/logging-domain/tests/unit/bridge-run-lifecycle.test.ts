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
});
