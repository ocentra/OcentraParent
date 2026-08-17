import {
  ParentBridgeCommand,
  type ParentBridgeCommandName,
  ParentDevBridgeRoute,
  type ParentDevBridgeRouteName,
  ParentHostBridgeRuntime,
  ParentUiActionKind,
  type HostBridge,
  type ParentDevBridgeUrl,
  type ParentRouteContext,
  type ParentRouteSnapshot,
  type ParentRouteSubscriptionEventName,
  type ParentRouteSubscriptionId,
  type ParentSubscriptionEvent,
  type ParentUiActionResult,
  type ParentUiAction,
  parentDevBridgeDispatchUnavailableMessage,
  parentDevBridgeHttpError,
  parentRouteSubscriptionEventName,
  presentationOnlyDevWebHostBridgeMessage,
} from '../generated/parent-ui-bridge';
import type { ParentRouteId, ParentUnknownRecord } from '../generated/parent-ui-bridge';
import { DirectEnforcementCommandBoundaryErrorText, isDirectEnforcementCommand } from './transport';
import { createDevWebRouteSubscription } from './host-bridge/dev-web-subscription';
import { createUnavailableDevWebRouteSnapshot } from './host-bridge/dev-web-unavailable-snapshot';

type TauriCoreModule = {
  invoke<TResult>(command: ParentBridgeCommandName, args?: ParentUnknownRecord): Promise<TResult>;
};

type TauriEventModule = {
  listen<TPayload>(
    event: ParentRouteSubscriptionEventName,
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
    loadRoute: createTauriLoadRouteAction(),
    dispatch: createTauriDispatchAction(),
    subscribe: createTauriSubscribeAction(),
  };
}

export function createDevWebHostBridge(parentDevBridgeUrl = resolveParentDevBridgeUrl()): HostBridge {
  if (parentDevBridgeUrl === null) {
    return createUnavailableDevWebHostBridge();
  }

  return {
    loadRoute: createDevWebLoadRouteAction(parentDevBridgeUrl),
    dispatch: createDevWebDispatchAction(parentDevBridgeUrl),
    subscribe: createDevWebSubscribeAction(parentDevBridgeUrl),
  };
}

function createUnavailableDevWebHostBridge(): HostBridge {
  const message = presentationOnlyDevWebHostBridgeMessage();
  const unavailable = async <T>(): Promise<T> => Promise.reject(new Error(message));
  return {
    loadRoute: createUnavailableLoadRouteAction(unavailable),
    dispatch: createUnavailableDispatchAction(unavailable),
    subscribe: createUnavailableSubscribeAction(unavailable),
  };
}

async function invokeParentDevBridgeCommand<TResult>(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentDevBridgeRouteName,
  payload: ParentUnknownRecord
): Promise<TResult> {
  const abortController = new AbortController();
  const timeoutId = globalThis.setTimeout(
    () => abortController.abort(),
    ParentHostBridgeRuntime.DevBridgeRequestTimeoutMs
  );
  try {
    const response = await fetch(
      `${trimTrailingSlash(parentDevBridgeUrl)}${ParentHostBridgeRuntime.UrlPathSeparator}${route}`,
      {
        method: ParentHostBridgeRuntime.PostMethod,
        headers: {
          [ParentHostBridgeRuntime.JsonContentTypeHeader]: ParentHostBridgeRuntime.JsonContentType,
        },
        body: JSON.stringify(payload),
        signal: abortController.signal,
      }
    );
    if (!response.ok) {
      throw new Error(parentDevBridgeHttpError(route, response.status));
    }
    return (await response.json()) as TResult;
  } finally {
    globalThis.clearTimeout(timeoutId);
  }
}

async function invokeParentDevBridgeCommandOrThrow<TResult>(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentDevBridgeRouteName,
  payload: ParentUnknownRecord
): Promise<TResult> {
  try {
    return await invokeParentDevBridgeCommand<TResult>(parentDevBridgeUrl, route, payload);
  } catch (error) {
    if (error instanceof TypeError || (error instanceof Error && error.name === 'AbortError')) {
      throw new Error(parentDevBridgeDispatchUnavailableMessage(parentDevBridgeUrl));
    }
    if (error instanceof Error) {
      throw error;
    }
    throw new Error(parentDevBridgeDispatchUnavailableMessage(parentDevBridgeUrl));
  }
}

async function loadDevWebRouteSnapshot(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId,
  context?: ParentRouteContext
): Promise<ParentRouteSnapshot> {
  try {
    return await invokeParentDevBridgeCommandOrThrow<ParentRouteSnapshot>(
      parentDevBridgeUrl,
      ParentDevBridgeRoute.LoadRoute,
      {
        route,
        context: context ?? null,
      }
    );
  } catch {
    return createUnavailableDevWebRouteSnapshot(parentDevBridgeUrl, route);
  }
}

function createTauriLoadRouteAction(): (
  route: ParentRouteId,
  context?: ParentRouteContext
) => Promise<ParentRouteSnapshot> {
  return (route, context) =>
    invokeParentBridgeCommand<ParentRouteSnapshot>(ParentBridgeCommand.LoadRoute, {
      route,
      context: context ?? null,
    });
}

function createTauriDispatchAction(): (action: ParentUiAction) => Promise<ParentUiActionResult> {
  return (action) =>
    dispatchPortalAction(action, () =>
      invokeParentBridgeCommand<ParentUiActionResult>(ParentBridgeCommand.Dispatch, { action })
    );
}

function createTauriSubscribeAction(): (
  route: ParentRouteId,
  context: ParentRouteContext | undefined,
  onEvent: (event: ParentSubscriptionEvent) => void
) => Promise<() => void> {
  return async (route, context, onEvent) => {
    const subscriptionId = await invokeParentBridgeCommand<ParentRouteSubscriptionId>(ParentBridgeCommand.Subscribe, {
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
      void invokeParentBridgeCommand<boolean>(ParentBridgeCommand.Unsubscribe, {
        subscriptionId,
      });
    };
  };
}

function createDevWebLoadRouteAction(
  parentDevBridgeUrl: ParentDevBridgeUrl
): (route: ParentRouteId, context?: ParentRouteContext) => Promise<ParentRouteSnapshot> {
  return (route, context) => loadDevWebRouteSnapshot(parentDevBridgeUrl, route, context);
}

function createDevWebDispatchAction(
  parentDevBridgeUrl: ParentDevBridgeUrl
): (action: ParentUiAction) => Promise<ParentUiActionResult> {
  return (action) =>
    dispatchPortalAction(action, () =>
      invokeParentDevBridgeCommandOrThrow<ParentUiActionResult>(parentDevBridgeUrl, ParentDevBridgeRoute.Dispatch, {
        action,
      })
    );
}

function dispatchPortalAction(
  action: ParentUiAction,
  dispatch: () => Promise<ParentUiActionResult>
): Promise<ParentUiActionResult> {
  if (action.action === ParentUiActionKind.AgentCommandRequested && isDirectEnforcementCommand(action.command)) {
    return Promise.reject(new Error(DirectEnforcementCommandBoundaryErrorText));
  }
  return dispatch();
}

function createDevWebSubscribeAction(
  parentDevBridgeUrl: ParentDevBridgeUrl
): (
  route: ParentRouteId,
  context: ParentRouteContext | undefined,
  onEvent: (event: ParentSubscriptionEvent) => void
) => Promise<() => void> {
  return async (route, context, onEvent) =>
    createDevWebRouteSubscription(parentDevBridgeUrl, route, context, onEvent, loadDevWebRouteSnapshot);
}

function createUnavailableLoadRouteAction(
  unavailable: <TResult>() => Promise<TResult>
): (route?: ParentRouteId, context?: ParentRouteContext) => Promise<ParentRouteSnapshot> {
  return async () => unavailable<ParentRouteSnapshot>();
}

function createUnavailableDispatchAction(
  unavailable: <TResult>() => Promise<TResult>
): (action?: ParentUiAction) => Promise<ParentUiActionResult> {
  return async () => unavailable<ParentUiActionResult>();
}

function createUnavailableSubscribeAction(
  unavailable: <TResult>() => Promise<TResult>
): (
  route?: ParentRouteId,
  context?: ParentRouteContext,
  onEvent?: (event: ParentSubscriptionEvent) => void
) => Promise<() => void> {
  return async () => unavailable<() => void>();
}

async function invokeParentBridgeCommand<TResult>(
  command: ParentBridgeCommandName,
  args: ParentUnknownRecord
): Promise<TResult> {
  const tauriCore = await loadTauriCoreModule();
  return tauriCore.invoke<TResult>(command, args);
}

function loadTauriCoreModule(): Promise<TauriCoreModule> {
  if (tauriCoreModulePromise === null) {
    tauriCoreModulePromise = import(ParentHostBridgeRuntime.TauriCoreModule) as Promise<TauriCoreModule>;
  }
  return tauriCoreModulePromise;
}

function loadTauriEventModule(): Promise<TauriEventModule> {
  if (tauriEventModulePromise === null) {
    tauriEventModulePromise = import(ParentHostBridgeRuntime.TauriEventModule) as Promise<TauriEventModule>;
  }
  return tauriEventModulePromise;
}

function resolveParentDevBridgeUrl(): ParentDevBridgeUrl | null {
  const value = import.meta.env[ParentHostBridgeRuntime.DevBridgeUrlEnvKey];
  return typeof value === ParentHostBridgeRuntime.StringType && value.trim().length > 0 ? value.trim() : null;
}

function trimTrailingSlash(value: ParentDevBridgeUrl): ParentDevBridgeUrl {
  return value.endsWith(ParentHostBridgeRuntime.UrlPathSeparator) ? value.slice(0, -1) : value;
}

function isTauriRuntime(): boolean {
  return (
    typeof window !== ParentHostBridgeRuntime.TypeofUndefined &&
    ParentHostBridgeRuntime.TauriInternalWindowKey in window
  );
}
