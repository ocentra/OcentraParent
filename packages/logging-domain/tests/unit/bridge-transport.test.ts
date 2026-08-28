import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import type { AddressInfo } from 'node:net';
import { afterEach, describe, expect, it } from 'vitest';
import { RunType, TestLogScope } from '../../src/test-log/types';
import { createBridgeServer } from '../../src/transport/bridgeServer';
import { BridgeTransport, sendToBridge } from '../../src/transport/bridgeTransport';
import { getTestLogScopeDir, listNdjsonFiles } from '../../src/test-log/ndjsonPaths';

function makeTempDir(): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-bridge-'));
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

function createBridgeEntry() {
  return {
    testName: 'stores logs through the bridge',
    runId: 'run-1',
    runType: RunType.Single,
    consumer: null,
    log: {
      log_timestamp: 1_718_000_000_000,
      level: 'info' as const,
      source: 'portal',
      context: 'transport',
      message: 'bridge write',
      data: '{"ok":true}',
      file: 'transport.ts',
      file_path: 'apps/portal/src/transport.ts',
      line: 10,
      column: 2,
      correlation_id: 'cid-1',
      tags: ['smoke'],
      stack: null,
      suite_type: 'unit' as const,
      origin: 'portal' as const,
      environment: 'test',
    },
  };
}

function readSingleStoredEntry(rootDir: string): {
  scope: string;
  source: string | null;
  context: string | null;
  message: string;
  filePath: string | null;
} {
  const files = listNdjsonFiles(getTestLogScopeDir(TestLogScope.ParentTest, rootDir));
  expect(files).toHaveLength(1);
  const filePath = files[0];
  if (filePath == null) {
    throw new Error('Expected one NDJSON file');
  }
  const lines = fs.readFileSync(filePath, 'utf8').trim().split(/\r?\n/);
  expect(lines).toHaveLength(1);
  return JSON.parse(lines[0]) as {
    scope: string;
    source: string | null;
    context: string | null;
    message: string;
    filePath: string | null;
  };
}

describe('bridge transport', () => {
  const tempDirs: string[] = [];
  const servers: Array<ReturnType<typeof createBridgeServer>> = [];

  afterEach(async () => {
    await Promise.all(servers.splice(0, servers.length).map(closeServer));

    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('serializes and sends valid payloads to the bridge with a parent-test default scope', async () => {
    const tempDir = makeTempDir();
    tempDirs.push(tempDir);
    const server = createBridgeServer({ rootDir: tempDir });
    servers.push(server);
    const address = await listen(server);
    await sendToBridge([createBridgeEntry()], `http://127.0.0.1:${address.port}`);
    const entry = readSingleStoredEntry(tempDir);
    expect(entry.scope).toBe('parent-test');
    expect(entry.source).toBe('portal');
    expect(entry.context).toBe('transport');
    expect(entry.message).toBe('bridge write');
    expect(entry.filePath).toBe('apps/portal/src/transport.ts');
  });

  it('rejects invalid payloads without crashing the bridge', async () => {
    const tempDir = makeTempDir();
    tempDirs.push(tempDir);
    const server = createBridgeServer({ rootDir: tempDir });
    servers.push(server);
    const address = await listen(server);
    const response = await fetch(`http://127.0.0.1:${address.port}/__logs__`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ not: 'an-array' }),
    });

    expect(response.status).toBe(400);

    const healthResponse = await fetch(`http://127.0.0.1:${address.port}/__health__`);
    expect(healthResponse.status).toBe(200);

    const files = listNdjsonFiles(getTestLogScopeDir(TestLogScope.ParentTest, tempDir));
    expect(files).toHaveLength(0);
  });

  it('BridgeTransport skips empty batches and emits to the configured endpoint', async () => {
    const transport = new BridgeTransport();
    await transport.emit([]);

    const tempDir = makeTempDir();
    tempDirs.push(tempDir);
    const server = createBridgeServer({ rootDir: tempDir });
    servers.push(server);
    const address = await listen(server);
    const explicit = new BridgeTransport(`http://127.0.0.1:${address.port}`, true);
    await explicit.emit([createBridgeEntry()]);

    const entry = readSingleStoredEntry(tempDir);
    expect(entry.message).toBe('bridge write');
  });

  it('rejects a non-empty emission without an endpoint', async () => {
    const transport = new BridgeTransport();

    await expect(transport.emit([createBridgeEntry()])).rejects.toThrow('log bridge endpoint is required');
  });

  it('keeps ingestion unavailable when the bridge is composed without a trusted transport', async () => {
    const tempDir = makeTempDir();
    tempDirs.push(tempDir);
    const server = createBridgeServer({
      rootDir: tempDir,
      destructiveOperations: 'disabled',
      logIngestion: 'disabled',
    });
    servers.push(server);
    const address = await listen(server);
    const endpoint = `http://127.0.0.1:${address.port}`;

    const healthResponse = await fetch(`${endpoint}/__health__`);
    expect(healthResponse.status).toBe(200);
    expect(await healthResponse.json()).toEqual({
      ok: false,
      directoryDurability: 'mutation-unsupported',
      logIngestion: 'disabled',
      operatorState: null,
    });

    const ingestionResponse = await fetch(`${endpoint}/__logs__`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify([createBridgeEntry()]),
    });
    expect(ingestionResponse.status).toBe(423);
    expect(await ingestionResponse.json()).toEqual({
      ok: false,
      error: 'bridge log ingestion requires a trusted authenticated transport identity',
    });
    expect(listNdjsonFiles(getTestLogScopeDir(TestLogScope.ParentTest, tempDir))).toHaveLength(0);
  });

  it('rejects bridge writes that do not declare JSON content', async () => {
    const tempDir = makeTempDir();
    tempDirs.push(tempDir);
    const server = createBridgeServer({ rootDir: tempDir });
    servers.push(server);
    const address = await listen(server);

    const response = await fetch(`http://127.0.0.1:${address.port}/__logs__`, {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: '[]',
    });

    expect(response.status).toBe(415);
    expect(await response.json()).toEqual({ ok: false, error: 'application/json is required' });
  });
});
