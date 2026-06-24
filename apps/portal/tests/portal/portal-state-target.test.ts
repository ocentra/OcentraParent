import { expect, it } from 'vitest';
import { applyParentRouteSnapshot, createPortalRuntimeState } from '../../src/portal-state';

it('createPortalRuntimeState: starts disconnected until the host bridge supplies a snapshot', () => {
  const state = createPortalRuntimeState();

  expect(state.agentEndpoint).toBe('host-bridge://pending');
  expect(state.commandEnabled).toBe(false);
  expect(state.routeSnapshot).toBeNull();
});

it('applyParentRouteSnapshot: updates portal runtime state from the Rust-owned bridge snapshot', () => {
  const state = createPortalRuntimeState();

  applyParentRouteSnapshot(state, {
    schemaVersion: 1,
    route: 'devices',
    generatedAt: '',
    seasonLabel: 'LOCAL',
    lastUpdated: '',
    connectionState: 'connected',
    commandEnabled: true,
    agentEndpoint: 'host-bridge://tauri-parent',
    dataSource: 'rust-read-model',
    diagnosticPanelsEnabled: false,
    parentPortalRows: [
      {
        label: 'Local agent',
        order: 1,
        signalScore: 100,
        readyCount: 1,
        gapCount: 0,
        primaryArea: 'Runtime',
        trend: 'connected',
        tone: 'cyan',
      },
    ],
    parentPortalShellStatus: {
      routeLabel: 'Devices',
      parentAccessState: 'proof-missing',
      globalConnectionState: 'manual-required',
      routeCapabilityState: 'available',
      dataSourceLabel: 'rust-read-model',
      cards: [],
    },
    summary: {
      title: 'Devices',
      routeCapability: 'available',
      parentAccess: 'proof-missing',
      household: 'proof-missing',
      childDevice: 'proof-missing',
    },
  });

  expect(state.agentEndpoint).toBe('host-bridge://tauri-parent');
  expect(state.connectionState).toBe('connected');
  expect(state.commandEnabled).toBe(true);
  expect(state.routeSnapshot?.dataSource).toBe('rust-read-model');
  expect(state.routeSnapshot?.parentPortalRows?.[0]?.label).toBe('Local agent');
});
