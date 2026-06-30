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
  type ParentRouteSubscriptionEventName,
  type ParentRouteSubscriptionId,
  type ParentSubscriptionEvent,
  type ParentUiActionResult,
  parentDevBridgeDispatchUnavailableMessage,
  parentDevBridgeHttpError,
  parentRouteSubscriptionEventName,
  presentationOnlyDevWebHostBridgeMessage,
} from '../generated/parent-ui-bridge';
import type { ParentRouteId, ParentUnknownRecord } from '../generated/parent-ui-bridge';

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
    loadRoute(route, context) {
      return invokeParentBridgeCommand<ParentRouteSnapshot>(ParentBridgeCommand.LoadRoute, {
        route,
        context: context ?? null,
      });
    },
    dispatch(action) {
      return invokeParentBridgeCommand<ParentUiActionResult>(ParentBridgeCommand.Dispatch, { action });
    },
    async subscribe(route, context, onEvent) {
      const subscriptionId = await invokeParentBridgeCommand<ParentRouteSubscriptionId>(
        ParentBridgeCommand.Subscribe,
        {
          route,
          context: context ?? null,
        }
      );
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
    },
  };
}

export function createDevWebHostBridge(parentDevBridgeUrl = resolveParentDevBridgeUrl()): HostBridge {
  if (parentDevBridgeUrl === null) {
    return createUnavailableDevWebHostBridge();
  }

  const loadSnapshot = async (
    route: ParentRouteId,
    context?: ParentRouteContext
  ): Promise<ParentRouteSnapshot> => {
    return invokeParentDevBridgeCommandOrThrow<ParentRouteSnapshot>(
      parentDevBridgeUrl,
      ParentDevBridgeRoute.LoadRoute,
      {
        route,
        context: context ?? null,
      }
    );
  };

  return {
    async loadRoute(route, context) {
      return loadSnapshot(route, context);
    },
    async dispatch(action) {
      return invokeParentDevBridgeCommandOrThrow<ParentUiActionResult>(
        parentDevBridgeUrl,
        ParentDevBridgeRoute.Dispatch,
        {
          action,
        }
      );
    },
    async subscribe(route, context, onEvent) {
      let active = true;
      let lastSnapshotJson = JSON.stringify(null);

      const emitNextSnapshot = async (): Promise<void> => {
        if (!active) {
          return;
        }
        let snapshot: ParentRouteSnapshot;
        try {
          snapshot = await loadSnapshot(route, context);
        } catch {
          return;
        }
        const snapshotJson = JSON.stringify(snapshot);
        if (snapshotJson === lastSnapshotJson) {
          return;
        }
        lastSnapshotJson = snapshotJson;
        onEvent({
          schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
          route,
          snapshot,
        });
      };

      void emitNextSnapshot();
      const intervalId = globalThis.setInterval(() => {
        void emitNextSnapshot();
      }, ParentHostBridgeRuntime.DevRouteSubscriptionPollMs);

      return () => {
        active = false;
        globalThis.clearInterval(intervalId);
      };
    },
  };
}

function createUnavailableDevWebHostBridge(): HostBridge {
  const message = presentationOnlyDevWebHostBridgeMessage();
  const unavailable = async <T>(): Promise<T> => Promise.reject(new Error(message));
  return {
    async loadRoute() {
      return unavailable();
    },
    async dispatch() {
      return unavailable();
    },
    async subscribe() {
      return unavailable();
    },
  };
}

async function invokeParentDevBridgeCommand<TResult>(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentDevBridgeRouteName,
  payload: ParentUnknownRecord
): Promise<TResult> {
  const response = await fetch(
    `${trimTrailingSlash(parentDevBridgeUrl)}${ParentHostBridgeRuntime.UrlPathSeparator}${route}`,
    {
      method: ParentHostBridgeRuntime.PostMethod,
      headers: {
        [ParentHostBridgeRuntime.JsonContentTypeHeader]: ParentHostBridgeRuntime.JsonContentType,
      },
      body: JSON.stringify(payload),
    }
  );
  if (!response.ok) {
    throw new Error(parentDevBridgeHttpError(route, response.status));
  }
  return (await response.json()) as TResult;
}

async function invokeParentDevBridgeCommandOrThrow<TResult>(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentDevBridgeRouteName,
  payload: ParentUnknownRecord
): Promise<TResult> {
  try {
    return await invokeParentDevBridgeCommand<TResult>(parentDevBridgeUrl, route, payload);
  } catch (error) {
    if (error instanceof Error && error.message.startsWith('parent dev bridge ')) {
      throw error;
    }
    throw new Error(parentDevBridgeDispatchUnavailableMessage(parentDevBridgeUrl));
  }
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
  return typeof value === ParentHostBridgeRuntime.StringType && value.trim().length > 0
    ? value.trim()
    : null;
}

function trimTrailingSlash(value: ParentDevBridgeUrl): ParentDevBridgeUrl {
  return value.endsWith(ParentHostBridgeRuntime.UrlPathSeparator)
    ? value.slice(0, -1)
    : value;
}

function isTauriRuntime(): boolean {
  return typeof window !== ParentHostBridgeRuntime.TypeofUndefined && ParentHostBridgeRuntime.TauriInternalWindowKey in window;
}
