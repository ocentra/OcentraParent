import {
  ParentHostBridgeRuntime,
  type ParentDevBridgeUrl,
  type ParentRouteContext,
  type ParentRouteId,
  type ParentRouteSnapshot,
  type ParentSubscriptionEvent,
} from '../../generated/parent-ui-bridge';

type LoadRouteSnapshot = (
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId,
  context?: ParentRouteContext
) => Promise<ParentRouteSnapshot>;

export function createDevWebRouteSubscription(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId,
  context: ParentRouteContext | undefined,
  onEvent: (event: ParentSubscriptionEvent) => void,
  loadRouteSnapshot: LoadRouteSnapshot
): () => void {
  const subscriptionState = {
    active: true,
    lastSnapshotJson: JSON.stringify(null),
  };
  const emitNextSnapshot = createDevWebEmitNextSnapshot(
    subscriptionState,
    parentDevBridgeUrl,
    route,
    context,
    onEvent,
    loadRouteSnapshot
  );
  void emitNextSnapshot();
  const intervalId = globalThis.setInterval(() => {
    void emitNextSnapshot();
  }, ParentHostBridgeRuntime.DevRouteSubscriptionPollMs);

  return () => {
    subscriptionState.active = false;
    globalThis.clearInterval(intervalId);
  };
}

function createDevWebEmitNextSnapshot(
  subscriptionState: {
    active: boolean;
    lastSnapshotJson: ReturnType<typeof JSON.stringify>;
  },
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId,
  context: ParentRouteContext | undefined,
  onEvent: (event: ParentSubscriptionEvent) => void,
  loadRouteSnapshot: LoadRouteSnapshot
): () => Promise<void> {
  return async function emitNextSnapshot(): Promise<void> {
    if (!subscriptionState.active) {
      return;
    }
    let snapshot: ParentRouteSnapshot;
    try {
      snapshot = await loadRouteSnapshot(parentDevBridgeUrl, route, context);
    } catch {
      return;
    }
    const snapshotJson = JSON.stringify(snapshot);
    if (snapshotJson === subscriptionState.lastSnapshotJson) {
      return;
    }
    subscriptionState.lastSnapshotJson = snapshotJson;
    onEvent({
      schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
      route,
      snapshot,
    });
  };
}
