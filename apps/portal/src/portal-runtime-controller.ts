import {
  ParentAgentCommand as AgentCommand,
  type ParentRouteContext,
  type ParentRouteId,
  type ParentRouteSnapshot,
  type ParentSubscriptionEvent,
  type ParentUiAction,
  type ParentUiActionResult,
} from '../generated/parent-ui-bridge';
import { createPortalRuntimeActions } from './portal-runtime-controller-actions';
import { createPortalRuntimeSession } from './portal-runtime-controller-session';
import type { PortalRuntimeState } from './portal-state';
import type { PortalRenderActions } from './portal-actions';

export interface PortalRuntimeController {
  readonly actions: PortalRenderActions;
  readonly start: () => Promise<void>;
  readonly handleRouteChange: () => Promise<void>;
  readonly dispose: () => void;
}

export function createPortalRuntimeController(deps: {
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
}): PortalRuntimeController {
  const session = createPortalRuntimeSession(deps, AgentCommand);
  const actions = createPortalRuntimeActions(deps, session.dispatchHostAction);

  return {
    actions,
    start: session.loadCurrentRoute,
    handleRouteChange: session.loadCurrentRoute,
    dispose: session.disposeRouteSubscription,
  };
}
