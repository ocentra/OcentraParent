import {
  type ParentAgentCommandName,
  type ParentAgentEventName as AgentEventName,
  ParentUiActionKind,
  type ParentRouteId,
  type ParentUiActionPayload,
  type ParentUiAction,
  type ParentUiActionResult,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';

type PortalRuntimeActionDeps = {
  state: PortalRuntimeState;
  refresh: () => void;
  getRoute: () => ParentRouteId;
};

function createReconnectAction(
  deps: PortalRuntimeActionDeps,
  dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>
): () => void {
  return () => {
    void dispatchHostAction({
      action: ParentUiActionKind.RefreshRoute,
      route: deps.getRoute(),
      payload: {},
    });
  };
}

function createSelectCommandResultAction(deps: PortalRuntimeActionDeps): (resultEvent: AgentEventName) => void {
  return (resultEvent) => {
    deps.state.selectedCommandResultEvent = resultEvent;
    deps.refresh();
  };
}

function createSendCommandAction(
  deps: PortalRuntimeActionDeps,
  dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>
): (command: ParentAgentCommandName, payload: ParentUiActionPayload) => Promise<ParentUiActionResult | null> {
  return (command, payload) => {
    return dispatchHostAction({
      action: ParentUiActionKind.AgentCommandRequested,
      route: deps.getRoute(),
      command,
      payload,
    });
  };
}

function createRefreshRouteSnapshotAction(
  deps: PortalRuntimeActionDeps,
  dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>
): () => Promise<ParentUiActionResult | null> {
  return () => {
    return dispatchHostAction({
      action: ParentUiActionKind.RefreshRoute,
      route: deps.getRoute(),
      payload: {},
    });
  };
}

function createDispatchAction(
  action: ParentUiActionKind,
  deps: PortalRuntimeActionDeps,
  dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>,
  payload: ParentUiActionPayload
): Promise<ParentUiActionResult | null> {
  return dispatchHostAction({
    action,
    route: deps.getRoute(),
    payload,
  });
}

function createDispatchActionWithPayload(
  action: ParentUiActionKind,
  deps: PortalRuntimeActionDeps,
  dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>,
  payload: ParentUiActionPayload
): Promise<ParentUiActionResult | null> {
  return createDispatchAction(action, deps, dispatchHostAction, payload);
}

function createPortalRuntimeRequestActions(
  deps: PortalRuntimeActionDeps,
  dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>
) {
  return {
    requestLanPairingBrowserDiscoveryScan: () =>
      createDispatchActionWithPayload(
        ParentUiActionKind.LanPairingBrowserDiscoveryScanRequested,
        deps,
        dispatchHostAction,
        {}
      ),
    requestNetworkFlowReadModelRefresh: () =>
      createDispatchActionWithPayload(
        ParentUiActionKind.NetworkFlowReadModelRefreshRequested,
        deps,
        dispatchHostAction,
        {}
      ),
    requestTrackingRetentionSettingsWrite: () =>
      createDispatchActionWithPayload(
        ParentUiActionKind.TrackingRetentionSettingsWriteRequested,
        deps,
        dispatchHostAction,
        {}
      ),
    stagePolicyPreviewAuthoringDraft: (payload: ParentUiActionPayload) =>
      createDispatchActionWithPayload(
        ParentUiActionKind.PolicyPreviewAuthoringDraftStaged,
        deps,
        dispatchHostAction,
        payload
      ),
    cancelPolicyPreviewAuthoringDraft: (payload: ParentUiActionPayload) =>
      createDispatchActionWithPayload(
        ParentUiActionKind.PolicyPreviewAuthoringDraftCancelled,
        deps,
        dispatchHostAction,
        payload
      ),
    requestPolicyRequestAssistantPreviewConfirm: (payload: ParentUiActionPayload) =>
      createDispatchActionWithPayload(
        ParentUiActionKind.PolicyRequestAssistantPreviewConfirmRequested,
        deps,
        dispatchHostAction,
        payload
      ),
    requestPolicyRequestParentResolution: (payload: ParentUiActionPayload) =>
      createDispatchActionWithPayload(
        ParentUiActionKind.PolicyRequestParentResolutionRequested,
        deps,
        dispatchHostAction,
        payload
      ),
    requestScreenSettingsGet: (payload: ParentUiActionPayload) =>
      createDispatchActionWithPayload(ParentUiActionKind.ScreenSettingsGetRequested, deps, dispatchHostAction, payload),
    requestScreenSettingsReplace: (payload: ParentUiActionPayload) =>
      createDispatchActionWithPayload(
        ParentUiActionKind.ScreenSettingsReplaceRequested,
        deps,
        dispatchHostAction,
        payload
      ),
    requestAppGameAdapterDispatchExecute: () =>
      createDispatchActionWithPayload(
        ParentUiActionKind.AppGameAdapterDispatchExecuteRequested,
        deps,
        dispatchHostAction,
        {}
      ),
    requestAppGameTimerParentPreferenceSetup: (payload: ParentUiActionPayload) =>
      createDispatchActionWithPayload(
        ParentUiActionKind.AppGameTimerParentPreferenceSetupRequested,
        deps,
        dispatchHostAction,
        payload
      ),
  };
}

export function createPortalRuntimeActions(
  deps: PortalRuntimeActionDeps,
  dispatchHostAction: (action: ParentUiAction) => Promise<ParentUiActionResult | null>
): PortalRenderActions {
  return {
    reconnect: createReconnectAction(deps, dispatchHostAction),
    selectCommandResult: createSelectCommandResultAction(deps),
    sendCommand: createSendCommandAction(deps, dispatchHostAction),
    refreshRouteSnapshot: createRefreshRouteSnapshotAction(deps, dispatchHostAction),
    ...createPortalRuntimeRequestActions(deps, dispatchHostAction),
  };
}
