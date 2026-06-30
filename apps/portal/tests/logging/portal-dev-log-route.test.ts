import { createServer } from 'node:http';
import type { AddressInfo } from 'node:net';
import { afterEach, describe, expect, it } from 'vitest';
import {
  GeneratedDevLogBridge as DevLogBridge,
  GeneratedDevLogEndpoint as DevLogEndpoint,
  GeneratedDevLogMessage as DevLogMessage,
  type GeneratedLogFields as LogFields,
} from '@ocentra-parent/schema-domain/generated/logging-contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { resolvePortalDevLogBridgeUrl, sendPortalDevLog } from '../../src/dev-logger';

async function closeServer(server: ReturnType<typeof createServer>): Promise<void> {
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

describe('portal dev log routing', () => {
  const servers: Array<ReturnType<typeof createServer>> = [];

  afterEach(async () => {
    Logger.instance.reset();
    while (servers.length > 0) {
      const server = servers.pop();
      if (server != null) {
        await closeServer(server);
      }
    }
  });

  portalDevLogBridgeUrlTests();
  portalDevLogBridgeWriteTests(servers);
  portalDevLogFallbackTests(servers);
  portalDevLogUnavailableTests();
});

function portalDevLogBridgeUrlTests(): void {
  it('resolvePortalDevLogBridgeUrl: prefers injected runtime config and otherwise falls back to the bridge default', () => {
    expect(resolvePortalDevLogBridgeUrl({})).toBe(DevLogBridge.DefaultUrl);
    expect(
      resolvePortalDevLogBridgeUrl({
        [DevLogBridge.GlobalUrlKey]: 'http://127.0.0.1:4999',
      })
    ).toBe('http://127.0.0.1:4999');
  });
}

function portalDevLogBridgeWriteTests(servers: Array<ReturnType<typeof createServer>>): void {
  it('sendPortalDevLog: writes bridge-compatible portal rows through the configured local bridge', async () => {
    let healthChecks = 0;
    let receivedBody = '';
    const server = createServer((request, response) => {
      if (request.url === '/__health__') {
        healthChecks += 1;
        response.statusCode = 200;
        response.end(JSON.stringify({ ok: true }));
        return;
      }

      if (request.url === '/__logs__') {
        request.setEncoding('utf8');
        request.on('data', (chunk) => {
          receivedBody += chunk;
        });
        request.on('end', () => {
          response.statusCode = 200;
          response.end(JSON.stringify({ ok: true }));
        });
        return;
      }

      response.statusCode = 404;
      response.end();
    });
    servers.push(server);

    await new Promise<void>((resolve) => {
      server.listen(0, '127.0.0.1', () => resolve());
    });
    const address = server.address() as AddressInfo;
    const fields: LogFields = { agentWebSocketUrl: 'ws://127.0.0.1:4477/api/dev/ws' };

    const sent = await sendPortalDevLog(DevLogMessage.PortalStarted, fields, `http://127.0.0.1:${address.port}`);

    expect(sent).toBe(true);
    expect(healthChecks).toBe(1);

    const payload = JSON.parse(receivedBody) as Array<{
      consumer: string;
      testName: string;
      log: {
        source: string;
        message: string;
        context: string;
        data: string | null;
        file: string | null;
        file_path: string | null;
      };
    }>;
    expect(payload).toHaveLength(1);
    expect(payload[0]).toMatchObject({
      consumer: 'parent-portal',
      testName: 'portal-dev-runtime',
      log: {
        source: 'DevLogger',
        message: 'Portal dev runtime started.',
        context: 'DevLogger.sendPortalDevLog',
        file: 'dev-logger.ts',
      },
    });
    expect(payload[0]?.log.file_path).toContain('dev-logger.ts');
    expect(JSON.parse(payload[0]?.log.data ?? '{}')).toMatchObject(fields);
  });
}

function portalDevLogFallbackTests(servers: Array<ReturnType<typeof createServer>>): void {
  it('sendPortalDevLog: falls back to the same-origin compatibility endpoint when the bridge is unavailable', async () => {
    let receivedBody = '';
    const server = createServer((request, response) => {
      if (request.url === DevLogEndpoint.Write) {
        request.setEncoding('utf8');
        request.on('data', (chunk) => {
          receivedBody += chunk;
        });
        request.on('end', () => {
          response.statusCode = 204;
          response.end();
        });
        return;
      }

      response.statusCode = 404;
      response.end();
    });
    servers.push(server);

    await new Promise<void>((resolve) => {
      server.listen(0, '127.0.0.1', () => resolve());
    });
    const address = server.address() as AddressInfo;
    const runtime = {
      location: {
        origin: `http://127.0.0.1:${address.port}`,
      },
    } as Record<string, unknown>;
    const fields: LogFields = { agentWebSocketUrl: 'ws://127.0.0.1:4477/api/dev/ws' };

    const sent = await sendPortalDevLog(DevLogMessage.PortalStarted, fields, 'http://127.0.0.1:1', runtime);

    expect(sent).toBe(true);
    expect(JSON.parse(receivedBody)).toMatchObject({
      schemaVersion: 1,
      source: 'portal',
      message: 'Portal dev runtime started.',
      fields,
    });
  });
}

function portalDevLogUnavailableTests(): void {
  it('sendPortalDevLog: returns false when the bridge is unavailable', async () => {
    const sent = await sendPortalDevLog(DevLogMessage.PortalStarted, {}, 'http://127.0.0.1:1');

    expect(sent).toBe(false);
  });
}
