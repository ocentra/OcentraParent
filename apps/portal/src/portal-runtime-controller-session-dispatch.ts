import {
  GeneratedDevLogField as DevLogField,
  GeneratedDevLogMessage as DevLogMessage,
} from '@ocentra-parent/logging-domain/generated/logging-contracts';
import {
  ParentBridgeConnectionState,
  type ParentRouteContext,
  type ParentRouteId,
  type ParentUiAction,
  type ParentUiActionResult,
} from '../generated/parent-ui-bridge';
import { writePortalDevLog } from './dev-logger';
import { applyParentRouteEvents, applyParentRouteSnapshot, type PortalRuntimeState } from './portal-state';

type PortalRuntimeDispatchDeps = {
  bridge: {
    dispatch(action: ParentUiAction): Promise<ParentUiActionResult>;
  };
  state: PortalRuntimeState;
  refresh: () => void;
  getRoute: () => ParentRouteId;
};

export function createPortalRuntimeDispatchHostAction(
  deps: PortalRuntimeDispatchDeps,
  currentRouteContext: () => ParentRouteContext,
  restartRouteSubscription: () => Promise<void>
): (action: ParentUiAction) => Promise<ParentUiActionResult | null> {
  return async function dispatchHostAction(action: ParentUiAction): Promise<ParentUiActionResult | null> {
    const context = currentRouteContext();
    const actionWithContext = context.selectedChildDeviceId === undefined ? action : { ...action, context };
    try {
      writePortalDevLog(DevLogMessage.PortalCommandSent, {
        [DevLogField.Command]: action.command ?? action.action,
        [DevLogField.ConnectionState]: deps.state.connectionState,
      });
      const result = await deps.bridge.dispatch(actionWithContext);
      if (action.route !== deps.getRoute()) {
        return result;
      }
      deps.state.connectionState = result.connectionState;
      deps.state.commandEnabled = result.connectionState === ParentBridgeConnectionState.Connected;
      deps.state.lastHostMessage = result.message;
      applyParentRouteEvents(deps.state, result.events);
      if (result.snapshot?.route === action.route) {
        applyParentRouteSnapshot(deps.state, result.snapshot);
        deps.state.lastHostMessage = result.message;
      }
      deps.refresh();
      await restartRouteSubscription();
      return result;
    } catch (error) {
      deps.state.connectionState = ParentBridgeConnectionState.Error;
      deps.state.commandEnabled = false;
      deps.state.lastHostMessage = error instanceof Error ? error.message : String(error);
      deps.refresh();
      return null;
    }
  };
}
