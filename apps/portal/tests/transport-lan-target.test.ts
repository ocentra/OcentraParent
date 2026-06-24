import { expect, it } from 'vitest';
import { AgentCommand } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { createHostBridge } from '../src/host-bridge';

it('HostBridge dev adapter: loads honest empty route snapshots without a UI WebSocket transport', async () => {
  const bridge = createHostBridge();

  const snapshot = await bridge.loadRoute('devices');

  expect(snapshot).toMatchObject({
    schemaVersion: 1,
    route: 'devices',
    seasonLabel: 'LOCAL',
    connectionState: 'disconnected',
    commandEnabled: false,
    agentEndpoint: 'host-bridge://dev-web',
    dataSource: 'unavailable',
    parentPortalShellStatus: {
      parentAccessState: 'proof-missing',
      routeCapabilityState: 'unavailable',
      dataSourceLabel: 'unavailable',
    },
  });
  expect(snapshot.liveActivity).toBeNull();
  expect(snapshot.browserPanels).toBeNull();
});

it('HostBridge dev adapter: only exposes diagnostics chrome for explicit dev routes', async () => {
  const bridge = createHostBridge();

  const diagnosticsSnapshot = await bridge.loadRoute('diagnostics');
  const browserSnapshot = await bridge.loadRoute('browser');

  expect(diagnosticsSnapshot.dataSource).toBe('dev-diagnostics');
  expect(diagnosticsSnapshot.diagnosticPanelsEnabled).toBe(true);
  expect(browserSnapshot.dataSource).toBe('unavailable');
  expect(browserSnapshot.browserPanels).toBeNull();
});

it('HostBridge dev adapter: refuses action dispatch on the presentation-only web bridge', async () => {
  const bridge = createHostBridge();

  const result = await bridge.dispatch({
    action: 'agent-command-requested',
    route: 'devices',
    command: AgentCommand.LanPairingBrowserDiscoveryScan,
    payload: {
      [AgentProtocolDefaults.Field.LanRouteId]: AgentProtocolDefaults.Target.LocalNetworkWindowsAgent.route,
    },
  });

  expect(result).toMatchObject({
    schemaVersion: 1,
    accepted: false,
    connectionState: 'disconnected',
    snapshot: {
      route: 'devices',
      summary: {
        childDevice: 'unavailable',
      },
    },
  });
  expect(result.message).toContain('presentation-only');
});

it('HostBridge dev adapter: emits route snapshots through the shared subscription contract', async () => {
  const bridge = createHostBridge();
  const events: unknown[] = [];

  const unsubscribe = await bridge.subscribe('devices', {}, (event) => {
    events.push(event);
  });
  await Promise.resolve();
  unsubscribe();

  expect(events).toMatchObject([
    {
      schemaVersion: 1,
      route: 'devices',
      snapshot: {
        route: 'devices',
        agentEndpoint: 'host-bridge://dev-web',
        dataSource: 'unavailable',
      },
    },
  ]);
});
