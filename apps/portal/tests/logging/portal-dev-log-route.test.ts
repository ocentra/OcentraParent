import type { Server } from 'node:http';
import { afterEach, describe, expect, it } from 'vitest';
import {
  GeneratedDevLogBridge as DevLogBridge,
  GeneratedDevLogMessage as DevLogMessage,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { resolvePortalDevLogBridgeUrl, sendPortalDevLog } from '../../src/dev-logger';
import {
  closeServers,
  createBridgeWriteServer,
  createFallbackWriteServer,
  listenOnLoopback,
} from './portal-dev-log-route.test.helpers';

describe('portal dev log routing', () => {
  const servers: Server[] = [];

  afterEach(async () => {
    Logger.instance.reset();
    await closeServers(servers);
  });

  it('resolvePortalDevLogBridgeUrl: prefers injected runtime config and otherwise falls back to the bridge default', () => {
    expect(resolvePortalDevLogBridgeUrl({})).toBe(DevLogBridge.DefaultUrl);
    expect(
      resolvePortalDevLogBridgeUrl({
        [DevLogBridge.GlobalUrlKey]: 'http://127.0.0.1:4999',
      })
    ).toBe('http://127.0.0.1:4999');
  });

  it('sendPortalDevLog: fails closed when the configured bridge has no durable browser queue', async () => {
    const bridge = createBridgeWriteServer();
    servers.push(bridge.server);
    await listenOnLoopback(bridge.server);
    const sent = await sendPortalDevLog(
      DevLogMessage.PortalStarted,
      { agentWebSocketUrl: 'ws://127.0.0.1:4477/api/dev/ws' },
      `http://127.0.0.1:${(bridge.server.address() as { port: number }).port}`,
      {}
    );

    expect(sent).toBe(false);
    expect(bridge.healthChecks()).toBe(0);
    expect(bridge.readBody()).toBe('');
  });

  it('sendPortalDevLog: falls back to the same-origin compatibility endpoint when the bridge is unavailable', async () => {
    const fallback = createFallbackWriteServer();
    servers.push(fallback.server);
    await listenOnLoopback(fallback.server);
    const sent = await sendPortalDevLog(
      DevLogMessage.PortalStarted,
      { agentWebSocketUrl: 'ws://127.0.0.1:4477/api/dev/ws' },
      'http://127.0.0.1:1',
      {
        location: {
          origin: `http://127.0.0.1:${(fallback.server.address() as { port: number }).port}`,
        },
      } as Record<string, unknown>
    );

    expect(sent).toBe(true);
    expect(JSON.parse(fallback.readBody())).toMatchObject({
      schemaVersion: 1,
      source: 'portal',
      message: 'Portal dev runtime started.',
      fields: { agentWebSocketUrl: '[REDACTED]' },
    });
  });

  it('sendPortalDevLog: returns false when the bridge is unavailable', async () => {
    const sent = await sendPortalDevLog(DevLogMessage.PortalStarted, {}, 'http://127.0.0.1:1');
    expect(sent).toBe(false);
  });
});
