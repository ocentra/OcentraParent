import {
  ParentAgentCommand as AgentCommand,
  ParentBridgeConnectionState,
  ParentUiActionKind,
  type ParentAgentCommandName,
  type ParentRouteContext,
  type ParentRouteId,
  type ParentRouteSnapshot,
  type ParentUiAction,
  type ParentUiActionResult,
} from '../generated/parent-ui-bridge';
import { shouldPrimeDeveloperRoute } from './portal-runtime-controller-session-context';
import type { PortalRuntimeState } from './portal-state';
import { applyParentRouteSnapshot, beginParentRouteLoad } from './portal-state';

type PortalRuntimeLoadDeps = {
  bridge: {
    loadRoute(route: ParentRouteId, context: ParentRouteContext): Promise<ParentRouteSnapshot>;
  };
  state: PortalRuntimeState;
  refresh: () => void;
  getRoute: () => ParentRouteId;
};

export function createPortalRuntimeLoadCurrentRoute(
  deps: PortalRuntimeLoadDeps,
  agentCommand: { readonly LogSnapshotGet: ParentAgentCommandName },
  currentRouteContext: () => ParentRouteContext,
  dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>,
  installRouteSubscription: (route: ParentRouteId) => Promise<void>,
  disposeRouteSubscription: () => void
): () => Promise<void> {
  let routeLoadSequence = 0;

  return async function loadCurrentRoute(): Promise<void> {
    const route = deps.getRoute();
    const sequence = routeLoadSequence + 1;
    routeLoadSequence = sequence;
    disposeRouteSubscription();
    beginParentRouteLoad(deps.state, route);
    deps.refresh();
    try {
      const snapshot = await deps.bridge.loadRoute(route, currentRouteContext());
      if (sequence !== routeLoadSequence || route !== deps.getRoute() || snapshot.route !== route) {
        return;
      }
      applyParentRouteSnapshot(deps.state, snapshot);
      deps.refresh();
      await installRouteSubscription(route);
      await maybePrimeDeveloperRoute(route);
    } catch (error) {
      if (sequence !== routeLoadSequence) {
        return;
      }
      deps.state.connectionState = ParentBridgeConnectionState.Error;
      deps.state.commandEnabled = false;
      deps.state.lastHostMessage = error instanceof Error ? error.message : String(error);
      deps.refresh();
    }
  };

  async function maybePrimeDeveloperRoute(route: ParentRouteId): Promise<void> {
    if (!shouldPrimeDeveloperRoute(route)) {
      return;
    }
    if (deps.state.events.length > 0 || deps.state.latestSnapshot !== null) {
      return;
    }

    await dispatchHostAction({
      action: ParentUiActionKind.AgentCommandRequested,
      route,
      command: agentCommand.LogSnapshotGet ?? AgentCommand.LogSnapshotGet,
      payload: {},
    });
  }
}
