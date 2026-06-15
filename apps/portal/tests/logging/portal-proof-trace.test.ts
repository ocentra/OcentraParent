import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import type { AddressInfo } from 'node:net';
import { afterEach, describe, expect, it } from 'vitest';
import { DevLogMessage } from '@ocentra-parent/logging-domain/contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { createBridgeServer } from '@ocentra-parent/logging-domain/transport/bridgeServer';
import {
  getProofTrace,
  getProofTraceGaps,
} from '../../../../scripts/dev/lib/log-query-service.mjs';
import {
  resolvePortalProofTraceConfig,
  sendPortalProofTraceLog,
} from '../../src/dev-logger';

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
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

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

  it('sendPortalProofTraceLog: writes queryable ordered proof-trace rows', async () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'portal-proof-trace-'));
    tempDirs.push(tempDir);
    process.env[LOG_ROOT_ENV] = tempDir;

    const server = createBridgeServer({ rootDir: tempDir });
    servers.push(server);
    await new Promise<void>((resolve) => {
      server.listen(0, '127.0.0.1', () => resolve());
    });
    const address = server.address() as AddressInfo;
    const endpoint = `http://127.0.0.1:${address.port}`;
    const proofId = 'proof-portal-click';

    const routeOpened = await sendPortalProofTraceLog(
      DevLogMessage.PortalStarted,
      {
        proofId,
        traceStep: 'portal.route.opened',
        eventType: 'route',
        status: 'ok',
        expectedNext: 'portal.action.clicked',
      },
      {}
    ,
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
        correlationId: 'portal-proof-correlation',
      },
      {
        renderState: 'visible',
      },
      endpoint
    );

    expect(routeOpened).toBe(true);
    expect(actionClicked).toBe(true);
    expect(uiRendered).toBe(true);

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

    const gaps = await getProofTraceGaps({
      scope: 'parent-portal',
      proofId,
      expectedSteps: [
        'portal.route.opened',
        'portal.action.clicked',
        'portal.ui.rendered',
      ],
      limit: 10,
    });

    expect(gaps.missingSteps).toEqual([]);
    expect(gaps.outOfOrderSteps).toEqual([]);

    const missingStepResult = await getProofTraceGaps({
      scope: 'parent-portal',
      proofId,
      expectedSteps: [
        'portal.route.opened',
        'portal.command.sent',
      ],
      limit: 10,
    });

    expect(missingStepResult.missingSteps).toEqual(['portal.command.sent']);
  });
});
