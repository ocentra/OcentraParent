import {
  ParentBridgeCommand,
  type ParentBridgeCommandName,
  ParentDevBridgeRoute,
  type ParentDevBridgeRouteName,
  ParentHostBridgeRuntime,
  type HostBridge,
  type ParentDevBridgeUrl,
  type ParentRouteContext,
  type ParentRouteSnapshot,
  type ParentSubscriptionEvent,
  type ParentUiActionResult,
  type ParentUiAction,
  decodeParentBridgeUnsubscribeResult,
  decodeParentRouteSubscriptionId,
  decodeParentSubscriptionEvent,
  decodeParentUiActionResult,
  parentDevBridgeDispatchUnavailableMessage,
  parentDevBridgeHttpError,
  parentRouteSubscriptionEventName,
  presentationOnlyDevWebHostBridgeMessage,
} from '../generated/parent-ui-bridge';
import type { ParentRouteId, ParentUnknownRecord } from '../generated/parent-ui-bridge';
import { PORTAL_HOST_BRIDGE_RUNTIME } from '@ocentra-parent/portal-domain/portal-host-bridge-runtime';
import { invoke as invokeTauriCommand } from '@tauri-apps/api/core';
import { listen as listenTauriEvent } from '@tauri-apps/api/event';
import { dispatchPortalAction } from './host-bridge/action-dispatch';
import { createDevWebRouteSubscription } from './host-bridge/dev-web-subscription';
import {
  createSchemaMismatchParentRouteSnapshot,
  createUnavailableDevWebRouteSnapshot,
} from './host-bridge/dev-web-unavailable-snapshot';
import { resolveParentDevBridgeUrl, trimTrailingSlash } from './host-bridge/dev-web-url';
import { decodeHostRouteSnapshot } from './host-bridge/route-snapshot';
import { isParentTauriRuntime } from './tauri-runtime';

export function createHostBridge(): HostBridge {
  return isParentTauriRuntime() ? createTauriHostBridge() : createDevWebHostBridge();
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

class ParentDevBridgeResponseSchemaError extends Error {}

async function invokeParentDevBridgeCommand(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentDevBridgeRouteName,
  payload: ParentUnknownRecord
): Promise<unknown> {
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
    try {
      return await response.json();
    } catch {
      throw new ParentDevBridgeResponseSchemaError();
    }
  } finally {
    globalThis.clearTimeout(timeoutId);
  }
}

async function invokeParentDevBridgeCommandOrThrow(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentDevBridgeRouteName,
  payload: ParentUnknownRecord
): Promise<unknown> {
  try {
    return await invokeParentDevBridgeCommand(parentDevBridgeUrl, route, payload);
  } catch (error) {
    if (
      error instanceof TypeError ||
      (error instanceof Error && error.name === PORTAL_HOST_BRIDGE_RUNTIME.AbortErrorName)
    ) {
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
  let response: unknown;
  try {
    response = await invokeParentDevBridgeCommandOrThrow(parentDevBridgeUrl, ParentDevBridgeRoute.LoadRoute, {
      route,
      context: context ?? null,
    });
  } catch (error) {
    if (error instanceof ParentDevBridgeResponseSchemaError) {
      return createSchemaMismatchParentRouteSnapshot(route, ParentHostBridgeRuntime.AgentEndpointDevWeb);
    }
    return createUnavailableDevWebRouteSnapshot(parentDevBridgeUrl, route);
  }
  return decodeHostRouteSnapshot(response, route, ParentHostBridgeRuntime.AgentEndpointDevWeb);
}

function createTauriLoadRouteAction(): (
  route: ParentRouteId,
  context?: ParentRouteContext
) => Promise<ParentRouteSnapshot> {
  return async (route, context) => {
    const response = await invokeParentBridgeCommand(ParentBridgeCommand.LoadRoute, {
      route,
      context: context ?? null,
    });
    return decodeHostRouteSnapshot(response, route);
  };
}

function createTauriDispatchAction(): (action: ParentUiAction) => Promise<ParentUiActionResult> {
  return (action) =>
    dispatchPortalAction(action, async () => {
      const response = await invokeParentBridgeCommand(ParentBridgeCommand.Dispatch, { action });
      return decodeParentUiActionResult(response);
    });
}

function createTauriSubscribeAction(): (
  route: ParentRouteId,
  context: ParentRouteContext | undefined,
  onEvent: (event: ParentSubscriptionEvent) => void
) => Promise<() => void> {
  return async (route, context, onEvent) => {
    const subscriptionId = decodeParentRouteSubscriptionId(
      await invokeParentBridgeCommand(ParentBridgeCommand.Subscribe, {
        route,
        context: context ?? null,
      })
    );
    const unlisten = await listenTauriEvent<unknown>(parentRouteSubscriptionEventName(subscriptionId), (event) => {
      try {
        const subscriptionEvent = decodeParentSubscriptionEvent(event.payload);
        if (subscriptionEvent.route === route) {
          onEvent(subscriptionEvent);
        }
      } catch {
        // Invalid host payloads never reach portal state.
      }
    });
    return () => {
      unlisten();
      void invokeParentBridgeCommand(ParentBridgeCommand.Unsubscribe, { subscriptionId })
        .then(decodeParentBridgeUnsubscribeResult)
        .catch(() => undefined);
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
    dispatchPortalAction(action, async () => {
      const response = await invokeParentDevBridgeCommandOrThrow(parentDevBridgeUrl, ParentDevBridgeRoute.Dispatch, {
        action,
      });
      return decodeParentUiActionResult(response);
    });
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

async function invokeParentBridgeCommand(
  command: ParentBridgeCommandName,
  args: ParentUnknownRecord
): Promise<unknown> {
  return invokeTauriCommand<unknown>(command, args);
}
