import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { afterEach, describe, expect, it } from 'vitest';

function workspaceRoot(): string {
  return path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..');
}

interface CliResult {
  readonly status: number | null;
  readonly stdout: string;
  readonly stderr: string;
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

function runMcp(
  args: readonly string[],
  env: NodeJS.ProcessEnv = process.env
): CliResult {
  const result = spawnSync(
    process.execPath,
    [path.join(workspaceRoot(), 'scripts/dev/mcp-logging-server.mjs'), ...args],
    {
      cwd: workspaceRoot(),
      encoding: 'utf8',
      env,
      windowsHide: true,
    }
  );

  return {
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
  };
}

function encodeMcpMessage(payload: object): string {
  const body = JSON.stringify(payload);
  return `Content-Length: ${Buffer.byteLength(body, 'utf8')}\r\n\r\n${body}`;
}

async function withMcpServer<T>(
  env: NodeJS.ProcessEnv,
  work: (call: (method: string, params?: object) => Promise<McpResponse>) => Promise<T>
): Promise<T> {
  const child = spawn(process.execPath, [path.join(workspaceRoot(), 'scripts/dev/mcp-logging-server.mjs')], {
    cwd: workspaceRoot(),
    env,
    stdio: ['pipe', 'pipe', 'pipe'],
    windowsHide: true,
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

function makeTempDir(prefix: string): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
}

async function removeDirWithRetries(dirPath: string): Promise<void> {
  for (let attempt = 0; attempt < 20; attempt += 1) {
    try {
      fs.rmSync(dirPath, { force: true, recursive: true });
      return;
    } catch (error) {
      const isBusyError = error instanceof Error && 'code' in error && error.code === 'EBUSY';
      if (!isBusyError) {
        throw error;
      }
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
  }
}

describe('logging-domain MCP query interface', () => {
  const tempDirs: string[] = [];

  afterEach(async () => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await removeDirWithRetries(tempDir);
    }
  });

  it('lists the expected tools', () => {
    const result = runMcp(['--list-tools']);
    expect(result.status).toBe(0);

    const tools = JSON.parse(result.stdout) as ReadonlyArray<{ readonly name: string }>;
    expect(tools.map((tool) => tool.name)).toEqual(
      expect.arrayContaining([
        'get_errors',
        'get_recent_logs',
        'get_logs_by_source',
        'get_logs_by_context',
        'query_logs',
        'get_log_stats',
        'get_latest_failures',
        'get_run_diagnostics',
        'get_artifact_slice',
        'get_proof_trace',
        'get_proof_trace_gaps',
        'query_proof_trace',
      ])
    );
  });

  it('keeps smoke output compact for the latest failed run', () => {
    const result = runMcp(['--smoke', 'latest-failures']);
    expect(result.status).toBe(0);

    const latest = JSON.parse(result.stdout) as Array<{ readonly runId: string; readonly status: string }>;
    expect(latest.length).toBeGreaterThan(0);
    expect(latest[0]?.runId).toContain('run-');
    expect(latest[0]?.status).toBe('failed');
  });

  it('makes proof-trace smoke honest in a clean workspace and keeps MCP plus CLI semantics aligned', async () => {
    const structuredRoot = makeTempDir('mcp-proof-trace-smoke-');
    tempDirs.push(structuredRoot);
    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: structuredRoot,
    };

    const result = runMcp(['--smoke', 'proof-trace', '--smoke-root', structuredRoot], env);
    expect(result.status).toBe(0);

    const smoke = JSON.parse(result.stdout) as {
      readonly proofId: string;
      readonly scope: string;
      readonly staleProofRemoved: boolean;
      readonly rows: ReadonlyArray<{ readonly traceStep: string }>;
      readonly gapSummary: {
        readonly matchedSteps: number;
        readonly missingSteps: number;
        readonly outOfOrderSteps: number;
        readonly unexpectedWarnOrErrorRows: number;
      };
      readonly cli: {
        readonly proofTrace: string;
        readonly proofTraceGaps: string;
      };
    };

    expect(smoke.scope).toBe('parent-portal');
    expect(smoke.proofId).toBe('wp10-proof-trace-smoke');
    expect(smoke.staleProofRemoved).toBe(true);
    expect(smoke.rows.map((row) => row.traceStep)).toEqual([
      'portal.route.opened',
      'portal.action.clicked',
      'portal.ui.rendered',
    ]);
    expect(smoke.gapSummary).toEqual({
      matchedSteps: 3,
      missingSteps: 0,
      outOfOrderSteps: 0,
      unexpectedWarnOrErrorRows: 0,
    });
    expect(smoke.cli.proofTrace).toContain(`proof_id: ${smoke.proofId}`);
    expect(smoke.cli.proofTraceGaps).toContain('matched_steps: 3');

    await withMcpServer(env, async (call) => {
      const initialize = await call('initialize', {});
      expect(initialize.error).toBeUndefined();

      const traceCall = await call('tools/call', {
        name: 'get_proof_trace',
        arguments: {
          scope: smoke.scope,
          proofId: smoke.proofId,
          limit: 10,
        },
      });
      expect(traceCall.error).toBeUndefined();
      const trace = traceCall.result?.structuredContent as { readonly rows: ReadonlyArray<{ readonly traceStep: string }> };
      expect(trace.rows.map((row) => row.traceStep)).toEqual([
        'portal.route.opened',
        'portal.action.clicked',
        'portal.ui.rendered',
      ]);

      const gapCall = await call('tools/call', {
        name: 'query_proof_trace',
        arguments: {
          scope: smoke.scope,
          proofId: smoke.proofId,
          expectedSteps: [
            'portal.route.opened',
            'portal.action.clicked',
            'portal.ui.rendered',
          ],
          limit: 10,
        },
      });
      expect(gapCall.error).toBeUndefined();
      const gapResult = gapCall.result?.structuredContent as {
        readonly matchedSteps: ReadonlyArray<unknown>;
        readonly missingSteps: ReadonlyArray<unknown>;
        readonly outOfOrderSteps: ReadonlyArray<unknown>;
      };
      expect(gapResult.matchedSteps).toHaveLength(3);
      expect(gapResult.missingSteps).toHaveLength(0);
      expect(gapResult.outOfOrderSteps).toHaveLength(0);
    });
  });

  it('surfaces unknown proof-trace scopes as MCP errors', async () => {
    const structuredRoot = makeTempDir('mcp-proof-trace-missing-scope-');
    tempDirs.push(structuredRoot);
    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: structuredRoot,
    };

    await withMcpServer(env, async (call) => {
      const response = await call('tools/call', {
        name: 'get_proof_trace',
        arguments: {
          scope: 'parent-missing',
        },
      });
      expect(response.error?.message).toContain('No proof trace rows found for scope "parent-missing".');
    });
  });

  it('rejects artifact-slice abuse outside the local logging roots', async () => {
    const structuredRoot = makeTempDir('mcp-artifact-slice-abuse-');
    tempDirs.push(structuredRoot);
    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: structuredRoot,
    };

    await withMcpServer(env, async (call) => {
      const response = await call('tools/call', {
        name: 'get_artifact_slice',
        arguments: {
          path: path.join(workspaceRoot(), 'package.json'),
          startLine: 1,
          maxLines: 5,
        },
      });
      expect(response.error?.message).toContain('Artifact path must stay inside local logging roots.');
    });
  });

  it('clamps artifact-slice requests to the bounded max-lines window', async () => {
    const structuredRoot = makeTempDir('mcp-artifact-slice-clamp-');
    tempDirs.push(structuredRoot);
    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: structuredRoot,
    };
    const artifactPath = path.join(structuredRoot, 'manual-artifact.txt');
    fs.writeFileSync(artifactPath, 'line1\nline2\nline3\nline4\nline5\n', 'utf8');

    await withMcpServer(env, async (call) => {
      const response = await call('tools/call', {
        name: 'get_artifact_slice',
        arguments: {
          path: artifactPath,
          startLine: 2,
          endLine: 99,
          maxLines: 2,
        },
      });
      expect(response.error).toBeUndefined();
      const slice = response.result?.structuredContent as {
        readonly startLine: number;
        readonly endLine: number;
        readonly lineCount: number;
        readonly lines: ReadonlyArray<string>;
      };
      expect(slice.startLine).toBe(2);
      expect(slice.endLine).toBe(3);
      expect(slice.lineCount).toBe(2);
      expect(slice.lines).toEqual(['line2', 'line3']);
    });
  });
});
