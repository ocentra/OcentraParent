import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import type { AddressInfo } from 'node:net';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { afterEach, describe, expect, it } from 'vitest';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { createBridgeServer } from '@ocentra-parent/logging-domain/transport/bridgeServer';
import { appendTestLogEntries } from '@ocentra-parent/logging-domain/test-log/ndjsonWriter';
import { getTestLogScopeDir, listNdjsonFiles } from '@ocentra-parent/logging-domain/test-log/ndjsonPaths';
import { TestLogDuckDb } from '@ocentra-parent/logging-domain/test-log/testLogDuckDb';
import {
  RunType,
  TestLogOrigin,
  TestLogScope,
  type StoredTestLogLine,
} from '@ocentra-parent/logging-domain/test-log/types';
import { ParentDomainLoggerConsumer } from './parent-domain-logger-consumer';

interface RustDevLogEvent {
  readonly timestamp: string;
  readonly level: 'info' | 'warn' | 'error' | 'debug' | 'trace';
  readonly source: string;
  readonly message: string;
  readonly fields?: Record<string, unknown>;
  readonly runId?: string | null;
  readonly commandId?: string | null;
  readonly correlationId?: string | null;
  readonly file?: string | null;
  readonly filePath?: string | null;
  readonly line?: number | null;
  readonly column?: number | null;
}

interface McpResponse {
  readonly id: number;
  readonly result?: {
    readonly tools?: Array<{ readonly name: string }>;
    readonly structuredContent?: unknown;
  };
  readonly error?: {
    readonly message: string;
  };
}

const workspaceRoot = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  '..',
  '..',
  '..',
  '..'
);

function makeTempDir(prefix: string): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
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

async function removeDirWithRetries(dirPath: string): Promise<void> {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    try {
      fs.rmSync(dirPath, { force: true, recursive: true });
      return;
    } catch (error) {
      const isBusyError = error instanceof Error && 'code' in error && error.code === 'EBUSY';
      if (!isBusyError) {
        throw error;
      }
      if (attempt === 39) {
        return;
      }
      await new Promise((resolve) => setTimeout(resolve, 125));
    }
  }
}

function readStructuredRows(scope: TestLogScope, rootDir: string): Array<Record<string, unknown>> {
  return listNdjsonFiles(getTestLogScopeDir(scope, rootDir)).flatMap((filePath) =>
    fs
      .readFileSync(filePath, 'utf8')
      .trim()
      .split(/\r?\n/)
      .filter((line) => line.trim().length > 0)
      .map((line) => JSON.parse(line) as Record<string, unknown>)
  );
}

function readRustDevLogRows(rustRoot: string): RustDevLogEvent[] {
  const streamRoot = path.join(rustRoot, TestLogScope.ParentAgent, 'ndjson', 'dev-log');
  return listNdjsonFiles(streamRoot).flatMap((filePath) =>
    fs
      .readFileSync(filePath, 'utf8')
      .trim()
      .split(/\r?\n/)
      .filter((line) => line.trim().length > 0)
      .map((line) => JSON.parse(line) as RustDevLogEvent)
  );
}

function normalizeRustRows(rows: readonly RustDevLogEvent[]): StoredTestLogLine[] {
  return rows.map((row, index) => ({
    schemaVersion: 1,
    type: 'log',
    scope: TestLogScope.ParentAgent,
    runId: row.runId ?? 'rust-dev-log-fixture-run',
    runType: RunType.Single,
    suiteType: 'unit',
    testName: 'write_agent_all_levels_emit_ndjson_lines',
    timestamp: Number.isNaN(Date.parse(row.timestamp)) ? Date.now() + index : Date.parse(row.timestamp),
    level: row.level === 'trace' ? 'debug' : row.level,
    source: row.source,
    context: typeof row.fields?.context === 'string' ? row.fields.context : null,
    message: row.message,
    data: row.fields == null ? null : JSON.stringify(row.fields),
    file: row.file ?? null,
    filePath: row.filePath ?? null,
    line: row.line ?? null,
    column: row.column ?? null,
    correlationId: row.correlationId ?? null,
    tags: [],
    stack: null,
    origin: TestLogOrigin.AgentService,
    environment: 'test',
  }));
}

function runRustFixture(rustRoot: string): void {
  const result = spawnSync(
    'cargo',
    ['test', '-p', 'ocentra-parent-agent-service', 'write_agent_all_levels_emit_ndjson_lines'],
    {
      cwd: workspaceRoot,
      env: {
        ...process.env,
        CARGO_TERM_COLOR: 'never',
        OCENTRA_PARENT_LOG_ROOT: rustRoot,
        OCENTRA_PARENT_LOG_SCOPE: TestLogScope.ParentAgent,
      },
      encoding: 'utf8',
    }
  );

  if (result.status !== 0) {
    throw new Error(
      `cargo rust logging fixture failed\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
  }
}

function encodeMcpMessage(payload: object): string {
  const body = JSON.stringify(payload);
  return `Content-Length: ${Buffer.byteLength(body, 'utf8')}\r\n\r\n${body}`;
}

async function withMcpServer<T>(
  env: NodeJS.ProcessEnv,
  work: (call: (method: string, params?: object) => Promise<McpResponse>) => Promise<T>
): Promise<T> {
  const child = spawn(process.execPath, [path.join(workspaceRoot, 'scripts/dev/mcp-logging-server.mjs')], {
    cwd: workspaceRoot,
    env,
    stdio: ['pipe', 'pipe', 'pipe'],
  });
  let buffer = '';
  let nextId = 0;
  const pending = new Map<number, (response: McpResponse) => void>();

  child.stdout.setEncoding('utf8');
  child.stdout.on('data', (chunk: string) => {
    buffer += chunk;
    while (true) {
      const headerEnd = buffer.indexOf('\r\n\r\n');
      if (headerEnd === -1) {
        break;
      }

      const header = buffer.slice(0, headerEnd);
      const lengthMatch = header.match(/Content-Length:\s*(\d+)/i);
      if (lengthMatch == null) {
        buffer = '';
        break;
      }

      const contentLength = Number(lengthMatch[1]);
      const messageStart = headerEnd + 4;
      if (buffer.length < messageStart + contentLength) {
        break;
      }

      const body = buffer.slice(messageStart, messageStart + contentLength);
      buffer = buffer.slice(messageStart + contentLength);
      const response = JSON.parse(body) as McpResponse;
      pending.get(response.id)?.(response);
      pending.delete(response.id);
    }
  });

  child.stderr.resume();

  const call = (method: string, params?: object): Promise<McpResponse> => {
    nextId += 1;
    const id = nextId;
    return new Promise((resolve) => {
      pending.set(id, resolve);
      child.stdin.write(encodeMcpMessage({ jsonrpc: '2.0', id, method, params }));
    });
  };

  try {
    return await work(call);
  } finally {
    child.kill();
    await new Promise<void>((resolve) => {
      child.once('exit', () => resolve());
    });
  }
}

describe('parent-domain logger consumer parity proof', () => {
  const tempDirs: string[] = [];
  const originalLogLevel = process.env.OCENTRA_PARENT_LOG_LEVEL;
  const originalStructuredRoot = process.env.OCENTRA_PARENT_LOG_DIR;
  const originalRustRoot = process.env.OCENTRA_PARENT_LOG_ROOT;

  afterEach(async () => {
    Logger.instance.reset();
    if (originalLogLevel == null) {
      delete process.env.OCENTRA_PARENT_LOG_LEVEL;
    } else {
      process.env.OCENTRA_PARENT_LOG_LEVEL = originalLogLevel;
    }
    if (originalStructuredRoot == null) {
      delete process.env.OCENTRA_PARENT_LOG_DIR;
    } else {
      process.env.OCENTRA_PARENT_LOG_DIR = originalStructuredRoot;
    }
    if (originalRustRoot == null) {
      delete process.env.OCENTRA_PARENT_LOG_ROOT;
    } else {
      process.env.OCENTRA_PARENT_LOG_ROOT = originalRustRoot;
    }
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await removeDirWithRetries(tempDir);
    }
  });

  it('proves TypeScript and Rust logs land in NDJSON, DuckDB, query service, and MCP', async () => {
    process.env.OCENTRA_PARENT_LOG_LEVEL = 'debug';
    const structuredRoot = makeTempDir('parent-domain-logger-structured-');
    const duckDbProofRoot = makeTempDir('parent-domain-logger-duckdb-');
    const rustRoot = makeTempDir('parent-domain-logger-rust-');
    tempDirs.push(structuredRoot, duckDbProofRoot, rustRoot);

    const bridgeServer = createBridgeServer({ rootDir: structuredRoot });
    const address = await listen(bridgeServer);

    try {
      Logger.instance.configure({
        bridgeEndpoint: `http://127.0.0.1:${address.port}`,
        runId: 'parent-domain-logger-run',
        testName: 'parent-domain-logger-consumer',
        scope: TestLogScope.ParentTest,
        runType: RunType.Single,
        suiteType: 'unit',
        origin: TestLogOrigin.Test,
        environment: 'test',
        skipHealthCheck: true,
      });

      const consumer = new ParentDomainLoggerConsumer();
      consumer.emitHelloWorldLogs();
      await Logger.instance.flush();

      const tsRows = readStructuredRows(TestLogScope.ParentTest, structuredRoot);
      expect(tsRows).toHaveLength(4);
      expect(tsRows.map((row) => row.level)).toEqual(['info', 'warn', 'error', 'debug']);

      const typeScriptSource = String(tsRows[0]?.source ?? '');
      const typeScriptContext = String(tsRows[0]?.context ?? '');
      expect(typeScriptSource).toBe('ParentDomainLoggerConsumer');
      expect(typeScriptContext).toContain('emitHelloWorldLogs');

      const tsDb = await TestLogDuckDb.create(TestLogScope.ParentTest, structuredRoot);
      try {
        const ingest = await tsDb.ingestFromScope(TestLogScope.ParentTest, structuredRoot, true);
        expect(ingest.logsInserted).toBe(4);
        const stats = await tsDb.getStats(TestLogScope.ParentTest);
        expect(stats.totalLogs).toBe(4);
        expect(stats.errorLogs).toBe(1);
      } finally {
        await tsDb.close();
      }

      runRustFixture(rustRoot);
      const rustRows = readRustDevLogRows(rustRoot);
      expect(rustRows).toHaveLength(4);
      expect(rustRows.map((row) => row.level)).toEqual(['info', 'warn', 'error', 'debug']);
      expect(rustRows.every((row) => row.source === 'agent-service')).toBe(true);

      appendTestLogEntries(normalizeRustRows(rustRows), duckDbProofRoot);
      const rustDb = await TestLogDuckDb.create(TestLogScope.ParentAgent, duckDbProofRoot);
      try {
        const ingest = await rustDb.ingestFromScope(TestLogScope.ParentAgent, duckDbProofRoot, true);
        expect(ingest.logsInserted).toBe(4);
        const stats = await rustDb.getStats(TestLogScope.ParentAgent);
        expect(stats.totalLogs).toBe(4);
        expect(stats.warnLogs).toBe(1);
      } finally {
        await rustDb.close();
      }

      process.env.OCENTRA_PARENT_LOG_DIR = structuredRoot;
      process.env.OCENTRA_PARENT_LOG_ROOT = rustRoot;
      const queryService = await import(
        `${pathToFileURL(path.join(workspaceRoot, 'scripts/dev/lib/log-query-service.mjs')).href}?parent-domain-logger-proof`
      );

      const queriedTypeScriptRows = await queryService.getLogsBySource({
        scope: TestLogScope.ParentTest,
        source: typeScriptSource,
        limit: 10,
      });
      expect(queriedTypeScriptRows).toHaveLength(4);

      const queriedTypeScriptContextRows = await queryService.getLogsByContext({
        scope: TestLogScope.ParentTest,
        context: typeScriptContext,
        limit: 10,
      });
      expect(queriedTypeScriptContextRows).toHaveLength(4);

      const queriedRustRows = await queryService.getLogsBySource({
        scope: TestLogScope.ParentAgent,
        source: 'agent-service',
        limit: 10,
      });
      expect(queriedRustRows).toHaveLength(4);

      const rustStats = await queryService.getLogStats({
        scope: TestLogScope.ParentAgent,
      });
      expect(rustStats.sources['agent-service']).toBe(4);

      await withMcpServer(
        {
          ...process.env,
          OCENTRA_PARENT_LOG_DIR: structuredRoot,
          OCENTRA_PARENT_LOG_ROOT: rustRoot,
        },
        async (call) => {
          const initialize = await call('initialize', {});
          expect(initialize.error).toBeUndefined();

          const tools = await call('tools/list');
          expect(tools.result?.tools?.some((tool) => tool.name === 'get_logs_by_source')).toBe(true);

          const tsCall = await call('tools/call', {
            name: 'get_logs_by_context',
            arguments: {
              scope: TestLogScope.ParentTest,
              context: typeScriptContext,
              limit: 10,
            },
          });
          expect(Array.isArray(tsCall.result?.structuredContent)).toBe(true);
          expect((tsCall.result?.structuredContent as Array<unknown>).length).toBe(4);

          const rustCall = await call('tools/call', {
            name: 'get_logs_by_source',
            arguments: {
              scope: TestLogScope.ParentAgent,
              source: 'agent-service',
              limit: 10,
            },
          });
          expect(Array.isArray(rustCall.result?.structuredContent)).toBe(true);
          expect((rustCall.result?.structuredContent as Array<unknown>).length).toBe(4);
        }
      );
    } finally {
      await closeServer(bridgeServer);
    }
  }, 180000);
});
