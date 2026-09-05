import { createServer, type Server } from 'node:http';
import { afterEach, describe, expect, it } from 'vitest';
import {
  ParentAgentCommand,
  ParentBridgeConnectionState,
  ParentHostBridgeRuntime,
  ParentRoute,
  ParentUiActionKind,
  type ParentUiActionResult,
} from '../../generated/parent-ui-bridge';
import { createDevWebHostBridge } from '../../src/host-bridge';
import { DirectEnforcementCommandBoundaryErrorText } from '../../src/transport';

const servers: Server[] = [];
const enforcementMutationCommands = [
  ParentAgentCommand.EnforcementExecute,
  ParentAgentCommand.EnforcementTimerRecover,
  ParentAgentCommand.EnforcementTimerExpire,
  ParentAgentCommand.EnforcementOverrideCancel,
] as const;

afterEach(async () => {
  await Promise.all(
    servers.splice(0).map(
      (server) =>
        new Promise<void>((resolve, reject) => {
          server.close((error) => (error === undefined ? resolve() : reject(error)));
        })
    )
  );
});

describe('portal command boundary', () => {
  it.each(enforcementMutationCommands)('rejects %s before dev transport serialization', async (command) => {
    let requestCount = 0;
    const server = createServer((_request, response) => {
      requestCount += 1;
      response.writeHead(500);
      response.end();
    });
    servers.push(server);
    const bridgeUrl = await listen(server);
    const bridge = createDevWebHostBridge(bridgeUrl);

    await expect(
      bridge.dispatch({
        action: ParentUiActionKind.AgentCommandRequested,
        route: ParentRoute.Commands,
        command,
        payload: {},
      })
    ).rejects.toThrow(DirectEnforcementCommandBoundaryErrorText);
    expect(requestCount).toBe(0);
  });

  it('serializes an enforcement read command through the real dev transport', async () => {
    let requestCount = 0;
    const server = createServer((_request, response) => {
      requestCount += 1;
      response.writeHead(200, { 'content-type': 'application/json' });
      response.end(
        JSON.stringify({
          schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
          accepted: true,
          connectionState: ParentBridgeConnectionState.Connected,
          message: 'enforcement-read-requested',
          snapshot: null,
          events: [],
        } satisfies ParentUiActionResult)
      );
    });
    servers.push(server);
    const bridgeUrl = await listen(server);
    const bridge = createDevWebHostBridge(bridgeUrl);

    await bridge.dispatch({
      action: ParentUiActionKind.AgentCommandRequested,
      route: ParentRoute.Commands,
      command: ParentAgentCommand.EnforcementProductControlSpineGet,
      payload: {},
    });
    expect(requestCount).toBe(1);
  });
});

async function listen(server: Server): Promise<string> {
  await new Promise<void>((resolve) => server.listen(0, '127.0.0.1', resolve));
  const address = server.address();
  if (address === null || typeof address === 'string') {
    throw new Error('Expected an ephemeral loopback server address.');
  }
  return `http://127.0.0.1:${address.port}`;
}
