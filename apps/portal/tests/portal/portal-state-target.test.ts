import { expect, it } from 'vitest';
import {
  applyParentRouteEvents,
  applyParentRouteSnapshot,
  applyParentSubscriptionEvent,
  beginParentRouteLoad,
  createPortalRuntimeState,
} from '../../src/portal-state';

const currentSubscribedDevicesRouteSnapshot = {
  schemaVersion: 1,
  route: 'devices',
  generatedAt: '2026-06-28T17:00:10Z',
  seasonLabel: 'LOCAL',
  lastUpdated: '2026-06-28T17:00:10Z',
  connectionState: 'connected',
  commandEnabled: true,
  agentEndpoint: 'host-bridge://tauri-parent',
  dataSource: 'rust-read-model',
  diagnosticPanelsEnabled: false,
  parentPortalRows: [],
  parentPortalShellStatus: {
    routeLabel: 'Devices',
    parentAccessState: 'active-controller',
    globalConnectionState: 'connected',
    routeCapabilityState: 'available',
    dataSourceLabel: 'rust-read-model',
    cards: [],
  },
  summary: {
    title: 'Devices',
    routeCapability: 'available',
    parentAccess: 'paired',
    household: 'available',
    childDevice: 'paired',
  },
} as const;

const currentSubscribedDevicesRouteEvent = {
  event: 'agent.lan-pairing.status.reported',
  eventId: 'evt-lan-status-newer',
  correlationId: 'corr-lan-status-newer',
  sentAt: '2026-06-28T17:00:10Z',
  sourcePeerId: 'local-dev-agent',
  sourceRole: 'agent-service',
  targetPeerId: 'portal-dev',
  targetRole: 'portal',
  severity: 'info',
  payload: { route: 'devices', freshness: 'newer' },
  snapshot: null,
} as const;

const staleSubscribedDevicesRouteSnapshot = {
  schemaVersion: 1,
  route: 'devices',
  generatedAt: '2026-06-28T17:00:05Z',
  seasonLabel: 'LOCAL',
  lastUpdated: '2026-06-28T17:00:05Z',
  connectionState: 'error',
  commandEnabled: false,
  agentEndpoint: 'host-bridge://stale-parent',
  dataSource: 'rust-read-model',
  diagnosticPanelsEnabled: false,
  parentPortalRows: [],
  parentPortalShellStatus: {
    routeLabel: 'Devices',
    parentAccessState: 'proof-missing',
    globalConnectionState: 'error',
    routeCapabilityState: 'degraded',
    dataSourceLabel: 'rust-read-model',
    cards: [],
  },
  summary: {
    title: 'Devices',
    routeCapability: 'degraded',
    parentAccess: 'proof-missing',
    household: 'manual-required',
    childDevice: 'manual-required',
  },
} as const;

const staleSubscribedDevicesRouteEvent = {
  event: 'agent.lan-pairing.status.reported',
  eventId: 'evt-lan-status-stale',
  correlationId: 'corr-lan-status-stale',
  sentAt: '2026-06-28T17:00:05Z',
  sourcePeerId: 'local-dev-agent',
  sourceRole: 'agent-service',
  targetPeerId: 'portal-dev',
  targetRole: 'portal',
  severity: 'warning',
  payload: { route: 'devices', freshness: 'stale' },
  snapshot: null,
} as const;

it('createPortalRuntimeState: starts disconnected until the host bridge supplies a snapshot', () => {
  const state = createPortalRuntimeState();

  expect(state.agentEndpoint).toBe('host-bridge://pending');
  expect(state.commandEnabled).toBe(false);
  expect(state.routeSnapshot).toBeNull();
});

it('beginParentRouteLoad: clears rows from a different route before the next bridge response arrives', () => {
  const state = createPortalRuntimeState();
  applyParentRouteSnapshot(state, currentSubscribedDevicesRouteSnapshot);

  beginParentRouteLoad(state, 'browser');

  expect(state.routeSnapshot).toBeNull();
  expect(state.connectionState).toBe('connecting');
  expect(state.commandEnabled).toBe(false);
});

it('beginParentRouteLoad: preserves same-route rows as stale reconnect evidence', () => {
  const state = createPortalRuntimeState();
  applyParentRouteSnapshot(state, currentSubscribedDevicesRouteSnapshot);

  beginParentRouteLoad(state, 'devices');

  expect(state.routeSnapshot).toBe(currentSubscribedDevicesRouteSnapshot);
  expect(state.connectionState).toBe('connecting');
  expect(state.commandEnabled).toBe(false);
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

it('applyParentRouteEvents: buffers returned host-bridge events newest first and updates the latest log snapshot', () => {
  const state = createPortalRuntimeState();

  applyParentRouteEvents(state, [
    {
      event: 'agent.connection.ready',
      eventId: 'evt-ready',
      correlationId: 'corr-ready',
      sentAt: '2026-06-24T00:00:00Z',
      sourcePeerId: 'local-dev-agent',
      sourceRole: 'agent-service',
      targetPeerId: 'portal-dev',
      targetRole: 'portal',
      severity: 'info',
      payload: {},
      snapshot: null,
    },
    {
      event: 'agent.log.snapshot.reported',
      eventId: 'evt-log-snapshot',
      correlationId: 'corr-log-snapshot',
      sentAt: '2026-06-24T00:00:01Z',
      sourcePeerId: 'local-dev-agent',
      sourceRole: 'agent-service',
      targetPeerId: 'portal-dev',
      targetRole: 'portal',
      severity: 'info',
      payload: {},
      snapshot: {
        schemaVersion: 1,
        agent: {
          deviceId: 'child-device-1',
          hostname: 'study-laptop',
          platform: 'windows',
          serviceVersion: '0.1.1',
        },
        entries: [],
      },
    },
  ]);

  expect(state.events).toHaveLength(2);
  expect(state.events[0]?.event).toBe('agent.log.snapshot.reported');
  expect(state.events[1]?.event).toBe('agent.connection.ready');
  expect(state.latestSnapshot?.agent.hostname).toBe('study-laptop');
});

it('applyParentRouteEvents: replaying the same host-bridge events does not duplicate buffered portal cards', () => {
  const state = createPortalRuntimeState();
  const snapshots = [
    {
      event: 'agent.connection.ready',
      eventId: 'evt-ready',
      correlationId: 'corr-ready',
      sentAt: '2026-06-24T00:00:00Z',
      sourcePeerId: 'local-dev-agent',
      sourceRole: 'agent-service',
      targetPeerId: 'portal-dev',
      targetRole: 'portal',
      severity: 'info',
      payload: {},
      snapshot: null,
    },
    {
      event: 'agent.log.snapshot.reported',
      eventId: 'evt-log-snapshot',
      correlationId: 'corr-log-snapshot',
      sentAt: '2026-06-24T00:00:01Z',
      sourcePeerId: 'local-dev-agent',
      sourceRole: 'agent-service',
      targetPeerId: 'portal-dev',
      targetRole: 'portal',
      severity: 'info',
      payload: {},
      snapshot: {
        schemaVersion: 1,
        agent: {
          deviceId: 'child-device-1',
          hostname: 'study-laptop',
          platform: 'windows',
          serviceVersion: '0.1.1',
        },
        entries: [],
      },
    },
  ] as const;

  applyParentRouteEvents(state, snapshots);
  applyParentRouteEvents(state, snapshots);

  expect(state.events).toHaveLength(2);
  expect(state.events[0]?.eventId).toBe('evt-log-snapshot');
  expect(state.events[1]?.eventId).toBe('evt-ready');
  expect(state.latestSnapshot?.agent.hostname).toBe('study-laptop');
});

it('applyParentRouteEvents: ignores incomplete host-bridge event snapshots instead of inventing event identity', () => {
  const state = createPortalRuntimeState();

  applyParentRouteEvents(state, [
    {
      event: 'agent.connection.ready',
      eventId: 'evt-ready-missing-sent-at',
      sourcePeerId: 'local-dev-agent',
      sourceRole: 'agent-service',
      targetPeerId: 'portal-dev',
      targetRole: 'portal',
      severity: 'info',
      payload: {},
      snapshot: null,
    },
  ]);

  expect(state.events).toHaveLength(0);
  expect(state.latestSnapshot).toBeNull();
});

it('applyParentSubscriptionEvent: applies subscribed route events before the latest Rust snapshot', () => {
  const state = createPortalRuntimeState();

  applyParentSubscriptionEvent(state, {
    schemaVersion: 1,
    route: 'devices',
    snapshot: {
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
      parentPortalRows: [],
      parentPortalShellStatus: {
        routeLabel: 'Devices',
        parentAccessState: 'active-controller',
        globalConnectionState: 'connected',
        routeCapabilityState: 'available',
        dataSourceLabel: 'rust-read-model',
        cards: [],
      },
      summary: {
        title: 'Devices',
        routeCapability: 'available',
        parentAccess: 'paired',
        household: 'available',
        childDevice: 'paired',
      },
    },
    events: [
      {
        event: 'agent.lan-pairing.status.reported',
        eventId: 'evt-lan-status',
        correlationId: 'corr-lan-status',
        sentAt: '2026-06-28T17:00:00Z',
        sourcePeerId: 'local-dev-agent',
        sourceRole: 'agent-service',
        targetPeerId: 'portal-dev',
        targetRole: 'portal',
        severity: 'info',
        payload: { route: 'devices' },
        snapshot: null,
      },
    ],
  });

  expect(state.routeSnapshot?.route).toBe('devices');
  expect(state.routeSnapshot?.dataSource).toBe('rust-read-model');
  expect(state.events).toHaveLength(1);
  expect(state.events[0]?.eventId).toBe('evt-lan-status');
  expect(state.events[0]?.event).toBe('agent.lan-pairing.status.reported');
});

it('applyParentSubscriptionEvent: rejects stale subscribed batches instead of regressing the current Rust route view', () => {
  const state = createPortalRuntimeState();

  applyParentSubscriptionEvent(state, {
    schemaVersion: 1,
    route: 'devices',
    snapshot: currentSubscribedDevicesRouteSnapshot,
    events: [currentSubscribedDevicesRouteEvent],
  });

  applyParentSubscriptionEvent(state, {
    schemaVersion: 1,
    route: 'devices',
    snapshot: staleSubscribedDevicesRouteSnapshot,
    events: [staleSubscribedDevicesRouteEvent],
  });

  expect(state.routeSnapshot?.generatedAt).toBe('2026-06-28T17:00:10Z');
  expect(state.routeSnapshot?.connectionState).toBe('connected');
  expect(state.routeSnapshot?.agentEndpoint).toBe('host-bridge://tauri-parent');
  expect(state.events).toHaveLength(1);
  expect(state.events[0]?.eventId).toBe('evt-lan-status-newer');
});

it('applyParentSubscriptionEvent: rejects a snapshot whose route disagrees with the subscription event', () => {
  const state = createPortalRuntimeState();

  applyParentSubscriptionEvent(state, {
    schemaVersion: 1,
    route: 'browser',
    snapshot: currentSubscribedDevicesRouteSnapshot,
    events: [],
  });

  expect(state.routeSnapshot).toBeNull();
});
