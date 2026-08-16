import {
  type ParentRouteContext,
  type ParentRouteId,
  type ParentSubscriptionEvent,
} from '../generated/parent-ui-bridge';
import {
  GeneratedDevLogField as DevLogField,
  GeneratedDevLogMessage as DevLogMessage,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { writePortalDevLog } from './dev-logger';
import { applyParentSubscriptionEvent, type PortalRuntimeState } from './portal-state';

type PortalRuntimeSubscriptionDeps = {
  bridge: {
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

function createRouteSubscriptionEventHandler(
  deps: PortalRuntimeSubscriptionDeps,
  isActiveToken: () => boolean
): (event: ParentSubscriptionEvent) => void {
  return (event) => {
    if (!isActiveToken() || event.route !== deps.getRoute()) {
      return;
    }
    writePortalDevLog(DevLogMessage.PortalEventReceived, {
      [DevLogField.Event]: String(event.route),
      [DevLogField.ConnectionState]: event.snapshot.connectionState,
    });
    applyParentSubscriptionEvent(deps.state, event);
    deps.refresh();
  };
}

function finalizeRouteSubscriptionInstall(
  deps: PortalRuntimeSubscriptionDeps,
  route: ParentRouteId,
  token: number,
  routeSubscriptionToken: number,
  unsubscribe: () => void
): boolean {
  if (token !== routeSubscriptionToken || route !== deps.getRoute()) {
    unsubscribe();
    return false;
  }
  return true;
}

export function createPortalRuntimeSubscriptionManager(
  deps: PortalRuntimeSubscriptionDeps,
  currentRouteContext: () => ParentRouteContext
): {
  readonly installRouteSubscription: (route: ParentRouteId) => Promise<void>;
  readonly restartRouteSubscription: () => Promise<void>;
  readonly disposeRouteSubscription: () => void;
} {
  let routeSubscriptionToken = 0;
  let activeRouteUnsubscribe: (() => void) | null = null;

  async function installRouteSubscription(route: ParentRouteId): Promise<void> {
    const token = routeSubscriptionToken + 1;
    routeSubscriptionToken = token;
    try {
      const unsubscribe = await deps.bridge.subscribe(
        route,
        currentRouteContext(),
        createRouteSubscriptionEventHandler(deps, () => token === routeSubscriptionToken)
      );
      if (!finalizeRouteSubscriptionInstall(deps, route, token, routeSubscriptionToken, unsubscribe)) {
        return;
      }
      activeRouteUnsubscribe = unsubscribe;
    } catch (error) {
      if (token !== routeSubscriptionToken) {
        return;
      }
      deps.state.lastHostMessage = error instanceof Error ? error.message : String(error);
      deps.refresh();
    }
  }

  async function restartRouteSubscription(): Promise<void> {
    disposeRouteSubscription();
    await installRouteSubscription(deps.getRoute());
  }

  function disposeRouteSubscription(): void {
    routeSubscriptionToken += 1;
    activeRouteUnsubscribe?.();
    activeRouteUnsubscribe = null;
  }

  return {
    installRouteSubscription,
    restartRouteSubscription,
    disposeRouteSubscription,
  };
}
