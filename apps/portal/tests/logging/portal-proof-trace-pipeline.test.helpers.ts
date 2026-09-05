import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import type { AddressInfo } from 'node:net';
import { afterEach, expect, it } from 'vitest';
import { GeneratedDevLogMessage as DevLogMessage } from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { createLocalArtifactBridgeQueueStorage } from '@ocentra-parent/logging-domain/core/localArtifactBridgeQueueStorage';
import { closeLocalArtifactMutationProvider } from '@ocentra-parent/logging-domain/local-artifact-mutation-provider';
import { appendTestLogEntries } from '@ocentra-parent/logging-domain/test-log/ndjsonWriter';
import { getTestLogScopeDir, listNdjsonFiles } from '@ocentra-parent/logging-domain/test-log/ndjsonPaths';
import { TestLogDuckDb } from '@ocentra-parent/logging-domain/test-log/testLogDuckDb';
import {
  fetchRunInfoFromBridge,
  flushBridgeRun,
  notifyBridgeRunStarted,
} from '@ocentra-parent/logging-domain/transport/bridgeTransport';
import { createBridgeServer } from '@ocentra-parent/logging-domain/transport/bridgeServer';
import type { PortalLoggerRuntime } from '@ocentra-parent/portal-domain/dev-logger';
import { resolvePortalProofTraceConfig, sendPortalProofTraceLog } from '../../src/dev-logger';
import { expectQueryProofTraceSurfaces } from './portal-proof-trace-pipeline.query.helpers';

interface PortalProofTracePipelineContext {
  readonly bridgeServer: ReturnType<typeof createBridgeServer>;
  readonly endpoint: string;
  readonly proofId: string;
  readonly staleProofId: string;
  readonly structuredRoot: string;
  readonly runtime: PortalProofTraceRuntime;
}

interface PortalProofTraceRuntime extends PortalLoggerRuntime {
  readonly localStorage: ReturnType<typeof createLocalArtifactBridgeQueueStorage>;
}

const TestLogScope = {
  ParentPortal: 'parent-portal',
} as const;
const RunType = {
  Single: 'single',
} as const;
const TestLogOrigin = {
  Test: 'test',
} as const;

type StoredTestLogLine = {
  readonly schemaVersion: 1;
  readonly type: 'log';
  readonly scope: (typeof TestLogScope)[keyof typeof TestLogScope];
  readonly runId: string;
  readonly runType: (typeof RunType)[keyof typeof RunType];
  readonly suiteType: 'integration' | null;
  readonly testName: string;
  readonly timestamp: number;
  readonly level: 'info';
  readonly source: string | null;
  readonly context: string | null;
  readonly message: string;
  readonly data: string | null;
  readonly file: string | null;
  readonly filePath: string | null;
  readonly line: number | null;
  readonly column: number | null;
  readonly correlationId: string | null;
  readonly tags: readonly string[];
  readonly stack: string | null;
  readonly origin: (typeof TestLogOrigin)[keyof typeof TestLogOrigin];
  readonly environment: string | null;
};

const proofTraceGlobalKeys = [
  '__OCENTRA_PARENT_PROOF_TRACE',
  '__OCENTRA_PARENT_PROOF_TRACE_ID',
  '__OCENTRA_PARENT_PROOF_TRACE_SCOPE',
  '__OCENTRA_PARENT_PROOF_TRACE_SOURCES',
  '__OCENTRA_PARENT_PROOF_TRACE_LEVEL',
] as const;

export function registerPortalProofTracePipelineSuite(tempDirs: string[]): void {
  const originalStructuredRoot = process.env['OCENTRA_PARENT_LOG_DIR'];

  afterEach(async () => {
    Logger.instance.reset();
    if (originalStructuredRoot == null) {
      delete process.env['OCENTRA_PARENT_LOG_DIR'];
    } else {
      process.env['OCENTRA_PARENT_LOG_DIR'] = originalStructuredRoot;
    }

    for (const key of proofTraceGlobalKeys) {
      delete (globalThis as Record<string, unknown>)[key];
    }

    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await closeLocalArtifactMutationProvider(tempDir);
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  portalProofTracePipelineLifecycleTests(tempDirs);
}

function portalProofTracePipelineLifecycleTests(tempDirs: string[]): void {
  it('proves one portal proof trace through wipe, query service, CLI, MCP, and DuckDB ingest', async () => {
    const context = await createPortalProofTracePipelineContext(tempDirs);
    try {
      await expectBridgeRunStartState(context);
      await emitPortalProofTracePipeline(context);
      await expectDuckDbIngest(context.structuredRoot);
      await expectQueryProofTraceSurfaces(context);
    } finally {
      await closeServer(context.bridgeServer);
    }
  }, 180000);
}

async function createPortalProofTracePipelineContext(tempDirs: string[]): Promise<PortalProofTracePipelineContext> {
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
  return {
    bridgeServer,
    endpoint: `http://127.0.0.1:${address.port}`,
    proofId: 'wp10-portal-proof-trace',
    staleProofId,
    structuredRoot,
    runtime: { localStorage: createLocalArtifactBridgeQueueStorage(structuredRoot) },
  };
}

async function expectBridgeRunStartState(context: PortalProofTracePipelineContext): Promise<void> {
  const scopeDir = getTestLogScopeDir(TestLogScope.ParentPortal, context.structuredRoot);
  expect(listNdjsonFiles(scopeDir).length).toBeGreaterThan(0);

  const started = await notifyBridgeRunStarted(context.endpoint, {
    runId: context.proofId,
    runType: RunType.Single,
    suiteType: 'integration',
    scope: TestLogScope.ParentPortal,
  });
  expect(started).toBe(true);

  const runInfo = await fetchRunInfoFromBridge(context.endpoint);
  expect(runInfo).toMatchObject({
    runId: context.proofId,
    runType: RunType.Single,
    suiteType: 'integration',
    scope: TestLogScope.ParentPortal,
  });

  expect(listNdjsonFiles(scopeDir)).toHaveLength(0);
}

async function emitPortalProofTracePipeline(context: PortalProofTracePipelineContext): Promise<void> {
  const { endpoint, proofId, runtime } = context;
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
    endpoint,
    runtime
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
    endpoint,
    runtime
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
    endpoint,
    runtime
  );

  expect(routeOpened).toBe(true);
  expect(actionClicked).toBe(true);
  expect(uiRendered).toBe(true);

  const flushed = await flushBridgeRun(endpoint, proofId);
  expect(flushed).toBe(true);
}

async function expectDuckDbIngest(structuredRoot: string): Promise<void> {
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
}

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
