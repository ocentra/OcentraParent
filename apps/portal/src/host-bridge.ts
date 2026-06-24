import { PortalDevToolWindow } from '@ocentra-parent/portal-domain/routes';
import type {
  HostBridge,
  ParentBridgeConnectionState,
  ParentPortalRowSnapshot,
  ParentRouteDataSource,
  ParentRouteId,
  ParentRouteSnapshot,
  ParentSubscriptionEvent,
  ParentUiAction,
  ParentUiActionResult,
} from './generated/parent-ui-bridge';

const DEV_DIAGNOSTIC_ROUTES = new Set<ParentRouteId>([
  'diagnostics',
  'proof-panels',
  'commands',
  'events',
  'logs',
  'app-layout',
]);

const PARENT_BRIDGE_COMMAND = {
  LoadRoute: 'parent_load_route',
  Dispatch: 'parent_dispatch',
  Subscribe: 'parent_subscribe_route',
  Unsubscribe: 'parent_unsubscribe_route',
} as const;
const PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX = 'parent-route-subscription-';

type ParentBridgeCommandName = (typeof PARENT_BRIDGE_COMMAND)[keyof typeof PARENT_BRIDGE_COMMAND];
type TauriCoreModule = {
  invoke<TResult>(command: ParentBridgeCommandName, args?: Record<string, unknown>): Promise<TResult>;
};
type TauriEventModule = {
  listen<TPayload>(
    event: string,
    handler: (event: { payload: TPayload }) => void
  ): Promise<() => void>;
};

let tauriCoreModulePromise: Promise<TauriCoreModule> | null = null;
let tauriEventModulePromise: Promise<TauriEventModule> | null = null;

export function createHostBridge(): HostBridge {
  return isTauriRuntime() ? createTauriHostBridge() : createDevWebHostBridge();
}

function createTauriHostBridge(): HostBridge {
  return {
    loadRoute(route, context) {
      return invokeParentBridgeCommand<ParentRouteSnapshot>(PARENT_BRIDGE_COMMAND.LoadRoute, {
        route,
        context: context ?? null,
      });
    },
    dispatch(action) {
      return invokeParentBridgeCommand<ParentUiActionResult>(PARENT_BRIDGE_COMMAND.Dispatch, { action });
    },
    async subscribe(route, context, onEvent) {
      const subscriptionId = await invokeParentBridgeCommand<string>(PARENT_BRIDGE_COMMAND.Subscribe, {
        route,
        context: context ?? null,
      });
      const tauriEvent = await loadTauriEventModule();
      const unlisten = await tauriEvent.listen<ParentSubscriptionEvent>(
        parentRouteSubscriptionEventName(subscriptionId),
        (event) => {
          onEvent(event.payload);
        }
      );
      return () => {
        unlisten();
        void invokeParentBridgeCommand<boolean>(PARENT_BRIDGE_COMMAND.Unsubscribe, {
          subscriptionId,
        });
      };
    },
  };
}

function createDevWebHostBridge(): HostBridge {
  return {
    async loadRoute(route) {
      return createDevSnapshot(route);
    },
    async dispatch(action) {
      return {
        schemaVersion: 1,
        accepted: false,
        connectionState: 'disconnected',
        message: 'Dev web host bridge is presentation-only. Launch the Tauri app for product data and actions.',
        snapshot: createDevSnapshot(action.route),
      };
    },
    async subscribe(route, _context, onEvent) {
      const snapshot = createDevSnapshot(route);
      queueMicrotask(() => {
        onEvent({
          schemaVersion: 1,
          route,
          snapshot,
        });
      });
      return () => undefined;
    },
  };
}

function createDevSnapshot(route: ParentRouteId): ParentRouteSnapshot {
  const dataSource = devDataSource(route);
  const routeCapability = dataSource === 'dev-diagnostics' ? 'available' : 'unavailable';

  return {
    schemaVersion: 1,
    route,
    generatedAt: '',
    seasonLabel: 'LOCAL',
    lastUpdated: '',
    connectionState: 'disconnected',
    commandEnabled: false,
    agentEndpoint: 'host-bridge://dev-web',
    dataSource,
    summary: {
      title: routeTitle(route),
      routeCapability,
      parentAccess: 'proof-missing',
      household: 'unavailable',
      childDevice: 'unavailable',
    },
    diagnosticPanelsEnabled: DEV_DIAGNOSTIC_ROUTES.has(route),
    parentPortalRows: devPortalRows(dataSource),
    parentPortalShellStatus: {
      routeLabel: routeTitle(route),
      parentAccessState: 'proof-missing',
      globalConnectionState: 'disconnected',
      routeCapabilityState: routeCapability,
      dataSourceLabel: dataSource,
      cards: [
        {
          id: 'ui-bridge',
          label: 'UI bridge',
          value: 'connected',
          detail: 'The TSX shell is running without a Tauri host.',
          tone: 'cyan',
        },
        {
          id: 'product-runtime',
          label: 'Product runtime',
          value: 'manual-required',
          detail: 'Launch the desktop app to load Rust-owned route snapshots.',
          tone: 'gold',
        },
        {
          id: 'route-capability',
          label: 'Route capability',
          value: routeCapability,
          detail: dataSource === 'dev-diagnostics' ? 'Diagnostics chrome only.' : 'No product read model is attached.',
          tone: dataSource === 'dev-diagnostics' ? 'muted' : 'red',
        },
      ],
    },
    liveActivity: null,
    browserPanels: null,
    screenSettingsServiceResponse: null,
  };
}

function devPortalRows(dataSource: ParentRouteDataSource): readonly ParentPortalRowSnapshot[] {
  const routeCapability = dataSource === 'dev-diagnostics' ? 'available' : 'unavailable';
  return [
    {
      label: 'UI bridge',
      order: 1,
      signalScore: 100,
      readyCount: 1,
      gapCount: 0,
      primaryArea: 'Bridge',
      trend: 'connected',
      tone: 'cyan',
    },
    {
      label: 'Product runtime',
      order: 2,
      signalScore: 0,
      readyCount: 0,
      gapCount: 1,
      primaryArea: 'Runtime',
      trend: 'manual-required',
      tone: 'gold',
    },
    {
      label: 'Route capability',
      order: 3,
      signalScore: routeCapability === 'available' ? 100 : 0,
      readyCount: routeCapability === 'available' ? 1 : 0,
      gapCount: routeCapability === 'available' ? 0 : 1,
      primaryArea: 'Route',
      trend: routeCapability,
      tone: routeCapability === 'available' ? 'muted' : 'red',
    },
  ];
}

function devDataSource(route: ParentRouteId): ParentRouteDataSource {
  return DEV_DIAGNOSTIC_ROUTES.has(route) ? 'dev-diagnostics' : 'unavailable';
}

function routeTitle(route: ParentRouteId): string {
  return route
    .split('-')
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

async function invokeParentBridgeCommand<TResult>(
  command: ParentBridgeCommandName,
  args: Record<string, unknown>
): Promise<TResult> {
  const tauriCore = await loadTauriCoreModule();
  return tauriCore.invoke<TResult>(command, args);
}

function loadTauriCoreModule(): Promise<TauriCoreModule> {
  if (tauriCoreModulePromise === null) {
    tauriCoreModulePromise = import('@tauri-apps/api/core') as Promise<TauriCoreModule>;
  }
  return tauriCoreModulePromise;
}

function loadTauriEventModule(): Promise<TauriEventModule> {
  if (tauriEventModulePromise === null) {
    tauriEventModulePromise = import('@tauri-apps/api/event') as Promise<TauriEventModule>;
  }
  return tauriEventModulePromise;
}

function parentRouteSubscriptionEventName(subscriptionId: string): string {
  return `${PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX}${subscriptionId}`;
}

function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && PortalDevToolWindow.TauriInternalKey in window;
}
