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

type McpResponseHandler = (response: McpResponse) => void;

interface ParsedMcpMessage {
  readonly buffer: string;
  readonly response: McpResponse;
}

interface ProofTraceSmokeResult {
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
}

const ExpectedProofTraceSteps = ['portal.route.opened', 'portal.action.clicked', 'portal.ui.rendered'] as const;
const mcpQueryTempDirs: string[] = [];

function readMcpMessage(buffer: string): ParsedMcpMessage | null {
  const headerEnd = buffer.indexOf('\r\n\r\n');
  if (headerEnd === -1) {
    return null;
  }

  const header = buffer.slice(0, headerEnd);
  const lengthMatch = header.match(/Content-Length:\s*(\d+)/i);
  if (lengthMatch == null) {
    return null;
  }

  const contentLength = Number(lengthMatch[1]);
  const messageStart = headerEnd + 4;
  if (buffer.length < messageStart + contentLength) {
    return null;
  }

  const body = buffer.slice(messageStart, messageStart + contentLength);
  return {
    buffer: buffer.slice(messageStart + contentLength),
    response: JSON.parse(body) as McpResponse,
  };
}

function drainMcpBuffer(buffer: string, pending: Map<number, McpResponseHandler>): string {
  while (true) {
    const message = readMcpMessage(buffer);
    if (message == null) {
      return buffer;
    }

    buffer = message.buffer;
    pending.get(message.response.id)?.(message.response);
    pending.delete(message.response.id);
  }
}

class McpServerSession {
  private buffer = '';
  private nextId = 0;
  private readonly pending = new Map<number, McpResponseHandler>();

  constructor(private readonly child: ReturnType<typeof spawn>) {
    this.child.stdout.setEncoding('utf8');
    this.child.stdout.on('data', this.handleStdoutData);
    this.child.stderr.resume();
  }

  readonly call = (method: string, params?: object): Promise<McpResponse> => {
    this.nextId += 1;
    const id = this.nextId;
    return new Promise((resolve) => {
      this.pending.set(id, resolve);
      this.child.stdin.write(encodeMcpMessage({ jsonrpc: '2.0', id, method, params }));
    });
  };

  async stop(): Promise<void> {
    this.child.kill();
    await new Promise<void>((resolve) => {
      this.child.once('exit', () => resolve());
    });
  }

  private readonly handleStdoutData = (chunk: string): void => {
    this.buffer = drainMcpBuffer(this.buffer + chunk, this.pending);
  };
}

function runMcp(args: readonly string[], env: NodeJS.ProcessEnv = process.env): CliResult {
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
  const session = new McpServerSession(child);

  try {
    return await work(session.call);
  } finally {
    await session.stop();
  }
}

function makeTempDir(prefix: string): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
}

function writeFile(filePath: string, content: string): void {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content, 'utf8');
}

function createProofInventoryFixture(rootDir: string): void {
  writeFile(
    path.join(rootDir, 'docs', 'plans', 'logging-domain-parity', 'PROOF_INDEX.md'),
    [
      '# Proof Index',
      'output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/',
      'output/logging-domain-parity-proof/06-validation-and-enforcement/',
      '',
    ].join('\n')
  );
  writeFile(
    path.join(rootDir, 'docs', 'plans', 'logging-domain-parity', 'WORKPACK_INDEX.md'),
    [
      '| Status | Workpack | Boxes | Primary source doc |',
      '| --- | --- | ---: | --- |',
      '| source-present | [WP03 Parent Logging Architecture and Routing](workpacks/03-parent-logging-architecture-and-routing.md) | 0/11 | `01-parent-logging-architecture.md` |',
      '| partial-proof | [WP06 Validation and Enforcement](workpacks/06-validation-and-enforcement.md) | 0/12 | `04-validation-and-enforcement.md` |',
      '',
    ].join('\n')
  );
  writeFile(
    path.join(rootDir, 'docs', 'plans', 'logging-domain-parity', 'CHECKLIST_INDEX.md'),
    [
      '## WP03 Parent Logging Architecture and Routing',
      '- [ ] Proof root written.',
      '- [ ] Workpack completion section filled.',
      '',
      '## WP06 Validation and Enforcement',
      '- [x] Proof root written.',
      '- [x] Workpack completion section filled.',
      '',
    ].join('\n')
  );
  writeFile(
    path.join(rootDir, 'docs', 'plans', 'logging-domain-parity', 'PLAN_STATE.md'),
    'Proof inventory root: output/logging-domain-parity-proof/ now exists in this checkout, but only WP07 and WP10 roots are restored so far\n'
  );
  writeFile(
    path.join(
      rootDir,
      'output',
      'logging-domain-parity-proof',
      '03-parent-logging-architecture-and-routing',
      '16-validation-commands.log'
    ),
    'command: fixture\nexit: 0\nresult: pass\nnotes: fixture proof\n'
  );
}

function makeLoggingEnv(rootDir: string): NodeJS.ProcessEnv {
  return {
    ...process.env,
    OCENTRA_PARENT_LOG_DIR: rootDir,
  };
}

function expectProofTraceSmokeResult(smoke: ProofTraceSmokeResult): void {
  expect(smoke.scope).toBe('parent-portal');
  expect(smoke.proofId).toBe('wp10-proof-trace-smoke');
  expect(smoke.staleProofRemoved).toBe(true);
  expect(smoke.rows.map((row) => row.traceStep)).toEqual(ExpectedProofTraceSteps);
  expect(smoke.gapSummary).toEqual({
    matchedSteps: 3,
    missingSteps: 0,
    outOfOrderSteps: 0,
    unexpectedWarnOrErrorRows: 0,
  });
  expect(smoke.cli.proofTrace).toContain(`proof_id: ${smoke.proofId}`);
  expect(smoke.cli.proofTraceGaps).toContain('matched_steps: 3');
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

async function expectProofTraceMcpAlignment(env: NodeJS.ProcessEnv, smoke: ProofTraceSmokeResult): Promise<void> {
  await withMcpServer(env, async (call) => assertProofTraceMcpAlignment(call, smoke));
}

async function assertProofTraceMcpAlignment(
  call: (method: string, params?: object) => Promise<McpResponse>,
  smoke: ProofTraceSmokeResult
): Promise<void> {
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
  const trace = traceCall.result?.structuredContent as {
    readonly rows: ReadonlyArray<{ readonly traceStep: string }>;
  };
  expect(trace.rows.map((row) => row.traceStep)).toEqual(ExpectedProofTraceSteps);

  const gapCall = await call('tools/call', {
    name: 'query_proof_trace',
    arguments: {
      scope: smoke.scope,
      proofId: smoke.proofId,
      expectedSteps: [...ExpectedProofTraceSteps],
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
}

async function expectUnknownProofTraceScopeError(env: NodeJS.ProcessEnv): Promise<void> {
  await withMcpServer(env, async (call) => assertUnknownProofTraceScopeError(call));
}

async function assertUnknownProofTraceScopeError(
  call: (method: string, params?: object) => Promise<McpResponse>
): Promise<void> {
  const response = await call('tools/call', {
    name: 'get_proof_trace',
    arguments: {
      scope: 'parent-missing',
    },
  });
  expect(response.error?.message).toContain('No proof trace rows found for scope "parent-missing".');
}

async function expectArtifactSliceAbuseRejected(env: NodeJS.ProcessEnv): Promise<void> {
  await withMcpServer(env, async (call) => assertArtifactSliceAbuseRejected(call));
}

async function assertArtifactSliceAbuseRejected(
  call: (method: string, params?: object) => Promise<McpResponse>
): Promise<void> {
  const response = await call('tools/call', {
    name: 'get_artifact_slice',
    arguments: {
      path: path.join(workspaceRoot(), 'package.json'),
      startLine: 1,
      maxLines: 5,
    },
  });
  expect(response.error?.message).toContain('Artifact path must stay inside local logging roots.');
}

async function expectArtifactSliceClampHonest(env: NodeJS.ProcessEnv, artifactPath: string): Promise<void> {
  await withMcpServer(env, async (call) => assertArtifactSliceClampHonest(call, artifactPath));
}

async function assertArtifactSliceClampHonest(
  call: (method: string, params?: object) => Promise<McpResponse>,
  artifactPath: string
): Promise<void> {
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
}

async function expectProofInventoryStatus(env: NodeJS.ProcessEnv): Promise<void> {
  await withMcpServer(env, async (call) => assertProofInventoryStatus(call));
}

async function assertProofInventoryStatus(
  call: (method: string, params?: object) => Promise<McpResponse>
): Promise<void> {
  const response = await call('tools/call', {
    name: 'get_proof_inventory_status',
    arguments: {},
  });
  expect(response.error).toBeUndefined();
  const inventory = response.result?.structuredContent as {
    readonly summary: { readonly blockingGapCount: number };
    readonly gaps: ReadonlyArray<{ readonly kind: string; readonly workpackId?: string }>;
  };
  expect(inventory.summary.blockingGapCount).toBeGreaterThan(0);
  expect(inventory.gaps).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        kind: 'checklist-claims-proof-root-written-but-root-missing',
        workpackId: '06',
      }),
      expect.objectContaining({
        kind: 'plan-state-restored-roots-drift',
      }),
    ])
  );
}

afterEach(async () => {
  for (const tempDir of mcpQueryTempDirs.splice(0, mcpQueryTempDirs.length)) {
    await removeDirWithRetries(tempDir);
  }
});

describe('logging-domain MCP query interface tool listing', () => {
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
    const logRoot = makeTempDir('mcp-latest-failures-');
    mcpQueryTempDirs.push(logRoot);
    const env = {
      ...process.env,
      OCENTRA_PARENT_LOG_ROOT: logRoot,
      LEDGER_LANE: 'logging-domain-mcp-smoke',
    };
    const agentRun = spawnSync(
      process.execPath,
      [
        path.join(workspaceRoot(), 'scripts', 'dev', 'agent-run.mjs'),
        '--',
        process.execPath,
        '-e',
        "process.stderr.write('expected smoke failure\\n'); process.exit(1)",
      ],
      {
        cwd: workspaceRoot(),
        env,
        encoding: 'utf8',
        windowsHide: true,
      }
    );
    expect(agentRun.status).toBe(1);

    const result = runMcp(['--smoke', 'latest-failures'], env);
    expect(result.status).toBe(0);

    const latest = JSON.parse(result.stdout) as Array<{ readonly runId: string; readonly status: string }>;
    expect(latest.length).toBeGreaterThan(0);
    expect(latest[0]?.runId).toContain('run-');
    expect(latest[0]?.status).toBe('failed');
  });
});

describe('logging-domain MCP query interface proof trace', () => {
  it.skipIf(process.platform !== 'win32')(
    'makes proof-trace smoke honest in a clean workspace and keeps MCP plus CLI semantics aligned',
    async () => {
      const structuredRoot = makeTempDir('mcp-proof-trace-smoke-');
      mcpQueryTempDirs.push(structuredRoot);
      const env = makeLoggingEnv(structuredRoot);

      const result = runMcp(['--smoke', 'proof-trace', '--smoke-root', structuredRoot], env);
      expect(result.status).toBe(0);

      const smoke = JSON.parse(result.stdout) as ProofTraceSmokeResult;
      expectProofTraceSmokeResult(smoke);
      await expectProofTraceMcpAlignment(env, smoke);
    }
  );

  it('surfaces unknown proof-trace scopes as MCP errors', async () => {
    const structuredRoot = makeTempDir('mcp-proof-trace-missing-scope-');
    mcpQueryTempDirs.push(structuredRoot);
    const env = makeLoggingEnv(structuredRoot);
    await expectUnknownProofTraceScopeError(env);
  });
});

describe('logging-domain MCP query interface artifact slices', () => {
  it('rejects artifact-slice abuse outside the local logging roots', async () => {
    const structuredRoot = makeTempDir('mcp-artifact-slice-abuse-');
    mcpQueryTempDirs.push(structuredRoot);
    const env = makeLoggingEnv(structuredRoot);
    await expectArtifactSliceAbuseRejected(env);
  });

  it('clamps artifact-slice requests to the bounded max-lines window', async () => {
    const structuredRoot = makeTempDir('mcp-artifact-slice-clamp-');
    mcpQueryTempDirs.push(structuredRoot);
    const env = makeLoggingEnv(structuredRoot);
    const artifactPath = path.join(structuredRoot, 'manual-artifact.txt');
    fs.writeFileSync(artifactPath, 'line1\nline2\nline3\nline4\nline5\n', 'utf8');
    await expectArtifactSliceClampHonest(env, artifactPath);
  });
});

describe('logging-domain MCP query interface proof inventory', () => {
  it('surfaces stale or missing logging proof inventory through MCP and smoke mode', async () => {
    const workspaceFixture = makeTempDir('mcp-proof-inventory-');
    mcpQueryTempDirs.push(workspaceFixture);
    createProofInventoryFixture(workspaceFixture);

    const env = {
      ...process.env,
      OCENTRA_PARENT_WORKSPACE_ROOT: workspaceFixture,
    };

    const smokeResult = runMcp(['--smoke', 'proof-inventory'], env);
    expect(smokeResult.status).toBe(0);

    const smoke = JSON.parse(smokeResult.stdout) as {
      readonly actualPresentWorkpackIds: ReadonlyArray<string>;
      readonly actualMissingWorkpackIds: ReadonlyArray<string>;
      readonly gaps: ReadonlyArray<{ readonly kind: string; readonly workpackId?: string }>;
    };

    expect(smoke.actualPresentWorkpackIds).toEqual(['03']);
    expect(smoke.actualMissingWorkpackIds).toEqual(['06']);
    expect(smoke.gaps).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          kind: 'status-underclaims-existing-proof-root',
          workpackId: '03',
        }),
        expect.objectContaining({
          kind: 'status-claims-proof-root-but-root-missing',
          workpackId: '06',
        }),
      ])
    );

    await expectProofInventoryStatus(env);
  });
});
