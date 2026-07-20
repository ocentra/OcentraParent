import { expect, it } from 'vitest';
import {
  applyParentRouteEvents,
  applyParentRouteSnapshot,
  applyParentSubscriptionEvent,
  beginParentRouteLoad,
  createPortalRuntimeState,
} from '../../src/portal-state';
import type { ParentRouteEventSnapshot, ParentRouteSnapshot } from '../../generated/parent-ui-bridge';

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

function replayEvent(
  eventId: string,
  event: string,
  sentAt: string,
  previousEventId: string | null
): ParentRouteEventSnapshot {
  return {
    event,
    eventId,
    correlationId: 'lan-scan-1',
    sentAt,
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: {
      schemaVersion: 1,
      eventId,
      eventKind: event,
      occurredAt: sentAt,
      previousEventId,
      scanSessionId: 'lan-scan-1',
      affectedDeviceId: event === 'device-found' ? 'network-neighbor-1' : null,
      evidenceId: event === 'device-found' ? 'evidence-1' : null,
      summary: `LAN replay ${event}`,
    },
    snapshot: null,
  };
}

function devicesSnapshotWithReplayHistory(
  replayRows: readonly ParentRouteEventSnapshot[],
  historyOverrides: Readonly<Record<string, unknown>> = {}
): ParentRouteSnapshot {
  const latest = replayRows.at(-1);
  return {
    ...currentSubscribedDevicesRouteSnapshot,
    liveActivity: {
      lanAddDeviceReadModel: {
        discoveryEventHistory: {
          schemaVersion: 1,
          generatedAt: currentSubscribedDevicesRouteSnapshot.generatedAt,
          state: replayRows.length === 0 ? 'empty' : 'ready',
          latestEventId: latest?.eventId ?? null,
          latestObservedAt: latest?.sentAt ?? null,
          rows: replayRows.map((row) => ({
            schemaVersion: 1,
            eventId: row.eventId,
            eventKind: row.event,
            occurredAt: row.sentAt,
            previousEventId: row.payload?.['previousEventId'] ?? null,
            scanSessionId: row.payload?.['scanSessionId'] ?? null,
            affectedDeviceId: row.payload?.['affectedDeviceId'] ?? null,
            evidenceId: row.payload?.['evidenceId'] ?? null,
            summary: row.payload?.['summary'] ?? '',
          })),
          ...historyOverrides,
        },
      },
    },
  } as unknown as ParentRouteSnapshot;
}

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

it('applyParentSubscriptionEvent: buffers Rust-replayed LAN stream rows newest first', () => {
  const state = createPortalRuntimeState();
  const replayRows = [
    replayEvent('lan-history-1', 'scan-started', '2026-06-28T17:00:01Z', null),
    replayEvent('lan-history-2', 'device-found', '2026-06-28T17:00:02Z', 'lan-history-1'),
  ];
  const snapshot = devicesSnapshotWithReplayHistory(replayRows);

  applyParentSubscriptionEvent(state, {
    schemaVersion: 1,
    route: 'devices',
    snapshot,
    events: replayRows,
  });

  expect(state.events.map((event) => event.eventId)).toEqual(['lan-history-2', 'lan-history-1']);
  expect(state.events[0]?.payload?.['previousEventId']).toBe('lan-history-1');
  expect(state.routeSnapshot).toBe(snapshot);
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

it('applyParentSubscriptionEvent: binds replay rows while accepting a realistic later status event', () => {
  const state = createPortalRuntimeState();
  const replayRows = [
    replayEvent('lan-history-1', 'scan-started', '2026-06-28T17:00:01Z', null),
    replayEvent('lan-history-2', 'device-found', '2026-06-28T17:00:02Z', 'lan-history-1'),
  ];
  applyParentSubscriptionEvent(state, {
    schemaVersion: 1,
    route: 'devices',
    snapshot: devicesSnapshotWithReplayHistory(replayRows),
    events: [
      ...replayRows,
      { ...currentSubscribedDevicesRouteEvent, eventId: 'status-after-snapshot', sentAt: '2026-06-28T17:00:11Z' },
    ],
  });

  expect(state.events.map((event) => event.eventId)).toEqual([
    'status-after-snapshot',
    'lan-history-2',
    'lan-history-1',
  ]);
});

function invalidLanReplayBatches(
  validReplay: ParentRouteEventSnapshot,
  replayWithoutTimestamp: ParentRouteEventSnapshot
): ReadonlyArray<{
  snapshot: ParentRouteSnapshot;
  events: readonly ParentRouteEventSnapshot[];
}> {
  const validSnapshot = devicesSnapshotWithReplayHistory([validReplay]);
  return [
    {
      snapshot: validSnapshot,
      events: [replayWithoutTimestamp],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, sentAt: 'not-rfc3339' }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, sentAt: '2026-06-28' }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, sentAt: '2026-06-28T17:00:01' }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, sentAt: '2026-02-30T17:00:01Z' }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, payload: null }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, eventId: 'substituted-replay-id', payload: null }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, eventId: 'substituted-replay-id', payload: {} }],
    },
    { snapshot: validSnapshot, events: [{ ...validReplay, sourcePeerId: '' }] },
    { snapshot: validSnapshot, events: [{ ...validReplay, sourcePeerId: 'spoofed-agent' }] },
    { snapshot: validSnapshot, events: [{ ...validReplay, sourceRole: 'portal' }] },
    { snapshot: validSnapshot, events: [{ ...validReplay, targetPeerId: 'spoofed-portal' }] },
    { snapshot: validSnapshot, events: [{ ...validReplay, targetRole: 'agent-service' }] },
    { snapshot: validSnapshot, events: [{ ...validReplay, severity: 'warning' }] },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, payload: { ...validReplay.payload, occurredAt: '2026-06-28T17:00:00Z' } }],
    },
    {
      snapshot: devicesSnapshotWithReplayHistory([validReplay], { latestEventId: 'different-history-row' }),
      events: [validReplay],
    },
    { snapshot: validSnapshot, events: [{ ...validReplay, payload: { ...validReplay.payload, schemaVersion: 2 } }] },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, payload: { ...validReplay.payload, scanSessionId: 'different-scan' } }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, payload: { ...validReplay.payload, affectedDeviceId: 'different-device' } }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, payload: { ...validReplay.payload, evidenceId: 'different-evidence' } }],
    },
    {
      snapshot: validSnapshot,
      events: [{ ...validReplay, payload: { ...validReplay.payload, summary: 'different summary' } }],
    },
  ];
}

it('applyParentSubscriptionEvent: fails closed for missing, invalid, or mismatched replay metadata', () => {
  const validReplay = replayEvent('lan-history-1', 'scan-started', '2026-06-28T17:00:01Z', null);
  const replayWithoutTimestamp = {
    event: 'scan-started',
    eventId: 'lan-history-1',
    correlationId: 'lan-scan-1',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: validReplay.payload ?? null,
    snapshot: null,
  } satisfies ParentRouteEventSnapshot;
  const invalidBatches = invalidLanReplayBatches(validReplay, replayWithoutTimestamp);

  for (const invalid of invalidBatches) {
    const state = createPortalRuntimeState();
    applyParentSubscriptionEvent(state, {
      schemaVersion: 1,
      route: 'devices',
      snapshot: invalid.snapshot,
      events: invalid.events,
    });
    expect(state.events).toEqual([]);
  }
});

it('applyParentRouteEvents: replaying a 129-row full history preserves the identical newest 128 rows', () => {
  const state = createPortalRuntimeState();
  const history = Array.from(
    { length: 129 },
    (_, index): ParentRouteEventSnapshot => ({
      ...currentSubscribedDevicesRouteEvent,
      eventId: `full-history-${index.toString().padStart(3, '0')}`,
      correlationId: `full-history-correlation-${index.toString().padStart(3, '0')}`,
      sentAt: new Date(Date.UTC(2026, 5, 28, 17, 0, index)).toISOString(),
    })
  );
  const expectedNewest = history
    .slice(-128)
    .reverse()
    .map((event) => event.eventId);

  applyParentRouteEvents(state, history);
  expect(state.events.map((event) => event.eventId)).toEqual(expectedNewest);
  applyParentRouteEvents(state, history);
  expect(state.events.map((event) => event.eventId)).toEqual(expectedNewest);
});

it('applyParentSubscriptionEvent: buffers one row per safe host replay warning episode', () => {
  const state = createPortalRuntimeState();
  const statusHistory = [replayEvent('lan-history-1', 'scan-started', '2026-06-28T17:00:01Z', null)];
  const warning: ParentRouteEventSnapshot = {
    event: 'lan-runtime-event-chain-replay-rejected',
    eventId: 'lan-runtime-event-chain-replay-rejected-host-1',
    correlationId: null,
    sentAt: '2026-06-28T17:00:09Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'warn',
    payload: null,
    snapshot: null,
  };
  const subscription = {
    schemaVersion: 1,
    route: 'devices',
    snapshot: devicesSnapshotWithReplayHistory(statusHistory),
    events: [warning],
  } as const;

  applyParentSubscriptionEvent(state, subscription);
  applyParentSubscriptionEvent(state, subscription);

  expect(state.events).toHaveLength(1);
  expect(state.events[0]).toBe(warning);

  const laterWarning = {
    ...warning,
    eventId: 'lan-runtime-event-chain-replay-rejected-host-2',
    sentAt: '2026-06-28T17:01:09Z',
  } as const;
  const laterSubscription = { ...subscription, events: [laterWarning] } as const;

  applyParentSubscriptionEvent(state, laterSubscription);
  applyParentSubscriptionEvent(state, laterSubscription);

  expect(
    state.events
      .filter((event) => event.event === 'lan-runtime-event-chain-replay-rejected')
      .map((event) => event.eventId)
  ).toEqual(['lan-runtime-event-chain-replay-rejected-host-2', 'lan-runtime-event-chain-replay-rejected-host-1']);
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
