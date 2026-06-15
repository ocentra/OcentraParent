import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import type { AddressInfo } from 'node:net';
import { fileURLToPath } from 'node:url';
import { afterEach, describe, expect, it } from 'vitest';
import { DevLogMessage } from '@ocentra-parent/logging-domain/contracts';
import { appendTestLogEntries } from '@ocentra-parent/logging-domain/test-log/ndjsonWriter';
import { getTestLogScopeDir, listNdjsonFiles } from '@ocentra-parent/logging-domain/test-log/ndjsonPaths';
import { TestLogDuckDb } from '@ocentra-parent/logging-domain/test-log/testLogDuckDb';
import {
  RunType,
  TestLogOrigin,
  TestLogScope,
  type StoredTestLogLine,
} from '@ocentra-parent/logging-domain/test-log/types';
import {
  fetchRunInfoFromBridge,
  flushBridgeRun,
  notifyBridgeRunStarted,
} from '@ocentra-parent/logging-domain/transport/bridgeTransport';
import { createBridgeServer } from '@ocentra-parent/logging-domain/transport/bridgeServer';
import {
  getProofTrace,
  getProofTraceGaps,
  queryProofTrace,
} from '../../../../scripts/dev/lib/log-query-service.mjs';
import {
  resolvePortalProofTraceConfig,
  sendPortalProofTraceLog,
} from '../../src/dev-logger';

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

const proofTraceGlobalKeys = [
  '__OCENTRA_PARENT_PROOF_TRACE',
  '__OCENTRA_PARENT_PROOF_TRACE_ID',
  '__OCENTRA_PARENT_PROOF_TRACE_SCOPE',
  '__OCENTRA_PARENT_PROOF_TRACE_SOURCES',
  '__OCENTRA_PARENT_PROOF_TRACE_LEVEL',
] as const;

function makeTempDir(prefix: string): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
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

function seedStaleProofTrace(rootDir: string, proofId: string): void {
  const staleLog: StoredTestLogLine = {
    schemaVersion: 1,
    type: 'log',
    scope: TestLogScope.ParentPortal,
    runId: proofId,
    runType: RunType.Single,
    suiteType: 'integration',
    testName: 'stale-proof-trace',
    timestamp: Date.now() - 10_000,
    level: 'info',
    source: 'stale-proof',
    context: 'stale-proof.seed',
    message: 'stale proof trace before wipe',
    data: JSON.stringify({
      proofId,
      traceStep: 'portal.route.opened',
      eventType: 'route',
      status: 'ok',
    }),
    file: 'portal-proof-trace-pipeline.test.ts',
    filePath: 'apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts',
    line: null,
    column: null,
    correlationId: `${proofId}-correlation`,
    tags: [],
    stack: null,
    origin: TestLogOrigin.Test,
    environment: 'test',
  };

  appendTestLogEntries([staleLog], rootDir);
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

function runAgentQuery(args: readonly string[], env: NodeJS.ProcessEnv): string {
  const result = spawnSync(process.execPath, [path.join(workspaceRoot, 'scripts/dev/agent-query.mjs'), ...args], {
    cwd: workspaceRoot,
    env,
    encoding: 'utf8',
  });

  if (result.status !== 0) {
    throw new Error(`agent-query failed\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`);
  }

  return result.stdout;
}

describe('portal proof trace pipeline', () => {
  const tempDirs: string[] = [];
  const originalStructuredRoot = process.env['OCENTRA_PARENT_LOG_DIR'];

  afterEach(async () => {
    if (originalStructuredRoot == null) {
      delete process.env['OCENTRA_PARENT_LOG_DIR'];
    } else {
      process.env['OCENTRA_PARENT_LOG_DIR'] = originalStructuredRoot;
    }

    for (const key of proofTraceGlobalKeys) {
      delete (globalThis as Record<string, unknown>)[key];
    }

    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await removeDirWithRetries(tempDir);
    }
  });

  it('proves one portal proof trace through wipe, query service, CLI, MCP, and DuckDB ingest', async () => {
    const structuredRoot = makeTempDir('portal-proof-trace-pipeline-');
    tempDirs.push(structuredRoot);
    process.env['OCENTRA_PARENT_LOG_DIR'] = structuredRoot;

    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE'] = true;
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE_ID'] = 'proof-global-config';
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE_SCOPE'] = TestLogScope.ParentPortal;
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE_SOURCES'] = 'portal,agent-service';
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE_LEVEL'] = 'debug';

    expect(resolvePortalProofTraceConfig()).toEqual({
      enabled: true,
      proofId: 'proof-global-config',
      scope: TestLogScope.ParentPortal,
      sources: ['portal', 'agent-service'],
      level: 'debug',
    });

    const staleProofId = 'wp10-stale-proof-trace';
    seedStaleProofTrace(structuredRoot, staleProofId);

    const bridgeServer = createBridgeServer({ rootDir: structuredRoot });
    const address = await listen(bridgeServer);

    try {
      const endpoint = `http://127.0.0.1:${address.port}`;
      const proofId = 'wp10-portal-proof-trace';

      expect(listNdjsonFiles(getTestLogScopeDir(TestLogScope.ParentPortal, structuredRoot)).length).toBeGreaterThan(0);

      const started = await notifyBridgeRunStarted(endpoint, {
        runId: proofId,
        runType: RunType.Single,
        suiteType: 'integration',
        scope: TestLogScope.ParentPortal,
      });
      expect(started).toBe(true);

      const runInfo = await fetchRunInfoFromBridge(endpoint);
      expect(runInfo).toMatchObject({
        runId: proofId,
        runType: RunType.Single,
        suiteType: 'integration',
        scope: TestLogScope.ParentPortal,
      });

      expect(listNdjsonFiles(getTestLogScopeDir(TestLogScope.ParentPortal, structuredRoot))).toHaveLength(0);
      await expect(
        getProofTrace({
          scope: TestLogScope.ParentPortal,
          proofId: staleProofId,
          limit: 10,
        })
      ).rejects.toThrow('No structured logs found for scope "parent-portal"');

      const routeOpened = await sendPortalProofTraceLog(
        DevLogMessage.PortalStarted,
        {
          proofId,
          traceStep: 'portal.route.opened',
          eventType: 'route',
          status: 'ok',
          expectedNext: 'portal.action.clicked',
        },
        {},
        endpoint
      );
      const actionClicked = await sendPortalProofTraceLog(
        DevLogMessage.PortalCommandSent,
        {
          proofId,
          traceStep: 'portal.action.clicked',
          eventType: 'action',
          action: 'clicked',
          status: 'ok',
          expectedNext: 'portal.ui.rendered',
        },
        {
          uiTarget: 'open-dev-panel',
        },
        endpoint
      );
      const uiRendered = await sendPortalProofTraceLog(
        DevLogMessage.PortalEventReceived,
        {
          proofId,
          traceStep: 'portal.ui.rendered',
          eventType: 'ui',
          status: 'ok',
          artifactRef: 'artifact://portal/ui-rendered',
          correlationId: 'wp10-portal-correlation',
        },
        {
          renderState: 'visible',
        },
        endpoint
      );

      expect(routeOpened).toBe(true);
      expect(actionClicked).toBe(true);
      expect(uiRendered).toBe(true);

      const flushed = await flushBridgeRun(endpoint, proofId);
      expect(flushed).toBe(true);

      const duckDb = await TestLogDuckDb.create(TestLogScope.ParentPortal, structuredRoot);
      try {
        const ingest = await duckDb.ingestFromScope(TestLogScope.ParentPortal, structuredRoot, true);
        expect(ingest.logsInserted).toBe(3);
        const stats = await duckDb.getStats(TestLogScope.ParentPortal);
        expect(stats.totalLogs).toBe(3);
        expect(stats.errorLogs).toBe(0);
      } finally {
        await duckDb.close();
      }

      const trace = await getProofTrace({
        scope: TestLogScope.ParentPortal,
        proofId,
        limit: 10,
      });
      expect(trace.rows.map((row) => row.traceStep)).toEqual([
        'portal.route.opened',
        'portal.action.clicked',
        'portal.ui.rendered',
      ]);
      expect(trace.rows[0]).toMatchObject({
        source: 'DevLogger',
        context: 'DevLogger.sendPortalProofTraceLog',
        eventType: 'route',
      });
      expect(trace.rows[1]).toMatchObject({
        source: 'DevLogger',
        context: 'DevLogger.sendPortalProofTraceLog',
        action: 'clicked',
        eventType: 'action',
      });
      expect(trace.rows[2]).toMatchObject({
        source: 'DevLogger',
        context: 'DevLogger.sendPortalProofTraceLog',
        artifactRef: 'artifact://portal/ui-rendered',
        correlationId: 'wp10-portal-correlation',
      });

      const gapResult = await getProofTraceGaps({
        scope: TestLogScope.ParentPortal,
        proofId,
        expectedSteps: [
          'portal.route.opened',
          'portal.action.clicked',
          'portal.ui.rendered',
        ],
        limit: 10,
      });
      expect(gapResult.missingSteps).toEqual([]);
      expect(gapResult.outOfOrderSteps).toEqual([]);
      expect(gapResult.unexpectedErrorRows).toEqual([]);

      const missingStepResult = await queryProofTrace({
        scope: TestLogScope.ParentPortal,
        proofId,
        expectedSteps: [
          'portal.route.opened',
          'portal.command.sent',
        ],
        limit: 10,
      });
      if (!('missingSteps' in missingStepResult)) {
        throw new Error('Expected proof-trace gap result for missing-step query.');
      }
      expect(missingStepResult.missingSteps).toEqual(['portal.command.sent']);

      const queryEnv = {
        ...process.env,
        OCENTRA_PARENT_LOG_DIR: structuredRoot,
      };
      const cliTrace = runAgentQuery(
        [
          'proof-trace',
          `--scope=${TestLogScope.ParentPortal}`,
          `--proof-id=${proofId}`,
          '--limit=10',
        ],
        queryEnv
      );
      expect(cliTrace).toContain(`proof_id: ${proofId}`);
      expect(cliTrace).toContain('[portal.route.opened] DevLogger/DevLogger.sendPortalProofTraceLog ok');
      expect(cliTrace).toContain('[portal.ui.rendered] DevLogger/DevLogger.sendPortalProofTraceLog ok');

      const cliGaps = runAgentQuery(
        [
          'proof-trace-gaps',
          `--scope=${TestLogScope.ParentPortal}`,
          `--proof-id=${proofId}`,
          '--expected-steps-json=["portal.route.opened","portal.action.clicked","portal.ui.rendered"]',
          '--limit=10',
        ],
        queryEnv
      );
      expect(cliGaps).toContain('matched_steps: 3');
      expect(cliGaps).toContain('missing_steps: 0');
      expect(cliGaps).toContain('unexpected_warn_or_error_rows: 0');

      await withMcpServer(queryEnv, async (call) => {
        const initialize = await call('initialize', {});
        expect(initialize.error).toBeUndefined();

        const tools = await call('tools/list');
        expect(tools.result?.tools?.some((tool) => tool.name === 'get_proof_trace')).toBe(true);
        expect(tools.result?.tools?.some((tool) => tool.name === 'get_proof_trace_gaps')).toBe(true);
        expect(tools.result?.tools?.some((tool) => tool.name === 'query_proof_trace')).toBe(true);

        const traceCall = await call('tools/call', {
          name: 'get_proof_trace',
          arguments: {
            scope: TestLogScope.ParentPortal,
            proofId,
            limit: 10,
          },
        });
        const traceContent = traceCall.result?.structuredContent;
        if (traceContent == null || typeof traceContent !== 'object' || !('rows' in traceContent)) {
          throw new Error('Expected MCP proof-trace content with rows.');
        }
        expect((traceContent as { readonly rows: readonly unknown[] }).rows).toHaveLength(3);

        const queryCall = await call('tools/call', {
          name: 'query_proof_trace',
          arguments: {
            scope: TestLogScope.ParentPortal,
            proofId,
            expectedSteps: [
              'portal.route.opened',
              'portal.action.clicked',
              'portal.ui.rendered',
            ],
            limit: 10,
          },
        });
        const queryContent = queryCall.result?.structuredContent;
        if (queryContent == null || typeof queryContent !== 'object' || !('matchedSteps' in queryContent)) {
          throw new Error('Expected MCP proof-trace gap content with matchedSteps.');
        }
        expect((queryContent as { readonly matchedSteps: readonly unknown[] }).matchedSteps).toHaveLength(3);
      });
    } finally {
      await closeServer(bridgeServer);
    }
  }, 180000);
});
