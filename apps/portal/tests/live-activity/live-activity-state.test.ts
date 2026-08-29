import { describe, expect, it } from 'vitest';
import {
  ParentAgentEvent as AgentEvent,
  ParentRoute,
  type ParentNetworkRuntimeEventResultSnapshot,
  type ParentRouteLiveActivitySnapshot,
} from '../../generated/parent-ui-bridge';
import { EMPTY_ROUTE_LIVE_ACTIVITY_STATE, resolveSnapshotLiveActivityState } from '../../src/route-live-activity-state';
import {
  activityTrackingReadModelResultSnapshot,
  networkFlowReadModelSnapshot,
} from './live-activity-state-test-support';
import { createParentPortalActivityUiIntent } from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import {
  AppGameInventorySessionDashboard,
  shouldRenderAppGameInventorySessionDashboard,
} from '../../src/ParentPortalRoute';

type ResolvedLiveActivityState = ReturnType<typeof resolveSnapshotLiveActivityState>;

const rustOwnedRouteSnapshotValue: ParentRouteLiveActivitySnapshot = {
  recentSummary: {
    returned: 1,
    mostRecentSubjectName: 'Child Laptop',
  },
  browserManagedEvent: {
    event: AgentEvent.BrowserManagedStatusReported,
    eventId: 'evt-rust-browser-managed',
    sentAt: '2026-06-23T00:00:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: {
      reason: 'Managed browser session is ready.',
    },
  },
  browserManagedStatus: {
    managedState: 'managed',
    capabilityStatus: 'available',
  },
  localAiRuntimeStatusEvent: {
    event: AgentEvent.LocalAiRuntimeStatusReported,
    eventId: 'evt-rust-local-ai',
    sentAt: '2026-06-23T00:00:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: {
      reason: 'Local AI runtime is ready.',
    },
  },
  networkFlowEvent: {
    event: AgentEvent.NetworkFlowReadModelReported,
    eventId: 'evt-rust-network-flow',
    sentAt: '2026-06-23T00:00:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: {},
  },
  networkFlowReadModel: networkFlowReadModelSnapshot(),
  appGameNotificationParentSurfacePanel: {
    eyebrow: 'Rust-owned panel',
    title: 'App/game notification parent surface',
    body: 'Rust-owned notification parent-surface rows are rendered directly in the portal.',
    state: 'manual-required',
    summary: '1 manual action',
    productClaim: 'Provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed.',
    metrics: [
      { label: 'Rows returned', value: '2' },
      { label: 'Status', value: '1 manual action' },
    ],
    rows: [
      {
        key: 'app-game-notification-parent-surface-time-limit',
        title: 'app-game-notification-parent-surface-time-limit',
        details: [{ label: 'Runtime reference', value: 'scheduler-entry-app-game-time-limit' }],
      },
    ],
    emptyMessage: 'No app/game notification parent-surface panel has been reported yet.',
  },
  appGamePolicyReadinessPanel: {
    eyebrow: 'Rust-owned panel',
    title: 'App/game policy readiness',
    body: 'Rust-owned policy readiness rows are rendered directly in the portal.',
    loadState: 'ready',
    summaryDetails: [{ label: 'Manual review', value: 'Manual required' }],
    rows: [{ title: 'Policy evidence', details: [{ label: 'Reason', value: 'Ready' }] }],
    emptyMessage: 'No app/game policy readiness panel has been reported yet.',
    productClaim: 'Approval workflow, category routing, and adapter dispatch remain unclaimed.',
  },
  appGamePlatformProofStatusPanel: {
    eyebrow: 'Rust-owned panel',
    title: 'App/game platform proof status',
    body: 'Rust-owned platform proof rows are rendered directly in the portal.',
    loadState: 'ready',
    summaryDetails: [{ label: 'Platform proofs', value: '2' }],
    rows: [{ title: 'windows', details: [{ label: 'Host capability', value: 'available' }] }],
    emptyMessage: 'No app/game platform proof-status panel has been reported yet.',
    productClaim: 'Broad blocking, platform enforcement, and child delivery remain unclaimed.',
  },
  appGameChildRuntimeTransportReceiptPanel: {
    eyebrow: 'Rust-owned panel',
    title: 'App/game child runtime transport receipt',
    body: 'Rust-owned child runtime transport rows are rendered directly in the portal.',
    loadState: 'ready',
    summaryDetails: [{ label: 'Transport rows', value: '2' }],
    rows: [
      {
        title: 'app-game-child-runtime-transport-receipt-warning',
        details: [{ label: 'Boundary state', value: 'child-runtime-transport-required' }],
      },
    ],
    emptyMessage: 'No app/game child runtime transport receipt panel has been reported yet.',
    productClaim: 'Runtime transport, runtime receipt, and provider delivery remain unclaimed.',
  },
  activityTrackingReadModelEvent: {
    event: AgentEvent.ActivityTrackingReadModelReported,
    eventId: 'evt-rust-tracking',
    sentAt: '2026-06-23T00:00:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: null,
  },
  activityTrackingReadModel: activityTrackingReadModelResultSnapshot(),
};

describe('portal live activity state', () => {
  it('mounts the dashboard only on the app and game sessions route', () => {
    expect(shouldRenderAppGameInventorySessionDashboard(ParentRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameInventorySessionDashboard(ParentRoute.Activity)).toBe(false);
    expect(shouldRenderAppGameInventorySessionDashboard(ParentRoute.Diagnostics)).toBe(false);
  });

  it('renders the mounted dashboard HTML for ready and unavailable service states', () => {
    const readyState = resolveSnapshotLiveActivityState({
      ...rustOwnedRouteSnapshot(),
      activityAppUseReadModel: {
        ok: true,
        state: 'ready',
        value: {
          state: 'ready',
          rows: [
            {
              rowId: 'app-1',
              appName: 'Study Timer',
              inventoryState: 'installed',
              runtimeState: 'running',
              foregroundState: 'foreground',
              runningRowCount: 1,
              foregroundRowCount: 1,
              inventoryRowCount: 1,
              launchCount: 3,
              totalMs: 120000,
              dailyRollupCount: 2,
              lastObservedAt: '2026-08-29T12:00:00Z',
              evidence: [{ evidenceId: 'evidence-app-1' }],
            },
          ],
        },
      },
      activityGamesReadModel: { ok: true, state: 'ready', value: { state: 'ready', rows: [] } },
    });
    const readyHtml = renderToStaticMarkup(
      createElement(AppGameInventorySessionDashboard, { activityState: readyState })
    );
    expect(readyHtml).toContain('aria-label="App inventory and running sessions"');
    expect(readyHtml).toContain('Study Timer');
    expect(readyHtml).toContain('inventory 1; running 1; foreground 1; 3 launches; 2 min');
    expect(readyHtml).toContain('2 daily rollups; 1 evidence refs');
    expect(readyHtml).toContain('Capability and authority');
    expect(readyHtml).toContain('Evidence details');
    expect(readyHtml).toContain('data-ocentra-app-game-dashboard-state="ready"');

    const unavailableHtml = renderToStaticMarkup(
      createElement(AppGameInventorySessionDashboard, {
        activityState: resolveSnapshotLiveActivityState(null),
      })
    );
    expect(unavailableHtml).toContain('No app/game read model rows reported by the local service.');
    expect(unavailableHtml).toContain('data-ocentra-app-game-dashboard-state="unavailable"');
  });

  it('derives the mounted app inventory/session dashboard from service-backed states', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityAppUseReadModel: {
          ok: true,
          state: 'ready',
          value: {
            state: 'ready',
            rows: [
              {
                rowId: 'app-1',
                appName: 'Study Timer',
                inventoryState: 'installed',
                runtimeState: 'running',
                foregroundState: 'foreground',
                runningRowCount: 1,
                foregroundRowCount: 1,
                inventoryRowCount: 1,
              },
            ],
          },
        },
        activityGamesReadModel: { ok: true, state: 'ready', value: { state: 'ready', rows: [] } },
      },
      0
    );

    expect(intent.appGameDashboard.rows[0]?.label).toBe('Study Timer');
    expect(intent.appGameDashboard.rows[0]?.inventoryCount).toBe(1);
    expect(intent.appGameDashboard.rows[0]?.runningCount).toBe(1);
    expect(intent.appGameDashboard.rows[0]?.foregroundCount).toBe(1);
    expect(intent.appGameDashboard.emptyMessage).toContain('No app/game read model rows');
  });

  it('overlays Rust-owned route snapshot activity values without requiring raw agent events', () => {
    const state = resolveSnapshotLiveActivityState(rustOwnedRouteSnapshot());

    expectRustOwnedSnapshotOverlay(state);
  });

  it('keeps the product snapshot path empty when Rust has not supplied route activity yet', () => {
    const state = resolveSnapshotLiveActivityState(null);

    expect(state.recentSummary).toBeNull();
    expect(state.networkFlowEvent).toBeNull();
    expect(state.browserManagedStatus).toBeNull();
    expect(state.localAiRuntimeStatusEvent).toBeNull();
    expect(state.networkRuntimeEventChainStream).toBeNull();
  });

  it('keeps the empty route live activity state aligned with the shared portal-domain defaults', () => {
    expect(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine.dataOwner).toBe('rust-service-read-model');
    expect(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine.uiConsumer).toBe('c-owned-activity-ui');
    expect(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine.viteDataOwner).toBe(false);
    expect(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine.currentState).toBe('unavailable');
  });

  it('surfaces Rust-owned runtime event-chain snapshots directly', () => {
    const runtimeEventValue = {
      aiAnalysisRef: 'analysis.network.flow.observed.1',
    } as const;
    const runtimeEvent = {
      ok: true,
      eventType: 'network.flow.observed',
      value: runtimeEventValue,
    } satisfies ParentNetworkRuntimeEventResultSnapshot;
    const state = resolveSnapshotLiveActivityState({
      networkRuntimeEventChainStream: {
        streamedEventCount: 1,
        invalidEventCount: 0,
        events: [runtimeEvent],
      },
    });

    expect(state.networkRuntimeEventChainStream).toEqual({
      streamedEventCount: 1,
      invalidEventCount: 0,
      events: [
        {
          ok: true,
          eventType: 'network.flow.observed',
          value: runtimeEventValue,
        },
      ],
    });
  });
});

function rustOwnedRouteSnapshot(): ParentRouteLiveActivitySnapshot {
  return rustOwnedRouteSnapshotValue;
}

function expectRustOwnedSnapshotOverlay(state: ResolvedLiveActivityState): void {
  expectRustOwnedStateSummary(state);
  expectRustOwnedTrackingState(state);
  expectRustOwnedAppGamePanels(state);
}

function expectRustOwnedStateSummary(state: ResolvedLiveActivityState): void {
  expect(state.recentSummary?.mostRecentSubjectName).toBe('Child Laptop');
  expect(state.browserManagedEvent?.event).toBe(AgentEvent.BrowserManagedStatusReported);
  expect(state.browserManagedStatus?.managedState).toBe('managed');
  expect(state.localAiRuntimeStatusEvent?.event).toBe(AgentEvent.LocalAiRuntimeStatusReported);
  expect(state.networkFlowEvent?.eventId).toBe('evt-rust-network-flow');
  expect(state.networkFlowReadModel?.rows.at(0)?.destinationDomain).toBe('example-network.test');
}

function expectRustOwnedTrackingState(state: ResolvedLiveActivityState): void {
  expect(state.activityTrackingReadModelEvent?.eventId).toBe('evt-rust-tracking');
  expect(state.activityTrackingReadModel?.ok ? state.activityTrackingReadModel.value.rows[0]?.deviceId : null).toBe(
    'child-device-1'
  );
}

function expectRustOwnedAppGamePanels(state: ResolvedLiveActivityState): void {
  const notificationPanel = state.appGameNotificationParentSurfacePanel as {
    readonly summary: string;
    readonly rows: readonly { readonly title: string }[];
  } | null;
  const policyPanel = state.appGamePolicyReadinessPanel as {
    readonly title: string;
    readonly summaryDetails: readonly { readonly value: string }[];
  } | null;
  const platformPanel = state.appGamePlatformProofStatusPanel as {
    readonly rows: readonly { readonly title: string }[];
  } | null;
  const childPanel = state.appGameChildRuntimeTransportReceiptPanel as {
    readonly rows: readonly { readonly title: string }[];
  } | null;

  expect(notificationPanel?.summary).toBe('1 manual action');
  expect(notificationPanel?.rows[0]?.title).toBe('app-game-notification-parent-surface-time-limit');
  expect(policyPanel?.title).toBe('App/game policy readiness');
  expect(policyPanel?.summaryDetails[0]?.value).toBe('Manual required');
  expect(platformPanel?.rows[0]?.title).toBe('windows');
  expect(childPanel?.rows[0]?.title).toBe('app-game-child-runtime-transport-receipt-warning');
}
