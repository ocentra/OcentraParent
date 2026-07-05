import {
  type ParentAgentCommandName,
  type ParentRouteContext,
  type ParentRouteId,
  type ParentRouteSnapshot,
  type ParentSubscriptionEvent,
  type ParentUiAction,
  type ParentUiActionResult,
} from '../generated/parent-ui-bridge';
import { createPortalRuntimeDispatchHostAction } from './portal-runtime-controller-session-dispatch';
import { createPortalRuntimeLoadCurrentRoute } from './portal-runtime-controller-session-load';
import { createPortalRuntimeSubscriptionManager } from './portal-runtime-controller-session-subscription';
import { createCurrentRouteContext } from './portal-runtime-controller-session-context';
import type { PortalRuntimeState } from './portal-state';

type PortalRuntimeDeps = {
  bridge: {
    loadRoute(route: ParentRouteId, context: ParentRouteContext): Promise<ParentRouteSnapshot>;
    dispatch(action: ParentUiAction): Promise<ParentUiActionResult>;
    subscribe(
      route: ParentRouteId,
      context: ParentRouteContext,
      onEvent: (event: ParentSubscriptionEvent) => void
    ): Promise<() => void>;
  };
  state: PortalRuntimeState;
  refresh: () => void;
  getRoute: () => ParentRouteId;
};

export function createPortalRuntimeSession(
  deps: PortalRuntimeDeps,
  agentCommand: { readonly LogSnapshotGet: ParentAgentCommandName }
): {
  readonly loadCurrentRoute: () => Promise<void>;
  readonly disposeRouteSubscription: () => void;
  readonly dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>;
} {
  const currentRouteContext = createCurrentRouteContext;
  const subscription = createPortalRuntimeSubscriptionManager(deps, currentRouteContext);
  const dispatchHostAction = createPortalRuntimeDispatchHostAction(
    deps,
    currentRouteContext,
    subscription.restartRouteSubscription
  );
  const loadCurrentRoute = createPortalRuntimeLoadCurrentRoute(
    deps,
    agentCommand,
    currentRouteContext,
    dispatchHostAction,
    subscription.installRouteSubscription,
    subscription.disposeRouteSubscription
  );

  return {
    loadCurrentRoute,
    disposeRouteSubscription: subscription.disposeRouteSubscription,
    dispatchHostAction,
  };
}
