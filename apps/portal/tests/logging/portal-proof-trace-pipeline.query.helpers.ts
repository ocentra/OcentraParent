import { spawn, spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect } from 'vitest';
import { getProofTrace, getProofTraceGaps, queryProofTrace } from '../../../../scripts/dev/lib/log-query-service.mjs';

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

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', '..', '..');
const TestLogScope = {
  ParentPortal: 'parent-portal',
} as const;

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

function queryEnvFor(structuredRoot: string): NodeJS.ProcessEnv {
  return {
    ...process.env,
    OCENTRA_PARENT_LOG_DIR: structuredRoot,
  };
}

async function expectMcpProofTrace(structuredRoot: string, proofId: string): Promise<void> {
  const queryEnv = queryEnvFor(structuredRoot);
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
    expectMcpTraceRows(traceCall.result?.structuredContent);

    const queryCall = await call('tools/call', {
      name: 'query_proof_trace',
      arguments: {
        scope: TestLogScope.ParentPortal,
        proofId,
        expectedSteps: ['portal.route.opened', 'portal.action.clicked', 'portal.ui.rendered'],
        limit: 10,
      },
    });
    expectMcpMatchedSteps(queryCall.result?.structuredContent);
  });
}

async function expectQueryServiceTrace(proofId: string): Promise<void> {
  const trace = await getProofTrace({
    scope: TestLogScope.ParentPortal,
    proofId,
    limit: 10,
  });
  expect(trace.rows.map((row) => row.traceStep ?? '')).toEqual([
    'portal.route.opened',
    'portal.action.clicked',
    'portal.ui.rendered',
  ]);
  const gapResult = await getProofTraceGaps({
    scope: TestLogScope.ParentPortal,
    proofId,
    expectedSteps: ['portal.route.opened', 'portal.action.clicked', 'portal.ui.rendered'],
    limit: 10,
  });
  expect(gapResult.missingSteps).toEqual([]);
  expect(gapResult.outOfOrderSteps).toEqual([]);
  expect(gapResult.unexpectedErrorRows).toEqual([]);

  const missingStepResult = await queryProofTrace({
    scope: TestLogScope.ParentPortal,
    proofId,
    expectedSteps: ['portal.route.opened', 'portal.command.sent'],
    limit: 10,
  });
  if (!('missingSteps' in missingStepResult)) {
    throw new Error('Expected proof-trace gap result for missing-step query.');
  }
  expect(missingStepResult.missingSteps).toEqual(['portal.command.sent']);

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
}

function expectCliProofTrace(structuredRoot: string, proofId: string): void {
  const queryEnv = queryEnvFor(structuredRoot);
  const cliTrace = runAgentQuery(
    ['proof-trace', `--scope=${TestLogScope.ParentPortal}`, `--proof-id=${proofId}`, '--limit=10'],
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
}

async function expectQueryProofTraceSurfaces(context: {
  readonly structuredRoot: string;
  readonly proofId: string;
}): Promise<void> {
  await expectQueryServiceTrace(context.proofId);
  expectCliProofTrace(context.structuredRoot, context.proofId);
  await expectMcpProofTrace(context.structuredRoot, context.proofId);
}

function expectMcpTraceRows(structuredContent: unknown): void {
  if (structuredContent == null || typeof structuredContent !== 'object' || !('rows' in structuredContent)) {
    throw new Error('Expected MCP proof-trace content with rows.');
  }

  expect((structuredContent as { readonly rows: readonly unknown[] }).rows).toHaveLength(3);
}

function expectMcpMatchedSteps(structuredContent: unknown): void {
  if (structuredContent == null || typeof structuredContent !== 'object' || !('matchedSteps' in structuredContent)) {
    throw new Error('Expected MCP proof-trace gap content with matchedSteps.');
  }

  expect((structuredContent as { readonly matchedSteps: readonly unknown[] }).matchedSteps).toHaveLength(3);
}

export { expectQueryProofTraceSurfaces };
