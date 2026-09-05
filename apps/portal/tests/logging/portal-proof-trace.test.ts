import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import type { AddressInfo } from 'node:net';
import { afterEach, describe, expect, it } from 'vitest';
import { GeneratedDevLogMessage as DevLogMessage } from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { createLocalArtifactBridgeQueueStorage } from '@ocentra-parent/logging-domain/core/localArtifactBridgeQueueStorage';
import { closeLocalArtifactMutationProvider } from '@ocentra-parent/logging-domain/local-artifact-mutation-provider';
import { localArtifactDirectoryDurability } from '@ocentra-parent/logging-domain/local-artifact-path';
import { createBridgeServer } from '@ocentra-parent/logging-domain/transport/bridgeServer';
import type { PortalLoggerRuntime } from '@ocentra-parent/portal-domain/dev-logger';
import { getProofTrace, getProofTraceGaps } from '../../../../scripts/dev/lib/log-query-service.mjs';
import { resolvePortalProofTraceConfig, sendPortalProofTraceLog } from '../../src/dev-logger';

const LOG_ROOT_ENV = 'OCENTRA_PARENT_LOG_DIR';
const proofTraceGlobalKeys = [
  '__OCENTRA_PARENT_PROOF_TRACE',
  '__OCENTRA_PARENT_PROOF_TRACE_ID',
  '__OCENTRA_PARENT_PROOF_TRACE_SCOPE',
  '__OCENTRA_PARENT_PROOF_TRACE_SOURCES',
  '__OCENTRA_PARENT_PROOF_TRACE_LEVEL',
] as const;

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

describe('portal proof trace logging', () => {
  const servers: Array<ReturnType<typeof createBridgeServer>> = [];
  const tempDirs: string[] = [];
  const originalLogRoot = process.env[LOG_ROOT_ENV];

  afterEach(async () => {
    Logger.instance.reset();
    process.env[LOG_ROOT_ENV] = originalLogRoot;
    if (originalLogRoot == null) {
      delete process.env[LOG_ROOT_ENV];
    }

    for (const key of proofTraceGlobalKeys) {
      delete (globalThis as Record<string, unknown>)[key];
    }

    while (servers.length > 0) {
      const server = servers.pop();
      if (server != null) {
        await closeServer(server);
      }
    }

    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await closeLocalArtifactMutationProvider(tempDir);
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  portalProofTraceConfigTests();
  if (localArtifactDirectoryDurability() === 'mutation-unsupported') {
    portalProofTraceCapabilityTests();
  } else {
    portalProofTraceLoggingTests(servers, tempDirs);
  }
});

function portalProofTraceConfigTests(): void {
  it('resolvePortalProofTraceConfig: reads global proof-trace controls', () => {
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE'] = true;
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE_ID'] = 'proof-global';
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE_SCOPE'] = 'parent-portal';
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE_SOURCES'] = 'portal,agent-service';
    (globalThis as Record<string, unknown>)['__OCENTRA_PARENT_PROOF_TRACE_LEVEL'] = 'debug';

    expect(resolvePortalProofTraceConfig()).toEqual({
      enabled: true,
      proofId: 'proof-global',
      scope: 'parent-portal',
      sources: ['portal', 'agent-service'],
      level: 'debug',
    });
  });
}

function portalProofTraceLoggingTests(servers: Array<ReturnType<typeof createBridgeServer>>, tempDirs: string[]): void {
  it('sendPortalProofTraceLog: writes queryable ordered proof-trace rows', async () => {
    const { endpoint, proofId, runtime } = await createPortalProofTraceEnvironment(servers, tempDirs);

    await emitPortalProofTraceSequence(endpoint, proofId, runtime);
    await expectPortalProofTraceRows(proofId);
    await expectPortalProofTraceGapResults(proofId);
  });
}

function portalProofTraceCapabilityTests(): void {
  it('reports unavailable local artifact mutation instead of fabricating proof rows', () => {
    expect(localArtifactDirectoryDurability()).toBe('mutation-unsupported');
  });
}

async function createPortalProofTraceEnvironment(
  servers: Array<ReturnType<typeof createBridgeServer>>,
  tempDirs: string[]
): Promise<{ endpoint: string; proofId: string; runtime: PortalProofTraceRuntime }> {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'portal-proof-trace-'));
  tempDirs.push(tempDir);
  process.env[LOG_ROOT_ENV] = tempDir;

  const server = createBridgeServer({ rootDir: tempDir });
  servers.push(server);
  await new Promise<void>((resolve) => {
    server.listen(0, '127.0.0.1', () => resolve());
  });
  const address = server.address() as AddressInfo;

  return {
    endpoint: `http://127.0.0.1:${address.port}`,
    proofId: 'proof-portal-click',
    runtime: { localStorage: createLocalArtifactBridgeQueueStorage(tempDir) },
  };
}

interface PortalProofTraceRuntime extends PortalLoggerRuntime {
  readonly localStorage: ReturnType<typeof createLocalArtifactBridgeQueueStorage>;
}

async function emitPortalProofTraceSequence(
  endpoint: string,
  proofId: string,
  runtime: PortalProofTraceRuntime
): Promise<void> {
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
      correlationId: 'portal-proof-correlation',
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
}

async function expectPortalProofTraceRows(proofId: string): Promise<void> {
  const trace = await getProofTrace({
    scope: 'parent-portal',
    proofId,
    limit: 10,
  });

  expect(trace.rows.map((row) => row.traceStep)).toEqual([
    'portal.route.opened',
    'portal.action.clicked',
    'portal.ui.rendered',
  ]);
  expect(trace.rows[0]?.filePath).toContain('dev-logger.ts');
  expect(trace.rows[0]?.source).toBe('DevLogger');
  expect(trace.rows[0]?.context).toBe('DevLogger.sendPortalProofTraceLog');
  expect(trace.rows[2]?.correlationId).toBe('portal-proof-correlation');
}

async function expectPortalProofTraceGapResults(proofId: string): Promise<void> {
  const gaps = await getProofTraceGaps({
    scope: 'parent-portal',
    proofId,
    expectedSteps: ['portal.route.opened', 'portal.action.clicked', 'portal.ui.rendered'],
    limit: 10,
  });

  expect(gaps.missingSteps).toEqual([]);
  expect(gaps.outOfOrderSteps).toEqual([]);

  const missingStepResult = await getProofTraceGaps({
    scope: 'parent-portal',
    proofId,
    expectedSteps: ['portal.route.opened', 'portal.command.sent'],
    limit: 10,
  });

  expect(missingStepResult.missingSteps).toEqual(['portal.command.sent']);
}
